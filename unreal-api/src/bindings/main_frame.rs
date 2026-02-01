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
    pub u_home_screen_web_open_web_page: *mut crate::ffi::UFunctionOpague,
    pub u_home_screen_web_open_getting_started_project: *mut crate::ffi::UFunctionOpague,
    pub u_home_screen_web_navigate_to: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_home_screen_web_open_web_page: std::ptr::null_mut(),
            u_home_screen_web_open_getting_started_project: std::ptr::null_mut(),
            u_home_screen_web_navigate_to: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UHomeScreenWeb::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OpenWebPage"),
                &raw mut __FUNCTION_PTRS.u_home_screen_web_open_web_page,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OpenGettingStartedProject"),
                &raw mut __FUNCTION_PTRS.u_home_screen_web_open_getting_started_project,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NavigateTo"),
                &raw mut __FUNCTION_PTRS.u_home_screen_web_navigate_to,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct UHomeScreenContext {
    __padding_end: [u8; 64],
}
impl UHomeScreenContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHomeScreenContext")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHomeScreenContext")
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
}
#[repr(C, align(8))]
pub struct UHomeScreenSettings {
    __padding_end: [u8; 88],
}
impl UHomeScreenSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHomeScreenSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHomeScreenSettings")
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
}
#[repr(C, align(8))]
pub struct UHomeScreenWeb {
    __padding_end: [u8; 120],
}
impl UHomeScreenWeb {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHomeScreenWeb")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHomeScreenWeb")
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
}
#[repr(transparent)]
pub struct EMainSectionMenu(pub u8);
impl EMainSectionMenu {
    pub const NONE: EMainSectionMenu = EMainSectionMenu(0);
    pub const HOME: EMainSectionMenu = EMainSectionMenu(1);
    pub const NEWS: EMainSectionMenu = EMainSectionMenu(2);
    pub const GETTING_STARTED: EMainSectionMenu = EMainSectionMenu(3);
    pub const SAMPLE_PROJECTS: EMainSectionMenu = EMainSectionMenu(4);
}
#[repr(transparent)]
pub struct EAutoLoadProject(pub i32);
impl EAutoLoadProject {
    pub const HOME_SCREEN: EAutoLoadProject = EAutoLoadProject(0);
    pub const LAST_PROJECT: EAutoLoadProject = EAutoLoadProject(1);
    pub const MAX: EAutoLoadProject = EAutoLoadProject(2);
}
