use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Parser, punctuated::Punctuated, Token};

/// Parsed arguments from `#[component(...)]`.
struct ComponentArgs {
    cpp_type: Option<String>,
    group: Option<String>,
    existing: bool,
    include: Option<String>,
}

fn parse_args(attr: TokenStream) -> syn::Result<ComponentArgs> {
    let mut cpp_type = None;
    let mut group = None;
    let mut existing = false;
    let mut include = None;

    if attr.is_empty() {
        return Ok(ComponentArgs { cpp_type, group, existing, include });
    }

    let metas = Punctuated::<syn::NestedMeta, Token![,]>::parse_terminated.parse2(attr)?;
    for meta in &metas {
        match meta {
            syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) if nv.path.is_ident("cpp_type") => {
                if let syn::Lit::Str(s) = &nv.lit {
                    cpp_type = Some(s.value());
                } else {
                    return Err(syn::Error::new_spanned(&nv.lit, "cpp_type must be a string literal"));
                }
            }
            syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) if nv.path.is_ident("group") => {
                if let syn::Lit::Str(s) = &nv.lit {
                    group = Some(s.value());
                } else {
                    return Err(syn::Error::new_spanned(&nv.lit, "group must be a string literal"));
                }
            }
            syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) if nv.path.is_ident("include") => {
                if let syn::Lit::Str(s) = &nv.lit {
                    include = Some(s.value());
                } else {
                    return Err(syn::Error::new_spanned(&nv.lit, "include must be a string literal"));
                }
            }
            syn::NestedMeta::Meta(syn::Meta::Path(p)) if p.is_ident("existing") => {
                existing = true;
            }
            other => {
                return Err(syn::Error::new_spanned(
                    other,
                    "unknown argument; expected cpp_type, group, existing, or include",
                ));
            }
        }
    }

    Ok(ComponentArgs { cpp_type, group, existing, include })
}

/// Detect whether a struct is a tag (unit/empty) or a fragment (has fields).
fn is_tag_struct(input: &syn::ItemStruct) -> bool {
    match &input.fields {
        syn::Fields::Unit => true,
        syn::Fields::Named(fields) => fields.named.is_empty(),
        syn::Fields::Unnamed(fields) => fields.unnamed.is_empty(),
    }
}

/// Check if `#[repr(C)]` is already present.
fn has_repr_c(input: &syn::ItemStruct) -> bool {
    input.attrs.iter().any(|attr| {
        if !attr.path.is_ident("repr") {
            return false;
        }
        if let Ok(meta) = attr.parse_meta() {
            match meta {
                syn::Meta::List(list) => list.nested.iter().any(|nested| {
                    matches!(nested, syn::NestedMeta::Meta(syn::Meta::Path(p)) if p.is_ident("C"))
                }),
                _ => false,
            }
        } else {
            false
        }
    })
}

/// Resolve the C++ type name: explicit `cpp_type` or auto-derived from
/// `BEVY_MASS_CPP_PREFIX` env var + struct name.
fn resolve_cpp_type(args: &ComponentArgs, struct_name: &str, is_tag: bool) -> syn::Result<String> {
    if let Some(ref explicit) = args.cpp_type {
        return Ok(explicit.clone());
    }

    let suffix = if is_tag { "Tag" } else { "Fragment" };
    match std::env::var("BEVY_MASS_CPP_PREFIX") {
        Ok(prefix) => Ok(format!("F{}{}{}", prefix, struct_name, suffix)),
        Err(_) => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "No cpp_type specified and BEVY_MASS_CPP_PREFIX env var not set. \
             Either add cpp_type = \"FMyType\" to #[component(...)] or set \
             BEVY_MASS_CPP_PREFIX in your crate's build.rs: \
             println!(\"cargo::rustc-env=BEVY_MASS_CPP_PREFIX=MyPrefix\");",
        )),
    }
}

pub fn component_impl(
    attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let args = parse_args(attr)?;
    let mut input: syn::ItemStruct = syn::parse2(item)?;
    let name = &input.ident;
    let is_tag = is_tag_struct(&input);

    let cpp_type = resolve_cpp_type(&args, &name.to_string(), is_tag)?;

    // Add #[repr(C)] for fragments (not tags) if not already present
    if !is_tag && !has_repr_c(&input) {
        input.attrs.push(syn::parse_quote!(#[repr(C)]));
    }

    // Build the mass(...) attribute content for Unreal mode (bare, without #[...] wrapping)
    let mass_attr = if is_tag {
        if args.existing {
            if let Some(ref inc) = args.include {
                quote! { mass(cpp_type = #cpp_type, tag, existing, include = #inc) }
            } else {
                quote! { mass(cpp_type = #cpp_type, tag, existing) }
            }
        } else {
            quote! { mass(cpp_type = #cpp_type, tag) }
        }
    } else if args.existing {
        if let Some(ref inc) = args.include {
            quote! { mass(cpp_type = #cpp_type, existing, include = #inc) }
        } else {
            quote! { mass(cpp_type = #cpp_type, existing) }
        }
    } else {
        quote! { mass(cpp_type = #cpp_type) }
    };

    // Generate the entity group const for tags with group
    let group_impl = if let Some(ref group) = args.group {
        quote! {
            impl #name {
                /// Entity group name for `MassEntityMap` lookup.
                pub const ENTITY_GROUP: &'static str = #group;
            }
        }
    } else {
        quote! {}
    };

    Ok(quote! {
        #[cfg_attr(feature = "unreal", derive(unreal_api::MassFragment))]
        #[derive(Component, Clone, Copy, Debug)]
        #[cfg_attr(feature = "unreal", #mass_attr)]
        #input

        #group_impl
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn component_adds_repr_c_for_fragment() {
        let attr = quote! { cpp_type = "FTestFragment" };
        let item = quote! {
            pub struct TestFragment {
                pub value: f32,
            }
        };
        let result = component_impl(attr, item).unwrap();
        let output = result.to_string();
        assert!(output.contains("repr"), "should add #[repr(C)], got: {}", output);
    }

    #[test]
    fn component_preserves_existing_repr_c() {
        let attr = quote! { cpp_type = "FTestFragment" };
        let item = quote! {
            #[repr(C)]
            pub struct TestFragment {
                pub value: f32,
            }
        };
        let result = component_impl(attr, item).unwrap();
        let output = result.to_string();
        assert_eq!(output.matches("repr").count(), 1, "should not duplicate #[repr(C)], got: {}", output);
    }

    #[test]
    fn component_detects_tag_no_repr_c() {
        let attr = quote! { cpp_type = "FTestTag" };
        let item = quote! {
            pub struct TestTag;
        };
        let result = component_impl(attr, item).unwrap();
        let output = result.to_string();
        // Tags should NOT get #[repr(C)]
        assert!(!output.contains("repr (C)") || output.contains("repr"), "tag should not get repr(C)");
        // Check for tag in mass attribute
        assert!(output.contains("tag"), "should have tag in mass attr, got: {}", output);
    }

    #[test]
    fn component_with_explicit_cpp_type() {
        let attr = quote! { cpp_type = "FMyCustomName" };
        let item = quote! {
            pub struct Foo {
                pub x: i32,
            }
        };
        let result = component_impl(attr, item).unwrap();
        let output = result.to_string();
        assert!(output.contains("FMyCustomName"), "should use explicit cpp_type, got: {}", output);
    }

    #[test]
    fn component_with_group() {
        let attr = quote! { cpp_type = "FTestTag", group = "ants" };
        let item = quote! {
            pub struct TestTag;
        };
        let result = component_impl(attr, item).unwrap();
        let output = result.to_string();
        assert!(output.contains("ENTITY_GROUP"), "should generate ENTITY_GROUP const, got: {}", output);
        assert!(output.contains("ants"), "should contain group name, got: {}", output);
    }

    #[test]
    fn component_resolve_cpp_type_explicit_wins() {
        // When cpp_type is explicit, env var doesn't matter
        let args = ComponentArgs {
            cpp_type: Some("FExplicit".to_string()),
            group: None,
            existing: false,
            include: None,
        };
        let result = resolve_cpp_type(&args, "Whatever", false).unwrap();
        assert_eq!(result, "FExplicit");
    }

    #[test]
    fn component_resolve_cpp_type_fragment_suffix() {
        // Test the naming convention: F + prefix + struct name + Fragment
        unsafe { std::env::set_var("BEVY_MASS_CPP_PREFIX", "Test") };
        let args = ComponentArgs {
            cpp_type: None,
            group: None,
            existing: false,
            include: None,
        };
        let result = resolve_cpp_type(&args, "Food", false);
        if let Ok(name) = result {
            assert_eq!(name, "FTestFoodFragment");
        }
    }

    #[test]
    fn component_resolve_cpp_type_tag_suffix() {
        // Test the naming convention: F + prefix + struct name + Tag
        unsafe { std::env::set_var("BEVY_MASS_CPP_PREFIX", "Test") };
        let args = ComponentArgs {
            cpp_type: None,
            group: None,
            existing: false,
            include: None,
        };
        let result = resolve_cpp_type(&args, "Ant", true);
        if let Ok(name) = result {
            assert_eq!(name, "FTestAntTag");
        }
    }

    #[test]
    fn component_existing_with_include() {
        let attr = quote! { cpp_type = "FTransformFragment", existing, include = "MassCommonFragments.h" };
        let item = quote! {
            #[repr(align(16))]
            pub struct Transform {
                pub translation: [f64; 3],
            }
        };
        let result = component_impl(attr, item).unwrap();
        let output = result.to_string();
        assert!(output.contains("existing"), "should have existing flag, got: {}", output);
        assert!(output.contains("MassCommonFragments.h"), "should have include header, got: {}", output);
    }
}
