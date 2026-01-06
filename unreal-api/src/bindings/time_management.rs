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
pub static mut U_MUSICAL_TIME_FUNCTION_LIBRARY_IS_VALID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MUSICAL_TIME_FUNCTION_LIBRARY_FRACTIONAL_BEATS_IN_BAR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MUSICAL_TIME_FUNCTION_LIBRARY_FRACTIONAL_BARS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MUSICAL_TIME_FUNCTION_LIBRARY_BAR_AND_BEAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_TRANSFORM_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_SUBTRACT_FRAME_NUMBER_INTEGER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_SUBTRACT_FRAME_NUMBER_FRAME_NUMBER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_SNAP_FRAME_TIME_TO_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_MULTIPLY_SECONDS_FRAME_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_MULTIPLY_FRAME_NUMBER_INTEGER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_IS_VALID_MULTIPLE_OF: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_IS_VALID_FRAMERATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_GET_TIMECODE_FRAME_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_GET_TIMECODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_DIVIDE_FRAME_NUMBER_INTEGER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_CONV_TIMECODE_TO_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_CONV_QUALIFIED_FRAME_TIME_TO_SECONDS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_CONV_FRAME_RATE_TO_SECONDS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_CONV_FRAME_RATE_TO_INTERVAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_CONV_FRAME_NUMBER_TO_INTEGER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_ADD_FRAME_NUMBER_INTEGER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_ADD_FRAME_NUMBER_FRAME_NUMBER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMusicalTimeFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValid"),
            &raw mut U_MUSICAL_TIME_FUNCTION_LIBRARY_IS_VALID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FractionalBeatsInBar"),
            &raw mut U_MUSICAL_TIME_FUNCTION_LIBRARY_FRACTIONAL_BEATS_IN_BAR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FractionalBars"),
            &raw mut U_MUSICAL_TIME_FUNCTION_LIBRARY_FRACTIONAL_BARS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BarAndBeat"),
            &raw mut U_MUSICAL_TIME_FUNCTION_LIBRARY_BAR_AND_BEAT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTimeManagementBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TransformTime"),
            &raw mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_TRANSFORM_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Subtract_FrameNumberInteger"),
            &raw mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_SUBTRACT_FRAME_NUMBER_INTEGER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Subtract_FrameNumberFrameNumber"),
            &raw mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_SUBTRACT_FRAME_NUMBER_FRAME_NUMBER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SnapFrameTimeToRate"),
            &raw mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_SNAP_FRAME_TIME_TO_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Multiply_SecondsFrameRate"),
            &raw mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_MULTIPLY_SECONDS_FRAME_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Multiply_FrameNumberInteger"),
            &raw mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_MULTIPLY_FRAME_NUMBER_INTEGER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValid_MultipleOf"),
            &raw mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_IS_VALID_MULTIPLE_OF,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValid_Framerate"),
            &raw mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_IS_VALID_FRAMERATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTimecodeFrameRate"),
            &raw mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_GET_TIMECODE_FRAME_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTimecode"),
            &raw mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_GET_TIMECODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Divide_FrameNumberInteger"),
            &raw mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_DIVIDE_FRAME_NUMBER_INTEGER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_TimecodeToString"),
            &raw mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_CONV_TIMECODE_TO_STRING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_QualifiedFrameTimeToSeconds"),
            &raw mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_CONV_QUALIFIED_FRAME_TIME_TO_SECONDS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_FrameRateToSeconds"),
            &raw mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_CONV_FRAME_RATE_TO_SECONDS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_FrameRateToInterval"),
            &raw mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_CONV_FRAME_RATE_TO_INTERVAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_FrameNumberToInteger"),
            &raw mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_CONV_FRAME_NUMBER_TO_INTEGER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Add_FrameNumberInteger"),
            &raw mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_ADD_FRAME_NUMBER_INTEGER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Add_FrameNumberFrameNumber"),
            &raw mut U_TIME_MANAGEMENT_BLUEPRINT_LIBRARY_ADD_FRAME_NUMBER_FRAME_NUMBER,
        );
    }
}
#[repr(C, align(8))]
pub struct FTimedDataChannelSampleTime {
    __padding_end: [u8; 24],
}
impl FTimedDataChannelSampleTime {}
#[repr(C, align(4))]
pub struct FTimedDataInputEvaluationData {
    pub distance_to_newest_sample_seconds: f32,
    pub distance_to_oldest_sample_seconds: f32,
}
impl FTimedDataInputEvaluationData {}
pub struct IClockedTimeStep {}
#[repr(C, align(8))]
pub struct UClockedTimeStep {
    __padding_end: [u8; 48],
}
impl UClockedTimeStep {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClockedTimeStep")
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
pub struct UFixedFrameRateCustomTimeStep {
    __padding_end: [u8; 48],
}
impl UFixedFrameRateCustomTimeStep {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFixedFrameRateCustomTimeStep")
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
pub struct UCatchupFixedRateCustomTimeStep {
    __padding_end: [u8; 80],
}
impl UCatchupFixedRateCustomTimeStep {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCatchupFixedRateCustomTimeStep")
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
pub struct UTimecodeRegressionProvider {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub num_sampled_frames: i32,
    __padding_end: [u8; 180],
}
impl UTimecodeRegressionProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTimecodeRegressionProvider")
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
pub struct UGenlockedCustomTimeStep {
    __padding_end: [u8; 56],
}
impl UGenlockedCustomTimeStep {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGenlockedCustomTimeStep")
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
pub struct UGenlockedFixedRateCustomTimeStep {
    __padding_end: [u8; 88],
}
impl UGenlockedFixedRateCustomTimeStep {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGenlockedFixedRateCustomTimeStep")
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
pub struct UGenlockedTimecodeProvider {
    __padding_end: [u8; 96],
}
impl UGenlockedTimecodeProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGenlockedTimecodeProvider")
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
pub struct UMusicalTimeFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UMusicalTimeFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMusicalTimeFunctionLibrary")
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
pub struct UTimeManagementBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UTimeManagementBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTimeManagementBlueprintLibrary")
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
pub struct UTimeSynchronizationSource {
    __padding_end: [u8; 56],
}
impl UTimeSynchronizationSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTimeSynchronizationSource")
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
pub struct EFrameNumberDisplayFormats(pub u8);
impl EFrameNumberDisplayFormats {
    pub const NON_DROP_FRAME_TIMECODE: EFrameNumberDisplayFormats = EFrameNumberDisplayFormats(
        0,
    );
    pub const DROP_FRAME_TIMECODE: EFrameNumberDisplayFormats = EFrameNumberDisplayFormats(
        1,
    );
    pub const SECONDS: EFrameNumberDisplayFormats = EFrameNumberDisplayFormats(2);
    pub const FRAMES: EFrameNumberDisplayFormats = EFrameNumberDisplayFormats(3);
    pub const CUSTOM: EFrameNumberDisplayFormats = EFrameNumberDisplayFormats(4);
    pub const MAX_COUNT: EFrameNumberDisplayFormats = EFrameNumberDisplayFormats(5);
}
