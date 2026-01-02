#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAnimNode_PreviewRetargetPose {
    __padding_end: [u8; 224],
}
impl FAnimNode_PreviewRetargetPose {}
#[repr(C, align(16))]
pub struct UIKRigStructViewer {
    __padding_end: [u8; 208],
}
impl UIKRigStructViewer {}
#[repr(C, align(16))]
pub struct UIKRigStructWrapperBase {
    __padding_end: [u8; 256],
}
impl UIKRigStructWrapperBase {}
#[repr(C, align(16))]
pub struct UBodyMoverSettingsWrapper {
    __padding_end: [u8; 320],
}
impl UBodyMoverSettingsWrapper {}
#[repr(C, align(16))]
pub struct UBodyMoverGoalSettingsWrapper {
    __padding_end: [u8; 288],
}
impl UBodyMoverGoalSettingsWrapper {}
#[repr(C, align(16))]
pub struct UFBIKSettingsWrapper {
    __padding_end: [u8; 336],
}
impl UFBIKSettingsWrapper {}
#[repr(C, align(16))]
pub struct UFBIKGoalSettingsWrapper {
    __padding_end: [u8; 304],
}
impl UFBIKGoalSettingsWrapper {}
#[repr(C, align(16))]
pub struct UFBIKBoneSettingsWrapper {
    __padding_end: [u8; 352],
}
impl UFBIKBoneSettingsWrapper {}
#[repr(C, align(16))]
pub struct ULimbSolverSettingsWrapper {
    __padding_end: [u8; 336],
}
impl ULimbSolverSettingsWrapper {}
#[repr(C, align(16))]
pub struct UPoleSolverSettingsWrapper {
    __padding_end: [u8; 304],
}
impl UPoleSolverSettingsWrapper {}
#[repr(C, align(16))]
pub struct USetTransformSettingsWrapper {
    __padding_end: [u8; 304],
}
impl USetTransformSettingsWrapper {}
#[repr(C, align(16))]
pub struct UStretchLimbSettingsWrapper {
    __padding_end: [u8; 352],
}
impl UStretchLimbSettingsWrapper {}
#[repr(C, align(16))]
pub struct UStretchLimbBoneSettingsWrapper {
    __padding_end: [u8; 304],
}
impl UStretchLimbBoneSettingsWrapper {}
#[repr(C, align(16))]
pub struct UIKRetargetAnimInstance {
    __padding_end: [u8; 2512],
}
impl UIKRetargetAnimInstance {}
#[repr(C, align(8))]
pub struct UIKRetargetBatchOperation {
    __padding_end: [u8; 320],
}
impl UIKRetargetBatchOperation {}
#[repr(C, align(16))]
pub struct UIKRetargetBoneDetails {
    __padding_end: [u8; 480],
}
impl UIKRetargetBoneDetails {}
#[repr(C, align(8))]
pub struct UIKRetargeterController {
    __padding_end: [u8; 248],
}
impl UIKRetargeterController {}
#[repr(C, align(8))]
pub struct UIKRetargeterThumbnailRenderer {
    __padding_end: [u8; 136],
}
impl UIKRetargeterThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UIKRetargetFactory {
    __padding_end: [u8; 144],
}
impl UIKRetargetFactory {}
#[repr(C, align(16))]
pub struct URetargetFKChainSettingsWrapper {
    __padding_end: [u8; 288],
}
impl URetargetFKChainSettingsWrapper {}
#[repr(C, align(16))]
pub struct URetargetIKChainSettingsWrapper {
    __padding_end: [u8; 416],
}
impl URetargetIKChainSettingsWrapper {}
#[repr(C, align(16))]
pub struct URetargetStrideWarpSettingsWrapper {
    __padding_end: [u8; 272],
}
impl URetargetStrideWarpSettingsWrapper {}
#[repr(C, align(16))]
pub struct URetargetSpeedPlantSettingsWrapper {
    __padding_end: [u8; 272],
}
impl URetargetSpeedPlantSettingsWrapper {}
#[repr(C, align(16))]
pub struct UPoleVectorSettingsWrapper {
    __padding_end: [u8; 288],
}
impl UPoleVectorSettingsWrapper {}
#[repr(C, align(16))]
pub struct URetargetPoseOpSettingsWrapper {
    __padding_end: [u8; 336],
}
impl URetargetPoseOpSettingsWrapper {}
#[repr(C, align(16))]
pub struct UStretchChainSettingsWrapper {
    __padding_end: [u8; 288],
}
impl UStretchChainSettingsWrapper {}
#[repr(C, align(16))]
pub struct UFloorConstraintSettingsWrapper {
    __padding_end: [u8; 288],
}
impl UFloorConstraintSettingsWrapper {}
#[repr(C, align(8))]
pub struct UBatchExportOptions {
    #[doc(hidden)]
    __padding_49: [u8; 49],
    pub b_include_referenced_assets: bool,
    pub b_retain_additive_flags: bool,
    __padding_end: [u8; 5],
}
impl UBatchExportOptions {}
#[repr(C, align(8))]
pub struct UBatchRetargetSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub source_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub target_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub b_auto_generate_retargeter: bool,
    pub retarget_asset: UPtr<crate::bindings::ik_rig::UIKRetargeter>,
}
impl UBatchRetargetSettings {}
#[repr(C, align(16))]
pub struct UIKRigAnimInstance {
    __padding_end: [u8; 2112],
}
impl UIKRigAnimInstance {}
#[repr(C, align(8))]
pub struct UIKRigController {
    __padding_end: [u8; 296],
}
impl UIKRigController {}
#[repr(C, align(8))]
pub struct UIKRigDefinitionFactory {
    __padding_end: [u8; 136],
}
impl UIKRigDefinitionFactory {}
#[repr(C, align(16))]
pub struct UIKRigBoneDetails {
    __padding_end: [u8; 272],
}
impl UIKRigBoneDetails {}
#[repr(C, align(8))]
pub struct UIKRigThumbnailRenderer {
    __padding_end: [u8; 136],
}
impl UIKRigThumbnailRenderer {}
#[repr(transparent)]
pub struct ERetargetAutoAlignMethod(pub u8);
impl ERetargetAutoAlignMethod {
    pub const CHAIN_TO_CHAIN: ERetargetAutoAlignMethod = ERetargetAutoAlignMethod(0);
    pub const MESH_TO_MESH: ERetargetAutoAlignMethod = ERetargetAutoAlignMethod(1);
    pub const LOCAL_ROTATION_AXES: ERetargetAutoAlignMethod = ERetargetAutoAlignMethod(
        2,
    );
    pub const GLOBAL_ROTATION_AXES: ERetargetAutoAlignMethod = ERetargetAutoAlignMethod(
        3,
    );
}
