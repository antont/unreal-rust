#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
impl UTakeRecorderActorSource {}
#[repr(C, align(8))]
pub struct UTakeRecorderCameraCutSource {
    __padding_end: [u8; 112],
}
impl UTakeRecorderCameraCutSource {}
#[repr(C, align(8))]
pub struct UTakeRecorderLevelSequenceSource {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub level_sequences_to_trigger: TArray<
        UPtr<crate::bindings::level_sequence::ULevelSequence>,
    >,
    __padding_end: [u8; 16],
}
impl UTakeRecorderLevelSequenceSource {}
#[repr(C, align(8))]
pub struct UTakeRecorderLevelVisibilitySourceSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub level_visibility_track_name: FText,
}
impl UTakeRecorderLevelVisibilitySourceSettings {}
#[repr(C, align(8))]
pub struct UTakeRecorderLevelVisibilitySource {
    __padding_end: [u8; 88],
}
impl UTakeRecorderLevelVisibilitySource {}
#[repr(C, align(8))]
pub struct UTakeRecorderMicrophoneAudioManager {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub audio_input_device: crate::bindings::takes_core::FAudioInputDeviceProperty,
    __padding_end: [u8; 40],
}
impl UTakeRecorderMicrophoneAudioManager {}
#[repr(C, align(8))]
pub struct UTakeRecorderMicrophoneAudioSourceSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub audio_source_name: FText,
    pub audio_track_name: FText,
    pub audio_asset_name: FString,
    pub audio_sub_directory: FString,
}
impl UTakeRecorderMicrophoneAudioSourceSettings {}
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
impl UTakeRecorderMicrophoneAudioSource {}
#[repr(C, align(8))]
pub struct UTakeRecorderNearbySpawnedActorSource {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub proximity: f32,
    pub b_filter_spawned_actors: bool,
    pub filter_types: TArray<TSubclassOf<crate::bindings::engine::AActor>>,
    __padding_end: [u8; 96],
}
impl UTakeRecorderNearbySpawnedActorSource {}
#[repr(C, align(8))]
pub struct UTakeRecorderPlayerSource {
    __padding_end: [u8; 72],
}
impl UTakeRecorderPlayerSource {}
#[repr(C, align(8))]
pub struct UTakeRecorderWorldSourceSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub b_record_world_settings: bool,
    pub b_autotrack_actors: bool,
    __padding_end: [u8; 6],
}
impl UTakeRecorderWorldSourceSettings {}
#[repr(C, align(8))]
pub struct UTakeRecorderWorldSource {
    __padding_end: [u8; 80],
}
impl UTakeRecorderWorldSource {}
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
