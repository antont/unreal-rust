#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FAnimPoseEvaluationOptions {
    pub evaluation_type: EAnimDataEvalType,
    pub b_should_retarget: bool,
    pub b_extract_root_motion: bool,
    pub b_incorporate_root_motion_into_pose: bool,
    pub optional_skeletal_mesh: UPtr<USkeletalMesh>,
    pub b_retrieve_additive_as_full_pose: bool,
    pub b_evaluate_curves: bool,
}
#[repr(C, align(8))]
pub struct FAnimPose {
    pub bone_names: TArray<FName>,
    pub bone_indices: TArray<i32>,
    pub parent_bone_indices: TArray<i32>,
    pub local_space_poses: TArray<FTransform>,
    pub world_space_poses: TArray<FTransform>,
    pub ref_local_space_poses: TArray<FTransform>,
    pub ref_world_space_poses: TArray<FTransform>,
    pub curve_names: TArray<FName>,
    pub curve_values: TArray<f32>,
    pub socket_names: TArray<FName>,
    pub socket_parent_bone_names: TArray<FName>,
    pub socket_transforms: TArray<FTransform>,
}
pub struct UAnimationAttributeBlueprintLibrary {}
pub struct UAnimationBlueprintLibrary {}
pub struct UAnimPoseExtensions {}
pub struct UK2Node_BaseAttributeActionNode {}
pub struct UK2Node_SetAttributeKeyGeneric {}
pub struct UK2Node_SetAttributeKeysGeneric {}
pub struct UK2Node_GetAttributeKeyGeneric {}
pub struct UK2Node_GetAttributeKeysGeneric {}
