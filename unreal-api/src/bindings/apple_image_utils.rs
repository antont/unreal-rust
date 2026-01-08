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
pub struct FunctionPtrs {
    pub u_apple_image_utils_base_async_task_blueprint_proxy_create_proxy_object_for_convert_to_tiff: *mut crate::ffi::UFunctionOpague,
    pub u_apple_image_utils_base_async_task_blueprint_proxy_create_proxy_object_for_convert_to_png: *mut crate::ffi::UFunctionOpague,
    pub u_apple_image_utils_base_async_task_blueprint_proxy_create_proxy_object_for_convert_to_jpeg: *mut crate::ffi::UFunctionOpague,
    pub u_apple_image_utils_base_async_task_blueprint_proxy_create_proxy_object_for_convert_to_heif: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_apple_image_utils_base_async_task_blueprint_proxy_create_proxy_object_for_convert_to_tiff: std::ptr::null_mut(),
            u_apple_image_utils_base_async_task_blueprint_proxy_create_proxy_object_for_convert_to_png: std::ptr::null_mut(),
            u_apple_image_utils_base_async_task_blueprint_proxy_create_proxy_object_for_convert_to_jpeg: std::ptr::null_mut(),
            u_apple_image_utils_base_async_task_blueprint_proxy_create_proxy_object_for_convert_to_heif: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAppleImageUtilsBaseAsyncTaskBlueprintProxy::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateProxyObjectForConvertToTIFF"),
            &raw mut __FUNCTION_PTRS
                .u_apple_image_utils_base_async_task_blueprint_proxy_create_proxy_object_for_convert_to_tiff,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateProxyObjectForConvertToPNG"),
            &raw mut __FUNCTION_PTRS
                .u_apple_image_utils_base_async_task_blueprint_proxy_create_proxy_object_for_convert_to_png,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateProxyObjectForConvertToJPEG"),
            &raw mut __FUNCTION_PTRS
                .u_apple_image_utils_base_async_task_blueprint_proxy_create_proxy_object_for_convert_to_jpeg,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateProxyObjectForConvertToHEIF"),
            &raw mut __FUNCTION_PTRS
                .u_apple_image_utils_base_async_task_blueprint_proxy_create_proxy_object_for_convert_to_heif,
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
    pub fn create_proxy_object_for_convert_to_tiff(
        source_image: UPtr<crate::bindings::engine::UTexture>,
        b_want_color: bool,
        b_use_gpu: bool,
        scale: f32,
        rotate: ETextureRotationDirection,
    ) -> UPtr<UAppleImageUtilsBaseAsyncTaskBlueprintProxy> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::apple_image_utils::__FUNCTION_PTRS
                    .u_apple_image_utils_base_async_task_blueprint_proxy_create_proxy_object_for_convert_to_tiff,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_image,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UTexture>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_want_color,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_use_gpu, __buffer.add(9).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&scale, __buffer.add(12).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotate,
                __buffer.add(16).cast::<ETextureRotationDirection>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::apple_image_utils::UAppleImageUtilsBaseAsyncTaskBlueprintProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::apple_image_utils::__FUNCTION_PTRS
                    .u_apple_image_utils_base_async_task_blueprint_proxy_create_proxy_object_for_convert_to_tiff,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<UPtr<UAppleImageUtilsBaseAsyncTaskBlueprintProxy>>()
                .read()
        }
    }
    pub fn create_proxy_object_for_convert_to_png(
        source_image: UPtr<crate::bindings::engine::UTexture>,
        b_want_color: bool,
        b_use_gpu: bool,
        scale: f32,
        rotate: ETextureRotationDirection,
    ) -> UPtr<UAppleImageUtilsBaseAsyncTaskBlueprintProxy> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::apple_image_utils::__FUNCTION_PTRS
                    .u_apple_image_utils_base_async_task_blueprint_proxy_create_proxy_object_for_convert_to_png,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_image,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UTexture>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_want_color,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_use_gpu, __buffer.add(9).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&scale, __buffer.add(12).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotate,
                __buffer.add(16).cast::<ETextureRotationDirection>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::apple_image_utils::UAppleImageUtilsBaseAsyncTaskBlueprintProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::apple_image_utils::__FUNCTION_PTRS
                    .u_apple_image_utils_base_async_task_blueprint_proxy_create_proxy_object_for_convert_to_png,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<UPtr<UAppleImageUtilsBaseAsyncTaskBlueprintProxy>>()
                .read()
        }
    }
    pub fn create_proxy_object_for_convert_to_jpeg(
        source_image: UPtr<crate::bindings::engine::UTexture>,
        quality: i32,
        b_want_color: bool,
        b_use_gpu: bool,
        scale: f32,
        rotate: ETextureRotationDirection,
    ) -> UPtr<UAppleImageUtilsBaseAsyncTaskBlueprintProxy> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::apple_image_utils::__FUNCTION_PTRS
                    .u_apple_image_utils_base_async_task_blueprint_proxy_create_proxy_object_for_convert_to_jpeg,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_image,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UTexture>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&quality, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_want_color,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_gpu,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&scale, __buffer.add(16).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotate,
                __buffer.add(20).cast::<ETextureRotationDirection>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::apple_image_utils::UAppleImageUtilsBaseAsyncTaskBlueprintProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::apple_image_utils::__FUNCTION_PTRS
                    .u_apple_image_utils_base_async_task_blueprint_proxy_create_proxy_object_for_convert_to_jpeg,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<UPtr<UAppleImageUtilsBaseAsyncTaskBlueprintProxy>>()
                .read()
        }
    }
    pub fn create_proxy_object_for_convert_to_heif(
        source_image: UPtr<crate::bindings::engine::UTexture>,
        quality: i32,
        b_want_color: bool,
        b_use_gpu: bool,
        scale: f32,
        rotate: ETextureRotationDirection,
    ) -> UPtr<UAppleImageUtilsBaseAsyncTaskBlueprintProxy> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::apple_image_utils::__FUNCTION_PTRS
                    .u_apple_image_utils_base_async_task_blueprint_proxy_create_proxy_object_for_convert_to_heif,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_image,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UTexture>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&quality, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_want_color,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_gpu,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&scale, __buffer.add(16).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotate,
                __buffer.add(20).cast::<ETextureRotationDirection>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::apple_image_utils::UAppleImageUtilsBaseAsyncTaskBlueprintProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::apple_image_utils::__FUNCTION_PTRS
                    .u_apple_image_utils_base_async_task_blueprint_proxy_create_proxy_object_for_convert_to_heif,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<UPtr<UAppleImageUtilsBaseAsyncTaskBlueprintProxy>>()
                .read()
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
