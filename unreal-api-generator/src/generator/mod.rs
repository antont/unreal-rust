use std::{
    collections::{HashMap, HashSet},
    error::Error,
    path::Path,
};

use heck::ToSnakeCase;
use proc_macro2::{Literal, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::Ident;

use crate::parse_api::{Api, EnumDefinition, Property, StructDefinition, Type};

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

pub fn generate_class(api: &Api) -> TokenStream {
    if let Some(actor) = api
        .classes
        .iter()
        .find(|class_def| class_def.class_name == "AActor")
    {
        let class_name = format_ident!("{}", actor.class_name);
        let trait_name = format_ident!("I{}", &actor.class_name[1..]);

        let q = quote! {
            struct #class_name {

            }
            trait #trait_name {

            }
        };
        return q;
    }
    quote! {}
}

pub fn generate_type(ty: &Type) -> Result<TokenStream, Box<dyn Error>> {
    match ty {
        Type::Concrete { type_name } => {
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
            return Ok(ty_name.to_token_stream());
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
            return Ok(tokens);
        }
        _ => (),
    }

    let tokens = quote! {
        UNKOWN
    };
    Ok(tokens)
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

pub fn generate_property(property: &Property) -> Result<TokenStream, Box<dyn Error>> {
    let ty = generate_type(&property.data_type)?;
    let snake_case_name = property.name.to_snake_case();

    let name_ident = format_ident!("{}", sanitize_parameter_name(&snake_case_name));
    let tokens = quote! {
        pub #name_ident: #ty
    };

    Ok(tokens)
}
pub fn generate_enum(enum_def: &EnumDefinition) -> Result<TokenStream, Box<dyn Error>> {
    let enum_name = format_ident!("{}", enum_def.enum_name);
    let ty = generate_type(&enum_def.ty)?;

    Ok(quote! {
        #[repr(#ty)]
        pub enum #enum_name
        {
            Foo = 0

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
pub fn generate_pod(
    struct_def: &StructDefinition,
) -> Result<(String, TokenStream), Box<dyn Error>> {
    let name = format_ident!("{}", struct_def.struct_name);
    let alignment = Literal::u32_unsuffixed(struct_def.min_alignment);

    let properties: Vec<TokenStream> = struct_def
        .properties
        .iter()
        .filter_map(|prop| generate_property(prop).ok())
        .collect();
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

pub fn generate_pods(api: &Api) -> Result<HashMap<String, Vec<TokenStream>>, Box<dyn Error>> {
    let mut all_pods: HashMap<String, Vec<TokenStream>> = HashMap::new();
    api.structs
        .iter()
        .filter_map(|struct_def| {
            // if !struct_def.is_plain_old_data {
            //     return None;
            // }

            generate_pod(struct_def).ok()
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

    let pods = generate_pods(api)?;
    let mut used_modules: HashSet<String> = HashSet::new();
    for (module_name, structs) in pods {
        let pod_tokens = quote! {
            pub use super::enum_definition::*;
            pub use super::super::core_data::*;
            #(#structs)*
        };
        used_modules.insert(module_name.clone());
        save_file(
            &pod_tokens,
            &out_path.join(module_name).with_added_extension("rs"),
        );
    }
    // let class = generate_class(api);
    let enum_defs = generate_enums(api)?;

    let used_module_idents: Vec<Ident> = used_modules
        .into_iter()
        .map(|name| format_ident!("{}", name))
        .collect();
    let mod_tokens = quote! {
        #(
            pub mod #used_module_idents;
            pub use #used_module_idents::*;
        )*
        pub mod enum_definition;
        pub use enum_definition::*;

    };

    let enum_tokens = quote! {
        #(#enum_defs)*
    };

    // save_file(&pod_tokens, &out_path.join("actor.rs"));
    save_file(&enum_tokens, &out_path.join("enum_definition.rs"));
    save_file(&mod_tokens, &out_path.join("mod.rs"));
    Ok(())
}
