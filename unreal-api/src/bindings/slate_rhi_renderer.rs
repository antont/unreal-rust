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
    pub u_slate_fx_subsystem_get_slate_post_processor: *mut crate::ffi::UFunctionOpague,
    pub u_slate_font_blueprint_library_make_slate_font_info: *mut crate::ffi::UFunctionOpague,
    pub u_slate_rhi_renderer_settings_get_slate_post_setting: *mut crate::ffi::UFunctionOpague,
    pub u_slate_rhi_renderer_settings_get_mutable_slate_post_setting: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_slate_fx_subsystem_get_slate_post_processor: std::ptr::null_mut(),
            u_slate_font_blueprint_library_make_slate_font_info: std::ptr::null_mut(),
            u_slate_rhi_renderer_settings_get_slate_post_setting: std::ptr::null_mut(),
            u_slate_rhi_renderer_settings_get_mutable_slate_post_setting: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USlateFXSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSlatePostProcessor"),
            &raw mut __FUNCTION_PTRS.u_slate_fx_subsystem_get_slate_post_processor,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USlateFontBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeSlateFontInfo"),
            &raw mut __FUNCTION_PTRS.u_slate_font_blueprint_library_make_slate_font_info,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USlateRHIRendererSettings::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSlatePostSetting"),
            &raw mut __FUNCTION_PTRS.u_slate_rhi_renderer_settings_get_slate_post_setting,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMutableSlatePostSetting"),
            &raw mut __FUNCTION_PTRS
                .u_slate_rhi_renderer_settings_get_mutable_slate_post_setting,
        );
    }
}
#[repr(C, align(8))]
pub struct FSlatePostSettings {
    pub flags_0: u8,
    pub resolution: ESlatePostResolution,
    pub post_processor_class: TSubclassOf<USlateRHIPostBufferProcessor>,
    pub(crate) __padding_end: [u8; 32],
}
impl FSlatePostSettings {}
#[repr(C, align(8))]
pub struct USlateFXSubsystem {
    __padding_end: [u8; 216],
}
impl USlateFXSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlateFXSubsystem")
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
    pub fn get_slate_post_processor(
        &mut self,
        in_post_buffer_bit: crate::bindings::slate_core::ESlatePostRT,
    ) -> UPtr<USlateRHIPostBufferProcessor> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::slate_rhi_renderer::__FUNCTION_PTRS
                    .u_slate_fx_subsystem_get_slate_post_processor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_post_buffer_bit,
                __buffer.add(0).cast::<crate::bindings::slate_core::ESlatePostRT>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::slate_rhi_renderer::__FUNCTION_PTRS
                    .u_slate_fx_subsystem_get_slate_post_processor,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<USlateRHIPostBufferProcessor>>().read() }
    }
}
#[repr(C, align(8))]
pub struct USlateRHIPostBufferProcessor {
    __padding_end: [u8; 48],
}
impl USlateRHIPostBufferProcessor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlateRHIPostBufferProcessor")
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
pub struct USlatePostBufferBlur {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub gaussian_blur_strength: f32,
    __padding_end: [u8; 20],
}
impl USlatePostBufferBlur {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlatePostBufferBlur")
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
pub struct USlateFontBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl USlateFontBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlateFontBlueprintLibrary")
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
    pub fn make_slate_font_info(
        font_object: UPtr<crate::bindings::core_u_object::UObject>,
        font_material: UPtr<crate::bindings::core_u_object::UObject>,
        outline_settings: crate::bindings::slate_core::FFontOutlineSettings,
        typeface_font_name: FName,
        size: f32,
        letter_spacing: i32,
        skew_amount: f32,
        b_force_monospaced: bool,
        b_material_is_stencil: bool,
        monospaced_width: f32,
    ) -> crate::bindings::slate_core::FSlateFontInfo {
        let mut __stack = crate::core_data::StackAlloc::<192>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::slate_rhi_renderer::__FUNCTION_PTRS
                    .u_slate_font_blueprint_library_make_slate_font_info,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &font_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &font_material,
                __buffer.add(8).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &outline_settings,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::slate_core::FFontOutlineSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &typeface_font_name,
                __buffer.add(48).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&size, __buffer.add(60).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &letter_spacing,
                __buffer.add(64).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skew_amount,
                __buffer.add(68).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_monospaced,
                __buffer.add(72).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_material_is_stencil,
                __buffer.add(73).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &monospaced_width,
                __buffer.add(76).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::slate_rhi_renderer::USlateFontBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::slate_rhi_renderer::__FUNCTION_PTRS
                    .u_slate_font_blueprint_library_make_slate_font_info,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(80).cast::<crate::bindings::slate_core::FSlateFontInfo>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct USlateRHIRendererSettings {
    __padding_end: [u8; 184],
}
impl USlateRHIRendererSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlateRHIRendererSettings")
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
    pub fn get_slate_post_setting(
        &self,
        in_post_buffer_bit: crate::bindings::slate_core::ESlatePostRT,
    ) -> FSlatePostSettings {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::slate_rhi_renderer::__FUNCTION_PTRS
                    .u_slate_rhi_renderer_settings_get_slate_post_setting,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_post_buffer_bit,
                __buffer.add(0).cast::<crate::bindings::slate_core::ESlatePostRT>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::slate_rhi_renderer::__FUNCTION_PTRS
                    .u_slate_rhi_renderer_settings_get_slate_post_setting,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FSlatePostSettings>().read() }
    }
    pub fn get_mutable_slate_post_setting(
        &mut self,
        in_post_buffer_bit: crate::bindings::slate_core::ESlatePostRT,
    ) -> FSlatePostSettings {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::slate_rhi_renderer::__FUNCTION_PTRS
                    .u_slate_rhi_renderer_settings_get_mutable_slate_post_setting,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_post_buffer_bit,
                __buffer.add(0).cast::<crate::bindings::slate_core::ESlatePostRT>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::slate_rhi_renderer::__FUNCTION_PTRS
                    .u_slate_rhi_renderer_settings_get_mutable_slate_post_setting,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FSlatePostSettings>().read() }
    }
}
#[repr(transparent)]
pub struct ESlatePostResolution(pub u8);
impl ESlatePostResolution {
    pub const FULL: ESlatePostResolution = ESlatePostResolution(0);
    pub const HALF: ESlatePostResolution = ESlatePostResolution(1);
}
