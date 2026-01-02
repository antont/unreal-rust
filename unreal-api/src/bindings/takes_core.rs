#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
impl UTakeRecorderNamingTokensContext {}
#[repr(C, align(8))]
pub struct UTakeMetaData {
    __padding_end: [u8; 280],
}
impl UTakeMetaData {}
#[repr(C, align(8))]
pub struct UTakePreset {
    __padding_end: [u8; 88],
}
impl UTakePreset {}
#[repr(C, align(8))]
pub struct UTakePresetSettings {
    __padding_end: [u8; 128],
}
impl UTakePresetSettings {}
#[repr(C, align(8))]
pub struct UTakeRecorderSource {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub b_enabled: bool,
    pub take_number: i32,
    pub track_tint: crate::bindings::core_u_object::FColor,
    __padding_end: [u8; 4],
}
impl UTakeRecorderSource {}
#[repr(C, align(8))]
pub struct UActorRecorderPropertyMap {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub recorded_object: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    pub properties: TArray<FActorRecordedProperty>,
    pub children: TArray<UPtr<UActorRecorderPropertyMap>>,
    __padding_end: [u8; 16],
}
impl UActorRecorderPropertyMap {}
#[repr(C, align(8))]
pub struct UTakeRecorderAudioInputSettings {
    __padding_end: [u8; 80],
}
impl UTakeRecorderAudioInputSettings {}
#[repr(C, align(16))]
pub struct UTakeRecorderSources {
    __padding_end: [u8; 416],
}
impl UTakeRecorderSources {}
#[repr(C, align(8))]
pub struct UTakesCoreBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UTakesCoreBlueprintLibrary {}
#[repr(transparent)]
pub struct FSetOnTakeRecorderSlateChanged_OnTakeRecorderSlateChanged {
    _opague: u8,
}
#[repr(transparent)]
pub struct FSetOnTakeRecorderTakeNumberChanged_OnTakeRecorderTakeNumberChanged {
    _opague: u8,
}
