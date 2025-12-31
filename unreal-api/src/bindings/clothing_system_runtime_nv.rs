#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FClothConstraintSetupNv {
    pub stiffness: f32,
    pub stiffness_multiplier: f32,
    pub stretch_limit: f32,
    pub compression_limit: f32,
}
pub struct UClothConfigNv {
    pub clothing_wind_method: EClothingWindMethodNv,
    pub vertical_constraint: FClothConstraintSetupNv,
    pub horizontal_constraint: FClothConstraintSetupNv,
    pub bend_constraint: FClothConstraintSetupNv,
    pub shear_constraint: FClothConstraintSetupNv,
    pub self_collision_radius: f32,
    pub self_collision_stiffness: f32,
    pub self_collision_cull_scale: f32,
    pub damping: crate::bindings::core_u_object::FVector,
    pub friction: f32,
    pub wind_drag_coefficient: f32,
    pub wind_lift_coefficient: f32,
    pub linear_drag: crate::bindings::core_u_object::FVector,
    pub angular_drag: crate::bindings::core_u_object::FVector,
    pub linear_inertia_scale: crate::bindings::core_u_object::FVector,
    pub angular_inertia_scale: crate::bindings::core_u_object::FVector,
    pub centrifugal_inertia_scale: crate::bindings::core_u_object::FVector,
    pub solver_frequency: f32,
    pub stiffness_frequency: f32,
    pub gravity_scale: f32,
    pub gravity_override: crate::bindings::core_u_object::FVector,
    pub b_use_gravity_override: bool,
    pub tether_stiffness: f32,
    pub tether_limit: f32,
    pub collision_thickness: f32,
    pub anim_drive_spring_stiffness: f32,
    pub anim_drive_damper_stiffness: f32,
    pub wind_method_deprecated: crate::bindings::clothing_system_runtime_common::EClothingWindMethod_Legacy,
    pub vertical_constraint_config_deprecated: crate::bindings::clothing_system_runtime_common::FClothConstraintSetup_Legacy,
    pub horizontal_constraint_config_deprecated: crate::bindings::clothing_system_runtime_common::FClothConstraintSetup_Legacy,
    pub bend_constraint_config_deprecated: crate::bindings::clothing_system_runtime_common::FClothConstraintSetup_Legacy,
    pub shear_constraint_config_deprecated: crate::bindings::clothing_system_runtime_common::FClothConstraintSetup_Legacy,
}
pub struct UClothingSimulationFactoryNv {}
pub struct UClothingSimulationInteractorNv {}
pub struct UClothPhysicalMeshDataNv_Legacy {
    pub max_distances: TArray<f32>,
    pub backstop_distances: TArray<f32>,
    pub backstop_radiuses: TArray<f32>,
    pub anim_drive_multipliers: TArray<f32>,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EClothingWindMethodNv(pub u8);
impl EClothingWindMethodNv {
    pub const LEGACY: EClothingWindMethodNv = EClothingWindMethodNv(0);
    pub const ACCURATE: EClothingWindMethodNv = EClothingWindMethodNv(1);
}
