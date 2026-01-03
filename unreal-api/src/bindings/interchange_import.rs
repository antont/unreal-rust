#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct IInterchangeAnimationPayloadInterface {}
#[repr(C, align(8))]
pub struct UInterchangeAnimationPayloadInterface {
    __padding_end: [u8; 48],
}
impl UInterchangeAnimationPayloadInterface {}
#[repr(C, align(8))]
pub struct UInterchangeAssetUserData {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub meta_data: TMap<FString, FString>,
}
impl UInterchangeAssetUserData {}
#[repr(C, align(8))]
pub struct UInterchangeLevelAssetUserData {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub scene_import_paths: TArray<FString>,
}
impl UInterchangeLevelAssetUserData {}
pub struct IInterchangeAudioPayloadInterface {}
#[repr(C, align(8))]
pub struct UInterchangeAudioPayloadInterface {
    __padding_end: [u8; 48],
}
impl UInterchangeAudioPayloadInterface {}
pub struct IInterchangeBlockedTexturePayloadInterface {}
#[repr(C, align(8))]
pub struct UInterchangeBlockedTexturePayloadInterface {
    __padding_end: [u8; 48],
}
impl UInterchangeBlockedTexturePayloadInterface {}
pub struct IInterchangeGroomPayloadInterface {}
#[repr(C, align(8))]
pub struct UInterchangeGroomPayloadInterface {
    __padding_end: [u8; 48],
}
impl UInterchangeGroomPayloadInterface {}
pub struct IInterchangeMeshPayloadInterface {}
#[repr(C, align(8))]
pub struct UInterchangeMeshPayloadInterface {
    __padding_end: [u8; 48],
}
impl UInterchangeMeshPayloadInterface {}
pub struct IInterchangeSlicedTexturePayloadInterface {}
#[repr(C, align(8))]
pub struct UInterchangeSlicedTexturePayloadInterface {
    __padding_end: [u8; 48],
}
impl UInterchangeSlicedTexturePayloadInterface {}
pub struct IInterchangeTextureLightProfilePayloadInterface {}
#[repr(C, align(8))]
pub struct UInterchangeTextureLightProfilePayloadInterface {
    __padding_end: [u8; 48],
}
impl UInterchangeTextureLightProfilePayloadInterface {}
pub struct IInterchangeTexturePayloadInterface {}
#[repr(C, align(8))]
pub struct UInterchangeTexturePayloadInterface {
    __padding_end: [u8; 48],
}
impl UInterchangeTexturePayloadInterface {}
pub struct IInterchangeVariantSetPayloadInterface {}
#[repr(C, align(8))]
pub struct UInterchangeVariantSetPayloadInterface {
    __padding_end: [u8; 48],
}
impl UInterchangeVariantSetPayloadInterface {}
pub struct IInterchangeVolumePayloadInterface {}
#[repr(C, align(8))]
pub struct UInterchangeVolumePayloadInterface {
    __padding_end: [u8; 48],
}
impl UInterchangeVolumePayloadInterface {}
#[repr(C, align(8))]
pub struct UInterchangeVolumeTranslatorSettings {
    __padding_end: [u8; 72],
}
impl UInterchangeVolumeTranslatorSettings {}
#[repr(C, align(8))]
pub struct UInterchangeAnimSequenceFactory {
    __padding_end: [u8; 480],
}
impl UInterchangeAnimSequenceFactory {}
#[repr(C, align(8))]
pub struct UInterchangeLevelSequenceFactory {
    __padding_end: [u8; 64],
}
impl UInterchangeLevelSequenceFactory {}
#[repr(C, align(8))]
pub struct UInterchangeAudioTranslatorBase {
    __padding_end: [u8; 88],
}
impl UInterchangeAudioTranslatorBase {}
#[repr(C, align(8))]
pub struct UInterchangeAudioTranslator_AIF {
    __padding_end: [u8; 88],
}
impl UInterchangeAudioTranslator_AIF {}
#[repr(C, align(8))]
pub struct UInterchangeAudioTranslator_AIFF {
    __padding_end: [u8; 88],
}
impl UInterchangeAudioTranslator_AIFF {}
#[repr(C, align(8))]
pub struct UInterchangeAudioTranslator_FLAC {
    __padding_end: [u8; 88],
}
impl UInterchangeAudioTranslator_FLAC {}
#[repr(C, align(8))]
pub struct UInterchangeAudioTranslator_MP3 {
    __padding_end: [u8; 88],
}
impl UInterchangeAudioTranslator_MP3 {}
#[repr(C, align(8))]
pub struct UInterchangeAudioTranslator_OGG {
    __padding_end: [u8; 88],
}
impl UInterchangeAudioTranslator_OGG {}
#[repr(C, align(8))]
pub struct UInterchangeAudioTranslator_OPUS {
    __padding_end: [u8; 88],
}
impl UInterchangeAudioTranslator_OPUS {}
#[repr(C, align(8))]
pub struct UInterchangeAudioTranslator_WAV {
    __padding_end: [u8; 88],
}
impl UInterchangeAudioTranslator_WAV {}
#[repr(C, align(8))]
pub struct UInterchangeAudioSoundWaveFactory {
    __padding_end: [u8; 72],
}
impl UInterchangeAudioSoundWaveFactory {}
#[repr(C, align(8))]
pub struct UInterchangeFbxTranslatorSettings {
    __padding_end: [u8; 56],
}
impl UInterchangeFbxTranslatorSettings {}
#[repr(C, align(8))]
pub struct UInterchangeFbxTranslator {
    __padding_end: [u8; 392],
}
impl UInterchangeFbxTranslator {}
#[repr(C, align(8))]
pub struct UInterchangeGLTFTranslator {
    __padding_end: [u8; 704],
}
impl UInterchangeGLTFTranslator {}
#[repr(C, align(8))]
pub struct UInterchangeGroomCacheFactory {
    __padding_end: [u8; 56],
}
impl UInterchangeGroomCacheFactory {}
#[repr(C, align(8))]
pub struct UInterchangeGroomFactory {
    __padding_end: [u8; 336],
}
impl UInterchangeGroomFactory {}
#[repr(C, align(8))]
pub struct UInterchangeMaterialXTranslator {
    __padding_end: [u8; 96],
}
impl UInterchangeMaterialXTranslator {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXAppend3Vector {
    __padding_end: [u8; 344],
}
impl UMaterialExpressionMaterialXAppend3Vector {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXAppend4Vector {
    __padding_end: [u8; 392],
}
impl UMaterialExpressionMaterialXAppend4Vector {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXBurn {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionMaterialXBurn {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXContrast {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionMaterialXContrast {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXDifference {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionMaterialXDifference {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXDisjointOver {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionMaterialXDisjointOver {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXDodge {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionMaterialXDodge {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXFractal3D {
    __padding_end: [u8; 488],
}
impl UMaterialExpressionMaterialXFractal3D {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXIn {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionMaterialXIn {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXLuminance {
    __padding_end: [u8; 272],
}
impl UMaterialExpressionMaterialXLuminance {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXMask {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionMaterialXMask {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXMatte {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionMaterialXMatte {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXMinus {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionMaterialXMinus {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXMod {
    __padding_end: [u8; 304],
}
impl UMaterialExpressionMaterialXMod {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXOut {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionMaterialXOut {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXOver {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionMaterialXOver {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXOverlay {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionMaterialXOverlay {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXPlace2D {
    __padding_end: [u8; 448],
}
impl UMaterialExpressionMaterialXPlace2D {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXPlus {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionMaterialXPlus {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXPremult {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionMaterialXPremult {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXRamp4 {
    __padding_end: [u8; 448],
}
impl UMaterialExpressionMaterialXRamp4 {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXRampLeftRight {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionMaterialXRampLeftRight {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXRampTopBottom {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionMaterialXRampTopBottom {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXRange {
    __padding_end: [u8; 560],
}
impl UMaterialExpressionMaterialXRange {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXRemap {
    __padding_end: [u8; 456],
}
impl UMaterialExpressionMaterialXRemap {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXRotate2D {
    __padding_end: [u8; 304],
}
impl UMaterialExpressionMaterialXRotate2D {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXScreen {
    __padding_end: [u8; 352],
}
impl UMaterialExpressionMaterialXScreen {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXSplitLeftRight {
    __padding_end: [u8; 400],
}
impl UMaterialExpressionMaterialXSplitLeftRight {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXSplitTopBottom {
    __padding_end: [u8; 400],
}
impl UMaterialExpressionMaterialXSplitTopBottom {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXSwizzle {
    __padding_end: [u8; 264],
}
impl UMaterialExpressionMaterialXSwizzle {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXTextureSampleParameterBlur {
    __padding_end: [u8; 648],
}
impl UMaterialExpressionMaterialXTextureSampleParameterBlur {}
#[repr(C, align(8))]
pub struct UMaterialExpressionMaterialXUnpremult {
    __padding_end: [u8; 248],
}
impl UMaterialExpressionMaterialXUnpremult {}
#[repr(C, align(8))]
pub struct UInterchangeMaterialFactory {
    __padding_end: [u8; 64],
}
impl UInterchangeMaterialFactory {}
#[repr(C, align(8))]
pub struct UInterchangeMaterialFunctionFactory {
    __padding_end: [u8; 64],
}
impl UInterchangeMaterialFunctionFactory {}
#[repr(C, align(8))]
pub struct UInterchangeGeometryCacheFactory {
    __padding_end: [u8; 160],
}
impl UInterchangeGeometryCacheFactory {}
#[repr(C, align(8))]
pub struct UInterchangeOBJTranslator {
    __padding_end: [u8; 104],
}
impl UInterchangeOBJTranslator {}
#[repr(C, align(8))]
pub struct UInterchangePhysicsAssetFactory {
    __padding_end: [u8; 56],
}
impl UInterchangePhysicsAssetFactory {}
#[repr(C, align(8))]
pub struct UInterchangeSkeletalMeshFactory {
    __padding_end: [u8; 248],
}
impl UInterchangeSkeletalMeshFactory {}
#[repr(C, align(8))]
pub struct UInterchangeSkeletonFactory {
    __padding_end: [u8; 56],
}
impl UInterchangeSkeletonFactory {}
#[repr(C, align(8))]
pub struct UInterchangeStaticMeshFactory {
    __padding_end: [u8; 328],
}
impl UInterchangeStaticMeshFactory {}
#[repr(C, align(8))]
pub struct UInterchangeActorFactory {
    __padding_end: [u8; 56],
}
impl UInterchangeActorFactory {}
#[repr(C, align(8))]
pub struct UInterchangeCineCameraActorFactory {
    __padding_end: [u8; 56],
}
impl UInterchangeCineCameraActorFactory {}
#[repr(C, align(8))]
pub struct UInterchangeCameraActorFactory {
    __padding_end: [u8; 56],
}
impl UInterchangeCameraActorFactory {}
#[repr(C, align(8))]
pub struct UInterchangeDecalActorFactory {
    __padding_end: [u8; 56],
}
impl UInterchangeDecalActorFactory {}
#[repr(C, align(8))]
pub struct UInterchangeHeterogeneousVolumeActorFactory {
    __padding_end: [u8; 56],
}
impl UInterchangeHeterogeneousVolumeActorFactory {}
#[repr(C, align(8))]
pub struct UInterchangeLevelFactory {
    __padding_end: [u8; 56],
}
impl UInterchangeLevelFactory {}
#[repr(C, align(8))]
pub struct UInterchangeLevelInstanceActorFactory {
    __padding_end: [u8; 56],
}
impl UInterchangeLevelInstanceActorFactory {}
#[repr(C, align(8))]
pub struct UInterchangeLightActorFactory {
    __padding_end: [u8; 56],
}
impl UInterchangeLightActorFactory {}
#[repr(C, align(8))]
pub struct UInterchangeSceneImportAssetFactory {
    __padding_end: [u8; 56],
}
impl UInterchangeSceneImportAssetFactory {}
#[repr(C, align(8))]
pub struct UInterchangeSceneVariantSetsFactory {
    __padding_end: [u8; 64],
}
impl UInterchangeSceneVariantSetsFactory {}
#[repr(C, align(8))]
pub struct UInterchangeSkeletalMeshActorFactory {
    __padding_end: [u8; 56],
}
impl UInterchangeSkeletalMeshActorFactory {}
#[repr(C, align(8))]
pub struct UInterchangeSkyLightActorFactory {
    __padding_end: [u8; 56],
}
impl UInterchangeSkyLightActorFactory {}
#[repr(C, align(8))]
pub struct UInterchangeStaticMeshActorFactory {
    __padding_end: [u8; 56],
}
impl UInterchangeStaticMeshActorFactory {}
#[repr(C, align(8))]
pub struct UInterchangeSpecularProfileFactory {
    __padding_end: [u8; 56],
}
impl UInterchangeSpecularProfileFactory {}
#[repr(C, align(8))]
pub struct UInterchangeDDSTranslator {
    __padding_end: [u8; 96],
}
impl UInterchangeDDSTranslator {}
#[repr(C, align(8))]
pub struct UInterchangeIESTranslator {
    __padding_end: [u8; 88],
}
impl UInterchangeIESTranslator {}
#[repr(C, align(8))]
pub struct UInterchangeImageWrapperTranslator {
    __padding_end: [u8; 96],
}
impl UInterchangeImageWrapperTranslator {}
#[repr(C, align(8))]
pub struct UInterchangeJPGTranslator {
    __padding_end: [u8; 88],
}
impl UInterchangeJPGTranslator {}
#[repr(C, align(8))]
pub struct UInterchangePSDTranslator {
    __padding_end: [u8; 88],
}
impl UInterchangePSDTranslator {}
#[repr(C, align(8))]
pub struct UInterchangeTextureFactory {
    __padding_end: [u8; 288],
}
impl UInterchangeTextureFactory {}
#[repr(C, align(8))]
pub struct UInterchangeUEJPEGTranslator {
    __padding_end: [u8; 88],
}
impl UInterchangeUEJPEGTranslator {}
#[repr(C, align(8))]
pub struct UInterchangeSparseVolumeTextureFactory {
    __padding_end: [u8; 96],
}
impl UInterchangeSparseVolumeTextureFactory {}
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
#[repr(transparent)]
pub struct EMaterialXLuminanceMode(pub u8);
impl EMaterialXLuminanceMode {
    pub const ACE_SCG: EMaterialXLuminanceMode = EMaterialXLuminanceMode(0);
    pub const REC709: EMaterialXLuminanceMode = EMaterialXLuminanceMode(1);
    pub const REC2020: EMaterialXLuminanceMode = EMaterialXLuminanceMode(2);
    pub const REC2100: EMaterialXLuminanceMode = EMaterialXLuminanceMode(2);
    pub const CUSTOM: EMaterialXLuminanceMode = EMaterialXLuminanceMode(3);
}
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
