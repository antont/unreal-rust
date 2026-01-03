#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAnimNode_IKRig {
    #[doc(hidden)]
    __padding_248: [u8; 248],
    pub goals: TArray<FIKRigGoal>,
    #[doc(hidden)]
    __padding_272: [u8; 8],
    pub alpha_input_type: crate::bindings::engine::EAnimAlphaInputType,
    pub b_alpha_bool_enabled: bool,
    pub alpha: f32,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub alpha_bool_blend: crate::bindings::engine::FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    __padding_end: [u8; 516],
}
impl FAnimNode_IKRig {}
#[repr(C, align(16))]
pub struct FIKRigGoal {
    pub name: FName,
    pub bone_name: FName,
    pub transform_source: EIKRigGoalTransformSource,
    #[doc(hidden)]
    __padding_48: [u8; 16],
    pub position: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub position_alpha: f32,
    pub rotation_alpha: f32,
    pub position_space: EIKRigGoalSpace,
    pub rotation_space: EIKRigGoalSpace,
    __padding_end: [u8; 86],
}
impl FIKRigGoal {}
#[repr(C, align(8))]
pub struct FAnimNode_RetargetPoseFromMesh {
    #[doc(hidden)]
    __padding_160: [u8; 160],
    pub retarget_from: ERetargetSourceMode,
    pub source_mesh_component: TWeakObjectPtr<
        crate::bindings::engine::USkeletalMeshComponent,
    >,
    pub ik_retargeter_asset: UPtr<UIKRetargeter>,
    pub custom_retarget_profile: FRetargetProfile,
    pub lod_threshold: i32,
    pub lod_threshold_for_ik: i32,
    pub b_suppress_warnings: bool,
    __padding_end: [u8; 599],
}
impl FAnimNode_RetargetPoseFromMesh {}
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
impl FRetargetProfile {}
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
impl FRetargetGlobalSettings {}
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
impl FTargetRootSettings {}
#[repr(C, align(8))]
pub struct FTargetChainSettings {
    pub fk: FTargetChainFKSettings,
    pub ik: FTargetChainIKSettings,
    pub speed_planting: FTargetChainSpeedPlantSettings,
    __padding_end: [u8; 4],
}
impl FTargetChainSettings {}
#[repr(C, align(4))]
pub struct FTargetChainSpeedPlantSettings {
    pub enable_speed_planting: bool,
    pub speed_curve_name: FName,
    pub speed_threshold: f32,
    pub unplant_stiffness: f32,
    pub unplant_critical_damping: f32,
}
impl FTargetChainSpeedPlantSettings {}
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
    __padding_end: [u8; 7],
}
impl FTargetChainIKSettings {}
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
impl FTargetChainFKSettings {}
#[repr(C, align(8))]
pub struct FRetargetOpProfile {
    pub op_to_apply_settings_to: FName,
    pub settings_to_apply: crate::bindings::core_u_object::FInstancedStruct,
    __padding_end: [u8; 8],
}
impl FRetargetOpProfile {}
#[repr(C, align(16))]
pub struct FIKRigGoalInput {
    __padding_end: [u8; 128],
}
impl FIKRigGoalInput {}
#[repr(C, align(8))]
pub struct FRigUnit_IKRig {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub ik_rig_asset: UPtr<UIKRigDefinition>,
    pub goals: TArray<FIKRigGoalInput>,
    __padding_end: [u8; 320],
}
impl FRigUnit_IKRig {}
#[repr(C, align(8))]
pub struct FIKRetargetPose {
    __padding_end: [u8; 112],
}
impl FIKRetargetPose {}
#[repr(C, align(8))]
pub struct FIKRetargetOpSettingsBase {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub lod_threshold: i32,
    __padding_end: [u8; 52],
}
impl FIKRetargetOpSettingsBase {}
#[repr(C, align(8))]
pub struct FIKRetargetOpBase {
    __padding_end: [u8; 56],
}
impl FIKRetargetOpBase {}
#[repr(C, align(8))]
pub struct FRetargetPoleVectorSettings {
    pub target_chain_name: FName,
    pub b_enabled: bool,
    pub align_alpha: f64,
    pub static_angular_offset: f64,
    pub maintain_offset: bool,
    __padding_end: [u8; 7],
}
impl FRetargetPoleVectorSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetAlignPoleVectorOpSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub ik_rig_asset: UPtr<UIKRigDefinition>,
    pub chains_to_align: TArray<FRetargetPoleVectorSettings>,
}
impl FIKRetargetAlignPoleVectorOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetAlignPoleVectorOp {
    __padding_end: [u8; 192],
}
impl FIKRetargetAlignPoleVectorOp {}
#[repr(C, align(8))]
pub struct FIKRetargetCopyBasePoseOpSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub b_copy_base_pose: bool,
    __padding_end: [u8; 55],
}
impl FIKRetargetCopyBasePoseOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetCopyBasePoseOp {
    __padding_end: [u8; 272],
}
impl FIKRetargetCopyBasePoseOp {}
#[repr(C, align(4))]
pub struct FCurveRemapPair {
    pub source_curve: FName,
    pub target_curve: FName,
}
impl FCurveRemapPair {}
#[repr(C, align(8))]
pub struct FIKRetargetCurveRemapOpSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub b_copy_all_source_curves: bool,
    pub b_remap_curves: bool,
    pub curves_to_remap: TArray<FCurveRemapPair>,
}
impl FIKRetargetCurveRemapOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetCurveRemapOp {
    __padding_end: [u8; 176],
}
impl FIKRetargetCurveRemapOp {}
#[repr(C, align(16))]
pub struct FFilterBoneData {
    __padding_end: [u8; 128],
}
impl FFilterBoneData {}
#[repr(C, align(8))]
pub struct FIKRetargetFilterBoneOpSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub bones_to_filter: TArray<FFilterBoneData>,
    pub min_frequency: f64,
    pub responsiveness: f64,
    pub velocity_cutoff_hz: f64,
    pub b_reset_playback: bool,
    __padding_end: [u8; 7],
}
impl FIKRetargetFilterBoneOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetFilterBoneOp {
    __padding_end: [u8; 176],
}
impl FIKRetargetFilterBoneOp {}
#[repr(C, align(8))]
pub struct FRetargetFKChainSettings {
    pub target_chain_name: FName,
    pub enable_fk: bool,
    pub rotation_mode: EFKChainRotationMode,
    pub rotation_alpha: f64,
    pub translation_mode: EFKChainTranslationMode,
    pub translation_alpha: f64,
}
impl FRetargetFKChainSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetFKChainsOpSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub ik_rig_asset: UPtr<UIKRigDefinition>,
    pub chains_to_retarget: TArray<FRetargetFKChainSettings>,
    __padding_end: [u8; 56],
}
impl FIKRetargetFKChainsOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetFKChainsOp {
    __padding_end: [u8; 296],
}
impl FIKRetargetFKChainsOp {}
#[repr(C, align(8))]
pub struct FFloorConstraintChainSettings {
    pub target_chain_name: FName,
    pub enable_floor_constraint: bool,
    pub alpha: f64,
    pub maintain_height_offset: f64,
}
impl FFloorConstraintChainSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetFloorConstraintOpSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub chains_to_affect: TArray<FFloorConstraintChainSettings>,
    pub height_falloff_offset: f64,
    pub height_falloff_distance: f64,
}
impl FIKRetargetFloorConstraintOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetFloorConstraintOp {
    __padding_end: [u8; 168],
}
impl FIKRetargetFloorConstraintOp {}
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
impl FRetargetIKChainSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetIKChainsOpSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub chains_to_retarget: TArray<FRetargetIKChainSettings>,
    __padding_end: [u8; 24],
}
impl FIKRetargetIKChainsOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetIKChainsOp {
    __padding_end: [u8; 216],
}
impl FIKRetargetIKChainsOp {}
#[repr(C, align(8))]
pub struct FIKRetargetPelvisMotionOpSettings {
    #[doc(hidden)]
    __padding_112: [u8; 112],
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
    __padding_end: [u8; 72],
}
impl FIKRetargetPelvisMotionOpSettings {}
#[repr(C, align(16))]
pub struct FIKRetargetPelvisMotionOp {
    __padding_end: [u8; 864],
}
impl FIKRetargetPelvisMotionOp {}
#[repr(C, align(16))]
pub struct FPinBoneData {
    __padding_end: [u8; 400],
}
impl FPinBoneData {}
#[repr(C, align(16))]
pub struct FIKRetargetPinBoneOpSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub bones_to_pin: TArray<FPinBoneData>,
    pub skeleton_to_copy_from: ERetargetSourceOrTarget,
    pub b_copy_translation: bool,
    pub translation_mode: EPinBoneTranslationMode,
    pub b_copy_rotation: bool,
    pub rotation_mode: EPinBoneRotationMode,
    pub b_copy_scale: bool,
    pub global_offset: crate::bindings::core_u_object::FTransform,
    pub local_offset: crate::bindings::core_u_object::FTransform,
}
impl FIKRetargetPinBoneOpSettings {}
#[repr(C, align(16))]
pub struct FIKRetargetPinBoneOp {
    __padding_end: [u8; 368],
}
impl FIKRetargetPinBoneOp {}
#[repr(C, align(8))]
pub struct FIKRetargetAdditivePoseOpSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub pose_to_apply: FName,
    pub alpha: f32,
}
impl FIKRetargetAdditivePoseOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetAdditivePoseOp {
    __padding_end: [u8; 152],
}
impl FIKRetargetAdditivePoseOp {}
#[repr(C, align(16))]
pub struct FIKRetargetRootMotionOpSettings {
    #[doc(hidden)]
    __padding_104: [u8; 104],
    pub root_motion_source: ERootMotionSource,
    #[doc(hidden)]
    __padding_128: [u8; 23],
    pub b_rotate_with_pelvis: bool,
    pub root_height_source: ERootMotionHeightSource,
    pub global_offset: crate::bindings::core_u_object::FTransform,
    pub b_maintain_offset_from_pelvis: bool,
    pub b_propagate_to_non_retargeted_children: bool,
    __padding_end: [u8; 46],
}
impl FIKRetargetRootMotionOpSettings {}
#[repr(C, align(16))]
pub struct FIKRetargetRootMotionOp {
    __padding_end: [u8; 768],
}
impl FIKRetargetRootMotionOp {}
#[repr(C, align(8))]
pub struct FIKRetargetRunIKRigOpSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub ik_rig_asset: UPtr<UIKRigDefinition>,
    pub excluded_goals: TArray<FName>,
    __padding_end: [u8; 24],
}
impl FIKRetargetRunIKRigOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetRunIKRigOp {
    __padding_end: [u8; 600],
}
impl FIKRetargetRunIKRigOp {}
#[repr(C, align(8))]
pub struct FIKRetargetScaleSourceOpSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub source_scale_factor: f64,
    pub scale_pivot: EScaleSourcePivot,
    #[doc(hidden)]
    __padding_96: [u8; 23],
    pub b_project_scale_pivot_to_floor: bool,
    __padding_end: [u8; 7],
}
impl FIKRetargetScaleSourceOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetScaleSourceOp {
    __padding_end: [u8; 176],
}
impl FIKRetargetScaleSourceOp {}
#[repr(C, align(4))]
pub struct FRetargetSpeedPlantingSettings {
    pub target_chain_name: FName,
    pub speed_curve_name: FName,
}
impl FRetargetSpeedPlantingSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetSpeedPlantingOpSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub chains_to_speed_plant: TArray<FRetargetSpeedPlantingSettings>,
    pub speed_threshold: f64,
    pub stiffness: f64,
    pub critical_damping: f64,
}
impl FIKRetargetSpeedPlantingOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetSpeedPlantingOp {
    __padding_end: [u8; 184],
}
impl FIKRetargetSpeedPlantingOp {}
#[repr(C, align(8))]
pub struct FRetargetStretchChainSettings {
    pub target_chain_name: FName,
    pub b_enabled: bool,
    pub match_source_length: f64,
    pub scale_chain_length: f64,
}
impl FRetargetStretchChainSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetStretchChainOpSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub ik_rig_asset: UPtr<UIKRigDefinition>,
    pub chains_to_stretch: TArray<FRetargetStretchChainSettings>,
}
impl FIKRetargetStretchChainOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetStretchChainOp {
    __padding_end: [u8; 192],
}
impl FIKRetargetStretchChainOp {}
#[repr(C, align(4))]
pub struct FRetargetStrideWarpChainSettings {
    pub target_chain_name: FName,
    pub enable_stride_warping: bool,
    __padding_end: [u8; 3],
}
impl FRetargetStrideWarpChainSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetStrideWarpingOpSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub chain_settings: TArray<FRetargetStrideWarpChainSettings>,
    pub direction_source: EWarpingDirectionSource,
    pub forward_direction: EBasicAxis,
    pub direction_chain: FName,
    pub warp_forwards: f64,
    pub sideways_offset: f64,
    pub warp_splay: f64,
    pub debug_draw_size: f64,
    pub debug_draw_thickness: f64,
    __padding_end: [u8; 8],
}
impl FIKRetargetStrideWarpingOpSettings {}
#[repr(C, align(16))]
pub struct FIKRetargetStrideWarpingOp {
    __padding_end: [u8; 336],
}
impl FIKRetargetStrideWarpingOp {}
#[repr(C, align(4))]
pub struct FBoneChain {
    pub chain_name: FName,
    #[doc(hidden)]
    __padding_52: [u8; 40],
    pub ik_goal_name: FName,
}
impl FBoneChain {}
#[repr(C, align(8))]
pub struct FRetargetDefinition {
    pub root_bone: FName,
    pub bone_chains: TArray<FBoneChain>,
}
impl FRetargetDefinition {}
#[repr(C, align(8))]
pub struct FIKRigSettingsBase {
    __padding_end: [u8; 8],
}
impl FIKRigSettingsBase {}
#[repr(C, align(8))]
pub struct FIKRigGoalSettingsBase {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub goal: FName,
    __padding_end: [u8; 4],
}
impl FIKRigGoalSettingsBase {}
#[repr(C, align(8))]
pub struct FIKRigBodyMoverGoalSettings {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub bone_name: FName,
    pub influence_multiplier: f32,
}
impl FIKRigBodyMoverGoalSettings {}
#[repr(C, align(8))]
pub struct FIKRigSolverSettingsBase {
    __padding_end: [u8; 8],
}
impl FIKRigSolverSettingsBase {}
#[repr(C, align(8))]
pub struct FIKRigBodyMoverSettings {
    #[doc(hidden)]
    __padding_8: [u8; 8],
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
impl FIKRigBodyMoverSettings {}
#[repr(C, align(8))]
pub struct FIKRigSolverBase {
    __padding_end: [u8; 24],
}
impl FIKRigSolverBase {}
#[repr(C, align(8))]
pub struct FIKRigBodyMoverSolver {
    __padding_end: [u8; 112],
}
impl FIKRigBodyMoverSolver {}
#[repr(C, align(8))]
pub struct FIKRigFBIKGoalSettings {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub bone_name: FName,
    pub chain_depth: i32,
    pub strength_alpha: f32,
    pub pull_chain_alpha: f32,
    pub pin_rotation: f32,
    __padding_end: [u8; 4],
}
impl FIKRigFBIKGoalSettings {}
#[repr(C, align(8))]
pub struct FIKRigBoneSettingsBase {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub bone: FName,
    __padding_end: [u8; 4],
}
impl FIKRigBoneSettingsBase {}
#[repr(C, align(8))]
pub struct FIKRigFBIKBoneSettings {
    #[doc(hidden)]
    __padding_24: [u8; 24],
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
impl FIKRigFBIKBoneSettings {}
#[repr(C, align(8))]
pub struct FIKRigFBIKSettings {
    #[doc(hidden)]
    __padding_8: [u8; 8],
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
impl FIKRigFBIKSettings {}
#[repr(C, align(8))]
pub struct FIKRigFullBodyIKSolver {
    __padding_end: [u8; 224],
}
impl FIKRigFullBodyIKSolver {}
#[repr(C, align(8))]
pub struct FLimbSolverSettings {
    __padding_end: [u8; 48],
}
impl FLimbSolverSettings {}
#[repr(C, align(8))]
pub struct FIKRigLimbSolverSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub start_bone: FName,
    pub goal_name: FName,
    pub end_bone: FName,
    __padding_end: [u8; 4],
}
impl FIKRigLimbSolverSettings {}
#[repr(C, align(8))]
pub struct FIKRigLimbSolver {
    __padding_end: [u8; 152],
}
impl FIKRigLimbSolver {}
#[repr(C, align(8))]
pub struct FIKRigPoleSolverSettings {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub start_bone: FName,
    pub end_bone: FName,
    pub aim_at_goal: FName,
    pub alpha: f32,
}
impl FIKRigPoleSolverSettings {}
#[repr(C, align(8))]
pub struct FIKRigPoleSolver {
    __padding_end: [u8; 104],
}
impl FIKRigPoleSolver {}
#[repr(C, align(8))]
pub struct FIKRigSetTransformSettings {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub goal: FName,
    pub bone_to_affect: FName,
    pub position_alpha: f32,
    pub rotation_alpha: f32,
    pub alpha: f32,
    __padding_end: [u8; 4],
}
impl FIKRigSetTransformSettings {}
#[repr(C, align(8))]
pub struct FIKRigSetTransform {
    __padding_end: [u8; 80],
}
impl FIKRigSetTransform {}
#[repr(C, align(8))]
pub struct FIKRigStretchLimbBoneSettings {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub squash_direction: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 8],
}
impl FIKRigStretchLimbBoneSettings {}
#[repr(C, align(8))]
pub struct FIKRigStretchLimbSettings {
    #[doc(hidden)]
    __padding_8: [u8; 8],
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
impl FIKRigStretchLimbSettings {}
#[repr(C, align(8))]
pub struct FIKRigStretchLimbSolver {
    __padding_end: [u8; 240],
}
impl FIKRigStretchLimbSolver {}
pub struct IIKGoalCreatorInterface {}
#[repr(C, align(8))]
pub struct UIKGoalCreatorInterface {
    __padding_end: [u8; 48],
}
impl UIKGoalCreatorInterface {}
#[repr(C, align(8))]
pub struct UIKRigComponent {
    __padding_end: [u8; 272],
}
impl UIKRigComponent {}
#[repr(C, align(8))]
pub struct URetargetChainSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub source_chain: FName,
    pub target_chain: FName,
    pub settings: FTargetChainSettings,
    __padding_end: [u8; 152],
}
impl URetargetChainSettings {}
#[repr(C, align(8))]
pub struct URetargetRootSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub settings: FTargetRootSettings,
    __padding_end: [u8; 88],
}
impl URetargetRootSettings {}
#[repr(C, align(8))]
pub struct UIKRetargetGlobalSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub settings: FRetargetGlobalSettings,
    __padding_end: [u8; 4],
}
impl UIKRetargetGlobalSettings {}
#[repr(C, align(8))]
pub struct UIKRetargeter {
    __padding_end: [u8; 848],
}
impl UIKRetargeter {}
#[repr(C, align(8))]
pub struct UIKRetargetOpControllerBase {
    __padding_end: [u8; 56],
}
impl UIKRetargetOpControllerBase {}
#[repr(C, align(8))]
pub struct URetargetOpBase {
    __padding_end: [u8; 56],
}
impl URetargetOpBase {}
#[repr(C, align(8))]
pub struct URetargetOpStack {
    __padding_end: [u8; 64],
}
impl URetargetOpStack {}
#[repr(C, align(8))]
pub struct UIKRetargetProcessor {
    __padding_end: [u8; 720],
}
impl UIKRetargetProcessor {}
#[repr(C, align(8))]
pub struct URetargetProfileLibrary {
    __padding_end: [u8; 48],
}
impl URetargetProfileLibrary {}
#[repr(C, align(8))]
pub struct UIKRetargetAlignPoleVectorController {
    __padding_end: [u8; 56],
}
impl UIKRetargetAlignPoleVectorController {}
#[repr(C, align(8))]
pub struct UIKRetargetCopyBasePoseController {
    __padding_end: [u8; 56],
}
impl UIKRetargetCopyBasePoseController {}
#[repr(C, align(8))]
pub struct UIKRetargetCurveRemapController {
    __padding_end: [u8; 56],
}
impl UIKRetargetCurveRemapController {}
#[repr(C, align(8))]
pub struct UCurveRemapOp {
    __padding_end: [u8; 80],
}
impl UCurveRemapOp {}
#[repr(C, align(8))]
pub struct UIKRetargetFilterBoneController {
    __padding_end: [u8; 56],
}
impl UIKRetargetFilterBoneController {}
#[repr(C, align(8))]
pub struct UIKRetargetFKChainsController {
    __padding_end: [u8; 56],
}
impl UIKRetargetFKChainsController {}
#[repr(C, align(8))]
pub struct UIKRetargetFloorGoalsController {
    __padding_end: [u8; 56],
}
impl UIKRetargetFloorGoalsController {}
#[repr(C, align(8))]
pub struct UIKRetargetIKChainsController {
    __padding_end: [u8; 56],
}
impl UIKRetargetIKChainsController {}
#[repr(C, align(8))]
pub struct UIKRetargetPelvisMotionController {
    __padding_end: [u8; 56],
}
impl UIKRetargetPelvisMotionController {}
#[repr(C, align(8))]
pub struct UIKRetargetPinBoneController {
    __padding_end: [u8; 56],
}
impl UIKRetargetPinBoneController {}
#[repr(C, align(16))]
pub struct UPinBoneOp {
    __padding_end: [u8; 288],
}
impl UPinBoneOp {}
#[repr(C, align(8))]
pub struct UIKRetargetAdditivePoseController {
    __padding_end: [u8; 56],
}
impl UIKRetargetAdditivePoseController {}
#[repr(C, align(8))]
pub struct UIKRetargetRootMotionController {
    __padding_end: [u8; 56],
}
impl UIKRetargetRootMotionController {}
#[repr(C, align(16))]
pub struct URootMotionGeneratorOp {
    __padding_end: [u8; 208],
}
impl URootMotionGeneratorOp {}
#[repr(C, align(8))]
pub struct UIKRetargetRunIKRigController {
    __padding_end: [u8; 56],
}
impl UIKRetargetRunIKRigController {}
#[repr(C, align(8))]
pub struct UIKRetargetScaleSourceController {
    __padding_end: [u8; 56],
}
impl UIKRetargetScaleSourceController {}
#[repr(C, align(8))]
pub struct UIKRetargetSpeedPlantingController {
    __padding_end: [u8; 56],
}
impl UIKRetargetSpeedPlantingController {}
#[repr(C, align(8))]
pub struct UIKRetargetStretchChainController {
    __padding_end: [u8; 56],
}
impl UIKRetargetStretchChainController {}
#[repr(C, align(8))]
pub struct UIKRetargetStrideWarpingController {
    __padding_end: [u8; 56],
}
impl UIKRetargetStrideWarpingController {}
#[repr(C, align(16))]
pub struct UIKRigEffectorGoal {
    #[doc(hidden)]
    __padding_48: [u8; 48],
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
    __padding_end: [u8; 2],
}
impl UIKRigEffectorGoal {}
#[repr(C, align(8))]
pub struct UIKRigDefinition {
    __padding_end: [u8; 320],
}
impl UIKRigDefinition {}
#[repr(C, align(8))]
pub struct UIKRigProcessor {
    __padding_end: [u8; 416],
}
impl UIKRigProcessor {}
#[repr(C, align(8))]
pub struct UIKRigSolverControllerBase {
    __padding_end: [u8; 56],
}
impl UIKRigSolverControllerBase {}
#[repr(C, align(8))]
pub struct UIKRigBodyMoverController {
    __padding_end: [u8; 56],
}
impl UIKRigBodyMoverController {}
#[repr(C, align(8))]
pub struct UIKRig_BodyMoverEffector {
    __padding_end: [u8; 80],
}
impl UIKRig_BodyMoverEffector {}
#[repr(C, align(8))]
pub struct UIKRigSolver {
    __padding_end: [u8; 56],
}
impl UIKRigSolver {}
#[repr(C, align(8))]
pub struct UIKRig_BodyMover {
    __padding_end: [u8; 128],
}
impl UIKRig_BodyMover {}
#[repr(C, align(8))]
pub struct UIKRigFBIKController {
    __padding_end: [u8; 56],
}
impl UIKRigFBIKController {}
#[repr(C, align(8))]
pub struct UIKRig_FBIKEffector {
    __padding_end: [u8; 88],
}
impl UIKRig_FBIKEffector {}
#[repr(C, align(8))]
pub struct UIKRig_FBIKBoneSettings {
    __padding_end: [u8; 136],
}
impl UIKRig_FBIKBoneSettings {}
#[repr(C, align(8))]
pub struct UIKRigFBIKSolver {
    __padding_end: [u8; 160],
}
impl UIKRigFBIKSolver {}
#[repr(C, align(8))]
pub struct UIKRigLimbSolverController {
    __padding_end: [u8; 56],
}
impl UIKRigLimbSolverController {}
#[repr(C, align(8))]
pub struct UIKRig_LimbEffector {
    __padding_end: [u8; 72],
}
impl UIKRig_LimbEffector {}
#[repr(C, align(8))]
pub struct UIKRig_LimbSolver {
    __padding_end: [u8; 112],
}
impl UIKRig_LimbSolver {}
#[repr(C, align(8))]
pub struct UIKRigPoleSolverController {
    __padding_end: [u8; 56],
}
impl UIKRigPoleSolverController {}
#[repr(C, align(8))]
pub struct UIKRig_PoleSolverEffector {
    __padding_end: [u8; 80],
}
impl UIKRig_PoleSolverEffector {}
#[repr(C, align(8))]
pub struct UIKRig_PoleSolver {
    __padding_end: [u8; 88],
}
impl UIKRig_PoleSolver {}
#[repr(C, align(8))]
pub struct UIKRigSetTransformController {
    __padding_end: [u8; 56],
}
impl UIKRigSetTransformController {}
#[repr(C, align(8))]
pub struct UIKRig_SetTransformEffector {
    __padding_end: [u8; 56],
}
impl UIKRig_SetTransformEffector {}
#[repr(C, align(8))]
pub struct UIKRig_SetTransform {
    __padding_end: [u8; 88],
}
impl UIKRig_SetTransform {}
#[repr(transparent)]
pub struct EIKRigGoalTransformSource(pub u8);
impl EIKRigGoalTransformSource {
    pub const MANUAL: EIKRigGoalTransformSource = EIKRigGoalTransformSource(0);
    pub const BONE: EIKRigGoalTransformSource = EIKRigGoalTransformSource(1);
    pub const ACTOR_COMPONENT: EIKRigGoalTransformSource = EIKRigGoalTransformSource(2);
}
#[repr(transparent)]
pub struct EIKRigGoalSpace(pub u8);
impl EIKRigGoalSpace {
    pub const COMPONENT: EIKRigGoalSpace = EIKRigGoalSpace(0);
    pub const ADDITIVE: EIKRigGoalSpace = EIKRigGoalSpace(1);
    pub const WORLD: EIKRigGoalSpace = EIKRigGoalSpace(2);
}
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
#[repr(transparent)]
pub struct EWarpingDirectionSource(pub i32);
impl EWarpingDirectionSource {
    pub const GOALS: EWarpingDirectionSource = EWarpingDirectionSource(0);
    pub const CHAIN: EWarpingDirectionSource = EWarpingDirectionSource(1);
    pub const ROOT_BONE: EWarpingDirectionSource = EWarpingDirectionSource(2);
}
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
#[repr(transparent)]
pub struct ERetargetSourceOrTarget(pub u8);
impl ERetargetSourceOrTarget {
    pub const SOURCE: ERetargetSourceOrTarget = ERetargetSourceOrTarget(0);
    pub const TARGET: ERetargetSourceOrTarget = ERetargetSourceOrTarget(1);
}
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
#[repr(transparent)]
pub struct EPinBoneRotationMode(pub u8);
impl EPinBoneRotationMode {
    pub const COPY_GLOBAL_ROTATION: EPinBoneRotationMode = EPinBoneRotationMode(0);
    pub const MAINTAIN_OFFSET_FROM_BONE_TO_COPY_FROM: EPinBoneRotationMode = EPinBoneRotationMode(
        1,
    );
}
#[repr(transparent)]
pub struct ERootMotionSource(pub u8);
impl ERootMotionSource {
    pub const COPY_FROM_SOURCE_ROOT: ERootMotionSource = ERootMotionSource(0);
    pub const GENERATE_FROM_TARGET_PELVIS: ERootMotionSource = ERootMotionSource(1);
}
#[repr(transparent)]
pub struct ERootMotionHeightSource(pub u8);
impl ERootMotionHeightSource {
    pub const COPY_HEIGHT_FROM_SOURCE: ERootMotionHeightSource = ERootMotionHeightSource(
        0,
    );
    pub const SNAP_TO_GROUND: ERootMotionHeightSource = ERootMotionHeightSource(1);
}
#[repr(transparent)]
pub struct EScaleSourcePivot(pub u8);
impl EScaleSourcePivot {
    pub const COMPONENT_ORIGIN: EScaleSourcePivot = EScaleSourcePivot(0);
    pub const BONE: EScaleSourcePivot = EScaleSourcePivot(1);
}
#[repr(transparent)]
pub struct EStretchLimbRotationMode(pub u8);
impl EStretchLimbRotationMode {
    pub const NONE: EStretchLimbRotationMode = EStretchLimbRotationMode(0);
    pub const ORIENT_TO_GOAL: EStretchLimbRotationMode = EStretchLimbRotationMode(1);
}
#[repr(transparent)]
pub struct EStretchLimbSquashMode(pub u8);
impl EStretchLimbSquashMode {
    pub const NONE: EStretchLimbSquashMode = EStretchLimbSquashMode(0);
    pub const UNIFORM: EStretchLimbSquashMode = EStretchLimbSquashMode(1);
    pub const BULGE: EStretchLimbSquashMode = EStretchLimbSquashMode(2);
}
#[repr(transparent)]
pub struct EAutoMapChainType(pub u8);
impl EAutoMapChainType {
    pub const EXACT: EAutoMapChainType = EAutoMapChainType(0);
    pub const FUZZY: EAutoMapChainType = EAutoMapChainType(1);
    pub const CLEAR: EAutoMapChainType = EAutoMapChainType(2);
}
#[repr(transparent)]
pub struct EPinBoneType(pub u8);
impl EPinBoneType {
    pub const FULL_TRANSFORM: EPinBoneType = EPinBoneType(0);
    pub const TRANSLATE_ONLY: EPinBoneType = EPinBoneType(1);
    pub const ROTATE_ONLY: EPinBoneType = EPinBoneType(2);
    pub const SCALE_ONLY: EPinBoneType = EPinBoneType(3);
}
#[repr(transparent)]
pub struct EIKRigGoalPreviewMode(pub u8);
impl EIKRigGoalPreviewMode {
    pub const ADDITIVE: EIKRigGoalPreviewMode = EIKRigGoalPreviewMode(0);
    pub const ABSOLUTE: EIKRigGoalPreviewMode = EIKRigGoalPreviewMode(1);
}
