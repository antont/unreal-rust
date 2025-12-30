#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FBodyInstanceCore {
    pub flags_8: u8,
    pub flags_9: u8,
}
#[repr(C, align(4))]
pub struct FPhysicalMaterialStrength {
    pub tensile_strength: f32,
    pub compression_strength: f32,
    pub shear_strength: f32,
}
#[repr(C, align(4))]
pub struct FPhysicalMaterialDamageModifier {
    pub damage_threshold_multiplier: f32,
}
pub struct UDEPRECATED_PhysicalMaterialPropertyBase {}
pub struct UBodySetupCore {
    pub bone_name: FName,
    pub physics_type: EPhysicsType,
    pub collision_trace_flag: ECollisionTraceFlag,
    pub collision_reponse: EBodyCollisionResponse,
}
pub struct UChaosPhysicalMaterial {
    pub friction: f32,
    pub static_friction: f32,
    pub restitution: f32,
    pub linear_ether_drag: f32,
    pub angular_ether_drag: f32,
    pub sleeping_linear_velocity_threshold: f32,
    pub sleeping_angular_velocity_threshold: f32,
}
pub struct UPhysicalMaterial {
    pub friction: f32,
    pub static_friction: f32,
    pub friction_combine_mode: EFrictionCombineMode,
    pub b_override_friction_combine_mode: bool,
    pub restitution: f32,
    pub restitution_combine_mode: EFrictionCombineMode,
    pub b_override_restitution_combine_mode: bool,
    pub density: f32,
    pub sleep_linear_velocity_threshold: f32,
    pub sleep_angular_velocity_threshold: f32,
    pub sleep_counter_threshold: i32,
    pub raise_mass_to_power: f32,
    pub destructible_damage_threshold_scale_deprecated: f32,
    pub physical_material_property_deprecated: UPtr<
        UDEPRECATED_PhysicalMaterialPropertyBase,
    >,
    pub surface_type: EPhysicalSurface,
    pub strength: FPhysicalMaterialStrength,
    pub damage_modifier: FPhysicalMaterialDamageModifier,
    pub debug_color: FLinearColor,
    pub b_show_experimental_properties: bool,
    pub soft_collision_mode: EPhysicalMaterialSoftCollisionMode,
    pub soft_collision_thickness: f32,
    pub base_friction_impulse: f32,
}
pub struct UPhysicsSettingsCore {
    pub default_gravity_z: f32,
    pub default_terminal_velocity: f32,
    pub default_fluid_friction: f32,
    pub simulate_scratch_memory_size: i32,
    pub ragdoll_aggregate_threshold: i32,
    pub triangle_mesh_triangle_min_area_threshold: f32,
    pub b_enable_enhanced_determinism: bool,
    pub b_enable_shape_sharing: bool,
    pub b_enable_pcm: bool,
    pub b_enable_stabilization: bool,
    pub b_warn_missing_locks: bool,
    pub b_enable2_d_physics: bool,
    pub b_default_has_complex_collision_deprecated: bool,
    pub bounce_threshold_velocity: f32,
    pub friction_combine_mode: EFrictionCombineMode,
    pub restitution_combine_mode: EFrictionCombineMode,
    pub max_angular_velocity: f32,
    pub max_depenetration_velocity: f32,
    pub contact_offset_multiplier: f32,
    pub min_contact_offset: f32,
    pub max_contact_offset: f32,
    pub b_simulate_skeletal_mesh_on_dedicated_server: bool,
    pub default_shape_complexity: ECollisionTraceFlag,
    pub solver_options: FChaosSolverConfiguration,
}
