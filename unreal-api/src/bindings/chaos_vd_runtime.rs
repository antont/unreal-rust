#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FChaosVDStartRecordingCommandMessage {
    pub recording_mode: EChaosVDRecordingMode,
    pub transport_mode: EChaosVDTransportMode,
    pub target: FString,
    pub data_channels_enabled_override_list: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FChaosVDTraceDetails {
    pub trace_guid: FGuid,
    pub session_guid: FGuid,
    pub trace_target: FString,
    pub port: u16,
    pub mode: EChaosVDRecordingMode,
    pub transport_mode: EChaosVDTransportMode,
    pub cert_auth: TArray<u8>,
    pub b_is_valid: bool,
}
#[repr(C, align(8))]
pub struct FChaosVDWrapperDataBase {
    pub b_has_valid_data: bool,
}
#[repr(C, align(8))]
pub struct FChaosVDAccelerationStructureBase {
    pub solver_id: i32,
    pub ty: EChaosVDAccelerationStructureType,
}
#[repr(C, align(8))]
pub struct FChaosVDBVCellElementDataWrapper {
    pub bounds: FBox,
    pub particle_index: i32,
}
#[repr(C, align(8))]
pub struct FChaosVDBoundingVolumeDataWrapper {
    pub max_payload_bounds: f64,
}
#[repr(C, align(8))]
pub struct FChaosVDAABBTreeNodeDataWrapper {
    pub children_bounds: FBox,
    pub children_nodes: i32,
    pub parent_node: i32,
    pub flags_140: u8,
}
#[repr(C, align(8))]
pub struct FChaosVDAABBTreePayloadBoundsElement {
    pub particle_index: i32,
    pub bounds: FBox,
}
#[repr(C, align(8))]
pub struct FChaosVDAABBTreeLeafDataWrapper {
    pub elements: TArray<FChaosVDAABBTreePayloadBoundsElement>,
    pub bounds: FBox,
}
#[repr(C, align(8))]
pub struct FChaosVDAccelerationStructureContainer {}
#[repr(C, align(8))]
pub struct FChaosVDAABBTreeDataWrapper {
    pub root_node_index: i32,
    pub tree_depth: i32,
    pub nodes_num: i32,
    pub leaves_num: i32,
    pub b_dynamic_tree: bool,
    pub max_children_in_leaf: i32,
    pub max_tree_depth: i32,
    pub max_payload_bounds: f64,
}
#[repr(C, align(8))]
pub struct FChaosVDCharacterGroundConstraintStateDataWrapper {
    pub b_disabled: bool,
    pub solver_applied_force: FVector,
    pub solver_applied_torque: FVector,
}
#[repr(C, align(8))]
pub struct FChaosVDCharacterGroundConstraintSettingsDataWrapper {
    pub vertical_axis: FVector,
    pub target_height: f64,
    pub radial_force_limit: f64,
    pub friction_force_limit: f64,
    pub twist_torque_limit: f64,
    pub swing_torque_limit: f64,
    pub cos_max_walkable_slope_angle: f64,
    pub damping_factor: f64,
    pub assumed_on_ground_height: f64,
}
#[repr(C, align(8))]
pub struct FChaosVDCharacterGroundConstraintDataDataWrapper {
    pub ground_normal: FVector,
    pub target_delta_position: FVector,
    pub target_delta_facing: f64,
    pub ground_distance: f64,
    pub cos_max_walkable_slope_angle: f64,
}
#[repr(C, align(8))]
pub struct FChaosVDConstraintDataWrapperBase {}
#[repr(C, align(8))]
pub struct FChaosVDCharacterGroundConstraint {
    pub constraint_index: i32,
    pub state: FChaosVDCharacterGroundConstraintStateDataWrapper,
    pub settings: FChaosVDCharacterGroundConstraintSettingsDataWrapper,
    pub data: FChaosVDCharacterGroundConstraintDataDataWrapper,
}
#[repr(C, align(8))]
pub struct FChaosVDContactPoint {
    pub shape_contact_points: FVector,
    pub shape_contact_normal: FVector,
    pub phi: f32,
    pub face_index: i32,
    pub contact_type: EChaosVDContactPointType,
}
#[repr(C, align(8))]
pub struct FChaosVDManifoldPoint {
    pub flags_0: u8,
    pub net_push_out: FVector,
    pub net_impulse: FVector,
    pub target_phi: f32,
    pub initial_phi: f32,
    pub shape_anchor_points: FVector,
    pub initial_shape_contact_points: FVector,
    pub contact_point: FChaosVDContactPoint,
    pub shape_contact_points: FVector,
}
#[repr(C, align(4))]
pub struct FChaosVDCollisionMaterial {
    pub face_index: i32,
    pub material_dynamic_friction: f32,
    pub material_static_friction: f32,
    pub material_restitution: f32,
    pub dynamic_friction: f32,
    pub static_friction: f32,
    pub restitution: f32,
    pub restitution_threshold: f32,
    pub inv_mass_scale0: f32,
    pub inv_mass_scale1: f32,
    pub inv_inertia_scale0: f32,
    pub inv_inertia_scale1: f32,
}
#[repr(C, align(16))]
pub struct FChaosVDConstraint {
    pub flags_0: u8,
    pub flags_1: u8,
    pub material: FChaosVDCollisionMaterial,
    pub accumulated_impulse: FVector,
    pub shapes_type: EChaosVDContactShapesType,
    pub shape_world_transforms: FTransform,
    pub implicit_transforms: FTransform,
    pub cull_distance: f32,
    pub collision_margins: TArray<f32>,
    pub collision_tolerance: f32,
    pub closest_manifold_point_index: i32,
    pub expected_num_manifold_points: i32,
    pub last_shape_world_position_delta: FVector,
    pub last_shape_world_rotation_delta: FQuat,
    pub stiffness: f32,
    pub min_initial_phi: f32,
    pub initial_overlap_depenetration_velocity: f32,
    pub ccd_time_of_impact: f32,
    pub ccd_enable_penetration: f32,
    pub ccd_target_penetration: f32,
    pub manifold_points: TArray<FChaosVDManifoldPoint>,
    pub particle0_index: i32,
    pub particle1_index: i32,
    pub solver_id: i32,
}
#[repr(C, align(8))]
pub struct FChaosVDParticlePairMidPhase {
    pub solver_id: i32,
    pub mid_phase_type: EChaosVDMidPhaseType,
    pub flags_5: u8,
    pub last_used_epoch: i32,
    pub particle0_idx: i32,
    pub particle1_idx: i32,
    pub constraints: TArray<FChaosVDConstraint>,
}
#[repr(C, align(4))]
pub struct FChaosVDCollisionFilterData {
    pub word0: u32,
    pub word1: u32,
    pub word2: u32,
    pub word3: u32,
}
#[repr(C, align(4))]
pub struct FChaosVDShapeCollisionData {
    pub collision_trace_type: EChaosVDCollisionTraceFlag,
    pub flags_4: u8,
    pub query_data: FChaosVDCollisionFilterData,
    pub sim_data: FChaosVDCollisionFilterData,
    pub b_is_complex: bool,
    pub b_is_valid: bool,
}
#[repr(C, align(8))]
pub struct FChaosVDCollisionChannelInfo {
    pub display_name: FString,
    pub collision_channel: i32,
    pub b_is_trace_type: bool,
}
#[repr(C, align(8))]
pub struct FChaosVDCollisionChannelsInfoContainer {
    pub custom_channels_names: FChaosVDCollisionChannelInfo,
}
#[repr(C, align(8))]
pub struct FChaosVDDebugShapeDataContainer {}
#[repr(C, align(8))]
pub struct FChaosVDMultiSolverDebugShapeDataContainer {}
#[repr(C, align(8))]
pub struct FChaosVDDebugDrawShapeBase {
    pub solver_id: i32,
    pub tag: FName,
    pub color: FColor,
    pub thread_context: EChaosVDParticleContext,
}
#[repr(C, align(8))]
pub struct FChaosVDDebugDrawBoxDataWrapper {
    pub box_: FBox,
}
#[repr(C, align(8))]
pub struct FChaosVDDebugDrawSphereDataWrapper {
    pub origin: FVector,
    pub radius: f32,
}
#[repr(C, align(8))]
pub struct FChaosVDDebugDrawLineDataWrapper {
    pub start_location: FVector,
    pub end_location: FVector,
    pub b_is_arrow: bool,
}
#[repr(C, align(16))]
pub struct FChaosVDDebugDrawImplicitObjectDataWrapper {}
#[repr(C, align(8))]
pub struct FChaosVDJointStateDataWrapper {
    pub flags_32: u8,
    pub linear_impulse: FVector,
    pub angular_impulse: FVector,
    pub resim_type: EChaosVDJointReSimType,
    pub sync_state: EChaosVDJointSyncType,
}
#[repr(C, align(8))]
pub struct FChaosVDGTJointStateDataWrapper {
    pub flags_16: u8,
    pub force: FVector,
    pub torque: FVector,
    pub linear_violation: f32,
    pub angular_violation: f32,
}
#[repr(C, align(8))]
pub struct FChaosVDJointSolverSettingsDataWrapper {
    pub swing_twist_angle_tolerance: f64,
    pub position_tolerance: f64,
    pub angle_tolerance: f64,
    pub min_parent_mass_ratio: f64,
    pub max_inertia_ratio: f64,
    pub min_solver_stiffness: f64,
    pub max_solver_stiffness: f64,
    pub num_iterations_at_max_solver_stiffness: i32,
    pub num_shock_propagation_iterations: i32,
    pub flags_80: u8,
    pub linear_stiffness_override: f64,
    pub twist_stiffness_override: f64,
    pub swing_stiffness_override: f64,
    pub linear_projection_override: f64,
    pub angular_projection_override: f64,
    pub shock_propagation_override: f64,
    pub linear_drive_stiffness_override: f64,
    pub linear_drive_damping_override: f64,
    pub angular_drive_stiffness_override: f64,
    pub angular_drive_damping_override: f64,
    pub soft_linear_stiffness_override: f64,
    pub soft_linear_damping_override: f64,
    pub soft_twist_stiffness_override: f64,
    pub soft_twist_damping_override: f64,
    pub soft_swing_stiffness_override: f64,
    pub soft_swing_damping_override: f64,
}
#[repr(C, align(16))]
pub struct FChaosVDJointSettingsDataWrapper {
    pub connector_transforms: FTransform,
    pub stiffness: f64,
    pub linear_projection: f64,
    pub angular_projection: f64,
    pub shock_propagation: f64,
    pub teleport_distance: f64,
    pub teleport_angle: f64,
    pub parent_inv_mass_scale: f64,
    pub flags_264: u8,
    pub flags_265: u8,
    pub linear_motion_types: EChaosVDJointMotionType,
    pub linear_limit: f64,
    pub angular_motion_types: EChaosVDJointMotionType,
    pub angular_limits: FVector,
    pub linear_soft_force_mode: EChaosVDJointForceMode,
    pub angular_soft_force_mode: EChaosVDJointForceMode,
    pub soft_linear_stiffness: f64,
    pub soft_linear_damping: f64,
    pub soft_twist_stiffness: f64,
    pub soft_twist_damping: f64,
    pub soft_swing_stiffness: f64,
    pub soft_swing_damping: f64,
    pub linear_restitution: f64,
    pub twist_restitution: f64,
    pub swing_restitution: f64,
    pub linear_contact_distance: f64,
    pub twist_contact_distance: f64,
    pub swing_contact_distance: f64,
    pub linear_drive_position_target: FVector,
    pub linear_drive_velocity_target: FVector,
    pub flags_480: u8,
    pub linear_drive_force_mode: EChaosVDJointForceMode,
    pub linear_drive_stiffness: FVector,
    pub linear_drive_damping: FVector,
    pub linear_drive_max_force: FVector,
    pub angular_drive_position_target: FQuat,
    pub angular_drive_velocity_target: FVector,
    pub angular_drive_force_mode: EChaosVDJointForceMode,
    pub angular_drive_stiffness: FVector,
    pub angular_drive_damping: FVector,
    pub angular_drive_max_torque: FVector,
    pub linear_break_force: f64,
    pub linear_plasticity_limit: f64,
    pub linear_plasticity_type: EChaosVDPlasticityType,
    pub linear_plasticity_initial_distance_squared: f64,
    pub angular_break_torque: f64,
    pub angular_plasticity_limit: f64,
    pub contact_transfer_scale: f64,
}
#[repr(C, align(16))]
pub struct FChaosVDJointConstraint {
    pub constraint_index: i32,
    pub physics_thread_joint_state: FChaosVDJointStateDataWrapper,
    pub game_thread_joint_state: FChaosVDGTJointStateDataWrapper,
    pub joint_settings: FChaosVDJointSettingsDataWrapper,
}
#[repr(C, align(8))]
pub struct FChaosVDParticleMetadata {
    pub owner_name: FName,
    pub component_name: FName,
    pub bone_name: FName,
    pub index: i32,
    pub map_asset_path: FTopLevelAssetPath,
    pub owner_asset_path: FTopLevelAssetPath,
    pub metadata_id: u64,
}
#[repr(C, align(8))]
pub struct FChaosVDFRigidParticleControlFlags {
    pub b_gravity_enabled: bool,
    pub b_ccd_enabled: bool,
    pub b_one_way_interaction_enabled: bool,
    pub b_inertia_conditioning_enabled: bool,
    pub gravity_group_index: i32,
    pub b_macd_enabled: bool,
    pub b_partial_island_sleep_allowed: bool,
    pub b_gyroscopic_torque_enabled: bool,
}
#[repr(C, align(16))]
pub struct FChaosVDParticlePositionRotation {
    pub mx: FVector,
    pub mr: FQuat,
}
#[repr(C, align(8))]
pub struct FChaosVDParticleVelocities {
    pub mv: FVector,
    pub mw: FVector,
}
#[repr(C, align(8))]
pub struct FChaosVDParticleBounds {
    pub m_min: FVector,
    pub m_max: FVector,
}
#[repr(C, align(8))]
pub struct FChaosVDParticleDynamics {
    pub m_acceleration: FVector,
    pub m_angular_acceleration: FVector,
    pub m_linear_impulse_velocity: FVector,
    pub m_angular_impulse_velocity: FVector,
}
#[repr(C, align(16))]
pub struct FChaosVDParticleMassProps {
    pub m_center_of_mass: FVector,
    pub m_rotation_of_mass: FQuat,
    pub mi: FVector,
    pub m_inv_i: FVector,
    pub mm: f64,
    pub m_inv_m: f64,
}
#[repr(C, align(8))]
pub struct FChaosVDParticleDynamicMisc {
    pub m_linear_ether_drag: f64,
    pub m_angular_ether_drag: f64,
    pub m_max_linear_speed_sq: f64,
    pub m_max_angular_speed_sq: f64,
    pub m_initial_overlap_depenetration_velocity: f32,
    pub m_sleep_threshold_multiplier: f32,
    pub m_collision_group: i32,
    pub m_object_state: EChaosVDObjectStateType,
    pub m_sleep_type: EChaosVDSleepType,
    pub m_collision_constraint_flag: u32,
    pub m_control_flags: FChaosVDFRigidParticleControlFlags,
    pub b_disabled: bool,
    pub position_solver_iteration_count: i8,
    pub velocity_solver_iteration_count: i8,
    pub projection_solver_iteration_count: i8,
}
#[repr(C, align(4))]
pub struct FChaosVDConnectivityEdge {
    pub sibling_particle_id: i32,
    pub strain: f32,
}
#[repr(C, align(16))]
pub struct FChaosVDParticleCluster {
    pub parent_particle_id: i32,
    pub num_children: i32,
    pub child_to_parent: FTransform,
    pub cluster_group_index: i32,
    pub b_internal_cluster: bool,
    pub collision_impulse: f32,
    pub external_strains: f32,
    pub internal_strains: f32,
    pub strain: f32,
    pub connectivity_edges: TArray<FChaosVDConnectivityEdge>,
    pub b_is_anchored: bool,
    pub b_unbreakable: bool,
    pub b_is_child_to_parent_locked: bool,
}
#[repr(C, align(16))]
pub struct FChaosVDKinematicTarget {
    pub rotation: FQuat,
    pub position: FVector,
    pub mode: EChaosVDKinematicTargetMode,
}
#[repr(C, align(8))]
pub struct FChaosVDVSmooth {
    pub mv: FVector,
    pub mw: FVector,
}
#[repr(C, align(16))]
pub struct FChaosVDParticleDataWrapper {
    pub dirty_flags_bits: i32,
    pub particle_context: EChaosVDParticleContext,
    pub geometry_hash: u32,
    pub debug_name: FString,
    pub metadata_id: u64,
    pub particle_index: i32,
    pub solver_id: i32,
    pub ty: EChaosVDParticleType,
    pub particle_position_rotation: FChaosVDParticlePositionRotation,
    pub particle_velocities: FChaosVDParticleVelocities,
    pub particle_inflated_bounds: FChaosVDParticleBounds,
    pub particle_kinematic_target: FChaosVDKinematicTarget,
    pub particle_vw_smooth: FChaosVDVSmooth,
    pub particle_dynamics: FChaosVDParticleDynamics,
    pub particle_dynamics_misc: FChaosVDParticleDynamicMisc,
    pub particle_mass_props: FChaosVDParticleMassProps,
    pub particle_cluster: FChaosVDParticleCluster,
    pub collision_data_per_shape: TArray<FChaosVDShapeCollisionData>,
}
#[repr(C, align(8))]
pub struct FChaosVDCollisionResponseParams {}
#[repr(C, align(8))]
pub struct FChaosVDCollisionObjectQueryParams {
    pub object_types_to_query: u8,
    pub ignore_mask: u8,
}
#[repr(C, align(8))]
pub struct FChaosVDCollisionQueryParams {
    pub trace_tag: FName,
    pub owner_tag: FName,
    pub flags_40: u8,
    pub flags_41: u8,
    pub ignore_mask: u8,
    pub ignored_actors_names: TArray<FName>,
    pub ignored_components_names: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FChaosVDQueryFastData {
    pub dir: FVector,
    pub inv_dir: FVector,
    pub current_length: f64,
    pub inv_current_length: f64,
    pub flags_80: u8,
}
#[repr(C, align(8))]
pub struct FChaosVDQueryHitData {
    pub distance: f32,
    pub face_idx: i32,
    pub flags: u16,
    pub world_position: FVector,
    pub world_normal: FVector,
    pub face_normal: FVector,
}
#[repr(C, align(16))]
pub struct FChaosVDQueryVisitStep {
    pub ty: EChaosVDSceneQueryVisitorType,
    pub shape_index: u32,
    pub particle_index: i32,
    pub particle_transform: FTransform,
    pub query_fast_data: FChaosVDQueryFastData,
    pub hit_type: EChaosVDCollisionQueryHitType,
    pub hit_data: FChaosVDQueryHitData,
    pub reject_reason: EChaosVDSQVisitRejectReason,
}
#[repr(C, align(16))]
pub struct FChaosVDQueryDataWrapper {
    pub id: i32,
    pub parent_query_id: i32,
    pub world_solver_id: i32,
    pub b_is_retry_query: bool,
    pub geometry_orientation: FQuat,
    pub ty: EChaosVDSceneQueryType,
    pub mode: EChaosVDSceneQueryMode,
    pub start_location: FVector,
    pub end_location: FVector,
    pub collision_channel: i32,
    pub collision_query_params: FChaosVDCollisionQueryParams,
    pub collision_response_params: FChaosVDCollisionResponseParams,
    pub collision_object_query_params: FChaosVDCollisionObjectQueryParams,
    pub hits: TArray<FChaosVDQueryVisitStep>,
}
#[repr(C, align(8))]
pub struct FChaosVDSceneQueriesDataContainer {}
