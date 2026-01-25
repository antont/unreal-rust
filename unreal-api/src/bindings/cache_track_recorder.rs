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
    pub u_cache_track_recorder_get_state: *mut crate::ffi::UFunctionOpague,
    pub u_cache_track_recorder_get_sequence: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_cache_track_recorder_get_state: std::ptr::null_mut(),
            u_cache_track_recorder_get_sequence: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCacheTrackRecorder::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetState"),
            &raw mut __FUNCTION_PTRS.u_cache_track_recorder_get_state,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequence"),
            &raw mut __FUNCTION_PTRS.u_cache_track_recorder_get_sequence,
        );
    }
}
#[repr(C, align(8))]
pub struct UCacheTrackRecorder {
    __padding_end: [u8; 288],
}
impl UCacheTrackRecorder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCacheTrackRecorder")
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
    pub fn get_state(&self) -> ECacheTrackRecorderState {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cache_track_recorder::__FUNCTION_PTRS
                    .u_cache_track_recorder_get_state,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cache_track_recorder::__FUNCTION_PTRS
                    .u_cache_track_recorder_get_state,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<ECacheTrackRecorderState>().read() }
    }
    pub fn get_sequence(&self) -> UPtr<crate::bindings::level_sequence::ULevelSequence> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cache_track_recorder::__FUNCTION_PTRS
                    .u_cache_track_recorder_get_sequence,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cache_track_recorder::__FUNCTION_PTRS
                    .u_cache_track_recorder_get_sequence,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>()
                .read()
        }
    }
}
#[repr(transparent)]
pub struct ECacheTrackRecorderState(pub u8);
impl ECacheTrackRecorderState {
    pub const STARTING: ECacheTrackRecorderState = ECacheTrackRecorderState(0);
    pub const PRE_RECORD: ECacheTrackRecorderState = ECacheTrackRecorderState(1);
    pub const TICKING_AFTER_PRE: ECacheTrackRecorderState = ECacheTrackRecorderState(2);
    pub const STARTED: ECacheTrackRecorderState = ECacheTrackRecorderState(3);
    pub const STOPPED: ECacheTrackRecorderState = ECacheTrackRecorderState(4);
    pub const CANCELLED: ECacheTrackRecorderState = ECacheTrackRecorderState(5);
}
