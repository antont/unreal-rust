#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FTakeRecorderTargetRecordClassProperty {
    pub target_record_class: TSoftObjectPtr<UClass>,
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
    pub actor: TWeakObjectPtr<AActor>,
    pub audio_input_device_channel: i32,
}
pub struct UTakeMetaData {
    pub b_is_locked: bool,
    pub slate: FString,
    pub take_number: i32,
    pub timestamp: FDateTime,
    pub timecode_in: FTimecode,
    pub timecode_out: FTimecode,
    pub duration: FFrameTime,
    pub frame_rate: FFrameRate,
    pub description: FString,
    pub preset_origin: TSoftObjectPtr<UTakePreset>,
    pub level_origin: TSoftObjectPtr<ULevel>,
    pub b_frame_rate_from_timecode: bool,
}
pub struct UTakePreset {
    pub level_sequence: UPtr<ULevelSequence>,
}
pub struct UTakePresetSettings {
    pub target_record_class: FTakeRecorderTargetRecordClassProperty,
}
pub struct UTakeRecorderSource {
    pub b_enabled: bool,
    pub take_number: i32,
    pub track_tint: FColor,
}
pub struct UActorRecorderPropertyMap {
    pub recorded_object: TSoftObjectPtr<UObject>,
    pub properties: TArray<FActorRecordedProperty>,
    pub children: TArray<UPtr<UActorRecorderPropertyMap>>,
}
pub struct UTakeRecorderAudioInputSettings {}
pub struct UTakeRecorderSources {
    pub sources: TArray<UPtr<UTakeRecorderSource>>,
    pub source_sub_sequence_map: TMap<UPtr<UTakeRecorderSource>, UPtr<ULevelSequence>>,
    pub active_sub_sections: TArray<UPtr<UMovieSceneSubSection>>,
}
pub struct UTakesCoreBlueprintLibrary {}
