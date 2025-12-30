#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FSequenceRecorderActorFilter {
    pub actor_classes_to_record: TArray<TSubclassOf<AActor>>,
}
#[repr(C, align(8))]
pub struct FActorRecordingSettings {
    pub settings: TArray<UPtr<UObject>>,
}
#[repr(C, align(4))]
pub struct FTimecodeBoneMethod {
    pub bone_mode: ETimecodeBoneMode,
    pub bone_name: FName,
}
#[repr(C, align(8))]
pub struct FPropertiesToRecordForActorClass {
    pub class: TSubclassOf<AActor>,
    pub properties: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FPropertiesToRecordForClass {
    pub class: TSubclassOf<UActorComponent>,
    pub properties: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FSettingsForActorClass {
    pub class: TSubclassOf<AActor>,
    pub b_record_to_possessable: bool,
}
pub struct UMovieScene3DTransformSectionRecorderSettings {
    pub b_record_transforms: bool,
}
pub struct USequenceRecordingBase {}
pub struct UActorRecording {
    pub actor_settings: FActorRecordingSettings,
    pub b_active: bool,
    pub b_create_level_sequence: bool,
    pub target_level_sequence: UPtr<ULevelSequence>,
    pub target_name: FText,
    pub take_number: u32,
    pub b_specify_target_animation: bool,
    pub target_animation: UPtr<UAnimSequence>,
    pub animation_settings: FAnimationRecordingSettings,
    pub b_record_to_possessable: bool,
    pub actor_to_record: TSoftObjectPtr<AActor>,
}
pub struct UAnimationRecordingParameters {
    pub sample_frame_rate: FFrameRate,
    pub b_end_after_duration: bool,
    pub maximum_duration_seconds: f32,
    pub sample_rate: f32,
}
pub struct USequenceRecorderActorGroup {
    pub group_name: FName,
    pub sequence_name: FString,
    pub sequence_recording_base_path: FDirectoryPath,
    pub b_specify_target_level_sequence: bool,
    pub target_level_sequence: UPtr<ULevelSequence>,
    pub b_duplicate_target_level_sequence: bool,
    pub b_record_target_level_sequence_length: bool,
    pub recorded_actors: TArray<UPtr<UActorRecording>>,
}
pub struct ASequenceRecorderGroup {
    pub actor_groups: TArray<UPtr<USequenceRecorderActorGroup>>,
}
pub struct USequenceRecorderBlueprintLibrary {}
pub struct USequenceRecorderSettings {
    pub b_create_level_sequence: bool,
    pub b_immersive_mode: bool,
    pub sequence_length: f32,
    pub recording_delay: f32,
    pub b_allow_looping: bool,
    pub global_time_dilation: f32,
    pub b_ignore_time_dilation: bool,
    pub animation_sub_directory: FString,
    pub record_audio: EAudioRecordingMode,
    pub audio_gain: f32,
    pub b_split_audio_channels_into_separate_tracks: bool,
    pub b_replace_recorded_audio: bool,
    pub audio_track_name: FText,
    pub audio_sub_directory: FString,
    pub b_record_nearby_spawned_actors: bool,
    pub nearby_actor_recording_proximity: f32,
    pub b_record_world_settings_actor: bool,
    pub b_reduce_keys: bool,
    pub b_auto_save_asset: bool,
    pub actor_filter: FSequenceRecorderActorFilter,
    pub level_sequence_actors_to_trigger: TArray<TLazyObjectPtr<ALevelSequenceActor>>,
    pub default_animation_settings: FAnimationRecordingSettings,
    pub b_record_sequencer_spawned_actors: bool,
    pub classes_and_properties_to_record: TArray<FPropertiesToRecordForClass>,
    pub actors_and_properties_to_record: TArray<FPropertiesToRecordForActorClass>,
    pub per_actor_settings: TArray<FSettingsForActorClass>,
}
