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
pub static mut U_APPLE_IMAGE_UTILS_BASE_ASYNC_TASK_BLUEPRINT_PROXY_CREATE_PROXY_OBJECT_FOR_CONVERT_TO_TIFF: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_APPLE_IMAGE_UTILS_BASE_ASYNC_TASK_BLUEPRINT_PROXY_CREATE_PROXY_OBJECT_FOR_CONVERT_TO_PNG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_APPLE_IMAGE_UTILS_BASE_ASYNC_TASK_BLUEPRINT_PROXY_CREATE_PROXY_OBJECT_FOR_CONVERT_TO_JPEG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_APPLE_IMAGE_UTILS_BASE_ASYNC_TASK_BLUEPRINT_PROXY_CREATE_PROXY_OBJECT_FOR_CONVERT_TO_HEIF: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAppleImageUtilsBaseAsyncTaskBlueprintProxy::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateProxyObjectForConvertToTIFF"),
            &raw mut U_APPLE_IMAGE_UTILS_BASE_ASYNC_TASK_BLUEPRINT_PROXY_CREATE_PROXY_OBJECT_FOR_CONVERT_TO_TIFF,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateProxyObjectForConvertToPNG"),
            &raw mut U_APPLE_IMAGE_UTILS_BASE_ASYNC_TASK_BLUEPRINT_PROXY_CREATE_PROXY_OBJECT_FOR_CONVERT_TO_PNG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateProxyObjectForConvertToJPEG"),
            &raw mut U_APPLE_IMAGE_UTILS_BASE_ASYNC_TASK_BLUEPRINT_PROXY_CREATE_PROXY_OBJECT_FOR_CONVERT_TO_JPEG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateProxyObjectForConvertToHEIF"),
            &raw mut U_APPLE_IMAGE_UTILS_BASE_ASYNC_TASK_BLUEPRINT_PROXY_CREATE_PROXY_OBJECT_FOR_CONVERT_TO_HEIF,
        );
    }
}
#[repr(C, align(8))]
pub struct FAppleImageUtilsImageConversionResult {
    pub error: FString,
    pub image_data: TArray<u8>,
}
impl FAppleImageUtilsImageConversionResult {}
pub struct IAppleImageInterface {}
#[repr(C, align(8))]
pub struct UAppleImageInterface {
    __padding_end: [u8; 48],
}
impl UAppleImageInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAppleImageInterface")
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
pub struct UAppleImageUtilsBaseAsyncTaskBlueprintProxy {
    #[doc(hidden)]
    __padding_120: [u8; 120],
    pub conversion_result: FAppleImageUtilsImageConversionResult,
    __padding_end: [u8; 8],
}
impl UAppleImageUtilsBaseAsyncTaskBlueprintProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAppleImageUtilsBaseAsyncTaskBlueprintProxy")
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
pub struct FAppleImageUtilsBaseAsyncTaskBlueprintProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAppleImageUtilsBaseAsyncTaskBlueprintProxy_OnFailure {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ETextureRotationDirection(pub u8);
impl ETextureRotationDirection {
    pub const NONE: ETextureRotationDirection = ETextureRotationDirection(0);
    pub const LEFT: ETextureRotationDirection = ETextureRotationDirection(1);
    pub const RIGHT: ETextureRotationDirection = ETextureRotationDirection(2);
    pub const DOWN: ETextureRotationDirection = ETextureRotationDirection(3);
    pub const LEFT_MIRRORED: ETextureRotationDirection = ETextureRotationDirection(4);
    pub const RIGHT_MIRRORED: ETextureRotationDirection = ETextureRotationDirection(5);
    pub const DOWN_MIRRORED: ETextureRotationDirection = ETextureRotationDirection(6);
    pub const UP_MIRRORED: ETextureRotationDirection = ETextureRotationDirection(7);
}
