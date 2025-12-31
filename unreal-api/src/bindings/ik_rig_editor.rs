#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
    pub creation_outer: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
    pub pool: TMap<TSubclassOf<UIKRigStructWrapperBase>, FIKRigStructWrapperBucket>,
}
#[repr(C, align(8))]
pub struct FIKRigHierarchyImportSettings {
    pub mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
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
    pub settings: crate::bindings::ik_rig::FIKRigBodyMoverSettings,
}
pub struct UBodyMoverGoalSettingsWrapper {
    pub settings: crate::bindings::ik_rig::FIKRigBodyMoverGoalSettings,
}
pub struct UFBIKSettingsWrapper {
    pub settings: crate::bindings::ik_rig::FIKRigFBIKSettings,
}
pub struct UFBIKGoalSettingsWrapper {
    pub settings: crate::bindings::ik_rig::FIKRigFBIKGoalSettings,
}
pub struct UFBIKBoneSettingsWrapper {
    pub settings: crate::bindings::ik_rig::FIKRigFBIKBoneSettings,
}
pub struct ULimbSolverSettingsWrapper {
    pub settings: crate::bindings::ik_rig::FIKRigLimbSolverSettings,
}
pub struct UPoleSolverSettingsWrapper {
    pub settings: crate::bindings::ik_rig::FIKRigPoleSolverSettings,
}
pub struct USetTransformSettingsWrapper {
    pub settings: crate::bindings::ik_rig::FIKRigSetTransformSettings,
}
pub struct UStretchLimbSettingsWrapper {
    pub settings: crate::bindings::ik_rig::FIKRigStretchLimbSettings,
}
pub struct UStretchLimbBoneSettingsWrapper {
    pub settings: crate::bindings::ik_rig::FIKRigStretchLimbBoneSettings,
}
pub struct UIKRetargetAnimInstance {
    pub preview_pose_node: FAnimNode_PreviewRetargetPose,
    pub retarget_node: crate::bindings::ik_rig::FAnimNode_RetargetPoseFromMesh,
}
pub struct UIKRetargetBatchOperation {}
pub struct UIKRetargetBoneDetails {
    pub selected_bone: FName,
    pub local_transform: crate::bindings::core_u_object::FTransform,
    pub offset_transform: crate::bindings::core_u_object::FTransform,
    pub current_transform: crate::bindings::core_u_object::FTransform,
    pub reference_transform: crate::bindings::core_u_object::FTransform,
}
pub struct UIKRetargeterController {
    pub struct_viewer: UPtr<UIKRigStructViewer>,
}
pub struct UIKRetargeterThumbnailRenderer {}
pub struct UIKRetargetFactory {
    pub source_ik_rig: TWeakObjectPtr<crate::bindings::ik_rig::UIKRigDefinition>,
}
pub struct URetargetFKChainSettingsWrapper {
    pub settings: crate::bindings::ik_rig::FRetargetFKChainSettings,
}
pub struct URetargetIKChainSettingsWrapper {
    pub settings: crate::bindings::ik_rig::FRetargetIKChainSettings,
}
pub struct URetargetStrideWarpSettingsWrapper {
    pub settings: crate::bindings::ik_rig::FRetargetStrideWarpChainSettings,
}
pub struct URetargetSpeedPlantSettingsWrapper {
    pub settings: crate::bindings::ik_rig::FRetargetSpeedPlantingSettings,
}
pub struct UPoleVectorSettingsWrapper {
    pub settings: crate::bindings::ik_rig::FRetargetPoleVectorSettings,
}
pub struct URetargetPoseOpSettingsWrapper {
    pub settings: crate::bindings::ik_rig::FIKRetargetAdditivePoseOpSettings,
}
pub struct UStretchChainSettingsWrapper {
    pub settings: crate::bindings::ik_rig::FRetargetStretchChainSettings,
}
pub struct UFloorConstraintSettingsWrapper {
    pub settings: crate::bindings::ik_rig::FFloorConstraintChainSettings,
}
pub struct UBatchExportOptions {
    pub b_overwrite_existing_files: bool,
    pub b_include_referenced_assets: bool,
    pub b_retain_additive_flags: bool,
}
pub struct UBatchRetargetSettings {
    pub source_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub target_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub b_auto_generate_retargeter: bool,
    pub retarget_asset: UPtr<crate::bindings::ik_rig::UIKRetargeter>,
}
pub struct UIKRigAnimInstance {
    pub ik_rig_node: crate::bindings::ik_rig::FAnimNode_IKRig,
}
pub struct UIKRigController {
    pub asset: UPtr<crate::bindings::ik_rig::UIKRigDefinition>,
    pub struct_viewer_pool: FIKRigStructWrapperPool,
}
pub struct UIKRigDefinitionFactory {}
pub struct UIKRigBoneDetails {
    pub selected_bone: FName,
    pub current_transform: crate::bindings::core_u_object::FTransform,
    pub reference_transform: crate::bindings::core_u_object::FTransform,
    pub anim_instance_ptr: TWeakObjectPtr<crate::bindings::engine::UAnimInstance>,
    pub asset_ptr: TWeakObjectPtr<crate::bindings::ik_rig::UIKRigDefinition>,
}
pub struct UIKRigThumbnailRenderer {}
#[allow(non_camel_case_types)]
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
