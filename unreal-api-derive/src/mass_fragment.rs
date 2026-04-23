use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field, Lit, Meta, NestedMeta};

/// Extracts the `cpp_type` value from `#[mass(cpp_type = "...")]` attribute.
/// Returns `None` if absent — caller falls back to env-var auto-derivation.
fn extract_cpp_type(ast: &DeriveInput) -> syn::Result<Option<String>> {
    for attr in &ast.attrs {
        if !attr.path.is_ident("mass") {
            continue;
        }
        let meta = attr.parse_meta()?;
        if let Meta::List(list) = meta {
            for nested in &list.nested {
                if let NestedMeta::Meta(Meta::NameValue(nv)) = nested {
                    if nv.path.is_ident("cpp_type") {
                        if let Lit::Str(lit_str) = &nv.lit {
                            return Ok(Some(lit_str.value()));
                        }
                    }
                }
            }
        }
    }
    Ok(None)
}

/// Detect whether a struct is a tag (unit/empty) or a fragment (has fields).
fn is_tag_struct(ast: &DeriveInput) -> bool {
    match &ast.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Unit => true,
            syn::Fields::Named(fields) => fields.named.is_empty(),
            syn::Fields::Unnamed(fields) => fields.unnamed.is_empty(),
        },
        _ => false,
    }
}

/// Resolve the C++ type name: explicit `cpp_type` or auto-derived from
/// `BEVY_MASS_CPP_PREFIX` env var + struct name + suffix.
fn resolve_cpp_type(ast: &DeriveInput, is_tag: bool) -> syn::Result<String> {
    if let Some(explicit) = extract_cpp_type(ast)? {
        return Ok(explicit);
    }
    let suffix = if is_tag { "Tag" } else { "Fragment" };
    match std::env::var("BEVY_MASS_CPP_PREFIX") {
        Ok(prefix) => Ok(format!("F{}{}{}", prefix, ast.ident, suffix)),
        Err(_) => Err(syn::Error::new_spanned(
            &ast.ident,
            "#[derive(MassFragment)]: no cpp_type set and BEVY_MASS_CPP_PREFIX env var not present. \
             Either add #[mass(cpp_type = \"FMyType\")] or set BEVY_MASS_CPP_PREFIX in your crate's \
             build.rs: println!(\"cargo::rustc-env=BEVY_MASS_CPP_PREFIX=MyPrefix\");",
        )),
    }
}

/// Checks if the struct has `#[mass(existing)]` attribute.
fn is_existing(ast: &DeriveInput) -> bool {
    has_mass_flag(ast, "existing")
}

/// Checks if the struct has `#[mass(tag)]` attribute (legacy opt-in).
/// Auto-detection from struct shape is preferred over this.
fn has_explicit_tag_flag(ast: &DeriveInput) -> bool {
    has_mass_flag(ast, "tag")
}

/// Extracts the `include` value from `#[mass(include = "...")]` attribute.
/// Returns empty string if not present.
fn extract_include_header(ast: &DeriveInput) -> String {
    for attr in &ast.attrs {
        if !attr.path.is_ident("mass") {
            continue;
        }
        if let Ok(Meta::List(list)) = attr.parse_meta() {
            for nested in &list.nested {
                if let NestedMeta::Meta(Meta::NameValue(nv)) = nested {
                    if nv.path.is_ident("include") {
                        if let Lit::Str(lit_str) = &nv.lit {
                            return lit_str.value();
                        }
                    }
                }
            }
        }
    }
    String::new()
}

/// Extracts the `group` value from `#[mass(group = "...")]` attribute.
fn extract_group(ast: &DeriveInput) -> Option<String> {
    for attr in &ast.attrs {
        if !attr.path.is_ident("mass") {
            continue;
        }
        if let Ok(Meta::List(list)) = attr.parse_meta() {
            for nested in &list.nested {
                if let NestedMeta::Meta(Meta::NameValue(nv)) = nested {
                    if nv.path.is_ident("group") {
                        if let Lit::Str(lit_str) = &nv.lit {
                            return Some(lit_str.value());
                        }
                    }
                }
            }
        }
    }
    None
}

/// Checks if the struct has a `#[mass(FLAG)]` path attribute.
fn has_mass_flag(ast: &DeriveInput, flag: &str) -> bool {
    for attr in &ast.attrs {
        if !attr.path.is_ident("mass") {
            continue;
        }
        if let Ok(Meta::List(list)) = attr.parse_meta() {
            for nested in &list.nested {
                if let NestedMeta::Meta(Meta::Path(path)) = nested {
                    if path.is_ident(flag) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Extracts the `default` value from `#[mass(default = "...")]` on a field.
/// Returns empty string if not present.
fn extract_field_default(field: &Field) -> String {
    for attr in &field.attrs {
        if !attr.path.is_ident("mass") {
            continue;
        }
        if let Ok(Meta::List(list)) = attr.parse_meta() {
            for nested in &list.nested {
                if let NestedMeta::Meta(Meta::NameValue(nv)) = nested {
                    if nv.path.is_ident("default") {
                        if let Lit::Str(lit_str) = &nv.lit {
                            return lit_str.value();
                        }
                    }
                }
            }
        }
    }
    String::new()
}

pub fn mass_fragment_derive(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let rust_type_name = name.to_string();

    // Tag = unit struct / empty struct, OR explicit #[mass(tag)] (legacy).
    let tag = is_tag_struct(ast) || has_explicit_tag_flag(ast);
    let existing = is_existing(ast);
    let include_header = extract_include_header(ast);
    let cpp_type = resolve_cpp_type(ast, tag)?;
    let group = extract_group(ast);

    // Extract field info for C++ codegen
    let fields = match &ast.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => fields
                .named
                .iter()
                .map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    let field_name_str = field_name.to_string();
                    let field_type = &f.ty;
                    let type_str = quote!(#field_type).to_string();
                    let default_str = extract_field_default(f);
                    quote! {
                        ::unreal_api::mass::MassFragmentFieldInfo {
                            name: #field_name_str,
                            type_name: #type_str,
                            offset: ::std::mem::offset_of!(#name, #field_name),
                            size: ::std::mem::size_of::<#field_type>(),
                            default_value: #default_str,
                        }
                    }
                })
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        },
        _ => Vec::new(),
    };

    let num_fields = fields.len();

    let fields_const_name = quote::format_ident!("__MASS_FRAGMENT_FIELDS_{}", name);
    let reg_static_name = quote::format_ident!("__mass_fragment_reg_{}", name);

    // For non-tag fragments, generate a write_default function that writes
    // Default::default() bytes into a buffer. This lets codegen derive C++
    // defaults from the Rust impl without manual #[mass(default = "...")] attrs.
    let write_default_expr = if tag {
        quote! { None }
    } else {
        quote! {
            Some(|buf: *mut u8| {
                let val = <#name as ::std::default::Default>::default();
                unsafe {
                    ::std::ptr::copy_nonoverlapping(
                        &val as *const #name as *const u8,
                        buf,
                        ::std::mem::size_of::<#name>(),
                    );
                }
                ::std::mem::forget(val);
            })
        }
    };

    // Emit `ENTITY_GROUP` constant when `#[mass(group = "...")]` is present.
    // Lives outside the `feature = "unreal"` gate so tests in pure-Bevy builds
    // can reference the constant. No runtime cost.
    let group_impl = if let Some(group) = &group {
        quote! {
            impl #name {
                /// Entity group name for `MassEntityMap` lookup.
                pub const ENTITY_GROUP: &'static str = #group;
            }
        }
    } else {
        quote! {}
    };

    // All the MassFragment / ChunkBacked / inventory registration code is
    // feature-gated so `#[derive(MassFragment)]` is a no-op in pure-Bevy
    // builds. Game code can write the derive unconditionally without
    // `#[cfg_attr]` dance.
    let unreal_block = quote! {
        #[cfg(feature = "unreal")]
        const _: () = {
            impl ::unreal_api::mass::MassFragment for #name {
                const CPP_TYPE_NAME: &'static str = #cpp_type;
            }

            impl ::unreal_api::mass::ChunkBacked for #name {}

            #[allow(non_upper_case_globals)]
            const #fields_const_name: [::unreal_api::mass::MassFragmentFieldInfo; #num_fields] = [
                #(#fields),*
            ];

            #[allow(non_upper_case_globals)]
            static #reg_static_name: () = {
                ::unreal_api::inventory::submit! {
                    ::unreal_api::mass::MassFragmentRegistration {
                        cpp_type_name: #cpp_type,
                        rust_type_name: #rust_type_name,
                        size: ::std::mem::size_of::<#name>(),
                        align: ::std::mem::align_of::<#name>(),
                        fields: &#fields_const_name,
                        is_tag: #tag,
                        existing: #existing,
                        include_header: #include_header,
                        write_default: #write_default_expr,
                    }
                }
            };
        };
    };

    Ok(quote! {
        #unreal_block
        #group_impl
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn extracts_cpp_type_from_attribute() {
        let input: DeriveInput = syn::parse2(quote! {
            #[mass(cpp_type = "FMyFragment")]
            #[repr(C)]
            struct MyFragment {
                x: f64,
            }
        })
        .unwrap();

        let cpp_type = extract_cpp_type(&input).unwrap();
        assert_eq!(cpp_type, Some("FMyFragment".to_string()));
    }

    #[test]
    fn missing_cpp_type_falls_back_to_env_or_errors() {
        // With BEVY_MASS_CPP_PREFIX unset, derive errors clearly.
        let input: DeriveInput = syn::parse2(quote! {
            #[repr(C)]
            struct MyFragment {
                x: f64,
            }
        })
        .unwrap();

        // Snapshot and restore the env var to keep the test hermetic.
        let prev = std::env::var("BEVY_MASS_CPP_PREFIX").ok();
        // Safety: test-only, serialized by cargo per test binary.
        unsafe {
            std::env::remove_var("BEVY_MASS_CPP_PREFIX");
        }
        let result = resolve_cpp_type(&input, false);
        if let Some(p) = prev {
            unsafe {
                std::env::set_var("BEVY_MASS_CPP_PREFIX", p);
            }
        }
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("BEVY_MASS_CPP_PREFIX"));
    }

    #[test]
    fn cpp_type_auto_derived_from_env() {
        let input: DeriveInput = syn::parse2(quote! {
            #[repr(C)]
            struct MyFragment {
                x: f64,
            }
        })
        .unwrap();

        let prev = std::env::var("BEVY_MASS_CPP_PREFIX").ok();
        unsafe {
            std::env::set_var("BEVY_MASS_CPP_PREFIX", "Test");
        }
        let name = resolve_cpp_type(&input, false).unwrap();
        if let Some(p) = prev {
            unsafe {
                std::env::set_var("BEVY_MASS_CPP_PREFIX", p);
            }
        } else {
            unsafe {
                std::env::remove_var("BEVY_MASS_CPP_PREFIX");
            }
        }
        assert_eq!(name, "FTestMyFragmentFragment");
    }

    #[test]
    fn auto_detects_unit_struct_as_tag() {
        let input: DeriveInput = syn::parse2(quote! {
            #[mass(cpp_type = "FMyTag")]
            struct MyTag;
        })
        .unwrap();

        assert!(is_tag_struct(&input));
    }

    #[test]
    fn auto_detects_empty_struct_as_tag() {
        let input: DeriveInput = syn::parse2(quote! {
            #[mass(cpp_type = "FMyTag")]
            struct MyTag {}
        })
        .unwrap();

        assert!(is_tag_struct(&input));
    }

    #[test]
    fn non_empty_struct_is_fragment_not_tag() {
        let input: DeriveInput = syn::parse2(quote! {
            #[mass(cpp_type = "FMyFrag")]
            #[repr(C)]
            struct MyFrag {
                x: f64,
            }
        })
        .unwrap();

        assert!(!is_tag_struct(&input));
    }

    #[test]
    fn extracts_group() {
        let input: DeriveInput = syn::parse2(quote! {
            #[mass(cpp_type = "FAntTag", group = "ants")]
            struct AntTag;
        })
        .unwrap();

        assert_eq!(extract_group(&input), Some("ants".to_string()));
    }

    #[test]
    fn generates_trait_impl() {
        let input: DeriveInput = syn::parse2(quote! {
            #[mass(cpp_type = "FTestFrag")]
            #[repr(C)]
            struct TestFrag {
                x: f64,
            }
        })
        .unwrap();

        let output = mass_fragment_derive(&input).unwrap().to_string();
        assert!(output.contains("MassFragment"), "should impl MassFragment");
        assert!(output.contains("FTestFrag"), "should reference C++ type name");
        assert!(output.contains("inventory"), "should submit to inventory");
        assert!(output.contains("ChunkBacked"), "should impl ChunkBacked");
        assert!(output.contains("feature = \"unreal\""), "unreal block should be cfg-gated");
    }

    #[test]
    fn group_emits_entity_group_const() {
        let input: DeriveInput = syn::parse2(quote! {
            #[mass(cpp_type = "FAntTag", group = "ants")]
            struct AntTag;
        })
        .unwrap();

        let output = mass_fragment_derive(&input).unwrap().to_string();
        assert!(output.contains("ENTITY_GROUP"), "should emit ENTITY_GROUP const");
        assert!(output.contains("\"ants\""), "ENTITY_GROUP value should be 'ants'");
    }

    #[test]
    fn existing_flag_parsed() {
        let input: DeriveInput = syn::parse2(quote! {
            #[mass(cpp_type = "FTransformFragment", existing)]
            #[repr(C)]
            struct Transform {
                rotation: [f64; 4],
            }
        })
        .unwrap();

        assert!(is_existing(&input));
        assert!(!is_tag_struct(&input));
    }

    #[test]
    fn non_existing_flag() {
        let input: DeriveInput = syn::parse2(quote! {
            #[mass(cpp_type = "FMyFrag")]
            #[repr(C)]
            struct MyFrag {
                x: f64,
            }
        })
        .unwrap();

        assert!(!is_existing(&input));
    }
}
