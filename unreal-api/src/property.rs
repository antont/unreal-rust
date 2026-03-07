use crate::bindings::core_u_object::{UClass, UObject};
use crate::bindings::opague_definitions::FText;
use crate::bindings::rust_plugin::{URustExtension_RustClassDef, URustType};
use crate::core_data::{FName, FString, TArray, TMap, TSet, TSubclassOf, UPtr};

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

/// Unreal property flags (`CPF_*`) for use with `#[uproperty(...)]`.
///
/// These map to `EPropertyFlags` values from Unreal's `ObjectMacros.h`.
pub mod flags {
    pub type PropertyFlags = u64;

    // Atomic flags (CPF_*)
    pub const EDIT: PropertyFlags = 0x0000000000000001;
    pub const BLUEPRINT_VISIBLE: PropertyFlags = 0x0000000000000004;
    pub const BLUEPRINT_READ_ONLY: PropertyFlags = 0x0000000000000008;
    pub const NET: PropertyFlags = 0x0000000000000020;
    pub const DISABLE_EDIT_ON_TEMPLATE: PropertyFlags = 0x0000000000000800;
    pub const TRANSIENT: PropertyFlags = 0x0000000000002000;
    pub const CONFIG: PropertyFlags = 0x0000000000004000;
    pub const DISABLE_EDIT_ON_INSTANCE: PropertyFlags = 0x0000000000010000;
    pub const EDIT_CONST: PropertyFlags = 0x0000000000020000;
    pub const SAVE_GAME: PropertyFlags = 0x0001000000000000;
    pub const SIMPLE_DISPLAY: PropertyFlags = 0x0020000000000000;
    pub const ADVANCED_DISPLAY: PropertyFlags = 0x0040000000000000;
    pub const INTERP: PropertyFlags = 0x0200000000000000;

    // Composite specifiers (matching UPROPERTY() names)
    pub const EDIT_ANYWHERE: PropertyFlags = EDIT;
    pub const EDIT_DEFAULTS_ONLY: PropertyFlags = EDIT | DISABLE_EDIT_ON_INSTANCE;
    pub const EDIT_INSTANCE_ONLY: PropertyFlags = EDIT | DISABLE_EDIT_ON_TEMPLATE;
    pub const VISIBLE_ANYWHERE: PropertyFlags = EDIT | EDIT_CONST;
    pub const VISIBLE_DEFAULTS_ONLY: PropertyFlags = EDIT | EDIT_CONST | DISABLE_EDIT_ON_INSTANCE;
    pub const VISIBLE_INSTANCE_ONLY: PropertyFlags = EDIT | EDIT_CONST | DISABLE_EDIT_ON_TEMPLATE;
    pub const BLUEPRINT_READ_ONLY_ACCESS: PropertyFlags = BLUEPRINT_VISIBLE | BLUEPRINT_READ_ONLY;
    pub const BLUEPRINT_READ_WRITE: PropertyFlags = BLUEPRINT_VISIBLE;
    pub const REPLICATED: PropertyFlags = NET;
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
