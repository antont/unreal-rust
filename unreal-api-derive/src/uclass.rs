use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, NestedMeta};

/// Maps a specifier identifier to its flag constant path token stream.
/// Returns the category ("edit", "blueprint", or "modifier") alongside the path.
fn specifier_to_flag(specifier: &str) -> Option<(&'static str, TokenStream)> {
    let (category, path) = match specifier {
        // Edit scope (mutually exclusive)
        "edit_anywhere" => ("edit", quote! { ::unreal_api::property::flags::EDIT_ANYWHERE }),
        "edit_defaults_only" => {
            ("edit", quote! { ::unreal_api::property::flags::EDIT_DEFAULTS_ONLY })
        }
        "edit_instance_only" => {
            ("edit", quote! { ::unreal_api::property::flags::EDIT_INSTANCE_ONLY })
        }
        "visible_anywhere" => {
            ("edit", quote! { ::unreal_api::property::flags::VISIBLE_ANYWHERE })
        }
        "visible_defaults_only" => {
            ("edit", quote! { ::unreal_api::property::flags::VISIBLE_DEFAULTS_ONLY })
        }
        "visible_instance_only" => {
            ("edit", quote! { ::unreal_api::property::flags::VISIBLE_INSTANCE_ONLY })
        }
        // Blueprint access (mutually exclusive)
        "blueprint_read_only" => (
            "blueprint",
            quote! { ::unreal_api::property::flags::BLUEPRINT_READ_ONLY_ACCESS },
        ),
        "blueprint_read_write" => {
            ("blueprint", quote! { ::unreal_api::property::flags::BLUEPRINT_READ_WRITE })
        }
        // Modifiers (additive)
        "replicated" => ("modifier", quote! { ::unreal_api::property::flags::REPLICATED }),
        "transient" => ("modifier", quote! { ::unreal_api::property::flags::TRANSIENT }),
        "save_game" => ("modifier", quote! { ::unreal_api::property::flags::SAVE_GAME }),
        "config" => ("modifier", quote! { ::unreal_api::property::flags::CONFIG }),
        "advanced_display" => {
            ("modifier", quote! { ::unreal_api::property::flags::ADVANCED_DISPLAY })
        }
        "interp" => ("modifier", quote! { ::unreal_api::property::flags::INTERP }),
        _ => return None,
    };
    Some((category, path))
}

/// Parses `#[uproperty(...)]` arguments into a flags expression token stream.
fn parse_uproperty_flags(attr: &syn::Attribute) -> syn::Result<TokenStream> {
    let meta = attr.parse_meta()?;

    let specifiers = match &meta {
        Meta::Path(_) => {
            // #[uproperty] with no arguments → defaults
            return Ok(
                quote! { (::unreal_api::property::flags::EDIT_ANYWHERE | ::unreal_api::property::flags::BLUEPRINT_READ_WRITE) as i64 },
            );
        }
        Meta::List(list) => &list.nested,
        _ => {
            return Err(syn::Error::new_spanned(
                attr,
                "#[uproperty] expects comma-separated specifiers, e.g. #[uproperty(edit_defaults_only)]",
            ));
        }
    };

    let mut flag_tokens: Vec<TokenStream> = Vec::new();
    let mut has_edit = false;
    let mut has_blueprint = false;

    for nested in specifiers {
        match nested {
            NestedMeta::Meta(Meta::Path(path)) => {
                let ident = path.get_ident().ok_or_else(|| {
                    syn::Error::new_spanned(path, "expected a simple identifier")
                })?;
                let ident_str = ident.to_string();
                let (category, flag_path) = specifier_to_flag(&ident_str).ok_or_else(|| {
                    syn::Error::new_spanned(
                        ident,
                        format!("unknown uproperty specifier `{}`", ident_str),
                    )
                })?;
                match category {
                    "edit" => has_edit = true,
                    "blueprint" => has_blueprint = true,
                    _ => {}
                }
                flag_tokens.push(flag_path);
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    nested,
                    "expected a specifier identifier (e.g. `edit_anywhere`, `replicated`)",
                ));
            }
        }
    }

    // Apply defaults for missing categories
    if !has_edit {
        flag_tokens.push(quote! { ::unreal_api::property::flags::EDIT_ANYWHERE });
    }
    if !has_blueprint {
        flag_tokens.push(quote! { ::unreal_api::property::flags::BLUEPRINT_READ_WRITE });
    }

    Ok(quote! { (#(#flag_tokens)|*) as i64 })
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
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                struct_name,
                "UClass can only be derived for structs",
            ))
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

            let flags_expr = parse_uproperty_flags(uproperty_attr)?;

            Ok(quote! {
                {
                    let ty = <#field_ty as ::unreal_api::property::UnrealProperty>::create_property_type();
                    ::unreal_api::bindings::rust_plugin::URustExtension_RustClassDef::add_property(
                        &mut def,
                        ::unreal_api::core_data::FString::from(#field_name_str),
                        std::mem::offset_of!(#struct_name, #field_name) as _,
                        ty,
                        #flags_expr,
                    );
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
