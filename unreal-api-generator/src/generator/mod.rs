use std::{
    collections::{HashMap, HashSet},
    error::Error,
    path::Path,
};

use heck::{ToShoutySnakeCase, ToSnakeCase};
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use serde_json::map::Iter;
use syn::{
    Ident,
    token::{Else, Token},
};

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
    fn get_type_ident(&self, current_module: Option<&str>, type_name: &str) -> TokenStream {
        let ty_ident = format_ident!("{}", type_name);
        if let Some(module) = self.name_to_module.get(type_name)
            && current_module.map_or(true, |current_module| current_module != module)
        {
            let module_ident = format_ident!("{}", module);
            quote! {
                crate::bindings::#module_ident::#ty_ident
            }
        } else {
            quote! {
                #ty_ident
            }
        }
    }

    pub fn from(api: &Api) -> Self {
        let mut ty_to_module: HashMap<String, String> = HashMap::new();
        for struct_def in api.iter_structs() {
            let module_name = module_name_from_package(&struct_def.package);
            ty_to_module.insert(struct_def.struct_name.clone(), module_name);
        }
        for class_name in api.iter_classes() {
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

pub struct Context {
    name_resolver: NameResolver,
}

pub fn iter_properties(properties: &[Property]) -> impl Iterator<Item = &Property> {
    properties.iter().filter(|p| {
        p.flags.contains(&PropertyFlag::BlueprintReadOnly)
            || p.flags.contains(&PropertyFlag::BlueprintReadWrite)
    })
}

impl Context {
    pub fn new(api: &Api) -> Self {
        Self {
            name_resolver: NameResolver::from(api),
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
pub fn generate_layout_tests(ctx: &Context, api: &Api) -> TokenStream {
    let mut tokens: Vec<TokenStream> = Vec::new();

    let class_tests = api
        .iter_classes()
        .map(|def| generate_layout_test(ctx, &def.class_name, &def.properties));
    tokens.extend(class_tests);

    let struct_tests = api
        .iter_structs()
        .map(|def| generate_layout_test(ctx, &def.struct_name, &def.properties));
    tokens.extend(struct_tests);

    quote! {
        #(
            #tokens
        )*
    }
}

pub fn generate_classes(
    ctx: &Context,
    api: &Api,
) -> Result<HashMap<String, Vec<TokenStream>>, Box<dyn Error>> {
    let mut all_classes: HashMap<String, Vec<TokenStream>> = HashMap::new();
    api.iter_classes()
        .filter_map(|class_def| generate_class(ctx, class_def).ok())
        .for_each(|(module_name, tokens)| {
            all_classes.entry(module_name).or_default().push(tokens);
        });

    Ok(all_classes)
}

pub fn generate_layout_test(ctx: &Context, name: &str, properties: &[Property]) -> TokenStream {
    let ty_ident = ctx.name_resolver.get_type_ident(None, name);
    let property_tokens: Vec<TokenStream> = iter_properties(properties)
        .filter_map(|property| {
            // TODO: Verify bitfields
            if let Type::Bitfield { .. } = &property.data_type {
                return None;
            }

            let field_name = property_name(property);
            let field_ident = format_ident!("{}", field_name);
            let offset = property.offset as usize;
            let q = quote! {
                assert_eq!(std::mem::offset_of!(#ty_ident, #field_ident), #offset, #field_name);
            };
            Some(q)
        })
        .collect();

    let fn_ident = format_ident!("verify_layout_{}", name);
    quote! {
        #[test]
        #[allow(non_snake_case)]
        fn #fn_ident()
        {
            #(
                #property_tokens
            )*
        }
    }
}
pub fn generate_layout_check(name: &str, properties: &[Property]) -> TokenStream {
    let property_tokens: Vec<TokenStream> = iter_properties(properties)
        .filter_map(|property| {
            // TODO: Verify bitfields
            if let Type::Bitfield { .. } = &property.data_type {
                return None;
            }

            let ty_ident = format_ident!("{}", name);
            let field_name = property_name(property);
            let field_ident = format_ident!("{}", field_name);
            let offset = property.offset as usize;
            // let q = quote! {
            //     assert_eq!(std::mem::offset_of!(#ty_ident, #field_ident), #offset, #field_name);
            // };
            let q = quote! {
                log::warn!("{} = {} vs {}", #field_name, std::mem::offset_of!(#ty_ident, #field_ident), #offset);
            };
            Some(q)
        })
        .collect();

    quote! {
        pub fn verify_layout()
        {
            // const _: () = {
            //     #(
            //         #property_tokens
            //     )*
            // };
            #(
                #property_tokens
            )*


        }
    }
}
pub fn generate_class(
    ctx: &Context,
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
        let alignment = Literal::u32_unsuffixed(class_def.min_alignment);

        let PropertyInfo { tokens, offset } =
            generate_properties(&module_name, ctx, &class_def.properties);

        let end_padding = if offset < class_def.property_sizes as usize {
            let padding_size =
                Literal::usize_unsuffixed(class_def.property_sizes as usize - offset);
            Some(quote! {
                __padding_end: [u8; #padding_size]
            })
        } else {
            None
        };

        let class_def_tokens = quote! {
            #[repr(C, align(#alignment))]
            pub struct #class_name {
                #(
                    #tokens,
                )*
                #end_padding
            }
        };

        let allow_list = [
            "AActor",
            "UCharacterMovementComponent",
            "USceneComponent",
            "UWorldPartition",
            "UStaticMesh",
            "UWidget",
            "UVolumetricCloudComponent"
        ];
        let layout_check_tokens = if allow_list.contains(&class_def.class_name.as_str()) {
            Some(generate_layout_check(
                &class_def.class_name,
                &class_def.properties,
            ))
        } else {
            None
        };

        quote! {
            #class_def_tokens
            impl #class_name
            {
                #layout_check_tokens
            }
        }
    };
    Ok((module_name, tokens))
}

pub fn generate_type(
    current_module: &str,
    ctx: &Context,
    ty: &Type,
) -> Result<TokenStream, Box<dyn Error>> {
    let inner_tokens = match ty {
        Type::Concrete {
            type_name,
            usage_hint,
            ..
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

            let ty_tokens = match *usage_hint {
                Some(TypeUsageHint::UObject) => {
                    let ty_ident = ctx
                        .name_resolver
                        .get_type_ident(Some(current_module), rust_ty_str);
                    quote! {UPtr<#ty_ident>}
                }
                Some(TypeUsageHint::ScriptInterface) => {
                    let remapped_ty_name = if rust_ty_str.starts_with("I") {
                        format!("U{}", rust_ty_str.split_at(1).1)
                    } else {
                        type_name.clone()
                    };
                    let ty_ident = ctx
                        .name_resolver
                        .get_type_ident(Some(current_module), &remapped_ty_name);
                    quote! {TScriptInterface<#ty_ident>}
                }
                _ => ctx
                    .name_resolver
                    .get_type_ident(Some(current_module), rust_ty_str),
            };

            ty_tokens
        }
        Type::Container {
            container_type_name,
            inner_type,
            ..
        } => {
            let container_ident = format_ident!("{}", container_type_name);
            let inner_tokens = generate_type(current_module, ctx, inner_type)?;
            let tokens = quote! {
                #container_ident<#inner_tokens>
            };
            tokens
        }
        Type::Map {
            key_type,
            value_type,
            ..
        } => {
            let key_tokens = generate_type(current_module, ctx, key_type)?;
            let value_tokens = generate_type(current_module, ctx, value_type)?;
            let tokens = quote! {
                TMap<#key_tokens, #value_tokens>
            };
            tokens
        }
        Type::Bitfield { .. } => quote! {
            u8
        },
    };

    let array_dim = match ty {
        Type::Concrete { array_dim, .. } => *array_dim,
        Type::Container { array_dim, .. } => *array_dim,
        Type::Map { array_dim, .. } => *array_dim,
        Type::Bitfield { .. } => 0,
    };

    let tokens = if array_dim > 1 {
        let array_dim_lit = Literal::usize_unsuffixed(array_dim);
        quote! {
            [#inner_tokens; #array_dim_lit]
        }
    } else {
        inner_tokens
    };

    return Ok(tokens);
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
fn align_up(offset: u32, align: u32) -> u32 {
    (offset + (align - 1)) & !(align - 1)
}

pub struct PropertyInfo {
    pub tokens: Vec<TokenStream>,
    pub offset: usize,
}

pub fn generate_properties(
    current_module: &str,
    ctx: &Context,
    properties: &[Property],
) -> PropertyInfo {
    let mut bitfield_set: HashSet<u32> = HashSet::new();

    let mut current_offset: u32 = 0;

    let tokens: Vec<_> = iter_properties(properties)
        .filter_map(|property| {
            if bitfield_set.contains(&property.offset) {
                return None;
            }

            let padding = property.offset - align_up(current_offset, property.alignment);
            let padding_tokens = if padding > 0 {
                let padding_lit = Literal::u32_unsuffixed(padding);
                let padding_ident = format_ident!("__padding_{}", property.offset);
                quote! {
                    #[doc(hidden)]
                    #padding_ident: [u8; #padding_lit],
                }
            } else {
                quote! {}
            };
            current_offset = property.offset + property.size;

            let name_ident = if let Type::Bitfield { .. } = property.data_type {
                bitfield_set.insert(property.offset);
                format_ident!("flags_{}", property.offset)
            } else {
                format_ident!("{}", property_name(property))
            };

            let ty = generate_type(current_module, ctx, &property.data_type).ok()?;

            let tokens = quote! {
                #padding_tokens
                pub #name_ident: #ty
            };

            Some(tokens)
        })
        .collect();

    PropertyInfo {
        tokens,
        offset: current_offset as usize,
    }
}

fn property_name(property: &Property) -> String {
    let snake_case_name = property.name.to_snake_case();

    let postfix = if property.flags.contains(&PropertyFlag::Deprecated) {
        "_deprecated"
    } else {
        ""
    };
    format!("{}{}", sanitize_parameter_name(&snake_case_name), postfix)
}

pub fn generate_enum(
    ctx: &Context,
    enum_def: &EnumDefinition,
) -> Result<(String, TokenStream), Box<dyn Error>> {
    let enum_name = format_ident!("{}", enum_def.enum_name);
    let module_name = module_name_from_package(&enum_def.package);
    let ty = generate_type(&module_name, ctx, &enum_def.ty)?;

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
    ctx: &Context,
    api: &Api,
) -> Result<HashMap<String, Vec<TokenStream>>, Box<dyn Error>> {
    let mut all_enums: HashMap<String, Vec<TokenStream>> = HashMap::new();
    api.enums
        .iter()
        .filter_map(|enum_def| generate_enum(ctx, enum_def).ok())
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
    let alignment = Literal::u32_unsuffixed(delegate_def.alignment);
    let size = Literal::u32_unsuffixed(delegate_def.size);
    Ok((
        module_name,
        quote! {
            #[repr(C, align(#alignment))]
            pub struct #name
            {
                _opague: [u8; #size]
            }
        },
    ))
}
pub fn generate_struct(
    ctx: &Context,
    struct_def: &StructDefinition,
) -> Result<(String, TokenStream), Box<dyn Error>> {
    let struct_ident = format_ident!("{}", struct_def.struct_name);
    let alignment = Literal::u32_unsuffixed(struct_def.min_alignment);
    let module_name = struct_def
        .package
        .split_terminator("/")
        .last()
        .unwrap()
        .to_snake_case();

    let PropertyInfo { tokens, offset } =
        generate_properties(&module_name, ctx, &struct_def.properties);

    let end_padding = if offset < struct_def.property_sizes as usize {
        let padding_size = Literal::usize_unsuffixed(struct_def.property_sizes as usize - offset);
        Some(quote! {
            __padding_end: [u8; #padding_size]
        })
    } else {
        None
    };

    let struct_def_tokens = quote! {
        #[repr(C, align(#alignment))]
        pub struct #struct_ident {
            #(
                #tokens,
            )*
            #end_padding
        }
    };

    let allow_list = ["FHitResult", "FBodyInstance"];
    let layout_check_tokens = if allow_list.contains(&struct_def.struct_name.as_str()) {
        Some(generate_layout_check(
            &struct_def.struct_name,
            &struct_def.properties,
        ))
    } else {
        None
    };

    let tokens = quote! {
        #struct_def_tokens
        impl #struct_ident
        {
            #layout_check_tokens
        }
    };

    Ok((module_name, tokens))
}

pub fn generate_structs(
    ctx: &Context,
    api: &Api,
) -> Result<HashMap<String, Vec<TokenStream>>, Box<dyn Error>> {
    let mut all_pods: HashMap<String, Vec<TokenStream>> = HashMap::new();
    api.iter_structs()
        .filter_map(|struct_def| generate_struct(ctx, struct_def).ok())
        .for_each(|(module_name, tokens)| {
            all_pods.entry(module_name).or_default().push(tokens);
        });

    Ok(all_pods)
}

pub fn save_file(tokens: &TokenStream, path: &Path) {
    let generated_output = match syn::parse_file(&tokens.to_string()) {
        Ok(syntax_tree) => prettyplease::unparse(&syntax_tree),
        Err(err) => {
            println!("{} {}", path.display(), err);
            tokens.to_string()
        }
    };
    // TODO: Can probably be sped up with parsing hases but this works for now
    // we don't want to resave the file if the output is exactly the same, this slows rust_analyzer
    // down extemely as we would resave 500 modules
    if let Ok(current_content) = std::fs::read_to_string(path)
        && current_content == generated_output
    {
        return;
    }
    let _ = std::fs::write(path, generated_output);
}

pub fn generate_crate(api: &Api, out_path: &Path) -> Result<(), Box<dyn Error>> {
    let ctx = Context::new(api);

    let pods = generate_structs(&ctx, api)?;
    let classes = generate_classes(&ctx, api)?;
    let delegates = generate_delegates(api)?;
    let enum_defs = generate_enums(&ctx, api)?;
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

    let mod_tests = generate_layout_tests(&ctx, api);
    let mod_tokens = quote! {
        #(
            pub mod #used_module_idents;
        )*

        pub mod opague_definitions;

        #[cfg(test)]
        mod tests;
        
    };

    save_file(&mod_tests, &out_path.join("tests.rs"));
    save_file(&mod_tokens, &out_path.join("mod.rs"));
    save_file(&opague_tokens, &out_path.join("opague_definitions.rs"));
    Ok(())
}
