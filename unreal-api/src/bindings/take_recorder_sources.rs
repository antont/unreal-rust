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
    pub u_take_recorder_actor_source_set_source_actor: *mut crate::ffi::UFunctionOpague,
    pub u_take_recorder_actor_source_remove_actor_from_sources: *mut crate::ffi::UFunctionOpague,
    pub u_take_recorder_actor_source_get_source_actor: *mut crate::ffi::UFunctionOpague,
    pub u_take_recorder_actor_source_add_source_for_actor: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_take_recorder_actor_source_set_source_actor: std::ptr::null_mut(),
            u_take_recorder_actor_source_remove_actor_from_sources: std::ptr::null_mut(),
            u_take_recorder_actor_source_get_source_actor: std::ptr::null_mut(),
            u_take_recorder_actor_source_add_source_for_actor: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTakeRecorderActorSource::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSourceActor"),
                &raw mut __FUNCTION_PTRS.u_take_recorder_actor_source_set_source_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveActorFromSources"),
                &raw mut __FUNCTION_PTRS
                    .u_take_recorder_actor_source_remove_actor_from_sources,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSourceActor"),
                &raw mut __FUNCTION_PTRS.u_take_recorder_actor_source_get_source_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddSourceForActor"),
                &raw mut __FUNCTION_PTRS
                    .u_take_recorder_actor_source_add_source_for_actor,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct UTakeRecorderActorSource {
    #[doc(hidden)]
    pub(crate) __padding_72: [u8; 72],
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
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderActorSource")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderActorSource")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn set_source_actor(
        &mut self,
        in_target: TSoftObjectPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder_sources::__FUNCTION_PTRS
                    .u_take_recorder_actor_source_set_source_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_target,
                __buffer
                    .add(0)
                    .cast::<TSoftObjectPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder_sources::__FUNCTION_PTRS
                    .u_take_recorder_actor_source_set_source_actor,
                __buffer,
            )
        };
    }
    pub fn remove_actor_from_sources(
        in_actor: UPtr<crate::bindings::engine::AActor>,
        in_sources: UPtr<crate::bindings::takes_core::UTakeRecorderSources>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder_sources::__FUNCTION_PTRS
                    .u_take_recorder_actor_source_remove_actor_from_sources,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sources,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::takes_core::UTakeRecorderSources>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::take_recorder_sources::UTakeRecorderActorSource::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder_sources::__FUNCTION_PTRS
                    .u_take_recorder_actor_source_remove_actor_from_sources,
                __buffer,
            )
        };
    }
    pub fn get_source_actor(&self) -> TSoftObjectPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder_sources::__FUNCTION_PTRS
                    .u_take_recorder_actor_source_get_source_actor,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder_sources::__FUNCTION_PTRS
                    .u_take_recorder_actor_source_get_source_actor,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TSoftObjectPtr<crate::bindings::engine::AActor>>()
                .read()
        }
    }
    pub fn add_source_for_actor(
        in_actor: UPtr<crate::bindings::engine::AActor>,
        in_sources: UPtr<crate::bindings::takes_core::UTakeRecorderSources>,
    ) -> UPtr<crate::bindings::takes_core::UTakeRecorderSource> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::take_recorder_sources::__FUNCTION_PTRS
                    .u_take_recorder_actor_source_add_source_for_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sources,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::takes_core::UTakeRecorderSources>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::take_recorder_sources::UTakeRecorderActorSource::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::take_recorder_sources::__FUNCTION_PTRS
                    .u_take_recorder_actor_source_add_source_for_actor,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::takes_core::UTakeRecorderSource>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UTakeRecorderCameraCutSource {
    __padding_end: [u8; 112],
}
impl UTakeRecorderCameraCutSource {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderCameraCutSource")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderCameraCutSource")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTakeRecorderLevelSequenceSource {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub level_sequences_to_trigger: TArray<
        UPtr<crate::bindings::level_sequence::ULevelSequence>,
    >,
    __padding_end: [u8; 16],
}
impl UTakeRecorderLevelSequenceSource {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderLevelSequenceSource")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderLevelSequenceSource")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTakeRecorderLevelVisibilitySourceSettings {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub level_visibility_track_name: FText,
}
impl UTakeRecorderLevelVisibilitySourceSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderLevelVisibilitySourceSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderLevelVisibilitySourceSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTakeRecorderLevelVisibilitySource {
    __padding_end: [u8; 88],
}
impl UTakeRecorderLevelVisibilitySource {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderLevelVisibilitySource")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderLevelVisibilitySource")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTakeRecorderMicrophoneAudioManager {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub audio_input_device: crate::bindings::takes_core::FAudioInputDeviceProperty,
    __padding_end: [u8; 40],
}
impl UTakeRecorderMicrophoneAudioManager {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderMicrophoneAudioManager")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderMicrophoneAudioManager")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTakeRecorderMicrophoneAudioSourceSettings {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub audio_source_name: FText,
    pub audio_track_name: FText,
    pub audio_asset_name: FString,
    pub audio_sub_directory: FString,
}
impl UTakeRecorderMicrophoneAudioSourceSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderMicrophoneAudioSourceSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderMicrophoneAudioSourceSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTakeRecorderMicrophoneAudioSource {
    #[doc(hidden)]
    pub(crate) __padding_128: [u8; 128],
    pub audio_gain: f32,
    #[doc(hidden)]
    pub(crate) __padding_133: [u8; 1],
    pub b_replace_recorded_audio: bool,
    pub audio_channel: crate::bindings::takes_core::FAudioInputDeviceChannelProperty,
    __padding_end: [u8; 84],
}
impl UTakeRecorderMicrophoneAudioSource {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderMicrophoneAudioSource")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderMicrophoneAudioSource")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTakeRecorderNearbySpawnedActorSource {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub proximity: f32,
    pub b_filter_spawned_actors: bool,
    pub filter_types: TArray<TSubclassOf<crate::bindings::engine::AActor>>,
    __padding_end: [u8; 96],
}
impl UTakeRecorderNearbySpawnedActorSource {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderNearbySpawnedActorSource")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderNearbySpawnedActorSource")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTakeRecorderPlayerSource {
    __padding_end: [u8; 72],
}
impl UTakeRecorderPlayerSource {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderPlayerSource")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderPlayerSource")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTakeRecorderWorldSourceSettings {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub b_record_world_settings: bool,
    pub b_autotrack_actors: bool,
}
impl UTakeRecorderWorldSourceSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderWorldSourceSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderWorldSourceSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTakeRecorderWorldSource {
    __padding_end: [u8; 80],
}
impl UTakeRecorderWorldSource {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderWorldSource")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTakeRecorderWorldSource")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
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
