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
pub static mut U_TAKE_META_DATA_UNLOCK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_TIMESTAMP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_TIMECODE_OUT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_TIMECODE_IN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_TAKE_NUMBER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_SLATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_PRESET_ORIGIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_LEVEL_ORIGIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_FRAME_RATE_FROM_TIMECODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_FRAME_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_DURATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_SET_DESCRIPTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_RECORDED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_LOCK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_IS_LOCKED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_TIMESTAMP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_TIMECODE_OUT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_TIMECODE_IN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_TAKE_NUMBER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_SLATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_PRESET_ORIGIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_LEVEL_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_LEVEL_ORIGIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_FRAME_RATE_FROM_TIMECODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_FRAME_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_DURATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GET_DESCRIPTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_META_DATA_GENERATE_ASSET_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SOURCES_START_RECORDING_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SOURCES_REMOVE_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SOURCES_GET_SOURCES_COPY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_SOURCES_ADD_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKES_CORE_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_TAKE_NUMBER_CHANGED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKES_CORE_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_SLATE_CHANGED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKES_CORE_BLUEPRINT_LIBRARY_ON_TAKE_RECORDER_TAKE_NUMBER_CHANGED_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKES_CORE_BLUEPRINT_LIBRARY_ON_TAKE_RECORDER_SLATE_CHANGED_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKES_CORE_BLUEPRINT_LIBRARY_FIND_TAKES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKES_CORE_BLUEPRINT_LIBRARY_COMPUTE_NEXT_TAKE_NUMBER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTakeMetaData::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Unlock"),
            &raw mut U_TAKE_META_DATA_UNLOCK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTimestamp"),
            &raw mut U_TAKE_META_DATA_SET_TIMESTAMP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTimecodeOut"),
            &raw mut U_TAKE_META_DATA_SET_TIMECODE_OUT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTimecodeIn"),
            &raw mut U_TAKE_META_DATA_SET_TIMECODE_IN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTakeNumber"),
            &raw mut U_TAKE_META_DATA_SET_TAKE_NUMBER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSlate"),
            &raw mut U_TAKE_META_DATA_SET_SLATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPresetOrigin"),
            &raw mut U_TAKE_META_DATA_SET_PRESET_ORIGIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLevelOrigin"),
            &raw mut U_TAKE_META_DATA_SET_LEVEL_ORIGIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFrameRateFromTimecode"),
            &raw mut U_TAKE_META_DATA_SET_FRAME_RATE_FROM_TIMECODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFrameRate"),
            &raw mut U_TAKE_META_DATA_SET_FRAME_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDuration"),
            &raw mut U_TAKE_META_DATA_SET_DURATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDescription"),
            &raw mut U_TAKE_META_DATA_SET_DESCRIPTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Recorded"),
            &raw mut U_TAKE_META_DATA_RECORDED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Lock"),
            &raw mut U_TAKE_META_DATA_LOCK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLocked"),
            &raw mut U_TAKE_META_DATA_IS_LOCKED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTimestamp"),
            &raw mut U_TAKE_META_DATA_GET_TIMESTAMP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTimecodeOut"),
            &raw mut U_TAKE_META_DATA_GET_TIMECODE_OUT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTimecodeIn"),
            &raw mut U_TAKE_META_DATA_GET_TIMECODE_IN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTakeNumber"),
            &raw mut U_TAKE_META_DATA_GET_TAKE_NUMBER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSlate"),
            &raw mut U_TAKE_META_DATA_GET_SLATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPresetOrigin"),
            &raw mut U_TAKE_META_DATA_GET_PRESET_ORIGIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLevelPath"),
            &raw mut U_TAKE_META_DATA_GET_LEVEL_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLevelOrigin"),
            &raw mut U_TAKE_META_DATA_GET_LEVEL_ORIGIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFrameRateFromTimecode"),
            &raw mut U_TAKE_META_DATA_GET_FRAME_RATE_FROM_TIMECODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFrameRate"),
            &raw mut U_TAKE_META_DATA_GET_FRAME_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDuration"),
            &raw mut U_TAKE_META_DATA_GET_DURATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDescription"),
            &raw mut U_TAKE_META_DATA_GET_DESCRIPTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenerateAssetPath"),
            &raw mut U_TAKE_META_DATA_GENERATE_ASSET_PATH,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTakeRecorderSources::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartRecordingSource"),
            &raw mut U_TAKE_RECORDER_SOURCES_START_RECORDING_SOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveSource"),
            &raw mut U_TAKE_RECORDER_SOURCES_REMOVE_SOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourcesCopy"),
            &raw mut U_TAKE_RECORDER_SOURCES_GET_SOURCES_COPY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSource"),
            &raw mut U_TAKE_RECORDER_SOURCES_ADD_SOURCE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTakesCoreBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOnTakeRecorderTakeNumberChanged"),
            &raw mut U_TAKES_CORE_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_TAKE_NUMBER_CHANGED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOnTakeRecorderSlateChanged"),
            &raw mut U_TAKES_CORE_BLUEPRINT_LIBRARY_SET_ON_TAKE_RECORDER_SLATE_CHANGED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "OnTakeRecorderTakeNumberChanged__DelegateSignature",
            ),
            &raw mut U_TAKES_CORE_BLUEPRINT_LIBRARY_ON_TAKE_RECORDER_TAKE_NUMBER_CHANGED_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnTakeRecorderSlateChanged__DelegateSignature"),
            &raw mut U_TAKES_CORE_BLUEPRINT_LIBRARY_ON_TAKE_RECORDER_SLATE_CHANGED_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindTakes"),
            &raw mut U_TAKES_CORE_BLUEPRINT_LIBRARY_FIND_TAKES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ComputeNextTakeNumber"),
            &raw mut U_TAKES_CORE_BLUEPRINT_LIBRARY_COMPUTE_NEXT_TAKE_NUMBER,
        );
    }
}
#[repr(C, align(8))]
pub struct FActorRecordedProperty {
    pub property_name: FName,
    pub b_enabled: bool,
    __padding_end: [u8; 19],
}
impl FActorRecordedProperty {}
#[repr(C, align(8))]
pub struct FAudioInputDeviceInfoProperty {
    pub device_name: FString,
    pub device_id: FString,
    pub input_channels: i32,
    pub preferred_sample_rate: i32,
    pub b_is_default_device: bool,
    __padding_end: [u8; 7],
}
impl FAudioInputDeviceInfoProperty {}
#[repr(C, align(8))]
pub struct FAudioInputDeviceProperty {
    pub b_use_system_default_audio_device: bool,
    pub device_info_array: TArray<FAudioInputDeviceInfoProperty>,
    pub device_id: FString,
    pub audio_input_buffer_size: i32,
    __padding_end: [u8; 4],
}
impl FAudioInputDeviceProperty {}
#[repr(C, align(4))]
pub struct FAudioInputDeviceChannelProperty {
    pub audio_input_device_channel: i32,
}
impl FAudioInputDeviceChannelProperty {}
#[repr(C, align(8))]
pub struct UTakeRecorderNamingTokensContext {
    __padding_end: [u8; 72],
}
impl UTakeRecorderNamingTokensContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderNamingTokensContext")
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
pub struct UTakeMetaData {
    __padding_end: [u8; 280],
}
impl UTakeMetaData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeMetaData")
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
pub struct UTakePreset {
    __padding_end: [u8; 88],
}
impl UTakePreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakePreset")
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
pub struct UTakePresetSettings {
    __padding_end: [u8; 128],
}
impl UTakePresetSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakePresetSettings")
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
pub struct UTakeRecorderSource {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub b_enabled: bool,
    pub take_number: i32,
    pub track_tint: crate::bindings::core_u_object::FColor,
    __padding_end: [u8; 4],
}
impl UTakeRecorderSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderSource")
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
pub struct UActorRecorderPropertyMap {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub recorded_object: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    pub properties: TArray<FActorRecordedProperty>,
    pub children: TArray<UPtr<UActorRecorderPropertyMap>>,
    __padding_end: [u8; 16],
}
impl UActorRecorderPropertyMap {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorRecorderPropertyMap")
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
pub struct UTakeRecorderAudioInputSettings {
    __padding_end: [u8; 80],
}
impl UTakeRecorderAudioInputSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderAudioInputSettings")
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
#[repr(C, align(16))]
pub struct UTakeRecorderSources {
    __padding_end: [u8; 416],
}
impl UTakeRecorderSources {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderSources")
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
pub struct UTakesCoreBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UTakesCoreBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakesCoreBlueprintLibrary")
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
pub struct FSetOnTakeRecorderSlateChanged_OnTakeRecorderSlateChanged {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetOnTakeRecorderTakeNumberChanged_OnTakeRecorderTakeNumberChanged {
    _opague: [u8; 32],
}
