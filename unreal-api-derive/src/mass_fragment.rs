use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Lit, Meta, NestedMeta};

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

pub fn mass_fragment_derive(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let cpp_type = extract_cpp_type(ast)?;
    let rust_type_name = name.to_string();

    Ok(quote! {
        impl ::unreal_api::mass::MassFragment for #name {
            const CPP_TYPE_NAME: &'static str = #cpp_type;
        }

        ::unreal_api::inventory::submit! {
            ::unreal_api::mass::MassFragmentRegistration {
                cpp_type_name: #cpp_type,
                rust_type_name: #rust_type_name,
                size: ::std::mem::size_of::<#name>(),
                align: ::std::mem::align_of::<#name>(),
            }
        }
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
