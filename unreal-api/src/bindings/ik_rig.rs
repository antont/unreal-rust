#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAnimNode_IKRig {
    pub source: crate::bindings::engine::FPoseLink,
    pub rig_definition_asset: UPtr<UIKRigDefinition>,
    pub goals: TArray<FIKRigGoal>,
    pub b_start_from_ref_pose: bool,
    pub b_enable_debug_draw: bool,
    pub debug_scale: f32,
    pub alpha_input_type: crate::bindings::engine::EAnimAlphaInputType,
    pub b_alpha_bool_enabled: bool,
    pub alpha: f32,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub alpha_bool_blend: crate::bindings::engine::FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    pub goal_creators: TArray<UPtr<crate::bindings::engine::UActorComponent>>,
}
#[repr(C, align(16))]
pub struct FIKRigGoal {
    pub name: FName,
    pub bone_name: FName,
    pub transform_source: EIKRigGoalTransformSource,
    pub source_bone: crate::bindings::engine::FBoneReference,
    pub position: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub position_alpha: f32,
    pub rotation_alpha: f32,
    pub position_space: EIKRigGoalSpace,
    pub rotation_space: EIKRigGoalSpace,
    pub final_blended_position: crate::bindings::core_u_object::FVector,
    pub final_blended_rotation: crate::bindings::core_u_object::FQuat,
    pub b_enabled: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_RetargetPoseFromMesh {
    pub source: crate::bindings::engine::FPoseLink,
    pub retarget_from: ERetargetSourceMode,
    pub source_mesh_component: TWeakObjectPtr<
        crate::bindings::engine::USkeletalMeshComponent,
    >,
    pub ik_retargeter_asset: UPtr<UIKRetargeter>,
    pub custom_retarget_profile: FRetargetProfile,
    pub lod_threshold: i32,
    pub lod_threshold_for_ik: i32,
    pub b_suppress_warnings: bool,
    pub b_use_attached_parent_deprecated: bool,
}
#[repr(C, align(8))]
pub struct FRetargetProfile {
    pub retarget_op_profiles: TArray<FRetargetOpProfile>,
    pub b_apply_target_retarget_pose: bool,
    pub target_retarget_pose_name: FName,
    pub b_apply_source_retarget_pose: bool,
    pub source_retarget_pose_name: FName,
    pub b_force_all_ik_off: bool,
    pub b_apply_chain_settings: bool,
    pub chain_settings: TMap<FName, FTargetChainSettings>,
    pub b_apply_root_settings: bool,
    pub root_settings: FTargetRootSettings,
    pub b_apply_global_settings: bool,
    pub global_settings: FRetargetGlobalSettings,
}
#[repr(C, align(4))]
pub struct FRetargetGlobalSettings {
    pub b_enable_root: bool,
    pub b_enable_fk: bool,
    pub b_enable_ik: bool,
    pub b_enable_post: bool,
    pub b_copy_base_pose: bool,
    pub copy_base_pose_root: FName,
    pub source_scale_factor: f32,
    pub b_warping: bool,
    pub direction_source: EWarpingDirectionSource,
    pub forward_direction: EBasicAxis,
    pub direction_chain: FName,
    pub warp_forwards: f32,
    pub sideways_offset: f32,
    pub warp_splay: f32,
}
#[repr(C, align(8))]
pub struct FTargetRootSettings {
    pub rotation_alpha: f32,
    pub translation_alpha: f32,
    pub blend_to_source: f32,
    pub blend_to_source_weights: crate::bindings::core_u_object::FVector,
    pub scale_horizontal: f32,
    pub scale_vertical: f32,
    pub translation_offset: crate::bindings::core_u_object::FVector,
    pub rotation_offset: crate::bindings::core_u_object::FRotator,
    pub affect_ik_horizontal: f32,
    pub affect_ik_vertical: f32,
}
#[repr(C, align(8))]
pub struct FTargetChainSettings {
    pub fk: FTargetChainFKSettings,
    pub ik: FTargetChainIKSettings,
    pub speed_planting: FTargetChainSpeedPlantSettings,
}
#[repr(C, align(4))]
pub struct FTargetChainSpeedPlantSettings {
    pub enable_speed_planting: bool,
    pub speed_curve_name: FName,
    pub speed_threshold: f32,
    pub unplant_stiffness: f32,
    pub unplant_critical_damping: f32,
}
#[repr(C, align(8))]
pub struct FTargetChainIKSettings {
    pub enable_ik: bool,
    pub blend_to_source: f32,
    pub blend_to_source_translation: f32,
    pub blend_to_source_rotation: f32,
    pub blend_to_source_weights: crate::bindings::core_u_object::FVector,
    pub static_offset: crate::bindings::core_u_object::FVector,
    pub static_local_offset: crate::bindings::core_u_object::FVector,
    pub static_rotation_offset: crate::bindings::core_u_object::FRotator,
    pub scale_vertical: f32,
    pub extension: f32,
    pub b_affected_by_ik_warping: bool,
}
#[repr(C, align(4))]
pub struct FTargetChainFKSettings {
    pub enable_fk: bool,
    pub rotation_mode: ERetargetRotationMode,
    pub rotation_alpha: f32,
    pub translation_mode: ERetargetTranslationMode,
    pub translation_alpha: f32,
    pub pole_vector_matching: f32,
    pub pole_vector_maintain_offset: bool,
    pub pole_vector_offset: f32,
}
#[repr(C, align(8))]
pub struct FRetargetOpProfile {
    pub op_to_apply_settings_to: FName,
    pub settings_to_apply: crate::bindings::core_u_object::FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FIKRigWorkData {}
#[repr(C, align(16))]
pub struct FIKRigGoalInput {
    pub goal_name: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub position_alpha: f64,
    pub rotation_alpha: f64,
}
#[repr(C, align(8))]
pub struct FRigUnit_IKRig {
    pub ik_rig_asset: UPtr<UIKRigDefinition>,
    pub goals: TArray<FIKRigGoalInput>,
    pub work_data: FIKRigWorkData,
}
#[repr(C, align(4))]
pub struct FRetargetChainPair {
    pub target_chain_name: FName,
    pub source_chain_name: FName,
}
#[repr(C, align(8))]
pub struct FRetargetChainMapping {
    pub chain_map: TArray<FRetargetChainPair>,
    pub source_ik_rig: UPtr<UIKRigDefinition>,
    pub target_ik_rig: UPtr<UIKRigDefinition>,
}
#[repr(C, align(8))]
pub struct FIKRetargetPose {
    pub root_translation_offset: crate::bindings::core_u_object::FVector,
    pub bone_rotation_offsets: TMap<FName, crate::bindings::core_u_object::FQuat>,
}
#[repr(C, align(8))]
pub struct FIKRetargetOpSettingsBase {
    pub lod_threshold: i32,
    pub b_debug_draw: bool,
    pub owning_op_name: FName,
}
#[repr(C, align(8))]
pub struct FIKRetargetOpBase {
    pub b_is_enabled: bool,
    pub name: FName,
    pub parent_op_name: FName,
    pub b_take_input_curves_from_source_anim_instance: bool,
}
#[repr(C, align(8))]
pub struct FRetargetPoleVectorSettings {
    pub target_chain_name: FName,
    pub b_enabled: bool,
    pub align_alpha: f64,
    pub static_angular_offset: f64,
    pub maintain_offset: bool,
}
#[repr(C, align(8))]
pub struct FIKRetargetAlignPoleVectorOpSettings {
    pub ik_rig_asset: UPtr<UIKRigDefinition>,
    pub chains_to_align: TArray<FRetargetPoleVectorSettings>,
}
#[repr(C, align(8))]
pub struct FIKRetargetAlignPoleVectorOp {
    pub settings: FIKRetargetAlignPoleVectorOpSettings,
    pub chain_mapping: FRetargetChainMapping,
}
#[repr(C, align(8))]
pub struct FIKRetargetCopyBasePoseOpSettings {
    pub b_copy_base_pose: bool,
    pub copy_from_start: crate::bindings::engine::FBoneReference,
    pub bones_to_exclude: TArray<crate::bindings::engine::FBoneReference>,
    pub copy_base_pose_root_deprecated: FName,
}
#[repr(C, align(8))]
pub struct FIKRetargetCopyBasePoseOp {
    pub settings: FIKRetargetCopyBasePoseOpSettings,
}
#[repr(C, align(4))]
pub struct FCurveRemapPair {
    pub source_curve: FName,
    pub target_curve: FName,
}
#[repr(C, align(8))]
pub struct FIKRetargetCurveRemapOpSettings {
    pub b_copy_all_source_curves: bool,
    pub b_remap_curves: bool,
    pub curves_to_remap: TArray<FCurveRemapPair>,
}
#[repr(C, align(8))]
pub struct FIKRetargetCurveRemapOp {
    pub settings: FIKRetargetCurveRemapOpSettings,
}
#[repr(C, align(16))]
pub struct FFilterBoneData {
    pub target_bone: crate::bindings::engine::FBoneReference,
}
#[repr(C, align(8))]
pub struct FIKRetargetFilterBoneOpSettings {
    pub bones_to_filter: TArray<FFilterBoneData>,
    pub min_frequency: f64,
    pub responsiveness: f64,
    pub velocity_cutoff_hz: f64,
    pub b_reset_playback: bool,
}
#[repr(C, align(8))]
pub struct FIKRetargetFilterBoneOp {
    pub settings: FIKRetargetFilterBoneOpSettings,
}
#[repr(C, align(8))]
pub struct FRetargetFKChainSettings {
    pub target_chain_name: FName,
    pub enable_fk: bool,
    pub rotation_mode: EFKChainRotationMode,
    pub rotation_alpha: f64,
    pub translation_mode: EFKChainTranslationMode,
    pub translation_alpha: f64,
}
#[repr(C, align(8))]
pub struct FIKRetargetFKChainsOpSettings {
    pub ik_rig_asset: UPtr<UIKRigDefinition>,
    pub chains_to_retarget: TArray<FRetargetFKChainSettings>,
    pub b_draw_chain_lines: bool,
    pub b_draw_single_bone_chains: bool,
    pub chain_draw_thickness: f64,
    pub chain_draw_size: f64,
    pub chain_mapping: FRetargetChainMapping,
}
#[repr(C, align(8))]
pub struct FIKRetargetFKChainsOp {
    pub settings: FIKRetargetFKChainsOpSettings,
    pub chain_mapping_deprecated: FRetargetChainMapping,
}
#[repr(C, align(8))]
pub struct FFloorConstraintChainSettings {
    pub target_chain_name: FName,
    pub enable_floor_constraint: bool,
    pub alpha: f64,
    pub maintain_height_offset: f64,
}
#[repr(C, align(8))]
pub struct FIKRetargetFloorConstraintOpSettings {
    pub chains_to_affect: TArray<FFloorConstraintChainSettings>,
    pub height_falloff_offset: f64,
    pub height_falloff_distance: f64,
}
#[repr(C, align(8))]
pub struct FIKRetargetFloorConstraintOp {
    pub settings: FIKRetargetFloorConstraintOpSettings,
}
#[repr(C, align(8))]
pub struct FRetargetIKChainSettings {
    pub target_chain_name: FName,
    pub enable_ik: bool,
    pub blend_to_source: f64,
    pub blend_to_source_translation: f64,
    pub blend_to_source_rotation: f64,
    pub blend_to_source_weights: crate::bindings::core_u_object::FVector,
    pub apply_pelvis_offset_to_source_goals: bool,
    pub static_offset: crate::bindings::core_u_object::FVector,
    pub static_local_offset: crate::bindings::core_u_object::FVector,
    pub static_rotation_offset: crate::bindings::core_u_object::FRotator,
    pub scale_vertical: f64,
    pub extension: f64,
}
#[repr(C, align(8))]
pub struct FIKRetargetIKChainsOpSettings {
    pub chains_to_retarget: TArray<FRetargetIKChainSettings>,
    pub b_draw_final_goals: bool,
    pub b_draw_source_locations: bool,
    pub goal_draw_size: f64,
    pub goal_draw_thickness: f64,
}
#[repr(C, align(8))]
pub struct FIKRetargetIKChainsOp {
    pub settings: FIKRetargetIKChainsOpSettings,
}
#[repr(C, align(8))]
pub struct FIKRetargetPelvisMotionOpSettings {
    pub source_pelvis_bone: crate::bindings::engine::FBoneReference,
    pub target_pelvis_bone: crate::bindings::engine::FBoneReference,
    pub floor_constraint_weight: f64,
    pub source_crotch_offset: f64,
    pub target_crotch_offset: f64,
    pub rotation_alpha: f64,
    pub rotation_offset_local: crate::bindings::core_u_object::FRotator,
    pub rotation_offset_global: crate::bindings::core_u_object::FRotator,
    pub translation_alpha: f64,
    pub translation_offset_local: crate::bindings::core_u_object::FVector,
    pub translation_offset_global: crate::bindings::core_u_object::FVector,
    pub blend_to_source_translation: f64,
    pub blend_to_source_translation_weights: crate::bindings::core_u_object::FVector,
    pub scale_horizontal: f64,
    pub scale_vertical: f64,
    pub affect_ik_horizontal: f64,
    pub affect_ik_vertical: f64,
    pub debug_draw_size: f64,
    pub debug_draw_thickness: f64,
    pub translation_offset_deprecated: crate::bindings::core_u_object::FVector,
    pub rotation_offset_deprecated: crate::bindings::core_u_object::FRotator,
    pub b_enable_debug_draw_deprecated: bool,
}
#[repr(C, align(16))]
pub struct FIKRetargetPelvisMotionOp {
    pub settings: FIKRetargetPelvisMotionOpSettings,
}
#[repr(C, align(16))]
pub struct FPinBoneData {
    pub bone_to_copy_from: crate::bindings::engine::FBoneReference,
    pub bone_to_copy_to: crate::bindings::engine::FBoneReference,
    pub bone_to_pin_deprecated: FName,
    pub bone_to_pin_to_deprecated: FName,
}
#[repr(C, align(16))]
pub struct FIKRetargetPinBoneOpSettings {
    pub bones_to_pin: TArray<FPinBoneData>,
    pub skeleton_to_copy_from: ERetargetSourceOrTarget,
    pub b_copy_translation: bool,
    pub translation_mode: EPinBoneTranslationMode,
    pub b_copy_rotation: bool,
    pub rotation_mode: EPinBoneRotationMode,
    pub b_copy_scale: bool,
    pub b_propagate_to_children: bool,
    pub global_offset: crate::bindings::core_u_object::FTransform,
    pub local_offset: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FIKRetargetPinBoneOp {
    pub settings: FIKRetargetPinBoneOpSettings,
}
#[repr(C, align(8))]
pub struct FIKRetargetAdditivePoseOpSettings {
    pub pose_to_apply: FName,
    pub alpha: f32,
}
#[repr(C, align(8))]
pub struct FIKRetargetAdditivePoseOp {
    pub settings: FIKRetargetAdditivePoseOpSettings,
}
#[repr(C, align(16))]
pub struct FIKRetargetRootMotionOpSettings {
    pub source_root: crate::bindings::engine::FBoneReference,
    pub target_root: crate::bindings::engine::FBoneReference,
    pub root_motion_source: ERootMotionSource,
    pub target_pelvis: crate::bindings::engine::FBoneReference,
    pub b_rotate_with_pelvis: bool,
    pub root_height_source: ERootMotionHeightSource,
    pub global_offset: crate::bindings::core_u_object::FTransform,
    pub b_maintain_offset_from_pelvis: bool,
    pub b_propagate_to_non_retargeted_children: bool,
    pub source_root_bone_deprecated: FName,
    pub target_root_bone_deprecated: FName,
    pub target_pelvis_bone_deprecated: FName,
}
#[repr(C, align(16))]
pub struct FIKRetargetRootMotionOp {
    pub settings: FIKRetargetRootMotionOpSettings,
}
#[repr(C, align(8))]
pub struct FIKRetargetRunIKRigOpSettings {
    pub ik_rig_asset: UPtr<UIKRigDefinition>,
    pub excluded_goals: TArray<FName>,
    pub b_draw_goals: bool,
    pub b_draw_goal_bone_locations: bool,
    pub goal_draw_size: f64,
    pub goal_draw_thickness: f64,
}
#[repr(C, align(8))]
pub struct FIKRetargetRunIKRigOp {
    pub settings: FIKRetargetRunIKRigOpSettings,
    pub chain_mapping: FRetargetChainMapping,
}
#[repr(C, align(8))]
pub struct FIKRetargetScaleSourceOpSettings {
    pub source_scale_factor: f64,
    pub scale_pivot: EScaleSourcePivot,
    pub scale_pivot_bone: crate::bindings::engine::FBoneReference,
    pub b_project_scale_pivot_to_floor: bool,
}
#[repr(C, align(8))]
pub struct FIKRetargetScaleSourceOp {
    pub settings: FIKRetargetScaleSourceOpSettings,
}
#[repr(C, align(4))]
pub struct FRetargetSpeedPlantingSettings {
    pub target_chain_name: FName,
    pub speed_curve_name: FName,
}
#[repr(C, align(8))]
pub struct FIKRetargetSpeedPlantingOpSettings {
    pub chains_to_speed_plant: TArray<FRetargetSpeedPlantingSettings>,
    pub speed_threshold: f64,
    pub stiffness: f64,
    pub critical_damping: f64,
}
#[repr(C, align(8))]
pub struct FIKRetargetSpeedPlantingOp {
    pub settings: FIKRetargetSpeedPlantingOpSettings,
}
#[repr(C, align(8))]
pub struct FRetargetStretchChainSettings {
    pub target_chain_name: FName,
    pub b_enabled: bool,
    pub match_source_length: f64,
    pub scale_chain_length: f64,
}
#[repr(C, align(8))]
pub struct FIKRetargetStretchChainOpSettings {
    pub ik_rig_asset: UPtr<UIKRigDefinition>,
    pub chains_to_stretch: TArray<FRetargetStretchChainSettings>,
}
#[repr(C, align(8))]
pub struct FIKRetargetStretchChainOp {
    pub settings: FIKRetargetStretchChainOpSettings,
    pub chain_mapping: FRetargetChainMapping,
}
#[repr(C, align(4))]
pub struct FRetargetStrideWarpChainSettings {
    pub target_chain_name: FName,
    pub enable_stride_warping: bool,
}
#[repr(C, align(8))]
pub struct FIKRetargetStrideWarpingOpSettings {
    pub chain_settings: TArray<FRetargetStrideWarpChainSettings>,
    pub direction_source: EWarpingDirectionSource,
    pub forward_direction: EBasicAxis,
    pub direction_chain: FName,
    pub warp_forwards: f64,
    pub sideways_offset: f64,
    pub warp_splay: f64,
    pub debug_draw_size: f64,
    pub debug_draw_thickness: f64,
    pub b_enable_debug_draw_deprecated: bool,
}
#[repr(C, align(16))]
pub struct FIKRetargetStrideWarpingOp {
    pub settings: FIKRetargetStrideWarpingOpSettings,
}
#[repr(C, align(8))]
pub struct FIKRigGoalContainer {}
#[repr(C, align(4))]
pub struct FBoneChain {
    pub chain_name: FName,
    pub start_bone: crate::bindings::engine::FBoneReference,
    pub end_bone: crate::bindings::engine::FBoneReference,
    pub ik_goal_name: FName,
}
#[repr(C, align(8))]
pub struct FRetargetDefinition {
    pub root_bone: FName,
    pub bone_chains: TArray<FBoneChain>,
}
#[repr(C, align(4))]
pub struct FGoalBone {}
#[repr(C, align(8))]
pub struct FIKRigInputSkeleton {}
#[repr(C, align(8))]
pub struct FIKRigSkeleton {
    pub bone_names: TArray<FName>,
    pub parent_indices: TArray<i32>,
    pub excluded_bones: TArray<FName>,
    pub current_pose_global: TArray<crate::bindings::core_u_object::FTransform>,
    pub current_pose_local: TArray<crate::bindings::core_u_object::FTransform>,
    pub ref_pose_global: TArray<crate::bindings::core_u_object::FTransform>,
}
#[repr(C, align(8))]
pub struct FIKRigSettingsBase {}
#[repr(C, align(8))]
pub struct FIKRigGoalSettingsBase {
    pub goal: FName,
}
#[repr(C, align(8))]
pub struct FIKRigBodyMoverGoalSettings {
    pub bone_name: FName,
    pub influence_multiplier: f32,
}
#[repr(C, align(8))]
pub struct FIKRigSolverSettingsBase {}
#[repr(C, align(8))]
pub struct FIKRigBodyMoverSettings {
    pub start_bone: FName,
    pub position_alpha: f32,
    pub position_positive_x: f32,
    pub position_negative_x: f32,
    pub position_positive_y: f32,
    pub position_negative_y: f32,
    pub position_positive_z: f32,
    pub position_negative_z: f32,
    pub rotation_alpha: f32,
    pub rotate_x_alpha: f32,
    pub rotate_y_alpha: f32,
    pub rotate_z_alpha: f32,
}
#[repr(C, align(8))]
pub struct FIKRigSolverBase {
    pub b_is_enabled: bool,
}
#[repr(C, align(8))]
pub struct FIKRigBodyMoverSolver {
    pub settings: FIKRigBodyMoverSettings,
    pub all_goal_settings: TArray<FIKRigBodyMoverGoalSettings>,
}
#[repr(C, align(8))]
pub struct FIKRigFBIKGoalSettings {
    pub bone_name: FName,
    pub chain_depth: i32,
    pub strength_alpha: f32,
    pub pull_chain_alpha: f32,
    pub pin_rotation: f32,
    pub index_in_solver: i32,
}
#[repr(C, align(8))]
pub struct FIKRigBoneSettingsBase {
    pub bone: FName,
}
#[repr(C, align(8))]
pub struct FIKRigFBIKBoneSettings {
    pub rotation_stiffness: f32,
    pub position_stiffness: f32,
    pub x: crate::bindings::pbik::EPBIKLimitType,
    pub min_x: f32,
    pub max_x: f32,
    pub y: crate::bindings::pbik::EPBIKLimitType,
    pub min_y: f32,
    pub max_y: f32,
    pub z: crate::bindings::pbik::EPBIKLimitType,
    pub min_z: f32,
    pub max_z: f32,
    pub b_use_preferred_angles: bool,
    pub preferred_angles: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FIKRigFBIKSettings {
    pub root_bone: FName,
    pub iterations: i32,
    pub sub_iterations: i32,
    pub mass_multiplier: f32,
    pub b_allow_stretch: bool,
    pub root_behavior: crate::bindings::pbik::EPBIKRootBehavior,
    pub pre_pull_root_settings: crate::bindings::pbik::FRootPrePullSettings,
    pub global_pull_chain_alpha: f32,
    pub max_angle: f32,
    pub over_relaxation: f32,
}
#[repr(C, align(8))]
pub struct FIKRigFullBodyIKSolver {
    pub settings: FIKRigFBIKSettings,
    pub all_goal_settings: TArray<FIKRigFBIKGoalSettings>,
    pub all_bone_settings: TArray<FIKRigFBIKBoneSettings>,
}
#[repr(C, align(8))]
pub struct FLimbSolverSettings {
    pub reach_precision: f32,
    pub hinge_rotation_axis: crate::bindings::core_u_object::EAxis,
    pub max_iterations: i32,
    pub b_enable_limit: bool,
    pub min_rotation_angle: f32,
    pub b_average_pull: bool,
    pub pull_distribution: f32,
    pub reach_step_alpha: f32,
    pub b_enable_twist_correction: bool,
    pub end_bone_forward_axis: crate::bindings::core_u_object::EAxis,
}
#[repr(C, align(8))]
pub struct FIKRigLimbSolverSettings {
    pub start_bone: FName,
    pub goal_name: FName,
    pub end_bone: FName,
}
#[repr(C, align(8))]
pub struct FIKRigLimbSolver {
    pub settings: FIKRigLimbSolverSettings,
}
#[repr(C, align(8))]
pub struct FIKRigPoleSolverSettings {
    pub start_bone: FName,
    pub end_bone: FName,
    pub aim_at_goal: FName,
    pub alpha: f32,
}
#[repr(C, align(8))]
pub struct FIKRigPoleSolver {
    pub settings: FIKRigPoleSolverSettings,
}
#[repr(C, align(8))]
pub struct FIKRigSetTransformSettings {
    pub goal: FName,
    pub bone_to_affect: FName,
    pub position_alpha: f32,
    pub rotation_alpha: f32,
    pub alpha: f32,
    pub b_propagate_to_children: bool,
}
#[repr(C, align(8))]
pub struct FIKRigSetTransform {
    pub settings: FIKRigSetTransformSettings,
}
#[repr(C, align(8))]
pub struct FIKRigStretchLimbBoneSettings {
    pub squash_direction: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FIKRigStretchLimbSettings {
    pub start_bone: FName,
    pub end_bone: FName,
    pub goal: FName,
    pub b_enable_stretching: bool,
    pub maximum_stretch_distance: f64,
    pub stretch_start_percent: f64,
    pub rotation_mode: EStretchLimbRotationMode,
    pub rotate_end_bone_with_goal: f64,
    pub iterations: i32,
    pub squash_mode: EStretchLimbSquashMode,
    pub squash_strength: f64,
}
#[repr(C, align(8))]
pub struct FIKRigStretchLimbSolver {
    pub settings: FIKRigStretchLimbSettings,
    pub all_bone_settings: TArray<FIKRigStretchLimbBoneSettings>,
}
#[repr(C, align(8))]
pub struct FLimbLink {}
#[repr(C, align(8))]
pub struct FLimbSolver {}
pub struct UIKGoalCreatorInterface {}
pub struct IIKGoalCreatorInterface {}
pub struct UIKRigComponent {}
pub struct URetargetChainSettings {
    pub source_chain: FName,
    pub target_chain: FName,
    pub settings: FTargetChainSettings,
    pub copy_pose_using_fk_deprecated: bool,
    pub rotation_mode_deprecated: ERetargetRotationMode,
    pub rotation_alpha_deprecated: f32,
    pub translation_mode_deprecated: ERetargetTranslationMode,
    pub translation_alpha_deprecated: f32,
    pub drive_ik_goal_deprecated: bool,
    pub blend_to_source_deprecated: f32,
    pub blend_to_source_weights_deprecated: crate::bindings::core_u_object::FVector,
    pub static_offset_deprecated: crate::bindings::core_u_object::FVector,
    pub static_local_offset_deprecated: crate::bindings::core_u_object::FVector,
    pub static_rotation_offset_deprecated: crate::bindings::core_u_object::FRotator,
    pub extension_deprecated: f32,
    pub use_speed_curve_to_plant_ik_deprecated: bool,
    pub speed_curve_name_deprecated: FName,
    pub velocity_threshold_deprecated: f32,
    pub unplant_stiffness_deprecated: f32,
    pub unplant_critical_damping_deprecated: f32,
}
pub struct URetargetRootSettings {
    pub settings: FTargetRootSettings,
    pub retarget_root_translation_deprecated: bool,
    pub global_scale_horizontal_deprecated: f32,
    pub global_scale_vertical_deprecated: f32,
    pub blend_to_source_deprecated: crate::bindings::core_u_object::FVector,
    pub static_offset_deprecated: crate::bindings::core_u_object::FVector,
    pub static_rotation_offset_deprecated: crate::bindings::core_u_object::FRotator,
}
pub struct UIKRetargetGlobalSettings {
    pub settings: FRetargetGlobalSettings,
}
pub struct UIKRetargeter {
    pub version: i32,
    pub source_ik_rig_asset: UPtr<UIKRigDefinition>,
    pub source_preview_mesh: TSoftObjectPtr<crate::bindings::engine::USkeletalMesh>,
    pub target_ik_rig_asset: UPtr<UIKRigDefinition>,
    pub target_preview_mesh: TSoftObjectPtr<crate::bindings::engine::USkeletalMesh>,
    pub target_mesh_offset: crate::bindings::core_u_object::FVector,
    pub target_mesh_scale: f32,
    pub source_mesh_offset: crate::bindings::core_u_object::FVector,
    pub b_show_source_mesh: bool,
    pub b_show_target_mesh: bool,
    pub b_show_source_skeleton: bool,
    pub b_show_target_skeleton: bool,
    pub b_override_source_skeleton_color: bool,
    pub source_overide_color: crate::bindings::core_u_object::FLinearColor,
    pub b_override_target_skeleton_color: bool,
    pub target_overide_color: crate::bindings::core_u_object::FLinearColor,
    pub b_ignore_root_lock_in_preview: bool,
    pub b_debug_draw: bool,
    pub b_profile_ops: bool,
    pub bone_draw_size: f32,
    pub controller: UPtr<crate::bindings::core_u_object::UObject>,
    pub retarget_ops: TArray<crate::bindings::core_u_object::FInstancedStruct>,
    pub profiles: TMap<FName, FRetargetProfile>,
    pub current_profile: FName,
    pub source_retarget_poses: TMap<FName, FIKRetargetPose>,
    pub target_retarget_poses: TMap<FName, FIKRetargetPose>,
    pub current_source_retarget_pose: FName,
    pub current_target_retarget_pose: FName,
    pub b_retarget_root_deprecated: bool,
    pub b_retarget_fk_deprecated: bool,
    pub b_retarget_ik_deprecated: bool,
    pub target_actor_offset_deprecated: f32,
    pub target_actor_scale_deprecated: f32,
    pub retarget_poses: TMap<FName, FIKRetargetPose>,
    pub current_retarget_pose: FName,
    pub chain_map_deprecated: FRetargetChainMapping,
    pub chain_settings_deprecated: TArray<UPtr<URetargetChainSettings>>,
    pub root_settings_deprecated: UPtr<URetargetRootSettings>,
    pub global_settings_deprecated: UPtr<UIKRetargetGlobalSettings>,
    pub op_stack_deprecated: UPtr<URetargetOpStack>,
}
pub struct UIKRetargetOpControllerBase {}
pub struct URetargetOpBase {
    pub b_is_enabled: bool,
}
pub struct URetargetOpStack {
    pub retarget_ops_deprecated: TArray<UPtr<URetargetOpBase>>,
}
pub struct UIKRetargetProcessor {}
pub struct URetargetProfileLibrary {}
pub struct UIKRetargetAlignPoleVectorController {}
pub struct UIKRetargetCopyBasePoseController {}
pub struct UIKRetargetCurveRemapController {}
pub struct UCurveRemapOp {
    pub curves_to_remap: TArray<FCurveRemapPair>,
    pub b_copy_all_source_curves: bool,
}
pub struct UIKRetargetFilterBoneController {}
pub struct UIKRetargetFKChainsController {}
pub struct UIKRetargetFloorGoalsController {}
pub struct UIKRetargetIKChainsController {}
pub struct UIKRetargetPelvisMotionController {}
pub struct UIKRetargetPinBoneController {}
pub struct UPinBoneOp {
    pub bones_to_pin: TArray<FPinBoneData>,
    pub pin_to: ERetargetSourceOrTarget,
    pub b_copy_translation: bool,
    pub b_copy_rotation: bool,
    pub b_copy_scale: bool,
    pub translation_mode: EPinBoneTranslationMode,
    pub rotation_mode: EPinBoneRotationMode,
    pub global_offset: crate::bindings::core_u_object::FTransform,
    pub local_offset: crate::bindings::core_u_object::FTransform,
    pub b_maintain_offset_deprecated: bool,
    pub pin_type_deprecated: EPinBoneType,
}
pub struct UIKRetargetAdditivePoseController {}
pub struct UIKRetargetRootMotionController {}
pub struct URootMotionGeneratorOp {
    pub source_root_bone: FName,
    pub target_root_bone: FName,
    pub target_pelvis_bone: FName,
    pub root_motion_source: ERootMotionSource,
    pub root_height_source: ERootMotionHeightSource,
    pub b_propagate_to_non_retargeted_children: bool,
    pub b_maintain_offset_from_pelvis: bool,
    pub b_rotate_with_pelvis: bool,
    pub global_offset: crate::bindings::core_u_object::FTransform,
}
pub struct UIKRetargetRunIKRigController {}
pub struct UIKRetargetScaleSourceController {}
pub struct UIKRetargetSpeedPlantingController {}
pub struct UIKRetargetStretchChainController {}
pub struct UIKRetargetStrideWarpingController {}
pub struct UIKRigEffectorGoal {
    pub goal_name: FName,
    pub bone_name: FName,
    pub position_alpha: f32,
    pub rotation_alpha: f32,
    pub current_transform: crate::bindings::core_u_object::FTransform,
    pub initial_transform: crate::bindings::core_u_object::FTransform,
    pub preview_mode: EIKRigGoalPreviewMode,
    pub size_multiplier: f32,
    pub thickness_multiplier: f32,
    pub b_expose_position: bool,
    pub b_expose_rotation: bool,
}
pub struct UIKRigDefinition {
    pub preview_skeletal_mesh: TSoftObjectPtr<crate::bindings::engine::USkeletalMesh>,
    pub bone_size: f32,
    pub draw_goals: bool,
    pub goal_size: f32,
    pub goal_thickness: f32,
    pub controller: UPtr<crate::bindings::core_u_object::UObject>,
    pub skeleton: FIKRigSkeleton,
    pub goals: TArray<UPtr<UIKRigEffectorGoal>>,
    pub solver_stack: TArray<crate::bindings::core_u_object::FInstancedStruct>,
    pub retarget_definition: FRetargetDefinition,
    pub solvers_deprecated: TArray<UPtr<UIKRigSolver>>,
}
pub struct UIKRigProcessor {}
pub struct UIKRigSolverControllerBase {}
pub struct UIKRigBodyMoverController {}
pub struct UIKRig_BodyMoverEffector {
    pub goal_name: FName,
    pub bone_name: FName,
    pub influence_multiplier: f32,
}
pub struct UIKRigSolver {
    pub b_is_enabled: bool,
}
pub struct UIKRig_BodyMover {
    pub root_bone: FName,
    pub position_alpha: f32,
    pub position_positive_x: f32,
    pub position_negative_x: f32,
    pub position_positive_y: f32,
    pub position_negative_y: f32,
    pub position_positive_z: f32,
    pub position_negative_z: f32,
    pub rotation_alpha: f32,
    pub rotate_x_alpha: f32,
    pub rotate_y_alpha: f32,
    pub rotate_z_alpha: f32,
    pub effectors_deprecated: TArray<UPtr<UIKRig_BodyMoverEffector>>,
}
pub struct UIKRigFBIKController {}
pub struct UIKRig_FBIKEffector {
    pub goal_name: FName,
    pub bone_name: FName,
    pub chain_depth: i32,
    pub strength_alpha: f32,
    pub pull_chain_alpha: f32,
    pub pin_rotation: f32,
}
pub struct UIKRig_FBIKBoneSettings {
    pub bone: FName,
    pub rotation_stiffness: f32,
    pub position_stiffness: f32,
    pub x: crate::bindings::pbik::EPBIKLimitType,
    pub min_x: f32,
    pub max_x: f32,
    pub y: crate::bindings::pbik::EPBIKLimitType,
    pub min_y: f32,
    pub max_y: f32,
    pub z: crate::bindings::pbik::EPBIKLimitType,
    pub min_z: f32,
    pub max_z: f32,
    pub b_use_preferred_angles: bool,
    pub preferred_angles: crate::bindings::core_u_object::FVector,
}
pub struct UIKRigFBIKSolver {
    pub root_bone: FName,
    pub iterations: i32,
    pub sub_iterations: i32,
    pub mass_multiplier: f32,
    pub b_allow_stretch: bool,
    pub root_behavior: crate::bindings::pbik::EPBIKRootBehavior,
    pub pre_pull_root_settings: crate::bindings::pbik::FRootPrePullSettings,
    pub pull_chain_alpha: f32,
    pub max_angle: f32,
    pub over_relaxation: f32,
    pub effectors_deprecated: TArray<UPtr<UIKRig_FBIKEffector>>,
    pub bone_settings_deprecated: TArray<UPtr<UIKRig_FBIKBoneSettings>>,
}
pub struct UIKRigLimbSolverController {}
pub struct UIKRig_LimbEffector {
    pub goal_name: FName,
    pub bone_name: FName,
}
pub struct UIKRig_LimbSolver {
    pub root_name: FName,
    pub reach_precision: f32,
    pub hinge_rotation_axis: crate::bindings::core_u_object::EAxis,
    pub max_iterations: i32,
    pub b_enable_limit: bool,
    pub min_rotation_angle: f32,
    pub b_average_pull: bool,
    pub pull_distribution: f32,
    pub reach_step_alpha: f32,
    pub b_enable_twist_correction: bool,
    pub end_bone_forward_axis: crate::bindings::core_u_object::EAxis,
    pub effector_deprecated: UPtr<UIKRig_LimbEffector>,
}
pub struct UIKRigPoleSolverController {}
pub struct UIKRig_PoleSolverEffector {
    pub goal_name: FName,
    pub bone_name: FName,
    pub alpha: f32,
}
pub struct UIKRig_PoleSolver {
    pub root_name: FName,
    pub end_name: FName,
    pub effector_deprecated: UPtr<UIKRig_PoleSolverEffector>,
}
pub struct UIKRigSetTransformController {}
pub struct UIKRig_SetTransformEffector {
    pub b_enable_position: bool,
    pub b_enable_rotation: bool,
    pub alpha: f32,
}
pub struct UIKRig_SetTransform {
    pub goal: FName,
    pub root_bone: FName,
    pub effector_deprecated: UPtr<UIKRig_SetTransformEffector>,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EIKRigGoalTransformSource(pub u8);
impl EIKRigGoalTransformSource {
    pub const MANUAL: EIKRigGoalTransformSource = EIKRigGoalTransformSource(0);
    pub const BONE: EIKRigGoalTransformSource = EIKRigGoalTransformSource(1);
    pub const ACTOR_COMPONENT: EIKRigGoalTransformSource = EIKRigGoalTransformSource(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EIKRigGoalSpace(pub u8);
impl EIKRigGoalSpace {
    pub const COMPONENT: EIKRigGoalSpace = EIKRigGoalSpace(0);
    pub const ADDITIVE: EIKRigGoalSpace = EIKRigGoalSpace(1);
    pub const WORLD: EIKRigGoalSpace = EIKRigGoalSpace(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERetargetSourceMode(pub u8);
impl ERetargetSourceMode {
    pub const PARENT_SKELETAL_MESH_COMPONENT: ERetargetSourceMode = ERetargetSourceMode(
        0,
    );
    pub const CUSTOM_SKELETAL_MESH_COMPONENT: ERetargetSourceMode = ERetargetSourceMode(
        1,
    );
    pub const SOURCE_POSE_PIN: ERetargetSourceMode = ERetargetSourceMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERetargetRotationMode(pub u8);
impl ERetargetRotationMode {
    pub const INTERPOLATED: ERetargetRotationMode = ERetargetRotationMode(0);
    pub const ONE_TO_ONE: ERetargetRotationMode = ERetargetRotationMode(1);
    pub const ONE_TO_ONE_REVERSED: ERetargetRotationMode = ERetargetRotationMode(2);
    pub const MATCH_CHAIN: ERetargetRotationMode = ERetargetRotationMode(3);
    pub const MATCH_SCALED_CHAIN: ERetargetRotationMode = ERetargetRotationMode(4);
    pub const NONE: ERetargetRotationMode = ERetargetRotationMode(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWarpingDirectionSource(pub i32);
impl EWarpingDirectionSource {
    pub const GOALS: EWarpingDirectionSource = EWarpingDirectionSource(0);
    pub const CHAIN: EWarpingDirectionSource = EWarpingDirectionSource(1);
    pub const ROOT_BONE: EWarpingDirectionSource = EWarpingDirectionSource(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBasicAxis(pub i32);
impl EBasicAxis {
    pub const X: EBasicAxis = EBasicAxis(0);
    pub const Y: EBasicAxis = EBasicAxis(1);
    pub const Z: EBasicAxis = EBasicAxis(2);
    pub const NEG_X: EBasicAxis = EBasicAxis(3);
    pub const NEG_Y: EBasicAxis = EBasicAxis(4);
    pub const NEG_Z: EBasicAxis = EBasicAxis(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERetargetTranslationMode(pub u8);
impl ERetargetTranslationMode {
    pub const NONE: ERetargetTranslationMode = ERetargetTranslationMode(0);
    pub const GLOBALLY_SCALED: ERetargetTranslationMode = ERetargetTranslationMode(1);
    pub const ABSOLUTE: ERetargetTranslationMode = ERetargetTranslationMode(2);
    pub const STRETCH_BONE_LENGTH_UNIFORMLY: ERetargetTranslationMode = ERetargetTranslationMode(
        3,
    );
    pub const STRETCH_BONE_LENGTH_NON_UNIFORMLY: ERetargetTranslationMode = ERetargetTranslationMode(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFKChainRotationMode(pub u8);
impl EFKChainRotationMode {
    pub const NONE: EFKChainRotationMode = EFKChainRotationMode(5);
    pub const INTERPOLATED: EFKChainRotationMode = EFKChainRotationMode(0);
    pub const ONE_TO_ONE: EFKChainRotationMode = EFKChainRotationMode(1);
    pub const ONE_TO_ONE_REVERSED: EFKChainRotationMode = EFKChainRotationMode(2);
    pub const MATCH_CHAIN: EFKChainRotationMode = EFKChainRotationMode(3);
    pub const MATCH_SCALED_CHAIN: EFKChainRotationMode = EFKChainRotationMode(4);
    pub const COPY_LOCAL: EFKChainRotationMode = EFKChainRotationMode(6);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFKChainTranslationMode(pub u8);
impl EFKChainTranslationMode {
    pub const NONE: EFKChainTranslationMode = EFKChainTranslationMode(0);
    pub const GLOBALLY_SCALED: EFKChainTranslationMode = EFKChainTranslationMode(1);
    pub const ABSOLUTE: EFKChainTranslationMode = EFKChainTranslationMode(2);
    pub const STRETCH_BONE_LENGTH_UNIFORMLY: EFKChainTranslationMode = EFKChainTranslationMode(
        3,
    );
    pub const STRETCH_BONE_LENGTH_NON_UNIFORMLY: EFKChainTranslationMode = EFKChainTranslationMode(
        4,
    );
    pub const ORIENT_AND_SCALE: EFKChainTranslationMode = EFKChainTranslationMode(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERetargetSourceOrTarget(pub u8);
impl ERetargetSourceOrTarget {
    pub const SOURCE: ERetargetSourceOrTarget = ERetargetSourceOrTarget(0);
    pub const TARGET: ERetargetSourceOrTarget = ERetargetSourceOrTarget(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPinBoneTranslationMode(pub u8);
impl EPinBoneTranslationMode {
    pub const COPY_GLOBAL_POSITION: EPinBoneTranslationMode = EPinBoneTranslationMode(0);
    pub const COPY_LOCAL_POSITION: EPinBoneTranslationMode = EPinBoneTranslationMode(1);
    pub const COPY_LOCAL_POSITION_RELATIVE_OFFSET: EPinBoneTranslationMode = EPinBoneTranslationMode(
        2,
    );
    pub const COPY_LOCAL_POSITION_RELATIVE_SCALED: EPinBoneTranslationMode = EPinBoneTranslationMode(
        3,
    );
    pub const COPY_GLOBAL_POSITION_AND_MAINTAIN_OFFSET: EPinBoneTranslationMode = EPinBoneTranslationMode(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPinBoneRotationMode(pub u8);
impl EPinBoneRotationMode {
    pub const COPY_GLOBAL_ROTATION: EPinBoneRotationMode = EPinBoneRotationMode(0);
    pub const MAINTAIN_OFFSET_FROM_BONE_TO_COPY_FROM: EPinBoneRotationMode = EPinBoneRotationMode(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERootMotionSource(pub u8);
impl ERootMotionSource {
    pub const COPY_FROM_SOURCE_ROOT: ERootMotionSource = ERootMotionSource(0);
    pub const GENERATE_FROM_TARGET_PELVIS: ERootMotionSource = ERootMotionSource(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERootMotionHeightSource(pub u8);
impl ERootMotionHeightSource {
    pub const COPY_HEIGHT_FROM_SOURCE: ERootMotionHeightSource = ERootMotionHeightSource(
        0,
    );
    pub const SNAP_TO_GROUND: ERootMotionHeightSource = ERootMotionHeightSource(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EScaleSourcePivot(pub u8);
impl EScaleSourcePivot {
    pub const COMPONENT_ORIGIN: EScaleSourcePivot = EScaleSourcePivot(0);
    pub const BONE: EScaleSourcePivot = EScaleSourcePivot(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStretchLimbRotationMode(pub u8);
impl EStretchLimbRotationMode {
    pub const NONE: EStretchLimbRotationMode = EStretchLimbRotationMode(0);
    pub const ORIENT_TO_GOAL: EStretchLimbRotationMode = EStretchLimbRotationMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStretchLimbSquashMode(pub u8);
impl EStretchLimbSquashMode {
    pub const NONE: EStretchLimbSquashMode = EStretchLimbSquashMode(0);
    pub const UNIFORM: EStretchLimbSquashMode = EStretchLimbSquashMode(1);
    pub const BULGE: EStretchLimbSquashMode = EStretchLimbSquashMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAutoMapChainType(pub u8);
impl EAutoMapChainType {
    pub const EXACT: EAutoMapChainType = EAutoMapChainType(0);
    pub const FUZZY: EAutoMapChainType = EAutoMapChainType(1);
    pub const CLEAR: EAutoMapChainType = EAutoMapChainType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPinBoneType(pub u8);
impl EPinBoneType {
    pub const FULL_TRANSFORM: EPinBoneType = EPinBoneType(0);
    pub const TRANSLATE_ONLY: EPinBoneType = EPinBoneType(1);
    pub const ROTATE_ONLY: EPinBoneType = EPinBoneType(2);
    pub const SCALE_ONLY: EPinBoneType = EPinBoneType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EIKRigGoalPreviewMode(pub u8);
impl EIKRigGoalPreviewMode {
    pub const ADDITIVE: EIKRigGoalPreviewMode = EIKRigGoalPreviewMode(0);
    pub const ABSOLUTE: EIKRigGoalPreviewMode = EIKRigGoalPreviewMode(1);
}
