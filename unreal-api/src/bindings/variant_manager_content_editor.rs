#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {}
    }
}
pub fn initialize() {}
#[repr(C, align(8))]
pub struct UAssetDefinition_LevelVariantSets {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_LevelVariantSets {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_LevelVariantSets")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelVariantSetsActorFactory {
    __padding_end: [u8; 144],
}
impl ULevelVariantSetsActorFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelVariantSetsActorFactory")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USwitchActorFactory {
    __padding_end: [u8; 144],
}
impl USwitchActorFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USwitchActorFactory")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UVariantManagerFactoryNew {
    __padding_end: [u8; 136],
}
impl UVariantManagerFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVariantManagerFactoryNew")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(16))]
pub struct AVariantManagerTestActor {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub enum_with_no_default: EVariantManagerTestEnum,
    pub enum_with_second_default: EVariantManagerTestEnum,
    pub captured_byte_property: u8,
    pub captured_int_property: i32,
    pub captured_float_property: f32,
    pub b_captured_bool_property: bool,
    pub captured_object_property: UPtr<crate::bindings::core_u_object::UObject>,
    pub captured_interface_property: FScriptInterface,
    pub captured_name_property: FName,
    pub captured_str_property: FString,
    pub captured_text_property: FText,
    pub captured_rotator_property: crate::bindings::core_u_object::FRotator,
    pub captured_color_property: crate::bindings::core_u_object::FColor,
    pub captured_linear_color_property: crate::bindings::core_u_object::FLinearColor,
    pub captured_vector_property: crate::bindings::core_u_object::FVector,
    pub captured_quat_property: crate::bindings::core_u_object::FQuat,
    pub captured_vector4_property: crate::bindings::core_u_object::FVector4,
    pub captured_vector2_d_property: crate::bindings::core_u_object::FVector2D,
    pub captured_int_point_property: crate::bindings::core_u_object::FIntPoint,
    pub captured_u_object_array_property: TArray<
        UPtr<crate::bindings::core_u_object::UObject>,
    >,
    pub captured_vector_array_property: TArray<crate::bindings::core_u_object::FVector>,
}
impl AVariantManagerTestActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AVariantManagerTestActor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(transparent)]
pub struct EVariantManagerTestEnum(pub u8);
impl EVariantManagerTestEnum {
    pub const NONE: EVariantManagerTestEnum = EVariantManagerTestEnum(0);
    pub const FIRST_OPTION: EVariantManagerTestEnum = EVariantManagerTestEnum(1);
    pub const SECOND_OPTION: EVariantManagerTestEnum = EVariantManagerTestEnum(3);
    pub const THIRD_OPTION: EVariantManagerTestEnum = EVariantManagerTestEnum(45);
}
