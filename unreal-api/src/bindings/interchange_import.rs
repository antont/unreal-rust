#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UInterchangeAnimationPayloadInterface {}
pub struct IInterchangeAnimationPayloadInterface {}
pub struct UInterchangeAssetUserData {
    pub meta_data: TMap<FString, FString>,
}
pub struct UInterchangeLevelAssetUserData {
    pub scene_import_paths: TArray<FString>,
}
pub struct UInterchangeAudioPayloadInterface {}
pub struct IInterchangeAudioPayloadInterface {}
pub struct UInterchangeBlockedTexturePayloadInterface {}
pub struct IInterchangeBlockedTexturePayloadInterface {}
pub struct UInterchangeGroomPayloadInterface {}
pub struct IInterchangeGroomPayloadInterface {}
pub struct UInterchangeMeshPayloadInterface {}
pub struct IInterchangeMeshPayloadInterface {}
pub struct UInterchangeSlicedTexturePayloadInterface {}
pub struct IInterchangeSlicedTexturePayloadInterface {}
pub struct UInterchangeTextureLightProfilePayloadInterface {}
pub struct IInterchangeTextureLightProfilePayloadInterface {}
pub struct UInterchangeTexturePayloadInterface {}
pub struct IInterchangeTexturePayloadInterface {}
pub struct UInterchangeVariantSetPayloadInterface {}
pub struct IInterchangeVariantSetPayloadInterface {}
pub struct UInterchangeVolumePayloadInterface {}
pub struct IInterchangeVolumePayloadInterface {}
pub struct UInterchangeVolumeTranslatorSettings {
    pub b_translate_adjacent_numbered_files: bool,
    pub animation_id: FString,
}
pub struct UInterchangeAnimSequenceFactory {}
pub struct UInterchangeLevelSequenceFactory {}
pub struct UInterchangeAudioTranslatorBase {}
pub struct UInterchangeAudioTranslator_AIF {}
pub struct UInterchangeAudioTranslator_AIFF {}
pub struct UInterchangeAudioTranslator_FLAC {}
pub struct UInterchangeAudioTranslator_MP3 {}
pub struct UInterchangeAudioTranslator_OGG {}
pub struct UInterchangeAudioTranslator_OPUS {}
pub struct UInterchangeAudioTranslator_WAV {}
pub struct UInterchangeAudioSoundWaveFactory {
    pub components_to_restart: TArray<UPtr<UAudioComponent>>,
}
pub struct UInterchangeFbxTranslatorSettings {
    pub coordinate_system_policy: EInterchangeCoordinateSystemPolicy,
    pub b_convert_scene: bool,
    pub b_force_front_x_axis: bool,
    pub b_convert_scene_unit: bool,
    pub b_keep_fbx_namespace: bool,
    pub b_use_ufbx_parser: bool,
    pub b_using_luf_coordinate_system: bool,
    pub b_display_ufbx_parser: bool,
}
pub struct UInterchangeFbxTranslator {
    pub cache_fbx_translator_settings: UPtr<UInterchangeFbxTranslatorSettings>,
}
pub struct UInterchangeGLTFTranslator {}
pub struct UInterchangeGroomCacheFactory {}
pub struct UInterchangeGroomFactory {}
pub struct UInterchangeMaterialXTranslator {}
pub struct UMaterialExpressionMaterialXAppend3Vector {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub c: FExpressionInput,
}
pub struct UMaterialExpressionMaterialXAppend4Vector {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub c: FExpressionInput,
    pub d: FExpressionInput,
}
pub struct UMaterialExpressionMaterialXBurn {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub alpha: FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXContrast {
    pub input: FExpressionInput,
    pub amount: FExpressionInput,
    pub pivot: FExpressionInput,
    pub const_amount: f32,
    pub const_pivot: f32,
}
pub struct UMaterialExpressionMaterialXDifference {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub alpha: FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXDisjointOver {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub alpha: FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXDodge {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub alpha: FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXFractal3D {
    pub position: FExpressionInput,
    pub amplitude: FExpressionInput,
    pub const_amplitude: f32,
    pub octaves: FExpressionInput,
    pub const_octaves: i32,
    pub lacunarity: FExpressionInput,
    pub const_lacunarity: f32,
    pub diminish: FExpressionInput,
    pub const_diminish: f32,
    pub scale_deprecated: f32,
    pub b_turbulence_deprecated: bool,
    pub levels_deprecated: i32,
    pub output_min_deprecated: f32,
    pub output_max_deprecated: f32,
}
pub struct UMaterialExpressionMaterialXIn {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub alpha: FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXLuminance {
    pub input: FExpressionInput,
    pub luminance_factors: FLinearColor,
    pub luminance_mode: EMaterialXLuminanceMode,
}
pub struct UMaterialExpressionMaterialXMask {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub alpha: FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXMatte {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub alpha: FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXMinus {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub alpha: FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXMod {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub const_b: f32,
}
pub struct UMaterialExpressionMaterialXOut {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub alpha: FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXOver {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub alpha: FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXOverlay {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub alpha: FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXPlace2D {
    pub coordinates: FExpressionInput,
    pub pivot: FExpressionInput,
    pub scale: FExpressionInput,
    pub offset: FExpressionInput,
    pub rotation_angle: FExpressionInput,
    pub const_rotation_angle: f32,
    pub const_coordinate: u8,
}
pub struct UMaterialExpressionMaterialXPlus {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub alpha: FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXPremult {
    pub input: FExpressionInput,
}
pub struct UMaterialExpressionMaterialXRamp4 {
    pub coordinates: FExpressionInput,
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub c: FExpressionInput,
    pub d: FExpressionInput,
    pub const_coordinate: u8,
}
pub struct UMaterialExpressionMaterialXRampLeftRight {
    pub coordinates: FExpressionInput,
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub const_coordinate: u8,
}
pub struct UMaterialExpressionMaterialXRampTopBottom {
    pub coordinates: FExpressionInput,
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub const_coordinate: u8,
}
pub struct UMaterialExpressionMaterialXRange {
    pub input: FExpressionInput,
    pub input_low: FExpressionInput,
    pub input_high: FExpressionInput,
    pub target_low: FExpressionInput,
    pub target_high: FExpressionInput,
    pub gamma: FExpressionInput,
    pub clamp: FExpressionInput,
    pub const_input_low: f32,
    pub const_input_high: f32,
    pub const_target_low: f32,
    pub const_target_high: f32,
    pub const_gamma: f32,
    pub b_const_clamp: bool,
}
pub struct UMaterialExpressionMaterialXRemap {
    pub input: FExpressionInput,
    pub input_low: FExpressionInput,
    pub input_high: FExpressionInput,
    pub target_low: FExpressionInput,
    pub target_high: FExpressionInput,
    pub input_low_default: f32,
    pub input_high_default: f32,
    pub target_low_default: f32,
    pub target_high_default: f32,
}
pub struct UMaterialExpressionMaterialXRotate2D {
    pub input: FExpressionInput,
    pub rotation_angle: FExpressionInput,
    pub const_rotation_angle: f32,
}
pub struct UMaterialExpressionMaterialXScreen {
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub alpha: FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXSplitLeftRight {
    pub coordinates: FExpressionInput,
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub center: FExpressionInput,
    pub const_center: f32,
    pub const_coordinate: u8,
}
pub struct UMaterialExpressionMaterialXSplitTopBottom {
    pub coordinates: FExpressionInput,
    pub a: FExpressionInput,
    pub b: FExpressionInput,
    pub center: FExpressionInput,
    pub const_center: f32,
    pub const_coordinate: u8,
}
pub struct UMaterialExpressionMaterialXSwizzle {
    pub input: FExpressionInput,
    pub channels: FString,
}
pub struct UMaterialExpressionMaterialXTextureSampleParameterBlur {
    pub kernel_size: EMAterialXTextureSampleBlurKernel,
    pub filter_size: f32,
    pub filter_offset: f32,
    pub filter: EMaterialXTextureSampleBlurFilter,
}
pub struct UMaterialExpressionMaterialXUnpremult {
    pub input: FExpressionInput,
}
pub struct UInterchangeMaterialFactory {}
pub struct UInterchangeMaterialFunctionFactory {}
pub struct UInterchangeGeometryCacheFactory {}
pub struct UInterchangeOBJTranslator {}
pub struct UInterchangePhysicsAssetFactory {}
pub struct UInterchangeSkeletalMeshFactory {}
pub struct UInterchangeSkeletonFactory {}
pub struct UInterchangeStaticMeshFactory {}
pub struct UInterchangeActorFactory {}
pub struct UInterchangeCineCameraActorFactory {}
pub struct UInterchangeCameraActorFactory {}
pub struct UInterchangeDecalActorFactory {}
pub struct UInterchangeHeterogeneousVolumeActorFactory {}
pub struct UInterchangeLevelFactory {}
pub struct UInterchangeLevelInstanceActorFactory {}
pub struct UInterchangeLightActorFactory {}
pub struct UInterchangeSceneImportAssetFactory {}
pub struct UInterchangeSceneVariantSetsFactory {}
pub struct UInterchangeSkeletalMeshActorFactory {}
pub struct UInterchangeSkyLightActorFactory {}
pub struct UInterchangeStaticMeshActorFactory {}
pub struct UInterchangeSpecularProfileFactory {}
pub struct UInterchangeDDSTranslator {}
pub struct UInterchangeIESTranslator {}
pub struct UInterchangeImageWrapperTranslator {}
pub struct UInterchangeJPGTranslator {}
pub struct UInterchangePSDTranslator {}
pub struct UInterchangeTextureFactory {}
pub struct UInterchangeUEJPEGTranslator {}
pub struct UInterchangeSparseVolumeTextureFactory {}
