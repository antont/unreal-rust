#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FFBIKBoneLimit {
    pub limit_type_x: EFBIKBoneLimitType,
    pub limit_type_y: EFBIKBoneLimitType,
    pub limit_type_z: EFBIKBoneLimitType,
    pub limit: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FFBIKConstraintOption {
    pub item: crate::bindings::control_rig::FRigElementKey,
    pub b_enabled: bool,
    pub b_use_stiffness: bool,
    pub linear_stiffness: crate::bindings::core_u_object::FVector,
    pub angular_stiffness: crate::bindings::core_u_object::FVector,
    pub b_use_angular_limit: bool,
    pub angular_limit: FFBIKBoneLimit,
    pub b_use_pole_vector: bool,
    pub pole_vector_option: EPoleVectorOption,
    pub pole_vector: crate::bindings::core_u_object::FVector,
    pub offset_rotation: crate::bindings::core_u_object::FRotator,
}
#[repr(C, align(1))]
pub struct FMotionProcessInput {
    pub b_force_effector_rotation_target: bool,
    pub b_only_apply_when_reached_to_target: bool,
}
#[repr(C, align(16))]
pub struct FFBIKDebugOption {
    pub b_draw_debug_hierarchy: bool,
    pub b_color_angular_motion_strength: bool,
    pub b_color_linear_motion_strength: bool,
    pub b_draw_debug_axes: bool,
    pub b_draw_debug_effector: bool,
    pub b_draw_debug_constraints: bool,
    pub draw_world_offset: crate::bindings::core_u_object::FTransform,
    pub draw_size: f32,
}
#[repr(C, align(4))]
pub struct FSolverInput {
    pub linear_motion_strength: f32,
    pub min_linear_motion_strength: f32,
    pub angular_motion_strength: f32,
    pub min_angular_motion_strength: f32,
    pub default_target_clamp: f32,
    pub precision: f32,
    pub damping: f32,
    pub max_iterations: i32,
    pub b_use_jacobian_transpose: bool,
}
#[repr(C, align(16))]
pub struct FFBIKEndEffector {
    pub item: crate::bindings::control_rig::FRigElementKey,
    pub position: crate::bindings::core_u_object::FVector,
    pub position_alpha: f32,
    pub position_depth: i32,
    pub rotation: crate::bindings::core_u_object::FQuat,
    pub rotation_alpha: f32,
    pub rotation_depth: i32,
    pub pull: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_FullbodyIK_WorkData {}
#[repr(C, align(16))]
pub struct FRigUnit_FullbodyIK {
    pub root: crate::bindings::control_rig::FRigElementKey,
    pub effectors: TArray<FFBIKEndEffector>,
    pub constraints: TArray<FFBIKConstraintOption>,
    pub solver_property: FSolverInput,
    pub motion_property: FMotionProcessInput,
    pub b_propagate_to_children: bool,
    pub debug_option: FFBIKDebugOption,
    pub work_data: FRigUnit_FullbodyIK_WorkData,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFBIKBoneLimitType(pub u8);
impl EFBIKBoneLimitType {
    pub const FREE: EFBIKBoneLimitType = EFBIKBoneLimitType(0);
    pub const LIMIT: EFBIKBoneLimitType = EFBIKBoneLimitType(1);
    pub const LOCKED: EFBIKBoneLimitType = EFBIKBoneLimitType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPoleVectorOption(pub u8);
impl EPoleVectorOption {
    pub const DIRECTION: EPoleVectorOption = EPoleVectorOption(0);
    pub const LOCATION: EPoleVectorOption = EPoleVectorOption(1);
}
