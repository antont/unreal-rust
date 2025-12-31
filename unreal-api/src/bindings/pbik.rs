#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FPBIKBoneSetting {
    pub bone: FName,
    pub rotation_stiffness: f32,
    pub position_stiffness: f32,
    pub x: EPBIKLimitType,
    pub min_x: f32,
    pub max_x: f32,
    pub y: EPBIKLimitType,
    pub min_y: f32,
    pub max_y: f32,
    pub z: EPBIKLimitType,
    pub min_z: f32,
    pub max_z: f32,
    pub b_use_preferred_angles: bool,
    pub preferred_angles: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(4))]
pub struct FRootPrePullSettings {
    pub rotation_alpha: f32,
    pub rotation_alpha_x: f32,
    pub rotation_alpha_y: f32,
    pub rotation_alpha_z: f32,
    pub position_alpha: f32,
    pub position_alpha_x: f32,
    pub position_alpha_y: f32,
    pub position_alpha_z: f32,
}
#[repr(C, align(4))]
pub struct FPBIKSolverSettings {
    pub iterations: i32,
    pub sub_iterations: i32,
    pub mass_multiplier: f32,
    pub b_allow_stretch: bool,
    pub root_behavior: EPBIKRootBehavior,
    pub pre_pull_root_settings: FRootPrePullSettings,
    pub global_pull_chain_alpha: f32,
    pub max_angle: f32,
    pub over_relaxation: f32,
    pub b_start_solve_from_input_pose_deprecated: bool,
}
#[repr(C, align(8))]
pub struct FPBIKSolver {}
#[repr(C, align(4))]
pub struct FPBIKDebug {
    pub draw_scale: f32,
    pub b_draw_debug: bool,
}
#[repr(C, align(16))]
pub struct FPBIKEffector {
    pub bone: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub position_alpha: f32,
    pub rotation_alpha: f32,
    pub strength_alpha: f32,
    pub chain_depth: i32,
    pub pull_chain_alpha: f32,
    pub pin_rotation: f32,
}
#[repr(C, align(8))]
pub struct FPBIKWorkData {
    pub b_needs_init: bool,
    pub hash_initialized_with: u32,
    pub bone_setting_to_solver_bone_index: TArray<i32>,
    pub solver_bone_to_element_index: TArray<i32>,
    pub solver: FPBIKSolver,
}
#[repr(C, align(8))]
pub struct FRigUnit_PBIK {
    pub root: FName,
    pub effectors: TArray<FPBIKEffector>,
    pub effector_solver_indices: TArray<i32>,
    pub bone_settings: TArray<FPBIKBoneSetting>,
    pub excluded_bones: TArray<FName>,
    pub settings: FPBIKSolverSettings,
    pub debug: FPBIKDebug,
    pub work_data: FPBIKWorkData,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPBIKLimitType(pub u8);
impl EPBIKLimitType {
    pub const FREE: EPBIKLimitType = EPBIKLimitType(0);
    pub const LIMITED: EPBIKLimitType = EPBIKLimitType(1);
    pub const LOCKED: EPBIKLimitType = EPBIKLimitType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPBIKRootBehavior(pub u8);
impl EPBIKRootBehavior {
    pub const PRE_PULL: EPBIKRootBehavior = EPBIKRootBehavior(0);
    pub const PIN_TO_INPUT: EPBIKRootBehavior = EPBIKRootBehavior(1);
    pub const FREE: EPBIKRootBehavior = EPBIKRootBehavior(2);
}
