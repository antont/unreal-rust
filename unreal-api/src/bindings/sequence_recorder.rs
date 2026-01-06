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
pub static mut U_SEQUENCE_RECORDER_BLUEPRINT_LIBRARY_STOP_RECORDING_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_RECORDER_BLUEPRINT_LIBRARY_START_RECORDING_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_RECORDER_BLUEPRINT_LIBRARY_IS_RECORDING_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USequenceRecorderBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopRecordingSequence"),
            &raw mut U_SEQUENCE_RECORDER_BLUEPRINT_LIBRARY_STOP_RECORDING_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartRecordingSequence"),
            &raw mut U_SEQUENCE_RECORDER_BLUEPRINT_LIBRARY_START_RECORDING_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRecordingSequence"),
            &raw mut U_SEQUENCE_RECORDER_BLUEPRINT_LIBRARY_IS_RECORDING_SEQUENCE,
        );
    }
}
#[repr(C, align(4))]
pub struct FTimecodeBoneMethod {
    pub bone_mode: ETimecodeBoneMode,
    pub bone_name: FName,
}
impl FTimecodeBoneMethod {}
#[repr(C, align(8))]
pub struct UMovieScene3DTransformSectionRecorderSettings {
    __padding_end: [u8; 56],
}
impl UMovieScene3DTransformSectionRecorderSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieScene3DTransformSectionRecorderSettings")
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
pub struct USequenceRecordingBase {
    __padding_end: [u8; 48],
}
impl USequenceRecordingBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequenceRecordingBase")
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
pub struct UActorRecording {
    __padding_end: [u8; 376],
}
impl UActorRecording {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorRecording")
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
pub struct UAnimationRecordingParameters {
    __padding_end: [u8; 72],
}
impl UAnimationRecordingParameters {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationRecordingParameters")
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
pub struct USequenceRecorderActorGroup {
    __padding_end: [u8; 136],
}
impl USequenceRecorderActorGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequenceRecorderActorGroup")
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
pub struct ASequenceRecorderGroup {
    __padding_end: [u8; 1152],
}
impl ASequenceRecorderGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ASequenceRecorderGroup")
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
pub struct USequenceRecorderBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl USequenceRecorderBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequenceRecorderBlueprintLibrary")
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
pub struct USequenceRecorderSettings {
    __padding_end: [u8; 304],
}
impl USequenceRecorderSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequenceRecorderSettings")
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
pub struct ETimecodeBoneMode(pub u8);
impl ETimecodeBoneMode {
    pub const ALL: ETimecodeBoneMode = ETimecodeBoneMode(0);
    pub const ROOT: ETimecodeBoneMode = ETimecodeBoneMode(1);
    pub const USER_DEFINED: ETimecodeBoneMode = ETimecodeBoneMode(2);
    pub const MAX: ETimecodeBoneMode = ETimecodeBoneMode(3);
}
#[repr(transparent)]
pub struct EAudioRecordingMode(pub u8);
impl EAudioRecordingMode {
    pub const NONE: EAudioRecordingMode = EAudioRecordingMode(0);
    pub const AUDIO_TRACK: EAudioRecordingMode = EAudioRecordingMode(1);
}
