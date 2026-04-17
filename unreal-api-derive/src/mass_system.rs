use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, Pat, Type, TypeReference};

/// Query scope: primary (per-chunk) or global (all matching entities).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum QueryScope {
    Primary, // MassQuery<&T> / MassQuery<&mut T>
    Global,  // MassQueryAll<&T> / MassQueryAll<&mut T>
}

/// A single fragment reference in a query (type + mutability + scope index).
#[derive(Debug, Clone)]
struct FragmentRef {
    fragment_type: syn::Type,
    is_mutable: bool,
    /// Index within the scope group (assigned during extraction).
    scope_index: usize,
}

/// What data the query accesses: single component or tuple of components.
#[derive(Debug)]
enum QueryData {
    /// `Query<&T>` or `Query<&mut T>` — single component, backward-compatible path.
    Single(FragmentRef),
    /// `Query<(Entity, &mut T, &U, ...)>` — tuple with optional Entity.
    Tuple {
        has_entity: bool,
        fragments: Vec<FragmentRef>,
    },
}

/// Information about one query parameter extracted from a system function signature.
struct QueryParam {
    /// The parameter name (e.g., `ants`).
    param_name: syn::Ident,
    /// What data the query accesses.
    data: QueryData,
    /// Query scope: primary (per-chunk) or global (all entities).
    scope: QueryScope,
    /// Filter tag types from With<Tag> (e.g., `AntTag`) — C++ archetype requirements.
    filter_tags: Vec<syn::Type>,
    /// Filter types from Without<T> (e.g., `Cooldown`) — per-entity Bevy component filters.
    without_filters: Vec<syn::Type>,
}

impl QueryParam {
    /// All fragment references in this query (1 for Single, N for Tuple).
    fn fragment_refs(&self) -> Vec<&FragmentRef> {
        match &self.data {
            QueryData::Single(frag) => vec![frag],
            QueryData::Tuple { fragments, .. } => fragments.iter().collect(),
        }
    }

    /// Whether this is a tuple query (needs facade struct codegen).
    fn is_tuple(&self) -> bool {
        matches!(&self.data, QueryData::Tuple { .. })
    }

    /// Whether the tuple includes Entity.
    fn has_entity(&self) -> bool {
        matches!(&self.data, QueryData::Tuple { has_entity: true, .. })
    }

    /// Whether this query needs shadow entity access (Entity or Without filters).
    fn needs_entity_map(&self) -> bool {
        self.has_entity() || !self.without_filters.is_empty()
    }

    /// For backward compat: single fragment type (panics on Tuple).
    fn single_fragment_type(&self) -> &syn::Type {
        match &self.data {
            QueryData::Single(frag) => &frag.fragment_type,
            QueryData::Tuple { .. } => panic!("single_fragment_type called on Tuple query"),
        }
    }

    /// For backward compat: single is_mutable (panics on Tuple).
    fn single_is_mutable(&self) -> bool {
        match &self.data {
            QueryData::Single(frag) => frag.is_mutable,
            QueryData::Tuple { .. } => panic!("single_is_mutable called on Tuple query"),
        }
    }

    /// For backward compat: single scope_index (panics on Tuple).
    fn single_scope_index(&self) -> usize {
        match &self.data {
            QueryData::Single(frag) => frag.scope_index,
            QueryData::Tuple { .. } => panic!("single_scope_index called on Tuple query"),
        }
    }
}

/// Information about one resource parameter (Res<T> or ResMut<T>).
struct ResourceParam {
    /// The parameter name (e.g., `spatial`).
    param_name: syn::Ident,
    /// The resource type (e.g., `MassSpatialQueries`).
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

/// Parses a function signature and extracts MassQuery/MassQueryAll/Query parameters.
/// Non-query params (like `dt: f32`) are treated as chunk-level data.
fn extract_query_params(func: &ItemFn) -> syn::Result<Vec<QueryParam>> {
    let mut params = Vec::new();
    let mut primary_index = 0;
    let mut global_index = 0;

    for arg in &func.sig.inputs {
        let FnArg::Typed(pat_type) = arg else {
            continue;
        };

        // #[bevy] attribute opts out of chunk rewriting — param passes through
        // as a real Bevy query (like BevyQuery, but using plain Query type).
        let has_bevy_attr = pat_type.attrs.iter().any(|a| a.path.is_ident("bevy"));
        if has_bevy_attr {
            continue;
        }

        if let Type::Path(type_path) = &*pat_type.ty {
            let last_seg = type_path.path.segments.last();
            if let Some(seg) = last_seg {
                let scope = if seg.ident == "MassQuery" || seg.ident == "Query" {
                    Some(QueryScope::Primary)
                } else if seg.ident == "MassQueryAll" || seg.ident == "QueryAll" {
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

                    let parsed = extract_query_inner_type(seg)?;

                    let data = match parsed.data {
                        ParsedQueryData::Single { fragment_type, is_mutable } => {
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
                            QueryData::Single(FragmentRef {
                                fragment_type,
                                is_mutable,
                                scope_index,
                            })
                        }
                        ParsedQueryData::Tuple { has_entity, fragments } => {
                            let frags = fragments
                                .into_iter()
                                .map(|(fragment_type, is_mutable)| {
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
                                    FragmentRef {
                                        fragment_type,
                                        is_mutable,
                                        scope_index,
                                    }
                                })
                                .collect();
                            QueryData::Tuple {
                                has_entity,
                                fragments: frags,
                            }
                        }
                    };

                    params.push(QueryParam {
                        param_name,
                        data,
                        scope,
                        filter_tags: parsed.filter_tags,
                        without_filters: parsed.without_filters,
                    });
                }
            }
        }
    }

    Ok(params)
}

/// Collects function parameters that are neither MassQuery/Query nor Res/ResMut
/// nor `dt: f32`. These are "passthrough" params — real Bevy system params like
/// `BevyQuery<D, F>` or `Commands` that the macro should not rewrite.
/// They are forwarded unchanged to the Bevy wrapper as real system params.
fn extract_passthrough_params(
    func: &ItemFn,
    query_params: &[QueryParam],
    resource_params: &[ResourceParam],
) -> Vec<TokenStream> {
    func.sig
        .inputs
        .iter()
        .filter_map(|arg| {
            let FnArg::Typed(pat_type) = arg else {
                return None;
            };
            let param_name = match &*pat_type.pat {
                Pat::Ident(pat_ident) => &pat_ident.ident,
                _ => return None,
            };

            // Skip known query params (MassQuery, Query, MassQueryAll)
            if query_params.iter().any(|p| p.param_name == *param_name) {
                return None;
            }
            // Skip known resource params (Res<T>, ResMut<T>)
            if resource_params.iter().any(|r| r.param_name == *param_name) {
                return None;
            }
            // Skip `dt: f32` (legacy parameter)
            if *param_name == "dt" {
                return None;
            }

            // Everything else (BevyQuery, Commands, etc.) passes through
            Some(quote! { #arg })
        })
        .collect()
}

/// Extracts the inner type `T` from any `MessageWriter<T>` or `MessageReader<T>` params.
/// Used to auto-register message types in `init_resources`.
fn extract_message_types(func: &ItemFn) -> Vec<syn::Type> {
    let mut types = Vec::new();
    for arg in &func.sig.inputs {
        let FnArg::Typed(pat_type) = arg else { continue };
        if let Type::Path(type_path) = &*pat_type.ty {
            if let Some(seg) = type_path.path.segments.last() {
                if seg.ident == "MessageWriter" || seg.ident == "MessageReader" {
                    if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                        if let Some(syn::GenericArgument::Type(ty)) = args.args.first() {
                            if !types.iter().any(|t: &syn::Type| *t == *ty) {
                                types.push(ty.clone());
                            }
                        }
                    }
                }
            }
        }
    }
    types
}

/// Result of parsing a Query's generic arguments.
struct ParsedQueryArgs {
    /// Single reference or tuple of references.
    data: ParsedQueryData,
    /// With<T> tags — C++ archetype requirements.
    filter_tags: Vec<syn::Type>,
    /// Without<T> types — per-entity Bevy component filters.
    without_filters: Vec<syn::Type>,
}

enum ParsedQueryData {
    /// `&T` or `&mut T` — single fragment reference.
    Single { fragment_type: syn::Type, is_mutable: bool },
    /// `(Entity, &mut T, &U, ...)` — tuple with optional Entity.
    Tuple { has_entity: bool, fragments: Vec<(syn::Type, bool)> },
}

/// Parse a single reference: `&T` → (T, false), `&mut T` → (T, true).
fn parse_reference(ty: &Type) -> Option<(syn::Type, bool)> {
    match ty {
        Type::Reference(TypeReference { mutability, elem, .. }) => {
            Some((*elem.clone(), mutability.is_some()))
        }
        _ => None,
    }
}

/// Check if a type is the `Entity` ident.
fn is_entity_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(seg) = type_path.path.segments.last() {
            return seg.ident == "Entity";
        }
    }
    false
}

/// Extract With/Without filters from a single filter type argument.
/// Handles both direct types (`With<Tag>`) and tuples (`(With<Tag>, Without<V>)`).
fn extract_filters(
    ty: &Type,
    filter_tags: &mut Vec<syn::Type>,
    without_filters: &mut Vec<syn::Type>,
) -> syn::Result<()> {
    match ty {
        Type::Tuple(tuple) => {
            for elem in &tuple.elems {
                extract_filters(elem, filter_tags, without_filters)?;
            }
        }
        Type::Path(type_path) => {
            let Some(seg) = type_path.path.segments.last() else {
                return Ok(());
            };
            if seg.ident == "With" {
                let syn::PathArguments::AngleBracketed(args) = &seg.arguments else {
                    return Err(syn::Error::new_spanned(seg, "With requires a type parameter"));
                };
                for arg in &args.args {
                    let syn::GenericArgument::Type(tag_type) = arg else {
                        return Err(syn::Error::new_spanned(arg, "expected type argument in With<>"));
                    };
                    filter_tags.push(tag_type.clone());
                }
            } else if seg.ident == "Without" {
                let syn::PathArguments::AngleBracketed(args) = &seg.arguments else {
                    return Err(syn::Error::new_spanned(seg, "Without requires a type parameter"));
                };
                for arg in &args.args {
                    let syn::GenericArgument::Type(filter_type) = arg else {
                        return Err(syn::Error::new_spanned(arg, "expected type argument in Without<>"));
                    };
                    without_filters.push(filter_type.clone());
                }
            }
        }
        _ => {}
    }
    Ok(())
}

/// Extracts the data shape, filter tags, and without filters from a Query's generic args.
///
/// Handles:
/// - `Query<&T>` / `Query<&mut T>` — single component
/// - `Query<(Entity, &mut T, &U)>` — tuple with optional Entity
/// - `Query<..., With<Tag>>` — tag filter (C++ requirement)
/// - `Query<..., Without<V>>` — component filter (Bevy shadow entity)
/// - `Query<..., (With<Tag>, Without<V>)>` — tuple of filters
fn extract_query_inner_type(
    seg: &syn::PathSegment,
) -> syn::Result<ParsedQueryArgs> {
    let syn::PathArguments::AngleBracketed(args) = &seg.arguments else {
        return Err(syn::Error::new_spanned(seg, "query type requires type parameter"));
    };

    let first_arg = args.args.first().ok_or_else(|| {
        syn::Error::new_spanned(seg, "query type requires one type parameter")
    })?;

    let syn::GenericArgument::Type(inner_ty) = first_arg else {
        return Err(syn::Error::new_spanned(first_arg, "expected type argument"));
    };

    // Determine data shape: single reference or tuple
    let data = if let Type::Tuple(tuple) = inner_ty {
        // Tuple: (Entity, &mut T, &U, ...)
        let mut has_entity = false;
        let mut fragments = Vec::new();
        for elem in &tuple.elems {
            if is_entity_type(elem) {
                has_entity = true;
            } else if let Some((frag_type, is_mutable)) = parse_reference(elem) {
                fragments.push((frag_type, is_mutable));
            } else {
                return Err(syn::Error::new_spanned(
                    elem,
                    "tuple Query elements must be Entity, &T, or &mut T",
                ));
            }
        }
        if fragments.is_empty() {
            return Err(syn::Error::new_spanned(
                inner_ty,
                "tuple Query must contain at least one fragment reference",
            ));
        }
        ParsedQueryData::Tuple { has_entity, fragments }
    } else if let Some((frag_type, is_mutable)) = parse_reference(inner_ty) {
        // Single: &T or &mut T
        ParsedQueryData::Single { fragment_type: frag_type, is_mutable }
    } else {
        // Bare type (no reference) — treat as immutable single
        ParsedQueryData::Single { fragment_type: inner_ty.clone(), is_mutable: false }
    };

    // Parse filter arguments (second, third, ... type args): With<T>, Without<T>, or tuple
    let mut filter_tags = Vec::new();
    let mut without_filters = Vec::new();
    for arg in args.args.iter().skip(1) {
        let syn::GenericArgument::Type(filter_ty) = arg else {
            continue;
        };
        extract_filters(filter_ty, &mut filter_tags, &mut without_filters)?;
    }

    Ok(ParsedQueryArgs { data, filter_tags, without_filters })
}

/// Parsed mass_system attribute: `order = N, entity_group = "name"`.
pub struct MassSystemAttr {
    pub order: u32,
    pub entity_group: Option<String>,
}

/// Parse `#[mass_system(order = N)]` or `#[mass_system(order = N, entity_group = "name")]`.
pub fn parse_mass_system_attr_full(attr: TokenStream) -> Option<MassSystemAttr> {
    if attr.is_empty() {
        return None;
    }

    let mut order: Option<u32> = None;
    let mut entity_group: Option<String> = None;

    // Try parsing as comma-separated assignments: `order = 30, entity_group = "ants"`
    // First try as a single assignment (backward compat)
    let parsed_single: syn::Result<syn::ExprAssign> = syn::parse2(attr.clone());
    if let Ok(assign) = parsed_single {
        parse_assignment(&assign, &mut order, &mut entity_group);
        if let Some(ord) = order {
            return Some(MassSystemAttr { order: ord, entity_group });
        }
    }

    // Try parsing as punctuated list of assignments using a custom parse step
    struct AssignList(syn::punctuated::Punctuated<syn::ExprAssign, syn::Token![,]>);
    impl syn::parse::Parse for AssignList {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Ok(AssignList(
                syn::punctuated::Punctuated::parse_terminated(input)?,
            ))
        }
    }
    if let Ok(AssignList(assigns)) = syn::parse2::<AssignList>(attr.clone()) {
        for assign in &assigns {
            parse_assignment(assign, &mut order, &mut entity_group);
        }
    }

    order.map(|ord| MassSystemAttr { order: ord, entity_group })
}

fn parse_assignment(
    assign: &syn::ExprAssign,
    order: &mut Option<u32>,
    entity_group: &mut Option<String>,
) {
    if let syn::Expr::Path(ref path) = *assign.left {
        if path.path.is_ident("order") {
            if let syn::Expr::Lit(ref lit) = *assign.right {
                if let syn::Lit::Int(ref int_lit) = lit.lit {
                    *order = int_lit.base10_parse::<u32>().ok();
                }
            }
        } else if path.path.is_ident("entity_group") {
            if let syn::Expr::Lit(ref lit) = *assign.right {
                if let syn::Lit::Str(ref str_lit) = lit.lit {
                    *entity_group = Some(str_lit.value());
                }
            }
        }
    }
}

pub fn mass_system_impl(func: &ItemFn, order: u32, entity_group: Option<&str>) -> syn::Result<TokenStream> {
    let func_name = &func.sig.ident;
    let wrapper_name = format_ident!("__mass_system_{}", func_name);
    let reg_name = format_ident!("__mass_system_reg_{}", func_name);
    let system_name_str = func_name.to_string();

    let query_params = extract_query_params(func)?;

    // Create a cleaned copy of the function with #[bevy] attrs stripped from params.
    // extract_query_params above needed the original attrs; everything below uses the cleaned version.
    // Create a cleaned copy of the function with #[bevy] attrs stripped from params.
    // For #[bevy] params with type `Query<...>`, rewrite to `bevy_ecs::system::Query<...>`
    // so the generated code uses real Bevy queries (not the facade marker type).
    let func = &{
        let mut cleaned = func.clone();
        for arg in &mut cleaned.sig.inputs {
            if let FnArg::Typed(pat_type) = arg {
                let had_bevy = pat_type.attrs.iter().any(|a| a.path.is_ident("bevy"));
                pat_type.attrs.retain(|a| !a.path.is_ident("bevy"));
                if had_bevy {
                    // Rewrite Query<D, F> → bevy_ecs::system::Query<D, F>
                    if let Type::Path(type_path) = &*pat_type.ty {
                        if let Some(seg) = type_path.path.segments.last() {
                            if seg.ident == "Query" {
                                let args = &seg.arguments;
                                pat_type.ty = Box::new(syn::parse2(
                                    quote! { bevy_ecs::system::Query #args }
                                ).unwrap());
                            }
                        }
                    }
                }
            }
        }
        cleaned
    };

    let resource_params = extract_resource_params(func)?;
    let passthrough_params = extract_passthrough_params(func, &query_params, &resource_params);
    let message_types = extract_message_types(func);

    // Detect whether the system uses `dt: f32` (legacy chunk-level delta time)
    let has_dt = func.sig.inputs.iter().any(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                return pat_ident.ident == "dt";
            }
        }
        false
    });

    // Detect tuple queries (need facade struct codegen and Bevy-only dispatch)
    let has_tuple_queries = query_params.iter().any(|p| p.is_tuple());
    let has_without_filters = query_params.iter().any(|p| !p.without_filters.is_empty());
    let has_single_primary = query_params.iter().any(|p| !p.is_tuple() && p.scope == QueryScope::Primary);

    // Generate fragment unpacking code (C++ direct dispatch — only for single queries)
    let unpack_stmts: Vec<TokenStream> = query_params
        .iter()
        .filter(|p| !p.is_tuple())
        .map(|p| {
            let param_name = &p.param_name;
            let frag_type = p.single_fragment_type();
            let idx = p.single_scope_index();

            match (p.scope, p.single_is_mutable()) {
                (QueryScope::Primary, true) => {
                    quote! {
                        let mut #param_name = ::unreal_api::mass::DualQueryMut::from_chunk(unsafe {
                            ::unreal_api::mass::MassQueryMut::<#frag_type>::from_raw(
                                (*chunk).fragments.add(#idx).read().data,
                                (*chunk).fragments.add(#idx).read().count as usize,
                            ).into_slice()
                        });
                    }
                }
                (QueryScope::Primary, false) => {
                    quote! {
                        let #param_name = ::unreal_api::mass::DualQueryRef::from_chunk(unsafe {
                            ::unreal_api::mass::MassQueryRef::<#frag_type>::from_raw(
                                (*chunk).fragments.add(#idx).read().data as *const ::std::ffi::c_void,
                                (*chunk).fragments.add(#idx).read().count as usize,
                            ).into_slice()
                        });
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

    // Generate requirement descriptors — one per fragment across all query params
    let requirement_entries: Vec<TokenStream> = query_params
        .iter()
        .flat_map(|p| {
            let scope = match p.scope {
                QueryScope::Primary => 0u8,
                QueryScope::Global => 1u8,
            };
            p.fragment_refs().into_iter().map(move |frag| {
                let frag_type = &frag.fragment_type;
                let access = if frag.is_mutable { 1u8 } else { 0u8 };
                quote! {
                    ::unreal_api::mass::MassSystemRequirement {
                        cpp_type_name: <#frag_type as ::unreal_api::mass::MaybeFragment>::CPP_TYPE_NAME_OR_EMPTY,
                        size: ::std::mem::size_of::<#frag_type>(),
                        align: ::std::mem::align_of::<#frag_type>(),
                        access_mode: #access,
                        is_tag: 0,
                        query_scope: #scope,
                        is_valid: <#frag_type as ::unreal_api::mass::MaybeFragment>::IS_FRAGMENT,
                    }
                }
            }).collect::<Vec<_>>()
        })
        .collect();

    // Collect unique filter tags across all query params and emit as additional requirements.
    // Tags inherit their parent query param's scope: Primary → scope 0, Global → scope 1.
    // Deduplicate by (type, scope) — same tag can appear in both primary and global queries.
    let mut seen_tags = std::collections::HashSet::new();
    let tag_requirement_entries: Vec<TokenStream> = query_params
        .iter()
        .flat_map(|p| p.filter_tags.iter().map(move |t| (t, p.scope)))
        .filter(|(tag_type, scope)| {
            let key = (quote!(#tag_type).to_string(), *scope as u8);
            seen_tags.insert(key)
        })
        .map(|(tag_type, scope)| {
            let scope_val = match scope {
                QueryScope::Primary => 0u8,
                QueryScope::Global => 1u8,
            };
            quote! {
                ::unreal_api::mass::MassSystemRequirement {
                    cpp_type_name: <#tag_type as ::unreal_api::mass::MassFragment>::CPP_TYPE_NAME,
                    size: 0,
                    align: 1,
                    access_mode: 0,
                    is_tag: 1,
                    query_scope: #scope_val,
                    is_valid: true,
                }
            }
        })
        .collect();

    let all_requirement_entries: Vec<&TokenStream> = requirement_entries.iter()
        .chain(tag_requirement_entries.iter())
        .collect();
    let num_requirements = all_requirement_entries.len();

    // Compile-time guard: all primary query fragment types must have the same IS_CHUNK value.
    // Mixing chunk-backed and Bevy-only fragments in one system's primary queries would cause
    // the Bevy-only data to be re-iterated on every chunk iteration (double-mutation bug).
    let primary_frag_types: Vec<&syn::Type> = query_params
        .iter()
        .filter(|p| p.scope == QueryScope::Primary)
        .flat_map(|p| p.fragment_refs().into_iter().map(|f| &f.fragment_type))
        .collect();
    let chunk_consistency_assert = if primary_frag_types.len() >= 2 {
        let first = &primary_frag_types[0];
        let checks: Vec<TokenStream> = primary_frag_types[1..].iter().map(|t| {
            quote! {
                <#first as ::unreal_api::mass::QueryBackend>::IS_CHUNK
                    == <#t as ::unreal_api::mass::QueryBackend>::IS_CHUNK
            }
        }).collect();
        quote! {
            const _: () = assert!(
                #(#checks)&&*,
                "mixed chunk-backed and Bevy-only fragments in one system's primary queries are not supported — split into separate #[mass_system] functions, one per storage kind"
            );
        }
    } else {
        quote! {}
    };

    // Generate facade struct names for tuple queries
    let facade_names: std::collections::HashMap<String, (syn::Ident, syn::Ident)> = query_params
        .iter()
        .filter(|p| p.is_tuple())
        .map(|p| {
            let struct_name = format_ident!("__FQ_{}_{}", func_name, p.param_name);
            let iter_name = format_ident!("__FQ_{}_{}_Iter", func_name, p.param_name);
            (p.param_name.to_string(), (struct_name, iter_name))
        })
        .collect();

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
                if let Some(_seg) = type_path.path.segments.last() {
                    if let Some(qp) = query_params.iter().find(|p| p.param_name == *param_name) {
                        if qp.is_tuple() {
                            // Tuple query → generated facade struct
                            let (struct_name, _) = &facade_names[&qp.param_name.to_string()];
                            return quote! { mut #param_name: #struct_name<'_> };
                        }
                        // Single query → DualQuery types (support both chunk and Bevy backing)
                        let inner = qp.single_fragment_type();
                        return match (qp.scope, qp.single_is_mutable()) {
                            (QueryScope::Primary, true) => {
                                quote! { mut #param_name: ::unreal_api::mass::DualQueryMut<'_, #inner> }
                            }
                            (QueryScope::Primary, false) => {
                                quote! { #param_name: ::unreal_api::mass::DualQueryRef<'_, #inner> }
                            }
                            (QueryScope::Global, true) => {
                                quote! { mut #param_name: ::unreal_api::mass::MassQueryAllMut<'_, #inner> }
                            }
                            (QueryScope::Global, false) => {
                                quote! { #param_name: ::unreal_api::mass::MassQueryAllRef<'_, #inner> }
                            }
                        };
                    }
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

            // MessageWriter/MessageReader: rewrite to &mut T so they survive the chunk loop.
            // These Bevy system params are non-Copy and would be moved on the first chunk
            // iteration, causing a use-after-move error on subsequent chunks.
            if let Type::Path(type_path) = &*pat_type.ty {
                if let Some(seg) = type_path.path.segments.last() {
                    if seg.ident == "MessageWriter" || seg.ident == "MessageReader" {
                        let ty = &pat_type.ty;
                        return quote! { #param_name: &mut #ty };
                    }
                }
            }

            // Passthrough param: preserve as-is.
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
    let global_marker_name = format_ident!("__mass_marker_{}_global", func_name);

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

    // Bevy system params: one Res/ResMut<MassSystemChunks<Marker, T>> per fragment, plus Res<Time> for dt
    // Global-scope queries use a separate marker to avoid Res/ResMut conflicts when the
    // same fragment type appears in both a primary and a global query within one system.
    //
    // For single primary queries, we also emit a bevy_ecs::system::Query<D, F> param
    // alongside the MassSystemChunks param — this enables dual-mode dispatch where
    // the const-if selects chunk or Bevy source based on QueryBackend::IS_CHUNK.
    let bevy_params: Vec<TokenStream> = query_params
        .iter()
        .flat_map(|p| {
            let scope_marker = if p.scope == QueryScope::Global { &global_marker_name } else { &marker_name };
            let mut params: Vec<TokenStream> = p.fragment_refs().into_iter().enumerate().map(|(i, frag)| {
                let frag_type = &frag.fragment_type;
                // For single queries: __mass_{param_name}
                // For tuple queries: __mass_{param_name}_{i}
                let param_name = if p.is_tuple() {
                    format_ident!("__mass_{}_{}", p.param_name, i)
                } else {
                    format_ident!("__mass_{}", p.param_name)
                };
                if frag.is_mutable {
                    quote! { mut #param_name: ::unreal_api::ecs::prelude::ResMut<::unreal_api::mass::MassSystemChunks<#scope_marker, #frag_type>> }
                } else {
                    quote! { #param_name: ::unreal_api::ecs::prelude::Res<::unreal_api::mass::MassSystemChunks<#scope_marker, #frag_type>> }
                }
            }).collect();

            // For primary queries: add a bevy_ecs::system::Query param for dual-mode dispatch.
            // The Query will be empty for chunk-backed types (no Bevy entities have those components),
            // and populated for Bevy-only types (chunk resource will be empty instead).
            if p.scope == QueryScope::Primary {
                let bevy_param_name = format_ident!("__bevy_{}", p.param_name);
                if p.is_tuple() {
                    let QueryData::Tuple { has_entity, fragments } = &p.data else { unreachable!() };
                    // Build tuple data type: (Entity, &mut T, &U, ...)
                    let mut tuple_elems: Vec<TokenStream> = Vec::new();
                    if *has_entity {
                        tuple_elems.push(quote! { ::unreal_api::ecs::entity::Entity });
                    }
                    for frag in fragments {
                        let ft = &frag.fragment_type;
                        if frag.is_mutable {
                            tuple_elems.push(quote! { &mut #ft });
                        } else {
                            tuple_elems.push(quote! { &#ft });
                        }
                    }
                    let data_type = quote! { (#(#tuple_elems),*) };

                    // Build filter type from filter_tags and without_filters
                    let filter_parts: Vec<TokenStream> = p.filter_tags.iter()
                        .map(|t| quote! { ::unreal_api::ecs::prelude::With<#t> })
                        .chain(p.without_filters.iter().map(|t| quote! { ::unreal_api::ecs::prelude::Without<#t> }))
                        .collect();
                    if filter_parts.is_empty() {
                        params.push(quote! {
                            mut #bevy_param_name: ::bevy_ecs::system::Query<'_, '_, #data_type>
                        });
                    } else if filter_parts.len() == 1 {
                        let f = &filter_parts[0];
                        params.push(quote! {
                            mut #bevy_param_name: ::bevy_ecs::system::Query<'_, '_, #data_type, #f>
                        });
                    } else {
                        params.push(quote! {
                            mut #bevy_param_name: ::bevy_ecs::system::Query<'_, '_, #data_type, (#(#filter_parts),*)>
                        });
                    }
                } else {
                    let frag_type = p.single_fragment_type();
                    if p.single_is_mutable() {
                        params.push(quote! {
                            mut #bevy_param_name: ::bevy_ecs::system::Query<'_, '_, &mut #frag_type>
                        });
                    } else {
                        params.push(quote! {
                            #bevy_param_name: ::bevy_ecs::system::Query<'_, '_, &#frag_type>
                        });
                    }
                }
            }

            params
        })
        .collect();

    // Pre-loop statements: for single primary queries backed by Bevy (non-chunk),
    // collect pointers from the bevy_ecs::system::Query before the chunk loop.
    // For chunk-backed types these are Empty (the bevy Query has no matching entities).
    let pre_loop_bevy_stmts: Vec<TokenStream> = query_params
        .iter()
        .filter(|p| !p.is_tuple() && p.scope == QueryScope::Primary)
        .map(|p| {
            let pre_name = format_ident!("__pre_{}", p.param_name);
            let bevy_param_name = format_ident!("__bevy_{}", p.param_name);
            let frag_type = p.single_fragment_type();
            if p.single_is_mutable() {
                quote! {
                    let mut #pre_name = if !<#frag_type as ::unreal_api::mass::QueryBackend>::IS_CHUNK {
                        let ptrs: Vec<*mut #frag_type> = #bevy_param_name.iter_mut()
                            .map(|mut r| &mut *r as *mut #frag_type)
                            .collect();
                        ::unreal_api::mass::DualQueryMut::Bevy(ptrs, ::std::marker::PhantomData)
                    } else {
                        ::unreal_api::mass::DualQueryMut::Empty
                    };
                }
            } else {
                quote! {
                    let mut #pre_name = if !<#frag_type as ::unreal_api::mass::QueryBackend>::IS_CHUNK {
                        let ptrs: Vec<*const #frag_type> = #bevy_param_name.iter()
                            .map(|r| &*r as *const #frag_type)
                            .collect();
                        ::unreal_api::mass::DualQueryRef::Bevy(ptrs, ::std::marker::PhantomData)
                    } else {
                        ::unreal_api::mass::DualQueryRef::Empty
                    };
                }
            }
        })
        .collect();

    // Pre-loop statements for tuple primary queries: declare per-component Vecs and
    // conditionally collect from the bevy_ecs::system::Query when fragments are Bevy-only.
    // Safety: the Vecs must not be resized or dropped while the facade struct's raw
    // pointers are live. Commands defers mutations, so this is safe today.
    let pre_loop_bevy_tuple_stmts: Vec<TokenStream> = query_params
        .iter()
        .filter(|p| p.is_tuple() && p.scope == QueryScope::Primary)
        .flat_map(|p| {
            let QueryData::Tuple { has_entity, fragments } = &p.data else { unreachable!() };
            let bevy_param_name = format_ident!("__bevy_{}", p.param_name);
            let first_frag_type = &fragments[0].fragment_type;

            let mut stmts: Vec<TokenStream> = Vec::new();

            // Declare Vecs for each fragment (values for contiguous storage).
            // For mutable fragments, also declare a pointer Vec for write-back.
            for (i, frag) in fragments.iter().enumerate() {
                let vec_name = format_ident!("__bvec_{}_{}", p.param_name, i);
                let ft = &frag.fragment_type;
                stmts.push(quote! { let mut #vec_name: Vec<#ft> = Vec::new(); });
                if frag.is_mutable {
                    let ptrs_name = format_ident!("__bptrs_{}_{}", p.param_name, i);
                    stmts.push(quote! { let mut #ptrs_name: Vec<*mut #ft> = Vec::new(); });
                }
            }

            // Entity vec (if tuple includes Entity)
            if *has_entity {
                let evec = format_ident!("__bvec_{}_entities", p.param_name);
                stmts.push(quote! { let mut #evec: Vec<::unreal_api::ecs::entity::Entity> = Vec::new(); });
            }

            // Build iteration pattern: (entity, mut v0, v1, ...)
            let mut pat_parts: Vec<TokenStream> = Vec::new();
            if *has_entity {
                pat_parts.push(quote! { __e });
            }
            for (i, frag) in fragments.iter().enumerate() {
                let var = format_ident!("__v{}", i);
                if frag.is_mutable {
                    pat_parts.push(quote! { mut #var });
                } else {
                    pat_parts.push(quote! { #var });
                }
            }
            let pattern = quote! { (#(#pat_parts),*) };

            // Push statements per component: collect values into contiguous Vecs
            // (the facade struct's iterator uses pointer arithmetic, requiring contiguous data).
            // For mutable fragments, we also save the original Bevy pointer for write-back.
            let mut push_stmts: Vec<TokenStream> = Vec::new();
            if *has_entity {
                let evec = format_ident!("__bvec_{}_entities", p.param_name);
                push_stmts.push(quote! { #evec.push(__e); });
            }
            for (i, frag) in fragments.iter().enumerate() {
                let vec_name = format_ident!("__bvec_{}_{}", p.param_name, i);
                let var = format_ident!("__v{}", i);
                if frag.is_mutable {
                    let ptrs_name = format_ident!("__bptrs_{}_{}", p.param_name, i);
                    push_stmts.push(quote! {
                        #ptrs_name.push(&mut *#var as *mut _);
                        #vec_name.push((*#var).clone());
                    });
                } else {
                    push_stmts.push(quote! { #vec_name.push((*#var).clone()); });
                }
            }

            stmts.push(quote! {
                if !<#first_frag_type as ::unreal_api::mass::QueryBackend>::IS_CHUNK {
                    for #pattern in #bevy_param_name.iter_mut() {
                        #(#push_stmts)*
                    }
                }
            });

            stmts
        })
        .collect();

    // Build the chunk iteration body for the Bevy wrapper.
    // Single primary queries use const-if dispatch: chunk-backed types read from
    // MassSystemChunks, Bevy-only types reborrow from the pre-loop DualQuery.
    let bevy_unpack_stmts: Vec<TokenStream> = query_params
        .iter()
        .flat_map(|p| {
            if p.is_tuple() {
                // Tuple query: unpack each fragment slice separately (always chunk-backed)
                p.fragment_refs().into_iter().enumerate().map(|(i, frag)| {
                    let slice_name = format_ident!("__slice_{}_{}", p.param_name, i);
                    let res_name = format_ident!("__mass_{}_{}", p.param_name, i);
                    match (p.scope, frag.is_mutable) {
                        (QueryScope::Primary, true) => {
                            quote! { let mut #slice_name = unsafe { #res_name.primary_chunk_mut(__chunk_idx) }; }
                        }
                        (QueryScope::Primary, false) => {
                            quote! { let #slice_name = unsafe { #res_name.primary_chunk_ref(__chunk_idx) }; }
                        }
                        (QueryScope::Global, _) => {
                            // Global queries in tuple: not yet supported
                            quote! {}
                        }
                    }
                }).collect::<Vec<_>>()
            } else {
                let param_name = &p.param_name;
                let res_name = format_ident!("__mass_{}", p.param_name);
                let stmt = match (p.scope, p.single_is_mutable()) {
                    (QueryScope::Primary, true) => {
                        // Const-if: chunk-backed reads from MassSystemChunks, Bevy-only reborrows pre-loop
                        let frag_type = p.single_fragment_type();
                        let pre_name = format_ident!("__pre_{}", p.param_name);
                        quote! {
                            let mut #param_name = if <#frag_type as ::unreal_api::mass::QueryBackend>::IS_CHUNK {
                                ::unreal_api::mass::DualQueryMut::from_chunk(
                                    unsafe { #res_name.primary_chunk_mut(__chunk_idx) }.into_slice()
                                )
                            } else {
                                #pre_name.reborrow()
                            };
                        }
                    }
                    (QueryScope::Primary, false) => {
                        let frag_type = p.single_fragment_type();
                        let pre_name = format_ident!("__pre_{}", p.param_name);
                        quote! {
                            let #param_name = if <#frag_type as ::unreal_api::mass::QueryBackend>::IS_CHUNK {
                                ::unreal_api::mass::DualQueryRef::from_chunk(
                                    unsafe { #res_name.primary_chunk_ref(__chunk_idx) }.into_slice()
                                )
                            } else {
                                #pre_name.reborrow()
                            };
                        }
                    }
                    (QueryScope::Global, true) => {
                        quote! {
                            let mut #param_name = unsafe { #res_name.global_query_mut() }
                                .unwrap_or_else(|| ::unreal_api::mass::MassQueryAllMut::empty());
                        }
                    }
                    (QueryScope::Global, false) => {
                        quote! {
                            let #param_name = unsafe { #res_name.global_query_ref() }
                                .unwrap_or_else(|| ::unreal_api::mass::MassQueryAllRef::empty());
                        }
                    }
                };
                vec![stmt]
            }
        })
        .collect();

    // Find the first primary fragment to determine chunk count
    let primary_count_source = query_params
        .iter()
        .find(|p| p.scope == QueryScope::Primary)
        .map(|p| {
            let res_name = if p.is_tuple() {
                format_ident!("__mass_{}_0", p.param_name)
            } else {
                format_ident!("__mass_{}", p.param_name)
            };
            quote! { #res_name.primary_chunk_count() }
        })
        .unwrap_or(quote! { 0 });

    // Bevy call args: map each function param to its Bevy wrapper equivalent
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
                        // Both single and tuple query params: pass the local variable
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
                return quote! { __mass_dt.delta_secs() };
            }
            // Non-Copy passthrough params must survive multiple chunk iterations.
            // Commands: use reborrow(). MessageWriter/MessageReader: pass &mut.
            if let Type::Path(type_path) = &*pat_type.ty {
                if let Some(seg) = type_path.path.segments.last() {
                    if seg.ident == "Commands" {
                        return quote! { #param_name.reborrow() };
                    }
                    if seg.ident == "MessageWriter" || seg.ident == "MessageReader" {
                        return quote! { &mut #param_name };
                    }
                }
            }
            quote! { #param_name }
        })
        .collect();

    // Resource initializers for MassBevySystemRegistration — one per fragment
    let chunk_init_stmts: Vec<TokenStream> = query_params
        .iter()
        .flat_map(|p| {
            let scope_marker = if p.scope == QueryScope::Global { &global_marker_name } else { &marker_name };
            p.fragment_refs().into_iter().map(move |frag| {
                let frag_type = &frag.fragment_type;
                quote! {
                    if !world.contains_resource::<::unreal_api::mass::MassSystemChunks<#scope_marker, #frag_type>>() {
                        world.insert_resource(::unreal_api::mass::MassSystemChunks::<#scope_marker, #frag_type>::new());
                    }
                }
            }).collect::<Vec<_>>()
        })
        .collect();

    // Auto-init `ResMut<T>` resources so game code doesn't need to manually insert them.
    // Does NOT auto-init `Res<T>` (read-only resources may need custom construction,
    // e.g. SpatialQuery). Requires `T: FromWorld` (blanket-impl'd for `T: Default`).
    let res_init_stmts: Vec<TokenStream> = resource_params
        .iter()
        .filter(|rp| rp.is_mutable)
        .map(|rp| {
            let ty = &rp.resource_type;
            quote! { world.init_resource::<#ty>(); }
        })
        .collect();

    let init_resource_stmts: Vec<TokenStream> =
        chunk_init_stmts.into_iter().chain(res_init_stmts).collect();

    // Auto-register message types via app.add_message::<T>() (deduplicates internally)
    let register_message_stmts: Vec<TokenStream> = message_types
        .iter()
        .map(|ty| {
            quote! {
                app.add_message::<#ty>();
            }
        })
        .collect();

    // Clear resources closure — one per fragment
    let clear_resource_stmts: Vec<TokenStream> = query_params
        .iter()
        .flat_map(|p| {
            let scope_marker = if p.scope == QueryScope::Global { &global_marker_name } else { &marker_name };
            p.fragment_refs().into_iter().map(move |frag| {
                let frag_type = &frag.fragment_type;
                quote! {
                    world.resource_mut::<::unreal_api::mass::MassSystemChunks<#scope_marker, #frag_type>>().clear();
                }
            }).collect::<Vec<_>>()
        })
        .collect();

    // Populate resources closure — one per fragment (skip Bevy-only types at runtime)
    let populate_primary_stmts: Vec<TokenStream> = query_params
        .iter()
        .filter(|p| p.scope == QueryScope::Primary)
        .flat_map(|p| {
            p.fragment_refs().into_iter().map(|frag| {
                let frag_type = &frag.fragment_type;
                let idx = frag.scope_index;
                quote! {
                    if <#frag_type as ::unreal_api::mass::MaybeFragment>::IS_FRAGMENT {
                        world.resource_mut::<::unreal_api::mass::MassSystemChunks<#marker_name, #frag_type>>()
                            .push_primary_slice(*chunk.fragments.add(#idx));
                    }
                }
            }).collect::<Vec<_>>()
        })
        .collect();

    let populate_global_stmts: Vec<TokenStream> = query_params
        .iter()
        .filter(|p| p.scope == QueryScope::Global)
        .flat_map(|p| {
            p.fragment_refs().into_iter().map(|frag| {
                let frag_type = &frag.fragment_type;
                let idx = frag.scope_index;
                quote! {
                    if <#frag_type as ::unreal_api::mass::MaybeFragment>::IS_FRAGMENT {
                        if world.resource::<::unreal_api::mass::MassSystemChunks<#global_marker_name, #frag_type>>().global().is_none() {
                            world.resource_mut::<::unreal_api::mass::MassSystemChunks<#global_marker_name, #frag_type>>()
                                .set_global(chunk.global_chunked_fragments.add(#idx));
                        }
                    }
                }
            }).collect::<Vec<_>>()
        })
        .collect();

    let has_resource_params = !resource_params.is_empty();
    let has_passthrough_params = !passthrough_params.is_empty();

    // C++ registration — generated when there are fragment requirements so C++
    // creates a processor that collects chunks for Bevy-scheduled dispatch.
    // Systems with Res<T>, passthrough params (BevyQuery, Commands, etc.),
    // tuple queries, or Without filters use a no-op execute_fn since C++ can't
    // provide these Bevy system params; only the Bevy schedule can dispatch them.
    //
    // Simple single-query systems keep C++ direct dispatch — the DualQuery wrapping
    // in the C++ path (from_chunk) works fine since C++ always provides chunk data.
    //
    // Systems with ZERO fragment requirements (pure BevyQuery + Commands) skip
    // C++ registration entirely — they run only via the Bevy schedule.
    // Mass Entity asserts on processors with empty queries.
    let needs_bevy_only_dispatch = has_resource_params || has_passthrough_params
        || has_tuple_queries || has_without_filters;
    let cpp_wrapper = if num_requirements == 0 {
        // No fragment requirements → pure Bevy system, no C++ processor needed
        quote! {}
    } else if needs_bevy_only_dispatch {
        quote! {
            #[cfg(feature = "unreal")]
            unsafe extern "C" fn #wrapper_name(_chunk: *const ::unreal_api::ffi::MassChunkData) {
                // No-op: this system uses Bevy resources and can only run via
                // the Bevy schedule, not direct C++ dispatch.
            }

            #[cfg(feature = "unreal")]
            #[allow(non_upper_case_globals)]
            static #reg_name: () = {
                const REQUIREMENTS: [::unreal_api::mass::MassSystemRequirement; #num_requirements] = [
                    #(#all_requirement_entries),*
                ];

                ::unreal_api::inventory::submit! {
                    ::unreal_api::mass::MassSystemRegistration {
                        name: #system_name_str,
                        execute_fn: #wrapper_name,
                        requirements: &REQUIREMENTS,
                        order: #order,
                    }
                }
            };
        }
    } else {
        quote! {
            #[cfg(feature = "unreal")]
            unsafe extern "C" fn #wrapper_name(chunk: *const ::unreal_api::ffi::MassChunkData) {
                if chunk.is_null() {
                    return;
                }
                #(#unpack_stmts)*
                #func_name(#(#call_args),*);
            }

            #[cfg(feature = "unreal")]
            #[allow(non_upper_case_globals)]
            static #reg_name: () = {
                const REQUIREMENTS: [::unreal_api::mass::MassSystemRequirement; #num_requirements] = [
                    #(#all_requirement_entries),*
                ];

                ::unreal_api::inventory::submit! {
                    ::unreal_api::mass::MassSystemRegistration {
                        name: #system_name_str,
                        execute_fn: #wrapper_name,
                        requirements: &REQUIREMENTS,
                        order: #order,
                    }
                }
            };
        }
    };

    // -----------------------------------------------------------------------
    // Facade struct + iterator generation for tuple queries
    // -----------------------------------------------------------------------

    let mut facade_struct_defs: Vec<TokenStream> = Vec::new();

    // Additional Bevy wrapper params for entity map and Without filters
    let mut extra_bevy_params: Vec<TokenStream> = Vec::new();

    // Whether we need the entity map resource
    let needs_entity_map = query_params.iter().any(|p| p.needs_entity_map());
    if needs_entity_map {
        extra_bevy_params.push(quote! {
            __entity_map: ::unreal_api::ecs::prelude::Res<::unreal_api::mass::MassEntityMap>
        });
    }

    // Collect unique Without filter types across all queries and generate filter query params
    let mut all_without_types: Vec<syn::Type> = Vec::new();
    let mut seen_without: std::collections::HashSet<String> = std::collections::HashSet::new();
    for p in &query_params {
        for wt in &p.without_filters {
            let key = quote!(#wt).to_string();
            if seen_without.insert(key) {
                let idx = all_without_types.len();
                let filter_param = format_ident!("__without_{}", idx);
                extra_bevy_params.push(quote! {
                    #filter_param: ::unreal_api::ecs::system::Query<'_, '_, (), ::unreal_api::ecs::prelude::Without<#wt>>
                });
                all_without_types.push(wt.clone());
            }
        }
    }

    // For each tuple query, generate the facade struct, iterator, and construction code
    let mut tuple_construct_stmts: Vec<TokenStream> = Vec::new();

    for p in query_params.iter().filter(|p| p.is_tuple()) {
        let (struct_name, iter_name) = &facade_names[&p.param_name.to_string()];
        let param_name = &p.param_name;

        let QueryData::Tuple { has_entity, fragments } = &p.data else { unreachable!() };

        // --- Generate struct fields ---
        let field_defs: Vec<TokenStream> = fragments.iter().enumerate().map(|(i, frag)| {
            let field_name = format_ident!("__p{}", i);
            let frag_type = &frag.fragment_type;
            if frag.is_mutable {
                quote! { #field_name: *mut #frag_type }
            } else {
                quote! { #field_name: *const #frag_type }
            }
        }).collect();

        let entity_field = if *has_entity {
            quote! { __entities: &'a [::unreal_api::ecs::entity::Entity], }
        } else {
            quote! {}
        };

        let filter_field = if !p.without_filters.is_empty() {
            quote! { __filter_mask: Vec<bool>, }
        } else {
            quote! {}
        };

        // --- Generate iterator Item type ---
        let item_types: Vec<TokenStream> = {
            let mut items = Vec::new();
            // Track position in fragments for Entity insertion
            // In the original tuple, Entity can be at any position.
            // For simplicity, Entity is always first if present.
            if *has_entity {
                items.push(quote! { ::unreal_api::ecs::entity::Entity });
            }
            for (_, frag) in fragments.iter().enumerate() {
                let frag_type = &frag.fragment_type;
                if frag.is_mutable {
                    items.push(quote! { &'a mut #frag_type });
                } else {
                    items.push(quote! { &'a #frag_type });
                }
            }
            items
        };
        let item_type = quote! { (#(#item_types),*) };

        // --- Generate iterator fields (copy raw pointers + metadata) ---
        let iter_field_defs: Vec<TokenStream> = fragments.iter().enumerate().map(|(i, frag)| {
            let field_name = format_ident!("__p{}", i);
            let frag_type = &frag.fragment_type;
            if frag.is_mutable {
                quote! { #field_name: *mut #frag_type }
            } else {
                quote! { #field_name: *const #frag_type }
            }
        }).collect();

        let iter_entity_field = if *has_entity {
            quote! { __entities: *const ::unreal_api::ecs::entity::Entity, }
        } else {
            quote! {}
        };

        let iter_filter_field = if !p.without_filters.is_empty() {
            quote! { __filter_mask: *const bool, }
        } else {
            quote! {}
        };

        // --- Generate iterator next() body ---
        let yield_fields: Vec<TokenStream> = {
            let mut yields = Vec::new();
            if *has_entity {
                yields.push(quote! { *self.__entities.add(__i) });
            }
            for (i, frag) in fragments.iter().enumerate() {
                let field_name = format_ident!("__p{}", i);
                if frag.is_mutable {
                    yields.push(quote! { &mut *self.#field_name.add(__i) });
                } else {
                    yields.push(quote! { &*self.#field_name.add(__i) });
                }
            }
            yields
        };

        let filter_check = if !p.without_filters.is_empty() {
            quote! { if unsafe { !*self.__filter_mask.add(__i) } { continue; } }
        } else {
            quote! {}
        };

        // --- Copy fields from struct to iterator ---
        let iter_copy_fields: Vec<TokenStream> = fragments.iter().enumerate().map(|(i, _)| {
            let field_name = format_ident!("__p{}", i);
            quote! { #field_name: self.#field_name }
        }).collect();

        let iter_copy_entity = if *has_entity {
            quote! { __entities: self.__entities.as_ptr(), }
        } else {
            quote! {}
        };

        let iter_copy_filter = if !p.without_filters.is_empty() {
            quote! { __filter_mask: self.__filter_mask.as_ptr(), }
        } else {
            quote! {}
        };

        // --- Emit struct + iterator + IntoIterator ---
        // Each item gets its own entry so the outer #[cfg(feature = "unreal")] applies to each.
        facade_struct_defs.push(quote! {
            #[allow(non_camel_case_types)]
            struct #struct_name<'a> {
                #(#field_defs,)*
                #entity_field
                #filter_field
                __len: usize,
                __phantom: ::std::marker::PhantomData<&'a ()>,
            }
        });

        facade_struct_defs.push(quote! {
            #[allow(non_camel_case_types)]
            struct #iter_name<'a> {
                #(#iter_field_defs,)*
                #iter_entity_field
                #iter_filter_field
                __len: usize,
                __idx: usize,
                __phantom: ::std::marker::PhantomData<&'a ()>,
            }
        });

        facade_struct_defs.push(quote! {
            impl<'a> Iterator for #iter_name<'a> {
                type Item = #item_type;

                fn next(&mut self) -> Option<Self::Item> {
                    loop {
                        if self.__idx >= self.__len {
                            return None;
                        }
                        let __i = self.__idx;
                        self.__idx += 1;
                        #filter_check
                        return Some(unsafe { (#(#yield_fields),*) });
                    }
                }
            }
        });

        facade_struct_defs.push(quote! {
            impl<'a> IntoIterator for &mut #struct_name<'a> {
                type Item = <#iter_name<'a> as Iterator>::Item;
                type IntoIter = #iter_name<'a>;
                fn into_iter(self) -> Self::IntoIter {
                    #iter_name {
                        #(#iter_copy_fields,)*
                        #iter_copy_entity
                        #iter_copy_filter
                        __len: self.__len,
                        __idx: 0,
                        __phantom: ::std::marker::PhantomData,
                    }
                }
            }
        });

        // --- Generate construction code for the Bevy wrapper ---
        let construct_fields: Vec<TokenStream> = fragments.iter().enumerate().map(|(i, frag)| {
            let field_name = format_ident!("__p{}", i);
            let slice_name = format_ident!("__slice_{}_{}", param_name, i);
            if frag.is_mutable {
                // MassQueryMut wraps &mut [T]; get raw pointer via as_slice + cast
                quote! { #field_name: #slice_name.as_slice().as_ptr() as *mut _ }
            } else {
                quote! { #field_name: #slice_name.as_slice().as_ptr() }
            }
        }).collect();

        let entity_construct = if *has_entity {
            quote! { __entities: __chunk_entities, }
        } else {
            quote! {}
        };

        // Build filter mask from Without filter queries
        let filter_construct = if !p.without_filters.is_empty() {
            // Find the indices of this param's Without types in the global list
            let filter_checks: Vec<TokenStream> = p.without_filters.iter().map(|wt| {
                let key = quote!(#wt).to_string();
                let idx = all_without_types.iter().position(|t| quote!(#t).to_string() == key).unwrap();
                let filter_param = format_ident!("__without_{}", idx);
                // Query is Query<(), Without<T>> — it matches entities that do NOT have T.
                // So get().is_ok() means the entity passes Without<T>.
                quote! { #filter_param.get(__e).is_ok() }
            }).collect();
            // All Without filters must pass (entity must NOT have any of them)
            quote! {
                let __filter_mask: Vec<bool> = __chunk_entities.iter()
                    .map(|&__e| { #(#filter_checks)&&* })
                    .collect();
            }
        } else {
            quote! {}
        };

        let filter_field_init = if !p.without_filters.is_empty() {
            quote! { __filter_mask: __filter_mask, }
        } else {
            quote! {}
        };

        // Length from first fragment slice
        let first_slice = format_ident!("__slice_{}_0", param_name);

        tuple_construct_stmts.push(quote! {
            #filter_construct
            let mut #param_name = #struct_name {
                #(#construct_fields,)*
                #entity_construct
                #filter_field_init
                __len: #first_slice.len(),
                __phantom: ::std::marker::PhantomData,
            };
        });

        // Entity offset tracking: add entity_offset increment after construction
        // (the entity slice is set up in the wrapper body)
    }

    // -----------------------------------------------------------------------
    // Wrapper body generation
    // -----------------------------------------------------------------------

    let has_primary_queries = query_params.iter().any(|p| p.scope == QueryScope::Primary);

    // For tuple queries needing entity map, add entity offset tracking.
    // Derive entity group from With<Tag>::ENTITY_GROUP (type-safe),
    // falling back to entity_group="name" attribute (stringly-typed).
    let pre_loop = if needs_entity_map && has_primary_queries {
        // Find the first With<Tag> from query params that need the entity map
        let tag_type = query_params.iter()
            .filter(|p| p.needs_entity_map())
            .flat_map(|p| p.filter_tags.iter())
            .next();

        let group_expr = if let Some(tag) = tag_type {
            // Type-safe: derived from With<Tag> filter — Tag must have ENTITY_GROUP const
            quote! { <#tag>::ENTITY_GROUP }
        } else if let Some(group) = entity_group {
            // Fallback: explicit entity_group attribute
            quote! { #group }
        } else if has_single_primary || query_params.iter().any(|p| p.is_tuple() && p.scope == QueryScope::Primary) {
            // Dual-mode: the chunk path is dead code for Bevy-only types (IS_CHUNK=false),
            // so entity_group is unreachable. Use a panic placeholder.
            quote! { panic!("unreachable: Bevy-only query has no entity group") }
        } else {
            return Err(syn::Error::new_spanned(
                &func.sig.ident,
                "system needs entity_group: add group=\"name\" to mass_tag! on the With<Tag> filter, \
                 or entity_group=\"name\" to the mass_system attribute",
            ));
        };

        quote! {
            let __group_entities = __entity_map.group(#group_expr).unwrap_or(&[]);
            let mut __entity_offset: usize = 0;
        }
    } else {
        quote! {}
    };

    // Entity slice setup per chunk
    let entity_slice_setup = if needs_entity_map && has_primary_queries {
        // Determine chunk length from the first primary fragment resource
        let first_primary = query_params.iter()
            .find(|p| p.scope == QueryScope::Primary)
            .unwrap();
        let first_len = if first_primary.is_tuple() {
            let slice_name = format_ident!("__slice_{}_0", first_primary.param_name);
            quote! { #slice_name.len() }
        } else {
            let param_name = &first_primary.param_name;
            quote! { #param_name.len() }
        };
        quote! {
            let __chunk_len = #first_len;
            let __chunk_entities = &__group_entities[__entity_offset..__entity_offset + __chunk_len];
        }
    } else {
        quote! {}
    };

    let post_chunk = if needs_entity_map && has_primary_queries {
        quote! { __entity_offset += __chunk_len; }
    } else {
        quote! {}
    };

    // Only inject Time resource when the system declares `dt: f32` (legacy shorthand)
    let bevy_dt_param: Vec<TokenStream> = if has_dt {
        vec![quote! { __mass_dt: ::unreal_api::ecs::prelude::Res<::bevy_time::Time>, }]
    } else {
        vec![]
    };

    // Global unpack stmts — these don't depend on __chunk_idx and must run
    // even when there are no primary queries (no chunk loop).
    let global_unpack_stmts: Vec<TokenStream> = query_params
        .iter()
        .filter(|p| p.scope == QueryScope::Global)
        .flat_map(|p| {
            if p.is_tuple() {
                // Global tuple queries not yet supported — shouldn't reach here
                vec![]
            } else {
                let param_name = &p.param_name;
                let res_name = format_ident!("__mass_{}", p.param_name);
                let stmt = if p.single_is_mutable() {
                    quote! {
                        let mut #param_name = unsafe { #res_name.global_query_mut() }
                            .unwrap_or_else(|| ::unreal_api::mass::MassQueryAllMut::empty());
                    }
                } else {
                    quote! {
                        let #param_name = unsafe { #res_name.global_query_ref() }
                            .unwrap_or_else(|| ::unreal_api::mass::MassQueryAllRef::empty());
                    }
                };
                vec![stmt]
            }
        })
        .collect();

    // Fallback unpack stmts for when chunk_count == 0 (all types are Bevy-only).
    // Single primary queries reborrow from the pre-loop DualQuery.
    let fallback_unpack_stmts: Vec<TokenStream> = query_params
        .iter()
        .filter(|p| !p.is_tuple() && p.scope == QueryScope::Primary)
        .map(|p| {
            let param_name = &p.param_name;
            let pre_name = format_ident!("__pre_{}", p.param_name);
            if p.single_is_mutable() {
                quote! { let mut #param_name = #pre_name.reborrow(); }
            } else {
                quote! { let #param_name = #pre_name.reborrow(); }
            }
        })
        .collect();

    // Fallback facade construction for tuple queries when chunk_count == 0 (Bevy-only).
    // Constructs facade structs from the pre-collected Vecs.
    let fallback_tuple_stmts: Vec<TokenStream> = query_params
        .iter()
        .filter(|p| p.is_tuple() && p.scope == QueryScope::Primary)
        .map(|p| {
            let (struct_name, _) = &facade_names[&p.param_name.to_string()];
            let param_name = &p.param_name;
            let QueryData::Tuple { has_entity, fragments } = &p.data else { unreachable!() };

            let field_assigns: Vec<TokenStream> = fragments.iter().enumerate().map(|(i, frag)| {
                let field_name = format_ident!("__p{}", i);
                let vec_name = format_ident!("__bvec_{}_{}", param_name, i);
                if frag.is_mutable {
                    quote! { #field_name: #vec_name.as_mut_ptr() as *mut _ }
                } else {
                    quote! { #field_name: #vec_name.as_ptr() }
                }
            }).collect();

            let entity_assign = if *has_entity {
                let evec = format_ident!("__bvec_{}_entities", param_name);
                quote! { __entities: &#evec[..], }
            } else {
                quote! {}
            };

            let filter_assign = if !p.without_filters.is_empty() {
                let first_vec = format_ident!("__bvec_{}_0", param_name);
                quote! { __filter_mask: vec![true; #first_vec.len()], }
            } else {
                quote! {}
            };

            let first_vec = format_ident!("__bvec_{}_0", param_name);
            quote! {
                let mut #param_name = #struct_name {
                    #(#field_assigns,)*
                    #entity_assign
                    #filter_assign
                    __len: #first_vec.len(),
                    __phantom: ::std::marker::PhantomData,
                };
            }
        })
        .collect();

    // Write-back statements: after the fallback system call, copy modified values
    // from the contiguous Vec back to the original Bevy entity storage.
    let fallback_writeback_stmts: Vec<TokenStream> = query_params
        .iter()
        .filter(|p| p.is_tuple() && p.scope == QueryScope::Primary)
        .flat_map(|p| {
            let QueryData::Tuple { fragments, .. } = &p.data else { unreachable!() };
            fragments.iter().enumerate().filter_map(|(i, frag)| {
                if frag.is_mutable {
                    let vec_name = format_ident!("__bvec_{}_{}", p.param_name, i);
                    let ptrs_name = format_ident!("__bptrs_{}_{}", p.param_name, i);
                    Some(quote! {
                        for (__idx, __val) in #vec_name.iter().enumerate() {
                            unsafe { *#ptrs_name[__idx] = __val.clone(); }
                        }
                    })
                } else {
                    None
                }
            }).collect::<Vec<_>>()
        })
        .collect();

    let has_primary_tuples = query_params.iter().any(|p| p.is_tuple() && p.scope == QueryScope::Primary);
    let has_dual_mode = has_single_primary || has_primary_tuples;

    let wrapper_body = if has_primary_queries {
        if has_dual_mode {
            // Dual-mode dispatch: pre-collect Bevy queries, then const-if in chunk loop.
            // If chunk_count == 0 (all types are Bevy-only or no data), run once with
            // Bevy-sourced data as fallback.
            quote! {
                #(#pre_loop_bevy_stmts)*
                #(#pre_loop_bevy_tuple_stmts)*
                let __chunk_count = #primary_count_source;
                if __chunk_count > 0 {
                    #pre_loop
                    for __chunk_idx in 0..__chunk_count {
                        #(#bevy_unpack_stmts)*
                        #entity_slice_setup
                        #(#tuple_construct_stmts)*
                        #func_name(#(#bevy_call_args),*);
                        #post_chunk
                    }
                } else {
                    // No chunks — pure Bevy fallback
                    #(#global_unpack_stmts)*
                    #(#fallback_unpack_stmts)*
                    #(#fallback_tuple_stmts)*
                    #func_name(#(#bevy_call_args),*);
                    #(#fallback_writeback_stmts)*
                }
            }
        } else {
            // Only tuple queries with no dual-mode — shouldn't happen since all
            // primary queries now get dual-mode, but keep as safety fallback
            quote! {
                #(#pre_loop_bevy_tuple_stmts)*
                #pre_loop
                for __chunk_idx in 0..#primary_count_source {
                    #(#bevy_unpack_stmts)*
                    #entity_slice_setup
                    #(#tuple_construct_stmts)*
                    #func_name(#(#bevy_call_args),*);
                    #post_chunk
                }
            }
        }
    } else {
        quote! {
            #(#global_unpack_stmts)*
            #func_name(#(#bevy_call_args),*);
        }
    };

    // Collect original function attributes (e.g., doc comments)
    let func_attrs = &func.attrs;
    let original_params = &func.sig.inputs;

    // Check if there are any global (QueryAll) params that need Bevy-mode rewriting
    let global_query_params: Vec<&QueryParam> = query_params
        .iter()
        .filter(|p| p.scope == QueryScope::Global)
        .collect();

    // Generate Bevy-mode output: either passthrough or wrapper with QueryAll rewriting
    let bevy_mode_output = if global_query_params.is_empty() {
        // No QueryAll params — simple passthrough
        quote! {
            #[cfg(not(feature = "unreal"))]
            #(#func_attrs)*
            #func_vis fn #func_name(#original_params) #func_ret
                #func_body
        }
    } else {
        // Has QueryAll params — generate wrapper that expands QueryAll to
        // Res<EntityIndex<Tag>> + Query<D, F> and constructs QueryAllWrapper.

        // Build the rewritten parameter list for Bevy mode:
        // - Non-QueryAll params pass through unchanged
        // - Each QueryAll<D, F> becomes two params: Res<EntityIndex<Tag>> + Query<D, F>
        let mut bevy_mode_params: Vec<TokenStream> = Vec::new();
        let mut query_all_setup_stmts: Vec<TokenStream> = Vec::new();

        for arg in &func.sig.inputs {
            let FnArg::Typed(pat_type) = arg else { continue };
            let param_name = match &*pat_type.pat {
                Pat::Ident(pat_ident) => &pat_ident.ident,
                _ => {
                    // Pass through non-ident patterns
                    bevy_mode_params.push(quote! { #arg });
                    continue;
                }
            };

            // Check if this is a QueryAll param
            let is_query_all = global_query_params.iter().any(|p| p.param_name == *param_name);

            if is_query_all {
                let qp = global_query_params.iter().find(|p| p.param_name == *param_name).unwrap();

                // Extract fragment types and build Query<D, F> type
                let frag_refs = qp.fragment_refs();
                let (frag_type, is_mutable) = match &qp.data {
                    QueryData::Single(frag) => (&frag.fragment_type, frag.is_mutable),
                    QueryData::Tuple { .. } => {
                        // For now, only support single-fragment QueryAll
                        // (which is what all current usage requires)
                        let first = &frag_refs[0];
                        (&first.fragment_type, first.is_mutable)
                    }
                };

                // Determine the tag type for EntityIndex
                // Use the first With<Tag> filter if present, otherwise use the fragment type
                let tag_type: &syn::Type = if !qp.filter_tags.is_empty() {
                    &qp.filter_tags[0]
                } else {
                    frag_type
                };

                // Build the data type for the Query: &T or &mut T
                let data_type = if is_mutable {
                    quote! { &mut #frag_type }
                } else {
                    quote! { &#frag_type }
                };

                // Build the filter type for the Query
                let filter_type = if !qp.filter_tags.is_empty() {
                    let tags = &qp.filter_tags;
                    if tags.len() == 1 {
                        quote! { With<#(#tags)*> }
                    } else {
                        quote! { (#(With<#tags>),*) }
                    }
                } else {
                    quote! { () }
                };

                let idx_name = format_ident!("__qa_{}_idx", param_name);
                let query_name = format_ident!("__qa_{}_q", param_name);

                bevy_mode_params.push(quote! {
                    #idx_name: bevy_ecs::prelude::Res<bevy_mass::query_all::EntityIndex<#tag_type>>
                });
                bevy_mode_params.push(quote! {
                    mut #query_name: bevy_ecs::system::Query<#data_type, #filter_type>
                });

                // Generate the setup statement that constructs QueryAllWrapper
                let mutability = if is_mutable {
                    quote! { mut }
                } else {
                    quote! {}
                };
                query_all_setup_stmts.push(quote! {
                    let #mutability #param_name = bevy_mass::query_all::QueryAllWrapper::new(
                        &#idx_name.entities,
                        &mut #query_name,
                    );
                });
            } else {
                bevy_mode_params.push(quote! { #arg });
            }
        }

        quote! {
            #[cfg(not(feature = "unreal"))]
            #(#func_attrs)*
            #[allow(unused_mut)]
            #func_vis fn #func_name(#(#bevy_mode_params),*) #func_ret {
                #(#query_all_setup_stmts)*
                #func_body
            }
        }
    };

    Ok(quote! {
        #bevy_mode_output

        // Unreal mode: rewritten function with chunk-based data access.
        #[cfg(feature = "unreal")]
        #[allow(unused_mut, unused_unsafe)]
        #func_vis fn #func_name(#(#rewritten_params),*) #func_ret
            #func_body

        // C++ extern wrapper + registration (each item is individually cfg-gated)
        #cpp_wrapper

        /// Zero-sized marker type for per-system chunk isolation (primary queries).
        #[cfg(feature = "unreal")]
        #[allow(non_camel_case_types)]
        struct #marker_name;

        /// Zero-sized marker type for global queries — separate from primary marker
        /// to avoid Res/ResMut conflicts when the same fragment type appears in both
        /// a primary and a global query within one system.
        #[cfg(feature = "unreal")]
        #[allow(non_camel_case_types)]
        struct #global_marker_name;

        #[cfg(feature = "unreal")]
        #chunk_consistency_assert

        #(
            #[cfg(feature = "unreal")]
            #facade_struct_defs
        )*

        #[cfg(feature = "unreal")]
        #[allow(unused_mut)]
        fn #bevy_wrapper_name(
            #(#bevy_params,)*
            #(#bevy_resource_params,)*
            #(#extra_bevy_params,)*
            #(#passthrough_params,)*
            #(#bevy_dt_param)*
        ) {
            #wrapper_body
        }

        #[cfg(feature = "unreal")]
        #[allow(non_upper_case_globals)]
        static #bevy_reg_name: () = {
            ::unreal_api::inventory::submit! {
                ::unreal_api::mass::MassBevySystemRegistration {
                    name: #system_name_str,
                    order: #order,
                    add_to_app: |app: &mut ::unreal_api::ecs::App,
                                 stage: ::unreal_api::mass::MassSystemStage| {
                        use ::unreal_api::ecs::schedule::IntoScheduleConfigs;
                        app.add_systems(::unreal_api::ecs::Update, #bevy_wrapper_name.in_set(stage));
                    },
                    register_messages: |app: &mut ::unreal_api::ecs::App| {
                        #(#register_message_stmts)*
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
        assert!(params[0].single_is_mutable());
        assert_eq!(params[0].scope, QueryScope::Primary);
        assert_eq!(params[0].single_scope_index(), 0);
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
        assert!(params[0].single_is_mutable());
        assert!(!params[1].single_is_mutable());
        assert_eq!(params[0].single_scope_index(), 0);
        assert_eq!(params[1].single_scope_index(), 1);
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
        assert_eq!(params[0].single_scope_index(), 0);
        assert!(params[0].single_is_mutable());

        assert_eq!(params[1].scope, QueryScope::Primary);
        assert_eq!(params[1].single_scope_index(), 1);
        assert!(!params[1].single_is_mutable());

        // Global query
        assert_eq!(params[2].scope, QueryScope::Global);
        assert_eq!(params[2].single_scope_index(), 0);
        assert!(params[2].single_is_mutable());
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

        let output = mass_system_impl(&func, 0, None).unwrap().to_string();
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

        let output = mass_system_impl(&func, 0, None).unwrap().to_string();
        assert!(output.contains("ant_movement_bevy"), "should generate Bevy wrapper fn");
        assert!(output.contains("ResMut"), "should use ResMut for mutable query");
        assert!(output.contains("MassSystemChunks"), "should reference MassSystemChunks resource");
        assert!(output.contains("bevy_time :: Time"), "should extract dt from Time resource");
        assert!(output.contains("MassSystemStage"), "add_to_schedule should accept MassSystemStage");
        assert!(output.contains("in_set"), "should add system to stage set");
    }

    #[test]
    fn bevy_wrapper_uses_res_for_readonly() {
        let func: ItemFn = syn::parse2(quote! {
            fn read_system(enc: MassQuery<&EncounterFragment>, dt: f32) {}
        })
        .unwrap();

        let output = mass_system_impl(&func, 0, None).unwrap().to_string();
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

        let output = mass_system_impl(&func, 0, None).unwrap().to_string();
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

        let output = mass_system_impl(&func, 0, None).unwrap().to_string();
        // Should use DualQueryMut for mutable primary (wraps chunk or Bevy data)
        assert!(output.contains("DualQueryMut"), "should use DualQueryMut for &mut primary");
        // Should use DualQueryRef for immutable primary
        assert!(output.contains("DualQueryRef"), "should use DualQueryRef for & primary");
        // Should use MassQueryAllMut for mutable global
        assert!(output.contains("MassQueryAllMut"), "should use MassQueryAllMut for &mut global");
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

        let output = mass_system_impl(&func, 0, None).unwrap().to_string();
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

        let output = mass_system_impl(&func, 0, None).unwrap().to_string();
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

        let output = mass_system_impl(&func, 0, None).unwrap().to_string();
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

        let output = mass_system_impl(&func, 0, None).unwrap().to_string();
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

        let output = mass_system_impl(&func, 0, None).unwrap().to_string();
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

    // -----------------------------------------------------------------------
    // With<Tag> filter tests
    // -----------------------------------------------------------------------

    #[test]
    fn parses_with_filter_on_query() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(pos: MassQuery<&mut Position, With<AntTag>>, dt: f32) {}
        })
        .unwrap();

        let params = extract_query_params(&func).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].filter_tags.len(), 1);
        let tag = &params[0].filter_tags[0];
        assert_eq!(quote!(#tag).to_string(), "AntTag");
    }

    #[test]
    fn parses_with_filter_on_query_alias() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(pos: Query<&mut Position, With<AntTag>>, dt: f32) {}
        })
        .unwrap();

        let params = extract_query_params(&func).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].filter_tags.len(), 1);
    }

    #[test]
    fn no_filter_tags_when_absent() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(pos: MassQuery<&mut Position>, dt: f32) {}
        })
        .unwrap();

        let params = extract_query_params(&func).unwrap();
        assert_eq!(params.len(), 1);
        assert!(params[0].filter_tags.is_empty());
    }

    #[test]
    fn with_filter_emits_tag_requirement() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(pos: MassQuery<&mut Position, With<AntTag>>) {}
        })
        .unwrap();

        let output = mass_system_impl(&func, 0, None).unwrap().to_string();
        assert!(output.contains("is_tag : 1"),
            "With<Tag> should emit a requirement with is_tag=1, got: {}", output);
    }

    #[test]
    fn with_filter_deduplicates_across_params() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(
                pos: MassQuery<&mut Position, With<AntTag>>,
                mov: MassQuery<&Movement, With<AntTag>>,
            ) {}
        })
        .unwrap();

        let output = mass_system_impl(&func, 0, None).unwrap().to_string();
        // Should have exactly 3 requirements: 2 fragments + 1 deduplicated tag
        assert!(output.contains("MassSystemRequirement ; 3usize"),
            "should have 3 requirements (2 fragments + 1 tag), got: {}", output);
    }

    #[test]
    fn parse_order_attribute() {
        let attr = quote! { order = 42 };
        let parsed = parse_mass_system_attr_full(attr).unwrap();
        assert_eq!(parsed.order, 42);
        assert_eq!(parsed.entity_group, None);
    }

    #[test]
    fn parse_empty_attribute() {
        let attr = quote! {};
        assert!(parse_mass_system_attr_full(attr).is_none());
    }

    #[test]
    fn order_emitted_in_registration() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(pos: MassQuery<&mut Position>, dt: f32) {}
        })
        .unwrap();

        let output = mass_system_impl(&func, 42, None).unwrap().to_string();
        assert!(output.contains("order : 42u32"),
            "should emit order value, got: {}", output);
    }

    #[test]
    fn with_filter_on_global_query() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(foods: MassQueryAll<&mut FoodFragment, With<FoodTag>>) {}
        })
        .unwrap();

        let params = extract_query_params(&func).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].filter_tags.len(), 1);
        assert_eq!(params[0].scope, QueryScope::Global);
    }

    // -----------------------------------------------------------------------
    // BevyQuery passthrough tests
    // -----------------------------------------------------------------------

    #[test]
    fn bevy_query_not_extracted_as_query_param() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(
                ants: MassQuery<&mut AntFragment>,
                cooldowns: BevyQuery<(Entity, &mut Cooldown)>,
                dt: f32,
            ) {}
        })
        .unwrap();

        let params = extract_query_params(&func).unwrap();
        // BevyQuery should NOT be extracted — only MassQuery
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].param_name, "ants");
    }

    #[test]
    fn bevy_query_detected_as_passthrough() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(
                ants: MassQuery<&mut AntFragment>,
                cooldowns: BevyQuery<(Entity, &mut Cooldown)>,
                time: Res<Time>,
                dt: f32,
            ) {}
        })
        .unwrap();

        let query_params = extract_query_params(&func).unwrap();
        let resource_params = extract_resource_params(&func).unwrap();
        let passthrough = extract_passthrough_params(&func, &query_params, &resource_params);

        assert_eq!(passthrough.len(), 1);
        let pt_str = passthrough[0].to_string();
        assert!(pt_str.contains("cooldowns"), "passthrough should contain BevyQuery param, got: {}", pt_str);
        assert!(pt_str.contains("BevyQuery"), "passthrough should preserve BevyQuery type, got: {}", pt_str);
    }

    #[test]
    fn commands_detected_as_passthrough() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(
                ants: MassQuery<&mut AntFragment>,
                mut commands: Commands,
                dt: f32,
            ) {}
        })
        .unwrap();

        let query_params = extract_query_params(&func).unwrap();
        let resource_params = extract_resource_params(&func).unwrap();
        let passthrough = extract_passthrough_params(&func, &query_params, &resource_params);

        assert_eq!(passthrough.len(), 1);
        let pt_str = passthrough[0].to_string();
        assert!(pt_str.contains("commands"), "passthrough should contain Commands param, got: {}", pt_str);
    }

    #[test]
    fn bevy_query_forwarded_in_wrapper() {
        let func: ItemFn = syn::parse2(quote! {
            fn entity_cooldown(
                cooldowns: BevyQuery<(Entity, &mut Cooldown)>,
                time: Res<Time>,
                mut commands: Commands,
            ) {}
        })
        .unwrap();

        let output = mass_system_impl(&func, 40, None).unwrap().to_string();

        // Wrapper should include BevyQuery and Commands as params
        assert!(output.contains("BevyQuery"),
            "wrapper should include BevyQuery param, got: {}", output);
        assert!(output.contains("Commands"),
            "wrapper should include Commands param, got: {}", output);
        // Should still have Bevy registration
        assert!(output.contains("MassBevySystemRegistration"),
            "should register for Bevy scheduling");
    }

    #[test]
    fn bevy_query_only_system_no_chunk_loop() {
        let func: ItemFn = syn::parse2(quote! {
            fn entity_cooldown(
                cooldowns: BevyQuery<(Entity, &mut Cooldown)>,
                time: Res<Time>,
            ) {}
        })
        .unwrap();

        let output = mass_system_impl(&func, 0, None).unwrap().to_string();

        // Should NOT contain chunk iteration (no MassQuery params)
        assert!(!output.contains("__chunk_idx"),
            "BevyQuery-only system should not have chunk loop, got: {}", output);
        // Should NOT contain MassSystemChunks (no fragment resources)
        assert!(!output.contains("MassSystemChunks"),
            "BevyQuery-only system should not reference MassSystemChunks, got: {}", output);
        // Should NOT inject Time resource (system doesn't declare dt: f32)
        assert!(!output.contains("bevy_time :: Time"),
            "system without dt: f32 should not inject Time resource, got: {}", output);
        // Should call the inner function directly
        assert!(output.contains("entity_cooldown"),
            "should call inner function");
    }

    #[test]
    fn passthrough_only_skips_cpp_registration() {
        let func: ItemFn = syn::parse2(quote! {
            fn entity_cooldown(
                cooldowns: BevyQuery<(Entity, &mut Cooldown)>,
            ) {}
        })
        .unwrap();

        let output = mass_system_impl(&func, 0, None).unwrap().to_string();
        // Pure-Bevy systems (no MassQuery params → zero fragment requirements)
        // must NOT register a C++ Mass Entity processor — ME asserts on empty queries.
        assert!(!output.contains("unsafe extern \"C\""),
            "should NOT generate C++ extern wrapper for pure-Bevy system");
        assert!(!output.contains("MassSystemRegistration"),
            "should NOT register C++ processor for pure-Bevy system");
        // Should still generate a Bevy system wrapper
        assert!(output.contains("entity_cooldown"),
            "should still reference the inner function");
    }

    // -----------------------------------------------------------------------
    // Tuple Query parsing tests
    // -----------------------------------------------------------------------

    #[test]
    fn parses_tuple_query_with_entity() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(
                ants: MassQuery<(Entity, &mut Position, &Movement)>,
                dt: f32,
            ) {}
        })
        .unwrap();

        let params = extract_query_params(&func).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].param_name, "ants");

        match &params[0].data {
            QueryData::Tuple { has_entity, fragments } => {
                assert!(has_entity, "should detect Entity in tuple");
                assert_eq!(fragments.len(), 2, "should have 2 fragments");
                assert!(fragments[0].is_mutable, "Position should be mutable");
                assert!(!fragments[1].is_mutable, "Movement should be immutable");
            }
            QueryData::Single(_) => panic!("expected Tuple, got Single"),
        }
    }

    #[test]
    fn parses_tuple_query_without_entity() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(
                ants: MassQuery<(&mut Position, &Movement)>,
                dt: f32,
            ) {}
        })
        .unwrap();

        let params = extract_query_params(&func).unwrap();
        match &params[0].data {
            QueryData::Tuple { has_entity, fragments } => {
                assert!(!has_entity, "should not detect Entity");
                assert_eq!(fragments.len(), 2);
            }
            QueryData::Single(_) => panic!("expected Tuple, got Single"),
        }
    }

    #[test]
    fn parses_without_filter() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(
                ants: MassQuery<(Entity, &mut Position), (With<AntTag>, Without<Cooldown>)>,
                dt: f32,
            ) {}
        })
        .unwrap();

        let params = extract_query_params(&func).unwrap();
        assert_eq!(params[0].filter_tags.len(), 1, "should have 1 With tag");
        assert_eq!(params[0].without_filters.len(), 1, "should have 1 Without filter");
    }

    #[test]
    fn single_query_still_works() {
        // Backward compatibility: single-component query
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(pos: MassQuery<&mut Position>, dt: f32) {}
        })
        .unwrap();

        let params = extract_query_params(&func).unwrap();
        assert_eq!(params.len(), 1);
        match &params[0].data {
            QueryData::Single(frag) => {
                assert!(frag.is_mutable);
            }
            QueryData::Tuple { .. } => panic!("expected Single, got Tuple"),
        }
    }

    // -----------------------------------------------------------------------
    // entity_group attribute parsing tests
    // -----------------------------------------------------------------------

    #[test]
    fn parse_entity_group_attribute() {
        let attr = quote! { order = 30, entity_group = "ants" };
        let parsed = parse_mass_system_attr_full(attr).unwrap();
        assert_eq!(parsed.order, 30);
        assert_eq!(parsed.entity_group.as_deref(), Some("ants"));
    }

    #[test]
    fn parse_order_only_no_entity_group() {
        let attr = quote! { order = 10 };
        let parsed = parse_mass_system_attr_full(attr).unwrap();
        assert_eq!(parsed.order, 10);
        assert_eq!(parsed.entity_group, None);
    }

    // -----------------------------------------------------------------------
    // Facade struct generation tests
    // -----------------------------------------------------------------------

    #[test]
    fn tuple_query_generates_facade_struct() {
        let func: ItemFn = syn::parse2(quote! {
            fn ant_move(
                ants: MassQuery<(Entity, &mut Position, &Movement), Without<Cooldown>>,
                dt: f32,
            ) {}
        })
        .unwrap();

        let output = mass_system_impl(&func, 0, Some("ants")).unwrap().to_string();
        // Should generate the facade struct
        assert!(output.contains("__FQ_ant_move_ants"),
            "should generate facade struct, got: {}", output);
        // Should generate iterator
        assert!(output.contains("__FQ_ant_move_ants_Iter"),
            "should generate iterator struct, got: {}", output);
        // Should have IntoIterator impl
        assert!(output.contains("IntoIterator"),
            "should implement IntoIterator, got: {}", output);
        // Should inject entity map
        assert!(output.contains("MassEntityMap"),
            "should inject MassEntityMap, got: {}", output);
    }

    #[test]
    fn tuple_query_generates_filter_mask() {
        let func: ItemFn = syn::parse2(quote! {
            fn ant_decide(
                ants: MassQuery<(Entity, &mut Position), (With<AntTag>, Without<Cooldown>)>,
                dt: f32,
            ) {}
        })
        .unwrap();

        let output = mass_system_impl(&func, 0, Some("ants")).unwrap().to_string();
        // Should have filter mask code
        assert!(output.contains("__filter_mask"),
            "should generate filter mask, got: {}", output);
        // Should inject Without query param
        assert!(output.contains("__without_0"),
            "should inject Without query param, got: {}", output);
    }

    #[test]
    fn tuple_query_without_entity_no_entity_map() {
        let func: ItemFn = syn::parse2(quote! {
            fn simple(
                items: MassQuery<(&mut Position, &Movement)>,
                dt: f32,
            ) {}
        })
        .unwrap();

        let output = mass_system_impl(&func, 0, None).unwrap().to_string();
        // No Entity in tuple, no Without → should NOT inject entity map
        assert!(!output.contains("MassEntityMap"),
            "should not inject MassEntityMap when no Entity/Without, got: {}", output);
    }

    // -----------------------------------------------------------------------
    // Entity group type safety tests
    // -----------------------------------------------------------------------

    #[test]
    fn entity_group_derived_from_with_tag() {
        // When With<Tag> is present and entity_group attribute is None,
        // the generated code should reference <Tag>::ENTITY_GROUP
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(
                ants: MassQuery<(Entity, &mut Position), With<AntTag>>,
            ) {}
        })
        .unwrap();

        let output = mass_system_impl(&func, 0, None).unwrap().to_string();
        assert!(output.contains("ENTITY_GROUP"),
            "should derive entity group from With<Tag>::ENTITY_GROUP, got: {}", output);
        assert!(output.contains("AntTag"),
            "should reference the tag type, got: {}", output);
    }

    #[test]
    fn entity_group_falls_back_to_attribute() {
        // When no With<Tag> filter but entity_group is specified, use the string
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(
                ants: MassQuery<(Entity, &mut Position), Without<Cooldown>>,
            ) {}
        })
        .unwrap();

        let output = mass_system_impl(&func, 0, Some("ants")).unwrap().to_string();
        assert!(output.contains("\"ants\""),
            "should use entity_group attribute as fallback, got: {}", output);
    }

    #[test]
    fn entity_group_uses_panic_placeholder_for_dual_mode() {
        // Dual-mode queries without entity_group get a panic placeholder
        // (chunk path is dead code for Bevy-only types)
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(
                ants: MassQuery<(Entity, &mut Position), Without<Cooldown>>,
            ) {}
        })
        .unwrap();

        let result = mass_system_impl(&func, 0, None);
        assert!(result.is_ok(), "dual-mode queries should not error without entity_group");
        let output = result.unwrap().to_string();
        assert!(output.contains("unreachable"), "should contain panic placeholder");
    }

    // -----------------------------------------------------------------------
    // QueryAll facade tests
    // -----------------------------------------------------------------------

    #[test]
    fn query_all_recognized_as_global_scope() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(foods: QueryAll<&mut FoodFragment, With<FoodTag>>) {}
        })
        .unwrap();

        let params = extract_query_params(&func).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].scope, QueryScope::Global);
        assert_eq!(params[0].param_name, "foods");
        assert_eq!(params[0].filter_tags.len(), 1);
    }

    #[test]
    fn query_all_bevy_mode_generates_entity_index() {
        let func: ItemFn = syn::parse2(quote! {
            fn apply_mutations(foods: QueryAll<&mut FoodFragment, With<FoodTag>>) {}
        })
        .unwrap();

        let output = mass_system_impl(&func, 35, None).unwrap().to_string();

        // Bevy mode should expand QueryAll to EntityIndex + Query + QueryAllWrapper
        assert!(output.contains("EntityIndex"),
            "Bevy mode should reference EntityIndex, got: {}", output);
        assert!(output.contains("QueryAllWrapper"),
            "Bevy mode should construct QueryAllWrapper, got: {}", output);
    }

    // -----------------------------------------------------------------------
    // #[bevy] attribute tests
    // -----------------------------------------------------------------------

    #[test]
    fn bevy_attr_skips_query_extraction() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(
                ants: Query<&mut AntFragment>,
                #[bevy] cooldowns: Query<(Entity, &mut Cooldown)>,
                dt: f32,
            ) {}
        })
        .unwrap();

        let query_params = extract_query_params(&func).unwrap();
        // Only the non-#[bevy] Query should be extracted
        assert_eq!(query_params.len(), 1);
        assert_eq!(query_params[0].param_name, "ants");
    }

    #[test]
    fn bevy_attr_becomes_passthrough() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(
                ants: Query<&mut AntFragment>,
                #[bevy] cooldowns: Query<(Entity, &mut Cooldown)>,
                dt: f32,
            ) {}
        })
        .unwrap();

        let query_params = extract_query_params(&func).unwrap();
        let resource_params = extract_resource_params(&func).unwrap();
        let passthrough = extract_passthrough_params(&func, &query_params, &resource_params);

        assert_eq!(passthrough.len(), 1);
        let pt_str = passthrough[0].to_string();
        assert!(pt_str.contains("cooldowns"), "should contain param name, got: {}", pt_str);
        assert!(pt_str.contains("Query"), "should preserve Query type, got: {}", pt_str);
        // Note: #[bevy] stripping happens in mass_system_impl, not in extract_passthrough_params
    }

    #[test]
    fn bevy_attr_stripped_from_rewritten_params() {
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(
                ants: Query<&mut AntFragment>,
                #[bevy] cooldowns: Query<(Entity, &mut Cooldown)>,
            ) {}
        })
        .unwrap();

        let output = mass_system_impl(&func, 10, None).unwrap().to_string();
        // The output should NOT contain #[bevy] as an attribute
        // (it would cause "unknown attribute" errors)
        assert!(!output.contains("# [bevy]") && !output.contains("#[bevy]"),
            "output should not contain #[bevy] attribute, got: {}", output);
        // But should contain the cooldowns param
        assert!(output.contains("cooldowns"),
            "output should contain cooldowns param, got: {}", output);
    }

    // -----------------------------------------------------------------------
    // Dual-mode dispatch (Step B) tests
    // -----------------------------------------------------------------------

    #[test]
    fn single_query_generates_dual_bevy_param() {
        // Single primary queries should generate both MassSystemChunks and bevy_ecs::Query params
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(ants: MassQuery<&mut AntFragment>) {}
        })
        .unwrap();

        let output = mass_system_impl(&func, 0, None).unwrap().to_string();
        // Should have __mass_ants (MassSystemChunks) param
        assert!(output.contains("__mass_ants"),
            "should have __mass_ants chunk resource param, got: {}", output);
        // Should have __bevy_ants (bevy_ecs::system::Query) param
        assert!(output.contains("__bevy_ants"),
            "should have __bevy_ants Bevy query param, got: {}", output);
    }

    #[test]
    fn single_query_generates_const_if_dispatch() {
        // Single primary queries should use QueryBackend::IS_CHUNK const-if dispatch
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(ants: MassQuery<&mut AntFragment>) {}
        })
        .unwrap();

        let output = mass_system_impl(&func, 0, None).unwrap().to_string();
        // Should have IS_CHUNK const-if
        assert!(output.contains("IS_CHUNK"),
            "should use QueryBackend::IS_CHUNK const-if dispatch, got: {}", output);
        // Should have pre-loop Bevy collection (__pre_ants)
        assert!(output.contains("__pre_ants"),
            "should have pre-loop Bevy collection, got: {}", output);
        // Should have chunk_count == 0 fallback
        assert!(output.contains("__chunk_count"),
            "should have __chunk_count for fallback branch, got: {}", output);
    }

    #[test]
    fn single_query_keeps_cpp_direct_dispatch() {
        // Simple single-query systems keep C++ direct dispatch (DualQuery wrapping works)
        let func: ItemFn = syn::parse2(quote! {
            fn my_system(ants: MassQuery<&mut AntFragment>) {}
        })
        .unwrap();

        let output = mass_system_impl(&func, 0, None).unwrap().to_string();
        // The C++ wrapper should call the inner function (not a no-op)
        assert!(output.contains("my_system (ants"),
            "C++ wrapper should call inner function, got: {}", output);
        // Should use DualQueryMut::from_chunk in the C++ wrapper
        assert!(output.contains("from_chunk"),
            "C++ wrapper should wrap chunk data in DualQuery, got: {}", output);
    }
}
