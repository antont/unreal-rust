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
    pub u_xcode_project_settings_should_disable_ios_settings: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_xcode_project_settings_should_disable_ios_settings: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UXcodeProjectSettings::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShouldDisableIOSSettings"),
            &raw mut __FUNCTION_PTRS.u_xcode_project_settings_should_disable_ios_settings,
        );
    }
}
#[repr(C, align(8))]
pub struct UMacTargetSettings {
    __padding_end: [u8; 176],
}
impl UMacTargetSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMacTargetSettings")
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
pub struct UXcodeProjectSettings {
    __padding_end: [u8; 392],
}
impl UXcodeProjectSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UXcodeProjectSettings")
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
pub struct EMacTargetArchitecture(pub u8);
impl EMacTargetArchitecture {
    pub const MAC_TARGET_ARCHITECTURE_INTEL: EMacTargetArchitecture = EMacTargetArchitecture(
        0,
    );
    pub const MAC_TARGET_ARCHITECTURE_UNIVERSAL: EMacTargetArchitecture = EMacTargetArchitecture(
        1,
    );
    pub const MAC_TARGET_ARCHITECTURE_APPLE_SILICON: EMacTargetArchitecture = EMacTargetArchitecture(
        2,
    );
    pub const MAC_TARGET_ARCHITECTURE_HOST: EMacTargetArchitecture = EMacTargetArchitecture(
        3,
    );
}
