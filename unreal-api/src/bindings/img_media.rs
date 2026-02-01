#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_img_media_source_set_tokenized_sequence_path: *mut crate::ffi::UFunctionOpague,
    pub u_img_media_source_set_sequence_path: *mut crate::ffi::UFunctionOpague,
    pub u_img_media_source_remove_target_object: *mut crate::ffi::UFunctionOpague,
    pub u_img_media_source_get_sequence_path: *mut crate::ffi::UFunctionOpague,
    pub u_img_media_source_get_proxies: *mut crate::ffi::UFunctionOpague,
    pub u_img_media_source_add_target_object: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_img_media_source_set_tokenized_sequence_path: std::ptr::null_mut(),
            u_img_media_source_set_sequence_path: std::ptr::null_mut(),
            u_img_media_source_remove_target_object: std::ptr::null_mut(),
            u_img_media_source_get_sequence_path: std::ptr::null_mut(),
            u_img_media_source_get_proxies: std::ptr::null_mut(),
            u_img_media_source_add_target_object: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UImgMediaSource::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTokenizedSequencePath"),
                &raw mut __FUNCTION_PTRS.u_img_media_source_set_tokenized_sequence_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSequencePath"),
                &raw mut __FUNCTION_PTRS.u_img_media_source_set_sequence_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveTargetObject"),
                &raw mut __FUNCTION_PTRS.u_img_media_source_remove_target_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSequencePath"),
                &raw mut __FUNCTION_PTRS.u_img_media_source_get_sequence_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetProxies"),
                &raw mut __FUNCTION_PTRS.u_img_media_source_get_proxies,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddTargetObject"),
                &raw mut __FUNCTION_PTRS.u_img_media_source_add_target_object,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FMediaSourceColorSettings {
    pub encoding_override: EMediaSourceEncoding,
    pub color_space_override: crate::bindings::engine::ETextureColorSpace,
    pub red_chromaticity_coordinate: crate::bindings::core_u_object::FVector2D,
    pub green_chromaticity_coordinate: crate::bindings::core_u_object::FVector2D,
    pub blue_chromaticity_coordinate: crate::bindings::core_u_object::FVector2D,
    pub white_chromaticity_coordinate: crate::bindings::core_u_object::FVector2D,
    pub chromatic_adaptation_method: crate::bindings::engine::ETextureChromaticAdaptationMethod,
}
impl FMediaSourceColorSettings {}
#[repr(C, align(8))]
pub struct UImgMediaSource {
    #[doc(hidden)]
    pub(crate) __padding_332: [u8; 332],
    pub frame_rate_override: crate::bindings::core_u_object::FFrameRate,
    pub proxy_override: FString,
    pub b_fill_gaps_in_sequence: bool,
    pub start_timecode: crate::bindings::core_u_object::FTimecode,
    pub source_color_settings: FMediaSourceColorSettings,
    pub sequence_path: crate::bindings::core_u_object::FDirectoryPath,
    __padding_end: [u8; 32],
}
impl UImgMediaSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImgMediaSource")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImgMediaSource")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_tokenized_sequence_path(&mut self, path: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::img_media::__FUNCTION_PTRS
                    .u_img_media_source_set_tokenized_sequence_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&path, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::img_media::__FUNCTION_PTRS
                    .u_img_media_source_set_tokenized_sequence_path,
                __buffer,
            )
        };
    }
    pub fn set_sequence_path(&mut self, path: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::img_media::__FUNCTION_PTRS
                    .u_img_media_source_set_sequence_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&path, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::img_media::__FUNCTION_PTRS
                    .u_img_media_source_set_sequence_path,
                __buffer,
            )
        };
    }
    pub fn remove_target_object(
        &mut self,
        in_actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::img_media::__FUNCTION_PTRS
                    .u_img_media_source_remove_target_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::img_media::__FUNCTION_PTRS
                    .u_img_media_source_remove_target_object,
                __buffer,
            )
        };
    }
    pub fn get_sequence_path(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::img_media::__FUNCTION_PTRS
                    .u_img_media_source_get_sequence_path,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::img_media::__FUNCTION_PTRS
                    .u_img_media_source_get_sequence_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_proxies(&self, out_proxies: &mut TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::img_media::__FUNCTION_PTRS
                    .u_img_media_source_get_proxies,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_proxies,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::img_media::__FUNCTION_PTRS
                    .u_img_media_source_get_proxies,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FString>>().swap(out_proxies);
        }
    }
    pub fn add_target_object(
        &mut self,
        in_actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::img_media::__FUNCTION_PTRS
                    .u_img_media_source_add_target_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::img_media::__FUNCTION_PTRS
                    .u_img_media_source_add_target_object,
                __buffer,
            )
        };
    }
}
#[repr(transparent)]
pub struct EMediaSourceEncoding(pub u8);
impl EMediaSourceEncoding {
    pub const MSE_NONE: EMediaSourceEncoding = EMediaSourceEncoding(0);
    pub const MSE_LINEAR: EMediaSourceEncoding = EMediaSourceEncoding(1);
    pub const MSE_S_RGB: EMediaSourceEncoding = EMediaSourceEncoding(2);
    pub const MSE_ST2084: EMediaSourceEncoding = EMediaSourceEncoding(3);
    pub const MSE_S_LOG3: EMediaSourceEncoding = EMediaSourceEncoding(12);
}
