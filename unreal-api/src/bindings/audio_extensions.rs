#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FSoundGeneratorOutput {
    pub name: FName,
}
impl FSoundGeneratorOutput {}
#[repr(C, align(8))]
pub struct FAudioParameter {
    pub param_name: FName,
    pub float_param: f32,
    pub bool_param: bool,
    pub int_param: i32,
    pub object_param: UPtr<crate::bindings::core_u_object::UObject>,
    pub string_param: FString,
    pub array_float_param: TArray<f32>,
    pub array_bool_param: TArray<bool>,
    pub array_int_param: TArray<i32>,
    pub array_object_param: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub array_string_param: TArray<FString>,
    pub param_type: EAudioParameterType,
    __padding_end: [u8; 31],
}
impl FAudioParameter {}
#[repr(C, align(8))]
pub struct FSoundWaveCuePoint {
    pub cue_point_id: i32,
    pub label: FString,
    pub frame_position: i64,
    pub frame_length: i64,
    pub b_is_loop_region: bool,
    __padding_end: [u8; 7],
}
impl FSoundWaveCuePoint {}
pub struct IAudioPropertiesSheetAssetUserInterface {}
#[repr(C, align(8))]
pub struct UAudioPropertiesSheetAssetUserInterface {
    __padding_end: [u8; 48],
}
impl UAudioPropertiesSheetAssetUserInterface {}
#[repr(C, align(8))]
pub struct UAudioPropertiesSheetAssetBase {
    __padding_end: [u8; 48],
}
impl UAudioPropertiesSheetAssetBase {}
#[repr(C, align(8))]
pub struct USpatializationPluginSourceSettingsBase {
    __padding_end: [u8; 48],
}
impl USpatializationPluginSourceSettingsBase {}
#[repr(C, align(8))]
pub struct USourceDataOverridePluginSourceSettingsBase {
    __padding_end: [u8; 48],
}
impl USourceDataOverridePluginSourceSettingsBase {}
#[repr(C, align(8))]
pub struct UOcclusionPluginSourceSettingsBase {
    __padding_end: [u8; 48],
}
impl UOcclusionPluginSourceSettingsBase {}
#[repr(C, align(8))]
pub struct UReverbPluginSourceSettingsBase {
    __padding_end: [u8; 48],
}
impl UReverbPluginSourceSettingsBase {}
pub struct IAudioParameterControllerInterface {}
#[repr(C, align(8))]
pub struct UAudioParameterControllerInterface {
    __padding_end: [u8; 48],
}
impl UAudioParameterControllerInterface {}
#[repr(C, align(8))]
pub struct UAudioEndpointSettingsBase {
    __padding_end: [u8; 48],
}
impl UAudioEndpointSettingsBase {}
#[repr(C, align(8))]
pub struct UDummyEndpointSettings {
    __padding_end: [u8; 48],
}
impl UDummyEndpointSettings {}
#[repr(C, align(8))]
pub struct USoundModulatorBase {
    __padding_end: [u8; 56],
}
impl USoundModulatorBase {}
#[repr(C, align(8))]
pub struct USoundfieldEndpointSettingsBase {
    __padding_end: [u8; 48],
}
impl USoundfieldEndpointSettingsBase {}
#[repr(C, align(8))]
pub struct USoundfieldEncodingSettingsBase {
    __padding_end: [u8; 48],
}
impl USoundfieldEncodingSettingsBase {}
#[repr(C, align(8))]
pub struct USoundfieldEffectSettingsBase {
    __padding_end: [u8; 48],
}
impl USoundfieldEffectSettingsBase {}
#[repr(C, align(8))]
pub struct USoundfieldEffectBase {
    __padding_end: [u8; 56],
}
impl USoundfieldEffectBase {}
#[repr(C, align(8))]
pub struct UWaveformTransformationBase {
    __padding_end: [u8; 96],
}
impl UWaveformTransformationBase {}
#[repr(C, align(8))]
pub struct UWaveformTransformationChain {
    __padding_end: [u8; 64],
}
impl UWaveformTransformationChain {}
#[repr(transparent)]
pub struct ESoundWaveCloudStreamingPlatformProjectEnableType(pub u8);
impl ESoundWaveCloudStreamingPlatformProjectEnableType {
    pub const ENABLED: ESoundWaveCloudStreamingPlatformProjectEnableType = ESoundWaveCloudStreamingPlatformProjectEnableType(
        0,
    );
    pub const DISABLED: ESoundWaveCloudStreamingPlatformProjectEnableType = ESoundWaveCloudStreamingPlatformProjectEnableType(
        1,
    );
}
#[repr(transparent)]
pub struct ESoundWaveCloudStreamingPlatformEnableType(pub u8);
impl ESoundWaveCloudStreamingPlatformEnableType {
    pub const INHERITED: ESoundWaveCloudStreamingPlatformEnableType = ESoundWaveCloudStreamingPlatformEnableType(
        0,
    );
    pub const DISABLED: ESoundWaveCloudStreamingPlatformEnableType = ESoundWaveCloudStreamingPlatformEnableType(
        1,
    );
    pub const SWC_MULTIPLE_VALUES: ESoundWaveCloudStreamingPlatformEnableType = ESoundWaveCloudStreamingPlatformEnableType(
        2,
    );
}
#[repr(transparent)]
pub struct EAudioParameterType(pub u8);
impl EAudioParameterType {
    pub const NONE: EAudioParameterType = EAudioParameterType(0);
    pub const BOOLEAN: EAudioParameterType = EAudioParameterType(1);
    pub const INTEGER: EAudioParameterType = EAudioParameterType(2);
    pub const FLOAT: EAudioParameterType = EAudioParameterType(3);
    pub const STRING: EAudioParameterType = EAudioParameterType(4);
    pub const OBJECT: EAudioParameterType = EAudioParameterType(5);
    pub const NONE_ARRAY: EAudioParameterType = EAudioParameterType(6);
    pub const BOOLEAN_ARRAY: EAudioParameterType = EAudioParameterType(7);
    pub const INTEGER_ARRAY: EAudioParameterType = EAudioParameterType(8);
    pub const FLOAT_ARRAY: EAudioParameterType = EAudioParameterType(9);
    pub const STRING_ARRAY: EAudioParameterType = EAudioParameterType(10);
    pub const OBJECT_ARRAY: EAudioParameterType = EAudioParameterType(11);
    pub const TRIGGER: EAudioParameterType = EAudioParameterType(12);
    pub const COUNT: EAudioParameterType = EAudioParameterType(13);
}
