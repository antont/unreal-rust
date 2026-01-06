#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub fn initialize() {}
#[repr(C, align(8))]
pub struct UMediaPlayerRecording {
    __padding_end: [u8; 112],
}
impl UMediaPlayerRecording {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaPlayerRecording")
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
pub struct UMediaSequenceRecorderSettings {
    __padding_end: [u8; 72],
}
impl UMediaSequenceRecorderSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaSequenceRecorderSettings")
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
pub struct UMediaCompositingTrackFilter {
    __padding_end: [u8; 48],
}
impl UMediaCompositingTrackFilter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaCompositingTrackFilter")
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
pub struct EMediaPlayerRecordingNumerationStyle(pub u8);
impl EMediaPlayerRecordingNumerationStyle {
    pub const APPEND_FRAME_NUMBER: EMediaPlayerRecordingNumerationStyle = EMediaPlayerRecordingNumerationStyle(
        0,
    );
    pub const APPEND_SAMPLE_TIME: EMediaPlayerRecordingNumerationStyle = EMediaPlayerRecordingNumerationStyle(
        1,
    );
}
#[repr(transparent)]
pub struct EMediaPlayerRecordingImageFormat(pub u8);
impl EMediaPlayerRecordingImageFormat {
    pub const PNG: EMediaPlayerRecordingImageFormat = EMediaPlayerRecordingImageFormat(
        0,
    );
    pub const JPEG: EMediaPlayerRecordingImageFormat = EMediaPlayerRecordingImageFormat(
        1,
    );
    pub const BMP: EMediaPlayerRecordingImageFormat = EMediaPlayerRecordingImageFormat(
        2,
    );
    pub const EXR: EMediaPlayerRecordingImageFormat = EMediaPlayerRecordingImageFormat(
        3,
    );
}
