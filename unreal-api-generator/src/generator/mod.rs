use std::{
    collections::{HashMap, HashSet},
    error::Error,
    hash::Hash,
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
            && current_module.map_or_else(|| true, |current_module| current_module != module)
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
    Api, ClassDefinition, DelegateDefinition, EnumDefinition, Function, FunctionFlag,
    OpagueDefinition, Property, PropertyFlag, StructDefinition, Type, TypeUsageHint,
};

pub struct ExtensionFns {
    pub origin_class: String,
    pub functions: Vec<Function>,
}

pub struct Context {
    name_resolver: NameResolver,
    extension_fns: HashMap<String, Vec<String>>,
    name_to_class: HashMap<String, ClassDefinition>,
}

impl Context {
    pub fn new(api: &Api) -> Self {
        let mut extension_fns: HashMap<String, Vec<String>> = HashMap::new();
        let name_to_class: HashMap<String, ClassDefinition> = api
            .classes
            .iter()
            .map(|def| (def.class_name.clone(), def.clone()))
            .collect();

        for class in &api.classes {
            let Some(name) = class.meta.get("Impl") else {
                continue;
            };

            let extension_fns = extension_fns.entry(name.clone()).or_default();
            extension_fns.push(class.class_name.clone());
        }

        Self {
            extension_fns,
            name_to_class,
            name_resolver: NameResolver::from(api),
        }
    }
}

pub fn iter_properties(properties: &[Property]) -> impl Iterator<Item = &Property> {
    properties
        .iter()
        .filter(|p| p.flags.contains(&PropertyFlag::BlueprintVisible))
}
pub fn generate_module_globals(api: &Api) -> HashMap<String, TokenStream> {
    let mut module_to_class = HashMap::<String, Vec<&ClassDefinition>>::new();
    for def in &api.classes {
        let module_name = module_name_from_package(&def.package);
        module_to_class
            .entry(module_name.clone())
            .or_default()
            .push(def);
    }

    // let mut module_to_struct = HashMap::new();
    // for def in &api.structs {
    //     let module_name = module_name_from_package(&def.package);
    //     module_to_struct.insert(module_name, def);
    // }

    fn convert_to_fn_name(name: &str, f: &Function) -> Ident {
        format_ident!(
            "{}_{}",
            name.to_shouty_snake_case(),
            f.function_name.to_shouty_snake_case()
        )
    }

    module_to_class
        .into_iter()
        .map(|(module_name, defs)| {
            let all_fn_pts = defs.iter()
                .flat_map(|&def| {
                def.functions.iter().map(|f| {
                    let ident = convert_to_fn_name(&def.class_name, f);
                    quote! {
                        #[doc(hidden)]
                        pub static mut #ident: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
                    }
                })
            });

            let initialize_all_fn_ptrs = defs.iter().flat_map(|&def| {
                let name = format_ident!("{}", def.class_name);

                let initialize_ptrs_scope: Vec<TokenStream> = def.functions.iter().map(|f| {
                    let fn_name = &f.function_name;
                    let ptr_ident = convert_to_fn_name(&def.class_name, f);
                    quote! {
                        (bindings
                            .core_fns
                            .find_function_by_name)(
                            class_ptr,
                            unreal_ffi::Utf8Str::from(#fn_name),
                            &raw mut #ptr_ident,
                        );
                    }
                }).collect();

                if initialize_ptrs_scope.is_empty()
                {
                    return None;

                }

                Some(
                    quote! {
                        unsafe {
                            let bindings = crate::module::bindings();
                            let class_ptr = #name::static_class();

                            #(
                                #initialize_ptrs_scope
                            )*
                        }
                    }
                )
            });

            let q = quote! {
                #(
                    #all_fn_pts
                )*
                pub fn initialize()
                {
                    #(
                        #initialize_all_fn_ptrs
                    )*

                }
            };
            (module_name, q)
        })
        .collect()
}

pub fn generate_globals(api: &Api) -> TokenStream {
    let class_db = quote! {
        pub struct ClassPtrDB{
            pub name_to_ptr: std::collections::HashMap<String, *mut crate::ffi::UObjectOpague>,
        }
        pub static CLASS_PTRS: std::sync::OnceLock<ClassPtrDB> = std::sync::OnceLock::new();

        unsafe impl Sync for ClassPtrDB {}
        unsafe impl Send for ClassPtrDB {}

        impl ClassPtrDB {
            pub fn from(bindings: &'static crate::ffi::UnrealBindings) -> Self {
                let mut name_to_ptr = std::collections::HashMap::new();

                let mut classes_alloc = crate::ffi::RustAlloc::empty();
                unsafe {
                    (bindings.core_fns.get_all_uclasses)(&mut classes_alloc);
                }

                let ptr = classes_alloc.ptr.cast::<*mut crate::ffi::UObjectOpague>();
                let len = classes_alloc.size / std::mem::size_of::<*mut crate::ffi::UObjectOpague>();

                let class_slice = unsafe { std::slice::from_raw_parts(ptr, len) };

                for &class_ptr in class_slice {
                    let mut str_alloc = crate::ffi::StrRustAlloc::empty();
                    unsafe {
                        (bindings.core_fns.get_class_name)(class_ptr, &mut str_alloc);
                    }
                    let class_name = str_alloc.into_string();
                    name_to_ptr.insert(class_name, class_ptr);
                }

                Self { name_to_ptr }
            }
        }
    };

    let struct_ptr_tokens = api.structs.iter().map(|def| {
        let name = format_ident!("{}", def.struct_name);
        quote! {
            #name: *mut crate::ffi::UObjectOpague
        }
    });

    let struct_db = quote! {
        pub struct StructPtrDB{
            pub name_to_ptr: std::collections::HashMap<String, *mut crate::ffi::UObjectOpague>,
        }
    };

    let modules: HashSet<String> = api
        .classes
        .iter()
        .map(|def| module_name_from_package(&def.package))
        .collect();

    let initialize_modules_tokens = modules.iter().map(|module| {
        let module_ident = format_ident!("{}", module);
        quote! {
            crate::bindings::#module_ident::initialize();
        }
    });

    quote! {
        #![allow(unused)]
        #![allow(non_snake_case)]
        #struct_db

        #class_db

        pub fn initialize_modules()
        {
            #(
               #initialize_modules_tokens
            )*
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
        .map(|def| generate_layout_test(ctx, &def.class_name, def.property_sizes, &def.properties));
    tokens.extend(class_tests);

    let struct_tests = api.iter_structs().map(|def| {
        generate_layout_test(ctx, &def.struct_name, def.property_sizes, &def.properties)
    });
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

pub fn generate_layout_test(
    ctx: &Context,
    name: &str,
    size: u32,
    properties: &[Property],
) -> TokenStream {
    let ty_ident = ctx.name_resolver.get_type_ident(None, name);
    let size_lit = Literal::u32_unsuffixed(size);
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
            assert_eq!(std::mem::size_of::<#ty_ident>(), #size_lit);

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

pub fn generate_function(
    ctx: &Context,
    ty_name: &str,
    module: &str,
    function: &Function,
) -> TokenStream {
    let fn_name = function
        .meta
        .get("ScriptName")
        .unwrap_or(&function.function_name);
    let fn_ident = format_ident!("{}", fn_name.to_snake_case());

    let return_ty = function.parameters.last().and_then(|param| {
        if param.property.name != "ReturnValue" {
            return None;
        }

        let ty = generate_type(module, ctx, &param.property.data_type).ok()?;

        Some(quote! {
                -> #ty
        })
    });

    let param_tokens: Vec<_> = function
        .parameters
        .iter()
        .filter_map(|param| {
            if param.property.flags.contains(&PropertyFlag::ReturnParm) {
                return None;
            }

            let name = format_ident!("{}", property_name(&param.property));
            let ty = generate_type(module, ctx, &param.property.data_type).ok()?;
            let reference = if param.property.flags.contains(&PropertyFlag::OutParm) {
                if param.property.flags.contains(&PropertyFlag::ConstParm) {
                    Some(quote! {
                        &
                    })
                } else {
                    Some(quote! {
                        &mut
                    })
                }
            } else {
                None
            };
            Some(quote! {
                #name: #reference #ty
            })
        })
        .collect();

    let stack_size_lit = Literal::u32_unsuffixed(function.param_size as _);

    let raw_fn_name = &function.function_name;

    let self_tokens = if !function.flags.contains(&FunctionFlag::Static) {
        let tokens = if function.flags.contains(&FunctionFlag::Const) {
            quote! {
                &self
            }
        } else {
            quote! {
                &mut self
            }
        };
        Some(tokens)
    } else {
        None
    }
    .into_iter();

    let out_tokens = function.parameters.iter().filter_map(|param| {
        let is_out_param = param.property.flags.contains(&PropertyFlag::OutParm)
            && !param.property.flags.contains(&PropertyFlag::ConstParm);

        if param.property.name == "ReturnValue" || !is_out_param {
            return None;
        }

        let name = format_ident!("{}", property_name(&param.property));
        let ty = generate_type(module, ctx, &param.property.data_type).unwrap();

        let offset = Literal::u32_unsuffixed(param.property.offset);

        Some(quote! {
            unsafe {
                __buffer.add(#offset).cast::<#ty>().swap(#name);
            }

        })
        .clone()
    });

    let ty_name_ident = ctx.name_resolver.get_type_ident(Some(module), ty_name);
    let module_ident = format_ident!("{}", module);
    let object_ptr_tokens = if function.flags.contains(&FunctionFlag::Static) {
        quote! {
            let __object_ptr = crate::bindings::#module_ident::#ty_name_ident::cdo();
        }
    } else {
        quote! {
            let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        }
    };
    let return_tokens = function.parameters.last().and_then(|param| {
        if param.property.name != "ReturnValue" {
            return None;
        }

        let ty = generate_type(module, ctx, &param.property.data_type).unwrap();

        let return_offset = Literal::u32_unsuffixed(param.property.offset);

        Some(quote! {
            unsafe {
                __buffer.add(#return_offset).cast::<#ty>().read()
            }
        })
    });

    let copy_params_to_buffer_tokens = function.parameters.iter().filter_map(|param| {
        if param.property.flags.contains(&PropertyFlag::ReturnParm) {
            return None;
        }

        let offset = Literal::u32_unsuffixed(param.property.offset);
        let name = format_ident!("{}", property_name(&param.property));
        let ty = generate_type(module, ctx, &param.property.data_type).unwrap();

        let borrow_token = if !param.property.flags.contains(&PropertyFlag::OutParm) {
            Some(quote! {&})
        } else {
            None
        };

        Some(quote! {
            unsafe {
                std::ptr::copy_nonoverlapping(#borrow_token #name, __buffer.add(#offset).cast::<#ty>(), 1);
            }
        })
    });

    let fn_ptr = format_ident!(
        "{}_{}",
        ty_name.to_shouty_snake_case(),
        function.function_name.to_shouty_snake_case()
    );

    quote! {
        pub fn #fn_ident(#(#self_tokens,)* #(#param_tokens),*) #return_ty
        {
            let mut __stack = crate::core_data::StackAlloc::<#stack_size_lit>::new();
            let __buffer = __stack.buffer_mut();

            let __bindings = crate::module::bindings();

            unsafe { (__bindings.core_fns.initialize_values_in_param_buffer)(crate::bindings::#module_ident::#fn_ptr, __buffer) };
            #(
                #copy_params_to_buffer_tokens
            )*

            #object_ptr_tokens
            unsafe { (__bindings.core_fns.process_event)(__object_ptr, crate::bindings::#module_ident::#fn_ptr, __buffer) };

            #(
                #out_tokens
            )*

            #return_tokens
        }
    }
}
pub fn generate_class(
    ctx: &Context,
    class_def: &ClassDefinition,
) -> Result<(String, TokenStream), Box<dyn Error>> {
    let class_str = &class_def.class_name;
    let class_name = format_ident!("{}", class_def.class_name);
    // let trait_name = format_ident!("I{}", &actor.class_name[1..]);

    let module_name = class_def
        .package
        .split_terminator("/")
        .last()
        .unwrap()
        .to_snake_case();

    let interface_tokens = if class_def.is_interface {
        let interface_name = format_ident!("I{}", class_def.class_name.trim_start_matches("U"));
        Some(quote! {
            pub struct #interface_name {
            }
        })
    } else {
        None
    };

    let alignment = Literal::u32_unsuffixed(class_def.min_alignment);

    let PropertyInfo { tokens, offset } =
        generate_properties(&module_name, ctx, &class_def.properties);

    let end_padding = if align_up(offset as u32, class_def.min_alignment) < class_def.property_sizes
    {
        let padding_size = Literal::usize_unsuffixed(class_def.property_sizes as usize - offset);
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
        "UVolumetricCloudComponent",
        "URustExtension_FHitResult",
    ];
    let layout_check_tokens = if allow_list.contains(&class_def.class_name.as_str()) {
        Some(generate_layout_check(
            &class_def.class_name,
            &class_def.properties,
        ))
    } else {
        None
    };

    let fn_allow_list = [
        "K2_GetActorLocation",
        "K2_SetActorLocation",
        "GetLevelTransform",
        "New",
        "SetTextureParameterValueByInfo",
    ];

    let function_tokens: Vec<_> = class_def
        .functions
        .iter()
        .filter(|f| {
            f.flags.contains(&FunctionFlag::BlueprintCallable)
                || f.flags.contains(&FunctionFlag::BlueprintPure)
        })
        // .filter(|f| fn_allow_list.contains(&f.function_name.as_str()))
        .map(|f| generate_function(ctx, &class_def.class_name, &module_name, f))
        .collect();

    let class_tokens = quote! {
        #interface_tokens
        #class_def_tokens
        impl #class_name
        {
            pub fn static_class() -> *mut crate::ffi::UObjectOpague
            {
                *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get(#class_str).unwrap()
            }
            pub fn cdo() -> *mut crate::ffi::UObjectOpague {
                let class = Self::static_class();
                unsafe {
                    let mut cdo = std::ptr::null_mut();
                    (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
                    cdo
                }
            }

            #(
                #function_tokens
            )*

            #layout_check_tokens
        }
    };

    Ok((module_name, class_tokens))
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

            match *usage_hint {
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
            }
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

    Ok(tokens)
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

    let end_padding = if align_up(offset as u32, struct_def.min_alignment)
        < struct_def.property_sizes
    {
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

    let fn_tokens = ctx
        .extension_fns
        .get(&struct_def.struct_name)
        .map(|extension_class_names| {
            let extension_tokens = extension_class_names
                .iter()
                .filter_map(|name| ctx.name_to_class.get(name))
                .flat_map(|def| {
                    def.functions.iter().map(|f| {
                        generate_function(
                            ctx,
                            &def.class_name,
                            &module_name_from_package(&def.package),
                            f,
                        )
                    })
                });

            quote! {
                #(
                    #extension_tokens
                )*
            }
        });

    let tokens = quote! {
        #struct_def_tokens
        impl #struct_ident
        {
            #layout_check_tokens
            #fn_tokens
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

    let global_module_tokens = generate_module_globals(api);
    for (module_name, tokens) in global_module_tokens {
        modules.entry(module_name.clone()).or_default().push(tokens);
    }

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
            #![allow(clippy::non_camel_case_types)]
            #![allow(clippy::new_without_default)]
            #![allow(clippy::new_ret_no_self)]
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
        pub mod globals;

        #[cfg(test)]
        mod tests;

    };
    let globals = generate_globals(api);

    save_file(&globals, &out_path.join("globals.rs"));
    save_file(&mod_tests, &out_path.join("tests.rs"));
    save_file(&mod_tokens, &out_path.join("mod.rs"));
    save_file(&opague_tokens, &out_path.join("opague_definitions.rs"));
    Ok(())
}
