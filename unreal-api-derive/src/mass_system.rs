use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, Pat, Type, TypeReference};

/// Information about one MassQuery parameter extracted from a system function signature.
struct QueryParam {
    /// The parameter name (e.g., `ants`).
    param_name: syn::Ident,
    /// The fragment type (e.g., `AntFragment`).
    fragment_type: syn::Type,
    /// Whether the query is mutable (&mut T).
    is_mutable: bool,
    /// Index in the parameter list (for mapping to MassChunkData.fragments).
    index: usize,
}

/// Parses a function signature and extracts MassQuery parameters.
/// Non-MassQuery params (like `dt: f32`) are treated as chunk-level data.
fn extract_query_params(func: &ItemFn) -> syn::Result<Vec<QueryParam>> {
    let mut params = Vec::new();
    let mut query_index = 0;

    for arg in &func.sig.inputs {
        let FnArg::Typed(pat_type) = arg else {
            continue;
        };

        // Check if this is a MassQuery<...> type
        if let Type::Path(type_path) = &*pat_type.ty {
            let last_seg = type_path.path.segments.last();
            if let Some(seg) = last_seg {
                if seg.ident == "MassQuery" {
                    let param_name = match &*pat_type.pat {
                        Pat::Ident(pat_ident) => pat_ident.ident.clone(),
                        _ => {
                            return Err(syn::Error::new_spanned(
                                &pat_type.pat,
                                "expected identifier pattern",
                            ));
                        }
                    };

                    // Extract the inner type from MassQuery<&T> or MassQuery<&mut T>
                    let (fragment_type, is_mutable) = extract_query_inner_type(seg)?;

                    params.push(QueryParam {
                        param_name,
                        fragment_type,
                        is_mutable,
                        index: query_index,
                    });
                    query_index += 1;
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
        return Err(syn::Error::new_spanned(seg, "MassQuery requires type parameter"));
    };

    let first_arg = args.args.first().ok_or_else(|| {
        syn::Error::new_spanned(seg, "MassQuery requires one type parameter")
    })?;

    let syn::GenericArgument::Type(inner_ty) = first_arg else {
        return Err(syn::Error::new_spanned(first_arg, "expected type argument"));
    };

    match inner_ty {
        Type::Reference(TypeReference {
            mutability, elem, ..
        }) => Ok((*elem.clone(), mutability.is_some())),
        // If not a reference, assume immutable owned
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
            let idx = p.index;
            quote! {
                let mut #param_name = unsafe {
                    ::unreal_api::mass::MassQuery::<#frag_type>::from_raw(
                        (*chunk).fragments.add(#idx).read().data,
                        (*chunk).fragments.add(#idx).read().count as usize,
                    )
                };
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

            // Check if it's a MassQuery param (pass by mut ref)
            if let Type::Path(type_path) = &*pat_type.ty {
                if let Some(seg) = type_path.path.segments.last() {
                    if seg.ident == "MassQuery" {
                        // Find if this query param is mutable
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

    // Generate requirement descriptors
    let requirement_entries: Vec<TokenStream> = query_params
        .iter()
        .map(|p| {
            let frag_type = &p.fragment_type;
            let access = if p.is_mutable { 1u8 } else { 0u8 };
            quote! {
                ::unreal_api::mass::MassSystemRequirement {
                    cpp_type_name: <#frag_type as ::unreal_api::mass::MassFragment>::CPP_TYPE_NAME,
                    size: ::std::mem::size_of::<#frag_type>(),
                    align: ::std::mem::align_of::<#frag_type>(),
                    access_mode: #access,
                    is_tag: 0,
                }
            }
        })
        .collect();

    let num_requirements = requirement_entries.len();

    // Rewrite the function signature to take references instead of owned MassQuery
    // We need to transform the function params
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
                    if seg.ident == "MassQuery" {
                        let is_mut = query_params
                            .iter()
                            .any(|p| p.param_name == *param_name && p.is_mutable);
                        let inner = &query_params
                            .iter()
                            .find(|p| p.param_name == *param_name)
                            .unwrap()
                            .fragment_type;
                        if is_mut {
                            return quote! { #param_name: &mut ::unreal_api::mass::MassQuery<'_, #inner> };
                        } else {
                            return quote! { #param_name: &::unreal_api::mass::MassQuery<'_, #inner> };
                        }
                    }
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
        assert_eq!(params[0].index, 0);
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
}
