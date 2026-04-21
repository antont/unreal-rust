use syn::DeriveInput;

mod component;
mod component_attr;
mod event;
mod mass_fragment;
mod mass_system;
mod reflect;
mod type_uuid;
mod uclass;
use quote::quote;

#[proc_macro_derive(Component, attributes(uuid, reflect))]
pub fn component_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    // let reflect = reflect::reflect_derive(&ast);
    let type_uuid = type_uuid::type_uuid_derive(&ast);
    let component = component::component_derive(&ast);
    quote! {
        // #reflect
        #type_uuid
        #component
    }
    .into()
}

#[proc_macro_derive(Event, attributes(uuid, reflect))]
pub fn event_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    // let reflect = reflect::reflect_derive(&ast);
    let type_uuid = type_uuid::type_uuid_derive(&ast);
    let event = event::event_derive(&ast);
    quote! {
        // #reflect
        #type_uuid
        #event
    }
    .into()
}

/// Attribute macro that registers a Rust function as a MassEntity system.
///
/// Generates an `extern "C"` wrapper function and inventory registration
/// so C++ can discover and dispatch to this system dynamically.
///
/// ```ignore
/// #[mass_system]
/// fn ant_movement(ants: MassQuery<&mut AntFragment>, dt: f32) {
///     for ant in ants.iter_mut() { /* ... */ }
/// }
/// ```
///
/// **`MessageReader<T>` / `MessageReplay<T>` — `T: Clone` requirement**:
/// because the macro is called once per Mass Entity chunk, a plain Bevy
/// `MessageReader` would drain on the first chunk and observe nothing on
/// subsequent ones. The expansion therefore pre-collects messages into a
/// shared per-frame buffer with `reader.read().cloned().collect::<Vec<T>>()`
/// (see the `MessageReplay` buffer construction in `mass_system.rs`), and
/// hands each chunk a fresh replay view over that buffer. This forces any
/// message type used as `MessageReader<T>` in a `#[mass_system]` parameter
/// to implement `Clone`. Payloads are small by convention, `Clone` is cheap,
/// and this avoids per-chunk borrowed-slice lifetimes.
#[proc_macro_attribute]
pub fn mass_system(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let func: syn::ItemFn = syn::parse(item).unwrap();
    let parsed = mass_system::parse_mass_system_attr_full(attr.into())
        .unwrap_or(mass_system::MassSystemAttr { order: 0, entity_group: None });
    match mass_system::mass_system_impl(&func, parsed.order, parsed.entity_group.as_deref()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_derive(MassFragment, attributes(mass))]
pub fn mass_fragment_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    match mass_fragment::mass_fragment_derive(&ast) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_derive(UClass, attributes(uproperty))]
pub fn uclass_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    match uclass::uclass_derive(&ast) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Attribute macro that defines a Bevy Component with automatic Unreal Mass Entity integration.
///
/// In Bevy mode: adds `#[repr(C)]`, `#[derive(Component, Clone, Copy, Debug)]`.
/// In Unreal mode: additionally generates `MassFragment` registration and C++ codegen metadata.
///
/// Detects tags automatically: unit structs (`struct Foo;`) become MassTags,
/// structs with fields become MassFragments.
///
/// ```ignore
/// use bevy_mass::prelude::*;
///
/// #[component]
/// pub struct FoodFragment {
///     pub is_loose: bool,
/// }
///
/// #[component(group = "ants")]
/// pub struct AntTag;
/// ```
///
/// The C++ type name is auto-derived from `BEVY_MASS_CPP_PREFIX` env var + struct name
/// (e.g., prefix "Gatherers" + "FoodFragment" → "FGatherersFoodFragment").
/// Override with `#[component(cpp_type = "FCustomName")]`.
#[proc_macro_attribute]
pub fn component(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match component_attr::component_impl(attr.into(), item.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Attribute macro that prepares a struct for UClass derivation.
///
/// Adds `#[repr(C)]` and validates that the first field matches the
/// specified parent type.
///
/// ```ignore
/// #[derive(UClass)]
/// #[inherit(UDataAsset)]
/// struct TestDataAsset {
///     base: UDataAsset,
///     #[uproperty]
///     foo: f32,
/// }
/// ```
#[proc_macro_attribute]
pub fn inherit(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match inherit_impl(attr.into(), item.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn inherit_impl(
    attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let parent_ty: syn::Type = syn::parse2(attr)?;
    let mut input: syn::ItemStruct = syn::parse2(item)?;

    let fields = match &input.fields {
        syn::Fields::Named(fields) => &fields.named,
        _ => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "#[inherit] requires a struct with named fields",
            ));
        }
    };

    let first_field = fields.first().ok_or_else(|| {
        syn::Error::new_spanned(
            &input.ident,
            "#[inherit] struct must have at least one field (the parent type)",
        )
    })?;

    let first_field_ty = &first_field.ty;
    let parent_tokens = quote::quote!(#parent_ty);
    let field_tokens = quote::quote!(#first_field_ty);
    if parent_tokens.to_string() != field_tokens.to_string() {
        let parent_str = parent_tokens.to_string();
        return Err(syn::Error::new_spanned(
            first_field_ty,
            format!(
                "#[inherit({0})]: first field must be of type `{0}`, but found `{1}`",
                parent_str,
                field_tokens.to_string(),
            ),
        ));
    }

    // Add #[repr(C)] if not already present
    let has_repr_c = input.attrs.iter().any(|attr| {
        if !attr.path.is_ident("repr") {
            return false;
        }
        if let Ok(meta) = attr.parse_meta() {
            match meta {
                syn::Meta::List(list) => list.nested.iter().any(|nested| {
                    matches!(nested, syn::NestedMeta::Meta(syn::Meta::Path(p)) if p.is_ident("C"))
                }),
                _ => false,
            }
        } else {
            false
        }
    });

    if !has_repr_c {
        input.attrs.push(syn::parse_quote!(#[repr(C)]));
    }

    Ok(quote::quote!(#input))
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn inherit_impl_adds_repr_c() {
        let attr = quote! { Foo };
        let item = quote! {
            struct Bar {
                base: Foo,
                x: f32,
            }
        };
        let result = inherit_impl(attr, item).unwrap();
        let output = result.to_string();
        assert!(output.contains("repr"), "should add #[repr(C)], got: {}", output);
    }

    #[test]
    fn inherit_impl_wrong_first_field_type() {
        let attr = quote! { Foo };
        let item = quote! {
            struct Bar {
                base: Wrong,
                x: f32,
            }
        };
        let result = inherit_impl(attr, item);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Foo"), "error should mention expected type, got: {}", err);
    }

    #[test]
    fn inherit_impl_empty_struct_errors() {
        let attr = quote! { Foo };
        let item = quote! {
            struct Bar {}
        };
        let result = inherit_impl(attr, item);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("at least one field"));
    }

    #[test]
    fn inherit_impl_tuple_struct_errors() {
        let attr = quote! { Foo };
        let item = quote! {
            struct Bar(Foo, f32);
        };
        let result = inherit_impl(attr, item);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("named fields"));
    }

    #[test]
    fn inherit_impl_preserves_existing_repr_c() {
        let attr = quote! { Foo };
        let item = quote! {
            #[repr(C)]
            struct Bar {
                base: Foo,
            }
        };
        let result = inherit_impl(attr, item).unwrap();
        let output = result.to_string();
        // Should have repr(C) exactly once, not duplicated
        assert_eq!(output.matches("repr").count(), 1, "should not duplicate #[repr(C)], got: {}", output);
    }
}
