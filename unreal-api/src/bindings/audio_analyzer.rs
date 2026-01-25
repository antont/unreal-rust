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
    pub u_audio_analyzer_stop_analyzing: *mut crate::ffi::UFunctionOpague,
    pub u_audio_analyzer_start_analyzing: *mut crate::ffi::UFunctionOpague,
    pub u_audio_analyzer_nrt_set_sound: *mut crate::ffi::UFunctionOpague,
    pub u_audio_analyzer_nrt_analyze_audio: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_audio_analyzer_stop_analyzing: std::ptr::null_mut(),
            u_audio_analyzer_start_analyzing: std::ptr::null_mut(),
            u_audio_analyzer_nrt_set_sound: std::ptr::null_mut(),
            u_audio_analyzer_nrt_analyze_audio: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAudioAnalyzer::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopAnalyzing"),
            &raw mut __FUNCTION_PTRS.u_audio_analyzer_stop_analyzing,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartAnalyzing"),
            &raw mut __FUNCTION_PTRS.u_audio_analyzer_start_analyzing,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAudioAnalyzerNRT::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSound"),
            &raw mut __FUNCTION_PTRS.u_audio_analyzer_nrt_set_sound,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AnalyzeAudio"),
            &raw mut __FUNCTION_PTRS.u_audio_analyzer_nrt_analyze_audio,
        );
    }
}
#[repr(C, align(8))]
pub struct UAudioAnalyzerAssetBase {
    __padding_end: [u8; 48],
}
impl UAudioAnalyzerAssetBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioAnalyzerAssetBase")
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
pub struct UAudioAnalyzerSettings {
    __padding_end: [u8; 48],
}
impl UAudioAnalyzerSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioAnalyzerSettings")
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
pub struct UAudioAnalyzer {
    __padding_end: [u8; 168],
}
impl UAudioAnalyzer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioAnalyzer")
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
    pub fn stop_analyzing(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_analyzer::__FUNCTION_PTRS
                    .u_audio_analyzer_stop_analyzing,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_analyzer::__FUNCTION_PTRS
                    .u_audio_analyzer_stop_analyzing,
                __buffer,
            )
        };
    }
    pub fn start_analyzing(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        audio_bus_to_analyze: UPtr<crate::bindings::engine::UAudioBus>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_analyzer::__FUNCTION_PTRS
                    .u_audio_analyzer_start_analyzing,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &audio_bus_to_analyze,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UAudioBus>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_analyzer::__FUNCTION_PTRS
                    .u_audio_analyzer_start_analyzing,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UAudioAnalyzerNRTSettings {
    __padding_end: [u8; 80],
}
impl UAudioAnalyzerNRTSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioAnalyzerNRTSettings")
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
pub struct UAudioAnalyzerNRT {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub sound: UPtr<crate::bindings::engine::USoundWave>,
    pub duration_in_seconds: f32,
    __padding_end: [u8; 172],
}
impl UAudioAnalyzerNRT {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioAnalyzerNRT")
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
    pub fn set_sound(&mut self, in_sound: UPtr<crate::bindings::engine::USoundWave>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_analyzer::__FUNCTION_PTRS
                    .u_audio_analyzer_nrt_set_sound,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sound,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USoundWave>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_analyzer::__FUNCTION_PTRS
                    .u_audio_analyzer_nrt_set_sound,
                __buffer,
            )
        };
    }
    pub fn analyze_audio(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_analyzer::__FUNCTION_PTRS
                    .u_audio_analyzer_nrt_analyze_audio,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_analyzer::__FUNCTION_PTRS
                    .u_audio_analyzer_nrt_analyze_audio,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UAudioAnalyzerSubsystem {
    __padding_end: [u8; 88],
}
impl UAudioAnalyzerSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioAnalyzerSubsystem")
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
pub struct FAudioAnalyzerNRT_OnAnalysisComplete {
    _opague: [u8; 24],
}
