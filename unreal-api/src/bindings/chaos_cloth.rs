#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FChaosClothWeightedValue {
    pub low: f32,
    pub high: f32,
}
pub struct UChaosClothConfig {
    pub mass_mode: crate::bindings::clothing_system_runtime_common::EClothMassMode,
    pub uniform_mass: f32,
    pub total_mass: f32,
    pub density: f32,
    pub min_per_particle_mass: f32,
    pub edge_stiffness_weighted: FChaosClothWeightedValue,
    pub bending_stiffness_weighted: FChaosClothWeightedValue,
    pub b_use_bending_elements: bool,
    pub buckling_ratio: f32,
    pub buckling_stiffness_weighted: FChaosClothWeightedValue,
    pub flatness_ratio: FChaosClothWeightedValue,
    pub area_stiffness_weighted: FChaosClothWeightedValue,
    pub volume_stiffness: f32,
    pub tether_stiffness: FChaosClothWeightedValue,
    pub tether_scale: FChaosClothWeightedValue,
    pub b_use_geodesic_distance: bool,
    pub shape_target_stiffness: f32,
    pub collision_thickness: f32,
    pub friction_coefficient: f32,
    pub b_use_ccd: bool,
    pub b_use_self_collisions: bool,
    pub self_collision_thickness: f32,
    pub self_collision_friction: f32,
    pub b_use_self_intersections: bool,
    pub b_use_self_collision_spheres: bool,
    pub self_collision_sphere_radius: f32,
    pub self_collision_sphere_stiffness: f32,
    pub self_collision_sphere_radius_cull_multiplier: f32,
    pub b_use_legacy_backstop: bool,
    pub damping_coefficient: f32,
    pub local_damping_coefficient: f32,
    pub b_use_point_based_wind_model: bool,
    pub drag: FChaosClothWeightedValue,
    pub b_enable_outer_drag: bool,
    pub outer_drag: FChaosClothWeightedValue,
    pub lift: FChaosClothWeightedValue,
    pub b_enable_outer_lift: bool,
    pub outer_lift: FChaosClothWeightedValue,
    pub b_use_gravity_override: bool,
    pub gravity_scale: f32,
    pub gravity: crate::bindings::core_u_object::FVector,
    pub pressure: FChaosClothWeightedValue,
    pub anim_drive_stiffness: FChaosClothWeightedValue,
    pub anim_drive_damping: FChaosClothWeightedValue,
    pub velocity_scale_space: crate::bindings::chaos::EChaosSoftsSimulationSpace,
    pub linear_velocity_scale: crate::bindings::core_u_object::FVector,
    pub b_enable_linear_velocity_clamping: bool,
    pub max_linear_velocity: crate::bindings::core_u_object::FVector3f,
    pub b_enable_linear_acceleration_clamping: bool,
    pub max_linear_acceleration: crate::bindings::core_u_object::FVector3f,
    pub angular_velocity_scale: f32,
    pub b_enable_angular_velocity_clamping: bool,
    pub max_angular_velocity: f32,
    pub b_enable_angular_acceleration_clamping: bool,
    pub max_angular_acceleration: f32,
    pub fictitious_angular_scale: f32,
    pub b_use_tetrahedral_constraints: bool,
    pub b_use_thin_shell_volume_constraints: bool,
    pub b_use_continuous_collision_detection: bool,
    pub edge_stiffness_deprecated: f32,
    pub bending_stiffness_deprecated: f32,
    pub area_stiffness_deprecated: f32,
    pub tether_mode_deprecated: EChaosClothTetherMode,
    pub limit_scale_deprecated: f32,
    pub drag_coefficient_deprecated: f32,
    pub lift_coefficient_deprecated: f32,
    pub anim_drive_spring_stiffness_deprecated: f32,
    pub strain_limiting_stiffness_deprecated: f32,
}
pub struct UChaosClothSharedSimConfig {
    pub iteration_count: i32,
    pub max_iteration_count: i32,
    pub subdivision_count: i32,
    pub self_collision_thickness_deprecated: f32,
    pub collision_thickness_deprecated: f32,
    pub b_use_damping_override_deprecated: bool,
    pub damping_deprecated: f32,
    pub b_use_gravity_override_deprecated: bool,
    pub gravity_scale_deprecated: f32,
    pub gravity_deprecated: crate::bindings::core_u_object::FVector,
    pub b_use_local_space_simulation: bool,
    pub b_use_xpbd_constraints: bool,
}
pub struct UChaosClothingSimulationFactory {}
pub struct UChaosClothingInteractor {}
pub struct UChaosClothingSimulationInteractor {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EChaosClothTetherMode(pub u8);
impl EChaosClothTetherMode {
    pub const FAST_TETHER_FAST_LENGTH: EChaosClothTetherMode = EChaosClothTetherMode(0);
    pub const ACCURATE_TETHER_FAST_LENGTH: EChaosClothTetherMode = EChaosClothTetherMode(
        1,
    );
    pub const ACCURATE_TETHER_ACCURATE_LENGTH: EChaosClothTetherMode = EChaosClothTetherMode(
        2,
    );
    pub const MAX_CHAOS_CLOTH_TETHER_MODE: EChaosClothTetherMode = EChaosClothTetherMode(
        3,
    );
}
