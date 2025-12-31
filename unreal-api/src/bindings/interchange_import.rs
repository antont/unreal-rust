#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
    pub components_to_restart: TArray<UPtr<crate::bindings::engine::UAudioComponent>>,
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
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub c: crate::bindings::engine::FExpressionInput,
}
pub struct UMaterialExpressionMaterialXAppend4Vector {
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub c: crate::bindings::engine::FExpressionInput,
    pub d: crate::bindings::engine::FExpressionInput,
}
pub struct UMaterialExpressionMaterialXBurn {
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub alpha: crate::bindings::engine::FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXContrast {
    pub input: crate::bindings::engine::FExpressionInput,
    pub amount: crate::bindings::engine::FExpressionInput,
    pub pivot: crate::bindings::engine::FExpressionInput,
    pub const_amount: f32,
    pub const_pivot: f32,
}
pub struct UMaterialExpressionMaterialXDifference {
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub alpha: crate::bindings::engine::FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXDisjointOver {
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub alpha: crate::bindings::engine::FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXDodge {
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub alpha: crate::bindings::engine::FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXFractal3D {
    pub position: crate::bindings::engine::FExpressionInput,
    pub amplitude: crate::bindings::engine::FExpressionInput,
    pub const_amplitude: f32,
    pub octaves: crate::bindings::engine::FExpressionInput,
    pub const_octaves: i32,
    pub lacunarity: crate::bindings::engine::FExpressionInput,
    pub const_lacunarity: f32,
    pub diminish: crate::bindings::engine::FExpressionInput,
    pub const_diminish: f32,
    pub scale_deprecated: f32,
    pub b_turbulence_deprecated: bool,
    pub levels_deprecated: i32,
    pub output_min_deprecated: f32,
    pub output_max_deprecated: f32,
}
pub struct UMaterialExpressionMaterialXIn {
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub alpha: crate::bindings::engine::FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXLuminance {
    pub input: crate::bindings::engine::FExpressionInput,
    pub luminance_factors: crate::bindings::core_u_object::FLinearColor,
    pub luminance_mode: EMaterialXLuminanceMode,
}
pub struct UMaterialExpressionMaterialXMask {
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub alpha: crate::bindings::engine::FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXMatte {
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub alpha: crate::bindings::engine::FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXMinus {
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub alpha: crate::bindings::engine::FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXMod {
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub const_b: f32,
}
pub struct UMaterialExpressionMaterialXOut {
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub alpha: crate::bindings::engine::FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXOver {
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub alpha: crate::bindings::engine::FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXOverlay {
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub alpha: crate::bindings::engine::FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXPlace2D {
    pub coordinates: crate::bindings::engine::FExpressionInput,
    pub pivot: crate::bindings::engine::FExpressionInput,
    pub scale: crate::bindings::engine::FExpressionInput,
    pub offset: crate::bindings::engine::FExpressionInput,
    pub rotation_angle: crate::bindings::engine::FExpressionInput,
    pub const_rotation_angle: f32,
    pub const_coordinate: u8,
}
pub struct UMaterialExpressionMaterialXPlus {
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub alpha: crate::bindings::engine::FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXPremult {
    pub input: crate::bindings::engine::FExpressionInput,
}
pub struct UMaterialExpressionMaterialXRamp4 {
    pub coordinates: crate::bindings::engine::FExpressionInput,
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub c: crate::bindings::engine::FExpressionInput,
    pub d: crate::bindings::engine::FExpressionInput,
    pub const_coordinate: u8,
}
pub struct UMaterialExpressionMaterialXRampLeftRight {
    pub coordinates: crate::bindings::engine::FExpressionInput,
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub const_coordinate: u8,
}
pub struct UMaterialExpressionMaterialXRampTopBottom {
    pub coordinates: crate::bindings::engine::FExpressionInput,
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub const_coordinate: u8,
}
pub struct UMaterialExpressionMaterialXRange {
    pub input: crate::bindings::engine::FExpressionInput,
    pub input_low: crate::bindings::engine::FExpressionInput,
    pub input_high: crate::bindings::engine::FExpressionInput,
    pub target_low: crate::bindings::engine::FExpressionInput,
    pub target_high: crate::bindings::engine::FExpressionInput,
    pub gamma: crate::bindings::engine::FExpressionInput,
    pub clamp: crate::bindings::engine::FExpressionInput,
    pub const_input_low: f32,
    pub const_input_high: f32,
    pub const_target_low: f32,
    pub const_target_high: f32,
    pub const_gamma: f32,
    pub b_const_clamp: bool,
}
pub struct UMaterialExpressionMaterialXRemap {
    pub input: crate::bindings::engine::FExpressionInput,
    pub input_low: crate::bindings::engine::FExpressionInput,
    pub input_high: crate::bindings::engine::FExpressionInput,
    pub target_low: crate::bindings::engine::FExpressionInput,
    pub target_high: crate::bindings::engine::FExpressionInput,
    pub input_low_default: f32,
    pub input_high_default: f32,
    pub target_low_default: f32,
    pub target_high_default: f32,
}
pub struct UMaterialExpressionMaterialXRotate2D {
    pub input: crate::bindings::engine::FExpressionInput,
    pub rotation_angle: crate::bindings::engine::FExpressionInput,
    pub const_rotation_angle: f32,
}
pub struct UMaterialExpressionMaterialXScreen {
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub alpha: crate::bindings::engine::FExpressionInput,
    pub const_alpha: f32,
}
pub struct UMaterialExpressionMaterialXSplitLeftRight {
    pub coordinates: crate::bindings::engine::FExpressionInput,
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub center: crate::bindings::engine::FExpressionInput,
    pub const_center: f32,
    pub const_coordinate: u8,
}
pub struct UMaterialExpressionMaterialXSplitTopBottom {
    pub coordinates: crate::bindings::engine::FExpressionInput,
    pub a: crate::bindings::engine::FExpressionInput,
    pub b: crate::bindings::engine::FExpressionInput,
    pub center: crate::bindings::engine::FExpressionInput,
    pub const_center: f32,
    pub const_coordinate: u8,
}
pub struct UMaterialExpressionMaterialXSwizzle {
    pub input: crate::bindings::engine::FExpressionInput,
    pub channels: FString,
}
pub struct UMaterialExpressionMaterialXTextureSampleParameterBlur {
    pub kernel_size: EMAterialXTextureSampleBlurKernel,
    pub filter_size: f32,
    pub filter_offset: f32,
    pub filter: EMaterialXTextureSampleBlurFilter,
}
pub struct UMaterialExpressionMaterialXUnpremult {
    pub input: crate::bindings::engine::FExpressionInput,
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInterchangeCoordinateSystemPolicy(pub u8);
impl EInterchangeCoordinateSystemPolicy {
    pub const MATCH_UP_FORWARD_AXES: EInterchangeCoordinateSystemPolicy = EInterchangeCoordinateSystemPolicy(
        0,
    );
    pub const MATCH_UP_AXIS: EInterchangeCoordinateSystemPolicy = EInterchangeCoordinateSystemPolicy(
        1,
    );
    pub const KEEP_XYZ_AXES: EInterchangeCoordinateSystemPolicy = EInterchangeCoordinateSystemPolicy(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMaterialXLuminanceMode(pub u8);
impl EMaterialXLuminanceMode {
    pub const ACE_SCG: EMaterialXLuminanceMode = EMaterialXLuminanceMode(0);
    pub const REC709: EMaterialXLuminanceMode = EMaterialXLuminanceMode(1);
    pub const REC2020: EMaterialXLuminanceMode = EMaterialXLuminanceMode(2);
    pub const REC2100: EMaterialXLuminanceMode = EMaterialXLuminanceMode(2);
    pub const CUSTOM: EMaterialXLuminanceMode = EMaterialXLuminanceMode(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMAterialXTextureSampleBlurKernel(pub i32);
impl EMAterialXTextureSampleBlurKernel {
    pub const KERNEL1: EMAterialXTextureSampleBlurKernel = EMAterialXTextureSampleBlurKernel(
        0,
    );
    pub const KERNEL3: EMAterialXTextureSampleBlurKernel = EMAterialXTextureSampleBlurKernel(
        1,
    );
    pub const KERNEL5: EMAterialXTextureSampleBlurKernel = EMAterialXTextureSampleBlurKernel(
        2,
    );
    pub const KERNEL7: EMAterialXTextureSampleBlurKernel = EMAterialXTextureSampleBlurKernel(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMaterialXTextureSampleBlurFilter(pub u8);
impl EMaterialXTextureSampleBlurFilter {
    pub const BOX: EMaterialXTextureSampleBlurFilter = EMaterialXTextureSampleBlurFilter(
        0,
    );
    pub const GAUSSIAN: EMaterialXTextureSampleBlurFilter = EMaterialXTextureSampleBlurFilter(
        1,
    );
}
