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
pub static mut U_TRACE_UTIL_LIBRARY_TRACE_SCREENSHOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TRACE_UTIL_LIBRARY_TRACE_MARK_REGION_START: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TRACE_UTIL_LIBRARY_TRACE_MARK_REGION_END: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TRACE_UTIL_LIBRARY_TRACE_BOOKMARK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TRACE_UTIL_LIBRARY_TOGGLE_CHANNEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TRACE_UTIL_LIBRARY_STOP_TRACING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TRACE_UTIL_LIBRARY_START_TRACE_TO_FILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TRACE_UTIL_LIBRARY_START_TRACE_SEND_TO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TRACE_UTIL_LIBRARY_RESUME_TRACING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TRACE_UTIL_LIBRARY_PAUSE_TRACING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TRACE_UTIL_LIBRARY_IS_TRACING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TRACE_UTIL_LIBRARY_IS_CHANNEL_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TRACE_UTIL_LIBRARY_GET_ENABLED_CHANNELS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TRACE_UTIL_LIBRARY_GET_ALL_CHANNELS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTraceUtilLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TraceScreenshot"),
            &raw mut U_TRACE_UTIL_LIBRARY_TRACE_SCREENSHOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TraceMarkRegionStart"),
            &raw mut U_TRACE_UTIL_LIBRARY_TRACE_MARK_REGION_START,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TraceMarkRegionEnd"),
            &raw mut U_TRACE_UTIL_LIBRARY_TRACE_MARK_REGION_END,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TraceBookmark"),
            &raw mut U_TRACE_UTIL_LIBRARY_TRACE_BOOKMARK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ToggleChannel"),
            &raw mut U_TRACE_UTIL_LIBRARY_TOGGLE_CHANNEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopTracing"),
            &raw mut U_TRACE_UTIL_LIBRARY_STOP_TRACING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartTraceToFile"),
            &raw mut U_TRACE_UTIL_LIBRARY_START_TRACE_TO_FILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartTraceSendTo"),
            &raw mut U_TRACE_UTIL_LIBRARY_START_TRACE_SEND_TO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResumeTracing"),
            &raw mut U_TRACE_UTIL_LIBRARY_RESUME_TRACING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PauseTracing"),
            &raw mut U_TRACE_UTIL_LIBRARY_PAUSE_TRACING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsTracing"),
            &raw mut U_TRACE_UTIL_LIBRARY_IS_TRACING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsChannelEnabled"),
            &raw mut U_TRACE_UTIL_LIBRARY_IS_CHANNEL_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEnabledChannels"),
            &raw mut U_TRACE_UTIL_LIBRARY_GET_ENABLED_CHANNELS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllChannels"),
            &raw mut U_TRACE_UTIL_LIBRARY_GET_ALL_CHANNELS,
        );
    }
}
#[repr(C, align(8))]
pub struct UTraceUtilLibrary {
    __padding_end: [u8; 48],
}
impl UTraceUtilLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTraceUtilLibrary")
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
