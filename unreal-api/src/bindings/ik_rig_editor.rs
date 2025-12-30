#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FAnimNode_PreviewRetargetPose {}
#[repr(C, align(16))]
pub struct FIKRetargetAnimInstanceProxy {}
#[repr(C, align(16))]
pub struct FIKRigAnimInstanceProxy {}
#[repr(C, align(8))]
pub struct FIKRigStructWrapperBucket {
    pub items: TArray<UPtr<UIKRigStructWrapperBase>>,
}
#[repr(C, align(8))]
pub struct FIKRigStructWrapperPool {
    pub creation_outer: TWeakObjectPtr<UObject>,
    pub pool: TMap<TSubclassOf<UIKRigStructWrapperBase>, FIKRigStructWrapperBucket>,
}
#[repr(C, align(8))]
pub struct FIKRigHierarchyImportSettings {
    pub mesh: UPtr<USkeletalMesh>,
}
#[repr(C, align(4))]
pub struct FIKRigRetargetChainSettings {
    pub chain_name: FName,
    pub start_bone: FName,
    pub end_bone: FName,
}
pub struct UIKRigStructViewer {}
pub struct UIKRigStructWrapperBase {}
pub struct UBodyMoverSettingsWrapper {
    pub settings: FIKRigBodyMoverSettings,
}
pub struct UBodyMoverGoalSettingsWrapper {
    pub settings: FIKRigBodyMoverGoalSettings,
}
pub struct UFBIKSettingsWrapper {
    pub settings: FIKRigFBIKSettings,
}
pub struct UFBIKGoalSettingsWrapper {
    pub settings: FIKRigFBIKGoalSettings,
}
pub struct UFBIKBoneSettingsWrapper {
    pub settings: FIKRigFBIKBoneSettings,
}
pub struct ULimbSolverSettingsWrapper {
    pub settings: FIKRigLimbSolverSettings,
}
pub struct UPoleSolverSettingsWrapper {
    pub settings: FIKRigPoleSolverSettings,
}
pub struct USetTransformSettingsWrapper {
    pub settings: FIKRigSetTransformSettings,
}
pub struct UStretchLimbSettingsWrapper {
    pub settings: FIKRigStretchLimbSettings,
}
pub struct UStretchLimbBoneSettingsWrapper {
    pub settings: FIKRigStretchLimbBoneSettings,
}
pub struct UIKRetargetAnimInstance {
    pub preview_pose_node: FAnimNode_PreviewRetargetPose,
    pub retarget_node: FAnimNode_RetargetPoseFromMesh,
}
pub struct UIKRetargetBatchOperation {}
pub struct UIKRetargetBoneDetails {
    pub selected_bone: FName,
    pub local_transform: FTransform,
    pub offset_transform: FTransform,
    pub current_transform: FTransform,
    pub reference_transform: FTransform,
}
pub struct UIKRetargeterController {
    pub struct_viewer: UPtr<UIKRigStructViewer>,
}
pub struct UIKRetargeterThumbnailRenderer {}
pub struct UIKRetargetFactory {
    pub source_ik_rig: TWeakObjectPtr<UIKRigDefinition>,
}
pub struct URetargetFKChainSettingsWrapper {
    pub settings: FRetargetFKChainSettings,
}
pub struct URetargetIKChainSettingsWrapper {
    pub settings: FRetargetIKChainSettings,
}
pub struct URetargetStrideWarpSettingsWrapper {
    pub settings: FRetargetStrideWarpChainSettings,
}
pub struct URetargetSpeedPlantSettingsWrapper {
    pub settings: FRetargetSpeedPlantingSettings,
}
pub struct UPoleVectorSettingsWrapper {
    pub settings: FRetargetPoleVectorSettings,
}
pub struct URetargetPoseOpSettingsWrapper {
    pub settings: FIKRetargetAdditivePoseOpSettings,
}
pub struct UStretchChainSettingsWrapper {
    pub settings: FRetargetStretchChainSettings,
}
pub struct UFloorConstraintSettingsWrapper {
    pub settings: FFloorConstraintChainSettings,
}
pub struct UBatchExportOptions {
    pub b_overwrite_existing_files: bool,
    pub b_include_referenced_assets: bool,
    pub b_retain_additive_flags: bool,
}
pub struct UBatchRetargetSettings {
    pub source_skeletal_mesh: UPtr<USkeletalMesh>,
    pub target_skeletal_mesh: UPtr<USkeletalMesh>,
    pub b_auto_generate_retargeter: bool,
    pub retarget_asset: UPtr<UIKRetargeter>,
}
pub struct UIKRigAnimInstance {
    pub ik_rig_node: FAnimNode_IKRig,
}
pub struct UIKRigController {
    pub asset: UPtr<UIKRigDefinition>,
    pub struct_viewer_pool: FIKRigStructWrapperPool,
}
pub struct UIKRigDefinitionFactory {}
pub struct UIKRigBoneDetails {
    pub selected_bone: FName,
    pub current_transform: FTransform,
    pub reference_transform: FTransform,
    pub anim_instance_ptr: TWeakObjectPtr<UAnimInstance>,
    pub asset_ptr: TWeakObjectPtr<UIKRigDefinition>,
}
pub struct UIKRigThumbnailRenderer {}
