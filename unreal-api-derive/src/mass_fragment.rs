use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field, Lit, Meta, NestedMeta};

/// Extracts the `cpp_type` value from `#[mass(cpp_type = "...")]` attribute.
fn extract_cpp_type(ast: &DeriveInput) -> syn::Result<String> {
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
                            return Ok(lit_str.value());
                        }
                    }
                }
            }
        }
    }
    Err(syn::Error::new_spanned(
        &ast.ident,
        "#[derive(MassFragment)] requires #[mass(cpp_type = \"FCppTypeName\")]",
    ))
}

/// Checks if the struct has `#[mass(tag)]` attribute.
fn is_tag(ast: &DeriveInput) -> bool {
    for attr in &ast.attrs {
        if !attr.path.is_ident("mass") {
            continue;
        }
        if let Ok(Meta::List(list)) = attr.parse_meta() {
            for nested in &list.nested {
                if let NestedMeta::Meta(Meta::Path(path)) = nested {
                    if path.is_ident("tag") {
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
    let cpp_type = extract_cpp_type(ast)?;
    let rust_type_name = name.to_string();
    let tag = is_tag(ast);

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

    Ok(quote! {
        impl ::unreal_api::mass::MassFragment for #name {
            const CPP_TYPE_NAME: &'static str = #cpp_type;
        }

        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        const #fields_const_name: [::unreal_api::mass::MassFragmentFieldInfo; #num_fields] = [
            #(#fields),*
        ];

        #[doc(hidden)]
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
                    write_default: #write_default_expr,
                }
            }
        };
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
        assert_eq!(cpp_type, "FMyFragment");
    }

    #[test]
    fn errors_without_mass_attribute() {
        let input: DeriveInput = syn::parse2(quote! {
            #[repr(C)]
            struct MyFragment {
                x: f64,
            }
        })
        .unwrap();

        let result = extract_cpp_type(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cpp_type"));
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
    }
}
