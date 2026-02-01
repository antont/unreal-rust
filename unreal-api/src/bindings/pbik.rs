#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
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
impl FPBIKBoneSetting {}
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
impl FRootPrePullSettings {}
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
    pub(crate) __padding_end: [u8; 4],
}
impl FPBIKSolverSettings {}
#[repr(C, align(4))]
pub struct FPBIKDebug {
    pub(crate) __padding_end: [u8; 8],
}
impl FPBIKDebug {}
#[repr(C, align(16))]
pub struct FPBIKEffector {
    #[doc(hidden)]
    pub(crate) __padding_124: [u8; 124],
    pub chain_depth: i32,
    pub(crate) __padding_end: [u8; 16],
}
impl FPBIKEffector {}
#[repr(C, align(8))]
pub struct FRigUnit_PBIK {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub root: FName,
    pub effectors: TArray<FPBIKEffector>,
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 16],
    pub bone_settings: TArray<FPBIKBoneSetting>,
    pub excluded_bones: TArray<FName>,
    pub settings: FPBIKSolverSettings,
    pub debug: FPBIKDebug,
    pub(crate) __padding_end: [u8; 128],
}
impl FRigUnit_PBIK {}
#[repr(transparent)]
pub struct EPBIKLimitType(pub u8);
impl EPBIKLimitType {
    pub const FREE: EPBIKLimitType = EPBIKLimitType(0);
    pub const LIMITED: EPBIKLimitType = EPBIKLimitType(1);
    pub const LOCKED: EPBIKLimitType = EPBIKLimitType(2);
}
#[repr(transparent)]
pub struct EPBIKRootBehavior(pub u8);
impl EPBIKRootBehavior {
    pub const PRE_PULL: EPBIKRootBehavior = EPBIKRootBehavior(0);
    pub const PIN_TO_INPUT: EPBIKRootBehavior = EPBIKRootBehavior(1);
    pub const FREE: EPBIKRootBehavior = EPBIKRootBehavior(2);
}
