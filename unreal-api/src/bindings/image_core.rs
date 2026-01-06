#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_IS_VALID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_WIDTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_PIXEL_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_PIXEL_LINEAR_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_HEIGHT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USharedImageConstRefBlueprintFns::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValid"),
            &raw mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_IS_VALID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWidth"),
            &raw mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_WIDTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSize"),
            &raw mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPixelValue"),
            &raw mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_PIXEL_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPixelLinearColor"),
            &raw mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_PIXEL_LINEAR_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHeight"),
            &raw mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_HEIGHT,
        );
    }
}
#[repr(C, align(8))]
pub struct FSharedImageConstRefBlueprint {
    __padding_end: [u8; 8],
}
impl FSharedImageConstRefBlueprint {}
#[repr(C, align(8))]
pub struct USharedImageConstRefBlueprintFns {
    __padding_end: [u8; 48],
}
impl USharedImageConstRefBlueprintFns {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USharedImageConstRefBlueprintFns")
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
