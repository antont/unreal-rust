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
    pub fn trace_screenshot(name: FString, b_show_ui: bool) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_TRACE_SCREENSHOT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_show_ui,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::trace_utilities::UTraceUtilLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_TRACE_SCREENSHOT,
                __buffer,
            )
        };
    }
    pub fn trace_mark_region_start(name: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_TRACE_MARK_REGION_START,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = crate::bindings::trace_utilities::UTraceUtilLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_TRACE_MARK_REGION_START,
                __buffer,
            )
        };
    }
    pub fn trace_mark_region_end(name: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_TRACE_MARK_REGION_END,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = crate::bindings::trace_utilities::UTraceUtilLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_TRACE_MARK_REGION_END,
                __buffer,
            )
        };
    }
    pub fn trace_bookmark(name: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_TRACE_BOOKMARK,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = crate::bindings::trace_utilities::UTraceUtilLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_TRACE_BOOKMARK,
                __buffer,
            )
        };
    }
    pub fn toggle_channel(channel_name: FString, enabled: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_TOGGLE_CHANNEL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &channel_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&enabled, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::trace_utilities::UTraceUtilLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_TOGGLE_CHANNEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn stop_tracing() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_STOP_TRACING,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::trace_utilities::UTraceUtilLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_STOP_TRACING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn start_trace_to_file(file_name: FString, channels: &TArray<FString>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_START_TRACE_TO_FILE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &file_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                channels,
                __buffer.add(16).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::trace_utilities::UTraceUtilLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_START_TRACE_TO_FILE,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn start_trace_send_to(target: FString, channels: &TArray<FString>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_START_TRACE_SEND_TO,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&target, __buffer.add(0).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                channels,
                __buffer.add(16).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::trace_utilities::UTraceUtilLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_START_TRACE_SEND_TO,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn resume_tracing() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_RESUME_TRACING,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::trace_utilities::UTraceUtilLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_RESUME_TRACING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn pause_tracing() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_PAUSE_TRACING,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::trace_utilities::UTraceUtilLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_PAUSE_TRACING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_tracing() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_IS_TRACING,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::trace_utilities::UTraceUtilLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_IS_TRACING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_channel_enabled(channel_name: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_IS_CHANNEL_ENABLED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &channel_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::trace_utilities::UTraceUtilLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_IS_CHANNEL_ENABLED,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_enabled_channels() -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_GET_ENABLED_CHANNELS,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::trace_utilities::UTraceUtilLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_GET_ENABLED_CHANNELS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
    pub fn get_all_channels() -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_GET_ALL_CHANNELS,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::trace_utilities::UTraceUtilLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::trace_utilities::U_TRACE_UTIL_LIBRARY_GET_ALL_CHANNELS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
}
