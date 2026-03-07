use crate::bindings::core_u_object::{UClass, UObject};
use crate::bindings::opague_definitions::FText;
use crate::bindings::rust_plugin::{URustExtension_RustClassDef, URustType};
use crate::core_data::{FName, FString, TArray, TMap, TSet, TSubclassOf, UPtr};

pub use crate::bindings::rust_plugin::ERustPropertySpecifier;

/// Maps a Rust type to its corresponding Unreal property type.
pub trait UnrealProperty {
    fn create_property_type() -> UPtr<URustType>;
}

/// Provides `static_class()` for UObject types, allowing automatic
/// type inference for `UPtr<T>` property fields.
pub trait HasStaticClass {
    fn static_class() -> UPtr<UClass>;
}

impl<T: HasStaticClass> UnrealProperty for UPtr<T> {
    fn create_property_type() -> UPtr<URustType> {
        URustExtension_RustClassDef::create_type_u_object(TSubclassOf::<UObject>::from(
            T::static_class(),
        ))
    }
}

macro_rules! impl_unreal_property {
    ($rust_ty:ty, $create_fn:ident) => {
        impl UnrealProperty for $rust_ty {
            fn create_property_type() -> UPtr<URustType> {
                URustExtension_RustClassDef::$create_fn()
            }
        }
    };
}

impl_unreal_property!(f32, create_type_float);
impl_unreal_property!(f64, create_type_double);
impl_unreal_property!(bool, create_type_bool);
impl_unreal_property!(i8, create_type_int8);
impl_unreal_property!(i16, create_type_int16);
impl_unreal_property!(i32, create_type_int32);
impl_unreal_property!(i64, create_type_int64);
impl_unreal_property!(u8, create_type_u_int8);
impl_unreal_property!(u16, create_type_u_int16);
impl_unreal_property!(u32, create_type_u_int32);
impl_unreal_property!(u64, create_type_u_int64);
impl_unreal_property!(FString, create_type_f_string);
impl_unreal_property!(FName, create_type_f_name);
impl_unreal_property!(FText, create_type_f_text);

impl<T: UnrealProperty> UnrealProperty for TArray<T> {
    fn create_property_type() -> UPtr<URustType> {
        URustExtension_RustClassDef::create_type_array(T::create_property_type())
    }
}

impl<T: UnrealProperty> UnrealProperty for TSet<T> {
    fn create_property_type() -> UPtr<URustType> {
        URustExtension_RustClassDef::create_type_set(T::create_property_type())
    }
}

impl<K: UnrealProperty, V: UnrealProperty> UnrealProperty for TMap<K, V> {
    fn create_property_type() -> UPtr<URustType> {
        URustExtension_RustClassDef::create_type_map(
            K::create_property_type(),
            V::create_property_type(),
        )
    }
}

/// Helper macro to implement `HasStaticClass` for types that already have
/// an inherent `static_class()` method (e.g. generated engine classes).
#[macro_export]
macro_rules! impl_has_static_class {
    ($($ty:ty),* $(,)?) => {
        $(
            impl $crate::property::HasStaticClass for $ty {
                fn static_class() -> $crate::core_data::UPtr<$crate::bindings::core_u_object::UClass> {
                    <$ty>::static_class()
                }
            }
        )*
    };
}

// Implement HasStaticClass for commonly used engine types.
// TODO: Generate these automatically in unreal-api-generator.
impl_has_static_class!(
    crate::bindings::enhanced_input::UInputAction,
    crate::bindings::core_u_object::UObject,
);
