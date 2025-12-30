#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpaceGraphBase {
    pub x: f32,
    pub y: f32,
    pub group_name: FName,
    pub group_role: EAnimGroupRole,
    pub blend_space: UPtr<UBlendSpace>,
    pub sample_pose_links: TArray<FPoseLink>,
}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpaceGraph {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpaceSampleResult {}
#[repr(C, align(8))]
pub struct FAnimNode_SkeletalControlBase {
    pub component_pose: FComponentSpacePoseLink,
    pub lod_threshold: i32,
    pub actual_alpha: f32,
    pub alpha_input_type: EAnimAlphaInputType,
    pub b_alpha_bool_enabled: bool,
    pub alpha: f32,
    pub alpha_scale_bias: FInputScaleBias,
    pub alpha_bool_blend: FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: FInputScaleBiasClamp,
    pub validation_visual_warning_message: FText,
}
#[repr(C, align(8))]
pub struct FAnimNode_RotationMultiplier {
    pub target_bone: FBoneReference,
    pub source_bone: FBoneReference,
    pub multiplier: f32,
    pub rotation_axis_to_refer: EBoneAxis,
    pub b_is_additive: bool,
}
#[repr(C, align(16))]
pub struct FRotationRetargetingInfo {
    pub b_enabled: bool,
    pub source: FTransform,
    pub target: FTransform,
    pub rotation_component: ERotationComponent,
    pub twist_axis: FVector,
    pub b_use_absolute_angle: bool,
    pub source_minimum: f32,
    pub source_maximum: f32,
    pub target_minimum: f32,
    pub target_maximum: f32,
    pub easing_type: EEasingFuncType,
    pub custom_curve: FRuntimeFloatCurve,
    pub b_flip_easing: bool,
    pub easing_weight: f32,
    pub b_clamp: bool,
}
#[repr(C, align(8))]
pub struct FPositionHistory {
    pub positions: TArray<FVector>,
    pub range: f32,
}
#[repr(C, align(8))]
pub struct FAnimationStateResultReference {}
#[repr(C, align(8))]
pub struct FAnimationStateMachineReference {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpacePlayerBase {
    pub previous_blend_space: UPtr<UBlendSpace>,
}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpacePlayer {
    pub group_name: FName,
    pub group_role: EAnimGroupRole,
    pub b_override_position_when_joining_sync_group_as_leader: bool,
    pub method: EAnimSyncMethod,
    pub b_ignore_for_relevancy_test: bool,
    pub x: f32,
    pub y: f32,
    pub play_rate: f32,
    pub b_loop: bool,
    pub b_reset_play_time_when_blend_space_changes: bool,
    pub start_position: f32,
    pub blend_space: UPtr<UBlendSpace>,
}
#[repr(C, align(16))]
pub struct FAnimNode_AimOffsetLookAt {
    pub base_pose: FPoseLink,
    pub lod_threshold: i32,
    pub source_socket_name: FName,
    pub pivot_socket_name: FName,
    pub look_at_location: FVector,
    pub socket_axis: FVector,
    pub alpha: f32,
}
#[repr(C, align(8))]
pub struct FAnimNode_ApplyAdditive {
    pub base: FPoseLink,
    pub additive: FPoseLink,
    pub alpha: f32,
    pub alpha_scale_bias: FInputScaleBias,
    pub lod_threshold: i32,
    pub alpha_bool_blend: FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: FInputScaleBiasClamp,
    pub alpha_input_type: EAnimAlphaInputType,
    pub b_alpha_bool_enabled: bool,
}
#[repr(C, align(4))]
pub struct FBlendBoneByChannelEntry {
    pub source_bone: FBoneReference,
    pub target_bone: FBoneReference,
    pub b_blend_translation: bool,
    pub b_blend_rotation: bool,
    pub b_blend_scale: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_BlendBoneByChannel {
    pub a: FPoseLink,
    pub b: FPoseLink,
    pub bone_definitions: TArray<FBlendBoneByChannelEntry>,
    pub alpha: f32,
    pub alpha_scale_bias: FInputScaleBias,
    pub transforms_space: EBoneControlSpace,
}
#[repr(C, align(8))]
pub struct FAnimNode_BlendListBase {
    pub blend_pose: TArray<FPoseLink>,
    pub blend_time: TArray<f32>,
    pub transition_type: EBlendListTransitionType,
    pub blend_type: EAlphaBlendOption,
    pub b_reset_child_on_activation_deprecated: bool,
    pub child_upate_mode: EBlendListChildUpdateMode,
    pub custom_blend_curve: UPtr<UCurveFloat>,
    pub blend_profile: UPtr<UBlendProfile>,
}
#[repr(C, align(8))]
pub struct FAnimNode_BlendListByBool {
    pub blend_profile_for_false: UPtr<UBlendProfile>,
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
    pub group_role: EAnimGroupRole,
    pub b_override_position_when_joining_sync_group_as_leader: bool,
    pub method: EAnimSyncMethod,
    pub b_ignore_for_relevancy_test: bool,
    pub x: f32,
    pub y: f32,
    pub play_rate: f32,
    pub b_loop: bool,
    pub b_reset_play_time_when_blend_space_changes: bool,
    pub start_position: f32,
    pub blend_space: UPtr<UBlendSpace>,
}
#[repr(C, align(8))]
pub struct FAnimNode_CallFunction {
    pub source: FPoseLink,
    pub function: FAnimNodeFunctionRef,
    pub call_site: EAnimFunctionCallSite,
}
#[repr(C, align(8))]
pub struct FAnimNode_CopyPoseFromMesh {
    pub source_mesh_component: TWeakObjectPtr<USkeletalMeshComponent>,
    pub flags_144: u8,
    pub b_copy_custom_attributes: bool,
    pub flags_146: u8,
    pub root_bone_to_copy: FName,
}
#[repr(C, align(8))]
pub struct FAnimNode_CurveSource {
    pub source_pose: FPoseLink,
    pub source_binding: FName,
    pub alpha: f32,
    pub curve_source: TScriptInterface<ICurveSourceInterface>,
}
#[repr(C, align(8))]
pub struct FAnimNode_LayeredBoneBlend {
    pub base_pose: FPoseLink,
    pub blend_poses: TArray<FPoseLink>,
    pub blend_mode: ELayeredBoneBlendMode,
    pub blend_masks: TArray<UPtr<UBlendProfile>>,
    pub layer_setup: TArray<FInputBlendPose>,
    pub blend_weights: TArray<f32>,
    pub per_bone_blend_weights: TArray<FPerBoneBlendWeight>,
    pub skeleton_guid: FGuid,
    pub virtual_bone_guid: FGuid,
    pub lod_threshold: i32,
    pub b_mesh_space_rotation_blend: bool,
    pub b_root_space_rotation_blend: bool,
    pub b_mesh_space_scale_blend: bool,
    pub curve_blend_option: ECurveBlendOption,
    pub b_blend_root_motion_based_on_root_bone: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_MakeDynamicAdditive {
    pub base: FPoseLink,
    pub additive: FPoseLink,
    pub b_mesh_space_additive: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_MirrorBase {
    pub source: FPoseLink,
}
#[repr(C, align(8))]
pub struct FAnimNode_Mirror {
    pub b_mirror: bool,
    pub mirror_data_table: UPtr<UMirrorDataTable>,
    pub blend_time: f32,
    pub b_reset_child: bool,
    pub b_bone_mirroring: bool,
    pub b_curve_mirroring: bool,
    pub b_attribute_mirroring: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_Mirror_Standalone {
    pub b_mirror: bool,
    pub mirror_data_table: UPtr<UMirrorDataTable>,
    pub blend_time: f32,
    pub b_reset_child: bool,
    pub b_bone_mirroring: bool,
    pub b_curve_mirroring: bool,
    pub b_attribute_mirroring: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_ModifyCurve {
    pub source_pose: FPoseLink,
    pub curve_map: TMap<FName, f32>,
    pub curve_values: TArray<f32>,
    pub curve_names: TArray<FName>,
    pub alpha: f32,
    pub apply_mode: EModifyCurveApplyMode,
}
#[repr(C, align(8))]
pub struct FAnimNode_MultiWayBlend {
    pub poses: TArray<FPoseLink>,
    pub desired_alphas: TArray<f32>,
    pub alpha_scale_bias: FInputScaleBias,
    pub b_additive_node: bool,
    pub b_normalize_alpha: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_PoseHandler {
    pub pose_asset: UPtr<UPoseAsset>,
}
#[repr(C, align(8))]
pub struct FAnimNode_PoseBlendNode {
    pub source_pose: FPoseLink,
    pub blend_option: EAlphaBlendOption,
    pub custom_curve: UPtr<UCurveFloat>,
}
#[repr(C, align(8))]
pub struct FAnimNode_PoseByName {
    pub pose_name: FName,
    pub pose_weight: f32,
}
#[repr(C, align(8))]
pub struct FPoseDriverTransform {
    pub target_translation: FVector,
    pub target_rotation: FRotator,
}
#[repr(C, align(8))]
pub struct FPoseDriverTarget {
    pub bone_transforms: TArray<FPoseDriverTransform>,
    pub target_rotation: FRotator,
    pub target_scale: f32,
    pub distance_method: ERBFDistanceMethod,
    pub function_type: ERBFFunctionType,
    pub b_apply_custom_curve: bool,
    pub custom_curve: FRichCurve,
    pub driven_name: FName,
    pub b_is_hidden: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_PoseDriver {
    pub source_pose: FPoseLink,
    pub source_bones: TArray<FBoneReference>,
    pub eval_space_bone: FBoneReference,
    pub b_eval_from_ref_pose: bool,
    pub only_drive_bones: TArray<FBoneReference>,
    pub pose_targets: TArray<FPoseDriverTarget>,
    pub rbf_params: FRBFParams,
    pub drive_source: EPoseDriverSource,
    pub drive_output: EPoseDriverOutput,
    pub source_bone_deprecated: FBoneReference,
    pub twist_axis_deprecated: EBoneAxis,
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
    pub twist_axis: EBoneAxis,
    pub weight_threshold: f32,
    pub normalize_method: ERBFNormalizeMethod,
    pub median_reference: FVector,
    pub median_min: f32,
    pub median_max: f32,
}
#[repr(C, align(8))]
pub struct FAnimNode_PoseSnapshot {
    pub snapshot_name: FName,
    pub snapshot: FPoseSnapshot,
    pub mode: ESnapshotSourceMode,
}
#[repr(C, align(8))]
pub struct FRandomPlayerSequenceEntry {
    pub sequence: UPtr<UAnimSequenceBase>,
    pub chance_to_play: f32,
    pub min_loop_count: i32,
    pub max_loop_count: i32,
    pub min_play_rate: f32,
    pub max_play_rate: f32,
    pub blend_in: FAlphaBlend,
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
    pub base_pose: FPoseLink,
    pub pitch: f32,
    pub yaw: f32,
    pub pitch_scale_bias_clamp: FInputScaleBiasClamp,
    pub yaw_scale_bias_clamp: FInputScaleBiasClamp,
    pub mesh_to_component: FRotator,
    pub b_rotate_root_motion_attribute: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_RotationOffsetBlendSpace {
    pub base_pose: FPoseLink,
    pub lod_threshold: i32,
    pub alpha: f32,
    pub alpha_scale_bias: FInputScaleBias,
    pub alpha_bool_blend: FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: FInputScaleBiasClamp,
    pub alpha_input_type: EAnimAlphaInputType,
    pub b_alpha_bool_enabled: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_RotationOffsetBlendSpaceGraph {
    pub base_pose: FPoseLink,
    pub lod_threshold: i32,
    pub alpha: f32,
    pub alpha_scale_bias: FInputScaleBias,
    pub alpha_bool_blend: FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: FInputScaleBiasClamp,
    pub alpha_input_type: EAnimAlphaInputType,
    pub b_alpha_bool_enabled: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_SequenceEvaluatorBase {}
#[repr(C, align(8))]
pub struct FAnimNode_SequenceEvaluator {
    pub group_name: FName,
    pub group_role: EAnimGroupRole,
    pub method: EAnimSyncMethod,
    pub b_ignore_for_relevancy_test: bool,
    pub sequence: UPtr<UAnimSequenceBase>,
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
    pub group_role: EAnimGroupRole,
    pub method: EAnimSyncMethod,
    pub b_ignore_for_relevancy_test: bool,
    pub sequence: UPtr<UAnimSequenceBase>,
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
    pub source: FPoseLink,
    pub slot_name: FName,
    pub b_always_update_source_pose: bool,
}
#[repr(C, align(8))]
pub struct FAnimNode_Sync {
    pub source: FPoseLink,
    pub group_name: FName,
    pub group_role: EAnimGroupRole,
}
#[repr(C, align(8))]
pub struct FAnimNode_TwoWayBlend {
    pub a: FPoseLink,
    pub b: FPoseLink,
    pub alpha_input_type: EAnimAlphaInputType,
    pub flags_185: u8,
    pub alpha: f32,
    pub alpha_scale_bias: FInputScaleBias,
    pub alpha_bool_blend: FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: FInputScaleBiasClamp,
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
    pub linear_axes_min: FVector,
    pub linear_axes_max: FVector,
    pub angular_constraint_type: AnimPhysAngularConstraintType,
    pub twist_axis: AnimPhysTwistAxis,
    pub angular_target_axis: AnimPhysTwistAxis,
    pub cone_angle: f32,
    pub angular_x_angle_deprecated: f32,
    pub angular_y_angle_deprecated: f32,
    pub angular_z_angle_deprecated: f32,
    pub angular_limits_min: FVector,
    pub angular_limits_max: FVector,
    pub angular_target: FVector,
}
#[repr(C, align(16))]
pub struct FAnimPhysPlanarLimit {
    pub driving_bone: FBoneReference,
    pub plane_transform: FTransform,
}
#[repr(C, align(8))]
pub struct FAnimPhysSphericalLimit {
    pub driving_bone: FBoneReference,
    pub sphere_local_offset: FVector,
    pub limit_radius: f32,
    pub limit_type: ESphericalLimitType,
}
#[repr(C, align(8))]
pub struct FAnimPhysBodyDefinition {
    pub bound_bone: FBoneReference,
    pub box_extents: FVector,
    pub local_joint_offset: FVector,
    pub constraint_setup: FAnimPhysConstraintSetup,
    pub collision_type: AnimPhysCollisionType,
    pub sphere_collision_radius: f32,
}
#[repr(C, align(8))]
pub struct FAnimPhysSimSpaceSettings {
    pub sim_space_angular_alpha: f32,
    pub max_angular_velocity: f32,
    pub max_angular_acceleration: f32,
    pub external_angular_velocity: FVector,
}
#[repr(C, align(16))]
pub struct FAnimNode_AnimDynamics {
    pub linear_damping_override: f32,
    pub angular_damping_override: f32,
    pub relative_space_bone: FBoneReference,
    pub bound_bone: FBoneReference,
    pub chain_end: FBoneReference,
    pub physics_body_definitions: TArray<FAnimPhysBodyDefinition>,
    pub gravity_scale: f32,
    pub gravity_override: FVector,
    pub linear_spring_constant: f32,
    pub angular_spring_constant: f32,
    pub wind_scale: f32,
    pub component_linear_acc_scale: FVector,
    pub component_linear_vel_scale: FVector,
    pub component_applied_linear_acc_clamp: FVector,
    pub sim_space_settings: FAnimPhysSimSpaceSettings,
    pub angular_bias_override: f32,
    pub num_solver_iterations_pre_update: i32,
    pub num_solver_iterations_post_update: i32,
    pub spherical_limits: TArray<FAnimPhysSphericalLimit>,
    pub external_force: FVector,
    pub planar_limits: TArray<FAnimPhysPlanarLimit>,
    pub simulation_space: AnimPhysSimSpaceType,
    pub flags_1035: u8,
    pub flags_1036: u8,
    pub retargeting_settings: FRotationRetargetingInfo,
    pub box_extents_deprecated: FVector,
    pub local_joint_offset_deprecated: FVector,
    pub constraint_setup_deprecated: FAnimPhysConstraintSetup,
    pub collision_type_deprecated: AnimPhysCollisionType,
    pub sphere_collision_radius_deprecated: f32,
}
#[repr(C, align(8))]
pub struct FAngularRangeLimit {
    pub limit_min: FVector,
    pub limit_max: FVector,
    pub bone: FBoneReference,
}
#[repr(C, align(8))]
pub struct FAnimNode_ApplyLimits {
    pub angular_range_limits: TArray<FAngularRangeLimit>,
    pub angular_offsets: TArray<FVector>,
}
#[repr(C, align(8))]
pub struct FAnimNode_BoneDrivenController {
    pub source_bone: FBoneReference,
    pub driving_curve: UPtr<UCurveFloat>,
    pub multiplier: f32,
    pub range_min: f64,
    pub range_max: f64,
    pub remapped_min: f64,
    pub remapped_max: f64,
    pub parameter_name: FName,
    pub target_bone: FBoneReference,
    pub target_component_deprecated: EComponentType,
    pub destination_mode: EDrivenDestinationMode,
    pub modification_mode: EDrivenBoneModificationMode,
    pub source_component: EComponentType,
    pub flags_532: u8,
    pub flags_533: u8,
}
#[repr(C, align(16))]
pub struct FAnimNode_CCDIK {
    pub effector_location: FVector,
    pub effector_location_space: EBoneControlSpace,
    pub effector_target: FBoneSocketTarget,
    pub tip_bone: FBoneReference,
    pub root_bone: FBoneReference,
    pub precision: f32,
    pub max_iterations: i32,
    pub b_start_from_tail: bool,
    pub b_enable_rotation_limit: bool,
    pub rotation_limit_per_joints: TArray<f32>,
}
#[repr(C, align(4))]
pub struct FConstraint {
    pub target_bone: FBoneReference,
    pub offset_option: EConstraintOffsetOption,
    pub transform_type: ETransformConstraintType,
    pub per_axis: FFilterOptionPerAxis,
}
#[repr(C, align(16))]
pub struct FAnimNode_Constraint {
    pub bone_to_modify: FBoneReference,
    pub constraint_setup: TArray<FConstraint>,
    pub constraint_weights: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FAnimNode_CopyBone {
    pub source_bone: FBoneReference,
    pub target_bone: FBoneReference,
    pub b_copy_translation: bool,
    pub b_copy_rotation: bool,
    pub b_copy_scale: bool,
    pub control_space: EBoneControlSpace,
}
#[repr(C, align(8))]
pub struct FAnimNode_CopyBoneDelta {
    pub source_bone: FBoneReference,
    pub target_bone: FBoneReference,
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
    pub effector_transform: FTransform,
    pub effector_target: FBoneSocketTarget,
    pub tip_bone: FBoneReference,
    pub root_bone: FBoneReference,
    pub precision: f32,
    pub max_iterations: i32,
    pub effector_transform_space: EBoneControlSpace,
    pub effector_rotation_source: EBoneRotationSource,
    pub b_enable_debug_draw: bool,
    pub effector_transform_bone_deprecated: FBoneReference,
}
#[repr(C, align(8))]
pub struct FAnimNode_HandIKRetargeting {
    pub right_hand_fk: FBoneReference,
    pub left_hand_fk: FBoneReference,
    pub right_hand_ik: FBoneReference,
    pub left_hand_ik: FBoneReference,
    pub ik_bones_to_move: TArray<FBoneReference>,
    pub per_axis_alpha: FVector,
    pub hand_fk_weight: f32,
}
#[repr(C, align(8))]
pub struct FIKChainLink {}
#[repr(C, align(8))]
pub struct FIKChain {}
#[repr(C, align(4))]
pub struct FAnimLegIKDefinition {
    pub ik_foot_bone: FBoneReference,
    pub fk_foot_bone: FBoneReference,
    pub num_bones_in_limb: i32,
    pub min_rotation_angle: f32,
    pub foot_bone_forward_axis: EAxis,
    pub hinge_rotation_axis: EAxis,
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
    pub bone_to_modify: FBoneReference,
    pub look_at_target: FBoneSocketTarget,
    pub look_at_location: FVector,
    pub look_at_axis: FAxis,
    pub b_use_look_up_axis: bool,
    pub interpolation_type: EInterpolationBlend,
    pub look_up_axis: FAxis,
    pub look_at_clamp: f32,
    pub interpolation_time: f32,
    pub interpolation_trigger_threashold: f32,
    pub look_at_bone_deprecated: FBoneReference,
    pub look_at_socket_deprecated: FName,
    pub look_at_axis_deprecated: EAxisOption,
    pub custom_look_at_axis_deprecated: FVector,
    pub look_up_axis_deprecated: EAxisOption,
    pub custom_look_up_axis_deprecated: FVector,
}
#[repr(C, align(8))]
pub struct FAnimNode_ModifyBone {
    pub bone_to_modify: FBoneReference,
    pub translation: FVector,
    pub rotation: FRotator,
    pub scale: FVector,
    pub translation_mode: EBoneModificationMode,
    pub rotation_mode: EBoneModificationMode,
    pub scale_mode: EBoneModificationMode,
    pub translation_space: EBoneControlSpace,
    pub rotation_space: EBoneControlSpace,
    pub scale_space: EBoneControlSpace,
}
#[repr(C, align(8))]
pub struct FAnimNode_ObserveBone {
    pub bone_to_observe: FBoneReference,
    pub display_space: EBoneControlSpace,
    pub b_relative_to_ref_pose: bool,
    pub translation: FVector,
    pub rotation: FRotator,
    pub scale: FVector,
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
    pub external_linear_drag_v: FVector,
    pub external_linear_velocity: FVector,
    pub external_angular_velocity: FVector,
}
#[repr(C, align(16))]
pub struct FAnimNode_RigidBody {
    pub override_physics_asset: UPtr<UPhysicsAsset>,
    pub b_default_to_skeletal_mesh_physics_asset: bool,
    pub b_use_default_as_simulated: bool,
    pub b_use_local_lod_threshold_only: bool,
    pub override_world_gravity: FVector,
    pub external_force: FVector,
    pub component_linear_acc_scale: FVector,
    pub component_linear_vel_scale: FVector,
    pub component_applied_linear_acc_clamp: FVector,
    pub sim_space_settings: FSimSpaceSettings,
    pub cached_bounds_scale: f32,
    pub base_bone_ref: FBoneReference,
    pub overlap_channel: ECollisionChannel,
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
    pub input_pose: FPoseLink,
    pub default_chain_length: f32,
    pub chain_start_bone: FBoneReference,
    pub chain_end_bone: FBoneReference,
    pub target_location: FVector,
    pub alpha: f32,
    pub alpha_scale_bias: FInputScaleBias,
    pub chain_initial_length: EScaleChainInitialLength,
}
#[repr(C, align(4))]
pub struct FSplineIKCachedBoneData {
    pub bone: FBoneReference,
    pub ref_skeleton_index: i32,
}
#[repr(C, align(8))]
pub struct FAnimNode_SplineIK {
    pub start_bone: FBoneReference,
    pub end_bone: FBoneReference,
    pub bone_axis: ESplineBoneAxis,
    pub b_auto_calculate_spline: bool,
    pub point_count: i32,
    pub control_points: TArray<FTransform>,
    pub roll: f32,
    pub twist_start: f32,
    pub twist_end: f32,
    pub twist_blend: FAlphaBlend,
    pub stretch: f32,
    pub offset: f32,
}
#[repr(C, align(8))]
pub struct FAnimNode_SpringBone {
    pub spring_bone: FBoneReference,
    pub max_displacement: f64,
    pub spring_stiffness: f64,
    pub spring_damping: f64,
    pub error_reset_thresh: f64,
    pub b_no_z_spring_deprecated: bool,
    pub flags_589: u8,
    pub flags_590: u8,
    pub owner_velocity_override: FVector,
}
#[repr(C, align(8))]
pub struct FRotationLimit {
    pub limit_min: FVector,
    pub limit_max: FVector,
}
#[repr(C, align(16))]
pub struct FAnimNode_Trail {
    pub trail_bone: FBoneReference,
    pub chain_length: i32,
    pub chain_bone_axis: EAxis,
    pub flags_553: u8,
    pub flags_554: u8,
    pub debug_life_time: f32,
    pub trail_relaxation_deprecated: f32,
    pub max_delta_time: f32,
    pub relaxation_speed_scale: f32,
    pub trail_relaxation_speed: FRuntimeFloatCurve,
    pub relaxation_speed_scale_input_processor: FInputScaleBiasClamp,
    pub rotation_limits: TArray<FRotationLimit>,
    pub rotation_offsets: TArray<FVector>,
    pub planar_limits: TArray<FAnimPhysPlanarLimit>,
    pub stretch_limit: f32,
    pub fake_velocity: FVector,
    pub base_joint: FBoneReference,
    pub trail_bone_rotation_blend_alpha_deprecated: f32,
    pub last_bone_rotation_anim_alpha_blend: f32,
}
#[repr(C, align(8))]
pub struct FReferenceBoneFrame {
    pub bone: FBoneReference,
    pub axis: FAxis,
}
#[repr(C, align(8))]
pub struct FAnimNode_TwistCorrectiveNode {
    pub base_frame: FReferenceBoneFrame,
    pub twist_frame: FReferenceBoneFrame,
    pub twist_plane_normal_axis: FAxis,
    pub range_max: f32,
    pub remapped_min: f32,
    pub remapped_max: f32,
    pub curve_deprecated: FAnimCurveParam,
    pub curve_name: FName,
}
#[repr(C, align(16))]
pub struct FAnimNode_TwoBoneIK {
    pub ik_bone: FBoneReference,
    pub start_stretch_ratio: f64,
    pub max_stretch_scale: f64,
    pub stretch_limits_deprecated: FVector2D,
    pub b_no_twist_deprecated: bool,
    pub joint_target_space_bone_name_deprecated: FName,
    pub effector_space_bone_name_deprecated: FName,
    pub effector_location: FVector,
    pub effector_target: FBoneSocketTarget,
    pub joint_target_location: FVector,
    pub joint_target: FBoneSocketTarget,
    pub twist_axis: FAxis,
    pub effector_location_space: EBoneControlSpace,
    pub joint_target_location_space: EBoneControlSpace,
    pub flags_930: u8,
}
#[repr(C, align(8))]
pub struct FIKFootPelvisPullDownSolver {
    pub pelvis_adjustment_interp: FVectorRK4SpringInterpolator,
    pub pelvis_adjustment_interp_alpha: f64,
    pub pelvis_adjustment_max_distance: f64,
    pub pelvis_adjustment_error_tolerance: f64,
    pub pelvis_adjustment_max_iter: i32,
}
#[repr(C, align(8))]
pub struct FWarpingVectorValue {
    pub mode: EWarpingVectorMode,
    pub value: FVector,
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
    pub custom_curve: FRichCurve,
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
