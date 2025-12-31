use std::{
    collections::{HashMap, HashSet},
    error::Error,
    path::Path,
};

use heck::{ToShoutySnakeCase, ToSnakeCase};
use proc_macro2::{Literal, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::Ident;

pub fn module_name_from_package(package: &str) -> String {
    package
        .split_terminator("/")
        .last()
        .unwrap()
        .to_snake_case()
}

pub struct NameResolver {
    pub name_to_module: HashMap<String, String>,
}

impl NameResolver {
    pub fn get_module_from_name(&self, name: &str) -> String {
        self.name_to_module.get(name).cloned().unwrap_or_default()
    }

    pub fn from(api: &Api) -> Self {
        let mut ty_to_module: HashMap<String, String> = HashMap::new();
        for struct_def in &api.structs {
            let module_name = module_name_from_package(&struct_def.package);
            ty_to_module.insert(struct_def.struct_name.clone(), module_name);
        }
        for class_name in &api.classes {
            let module_name = module_name_from_package(&class_name.package);
            ty_to_module.insert(class_name.class_name.clone(), module_name);
        }
        for enum_def in &api.enums {
            let module_name = module_name_from_package(&enum_def.package);
            ty_to_module.insert(enum_def.enum_name.clone(), module_name);
        }

        Self {
            name_to_module: ty_to_module,
        }
    }
}
use crate::parse_api::{
    Api, ClassDefinition, DelegateDefinition, EnumDefinition, OpagueDefinition, Property,
    PropertyFlag, StructDefinition, Type, TypeUsageHint,
};

struct Context {
    banned_pod_types: HashSet<String>,
    structs: HashMap<String, StructDefinition>,
}

impl Context {
    pub fn new(api: &Api) -> Self {
        let structs: HashMap<String, StructDefinition> = api
            .structs
            .iter()
            .map(|struc_def| (struc_def.struct_name.clone(), struc_def.clone()))
            .collect();

        Self {
            banned_pod_types: HashSet::default(),
            structs,
        }
    }
}
pub fn generate_opagues(api: &Api) -> Result<Vec<TokenStream>, Box<dyn Error>> {
    Ok(api
        .opague_defs
        .iter()
        .filter_map(|def| generate_opague(def).ok())
        .collect())
}

pub fn generate_opague(opague_def: &OpagueDefinition) -> Result<TokenStream, Box<dyn Error>> {
    let ident = format_ident!("{}", opague_def.name);
    let alignment = Literal::u32_unsuffixed(opague_def.alignment);
    let size = Literal::u32_unsuffixed(opague_def.size);
    Ok(quote! {
        #[repr(C, align(#alignment))]
        pub struct #ident
        {
            data: [u8; #size]
        }
    })
}

pub fn generate_classes(
    name_resolver: &NameResolver,
    api: &Api,
) -> Result<HashMap<String, Vec<TokenStream>>, Box<dyn Error>> {
    let mut all_classes: HashMap<String, Vec<TokenStream>> = HashMap::new();
    api.classes
        .iter()
        .filter_map(|class_def| generate_class(name_resolver, class_def).ok())
        .for_each(|(module_name, tokens)| {
            all_classes.entry(module_name).or_default().push(tokens);
        });

    Ok(all_classes)
}
pub fn generate_class(
    name_resolver: &NameResolver,
    class_def: &ClassDefinition,
) -> Result<(String, TokenStream), Box<dyn Error>> {
    let class_name = format_ident!("{}", class_def.class_name);
    // let trait_name = format_ident!("I{}", &actor.class_name[1..]);

    let module_name = class_def
        .package
        .split_terminator("/")
        .last()
        .unwrap()
        .to_snake_case();

    let tokens = if class_def.is_interface {
        let interface_name = format_ident!("I{}", class_def.class_name.trim_start_matches("U"));
        quote! {
            pub struct #class_name {
            }
            pub struct #interface_name {
            }
        }
    } else {
        let properties: Vec<TokenStream> =
            generate_properties(&module_name, name_resolver, &class_def.properties);
        quote! {
            pub struct #class_name {
                #(
                    #properties,
                )*
            }
        }
    };
    Ok((module_name, tokens))
}

pub fn generate_type(
    current_module: &str,
    name_resolver: &NameResolver,
    ty: &Type,
) -> Result<TokenStream, Box<dyn Error>> {
    match ty {
        Type::Concrete {
            type_name,
            usage_hint,
        } => {
            let rust_ty_str = match type_name.as_str() {
                "float" => "f32",
                "double" => "f64",
                "uint8" => "u8",
                "uint16" => "u16",
                "uint32" => "u32",
                "uint64" => "u64",
                "int8" => "i8",
                "int16" => "i16",
                "int32" => "i32",
                "int64" => "i64",
                rest => rest,
            };

            let ty_name = format_ident!("{}", rust_ty_str);

            let get_type_ident = |type_name: &str| -> TokenStream {
                if let Some(module) = name_resolver.name_to_module.get(type_name)
                    && module != current_module
                {
                    let module_ident = format_ident!("{}", module);
                    quote! {
                        crate::bindings::#module_ident::#ty_name
                    }
                } else {
                    quote! {
                        #ty_name
                    }
                }
            };

            let ty_tokens = match *usage_hint {
                Some(TypeUsageHint::UObject) => {
                    let ty_ident = get_type_ident(type_name);
                    quote! {UPtr<#ty_ident>}
                }
                Some(TypeUsageHint::ScriptInterface) => {
                    let remapped_ty_name = if type_name.starts_with("I") {
                        format!("U{}", type_name.split_at(1).1)
                    } else {
                        type_name.clone()
                    };
                    let ty_ident = get_type_ident(&remapped_ty_name);
                    quote! {TScriptInterface<#ty_ident>}
                }
                _ => get_type_ident(type_name),
            };

            Ok(ty_tokens)
        }
        Type::Container {
            container_type_name,
            inner_type,
        } => {
            let container_ident = format_ident!("{}", container_type_name);
            let inner_tokens = generate_type(current_module, name_resolver, inner_type)?;
            let tokens = quote! {
                #container_ident<#inner_tokens>
            };
            Ok(tokens)
        }
        Type::Map {
            key_type,
            value_type,
        } => {
            let key_tokens = generate_type(current_module, name_resolver, key_type)?;
            let value_tokens = generate_type(current_module, name_resolver, value_type)?;
            let tokens = quote! {
                TMap<#key_tokens, #value_tokens>
            };
            Ok(tokens)
        }
        Type::Bitfield { .. } => Ok(quote! {
            u8
        }),
    }
}
pub fn sanitize_type_name(name: &str) -> &str {
    let trimmed_name = name.trim_start_matches("b_");

    match trimmed_name {
        "Self" => "Self_",
        name => name,
    }
}

pub fn sanitize_parameter_name(name: &str) -> &str {
    // let trimmed_name = name.trim_start_matches("b_");

    match name {
        "struct" => "_struct",
        "type" => "ty",
        "enum" => "enum_",
        "loop" => "loop_",
        "in" => "in_",
        "match" => "match_",
        "macro" => "macro_",
        "override" => "override_",
        "box" => "box_",
        "async" => "async_",
        "true" => "true_",
        "false" => "false_",
        "virtual" => "virtual_",
        name => name,
    }
}

pub fn generate_properties(
    current_module: &str,
    name_resolver: &NameResolver,
    properies: &[Property],
) -> Vec<TokenStream> {
    let mut bitfield_set: HashSet<u32> = HashSet::new();

    let mut current_offset: u32 = 0;

    properies
        .iter()
        .filter_map(|property| {
            if bitfield_set.contains(&property.offset) {
                return None;
            }
            let padding = property.offset - current_offset;
            let padding_tokens = if padding > 0 {
                let padding_lit = Literal::u32_unsuffixed(padding);
                let padding_ident = format_ident!("__padding_{}", property.offset);
                // quote! {
                //     #[doc(hidden)]
                //     #padding_ident: [u8; #padding_lit],
                // }
                quote! {}
            } else {
                quote! {}
            };
            current_offset = property.offset;

            let name_ident = if let Type::Bitfield { .. } = property.data_type {
                bitfield_set.insert(property.offset);
                format_ident!("flags_{}", property.offset)
            } else {
                let snake_case_name = property.name.to_snake_case();

                let postfix = if property.flags.contains(&PropertyFlag::Deprecated) {
                    "_deprecated"
                } else {
                    ""
                };
                format_ident!("{}{}", sanitize_parameter_name(&snake_case_name), postfix)
            };

            let ty = generate_type(current_module, name_resolver, &property.data_type).ok()?;

            let tokens = quote! {
                #padding_tokens
                pub #name_ident: #ty
            };

            Some(tokens)
        })
        .collect()
}

pub fn generate_enum(
    name_resolver: &NameResolver,
    enum_def: &EnumDefinition,
) -> Result<(String, TokenStream), Box<dyn Error>> {
    let enum_name = format_ident!("{}", enum_def.enum_name);
    let module_name = module_name_from_package(&enum_def.package);
    let ty = generate_type(&module_name, name_resolver, &enum_def.ty)?;

    let mut entry_name_idents = Vec::new();
    let mut entry_value_literals = Vec::new();
    let mut entry_documentation_tokens = Vec::new();

    enum_def.entries.iter().for_each(|entry| {
        let value = Literal::i64_unsuffixed(entry.value);
        let shouty_case = entry.name.to_shouty_snake_case();
        let sanitized_name = sanitize_type_name(&shouty_case);
        let name = if sanitized_name
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_digit())
        {
            format_ident!("N{}", sanitized_name)
        } else {
            format_ident!("{}", sanitized_name)
        };

        entry_name_idents.push(name);
        entry_value_literals.push(value);

        // TODO: Need to sanitize the comments as well
        // let doc_tokens = if let Some(doc) = &entry.documentation {
        //     // quote! {
        //     //     #[doc = #doc]
        //     // }
        // } else {
        //     quote! {}
        // };

        entry_documentation_tokens.push(quote! {});
    });

    // TODO: Some of this could be enums but unreal has no information of which are bitflags, so
    // for now we just generate bitflags for every enum
    let tokens = quote! {
        #[allow(non_camel_case_types)]
        #[repr(transparent)]
        pub struct #enum_name(pub #ty);
        impl #enum_name
        {
            #(
                #entry_documentation_tokens
                pub const #entry_name_idents: #enum_name = #enum_name(#entry_value_literals);
            )*
        }
    };
    Ok((module_name, tokens))
}

pub fn generate_enums(
    name_resolver: &NameResolver,
    api: &Api,
) -> Result<HashMap<String, Vec<TokenStream>>, Box<dyn Error>> {
    let mut all_enums: HashMap<String, Vec<TokenStream>> = HashMap::new();
    api.enums
        .iter()
        .filter_map(|enum_def| generate_enum(name_resolver, enum_def).ok())
        .for_each(|(module_name, tokens)| {
            all_enums.entry(module_name).or_default().push(tokens);
        });

    Ok(all_enums)
}
pub fn generate_delegates(api: &Api) -> Result<HashMap<String, Vec<TokenStream>>, Box<dyn Error>> {
    let mut all_delegates: HashMap<String, Vec<TokenStream>> = HashMap::new();
    api.delegate_defs
        .iter()
        .filter_map(|delegate_def| generate_delegate(delegate_def).ok())
        .for_each(|(module_name, tokens)| {
            all_delegates.entry(module_name).or_default().push(tokens);
        });

    Ok(all_delegates)
}

pub fn generate_delegate(
    delegate_def: &DelegateDefinition,
) -> Result<(String, TokenStream), Box<dyn Error>> {
    let name = format_ident!("{}", delegate_def.name);
    let module_name = module_name_from_package(&delegate_def.package);
    Ok((
        module_name,
        quote! {
            pub struct #name;
        },
    ))
}
pub fn generate_struct(
    name_resolver: &NameResolver,
    struct_def: &StructDefinition,
) -> Result<(String, TokenStream), Box<dyn Error>> {
    let name = format_ident!("{}", struct_def.struct_name);
    let alignment = Literal::u32_unsuffixed(struct_def.min_alignment);
    let module_name = struct_def
        .package
        .split_terminator("/")
        .last()
        .unwrap()
        .to_snake_case();

    let properties: Vec<TokenStream> =
        generate_properties(&module_name, name_resolver, &struct_def.properties);
    let tokens = quote! {
        #[repr(C, align(#alignment))]
        pub struct #name {
            #(#properties),*
        }
    };

    Ok((module_name, tokens))
}

pub fn generate_structs(
    name_resolver: &NameResolver,
    api: &Api,
) -> Result<HashMap<String, Vec<TokenStream>>, Box<dyn Error>> {
    let mut all_pods: HashMap<String, Vec<TokenStream>> = HashMap::new();
    api.structs
        .iter()
        .filter_map(|struct_def| {
            // if !struct_def.is_plain_old_data {
            //     return None;
            // }

            generate_struct(name_resolver, struct_def).ok()
        })
        .for_each(|(module_name, tokens)| {
            all_pods.entry(module_name).or_default().push(tokens);
        });

    Ok(all_pods)
}

pub fn save_file(tokens: &TokenStream, path: &Path) {
    let output = match syn::parse_file(&tokens.to_string()) {
        Ok(syntax_tree) => prettyplease::unparse(&syntax_tree),
        Err(err) => {
            println!("{} {}", path.display(), err);
            tokens.to_string()
        }
    };
    std::fs::write(path, output);
}

pub fn generate_crate(api: &Api, out_path: &Path) -> Result<(), Box<dyn Error>> {
    let name_resolver = NameResolver::from(api);

    let mut ctx = Context::new(api);
    ctx.banned_pod_types.insert("FStateTreeAnyEnum".to_string());

    let pods = generate_structs(&name_resolver, api)?;
    let classes = generate_classes(&name_resolver, api)?;
    let delegates = generate_delegates(api)?;
    let enum_defs = generate_enums(&name_resolver, api)?;
    let mut modules: HashMap<String, Vec<TokenStream>> = HashMap::new();

    for (module_name, structs) in pods {
        let struct_tokens = quote! {
            #(#structs)*
        };
        modules
            .entry(module_name.clone())
            .or_default()
            .push(struct_tokens);
    }
    for (module_name, classes) in classes {
        let class_tokens = quote! {
            #(#classes)*
        };
        modules
            .entry(module_name.clone())
            .or_default()
            .push(class_tokens);
    }

    for (module_name, delegate_token_vec) in delegates {
        let delegate_tokens = quote! {
            #(#delegate_token_vec)*
        };
        modules
            .entry(module_name.clone())
            .or_default()
            .push(delegate_tokens);
    }

    for (module_name, enum_token_vec) in enum_defs {
        let enum_tokens = quote! {
            #(#enum_token_vec)*
        };
        modules
            .entry(module_name.clone())
            .or_default()
            .push(enum_tokens);
    }

    let opagues = generate_opagues(api)?;
    let opague_tokens = quote! {
        #(
            #opagues
        )*

    };

    for (module_name, all_tokens) in &modules {
        let tokens = quote! {
            #![allow(dead_code)]
            #![allow(unused_imports)]
            #![allow(unused_variables)]
            #![allow(non_camel_case_types)]
            pub use crate::bindings::opague_definitions::*;
            pub use crate::core_data::*;
            #(#all_tokens)*
        };

        save_file(
            &tokens,
            &out_path.join(module_name).with_added_extension("rs"),
        );
    }

    let used_module_idents: Vec<Ident> = modules
        .keys()
        .map(|name| format_ident!("{}", name))
        .collect();

    let mod_tokens = quote! {
        #(
            pub mod #used_module_idents;
        )*

        pub mod opague_definitions;
    };

    save_file(&mod_tokens, &out_path.join("mod.rs"));
    save_file(&opague_tokens, &out_path.join("opague_definitions.rs"));
    Ok(())
}
