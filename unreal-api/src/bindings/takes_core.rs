#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FTakeRecorderTargetRecordClassProperty {
    pub target_record_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
}
#[repr(C, align(8))]
pub struct FActorRecordedProperty {
    pub property_name: FName,
    pub b_enabled: bool,
    pub recorder_name: FText,
}
#[repr(C, align(8))]
pub struct FAudioInputDeviceInfoProperty {
    pub device_name: FString,
    pub device_id: FString,
    pub input_channels: i32,
    pub preferred_sample_rate: i32,
    pub b_is_default_device: bool,
}
#[repr(C, align(8))]
pub struct FAudioInputDeviceProperty {
    pub b_use_system_default_audio_device: bool,
    pub device_info_array: TArray<FAudioInputDeviceInfoProperty>,
    pub device_id: FString,
    pub audio_input_buffer_size: i32,
}
#[repr(C, align(4))]
pub struct FAudioInputDeviceChannelProperty {
    pub audio_input_device_channel: i32,
}
pub struct UTakeRecorderNamingTokensContext {
    pub take_meta_data: TWeakObjectPtr<UTakeMetaData>,
    pub actor: TWeakObjectPtr<crate::bindings::engine::AActor>,
    pub audio_input_device_channel: i32,
}
pub struct UTakeMetaData {
    pub b_is_locked: bool,
    pub slate: FString,
    pub take_number: i32,
    pub timestamp: crate::bindings::core_u_object::FDateTime,
    pub timecode_in: crate::bindings::core_u_object::FTimecode,
    pub timecode_out: crate::bindings::core_u_object::FTimecode,
    pub duration: crate::bindings::core_u_object::FFrameTime,
    pub frame_rate: crate::bindings::core_u_object::FFrameRate,
    pub description: FString,
    pub preset_origin: TSoftObjectPtr<UTakePreset>,
    pub level_origin: TSoftObjectPtr<crate::bindings::engine::ULevel>,
    pub b_frame_rate_from_timecode: bool,
}
pub struct UTakePreset {
    pub level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
}
pub struct UTakePresetSettings {
    pub target_record_class: FTakeRecorderTargetRecordClassProperty,
}
pub struct UTakeRecorderSource {
    pub b_enabled: bool,
    pub take_number: i32,
    pub track_tint: crate::bindings::core_u_object::FColor,
}
pub struct UActorRecorderPropertyMap {
    pub recorded_object: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    pub properties: TArray<FActorRecordedProperty>,
    pub children: TArray<UPtr<UActorRecorderPropertyMap>>,
}
pub struct UTakeRecorderAudioInputSettings {}
pub struct UTakeRecorderSources {
    pub sources: TArray<UPtr<UTakeRecorderSource>>,
    pub source_sub_sequence_map: TMap<
        UPtr<UTakeRecorderSource>,
        UPtr<crate::bindings::level_sequence::ULevelSequence>,
    >,
    pub active_sub_sections: TArray<
        UPtr<crate::bindings::movie_scene::UMovieSceneSubSection>,
    >,
}
pub struct UTakesCoreBlueprintLibrary {}
pub struct FSetOnTakeRecorderSlateChanged_OnTakeRecorderSlateChanged;
pub struct FSetOnTakeRecorderTakeNumberChanged_OnTakeRecorderTakeNumberChanged;
