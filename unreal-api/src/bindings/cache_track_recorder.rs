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
pub static mut U_CACHE_TRACK_RECORDER_GET_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CACHE_TRACK_RECORDER_GET_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCacheTrackRecorder::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetState"),
            &raw mut U_CACHE_TRACK_RECORDER_GET_STATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequence"),
            &raw mut U_CACHE_TRACK_RECORDER_GET_SEQUENCE,
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
