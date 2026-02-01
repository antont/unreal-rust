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
    pub u_trace_util_library_trace_screenshot: *mut crate::ffi::UFunctionOpague,
    pub u_trace_util_library_trace_mark_region_start: *mut crate::ffi::UFunctionOpague,
    pub u_trace_util_library_trace_mark_region_end: *mut crate::ffi::UFunctionOpague,
    pub u_trace_util_library_trace_bookmark: *mut crate::ffi::UFunctionOpague,
    pub u_trace_util_library_toggle_channel: *mut crate::ffi::UFunctionOpague,
    pub u_trace_util_library_stop_tracing: *mut crate::ffi::UFunctionOpague,
    pub u_trace_util_library_start_trace_to_file: *mut crate::ffi::UFunctionOpague,
    pub u_trace_util_library_start_trace_send_to: *mut crate::ffi::UFunctionOpague,
    pub u_trace_util_library_resume_tracing: *mut crate::ffi::UFunctionOpague,
    pub u_trace_util_library_pause_tracing: *mut crate::ffi::UFunctionOpague,
    pub u_trace_util_library_is_tracing: *mut crate::ffi::UFunctionOpague,
    pub u_trace_util_library_is_channel_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_trace_util_library_get_enabled_channels: *mut crate::ffi::UFunctionOpague,
    pub u_trace_util_library_get_all_channels: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_trace_util_library_trace_screenshot: std::ptr::null_mut(),
            u_trace_util_library_trace_mark_region_start: std::ptr::null_mut(),
            u_trace_util_library_trace_mark_region_end: std::ptr::null_mut(),
            u_trace_util_library_trace_bookmark: std::ptr::null_mut(),
            u_trace_util_library_toggle_channel: std::ptr::null_mut(),
            u_trace_util_library_stop_tracing: std::ptr::null_mut(),
            u_trace_util_library_start_trace_to_file: std::ptr::null_mut(),
            u_trace_util_library_start_trace_send_to: std::ptr::null_mut(),
            u_trace_util_library_resume_tracing: std::ptr::null_mut(),
            u_trace_util_library_pause_tracing: std::ptr::null_mut(),
            u_trace_util_library_is_tracing: std::ptr::null_mut(),
            u_trace_util_library_is_channel_enabled: std::ptr::null_mut(),
            u_trace_util_library_get_enabled_channels: std::ptr::null_mut(),
            u_trace_util_library_get_all_channels: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTraceUtilLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TraceScreenshot"),
                &raw mut __FUNCTION_PTRS.u_trace_util_library_trace_screenshot,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TraceMarkRegionStart"),
                &raw mut __FUNCTION_PTRS.u_trace_util_library_trace_mark_region_start,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TraceMarkRegionEnd"),
                &raw mut __FUNCTION_PTRS.u_trace_util_library_trace_mark_region_end,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TraceBookmark"),
                &raw mut __FUNCTION_PTRS.u_trace_util_library_trace_bookmark,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ToggleChannel"),
                &raw mut __FUNCTION_PTRS.u_trace_util_library_toggle_channel,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StopTracing"),
                &raw mut __FUNCTION_PTRS.u_trace_util_library_stop_tracing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartTraceToFile"),
                &raw mut __FUNCTION_PTRS.u_trace_util_library_start_trace_to_file,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartTraceSendTo"),
                &raw mut __FUNCTION_PTRS.u_trace_util_library_start_trace_send_to,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResumeTracing"),
                &raw mut __FUNCTION_PTRS.u_trace_util_library_resume_tracing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PauseTracing"),
                &raw mut __FUNCTION_PTRS.u_trace_util_library_pause_tracing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsTracing"),
                &raw mut __FUNCTION_PTRS.u_trace_util_library_is_tracing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsChannelEnabled"),
                &raw mut __FUNCTION_PTRS.u_trace_util_library_is_channel_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEnabledChannels"),
                &raw mut __FUNCTION_PTRS.u_trace_util_library_get_enabled_channels,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllChannels"),
                &raw mut __FUNCTION_PTRS.u_trace_util_library_get_all_channels,
            );
        }
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTraceUtilLibrary")
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
    pub fn trace_screenshot(name: FString, b_show_ui: bool) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_trace_screenshot,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_trace_screenshot,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_trace_mark_region_start,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_trace_mark_region_start,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_trace_mark_region_end,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_trace_mark_region_end,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_trace_bookmark,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_trace_bookmark,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_toggle_channel,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_toggle_channel,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_stop_tracing,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::trace_utilities::UTraceUtilLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_stop_tracing,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_start_trace_to_file,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_start_trace_to_file,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_start_trace_send_to,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_start_trace_send_to,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_resume_tracing,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::trace_utilities::UTraceUtilLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_resume_tracing,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_pause_tracing,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::trace_utilities::UTraceUtilLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_pause_tracing,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_is_tracing,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::trace_utilities::UTraceUtilLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_is_tracing,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_is_channel_enabled,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_is_channel_enabled,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_get_enabled_channels,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::trace_utilities::UTraceUtilLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_get_enabled_channels,
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
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_get_all_channels,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::trace_utilities::UTraceUtilLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::trace_utilities::__FUNCTION_PTRS
                    .u_trace_util_library_get_all_channels,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
}
