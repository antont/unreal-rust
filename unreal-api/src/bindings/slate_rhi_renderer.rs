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
pub static mut U_SLATE_FX_SUBSYSTEM_GET_SLATE_POST_PROCESSOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_FONT_BLUEPRINT_LIBRARY_MAKE_SLATE_FONT_INFO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_RHI_RENDERER_SETTINGS_GET_SLATE_POST_SETTING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SLATE_RHI_RENDERER_SETTINGS_GET_MUTABLE_SLATE_POST_SETTING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USlateFXSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSlatePostProcessor"),
            &raw mut U_SLATE_FX_SUBSYSTEM_GET_SLATE_POST_PROCESSOR,
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
            &raw mut U_SLATE_FONT_BLUEPRINT_LIBRARY_MAKE_SLATE_FONT_INFO,
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
            &raw mut U_SLATE_RHI_RENDERER_SETTINGS_GET_SLATE_POST_SETTING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMutableSlatePostSetting"),
            &raw mut U_SLATE_RHI_RENDERER_SETTINGS_GET_MUTABLE_SLATE_POST_SETTING,
        );
    }
}
#[repr(C, align(8))]
pub struct FSlatePostSettings {
    pub flags_0: u8,
    pub resolution: ESlatePostResolution,
    pub post_processor_class: TSubclassOf<USlateRHIPostBufferProcessor>,
    __padding_end: [u8; 32],
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
    __padding_48: [u8; 48],
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
}
#[repr(transparent)]
pub struct ESlatePostResolution(pub u8);
impl ESlatePostResolution {
    pub const FULL: ESlatePostResolution = ESlatePostResolution(0);
    pub const HALF: ESlatePostResolution = ESlatePostResolution(1);
}
