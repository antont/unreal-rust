use std::{error::Error, path::Path};

use heck::ToSnakeCase;
use proc_macro2::{Literal, TokenStream};
use quote::{ToTokens, format_ident, quote};

use crate::parse_api::{Api, EnumDefinition, Property, StructDefinition, Type};

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
    if let Type::Concrete { type_name } = ty {
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

    let tokens = quote! {
        UNKOWN
    };
    Ok(tokens)
}

pub fn generate_property(property: &Property) -> Result<TokenStream, Box<dyn Error>> {
    let ty = generate_type(&property.data_type)?;
    let name = format_ident!("{}", property.name.to_snake_case().trim_start_matches("b_"));
    let tokens = quote! {
        pub #name: #ty
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
pub fn generate_pod(struct_def: &StructDefinition) -> Result<TokenStream, Box<dyn Error>> {
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

    Ok(tokens)
}

pub fn generate_pods(api: &Api) -> Result<Vec<TokenStream>, Box<dyn Error>> {
    let all_pods = api
        .structs
        .iter()
        .filter_map(|struct_def| {
            if !struct_def.is_plain_old_data {
                return None;
            }

            generate_pod(struct_def).ok()
        })
        .collect();

    Ok(all_pods)
}

pub fn save_file(tokens: &TokenStream, path: &Path) {
    let output = if let Ok(syntax_tree) = syn::parse_file(&tokens.to_string()) {
        prettyplease::unparse(&syntax_tree)
    } else {
        tokens.to_string()
    };
    std::fs::write(path, output);
}

pub fn generate_crate(api: &Api, out_path: &Path) -> Result<(), Box<dyn Error>> {
    let pods = generate_pods(api)?;
    let class = generate_class(api);
    let pod_tokens = quote! {
        pub use super::enum_definition::*;
        #class
        #(#pods)*
    };
    let enum_defs = generate_enums(api)?;

    let enum_tokens = quote! {
        #(#enum_defs)*
    };

    save_file(&pod_tokens, &out_path.join("actor.rs"));
    save_file(&enum_tokens, &out_path.join("enum_definition.rs"));
    Ok(())
}
