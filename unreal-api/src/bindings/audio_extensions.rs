#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FSoundWaveCloudStreamingPlatformProjectSettings {
    pub enablement_setting: ESoundWaveCloudStreamingPlatformProjectEnableType,
}
#[repr(C, align(1))]
pub struct FSoundWaveCloudStreamingPlatformSettings {
    pub enablement_setting: ESoundWaveCloudStreamingPlatformEnableType,
}
#[repr(C, align(4))]
pub struct FSoundGeneratorOutput {
    pub name: FName,
}
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
    pub type_name: FName,
}
#[repr(C, align(8))]
pub struct FSoundWaveCuePoint {
    pub cue_point_id: i32,
    pub label: FString,
    pub frame_position: i64,
    pub frame_length: i64,
    pub b_is_loop_region: bool,
}
pub struct UAudioPropertiesSheetAssetUserInterface {}
pub struct IAudioPropertiesSheetAssetUserInterface {}
pub struct UAudioPropertiesSheetAssetBase {}
pub struct USpatializationPluginSourceSettingsBase {}
pub struct USourceDataOverridePluginSourceSettingsBase {}
pub struct UOcclusionPluginSourceSettingsBase {}
pub struct UReverbPluginSourceSettingsBase {}
pub struct UAudioParameterControllerInterface {}
pub struct IAudioParameterControllerInterface {}
pub struct UAudioEndpointSettingsBase {}
pub struct UDummyEndpointSettings {}
pub struct USoundModulatorBase {}
pub struct USoundfieldEndpointSettingsBase {}
pub struct USoundfieldEncodingSettingsBase {}
pub struct USoundfieldEffectSettingsBase {}
pub struct USoundfieldEffectBase {
    pub settings: UPtr<USoundfieldEffectSettingsBase>,
}
pub struct UWaveformTransformationBase {}
pub struct UWaveformTransformationChain {
    pub transformations: TArray<UPtr<UWaveformTransformationBase>>,
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
