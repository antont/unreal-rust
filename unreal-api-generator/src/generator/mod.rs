use std::{
    collections::{HashMap, HashSet},
    error::Error,
    path::Path,
};

use heck::{ToShoutySnakeCase, ToSnakeCase};
use proc_macro2::{Literal, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::Ident;

use crate::parse_api::{
    Api, ClassDefinition, DelegateDefinition, EnumDefinition, OpagueDefinition, Property,
    StructDefinition, Type, TypeUsageHint,
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

pub fn generate_classes(api: &Api) -> Result<HashMap<String, Vec<TokenStream>>, Box<dyn Error>> {
    let mut all_classes: HashMap<String, Vec<TokenStream>> = HashMap::new();
    api.classes
        .iter()
        .filter_map(|class_def| generate_class(class_def).ok())
        .for_each(|(module_name, tokens)| {
            all_classes.entry(module_name).or_default().push(tokens);
        });

    Ok(all_classes)
}
pub fn generate_class(
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
    let properties: Vec<TokenStream> = generate_properties(&class_def.properties);
    let q = quote! {
        pub struct #class_name {
            #(
                #properties,
            )*
        }
        // trait #trait_name {
        //
        // }
    };
    Ok((module_name, q))
}

pub fn generate_type(ty: &Type) -> Result<TokenStream, Box<dyn Error>> {
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

            if let Some(TypeUsageHint::UObject) = *usage_hint {
                Ok(quote! {UPtr<#ty_name>})
            } else {
                Ok(ty_name.to_token_stream())
            }
        }
        Type::Container {
            container_type_name,
            inner_type,
        } => {
            let container_ident = format_ident!("{}", container_type_name);
            let inner_tokens = generate_type(inner_type)?;
            let tokens = quote! {
                #container_ident<#inner_tokens>
            };
            Ok(tokens)
        }
        Type::Map {
            key_type,
            value_type,
        } => {
            let key_tokens = generate_type(key_type)?;
            let value_tokens = generate_type(value_type)?;
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
    let trimmed_name = name.trim_start_matches("b_");

    match trimmed_name {
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
        name => name,
    }
}

pub fn generate_properties(properies: &[Property]) -> Vec<TokenStream> {
    let mut bitfield_set: HashSet<u32> = HashSet::new();

    properies
        .iter()
        .filter_map(|property| {
            if bitfield_set.contains(&property.offset) {
                return None;
            }

            let name_ident = if let Type::Bitfield { .. } = property.data_type {
                bitfield_set.insert(property.offset);
                format_ident!("flags_{}", property.offset)
            } else {
                let snake_case_name = property.name.to_snake_case();
                format_ident!("{}", sanitize_parameter_name(&snake_case_name))
            };

            let ty = generate_type(&property.data_type).ok()?;

            let tokens = quote! {
                pub #name_ident: #ty
            };

            Some(tokens)
        })
        .collect()
}

pub fn generate_enum(enum_def: &EnumDefinition) -> Result<TokenStream, Box<dyn Error>> {
    let enum_name = format_ident!("{}", enum_def.enum_name);
    let ty = generate_type(&enum_def.ty)?;

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
    Ok(quote! {
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
    })
}

pub fn generate_enums(api: &Api) -> Result<Vec<TokenStream>, Box<dyn Error>> {
    let all_pods = api
        .enums
        .iter()
        .filter_map(|enum_def| generate_enum(enum_def).ok())
        .collect();

    Ok(all_pods)
}
pub fn generate_delegates(api: &Api) -> Result<Vec<TokenStream>, Box<dyn Error>> {
    Ok(api
        .delegate_defs
        .iter()
        .filter_map(|delgate_def| generate_delegate(delgate_def).ok())
        .collect())
}

pub fn generate_delegate(delegate_def: &DelegateDefinition) -> Result<TokenStream, Box<dyn Error>> {
    let name = format_ident!("{}", delegate_def.name);
    Ok(quote! {
        pub struct #name;
    })
}
pub fn generate_struct(
    struct_def: &StructDefinition,
) -> Result<(String, TokenStream), Box<dyn Error>> {
    let name = format_ident!("{}", struct_def.struct_name);
    let alignment = Literal::u32_unsuffixed(struct_def.min_alignment);

    let properties: Vec<TokenStream> = generate_properties(&struct_def.properties);
    let tokens = quote! {
        #[repr(C, align(#alignment))]
        pub struct #name {
            #(#properties),*
        }
    };

    let module_name = struct_def
        .package
        .split_terminator("/")
        .last()
        .unwrap()
        .to_snake_case();

    Ok((module_name, tokens))
}

pub fn generate_structs(api: &Api) -> Result<HashMap<String, Vec<TokenStream>>, Box<dyn Error>> {
    let mut all_pods: HashMap<String, Vec<TokenStream>> = HashMap::new();
    api.structs
        .iter()
        .filter_map(|struct_def| {
            // if !struct_def.is_plain_old_data {
            //     return None;
            // }

            generate_struct(struct_def).ok()
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
    let mut ctx = Context::new(api);
    ctx.banned_pod_types.insert("FStateTreeAnyEnum".to_string());

    let pods = generate_structs(api)?;
    let classes = generate_classes(api)?;
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

    let enum_defs = generate_enums(api)?;

    modules.retain(|name, _| name == "core_u_object" || name == "engine");

    for (module_name, all_tokens) in &modules {
        let tokens = quote! {
            #![allow(dead_code)]
            #![allow(unused_imports)]
            #![allow(unused_variables)]
            #![allow(non_camel_case_types)]
            pub use super::*;
            pub use super::super::core_data::*;
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
            pub use #used_module_idents::*;
        )*
        pub mod enum_definition;
        pub use enum_definition::*;

        pub mod opague_definitions;
        pub use opague_definitions::*;

        pub mod delegate_definitions;
        pub use delegate_definitions::*;
    };

    let delegates = generate_delegates(api)?;
    let delegate_tokens = quote! {
        #![allow(dead_code)]
        #![allow(unused_imports)]
        #![allow(unused_variables)]
        #![allow(non_camel_case_types)]
        #(
            #delegates
        )*
    };
    let opagues = generate_opagues(api)?;

    let opague_tokens = quote! {
        #(
            #opagues
        )*

    };

    let enum_tokens = quote! {
        #![allow(dead_code)]
        #![allow(unused_imports)]
        #![allow(unused_variables)]
        #![allow(non_camel_case_types)]
        #(#enum_defs)*
    };

    // save_file(&pod_tokens, &out_path.join("actor.rs"));
    save_file(&delegate_tokens, &out_path.join("delegate_definitions.rs"));
    save_file(&enum_tokens, &out_path.join("enum_definition.rs"));
    save_file(&mod_tokens, &out_path.join("mod.rs"));
    save_file(&opague_tokens, &out_path.join("opague_definitions.rs"));
    Ok(())
}
