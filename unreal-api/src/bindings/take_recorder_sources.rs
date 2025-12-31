#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UTakeRecorderActorSource {
    pub target: TSoftObjectPtr<crate::bindings::engine::AActor>,
    pub record_type: ETakeRecorderActorRecordType,
    pub b_record_parent_hierarchy: bool,
    pub b_reduce_keys: bool,
    pub recorded_properties: UPtr<
        crate::bindings::takes_core::UActorRecorderPropertyMap,
    >,
    pub include_animation_names: TArray<FString>,
    pub exclude_animation_names: TArray<FString>,
    pub target_level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    pub root_level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    pub factory_settings: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub track_recorders: TArray<
        UPtr<crate::bindings::take_track_recorders::UMovieSceneTrackRecorder>,
    >,
    pub parent_source: UPtr<UTakeRecorderActorSource>,
    pub b_show_progress_dialog: bool,
}
pub struct UTakeRecorderCameraCutSource {
    pub world: UPtr<crate::bindings::engine::UWorld>,
    pub root_level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
}
pub struct UTakeRecorderLevelSequenceSource {
    pub level_sequences_to_trigger: TArray<
        UPtr<crate::bindings::level_sequence::ULevelSequence>,
    >,
}
pub struct UTakeRecorderLevelVisibilitySourceSettings {
    pub level_visibility_track_name: FText,
}
pub struct UTakeRecorderLevelVisibilitySource {}
pub struct UTakeRecorderMicrophoneAudioManager {
    pub audio_input_device: crate::bindings::takes_core::FAudioInputDeviceProperty,
}
pub struct UTakeRecorderMicrophoneAudioSourceSettings {
    pub audio_source_name: FText,
    pub audio_track_name: FText,
    pub audio_asset_name: FString,
    pub audio_sub_directory: FString,
}
pub struct UTakeRecorderMicrophoneAudioSource {
    pub audio_gain: f32,
    pub b_split_audio_channels_into_separate_tracks_deprecated: bool,
    pub b_replace_recorded_audio: bool,
    pub audio_channel: crate::bindings::takes_core::FAudioInputDeviceChannelProperty,
}
pub struct UTakeRecorderNearbySpawnedActorSource {
    pub proximity: f32,
    pub b_filter_spawned_actors: bool,
    pub filter_types: TArray<TSubclassOf<crate::bindings::engine::AActor>>,
}
pub struct UTakeRecorderPlayerSource {}
pub struct UTakeRecorderWorldSourceSettings {
    pub b_record_world_settings: bool,
    pub b_autotrack_actors: bool,
}
pub struct UTakeRecorderWorldSource {}
#[allow(non_camel_case_types)]
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
