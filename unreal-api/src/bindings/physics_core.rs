#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
    pub debug_color: crate::bindings::core_u_object::FLinearColor,
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
    pub solver_options: crate::bindings::chaos::FChaosSolverConfiguration,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESleepFamily(pub u8);
impl ESleepFamily {
    pub const NORMAL: ESleepFamily = ESleepFamily(0);
    pub const SENSITIVE: ESleepFamily = ESleepFamily(1);
    pub const CUSTOM: ESleepFamily = ESleepFamily(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAngularConstraintMotion(pub u8);
impl EAngularConstraintMotion {
    pub const ACM_FREE: EAngularConstraintMotion = EAngularConstraintMotion(0);
    pub const ACM_LIMITED: EAngularConstraintMotion = EAngularConstraintMotion(1);
    pub const ACM_LOCKED: EAngularConstraintMotion = EAngularConstraintMotion(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELinearConstraintMotion(pub u8);
impl ELinearConstraintMotion {
    pub const LCM_FREE: ELinearConstraintMotion = ELinearConstraintMotion(0);
    pub const LCM_LIMITED: ELinearConstraintMotion = ELinearConstraintMotion(1);
    pub const LCM_LOCKED: ELinearConstraintMotion = ELinearConstraintMotion(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EConstraintPlasticityType(pub u8);
impl EConstraintPlasticityType {
    pub const CCPT_FREE: EConstraintPlasticityType = EConstraintPlasticityType(0);
    pub const CCPT_SHRINK: EConstraintPlasticityType = EConstraintPlasticityType(1);
    pub const CCPT_GROW: EConstraintPlasticityType = EConstraintPlasticityType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPhysicalSurface(pub u8);
impl EPhysicalSurface {
    pub const SURFACE_TYPE_DEFAULT: EPhysicalSurface = EPhysicalSurface(0);
    pub const SURFACE_TYPE1: EPhysicalSurface = EPhysicalSurface(1);
    pub const SURFACE_TYPE2: EPhysicalSurface = EPhysicalSurface(2);
    pub const SURFACE_TYPE3: EPhysicalSurface = EPhysicalSurface(3);
    pub const SURFACE_TYPE4: EPhysicalSurface = EPhysicalSurface(4);
    pub const SURFACE_TYPE5: EPhysicalSurface = EPhysicalSurface(5);
    pub const SURFACE_TYPE6: EPhysicalSurface = EPhysicalSurface(6);
    pub const SURFACE_TYPE7: EPhysicalSurface = EPhysicalSurface(7);
    pub const SURFACE_TYPE8: EPhysicalSurface = EPhysicalSurface(8);
    pub const SURFACE_TYPE9: EPhysicalSurface = EPhysicalSurface(9);
    pub const SURFACE_TYPE10: EPhysicalSurface = EPhysicalSurface(10);
    pub const SURFACE_TYPE11: EPhysicalSurface = EPhysicalSurface(11);
    pub const SURFACE_TYPE12: EPhysicalSurface = EPhysicalSurface(12);
    pub const SURFACE_TYPE13: EPhysicalSurface = EPhysicalSurface(13);
    pub const SURFACE_TYPE14: EPhysicalSurface = EPhysicalSurface(14);
    pub const SURFACE_TYPE15: EPhysicalSurface = EPhysicalSurface(15);
    pub const SURFACE_TYPE16: EPhysicalSurface = EPhysicalSurface(16);
    pub const SURFACE_TYPE17: EPhysicalSurface = EPhysicalSurface(17);
    pub const SURFACE_TYPE18: EPhysicalSurface = EPhysicalSurface(18);
    pub const SURFACE_TYPE19: EPhysicalSurface = EPhysicalSurface(19);
    pub const SURFACE_TYPE20: EPhysicalSurface = EPhysicalSurface(20);
    pub const SURFACE_TYPE21: EPhysicalSurface = EPhysicalSurface(21);
    pub const SURFACE_TYPE22: EPhysicalSurface = EPhysicalSurface(22);
    pub const SURFACE_TYPE23: EPhysicalSurface = EPhysicalSurface(23);
    pub const SURFACE_TYPE24: EPhysicalSurface = EPhysicalSurface(24);
    pub const SURFACE_TYPE25: EPhysicalSurface = EPhysicalSurface(25);
    pub const SURFACE_TYPE26: EPhysicalSurface = EPhysicalSurface(26);
    pub const SURFACE_TYPE27: EPhysicalSurface = EPhysicalSurface(27);
    pub const SURFACE_TYPE28: EPhysicalSurface = EPhysicalSurface(28);
    pub const SURFACE_TYPE29: EPhysicalSurface = EPhysicalSurface(29);
    pub const SURFACE_TYPE30: EPhysicalSurface = EPhysicalSurface(30);
    pub const SURFACE_TYPE31: EPhysicalSurface = EPhysicalSurface(31);
    pub const SURFACE_TYPE32: EPhysicalSurface = EPhysicalSurface(32);
    pub const SURFACE_TYPE33: EPhysicalSurface = EPhysicalSurface(33);
    pub const SURFACE_TYPE34: EPhysicalSurface = EPhysicalSurface(34);
    pub const SURFACE_TYPE35: EPhysicalSurface = EPhysicalSurface(35);
    pub const SURFACE_TYPE36: EPhysicalSurface = EPhysicalSurface(36);
    pub const SURFACE_TYPE37: EPhysicalSurface = EPhysicalSurface(37);
    pub const SURFACE_TYPE38: EPhysicalSurface = EPhysicalSurface(38);
    pub const SURFACE_TYPE39: EPhysicalSurface = EPhysicalSurface(39);
    pub const SURFACE_TYPE40: EPhysicalSurface = EPhysicalSurface(40);
    pub const SURFACE_TYPE41: EPhysicalSurface = EPhysicalSurface(41);
    pub const SURFACE_TYPE42: EPhysicalSurface = EPhysicalSurface(42);
    pub const SURFACE_TYPE43: EPhysicalSurface = EPhysicalSurface(43);
    pub const SURFACE_TYPE44: EPhysicalSurface = EPhysicalSurface(44);
    pub const SURFACE_TYPE45: EPhysicalSurface = EPhysicalSurface(45);
    pub const SURFACE_TYPE46: EPhysicalSurface = EPhysicalSurface(46);
    pub const SURFACE_TYPE47: EPhysicalSurface = EPhysicalSurface(47);
    pub const SURFACE_TYPE48: EPhysicalSurface = EPhysicalSurface(48);
    pub const SURFACE_TYPE49: EPhysicalSurface = EPhysicalSurface(49);
    pub const SURFACE_TYPE50: EPhysicalSurface = EPhysicalSurface(50);
    pub const SURFACE_TYPE51: EPhysicalSurface = EPhysicalSurface(51);
    pub const SURFACE_TYPE52: EPhysicalSurface = EPhysicalSurface(52);
    pub const SURFACE_TYPE53: EPhysicalSurface = EPhysicalSurface(53);
    pub const SURFACE_TYPE54: EPhysicalSurface = EPhysicalSurface(54);
    pub const SURFACE_TYPE55: EPhysicalSurface = EPhysicalSurface(55);
    pub const SURFACE_TYPE56: EPhysicalSurface = EPhysicalSurface(56);
    pub const SURFACE_TYPE57: EPhysicalSurface = EPhysicalSurface(57);
    pub const SURFACE_TYPE58: EPhysicalSurface = EPhysicalSurface(58);
    pub const SURFACE_TYPE59: EPhysicalSurface = EPhysicalSurface(59);
    pub const SURFACE_TYPE60: EPhysicalSurface = EPhysicalSurface(60);
    pub const SURFACE_TYPE61: EPhysicalSurface = EPhysicalSurface(61);
    pub const SURFACE_TYPE62: EPhysicalSurface = EPhysicalSurface(62);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECollisionTraceFlag(pub u8);
impl ECollisionTraceFlag {
    pub const CTF_USE_DEFAULT: ECollisionTraceFlag = ECollisionTraceFlag(0);
    pub const CTF_USE_SIMPLE_AND_COMPLEX: ECollisionTraceFlag = ECollisionTraceFlag(1);
    pub const CTF_USE_SIMPLE_AS_COMPLEX: ECollisionTraceFlag = ECollisionTraceFlag(2);
    pub const CTF_USE_COMPLEX_AS_SIMPLE: ECollisionTraceFlag = ECollisionTraceFlag(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERadialImpulseFalloff(pub u8);
impl ERadialImpulseFalloff {
    pub const RIF_CONSTANT: ERadialImpulseFalloff = ERadialImpulseFalloff(0);
    pub const RIF_LINEAR: ERadialImpulseFalloff = ERadialImpulseFalloff(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EConstraintFrame(pub u8);
impl EConstraintFrame {
    pub const FRAME1: EConstraintFrame = EConstraintFrame(0);
    pub const FRAME2: EConstraintFrame = EConstraintFrame(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPhysicsType(pub u8);
impl EPhysicsType {
    pub const PHYS_TYPE_DEFAULT: EPhysicsType = EPhysicsType(0);
    pub const PHYS_TYPE_KINEMATIC: EPhysicsType = EPhysicsType(1);
    pub const PHYS_TYPE_SIMULATED: EPhysicsType = EPhysicsType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBodyCollisionResponse(pub u8);
impl EBodyCollisionResponse {
    pub const BODY_COLLISION_ENABLED: EBodyCollisionResponse = EBodyCollisionResponse(0);
    pub const BODY_COLLISION_DISABLED: EBodyCollisionResponse = EBodyCollisionResponse(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFrictionCombineMode(pub u8);
impl EFrictionCombineMode {
    pub const AVERAGE: EFrictionCombineMode = EFrictionCombineMode(0);
    pub const MIN: EFrictionCombineMode = EFrictionCombineMode(1);
    pub const MULTIPLY: EFrictionCombineMode = EFrictionCombineMode(2);
    pub const MAX: EFrictionCombineMode = EFrictionCombineMode(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPhysicalMaterialSoftCollisionMode(pub u8);
impl EPhysicalMaterialSoftCollisionMode {
    pub const NONE: EPhysicalMaterialSoftCollisionMode = EPhysicalMaterialSoftCollisionMode(
        0,
    );
    pub const RELATIVE_THICKNESS: EPhysicalMaterialSoftCollisionMode = EPhysicalMaterialSoftCollisionMode(
        1,
    );
    pub const ABSOLUTE_THICKESS: EPhysicalMaterialSoftCollisionMode = EPhysicalMaterialSoftCollisionMode(
        2,
    );
}
