#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpaceGraphBase {
    pub x: f32,
    pub y: f32,
    pub group_name: FName,
    pub group_role: crate::bindings::engine::EAnimGroupRole,
    pub blend_space: UPtr<crate::bindings::engine::UBlendSpace>,
    pub sample_pose_links: TArray<crate::bindings::engine::FPoseLink>,
}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpaceGraph {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpaceSampleResult {}
#[repr(C, align(8))]
pub struct FAnimNode_SkeletalControlBase {
    pub component_pose: crate::bindings::engine::FComponentSpacePoseLink,
    pub lod_threshold: i32,
    pub actual_alpha: f32,
    pub alpha_input_type: crate::bindings::engine::EAnimAlphaInputType,
    pub b_alpha_bool_enabled: bool,
    pub alpha: f32,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub alpha_bool_blend: crate::bindings::engine::FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    pub validation_visual_warning_message: FText,
}
#[repr(C, align(8))]
pub struct FAnimNode_RotationMultiplier {
    pub target_bone: crate::bindings::engine::FBoneReference,
    pub source_bone: crate::bindings::engine::FBoneReference,
    pub multiplier: f32,
    pub rotation_axis_to_refer: crate::bindings::engine::EBoneAxis,
    pub b_is_additive: bool,
}
#[repr(C, align(16))]
pub struct FRotationRetargetingInfo {
    pub b_enabled: bool,
    pub source: crate::bindings::core_u_object::FTransform,
    pub target: crate::bindings::core_u_object::FTransform,
    pub rotation_component: ERotationComponent,
    pub twist_axis: crate::bindings::core_u_object::FVector,
    pub b_use_absolute_angle: bool,
    pub source_minimum: f32,
    pub source_maximum: f32,
    pub target_minimum: f32,
    pub target_maximum: f32,
    pub easing_type: EEasingFuncType,
    pub custom_curve: crate::bindings::engine::FRuntimeFloatCurve,
    pub b_flip_easing: bool,
    pub easing_weight: f32,
    pub b_clamp: bool,
}
#[repr(C, align(8))]
pub struct FPositionHistory {
    pub positions: TArray<crate::bindings::core_u_object::FVector>,
    pub range: f32,
}
#[repr(C, align(8))]
pub struct FAnimationStateResultReference {}
#[repr(C, align(8))]
pub struct FAnimationStateMachineReference {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpacePlayerBase {
    pub previous_blend_space: UPtr<crate::bindings::engine::UBlendSpace>,
}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpacePlayer {
    pub group_name: FName,
    pub group_role: crate::bindings::engine::EAnimGroupRole,
    pub b_override_position_when_joining_sync_group_as_leader: bool,
    pub method: crate::bindings::engine::EAnimSyncMethod,
    pub b_ignore_for_relevancy_test: bool,
    pub x: f32,
    pub y: f32,
    pub play_rate: f32,
    pub b_loop: bool,
    pub b_reset_play_time_when_blend_space_changes: bool,
    pub start_position: f32,
    pub blend_space: UPtr<crate::bindings::engine::UBlendSpace>,
}
#[repr(C, align(16))]
pub struct FAnimNode_AimOffsetLookAt {
    pub base_pose: crate::bindings::engine::FPoseLink,
    pub lod_threshold: i32,
    pub source_socket_name: FName,
    pub pivot_socket_name: FName,
    pub look_at_location: crate::bindings::core_u_object::FVector,
    pub socket_axis: crate::bindings::core_u_object::FVector,
    pub alpha: f32,
}
#[repr(C, align(8))]
pub struct FAnimNode_ApplyAdditive {
    pub base: crate::bindings::engine::FPoseLink,
    pub additive: crate::bindings::engine::FPoseLink,
    pub alpha: f32,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub lod_threshold: i32,
    pub alpha_bool_blend: crate::bindings::engine::FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    pub alpha_input_type: crate::bindings::engine::EAnimAlphaInputType,
    pub b_alpha_bool_enabled: bool,
}
#[repr(C, align(4))]
pub struct FBlendBoneByChannelEntry {
    pub source_bone: crate::bindings::engine::FBoneReference,
    pub target_bone: crate::bindings::engine::FBoneReference,
    pub b_blend_translation: bool,
    pub b_blend_rotation: bool,
    pub b_blend_scale: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_BlendBoneByChannel {
    pub a: crate::bindings::engine::FPoseLink,
    pub b: crate::bindings::engine::FPoseLink,
    pub bone_definitions: TArray<FBlendBoneByChannelEntry>,
    pub alpha: f32,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub transforms_space: crate::bindings::engine::EBoneControlSpace,
}
#[repr(C, align(8))]
pub struct FAnimNode_BlendListBase {
    pub blend_pose: TArray<crate::bindings::engine::FPoseLink>,
    pub blend_time: TArray<f32>,
    pub transition_type: EBlendListTransitionType,
    pub blend_type: crate::bindings::engine::EAlphaBlendOption,
    pub b_reset_child_on_activation_deprecated: bool,
    pub child_upate_mode: EBlendListChildUpdateMode,
    pub custom_blend_curve: UPtr<crate::bindings::engine::UCurveFloat>,
    pub blend_profile: UPtr<crate::bindings::engine::UBlendProfile>,
}
#[repr(C, align(8))]
pub struct FAnimNode_BlendListByBool {
    pub blend_profile_for_false: UPtr<crate::bindings::engine::UBlendProfile>,
    pub b_use_seperate_blend_profile_for_false: bool,
    pub b_active_value: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_BlendListByEnum {
    pub enum_to_pose_index: TArray<i32>,
    pub active_enum_value: u8,
}
#[repr(C, align(8))]
pub struct FAnimNode_BlendListByInt {
    pub active_child_index: i32,
}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpaceEvaluator {
    pub normalized_time: f32,
    pub b_teleport_to_normalized_time: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpacePlayer_Standalone {
    pub group_name: FName,
    pub group_role: crate::bindings::engine::EAnimGroupRole,
    pub b_override_position_when_joining_sync_group_as_leader: bool,
    pub method: crate::bindings::engine::EAnimSyncMethod,
    pub b_ignore_for_relevancy_test: bool,
    pub x: f32,
    pub y: f32,
    pub play_rate: f32,
    pub b_loop: bool,
    pub b_reset_play_time_when_blend_space_changes: bool,
    pub start_position: f32,
    pub blend_space: UPtr<crate::bindings::engine::UBlendSpace>,
}
#[repr(C, align(8))]
pub struct FAnimNode_CallFunction {
    pub source: crate::bindings::engine::FPoseLink,
    pub function: crate::bindings::engine::FAnimNodeFunctionRef,
    pub call_site: EAnimFunctionCallSite,
}
#[repr(C, align(8))]
pub struct FAnimNode_CopyPoseFromMesh {
    pub source_mesh_component: TWeakObjectPtr<
        crate::bindings::engine::USkeletalMeshComponent,
    >,
    pub flags_144: u8,
    pub b_copy_custom_attributes: bool,
    pub flags_146: u8,
    pub root_bone_to_copy: FName,
}
#[repr(C, align(8))]
pub struct FAnimNode_CurveSource {
    pub source_pose: crate::bindings::engine::FPoseLink,
    pub source_binding: FName,
    pub alpha: f32,
    pub curve_source: TScriptInterface<crate::bindings::engine::ICurveSourceInterface>,
}
#[repr(C, align(8))]
pub struct FAnimNode_LayeredBoneBlend {
    pub base_pose: crate::bindings::engine::FPoseLink,
    pub blend_poses: TArray<crate::bindings::engine::FPoseLink>,
    pub blend_mode: ELayeredBoneBlendMode,
    pub blend_masks: TArray<UPtr<crate::bindings::engine::UBlendProfile>>,
    pub layer_setup: TArray<crate::bindings::engine::FInputBlendPose>,
    pub blend_weights: TArray<f32>,
    pub per_bone_blend_weights: TArray<crate::bindings::engine::FPerBoneBlendWeight>,
    pub skeleton_guid: crate::bindings::core_u_object::FGuid,
    pub virtual_bone_guid: crate::bindings::core_u_object::FGuid,
    pub lod_threshold: i32,
    pub b_mesh_space_rotation_blend: bool,
    pub b_root_space_rotation_blend: bool,
    pub b_mesh_space_scale_blend: bool,
    pub curve_blend_option: crate::bindings::engine::ECurveBlendOption,
    pub b_blend_root_motion_based_on_root_bone: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_MakeDynamicAdditive {
    pub base: crate::bindings::engine::FPoseLink,
    pub additive: crate::bindings::engine::FPoseLink,
    pub b_mesh_space_additive: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_MirrorBase {
    pub source: crate::bindings::engine::FPoseLink,
}
#[repr(C, align(8))]
pub struct FAnimNode_Mirror {
    pub b_mirror: bool,
    pub mirror_data_table: UPtr<crate::bindings::engine::UMirrorDataTable>,
    pub blend_time: f32,
    pub b_reset_child: bool,
    pub b_bone_mirroring: bool,
    pub b_curve_mirroring: bool,
    pub b_attribute_mirroring: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_Mirror_Standalone {
    pub b_mirror: bool,
    pub mirror_data_table: UPtr<crate::bindings::engine::UMirrorDataTable>,
    pub blend_time: f32,
    pub b_reset_child: bool,
    pub b_bone_mirroring: bool,
    pub b_curve_mirroring: bool,
    pub b_attribute_mirroring: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_ModifyCurve {
    pub source_pose: crate::bindings::engine::FPoseLink,
    pub curve_map: TMap<FName, f32>,
    pub curve_values: TArray<f32>,
    pub curve_names: TArray<FName>,
    pub alpha: f32,
    pub apply_mode: EModifyCurveApplyMode,
}
#[repr(C, align(8))]
pub struct FAnimNode_MultiWayBlend {
    pub poses: TArray<crate::bindings::engine::FPoseLink>,
    pub desired_alphas: TArray<f32>,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub b_additive_node: bool,
    pub b_normalize_alpha: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_PoseHandler {
    pub pose_asset: UPtr<crate::bindings::engine::UPoseAsset>,
}
#[repr(C, align(8))]
pub struct FAnimNode_PoseBlendNode {
    pub source_pose: crate::bindings::engine::FPoseLink,
    pub blend_option: crate::bindings::engine::EAlphaBlendOption,
    pub custom_curve: UPtr<crate::bindings::engine::UCurveFloat>,
}
#[repr(C, align(8))]
pub struct FAnimNode_PoseByName {
    pub pose_name: FName,
    pub pose_weight: f32,
}
#[repr(C, align(8))]
pub struct FPoseDriverTransform {
    pub target_translation: crate::bindings::core_u_object::FVector,
    pub target_rotation: crate::bindings::core_u_object::FRotator,
}
#[repr(C, align(8))]
pub struct FPoseDriverTarget {
    pub bone_transforms: TArray<FPoseDriverTransform>,
    pub target_rotation: crate::bindings::core_u_object::FRotator,
    pub target_scale: f32,
    pub distance_method: ERBFDistanceMethod,
    pub function_type: ERBFFunctionType,
    pub b_apply_custom_curve: bool,
    pub custom_curve: crate::bindings::engine::FRichCurve,
    pub driven_name: FName,
    pub b_is_hidden: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_PoseDriver {
    pub source_pose: crate::bindings::engine::FPoseLink,
    pub source_bones: TArray<crate::bindings::engine::FBoneReference>,
    pub eval_space_bone: crate::bindings::engine::FBoneReference,
    pub b_eval_from_ref_pose: bool,
    pub only_drive_bones: TArray<crate::bindings::engine::FBoneReference>,
    pub pose_targets: TArray<FPoseDriverTarget>,
    pub rbf_params: FRBFParams,
    pub drive_source: EPoseDriverSource,
    pub drive_output: EPoseDriverOutput,
    pub source_bone_deprecated: crate::bindings::engine::FBoneReference,
    pub twist_axis_deprecated: crate::bindings::engine::EBoneAxis,
    pub ty_deprecated: EPoseDriverType,
    pub radial_scaling_deprecated: f32,
    pub b_solo_driven_only: bool,
    pub lod_threshold: i32,
}
#[repr(C, align(8))]
pub struct FRBFParams {
    pub target_dimensions: i32,
    pub solver_type: ERBFSolverType,
    pub radius: f32,
    pub b_automatic_radius: bool,
    pub function: ERBFFunctionType,
    pub distance_method: ERBFDistanceMethod,
    pub twist_axis: crate::bindings::engine::EBoneAxis,
    pub weight_threshold: f32,
    pub normalize_method: ERBFNormalizeMethod,
    pub median_reference: crate::bindings::core_u_object::FVector,
    pub median_min: f32,
    pub median_max: f32,
}
#[repr(C, align(8))]
pub struct FAnimNode_PoseSnapshot {
    pub snapshot_name: FName,
    pub snapshot: crate::bindings::engine::FPoseSnapshot,
    pub mode: ESnapshotSourceMode,
}
#[repr(C, align(8))]
pub struct FRandomPlayerSequenceEntry {
    pub sequence: UPtr<crate::bindings::engine::UAnimSequenceBase>,
    pub chance_to_play: f32,
    pub min_loop_count: i32,
    pub max_loop_count: i32,
    pub min_play_rate: f32,
    pub max_play_rate: f32,
    pub blend_in: crate::bindings::engine::FAlphaBlend,
}
#[repr(C, align(8))]
pub struct FAnimNode_RandomPlayer {
    pub entries: TArray<FRandomPlayerSequenceEntry>,
    pub b_ignore_for_relevancy_test: bool,
    pub blend_weight: f32,
    pub b_shuffle_mode: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_RefPose {
    pub ref_pose_type: ERefPoseType,
}
#[repr(C, align(8))]
pub struct FAnimNode_MeshSpaceRefPose {}
#[repr(C, align(8))]
pub struct FAnimNode_RotateRootBone {
    pub base_pose: crate::bindings::engine::FPoseLink,
    pub pitch: f32,
    pub yaw: f32,
    pub pitch_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    pub yaw_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    pub mesh_to_component: crate::bindings::core_u_object::FRotator,
    pub b_rotate_root_motion_attribute: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_RotationOffsetBlendSpace {
    pub base_pose: crate::bindings::engine::FPoseLink,
    pub lod_threshold: i32,
    pub alpha: f32,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub alpha_bool_blend: crate::bindings::engine::FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    pub alpha_input_type: crate::bindings::engine::EAnimAlphaInputType,
    pub b_alpha_bool_enabled: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_RotationOffsetBlendSpaceGraph {
    pub base_pose: crate::bindings::engine::FPoseLink,
    pub lod_threshold: i32,
    pub alpha: f32,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub alpha_bool_blend: crate::bindings::engine::FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    pub alpha_input_type: crate::bindings::engine::EAnimAlphaInputType,
    pub b_alpha_bool_enabled: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_SequenceEvaluatorBase {}
#[repr(C, align(8))]
pub struct FAnimNode_SequenceEvaluator {
    pub group_name: FName,
    pub group_role: crate::bindings::engine::EAnimGroupRole,
    pub method: crate::bindings::engine::EAnimSyncMethod,
    pub b_ignore_for_relevancy_test: bool,
    pub sequence: UPtr<crate::bindings::engine::UAnimSequenceBase>,
    pub explicit_time: f32,
    pub b_use_explicit_frame: bool,
    pub explicit_frame: i32,
    pub b_should_loop: bool,
    pub b_teleport_to_explicit_time: bool,
    pub reinitialization_behavior: ESequenceEvalReinit,
    pub start_position: f32,
}
#[repr(C, align(8))]
pub struct FAnimNode_SequenceEvaluator_Standalone {
    pub group_name: FName,
    pub group_role: crate::bindings::engine::EAnimGroupRole,
    pub method: crate::bindings::engine::EAnimSyncMethod,
    pub b_ignore_for_relevancy_test: bool,
    pub sequence: UPtr<crate::bindings::engine::UAnimSequenceBase>,
    pub explicit_time: f32,
    pub b_use_explicit_frame: bool,
    pub explicit_frame: i32,
    pub b_should_loop: bool,
    pub b_teleport_to_explicit_time: bool,
    pub reinitialization_behavior: ESequenceEvalReinit,
    pub start_position: f32,
}
#[repr(C, align(8))]
pub struct FAnimNode_Slot {
    pub source: crate::bindings::engine::FPoseLink,
    pub slot_name: FName,
    pub b_always_update_source_pose: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_Sync {
    pub source: crate::bindings::engine::FPoseLink,
    pub group_name: FName,
    pub group_role: crate::bindings::engine::EAnimGroupRole,
}
#[repr(C, align(8))]
pub struct FAnimNode_TwoWayBlend {
    pub a: crate::bindings::engine::FPoseLink,
    pub b: crate::bindings::engine::FPoseLink,
    pub alpha_input_type: crate::bindings::engine::EAnimAlphaInputType,
    pub flags_185: u8,
    pub alpha: f32,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub alpha_bool_blend: crate::bindings::engine::FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
}
#[repr(C, align(16))]
pub struct FAnimSequencerInstanceProxy {}
#[repr(C, align(8))]
pub struct FBlendListBaseReference {}
#[repr(C, align(8))]
pub struct FBlendSpaceReference {}
#[repr(C, align(8))]
pub struct FBlendSpacePlayerReference {}
#[repr(C, align(8))]
pub struct FAnimPhysConstraintSetup {
    pub linear_x_limit_type: AnimPhysLinearConstraintType,
    pub linear_y_limit_type: AnimPhysLinearConstraintType,
    pub linear_z_limit_type: AnimPhysLinearConstraintType,
    pub linear_axes_min: crate::bindings::core_u_object::FVector,
    pub linear_axes_max: crate::bindings::core_u_object::FVector,
    pub angular_constraint_type: AnimPhysAngularConstraintType,
    pub twist_axis: crate::bindings::engine::AnimPhysTwistAxis,
    pub angular_target_axis: crate::bindings::engine::AnimPhysTwistAxis,
    pub cone_angle: f32,
    pub angular_x_angle_deprecated: f32,
    pub angular_y_angle_deprecated: f32,
    pub angular_z_angle_deprecated: f32,
    pub angular_limits_min: crate::bindings::core_u_object::FVector,
    pub angular_limits_max: crate::bindings::core_u_object::FVector,
    pub angular_target: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FAnimPhysPlanarLimit {
    pub driving_bone: crate::bindings::engine::FBoneReference,
    pub plane_transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FAnimPhysSphericalLimit {
    pub driving_bone: crate::bindings::engine::FBoneReference,
    pub sphere_local_offset: crate::bindings::core_u_object::FVector,
    pub limit_radius: f32,
    pub limit_type: ESphericalLimitType,
}
#[repr(C, align(8))]
pub struct FAnimPhysBodyDefinition {
    pub bound_bone: crate::bindings::engine::FBoneReference,
    pub box_extents: crate::bindings::core_u_object::FVector,
    pub local_joint_offset: crate::bindings::core_u_object::FVector,
    pub constraint_setup: FAnimPhysConstraintSetup,
    pub collision_type: crate::bindings::engine::AnimPhysCollisionType,
    pub sphere_collision_radius: f32,
}
#[repr(C, align(8))]
pub struct FAnimPhysSimSpaceSettings {
    pub sim_space_angular_alpha: f32,
    pub max_angular_velocity: f32,
    pub max_angular_acceleration: f32,
    pub external_angular_velocity: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FAnimNode_AnimDynamics {
    pub linear_damping_override: f32,
    pub angular_damping_override: f32,
    pub relative_space_bone: crate::bindings::engine::FBoneReference,
    pub bound_bone: crate::bindings::engine::FBoneReference,
    pub chain_end: crate::bindings::engine::FBoneReference,
    pub physics_body_definitions: TArray<FAnimPhysBodyDefinition>,
    pub gravity_scale: f32,
    pub gravity_override: crate::bindings::core_u_object::FVector,
    pub linear_spring_constant: f32,
    pub angular_spring_constant: f32,
    pub wind_scale: f32,
    pub component_linear_acc_scale: crate::bindings::core_u_object::FVector,
    pub component_linear_vel_scale: crate::bindings::core_u_object::FVector,
    pub component_applied_linear_acc_clamp: crate::bindings::core_u_object::FVector,
    pub sim_space_settings: FAnimPhysSimSpaceSettings,
    pub angular_bias_override: f32,
    pub num_solver_iterations_pre_update: i32,
    pub num_solver_iterations_post_update: i32,
    pub spherical_limits: TArray<FAnimPhysSphericalLimit>,
    pub external_force: crate::bindings::core_u_object::FVector,
    pub planar_limits: TArray<FAnimPhysPlanarLimit>,
    pub simulation_space: AnimPhysSimSpaceType,
    pub flags_1035: u8,
    pub flags_1036: u8,
    pub retargeting_settings: FRotationRetargetingInfo,
    pub box_extents_deprecated: crate::bindings::core_u_object::FVector,
    pub local_joint_offset_deprecated: crate::bindings::core_u_object::FVector,
    pub constraint_setup_deprecated: FAnimPhysConstraintSetup,
    pub collision_type_deprecated: crate::bindings::engine::AnimPhysCollisionType,
    pub sphere_collision_radius_deprecated: f32,
}
#[repr(C, align(8))]
pub struct FAngularRangeLimit {
    pub limit_min: crate::bindings::core_u_object::FVector,
    pub limit_max: crate::bindings::core_u_object::FVector,
    pub bone: crate::bindings::engine::FBoneReference,
}
#[repr(C, align(8))]
pub struct FAnimNode_ApplyLimits {
    pub angular_range_limits: TArray<FAngularRangeLimit>,
    pub angular_offsets: TArray<crate::bindings::core_u_object::FVector>,
}
#[repr(C, align(8))]
pub struct FAnimNode_BoneDrivenController {
    pub source_bone: crate::bindings::engine::FBoneReference,
    pub driving_curve: UPtr<crate::bindings::engine::UCurveFloat>,
    pub multiplier: f32,
    pub range_min: f64,
    pub range_max: f64,
    pub remapped_min: f64,
    pub remapped_max: f64,
    pub parameter_name: FName,
    pub target_bone: crate::bindings::engine::FBoneReference,
    pub target_component_deprecated: crate::bindings::engine::EComponentType,
    pub destination_mode: EDrivenDestinationMode,
    pub modification_mode: EDrivenBoneModificationMode,
    pub source_component: crate::bindings::engine::EComponentType,
    pub flags_532: u8,
    pub flags_533: u8,
}
#[repr(C, align(16))]
pub struct FAnimNode_CCDIK {
    pub effector_location: crate::bindings::core_u_object::FVector,
    pub effector_location_space: crate::bindings::engine::EBoneControlSpace,
    pub effector_target: crate::bindings::engine::FBoneSocketTarget,
    pub tip_bone: crate::bindings::engine::FBoneReference,
    pub root_bone: crate::bindings::engine::FBoneReference,
    pub precision: f32,
    pub max_iterations: i32,
    pub b_start_from_tail: bool,
    pub b_enable_rotation_limit: bool,
    pub rotation_limit_per_joints: TArray<f32>,
}
#[repr(C, align(4))]
pub struct FConstraint {
    pub target_bone: crate::bindings::engine::FBoneReference,
    pub offset_option: EConstraintOffsetOption,
    pub transform_type: crate::bindings::animation_core::ETransformConstraintType,
    pub per_axis: crate::bindings::animation_core::FFilterOptionPerAxis,
}
#[repr(C, align(16))]
pub struct FAnimNode_Constraint {
    pub bone_to_modify: crate::bindings::engine::FBoneReference,
    pub constraint_setup: TArray<FConstraint>,
    pub constraint_weights: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FAnimNode_CopyBone {
    pub source_bone: crate::bindings::engine::FBoneReference,
    pub target_bone: crate::bindings::engine::FBoneReference,
    pub b_copy_translation: bool,
    pub b_copy_rotation: bool,
    pub b_copy_scale: bool,
    pub control_space: crate::bindings::engine::EBoneControlSpace,
}
#[repr(C, align(8))]
pub struct FAnimNode_CopyBoneDelta {
    pub source_bone: crate::bindings::engine::FBoneReference,
    pub target_bone: crate::bindings::engine::FBoneReference,
    pub b_copy_translation: bool,
    pub b_copy_rotation: bool,
    pub b_copy_scale: bool,
    pub copy_mode: CopyBoneDeltaMode,
    pub translation_multiplier: f32,
    pub rotation_multiplier: f32,
    pub scale_multiplier: f32,
}
#[repr(C, align(16))]
pub struct FAnimNode_Fabrik {
    pub effector_transform: crate::bindings::core_u_object::FTransform,
    pub effector_target: crate::bindings::engine::FBoneSocketTarget,
    pub tip_bone: crate::bindings::engine::FBoneReference,
    pub root_bone: crate::bindings::engine::FBoneReference,
    pub precision: f32,
    pub max_iterations: i32,
    pub effector_transform_space: crate::bindings::engine::EBoneControlSpace,
    pub effector_rotation_source: crate::bindings::engine::EBoneRotationSource,
    pub b_enable_debug_draw: bool,
    pub effector_transform_bone_deprecated: crate::bindings::engine::FBoneReference,
}
#[repr(C, align(8))]
pub struct FAnimNode_HandIKRetargeting {
    pub right_hand_fk: crate::bindings::engine::FBoneReference,
    pub left_hand_fk: crate::bindings::engine::FBoneReference,
    pub right_hand_ik: crate::bindings::engine::FBoneReference,
    pub left_hand_ik: crate::bindings::engine::FBoneReference,
    pub ik_bones_to_move: TArray<crate::bindings::engine::FBoneReference>,
    pub per_axis_alpha: crate::bindings::core_u_object::FVector,
    pub hand_fk_weight: f32,
}
#[repr(C, align(8))]
pub struct FIKChainLink {}
#[repr(C, align(8))]
pub struct FIKChain {}
#[repr(C, align(4))]
pub struct FAnimLegIKDefinition {
    pub ik_foot_bone: crate::bindings::engine::FBoneReference,
    pub fk_foot_bone: crate::bindings::engine::FBoneReference,
    pub num_bones_in_limb: i32,
    pub min_rotation_angle: f32,
    pub foot_bone_forward_axis: crate::bindings::core_u_object::EAxis,
    pub hinge_rotation_axis: crate::bindings::core_u_object::EAxis,
    pub b_enable_rotation_limit: bool,
    pub b_enable_knee_twist_correction: bool,
    pub twist_offset_curve_name: FName,
}
#[repr(C, align(16))]
pub struct FAnimLegIKData {}
#[repr(C, align(8))]
pub struct FAnimNode_LegIK {
    pub reach_precision: f32,
    pub max_iterations: i32,
    pub soft_percent_length: f32,
    pub soft_alpha: f32,
    pub legs_definition: TArray<FAnimLegIKDefinition>,
}
#[repr(C, align(16))]
pub struct FAnimNode_LookAt {
    pub bone_to_modify: crate::bindings::engine::FBoneReference,
    pub look_at_target: crate::bindings::engine::FBoneSocketTarget,
    pub look_at_location: crate::bindings::core_u_object::FVector,
    pub look_at_axis: crate::bindings::animation_core::FAxis,
    pub b_use_look_up_axis: bool,
    pub interpolation_type: EInterpolationBlend,
    pub look_up_axis: crate::bindings::animation_core::FAxis,
    pub look_at_clamp: f32,
    pub interpolation_time: f32,
    pub interpolation_trigger_threashold: f32,
    pub look_at_bone_deprecated: crate::bindings::engine::FBoneReference,
    pub look_at_socket_deprecated: FName,
    pub look_at_axis_deprecated: crate::bindings::engine::EAxisOption,
    pub custom_look_at_axis_deprecated: crate::bindings::core_u_object::FVector,
    pub look_up_axis_deprecated: crate::bindings::engine::EAxisOption,
    pub custom_look_up_axis_deprecated: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FAnimNode_ModifyBone {
    pub bone_to_modify: crate::bindings::engine::FBoneReference,
    pub translation: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub scale: crate::bindings::core_u_object::FVector,
    pub translation_mode: EBoneModificationMode,
    pub rotation_mode: EBoneModificationMode,
    pub scale_mode: EBoneModificationMode,
    pub translation_space: crate::bindings::engine::EBoneControlSpace,
    pub rotation_space: crate::bindings::engine::EBoneControlSpace,
    pub scale_space: crate::bindings::engine::EBoneControlSpace,
}
#[repr(C, align(8))]
pub struct FAnimNode_ObserveBone {
    pub bone_to_observe: crate::bindings::engine::FBoneReference,
    pub display_space: crate::bindings::engine::EBoneControlSpace,
    pub b_relative_to_ref_pose: bool,
    pub translation: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub scale: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FAnimNode_ResetRoot {}
#[repr(C, align(8))]
pub struct FSimSpaceSettings {
    pub world_alpha: f32,
    pub velocity_scale_z: f32,
    pub damping_alpha: f32,
    pub max_linear_velocity: f32,
    pub max_angular_velocity: f32,
    pub max_linear_acceleration: f32,
    pub max_angular_acceleration: f32,
    pub external_linear_drag_deprecated: f32,
    pub external_linear_drag_v: crate::bindings::core_u_object::FVector,
    pub external_linear_velocity: crate::bindings::core_u_object::FVector,
    pub external_angular_velocity: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FAnimNode_RigidBody {
    pub override_physics_asset: UPtr<crate::bindings::engine::UPhysicsAsset>,
    pub b_default_to_skeletal_mesh_physics_asset: bool,
    pub b_use_default_as_simulated: bool,
    pub b_use_local_lod_threshold_only: bool,
    pub override_world_gravity: crate::bindings::core_u_object::FVector,
    pub external_force: crate::bindings::core_u_object::FVector,
    pub component_linear_acc_scale: crate::bindings::core_u_object::FVector,
    pub component_linear_vel_scale: crate::bindings::core_u_object::FVector,
    pub component_applied_linear_acc_clamp: crate::bindings::core_u_object::FVector,
    pub sim_space_settings: FSimSpaceSettings,
    pub cached_bounds_scale: f32,
    pub base_bone_ref: crate::bindings::engine::FBoneReference,
    pub overlap_channel: crate::bindings::engine::ECollisionChannel,
    pub simulation_space: ESimulationSpace,
    pub b_force_disable_collision_between_constraint_bodies: bool,
    pub b_use_external_cloth_collision: bool,
    pub flags_1013: u8,
    pub world_space_minimum_scale: f32,
    pub evaluation_reset_time: f32,
    pub b_component_space_simulation_deprecated: bool,
    pub simulation_timing: ESimulationTiming,
}
#[repr(C, align(8))]
pub struct FRigidBodyAnimNodeReference {}
#[repr(C, align(8))]
pub struct FAnimNode_ScaleChainLength {
    pub input_pose: crate::bindings::engine::FPoseLink,
    pub default_chain_length: f32,
    pub chain_start_bone: crate::bindings::engine::FBoneReference,
    pub chain_end_bone: crate::bindings::engine::FBoneReference,
    pub target_location: crate::bindings::core_u_object::FVector,
    pub alpha: f32,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub chain_initial_length: EScaleChainInitialLength,
}
#[repr(C, align(4))]
pub struct FSplineIKCachedBoneData {
    pub bone: crate::bindings::engine::FBoneReference,
    pub ref_skeleton_index: i32,
}
#[repr(C, align(8))]
pub struct FAnimNode_SplineIK {
    pub start_bone: crate::bindings::engine::FBoneReference,
    pub end_bone: crate::bindings::engine::FBoneReference,
    pub bone_axis: ESplineBoneAxis,
    pub b_auto_calculate_spline: bool,
    pub point_count: i32,
    pub control_points: TArray<crate::bindings::core_u_object::FTransform>,
    pub roll: f32,
    pub twist_start: f32,
    pub twist_end: f32,
    pub twist_blend: crate::bindings::engine::FAlphaBlend,
    pub stretch: f32,
    pub offset: f32,
}
#[repr(C, align(8))]
pub struct FAnimNode_SpringBone {
    pub spring_bone: crate::bindings::engine::FBoneReference,
    pub max_displacement: f64,
    pub spring_stiffness: f64,
    pub spring_damping: f64,
    pub error_reset_thresh: f64,
    pub b_no_z_spring_deprecated: bool,
    pub flags_589: u8,
    pub flags_590: u8,
    pub owner_velocity_override: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRotationLimit {
    pub limit_min: crate::bindings::core_u_object::FVector,
    pub limit_max: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FAnimNode_Trail {
    pub trail_bone: crate::bindings::engine::FBoneReference,
    pub chain_length: i32,
    pub chain_bone_axis: crate::bindings::core_u_object::EAxis,
    pub flags_553: u8,
    pub flags_554: u8,
    pub debug_life_time: f32,
    pub trail_relaxation_deprecated: f32,
    pub max_delta_time: f32,
    pub relaxation_speed_scale: f32,
    pub trail_relaxation_speed: crate::bindings::engine::FRuntimeFloatCurve,
    pub relaxation_speed_scale_input_processor: crate::bindings::engine::FInputScaleBiasClamp,
    pub rotation_limits: TArray<FRotationLimit>,
    pub rotation_offsets: TArray<crate::bindings::core_u_object::FVector>,
    pub planar_limits: TArray<FAnimPhysPlanarLimit>,
    pub stretch_limit: f32,
    pub fake_velocity: crate::bindings::core_u_object::FVector,
    pub base_joint: crate::bindings::engine::FBoneReference,
    pub trail_bone_rotation_blend_alpha_deprecated: f32,
    pub last_bone_rotation_anim_alpha_blend: f32,
}
#[repr(C, align(8))]
pub struct FReferenceBoneFrame {
    pub bone: crate::bindings::engine::FBoneReference,
    pub axis: crate::bindings::animation_core::FAxis,
}
#[repr(C, align(8))]
pub struct FAnimNode_TwistCorrectiveNode {
    pub base_frame: FReferenceBoneFrame,
    pub twist_frame: FReferenceBoneFrame,
    pub twist_plane_normal_axis: crate::bindings::animation_core::FAxis,
    pub range_max: f32,
    pub remapped_min: f32,
    pub remapped_max: f32,
    pub curve_deprecated: crate::bindings::engine::FAnimCurveParam,
    pub curve_name: FName,
}
#[repr(C, align(16))]
pub struct FAnimNode_TwoBoneIK {
    pub ik_bone: crate::bindings::engine::FBoneReference,
    pub start_stretch_ratio: f64,
    pub max_stretch_scale: f64,
    pub stretch_limits_deprecated: crate::bindings::core_u_object::FVector2D,
    pub b_no_twist_deprecated: bool,
    pub joint_target_space_bone_name_deprecated: FName,
    pub effector_space_bone_name_deprecated: FName,
    pub effector_location: crate::bindings::core_u_object::FVector,
    pub effector_target: crate::bindings::engine::FBoneSocketTarget,
    pub joint_target_location: crate::bindings::core_u_object::FVector,
    pub joint_target: crate::bindings::engine::FBoneSocketTarget,
    pub twist_axis: crate::bindings::animation_core::FAxis,
    pub effector_location_space: crate::bindings::engine::EBoneControlSpace,
    pub joint_target_location_space: crate::bindings::engine::EBoneControlSpace,
    pub flags_930: u8,
}
#[repr(C, align(8))]
pub struct FIKFootPelvisPullDownSolver {
    pub pelvis_adjustment_interp: crate::bindings::engine::FVectorRK4SpringInterpolator,
    pub pelvis_adjustment_interp_alpha: f64,
    pub pelvis_adjustment_max_distance: f64,
    pub pelvis_adjustment_error_tolerance: f64,
    pub pelvis_adjustment_max_iter: i32,
}
#[repr(C, align(8))]
pub struct FWarpingVectorValue {
    pub mode: EWarpingVectorMode,
    pub value: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FLayeredBoneBlendReference {}
#[repr(C, align(8))]
pub struct FLinkedAnimGraphReference {}
#[repr(C, align(8))]
pub struct FMirrorAnimNodeReference {}
#[repr(C, align(8))]
pub struct FModifyCurveAnimNodeReference {}
#[repr(C, align(8))]
pub struct FRBFEntry {
    pub values: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FRBFTarget {
    pub scale_factor: f32,
    pub b_apply_custom_curve: bool,
    pub custom_curve: crate::bindings::engine::FRichCurve,
    pub distance_method: ERBFDistanceMethod,
    pub function_type: ERBFFunctionType,
}
#[repr(C, align(8))]
pub struct FSequenceEvaluatorReference {}
#[repr(C, align(8))]
pub struct FSequencePlayerReference {}
#[repr(C, align(8))]
pub struct FSkeletalControlReference {}
pub struct USequencerAnimationOverride {}
pub struct ISequencerAnimationOverride {}
pub struct UAnimationStateMachineLibrary {}
pub struct UAnimExecutionContextLibrary {}
pub struct UAnimNotify_PlayMontageNotify {
    pub notify_name: FName,
}
pub struct UAnimNotify_PlayMontageNotifyWindow {
    pub notify_name: FName,
}
pub struct UAnimSequencerInstance {}
pub struct UBlendListBaseLibrary {}
pub struct UBlendSpaceLibrary {}
pub struct UBlendSpacePlayerLibrary {}
pub struct UAnimNodeRigidBodyLibrary {}
pub struct UKismetAnimationLibrary {}
pub struct ULayeredBoneBlendLibrary {}
pub struct ULinkedAnimGraphLibrary {}
pub struct UMirrorAnimLibrary {}
pub struct UModifyCurveAnimLibrary {}
pub struct UPlayMontageCallbackProxy {
    pub on_completed: FPlayMontageCallbackProxy_OnCompleted,
    pub on_blend_out: FPlayMontageCallbackProxy_OnBlendOut,
    pub on_interrupted: FPlayMontageCallbackProxy_OnInterrupted,
    pub on_notify_begin: FPlayMontageCallbackProxy_OnNotifyBegin,
    pub on_notify_end: FPlayMontageCallbackProxy_OnNotifyEnd,
}
pub struct USequenceEvaluatorLibrary {}
pub struct USequencePlayerLibrary {}
pub struct USequencerAnimationSupport {}
pub struct ISequencerAnimationSupport {}
pub struct USkeletalControlLibrary {}
pub struct FPlayMontageCallbackProxy_OnCompleted;
pub struct FPlayMontageCallbackProxy_OnBlendOut;
pub struct FPlayMontageCallbackProxy_OnInterrupted;
pub struct FPlayMontageCallbackProxy_OnNotifyBegin;
pub struct FPlayMontageCallbackProxy_OnNotifyEnd;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERotationComponent(pub u8);
impl ERotationComponent {
    pub const EULER_X: ERotationComponent = ERotationComponent(0);
    pub const EULER_Y: ERotationComponent = ERotationComponent(1);
    pub const EULER_Z: ERotationComponent = ERotationComponent(2);
    pub const QUATERNION_ANGLE: ERotationComponent = ERotationComponent(3);
    pub const SWING_ANGLE: ERotationComponent = ERotationComponent(4);
    pub const TWIST_ANGLE: ERotationComponent = ERotationComponent(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EEasingFuncType(pub u8);
impl EEasingFuncType {
    pub const LINEAR: EEasingFuncType = EEasingFuncType(0);
    pub const SINUSOIDAL: EEasingFuncType = EEasingFuncType(1);
    pub const CUBIC: EEasingFuncType = EEasingFuncType(2);
    pub const QUADRATIC_IN_OUT: EEasingFuncType = EEasingFuncType(3);
    pub const CUBIC_IN_OUT: EEasingFuncType = EEasingFuncType(4);
    pub const HERMITE_CUBIC: EEasingFuncType = EEasingFuncType(5);
    pub const QUARTIC_IN_OUT: EEasingFuncType = EEasingFuncType(6);
    pub const QUINTIC_IN_OUT: EEasingFuncType = EEasingFuncType(7);
    pub const CIRCULAR_IN: EEasingFuncType = EEasingFuncType(8);
    pub const CIRCULAR_OUT: EEasingFuncType = EEasingFuncType(9);
    pub const CIRCULAR_IN_OUT: EEasingFuncType = EEasingFuncType(10);
    pub const EXP_IN: EEasingFuncType = EEasingFuncType(11);
    pub const EXP_OUT: EEasingFuncType = EEasingFuncType(12);
    pub const EXP_IN_OUT: EEasingFuncType = EEasingFuncType(13);
    pub const CUSTOM_CURVE: EEasingFuncType = EEasingFuncType(14);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBlendListTransitionType(pub u8);
impl EBlendListTransitionType {
    pub const STANDARD_BLEND: EBlendListTransitionType = EBlendListTransitionType(0);
    pub const INERTIALIZATION: EBlendListTransitionType = EBlendListTransitionType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBlendListChildUpdateMode(pub u8);
impl EBlendListChildUpdateMode {
    pub const DEFAULT: EBlendListChildUpdateMode = EBlendListChildUpdateMode(0);
    pub const RESET_CHILD_ON_ACTIVATE: EBlendListChildUpdateMode = EBlendListChildUpdateMode(
        1,
    );
    pub const ALWAYS_TICK_CHILDREN: EBlendListChildUpdateMode = EBlendListChildUpdateMode(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAnimFunctionCallSite(pub i32);
impl EAnimFunctionCallSite {
    pub const ON_INITIALIZE: EAnimFunctionCallSite = EAnimFunctionCallSite(0);
    pub const ON_UPDATE: EAnimFunctionCallSite = EAnimFunctionCallSite(1);
    pub const ON_BECOME_RELEVANT: EAnimFunctionCallSite = EAnimFunctionCallSite(2);
    pub const ON_EVALUATE: EAnimFunctionCallSite = EAnimFunctionCallSite(3);
    pub const ON_INITIALIZE_POST_RECURSION: EAnimFunctionCallSite = EAnimFunctionCallSite(
        4,
    );
    pub const ON_UPDATE_POST_RECURSION: EAnimFunctionCallSite = EAnimFunctionCallSite(5);
    pub const ON_BECOME_RELEVANT_POST_RECURSION: EAnimFunctionCallSite = EAnimFunctionCallSite(
        6,
    );
    pub const ON_EVALUATE_POST_RECURSION: EAnimFunctionCallSite = EAnimFunctionCallSite(
        7,
    );
    pub const ON_STARTED_BLENDING_OUT: EAnimFunctionCallSite = EAnimFunctionCallSite(8);
    pub const ON_STARTED_BLENDING_IN: EAnimFunctionCallSite = EAnimFunctionCallSite(9);
    pub const ON_FINISHED_BLENDING_OUT: EAnimFunctionCallSite = EAnimFunctionCallSite(
        10,
    );
    pub const ON_FINISHED_BLENDING_IN: EAnimFunctionCallSite = EAnimFunctionCallSite(11);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELayeredBoneBlendMode(pub u8);
impl ELayeredBoneBlendMode {
    pub const BRANCH_FILTER: ELayeredBoneBlendMode = ELayeredBoneBlendMode(0);
    pub const BLEND_MASK: ELayeredBoneBlendMode = ELayeredBoneBlendMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EModifyCurveApplyMode(pub u8);
impl EModifyCurveApplyMode {
    pub const ADD: EModifyCurveApplyMode = EModifyCurveApplyMode(0);
    pub const SCALE: EModifyCurveApplyMode = EModifyCurveApplyMode(1);
    pub const BLEND: EModifyCurveApplyMode = EModifyCurveApplyMode(2);
    pub const WEIGHTED_MOVING_AVERAGE: EModifyCurveApplyMode = EModifyCurveApplyMode(3);
    pub const REMAP_CURVE: EModifyCurveApplyMode = EModifyCurveApplyMode(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERBFDistanceMethod(pub u8);
impl ERBFDistanceMethod {
    pub const EUCLIDEAN: ERBFDistanceMethod = ERBFDistanceMethod(0);
    pub const QUATERNION: ERBFDistanceMethod = ERBFDistanceMethod(1);
    pub const SWING_ANGLE: ERBFDistanceMethod = ERBFDistanceMethod(2);
    pub const TWIST_ANGLE: ERBFDistanceMethod = ERBFDistanceMethod(3);
    pub const DEFAULT_METHOD: ERBFDistanceMethod = ERBFDistanceMethod(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERBFFunctionType(pub u8);
impl ERBFFunctionType {
    pub const GAUSSIAN: ERBFFunctionType = ERBFFunctionType(0);
    pub const EXPONENTIAL: ERBFFunctionType = ERBFFunctionType(1);
    pub const LINEAR: ERBFFunctionType = ERBFFunctionType(2);
    pub const CUBIC: ERBFFunctionType = ERBFFunctionType(3);
    pub const QUINTIC: ERBFFunctionType = ERBFFunctionType(4);
    pub const DEFAULT_FUNCTION: ERBFFunctionType = ERBFFunctionType(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERBFSolverType(pub u8);
impl ERBFSolverType {
    pub const ADDITIVE: ERBFSolverType = ERBFSolverType(0);
    pub const INTERPOLATIVE: ERBFSolverType = ERBFSolverType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPoseDriverSource(pub u8);
impl EPoseDriverSource {
    pub const ROTATION: EPoseDriverSource = EPoseDriverSource(0);
    pub const TRANSLATION: EPoseDriverSource = EPoseDriverSource(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPoseDriverOutput(pub u8);
impl EPoseDriverOutput {
    pub const DRIVE_POSES: EPoseDriverOutput = EPoseDriverOutput(0);
    pub const DRIVE_CURVES: EPoseDriverOutput = EPoseDriverOutput(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPoseDriverType(pub u8);
impl EPoseDriverType {
    pub const SWING_AND_TWIST: EPoseDriverType = EPoseDriverType(0);
    pub const SWING_ONLY: EPoseDriverType = EPoseDriverType(1);
    pub const TRANSLATION: EPoseDriverType = EPoseDriverType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERBFNormalizeMethod(pub u8);
impl ERBFNormalizeMethod {
    pub const ONLY_NORMALIZE_ABOVE_ONE: ERBFNormalizeMethod = ERBFNormalizeMethod(0);
    pub const ALWAYS_NORMALIZE: ERBFNormalizeMethod = ERBFNormalizeMethod(1);
    pub const NORMALIZE_WITHIN_MEDIAN: ERBFNormalizeMethod = ERBFNormalizeMethod(2);
    pub const NO_NORMALIZATION: ERBFNormalizeMethod = ERBFNormalizeMethod(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESnapshotSourceMode(pub u8);
impl ESnapshotSourceMode {
    pub const NAMED_SNAPSHOT: ESnapshotSourceMode = ESnapshotSourceMode(0);
    pub const SNAPSHOT_PIN: ESnapshotSourceMode = ESnapshotSourceMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERefPoseType(pub u8);
impl ERefPoseType {
    pub const EIT_LOCAL_SPACE: ERefPoseType = ERefPoseType(0);
    pub const EIT_ADDITIVE: ERefPoseType = ERefPoseType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESequenceEvalReinit(pub u8);
impl ESequenceEvalReinit {
    pub const NO_RESET: ESequenceEvalReinit = ESequenceEvalReinit(0);
    pub const START_POSITION: ESequenceEvalReinit = ESequenceEvalReinit(1);
    pub const EXPLICIT_TIME: ESequenceEvalReinit = ESequenceEvalReinit(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct AnimPhysLinearConstraintType(pub u8);
impl AnimPhysLinearConstraintType {
    pub const FREE: AnimPhysLinearConstraintType = AnimPhysLinearConstraintType(0);
    pub const LIMITED: AnimPhysLinearConstraintType = AnimPhysLinearConstraintType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct AnimPhysAngularConstraintType(pub u8);
impl AnimPhysAngularConstraintType {
    pub const ANGULAR: AnimPhysAngularConstraintType = AnimPhysAngularConstraintType(0);
    pub const CONE: AnimPhysAngularConstraintType = AnimPhysAngularConstraintType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESphericalLimitType(pub u8);
impl ESphericalLimitType {
    pub const INNER: ESphericalLimitType = ESphericalLimitType(0);
    pub const OUTER: ESphericalLimitType = ESphericalLimitType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct AnimPhysSimSpaceType(pub u8);
impl AnimPhysSimSpaceType {
    pub const COMPONENT: AnimPhysSimSpaceType = AnimPhysSimSpaceType(0);
    pub const ACTOR: AnimPhysSimSpaceType = AnimPhysSimSpaceType(1);
    pub const WORLD: AnimPhysSimSpaceType = AnimPhysSimSpaceType(2);
    pub const ROOT_RELATIVE: AnimPhysSimSpaceType = AnimPhysSimSpaceType(3);
    pub const BONE_RELATIVE: AnimPhysSimSpaceType = AnimPhysSimSpaceType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDrivenDestinationMode(pub u8);
impl EDrivenDestinationMode {
    pub const BONE: EDrivenDestinationMode = EDrivenDestinationMode(0);
    pub const MORPH_TARGET: EDrivenDestinationMode = EDrivenDestinationMode(1);
    pub const MATERIAL_PARAMETER: EDrivenDestinationMode = EDrivenDestinationMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDrivenBoneModificationMode(pub u8);
impl EDrivenBoneModificationMode {
    pub const ADD_TO_INPUT: EDrivenBoneModificationMode = EDrivenBoneModificationMode(0);
    pub const REPLACE_COMPONENT: EDrivenBoneModificationMode = EDrivenBoneModificationMode(
        1,
    );
    pub const ADD_TO_REF_POSE: EDrivenBoneModificationMode = EDrivenBoneModificationMode(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EConstraintOffsetOption(pub u8);
impl EConstraintOffsetOption {
    pub const NONE: EConstraintOffsetOption = EConstraintOffsetOption(0);
    pub const OFFSET_REF_POSE: EConstraintOffsetOption = EConstraintOffsetOption(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct CopyBoneDeltaMode(pub u8);
impl CopyBoneDeltaMode {
    pub const ACCUMULATE: CopyBoneDeltaMode = CopyBoneDeltaMode(0);
    pub const COPY: CopyBoneDeltaMode = CopyBoneDeltaMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInterpolationBlend(pub u8);
impl EInterpolationBlend {
    pub const LINEAR: EInterpolationBlend = EInterpolationBlend(0);
    pub const CUBIC: EInterpolationBlend = EInterpolationBlend(1);
    pub const SINUSOIDAL: EInterpolationBlend = EInterpolationBlend(2);
    pub const EASE_IN_OUT_EXPONENT2: EInterpolationBlend = EInterpolationBlend(3);
    pub const EASE_IN_OUT_EXPONENT3: EInterpolationBlend = EInterpolationBlend(4);
    pub const EASE_IN_OUT_EXPONENT4: EInterpolationBlend = EInterpolationBlend(5);
    pub const EASE_IN_OUT_EXPONENT5: EInterpolationBlend = EInterpolationBlend(6);
    pub const MAX: EInterpolationBlend = EInterpolationBlend(7);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBoneModificationMode(pub u8);
impl EBoneModificationMode {
    pub const BMM_IGNORE: EBoneModificationMode = EBoneModificationMode(0);
    pub const BMM_REPLACE: EBoneModificationMode = EBoneModificationMode(1);
    pub const BMM_ADDITIVE: EBoneModificationMode = EBoneModificationMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESimulationSpace(pub u8);
impl ESimulationSpace {
    pub const COMPONENT_SPACE: ESimulationSpace = ESimulationSpace(0);
    pub const WORLD_SPACE: ESimulationSpace = ESimulationSpace(1);
    pub const BASE_BONE_SPACE: ESimulationSpace = ESimulationSpace(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESimulationTiming(pub u8);
impl ESimulationTiming {
    pub const DEFAULT: ESimulationTiming = ESimulationTiming(0);
    pub const SYNCHRONOUS: ESimulationTiming = ESimulationTiming(1);
    pub const DEFERRED: ESimulationTiming = ESimulationTiming(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EScaleChainInitialLength(pub u8);
impl EScaleChainInitialLength {
    pub const FIXED_DEFAULT_LENGTH_VALUE: EScaleChainInitialLength = EScaleChainInitialLength(
        0,
    );
    pub const DISTANCE: EScaleChainInitialLength = EScaleChainInitialLength(1);
    pub const CHAIN_LENGTH: EScaleChainInitialLength = EScaleChainInitialLength(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESplineBoneAxis(pub u8);
impl ESplineBoneAxis {
    pub const NONE: ESplineBoneAxis = ESplineBoneAxis(0);
    pub const X: ESplineBoneAxis = ESplineBoneAxis(1);
    pub const Y: ESplineBoneAxis = ESplineBoneAxis(2);
    pub const Z: ESplineBoneAxis = ESplineBoneAxis(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWarpingVectorMode(pub u8);
impl EWarpingVectorMode {
    pub const COMPONENT_SPACE_VECTOR: EWarpingVectorMode = EWarpingVectorMode(0);
    pub const ACTOR_SPACE_VECTOR: EWarpingVectorMode = EWarpingVectorMode(1);
    pub const WORLD_SPACE_VECTOR: EWarpingVectorMode = EWarpingVectorMode(2);
    pub const IK_FOOT_ROOT_LOCAL_SPACE_VECTOR: EWarpingVectorMode = EWarpingVectorMode(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESwapRootBone(pub u8);
impl ESwapRootBone {
    pub const SWAP_ROOT_BONE_COMPONENT: ESwapRootBone = ESwapRootBone(0);
    pub const SWAP_ROOT_BONE_ACTOR: ESwapRootBone = ESwapRootBone(1);
    pub const SWAP_ROOT_BONE_NONE: ESwapRootBone = ESwapRootBone(2);
}
