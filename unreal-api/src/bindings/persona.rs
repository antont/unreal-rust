#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAnimationEditorsAssetFamilyExtension {
    __padding_end: [u8; 48],
}
impl UAnimationEditorsAssetFamilyExtension {}
#[repr(C, align(8))]
pub struct UAnimationSequenceBrowserContextMenuContext {
    __padding_end: [u8; 80],
}
impl UAnimationSequenceBrowserContextMenuContext {}
#[repr(C, align(8))]
pub struct UAnimNotifyPanelContextMenuContext {
    __padding_end: [u8; 224],
}
impl UAnimNotifyPanelContextMenuContext {}
#[repr(C, align(8))]
pub struct UAnimViewportContext {
    __padding_end: [u8; 80],
}
impl UAnimViewportContext {}
#[repr(C, align(8))]
pub struct UAnimViewportToolBarToolMenuContext {
    __padding_end: [u8; 64],
}
impl UAnimViewportToolBarToolMenuContext {}
#[repr(C, align(16))]
pub struct UCachedAnalysisProperties {
    __padding_end: [u8; 576],
}
impl UCachedAnalysisProperties {}
#[repr(C, align(16))]
pub struct ULinearAnalysisPropertiesBase {
    __padding_end: [u8; 416],
}
impl ULinearAnalysisPropertiesBase {}
#[repr(C, align(16))]
pub struct ULinearAnalysisProperties {
    __padding_end: [u8; 416],
}
impl ULinearAnalysisProperties {}
#[repr(C, align(16))]
pub struct UEulerAnalysisProperties {
    __padding_end: [u8; 432],
}
impl UEulerAnalysisProperties {}
#[repr(C, align(8))]
pub struct UPersonaPreviewSceneDescription {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub preview_controller: TSubclassOf<UPersonaPreviewSceneController>,
    __padding_end: [u8; 192],
}
impl UPersonaPreviewSceneDescription {}
#[repr(C, align(8))]
pub struct UAnimAssetFindReplaceContext {
    __padding_end: [u8; 64],
}
impl UAnimAssetFindReplaceContext {}
#[repr(C, align(8))]
pub struct UAnimAssetFindReplaceProcessor {
    __padding_end: [u8; 64],
}
impl UAnimAssetFindReplaceProcessor {}
#[repr(C, align(8))]
pub struct UAnimAssetFindReplaceProcessor_StringBase {
    __padding_end: [u8; 320],
}
impl UAnimAssetFindReplaceProcessor_StringBase {}
#[repr(C, align(8))]
pub struct UAnimAssetFindReplaceCurves {
    __padding_end: [u8; 328],
}
impl UAnimAssetFindReplaceCurves {}
#[repr(C, align(8))]
pub struct UAnimAssetFindReplaceNotifies {
    __padding_end: [u8; 320],
}
impl UAnimAssetFindReplaceNotifies {}
#[repr(C, align(8))]
pub struct UAnimAssetFindReplaceSyncMarkers {
    __padding_end: [u8; 320],
}
impl UAnimAssetFindReplaceSyncMarkers {}
#[repr(C, align(8))]
pub struct AAnimationEditorPreviewActor {
    __padding_end: [u8; 1136],
}
impl AAnimationEditorPreviewActor {}
#[repr(C, align(8))]
pub struct UAnimCurveBaseCopyObject {
    __padding_end: [u8; 80],
}
impl UAnimCurveBaseCopyObject {}
#[repr(C, align(8))]
pub struct UFloatCurveCopyObject {
    __padding_end: [u8; 288],
}
impl UFloatCurveCopyObject {}
#[repr(C, align(8))]
pub struct UTransformCurveCopyObject {
    __padding_end: [u8; 1552],
}
impl UTransformCurveCopyObject {}
#[repr(C, align(8))]
pub struct UVectorCurveCopyObject {
    __padding_end: [u8; 544],
}
impl UVectorCurveCopyObject {}
#[repr(C, align(8))]
pub struct UAnimTimelineClipboardContent {
    __padding_end: [u8; 64],
}
impl UAnimTimelineClipboardContent {}
pub struct IPersonaManagerContext {}
#[repr(C, align(8))]
pub struct UPersonaManagerContext {
    __padding_end: [u8; 48],
}
impl UPersonaManagerContext {}
#[repr(C, align(8))]
pub struct UPersonaEditorModeManagerContext {
    __padding_end: [u8; 64],
}
impl UPersonaEditorModeManagerContext {}
#[repr(C, align(8))]
pub struct ULODInfoUILayout {
    __padding_end: [u8; 576],
}
impl ULODInfoUILayout {}
#[repr(C, align(8))]
pub struct UAnimationEditorsAssetFamilyExtension_SkeletonAsset {
    __padding_end: [u8; 48],
}
impl UAnimationEditorsAssetFamilyExtension_SkeletonAsset {}
#[repr(C, align(8))]
pub struct UAnimationEditorsAssetFamilyExtension_SkeletalMeshAsset {
    __padding_end: [u8; 48],
}
impl UAnimationEditorsAssetFamilyExtension_SkeletalMeshAsset {}
#[repr(C, align(8))]
pub struct UAnimationEditorsAssetFamilyExtension_AnimationAsset {
    __padding_end: [u8; 48],
}
impl UAnimationEditorsAssetFamilyExtension_AnimationAsset {}
#[repr(C, align(8))]
pub struct UAnimationEditorsAssetFamilyExtension_AnimBlueprintAsset {
    __padding_end: [u8; 48],
}
impl UAnimationEditorsAssetFamilyExtension_AnimBlueprintAsset {}
#[repr(C, align(8))]
pub struct UAnimationEditorsAssetFamilyExtension_PhysicsAsset {
    __padding_end: [u8; 48],
}
impl UAnimationEditorsAssetFamilyExtension_PhysicsAsset {}
#[repr(C, align(8))]
pub struct UPersonaPreviewSceneController {
    __padding_end: [u8; 48],
}
impl UPersonaPreviewSceneController {}
#[repr(C, align(8))]
pub struct UPersonaPreviewSceneAnimationController {
    __padding_end: [u8; 96],
}
impl UPersonaPreviewSceneAnimationController {}
#[repr(C, align(8))]
pub struct UPersonaPreviewSceneDefaultController {
    __padding_end: [u8; 48],
}
impl UPersonaPreviewSceneDefaultController {}
#[repr(C, align(8))]
pub struct UPersonaPreviewSceneRefPoseController {
    __padding_end: [u8; 56],
}
impl UPersonaPreviewSceneRefPoseController {}
#[repr(C, align(8))]
pub struct UPersonaPreviewSceneSkelMeshInstanceController {
    __padding_end: [u8; 56],
}
impl UPersonaPreviewSceneSkelMeshInstanceController {}
#[repr(C, align(8))]
pub struct UPersonaToolMenuContext {
    __padding_end: [u8; 64],
}
impl UPersonaToolMenuContext {}
#[repr(C, align(8))]
pub struct UPhysicsAssetRenderUtilities {
    __padding_end: [u8; 152],
}
impl UPhysicsAssetRenderUtilities {}
#[repr(C, align(8))]
pub struct USkinWeightImportOptions {
    __padding_end: [u8; 88],
}
impl USkinWeightImportOptions {}
#[repr(transparent)]
pub struct EAnalysisSpace(pub u8);
impl EAnalysisSpace {
    pub const WORLD: EAnalysisSpace = EAnalysisSpace(0);
    pub const FIXED: EAnalysisSpace = EAnalysisSpace(1);
    pub const CHANGING: EAnalysisSpace = EAnalysisSpace(2);
    pub const MOVING: EAnalysisSpace = EAnalysisSpace(3);
}
#[repr(transparent)]
pub struct EAnalysisLinearAxis(pub u8);
impl EAnalysisLinearAxis {
    pub const PLUS_X: EAnalysisLinearAxis = EAnalysisLinearAxis(0);
    pub const PLUS_Y: EAnalysisLinearAxis = EAnalysisLinearAxis(1);
    pub const PLUS_Z: EAnalysisLinearAxis = EAnalysisLinearAxis(2);
    pub const MINUS_X: EAnalysisLinearAxis = EAnalysisLinearAxis(3);
    pub const MINUS_Y: EAnalysisLinearAxis = EAnalysisLinearAxis(4);
    pub const MINUS_Z: EAnalysisLinearAxis = EAnalysisLinearAxis(5);
}
#[repr(transparent)]
pub struct EAnalysisEulerAxis(pub u8);
impl EAnalysisEulerAxis {
    pub const ROLL: EAnalysisEulerAxis = EAnalysisEulerAxis(0);
    pub const PITCH: EAnalysisEulerAxis = EAnalysisEulerAxis(1);
    pub const YAW: EAnalysisEulerAxis = EAnalysisEulerAxis(2);
}
#[repr(transparent)]
pub struct EEulerCalculationMethod(pub u8);
impl EEulerCalculationMethod {
    pub const AIM_DIRECTION: EEulerCalculationMethod = EEulerCalculationMethod(0);
    pub const POINT_DIRECTION: EEulerCalculationMethod = EEulerCalculationMethod(1);
}
