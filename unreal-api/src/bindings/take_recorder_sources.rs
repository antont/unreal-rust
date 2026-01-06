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
pub static mut U_TAKE_RECORDER_ACTOR_SOURCE_SET_SOURCE_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_ACTOR_SOURCE_REMOVE_ACTOR_FROM_SOURCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_ACTOR_SOURCE_GET_SOURCE_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TAKE_RECORDER_ACTOR_SOURCE_ADD_SOURCE_FOR_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTakeRecorderActorSource::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSourceActor"),
            &raw mut U_TAKE_RECORDER_ACTOR_SOURCE_SET_SOURCE_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveActorFromSources"),
            &raw mut U_TAKE_RECORDER_ACTOR_SOURCE_REMOVE_ACTOR_FROM_SOURCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourceActor"),
            &raw mut U_TAKE_RECORDER_ACTOR_SOURCE_GET_SOURCE_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSourceForActor"),
            &raw mut U_TAKE_RECORDER_ACTOR_SOURCE_ADD_SOURCE_FOR_ACTOR,
        );
    }
}
#[repr(C, align(8))]
pub struct UTakeRecorderActorSource {
    #[doc(hidden)]
    __padding_72: [u8; 72],
    pub target: TSoftObjectPtr<crate::bindings::engine::AActor>,
    pub record_type: ETakeRecorderActorRecordType,
    pub b_record_parent_hierarchy: bool,
    pub b_reduce_keys: bool,
    pub recorded_properties: UPtr<
        crate::bindings::takes_core::UActorRecorderPropertyMap,
    >,
    pub include_animation_names: TArray<FString>,
    pub exclude_animation_names: TArray<FString>,
    __padding_end: [u8; 344],
}
impl UTakeRecorderActorSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderActorSource")
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
pub struct UTakeRecorderCameraCutSource {
    __padding_end: [u8; 112],
}
impl UTakeRecorderCameraCutSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderCameraCutSource")
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
pub struct UTakeRecorderLevelSequenceSource {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub level_sequences_to_trigger: TArray<
        UPtr<crate::bindings::level_sequence::ULevelSequence>,
    >,
    __padding_end: [u8; 16],
}
impl UTakeRecorderLevelSequenceSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderLevelSequenceSource")
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
pub struct UTakeRecorderLevelVisibilitySourceSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub level_visibility_track_name: FText,
}
impl UTakeRecorderLevelVisibilitySourceSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderLevelVisibilitySourceSettings")
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
pub struct UTakeRecorderLevelVisibilitySource {
    __padding_end: [u8; 88],
}
impl UTakeRecorderLevelVisibilitySource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderLevelVisibilitySource")
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
pub struct UTakeRecorderMicrophoneAudioManager {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub audio_input_device: crate::bindings::takes_core::FAudioInputDeviceProperty,
    __padding_end: [u8; 40],
}
impl UTakeRecorderMicrophoneAudioManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderMicrophoneAudioManager")
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
pub struct UTakeRecorderMicrophoneAudioSourceSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub audio_source_name: FText,
    pub audio_track_name: FText,
    pub audio_asset_name: FString,
    pub audio_sub_directory: FString,
}
impl UTakeRecorderMicrophoneAudioSourceSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderMicrophoneAudioSourceSettings")
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
pub struct UTakeRecorderMicrophoneAudioSource {
    #[doc(hidden)]
    __padding_128: [u8; 128],
    pub audio_gain: f32,
    #[doc(hidden)]
    __padding_133: [u8; 1],
    pub b_replace_recorded_audio: bool,
    pub audio_channel: crate::bindings::takes_core::FAudioInputDeviceChannelProperty,
    __padding_end: [u8; 84],
}
impl UTakeRecorderMicrophoneAudioSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderMicrophoneAudioSource")
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
pub struct UTakeRecorderNearbySpawnedActorSource {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub proximity: f32,
    pub b_filter_spawned_actors: bool,
    pub filter_types: TArray<TSubclassOf<crate::bindings::engine::AActor>>,
    __padding_end: [u8; 96],
}
impl UTakeRecorderNearbySpawnedActorSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderNearbySpawnedActorSource")
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
pub struct UTakeRecorderPlayerSource {
    __padding_end: [u8; 72],
}
impl UTakeRecorderPlayerSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderPlayerSource")
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
pub struct UTakeRecorderWorldSourceSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub b_record_world_settings: bool,
    pub b_autotrack_actors: bool,
    __padding_end: [u8; 6],
}
impl UTakeRecorderWorldSourceSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderWorldSourceSettings")
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
pub struct UTakeRecorderWorldSource {
    __padding_end: [u8; 80],
}
impl UTakeRecorderWorldSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderWorldSource")
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
pub struct ETakeRecorderActorRecordType(pub u8);
impl ETakeRecorderActorRecordType {
    pub const POSSESSABLE: ETakeRecorderActorRecordType = ETakeRecorderActorRecordType(
        0,
    );
    pub const SPAWNABLE: ETakeRecorderActorRecordType = ETakeRecorderActorRecordType(1);
    pub const PROJECT_DEFAULT: ETakeRecorderActorRecordType = ETakeRecorderActorRecordType(
        2,
    );
}
