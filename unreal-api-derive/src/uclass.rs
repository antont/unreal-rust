use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Lit, Meta, NestedMeta};

/// Maps a specifier identifier to its constant token stream.
fn specifier_to_const(specifier: &str) -> Option<TokenStream> {
    let path = match specifier {
        "edit_anywhere" => quote! { ::unreal_api::property::ERustPropertySpecifier::EDIT_ANYWHERE },
        "edit_defaults_only" => {
            quote! { ::unreal_api::property::ERustPropertySpecifier::EDIT_DEFAULTS_ONLY }
        }
        "edit_instance_only" => {
            quote! { ::unreal_api::property::ERustPropertySpecifier::EDIT_INSTANCE_ONLY }
        }
        "visible_anywhere" => {
            quote! { ::unreal_api::property::ERustPropertySpecifier::VISIBLE_ANYWHERE }
        }
        "visible_defaults_only" => {
            quote! { ::unreal_api::property::ERustPropertySpecifier::VISIBLE_DEFAULTS_ONLY }
        }
        "visible_instance_only" => {
            quote! { ::unreal_api::property::ERustPropertySpecifier::VISIBLE_INSTANCE_ONLY }
        }
        "blueprint_read_only" => {
            quote! { ::unreal_api::property::ERustPropertySpecifier::BLUEPRINT_READ_ONLY }
        }
        "blueprint_read_write" => {
            quote! { ::unreal_api::property::ERustPropertySpecifier::BLUEPRINT_READ_WRITE }
        }
        "replicated" => quote! { ::unreal_api::property::ERustPropertySpecifier::REPLICATED },
        "transient" => quote! { ::unreal_api::property::ERustPropertySpecifier::TRANSIENT },
        "save_game" => quote! { ::unreal_api::property::ERustPropertySpecifier::SAVE_GAME },
        "config" => quote! { ::unreal_api::property::ERustPropertySpecifier::CONFIG },
        "advanced_display" => {
            quote! { ::unreal_api::property::ERustPropertySpecifier::ADVANCED_DISPLAY }
        }
        "interp" => quote! { ::unreal_api::property::ERustPropertySpecifier::INTERP },
        _ => return None,
    };
    Some(path)
}

/// Maps known shorthand metadata keys to Unreal metadata key names.
fn map_metadata_key(key: &str) -> String {
    match key {
        "unit" => "Units".to_string(),
        "clamp_min" => "ClampMin".to_string(),
        "clamp_max" => "ClampMax".to_string(),
        "display_name" => "DisplayName".to_string(),
        "tooltip" => "Tooltip".to_string(),
        _ => key.to_string(),
    }
}

struct UPropertyArgs {
    specifier_consts: Vec<TokenStream>,
    metadata: Vec<(String, String)>,
}

/// Parses `#[uproperty(...)]` arguments into specifier constants and metadata key-value pairs.
fn parse_uproperty_args(attr: &syn::Attribute) -> syn::Result<UPropertyArgs> {
    let meta = attr.parse_meta()?;

    let specifiers = match &meta {
        Meta::Path(_) => {
            // #[uproperty] with no arguments -> C++ resolve defaults.
            return Ok(UPropertyArgs {
                specifier_consts: Vec::new(),
                metadata: Vec::new(),
            });
        }
        Meta::List(list) => &list.nested,
        _ => {
            return Err(syn::Error::new_spanned(
                attr,
                "#[uproperty] expects comma-separated specifiers, e.g. #[uproperty(edit_defaults_only)]",
            ));
        }
    };

    let mut consts: Vec<TokenStream> = Vec::new();
    let mut metadata: Vec<(String, String)> = Vec::new();

    for nested in specifiers {
        match nested {
            NestedMeta::Meta(Meta::Path(path)) => {
                let ident = path
                    .get_ident()
                    .ok_or_else(|| syn::Error::new_spanned(path, "expected a simple identifier"))?;
                let ident_str = ident.to_string();
                let constant = specifier_to_const(&ident_str).ok_or_else(|| {
                    syn::Error::new_spanned(
                        ident,
                        format!("unknown uproperty specifier `{}`", ident_str),
                    )
                })?;
                consts.push(constant);
            }
            NestedMeta::Meta(Meta::NameValue(nv)) => {
                let key = nv.path.get_ident().ok_or_else(|| {
                    syn::Error::new_spanned(
                        &nv.path,
                        "expected a simple identifier for metadata key",
                    )
                })?;
                let value = match &nv.lit {
                    Lit::Str(s) => s.value(),
                    Lit::Int(i) => i.to_string(),
                    Lit::Float(f) => f.to_string(),
                    _ => {
                        return Err(syn::Error::new_spanned(
                            &nv.lit,
                            "metadata value must be a string or number literal",
                        ));
                    }
                };
                let mapped_key = map_metadata_key(&key.to_string());
                metadata.push((mapped_key, value));
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    nested,
                    "expected a specifier identifier (e.g. `edit_anywhere`) or metadata (e.g. `unit = \"CM\"`)",
                ));
            }
        }
    }

    Ok(UPropertyArgs {
        specifier_consts: consts,
        metadata,
    })
}

pub fn uclass_derive(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &ast.ident;
    let class_name_str = struct_name.to_string();

    let fields = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    struct_name,
                    "UClass can only be derived for structs with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                struct_name,
                "UClass can only be derived for structs",
            ));
        }
    };

    let property_registrations: Vec<TokenStream> = fields
        .iter()
        .filter_map(|f| {
            let attr = f.attrs.iter().find(|a| a.path.is_ident("uproperty"))?;
            Some((f, attr))
        })
        .map(|(f, uproperty_attr)| {
            let field_name = f.ident.as_ref().unwrap();
            let field_name_str = field_name.to_string();
            let field_ty = &f.ty;

            let args = parse_uproperty_args(uproperty_attr)?;
            let specifier_consts = &args.specifier_consts;

            let meta_calls: Vec<TokenStream> = args
                .metadata
                .iter()
                .map(|(key, value)| {
                    quote! {
                        ::unreal_api::bindings::rust_plugin::FRustClassDef::set_property_meta(
                            &mut def,
                            ::unreal_api::core_data::FString::from(#field_name_str),
                            ::unreal_api::core_data::FString::from(#key),
                            ::unreal_api::core_data::FString::from(#value),
                        );
                    }
                })
                .collect();

            Ok(quote! {
                {
                    let ty = <#field_ty as ::unreal_api::property::UnrealProperty>::create_property_type();
                    let mut specifiers: ::unreal_api::core_data::TArray<::unreal_api::property::ERustPropertySpecifier> =
                        ::unreal_api::core_data::TArray::new();
                    #(specifiers.add(#specifier_consts);)*
                    ::unreal_api::bindings::rust_plugin::URustExtension_RustClassDef::add_property(
                        &mut def,
                        ::unreal_api::core_data::FString::from(#field_name_str),
                        std::mem::offset_of!(#struct_name, #field_name) as _,
                        ty,
                        &specifiers,
                    );
                    #(#meta_calls)*
                }
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;

    Ok(quote! {
        impl #struct_name {
            #[doc(hidden)]
            pub fn __register_uclass() {
                let mut def = ::unreal_api::bindings::rust_plugin::FRustClassDef::new(
                    ::unreal_api::core_data::FString::from(#class_name_str),
                    std::mem::size_of::<#struct_name>() as _,
                );

                #(#property_registrations)*

                ::unreal_api::bindings::rust_plugin::URustExtension_RustClassDef::register(&def);
            }
        }

        ::unreal_api::inventory::submit! {
            ::unreal_api::registration::UClassRegistration {
                register_fn: #struct_name::__register_uclass,
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn specifier_to_const_known_specifiers() {
        let known = [
            "edit_anywhere",
            "edit_defaults_only",
            "edit_instance_only",
            "visible_anywhere",
            "visible_defaults_only",
            "visible_instance_only",
            "blueprint_read_only",
            "blueprint_read_write",
            "replicated",
            "transient",
            "save_game",
            "config",
            "advanced_display",
            "interp",
        ];
        for spec in known {
            assert!(
                specifier_to_const(spec).is_some(),
                "specifier '{}' should be recognized",
                spec
            );
        }
    }

    #[test]
    fn specifier_to_const_unknown_returns_none() {
        assert!(specifier_to_const("not_a_specifier").is_none());
        assert!(specifier_to_const("").is_none());
        assert!(specifier_to_const("EditAnywhere").is_none()); // wrong case
    }

    #[test]
    fn map_metadata_key_known_keys() {
        assert_eq!(map_metadata_key("unit"), "Units");
        assert_eq!(map_metadata_key("clamp_min"), "ClampMin");
        assert_eq!(map_metadata_key("clamp_max"), "ClampMax");
        assert_eq!(map_metadata_key("display_name"), "DisplayName");
        assert_eq!(map_metadata_key("tooltip"), "Tooltip");
    }

    #[test]
    fn map_metadata_key_unknown_passes_through() {
        assert_eq!(map_metadata_key("custom_key"), "custom_key");
        assert_eq!(map_metadata_key("MyMeta"), "MyMeta");
    }
}
