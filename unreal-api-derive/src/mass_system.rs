use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, Pat, Type, TypeReference};

/// Query scope: primary (per-chunk) or global (all matching entities).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum QueryScope {
    Primary, // MassQuery<&T> / MassQuery<&mut T>
    Global,  // MassQueryAll<&T> / MassQueryAll<&mut T>
}

/// Information about one query parameter extracted from a system function signature.
struct QueryParam {
    /// The parameter name (e.g., `ants`).
    param_name: syn::Ident,
    /// The fragment type (e.g., `AntFragment`).
    fragment_type: syn::Type,
    /// Whether the query is mutable (&mut T).
    is_mutable: bool,
    /// Query scope: primary (per-chunk) or global (all entities).
    scope: QueryScope,
    /// Index within its scope group (primary index or global index).
    scope_index: usize,
}

/// Parses a function signature and extracts MassQuery/MassQueryAll parameters.
/// Non-query params (like `dt: f32`) are treated as chunk-level data.
fn extract_query_params(func: &ItemFn) -> syn::Result<Vec<QueryParam>> {
    let mut params = Vec::new();
    let mut primary_index = 0;
    let mut global_index = 0;

    for arg in &func.sig.inputs {
        let FnArg::Typed(pat_type) = arg else {
            continue;
        };

        if let Type::Path(type_path) = &*pat_type.ty {
            let last_seg = type_path.path.segments.last();
            if let Some(seg) = last_seg {
                let scope = if seg.ident == "MassQuery" {
                    Some(QueryScope::Primary)
                } else if seg.ident == "MassQueryAll" {
                    Some(QueryScope::Global)
                } else {
                    None
                };

                if let Some(scope) = scope {
                    let param_name = match &*pat_type.pat {
                        Pat::Ident(pat_ident) => pat_ident.ident.clone(),
                        _ => {
                            return Err(syn::Error::new_spanned(
                                &pat_type.pat,
                                "expected identifier pattern",
                            ));
                        }
                    };

                    let (fragment_type, is_mutable) = extract_query_inner_type(seg)?;

                    let scope_index = match scope {
                        QueryScope::Primary => {
                            let idx = primary_index;
                            primary_index += 1;
                            idx
                        }
                        QueryScope::Global => {
                            let idx = global_index;
                            global_index += 1;
                            idx
                        }
                    };

                    params.push(QueryParam {
                        param_name,
                        fragment_type,
                        is_mutable,
                        scope,
                        scope_index,
                    });
                }
            }
        }
    }

    Ok(params)
}

/// Extracts the inner type and mutability from MassQuery<&T> or MassQuery<&mut T>.
fn extract_query_inner_type(
    seg: &syn::PathSegment,
) -> syn::Result<(syn::Type, bool)> {
    let syn::PathArguments::AngleBracketed(args) = &seg.arguments else {
        return Err(syn::Error::new_spanned(seg, "query type requires type parameter"));
    };

    let first_arg = args.args.first().ok_or_else(|| {
        syn::Error::new_spanned(seg, "query type requires one type parameter")
    })?;

    let syn::GenericArgument::Type(inner_ty) = first_arg else {
        return Err(syn::Error::new_spanned(first_arg, "expected type argument"));
    };

    match inner_ty {
        Type::Reference(TypeReference {
            mutability, elem, ..
        }) => Ok((*elem.clone(), mutability.is_some())),
        other => Ok((other.clone(), false)),
    }
}

pub fn mass_system_impl(func: &ItemFn) -> syn::Result<TokenStream> {
    let func_name = &func.sig.ident;
    let wrapper_name = format_ident!("__mass_system_{}", func_name);
    let reg_name = format_ident!("__mass_system_reg_{}", func_name);
    let system_name_str = func_name.to_string();

    let query_params = extract_query_params(func)?;

    // Generate fragment unpacking code
    let unpack_stmts: Vec<TokenStream> = query_params
        .iter()
        .map(|p| {
            let param_name = &p.param_name;
            let frag_type = &p.fragment_type;
            let idx = p.scope_index;

            match (p.scope, p.is_mutable) {
                (QueryScope::Primary, true) => {
                    quote! {
                        let mut #param_name = unsafe {
                            ::unreal_api::mass::MassQueryMut::<#frag_type>::from_raw(
                                (*chunk).fragments.add(#idx).read().data,
                                (*chunk).fragments.add(#idx).read().count as usize,
                            )
                        };
                    }
                }
                (QueryScope::Primary, false) => {
                    quote! {
                        let #param_name = unsafe {
                            ::unreal_api::mass::MassQueryRef::<#frag_type>::from_raw(
                                (*chunk).fragments.add(#idx).read().data as *const ::std::ffi::c_void,
                                (*chunk).fragments.add(#idx).read().count as usize,
                            )
                        };
                    }
                }
                (QueryScope::Global, true) => {
                    quote! {
                        let mut #param_name = unsafe {
                            ::unreal_api::mass::MassQueryAllMut::<#frag_type>::from_raw(
                                (*chunk).global_fragments.add(#idx).read().data,
                                (*chunk).global_entity_handles,
                                (*chunk).global_num_entities as usize,
                            )
                        };
                    }
                }
                (QueryScope::Global, false) => {
                    quote! {
                        let #param_name = unsafe {
                            ::unreal_api::mass::MassQueryAllRef::<#frag_type>::from_raw(
                                (*chunk).global_fragments.add(#idx).read().data as *const ::std::ffi::c_void,
                                (*chunk).global_entity_handles,
                                (*chunk).global_num_entities as usize,
                            )
                        };
                    }
                }
            }
        })
        .collect();

    // Generate the call arguments — pass query params and dt
    let call_args: Vec<TokenStream> = func
        .sig
        .inputs
        .iter()
        .map(|arg| {
            let FnArg::Typed(pat_type) = arg else {
                return quote! {};
            };
            let param_name = match &*pat_type.pat {
                Pat::Ident(pat_ident) => &pat_ident.ident,
                _ => return quote! {},
            };

            // Check if it's a query param
            if let Type::Path(type_path) = &*pat_type.ty {
                if let Some(seg) = type_path.path.segments.last() {
                    if seg.ident == "MassQuery" || seg.ident == "MassQueryAll" {
                        let is_mut = query_params
                            .iter()
                            .any(|p| p.param_name == *param_name && p.is_mutable);
                        if is_mut {
                            return quote! { &mut #param_name };
                        } else {
                            return quote! { &#param_name };
                        }
                    }
                }
            }

            // For `dt: f32`, extract from chunk data
            let param_str = param_name.to_string();
            if param_str == "dt" {
                return quote! { unsafe { (*chunk).dt } };
            }

            quote! { #param_name }
        })
        .collect();

    // Generate requirement descriptors (primary first, then global)
    let requirement_entries: Vec<TokenStream> = query_params
        .iter()
        .map(|p| {
            let frag_type = &p.fragment_type;
            let access = if p.is_mutable { 1u8 } else { 0u8 };
            let scope = match p.scope {
                QueryScope::Primary => 0u8,
                QueryScope::Global => 1u8,
            };
            quote! {
                ::unreal_api::mass::MassSystemRequirement {
                    cpp_type_name: <#frag_type as ::unreal_api::mass::MassFragment>::CPP_TYPE_NAME,
                    size: ::std::mem::size_of::<#frag_type>(),
                    align: ::std::mem::align_of::<#frag_type>(),
                    access_mode: #access,
                    is_tag: 0,
                    query_scope: #scope,
                }
            }
        })
        .collect();

    let num_requirements = requirement_entries.len();

    // Rewrite the function signature to take references to concrete query types
    let rewritten_params: Vec<TokenStream> = func
        .sig
        .inputs
        .iter()
        .map(|arg| {
            let FnArg::Typed(pat_type) = arg else {
                return quote! { #arg };
            };
            let param_name = match &*pat_type.pat {
                Pat::Ident(pat_ident) => &pat_ident.ident,
                _ => return quote! { #arg },
            };

            if let Type::Path(type_path) = &*pat_type.ty {
                if let Some(seg) = type_path.path.segments.last() {
                    if let Some(qp) = query_params.iter().find(|p| p.param_name == *param_name) {
                        let inner = &qp.fragment_type;
                        return match (qp.scope, qp.is_mutable) {
                            (QueryScope::Primary, true) => {
                                quote! { #param_name: &mut ::unreal_api::mass::MassQueryMut<'_, #inner> }
                            }
                            (QueryScope::Primary, false) => {
                                quote! { #param_name: &::unreal_api::mass::MassQueryRef<'_, #inner> }
                            }
                            (QueryScope::Global, true) => {
                                quote! { #param_name: &mut ::unreal_api::mass::MassQueryAllMut<'_, #inner> }
                            }
                            (QueryScope::Global, false) => {
                                quote! { #param_name: &::unreal_api::mass::MassQueryAllRef<'_, #inner> }
                            }
                        };
                    }
                    // Unreachable for query params, but handle gracefully
                }
            }

            quote! { #arg }
        })
        .collect();

    let func_body = &func.block;
    let func_vis = &func.vis;
    let func_ret = &func.sig.output;

    Ok(quote! {
        #func_vis fn #func_name(#(#rewritten_params),*) #func_ret
            #func_body

        unsafe extern "C" fn #wrapper_name(chunk: *const ::unreal_api::ffi::MassChunkData) {
            if chunk.is_null() {
                return;
            }
            #(#unpack_stmts)*
            #func_name(#(#call_args),*);
        }

        static #reg_name: () = {
            const REQUIREMENTS: [::unreal_api::mass::MassSystemRequirement; #num_requirements] = [
                #(#requirement_entries),*
            ];

            ::unreal_api::inventory::submit! {
                ::unreal_api::mass::MassSystemRegistration {
                    name: #system_name_str,
                    execute_fn: #wrapper_name,
                    requirements: &REQUIREMENTS,
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
    fn parses_single_query_param() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(ants: MassQuery<&mut AntFragment>, dt: f32) {}
        })
        .unwrap();

        let params = extract_query_params(&func).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].param_name, "ants");
        assert!(params[0].is_mutable);
        assert_eq!(params[0].scope, QueryScope::Primary);
        assert_eq!(params[0].scope_index, 0);
    }

    #[test]
    fn parses_multiple_query_params() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(
                ants: MassQuery<&mut AntFragment>,
                food: MassQuery<&FoodFragment>,
                dt: f32,
            ) {}
        })
        .unwrap();

        let params = extract_query_params(&func).unwrap();
        assert_eq!(params.len(), 2);
        assert!(params[0].is_mutable);
        assert!(!params[1].is_mutable);
        assert_eq!(params[0].scope_index, 0);
        assert_eq!(params[1].scope_index, 1);
    }

    #[test]
    fn parses_global_query_params() {
        let func: ItemFn = syn::parse2(quote! {
            fn food_system(
                ants: MassQuery<&mut AntFragment>,
                encounters: MassQuery<&EncounterFragment>,
                foods: MassQueryAll<&mut FoodFragment>,
                dt: f32,
            ) {}
        })
        .unwrap();

        let params = extract_query_params(&func).unwrap();
        assert_eq!(params.len(), 3);

        // Primary queries
        assert_eq!(params[0].scope, QueryScope::Primary);
        assert_eq!(params[0].scope_index, 0);
        assert!(params[0].is_mutable);

        assert_eq!(params[1].scope, QueryScope::Primary);
        assert_eq!(params[1].scope_index, 1);
        assert!(!params[1].is_mutable);

        // Global query
        assert_eq!(params[2].scope, QueryScope::Global);
        assert_eq!(params[2].scope_index, 0);
        assert!(params[2].is_mutable);
    }

    #[test]
    fn generates_wrapper_and_registration() {
        let func: ItemFn = syn::parse2(quote! {
            fn ant_movement(ants: MassQuery<&mut AntFragment>, dt: f32) {
                for ant in ants.iter_mut() {
                    ant.x += dt as f64;
                }
            }
        })
        .unwrap();

        let output = mass_system_impl(&func).unwrap().to_string();
        assert!(output.contains("__mass_system_ant_movement"), "should generate wrapper fn");
        assert!(output.contains("MassSystemRegistration"), "should register with inventory");
        assert!(output.contains("ant_movement"), "should reference system name");
    }

    #[test]
    fn generates_correct_types_for_mixed_queries() {
        let func: ItemFn = syn::parse2(quote! {
            fn food_decision(
                ants: MassQuery<&mut AntFragment>,
                encounters: MassQuery<&EncounterFragment>,
                foods: MassQueryAll<&mut FoodFragment>,
                dt: f32,
            ) {}
        })
        .unwrap();

        let output = mass_system_impl(&func).unwrap().to_string();
        // Should use MassQueryMut for mutable primary
        assert!(output.contains("MassQueryMut"), "should use MassQueryMut for &mut primary");
        // Should use MassQueryRef for immutable primary
        assert!(output.contains("MassQueryRef"), "should use MassQueryRef for & primary");
        // Should use MassQueryAllMut for mutable global
        assert!(output.contains("MassQueryAllMut"), "should use MassQueryAllMut for &mut global");
        // Should have query_scope: 1 for global
        assert!(output.contains("query_scope : 1"), "should set query_scope=1 for global");
    }
}
