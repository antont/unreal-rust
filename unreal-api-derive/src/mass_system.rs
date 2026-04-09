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

/// Information about one resource parameter (Res<T> or ResMut<T>).
struct ResourceParam {
    /// The parameter name (e.g., `spatial`).
    param_name: syn::Ident,
    /// The resource type (e.g., `MassSpatialQueryCallback`).
    resource_type: syn::Type,
    /// Whether the resource is mutable (ResMut vs Res).
    is_mutable: bool,
}

/// Extracts the inner type from Res<T> or ResMut<T>.
fn extract_resource_inner_type(seg: &syn::PathSegment) -> syn::Result<syn::Type> {
    let syn::PathArguments::AngleBracketed(args) = &seg.arguments else {
        return Err(syn::Error::new_spanned(seg, "Res/ResMut requires type parameter"));
    };
    let first_arg = args.args.first().ok_or_else(|| {
        syn::Error::new_spanned(seg, "Res/ResMut requires one type parameter")
    })?;
    let syn::GenericArgument::Type(inner_ty) = first_arg else {
        return Err(syn::Error::new_spanned(first_arg, "expected type argument"));
    };
    Ok(inner_ty.clone())
}

/// Parses a function signature and extracts Res<T>/ResMut<T> parameters.
fn extract_resource_params(func: &ItemFn) -> syn::Result<Vec<ResourceParam>> {
    let mut params = Vec::new();

    for arg in &func.sig.inputs {
        let FnArg::Typed(pat_type) = arg else {
            continue;
        };

        if let Type::Path(type_path) = &*pat_type.ty {
            if let Some(seg) = type_path.path.segments.last() {
                let is_res = seg.ident == "Res";
                let is_resmut = seg.ident == "ResMut";

                if is_res || is_resmut {
                    let param_name = match &*pat_type.pat {
                        Pat::Ident(pat_ident) => pat_ident.ident.clone(),
                        _ => {
                            return Err(syn::Error::new_spanned(
                                &pat_type.pat,
                                "expected identifier pattern",
                            ));
                        }
                    };

                    let resource_type = extract_resource_inner_type(seg)?;

                    params.push(ResourceParam {
                        param_name,
                        resource_type,
                        is_mutable: is_resmut,
                    });
                }
            }
        }
    }

    Ok(params)
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
                let scope = if seg.ident == "MassQuery" || seg.ident == "Query" {
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
    let resource_params = extract_resource_params(func)?;

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
                            ::unreal_api::mass::MassQueryAllMut::<#frag_type>::from_chunked(
                                (*chunk).global_chunked_fragments.add(#idx),
                            )
                        };
                    }
                }
                (QueryScope::Global, false) => {
                    quote! {
                        let #param_name = unsafe {
                            ::unreal_api::mass::MassQueryAllRef::<#frag_type>::from_chunked(
                                (*chunk).global_chunked_fragments.add(#idx),
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

            // Check if it's a query param — pass by value (rewritten fn takes owned types)
            if let Type::Path(type_path) = &*pat_type.ty {
                if let Some(seg) = type_path.path.segments.last() {
                    if seg.ident == "MassQuery" || seg.ident == "MassQueryAll" || seg.ident == "Query" {
                        return quote! { #param_name };
                    }
                }
            }

            // Check if it's a resource param (Res<T> / ResMut<T>) — skip in C++ wrapper
            if let Type::Path(type_path) = &*pat_type.ty {
                if let Some(seg) = type_path.path.segments.last() {
                    if seg.ident == "Res" || seg.ident == "ResMut" {
                        // C++ extern wrapper doesn't have Bevy resources — skip
                        return quote! {};
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

    // Filter out empty tokens from skipped resource params in C++ call_args
    let call_args: Vec<TokenStream> = call_args.into_iter().filter(|t| !t.is_empty()).collect();

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
                                quote! { mut #param_name: ::unreal_api::mass::MassQueryMut<'_, #inner> }
                            }
                            (QueryScope::Primary, false) => {
                                quote! { #param_name: ::unreal_api::mass::MassQueryRef<'_, #inner> }
                            }
                            (QueryScope::Global, true) => {
                                quote! { mut #param_name: ::unreal_api::mass::MassQueryAllMut<'_, #inner> }
                            }
                            (QueryScope::Global, false) => {
                                quote! { #param_name: ::unreal_api::mass::MassQueryAllRef<'_, #inner> }
                            }
                        };
                    }
                    // Unreachable for query params, but handle gracefully
                }
            }

            // Check if it's a resource param — rewrite Res<T> → &T, ResMut<T> → &mut T
            if let Some(rp) = resource_params.iter().find(|r| r.param_name == *param_name) {
                let res_type = &rp.resource_type;
                if rp.is_mutable {
                    return quote! { #param_name: &mut #res_type };
                } else {
                    return quote! { #param_name: &#res_type };
                }
            }

            quote! { #arg }
        })
        .collect();

    let func_body = &func.block;
    let func_vis = &func.vis;
    let func_ret = &func.sig.output;

    // --- Generate Bevy wrapper function ---
    let bevy_wrapper_name = format_ident!("{}_bevy", func_name);
    let bevy_reg_name = format_ident!("__mass_bevy_reg_{}", func_name);
    let marker_name = format_ident!("__mass_marker_{}", func_name);

    // Bevy resource params: Res<T> / ResMut<T> pass through to the Bevy wrapper
    let bevy_resource_params: Vec<TokenStream> = resource_params
        .iter()
        .map(|r| {
            let param_name = &r.param_name;
            let res_type = &r.resource_type;
            if r.is_mutable {
                quote! { mut #param_name: ::unreal_api::ecs::prelude::ResMut<#res_type> }
            } else {
                quote! { #param_name: ::unreal_api::ecs::prelude::Res<#res_type> }
            }
        })
        .collect();

    // Bevy system params: one Res/ResMut<MassSystemChunks<Marker, T>> per unique fragment type, plus Res<MassDeltaTime>
    let bevy_params: Vec<TokenStream> = query_params
        .iter()
        .map(|p| {
            let frag_type = &p.fragment_type;
            let param_name = format_ident!("__mass_{}", p.param_name);
            if p.is_mutable {
                quote! { mut #param_name: ::unreal_api::ecs::prelude::ResMut<::unreal_api::mass::MassSystemChunks<#marker_name, #frag_type>> }
            } else {
                quote! { #param_name: ::unreal_api::ecs::prelude::Res<::unreal_api::mass::MassSystemChunks<#marker_name, #frag_type>> }
            }
        })
        .collect();

    // Build the chunk iteration body for the Bevy wrapper
    let bevy_unpack_stmts: Vec<TokenStream> = query_params
        .iter()
        .map(|p| {
            let param_name = &p.param_name;
            let res_name = format_ident!("__mass_{}", p.param_name);
            match (p.scope, p.is_mutable) {
                (QueryScope::Primary, true) => {
                    quote! {
                        let mut #param_name = unsafe { #res_name.primary_chunk_mut(__chunk_idx) };
                    }
                }
                (QueryScope::Primary, false) => {
                    quote! {
                        let #param_name = unsafe { #res_name.primary_chunk_ref(__chunk_idx) };
                    }
                }
                (QueryScope::Global, true) => {
                    quote! {
                        let mut #param_name = unsafe { #res_name.global_query_mut().expect("global query not set") };
                    }
                }
                (QueryScope::Global, false) => {
                    quote! {
                        let #param_name = unsafe { #res_name.global_query_ref().expect("global query not set") };
                    }
                }
            }
        })
        .collect();

    // Find the first primary query param to determine chunk count
    let primary_count_source = query_params
        .iter()
        .find(|p| p.scope == QueryScope::Primary)
        .map(|p| {
            let res_name = format_ident!("__mass_{}", p.param_name);
            quote! { #res_name.primary_chunk_count() }
        })
        .unwrap_or(quote! { 0 });

    // Bevy call args are the same as regular call args but with local variable names
    let bevy_call_args: Vec<TokenStream> = func
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
            if let Type::Path(type_path) = &*pat_type.ty {
                if let Some(seg) = type_path.path.segments.last() {
                    if seg.ident == "MassQuery" || seg.ident == "MassQueryAll" || seg.ident == "Query" {
                        return quote! { #param_name };
                    }
                }
            }
            // Check if it's a resource param
            if let Some(rp) = resource_params.iter().find(|r| r.param_name == *param_name) {
                if rp.is_mutable {
                    return quote! { &mut #param_name };
                } else {
                    return quote! { &#param_name };
                }
            }

            let param_str = param_name.to_string();
            if param_str == "dt" {
                return quote! { __mass_dt.0 };
            }
            quote! { #param_name }
        })
        .collect();

    // Resource initializers for MassBevySystemRegistration
    let init_resource_stmts: Vec<TokenStream> = query_params
        .iter()
        .map(|p| {
            let frag_type = &p.fragment_type;
            quote! {
                if !world.contains_resource::<::unreal_api::mass::MassSystemChunks<#marker_name, #frag_type>>() {
                    world.insert_resource(::unreal_api::mass::MassSystemChunks::<#marker_name, #frag_type>::new());
                }
            }
        })
        .collect();

    // Clear resources closure — clears all MassSystemChunks<Marker, T> this system uses
    let clear_resource_stmts: Vec<TokenStream> = query_params
        .iter()
        .map(|p| {
            let frag_type = &p.fragment_type;
            quote! {
                world.resource_mut::<::unreal_api::mass::MassSystemChunks<#marker_name, #frag_type>>().clear();
            }
        })
        .collect();

    // Populate resources closure — unpacks MassSystemChunkBatch into typed MassChunks<T>
    // Primary fragments: indexed by scope_index within MassChunkData.fragments
    // Global fragments: indexed by scope_index within MassChunkData.global_chunked_fragments
    let populate_primary_stmts: Vec<TokenStream> = query_params
        .iter()
        .filter(|p| p.scope == QueryScope::Primary)
        .map(|p| {
            let frag_type = &p.fragment_type;
            let idx = p.scope_index;
            quote! {
                world.resource_mut::<::unreal_api::mass::MassSystemChunks<#marker_name, #frag_type>>()
                    .push_primary_slice(*chunk.fragments.add(#idx));
            }
        })
        .collect();

    let populate_global_stmts: Vec<TokenStream> = query_params
        .iter()
        .filter(|p| p.scope == QueryScope::Global)
        .map(|p| {
            let frag_type = &p.fragment_type;
            let idx = p.scope_index;
            quote! {
                if world.resource::<::unreal_api::mass::MassSystemChunks<#marker_name, #frag_type>>().global().is_none() {
                    world.resource_mut::<::unreal_api::mass::MassSystemChunks<#marker_name, #frag_type>>()
                        .set_global(chunk.global_chunked_fragments.add(#idx));
                }
            }
        })
        .collect();

    let has_resource_params = !resource_params.is_empty();

    // C++ registration — always generated so C++ creates a processor that
    // collects chunks for Bevy-scheduled dispatch.
    // For systems with Res<T> params, the execute_fn is a no-op since direct
    // dispatch can't provide Bevy resources; only Bevy scheduling uses them.
    let cpp_wrapper = if has_resource_params {
        quote! {
            unsafe extern "C" fn #wrapper_name(_chunk: *const ::unreal_api::ffi::MassChunkData) {
                // No-op: this system uses Bevy resources and can only run via
                // the Bevy schedule, not direct C++ dispatch.
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
        }
    } else {
        quote! {
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
        }
    };

    Ok(quote! {
        #func_vis fn #func_name(#(#rewritten_params),*) #func_ret
            #func_body

        #cpp_wrapper

        /// Zero-sized marker type for per-system chunk isolation.
        struct #marker_name;

        fn #bevy_wrapper_name(
            #(#bevy_params,)*
            #(#bevy_resource_params,)*
            __mass_dt: ::unreal_api::ecs::prelude::Res<::unreal_api::mass::MassDeltaTime>,
        ) {
            for __chunk_idx in 0..#primary_count_source {
                #(#bevy_unpack_stmts)*
                #func_name(#(#bevy_call_args),*);
            }
        }

        static #bevy_reg_name: () = {
            ::unreal_api::inventory::submit! {
                ::unreal_api::mass::MassBevySystemRegistration {
                    name: #system_name_str,
                    add_to_schedule: |schedule: &mut ::unreal_api::ecs::schedule::Schedule,
                                      stage: ::unreal_api::mass::MassSystemStage| {
                        use ::unreal_api::ecs::schedule::IntoScheduleConfigs;
                        schedule.add_systems(#bevy_wrapper_name.in_set(stage));
                    },
                    init_resources: |world: &mut ::unreal_api::ecs::world::World| {
                        #(#init_resource_stmts)*
                    },
                    clear_resources: |world: &mut ::unreal_api::ecs::world::World| {
                        #(#clear_resource_stmts)*
                    },
                    populate_resources: |world: &mut ::unreal_api::ecs::world::World,
                                         batch: &::unreal_api::ffi::MassSystemChunkBatch| {
                        unsafe {
                            let chunks = ::std::slice::from_raw_parts(
                                batch.primary_chunks,
                                batch.num_primary_chunks as usize,
                            );
                            for chunk in chunks {
                                #(#populate_primary_stmts)*
                            }
                            if let Some(chunk) = chunks.first() {
                                #(#populate_global_stmts)*
                            }
                        }
                    },
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
    fn generates_bevy_wrapper() {
        let func: ItemFn = syn::parse2(quote! {
            fn ant_movement(ants: MassQuery<&mut AntFragment>, dt: f32) {
                for ant in ants.iter_mut() { ant.x += dt as f64; }
            }
        })
        .unwrap();

        let output = mass_system_impl(&func).unwrap().to_string();
        assert!(output.contains("ant_movement_bevy"), "should generate Bevy wrapper fn");
        assert!(output.contains("ResMut"), "should use ResMut for mutable query");
        assert!(output.contains("MassSystemChunks"), "should reference MassSystemChunks resource");
        assert!(output.contains("MassDeltaTime"), "should extract dt from resource");
        assert!(output.contains("MassSystemStage"), "add_to_schedule should accept MassSystemStage");
        assert!(output.contains("in_set"), "should add system to stage set");
    }

    #[test]
    fn bevy_wrapper_uses_res_for_readonly() {
        let func: ItemFn = syn::parse2(quote! {
            fn read_system(enc: MassQuery<&EncounterFragment>, dt: f32) {}
        })
        .unwrap();

        let output = mass_system_impl(&func).unwrap().to_string();
        // Read-only primary should use Res, not ResMut
        assert!(output.contains("Res <"), "read-only query should use Res");
    }

    #[test]
    fn bevy_wrapper_handles_global_query() {
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
        assert!(output.contains("MassBevySystemRegistration"), "should register for Bevy scheduling");
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
        // Should use MassQueryAllMut for mutable global via from_chunked
        assert!(output.contains("MassQueryAllMut"), "should use MassQueryAllMut for &mut global");
        assert!(output.contains("from_chunked"), "should use from_chunked for global query");
        // Should have query_scope: 1 for global
        assert!(output.contains("query_scope : 1"), "should set query_scope=1 for global");
    }

    #[test]
    fn generates_populate_and_clear_resources() {
        let func: ItemFn = syn::parse2(quote! {
            fn ant_movement(ants: MassQuery<&mut AntFragment>, dt: f32) {
                for ant in ants.iter_mut() { ant.x += dt as f64; }
            }
        })
        .unwrap();

        let output = mass_system_impl(&func).unwrap().to_string();
        assert!(output.contains("populate_resources"), "should generate populate_resources closure");
        assert!(output.contains("clear_resources"), "should generate clear_resources closure");
        assert!(output.contains("push_primary_slice"), "populate should push primary slices");
    }

    #[test]
    fn generates_populate_with_global_fragments() {
        let func: ItemFn = syn::parse2(quote! {
            fn food_decision(
                ants: MassQuery<&mut AntFragment>,
                foods: MassQueryAll<&mut FoodFragment>,
                dt: f32,
            ) {}
        })
        .unwrap();

        let output = mass_system_impl(&func).unwrap().to_string();
        assert!(output.contains("set_global"), "populate should set global descriptor for global queries");
        assert!(output.contains("global_chunked_fragments"), "should read from global_chunked_fragments");
    }

    // -----------------------------------------------------------------------
    // Resource parameter (Res<T> / ResMut<T>) tests
    // -----------------------------------------------------------------------

    #[test]
    fn parses_res_parameter() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(
                ants: MassQuery<&mut AntFragment>,
                spatial: Res<SpatialCallback>,
                dt: f32,
            ) {}
        })
        .unwrap();

        let resource_params = extract_resource_params(&func).unwrap();
        assert_eq!(resource_params.len(), 1);
        assert_eq!(resource_params[0].param_name, "spatial");
        assert!(!resource_params[0].is_mutable);

        // Query params should not include the Res param
        let query_params = extract_query_params(&func).unwrap();
        assert_eq!(query_params.len(), 1);
    }

    #[test]
    fn parses_resmut_parameter() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(
                ants: MassQuery<&mut AntFragment>,
                state: ResMut<MyState>,
                dt: f32,
            ) {}
        })
        .unwrap();

        let resource_params = extract_resource_params(&func).unwrap();
        assert_eq!(resource_params.len(), 1);
        assert_eq!(resource_params[0].param_name, "state");
        assert!(resource_params[0].is_mutable);
    }

    #[test]
    fn bevy_wrapper_includes_res_param() {
        let func: ItemFn = syn::parse2(quote! {
            fn collision_prepass(
                ants: MassQuery<&AntFragment>,
                encounters: MassQuery<&mut EncounterFragment>,
                spatial: Res<SpatialCallback>,
            ) {}
        })
        .unwrap();

        let output = mass_system_impl(&func).unwrap().to_string();
        // Bevy wrapper should have Res<SpatialCallback> param
        assert!(output.contains("Res < SpatialCallback >") || output.contains("Res<SpatialCallback>"),
            "bevy wrapper should include Res<T> param, got: {}", output);
        // Rewritten inner fn should have &SpatialCallback
        assert!(output.contains("& SpatialCallback") || output.contains("&SpatialCallback"),
            "rewritten fn should have &T for Res<T> param");
    }

    #[test]
    fn bevy_wrapper_includes_resmut_param() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(
                ants: MassQuery<&mut AntFragment>,
                state: ResMut<MyState>,
                dt: f32,
            ) {}
        })
        .unwrap();

        let output = mass_system_impl(&func).unwrap().to_string();
        assert!(output.contains("ResMut < MyState >") || output.contains("ResMut<MyState>"),
            "bevy wrapper should include ResMut<T> param");
        assert!(output.contains("& mut MyState") || output.contains("&mut MyState"),
            "rewritten fn should have &mut T for ResMut<T> param");
    }

    #[test]
    fn res_param_generates_noop_cpp_wrapper() {
        let func: ItemFn = syn::parse2(quote! {
            fn collision_prepass(
                ants: MassQuery<&AntFragment>,
                spatial: Res<SpatialCallback>,
            ) {}
        })
        .unwrap();

        let output = mass_system_impl(&func).unwrap().to_string();
        // Systems with Res<T> generate a no-op C++ wrapper so C++ creates
        // a processor that collects chunks for Bevy-scheduled dispatch.
        assert!(output.contains("unsafe extern \"C\""),
            "should generate no-op C++ extern wrapper for chunk collection");
        assert!(output.contains("MassSystemRegistration"),
            "should register for C++ processor creation");
        // Should still have Bevy registration
        assert!(output.contains("MassBevySystemRegistration"),
            "should still register for Bevy scheduling");
    }
}
