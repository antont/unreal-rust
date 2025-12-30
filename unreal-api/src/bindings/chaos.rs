#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FManagedArrayCollection {}
#[repr(C, align(8))]
pub struct FFieldCollection {}
#[repr(C, align(4))]
pub struct FChaosSolverDestructionSettings {
    pub per_advance_breaks_allowed: i32,
    pub per_advance_breaks_reschedule_limit: i32,
    pub clustering_particle_release_throttling_min_count: i32,
    pub clustering_particle_release_throttling_max_count: i32,
    pub b_optimize_for_runtime_memory: bool,
}
#[repr(C, align(4))]
pub struct FChaosSolverConfiguration {
    pub position_iterations: i32,
    pub velocity_iterations: i32,
    pub projection_iterations: i32,
    pub collision_margin_fraction: f32,
    pub collision_margin_max: f32,
    pub collision_cull_distance: f32,
    pub collision_max_push_out_velocity: f32,
    pub collision_initial_overlap_depenetration_velocity: f32,
    pub cluster_connection_factor: f32,
    pub cluster_union_connection_type: EClusterUnionMethod,
    pub destruction_settings: FChaosSolverDestructionSettings,
    pub b_generate_collision_data: bool,
    pub collision_filter_settings: FSolverCollisionFilterSettings,
    pub b_generate_break_data: bool,
    pub breaking_filter_settings: FSolverBreakingFilterSettings,
    pub b_generate_trailing_data: bool,
    pub trailing_filter_settings: FSolverTrailingFilterSettings,
    pub iterations_deprecated: i32,
    pub push_out_iterations_deprecated: i32,
    pub b_generate_contact_graph_deprecated: bool,
}
#[repr(C, align(4))]
pub struct FSolverTrailingFilterSettings {
    pub filter_enabled: bool,
    pub min_mass: f32,
    pub min_speed: f32,
    pub min_volume: f32,
}
#[repr(C, align(4))]
pub struct FSolverBreakingFilterSettings {
    pub filter_enabled: bool,
    pub min_mass: f32,
    pub min_speed: f32,
    pub min_volume: f32,
}
#[repr(C, align(4))]
pub struct FSolverCollisionFilterSettings {
    pub filter_enabled: bool,
    pub min_mass: f32,
    pub min_speed: f32,
    pub min_impulse: f32,
}
#[repr(C, align(8))]
pub struct FClosestPhysicsObjectResult {}
#[repr(C, align(8))]
pub struct FSerializedSolverScene {
    pub particle_data: TArray<FChaosVDParticleDataWrapper>,
    pub joint_constraint_data: TArray<FChaosVDJointConstraint>,
    pub character_ground_constraint_data: TArray<FChaosVDCharacterGroundConstraint>,
    pub collision_mid_phase_data: TArray<FChaosVDParticlePairMidPhase>,
}
#[repr(C, align(8))]
pub struct FSolverCollisionData {
    pub location: FVector,
    pub accumulated_impulse: FVector,
    pub normal: FVector,
    pub velocity1: FVector,
    pub velocity2: FVector,
    pub angular_velocity1: FVector,
    pub angular_velocity2: FVector,
    pub mass1: f32,
    pub mass2: f32,
    pub particle_index: i32,
    pub levelset_index: i32,
    pub particle_index_mesh: i32,
    pub levelset_index_mesh: i32,
}
#[repr(C, align(8))]
pub struct FSolverBreakingData {
    pub location: FVector,
    pub velocity: FVector,
    pub angular_velocity: FVector,
    pub mass: f32,
    pub particle_index: i32,
    pub particle_index_mesh: i32,
}
#[repr(C, align(8))]
pub struct FSolverTrailingData {
    pub location: FVector,
    pub velocity: FVector,
    pub angular_velocity: FVector,
    pub mass: f32,
    pub particle_index: i32,
    pub particle_index_mesh: i32,
}
#[repr(C, align(8))]
pub struct FRecordedFrame {
    pub transforms: TArray<FTransform>,
    pub transform_indices: TArray<i32>,
    pub previous_transform_indices: TArray<i32>,
    pub disabled_flags: TArray<bool>,
    pub collisions: TArray<FSolverCollisionData>,
    pub breakings: TArray<FSolverBreakingData>,
    pub trailings: TSet<FSolverTrailingData>,
    pub timestamp: f32,
}
#[repr(C, align(8))]
pub struct FRecordedTransformTrack {
    pub records: TArray<FRecordedFrame>,
}
#[repr(C, align(4))]
pub struct FSolverRemovalFilterSettings {
    pub filter_enabled: bool,
    pub min_mass: f32,
    pub min_volume: f32,
}
