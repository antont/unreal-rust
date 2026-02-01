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
pub struct FFBIKConstraintOption {
    pub item: crate::bindings::control_rig::FRigElementKey,
    pub(crate) __padding_end: [u8; 152],
}
impl FFBIKConstraintOption {}
#[repr(C, align(1))]
pub struct FMotionProcessInput {
    pub(crate) __padding_end: [u8; 2],
}
impl FMotionProcessInput {}
#[repr(C, align(16))]
pub struct FFBIKDebugOption {
    pub(crate) __padding_end: [u8; 128],
}
impl FFBIKDebugOption {}
#[repr(C, align(4))]
pub struct FSolverInput {
    pub(crate) __padding_end: [u8; 36],
}
impl FSolverInput {}
#[repr(C, align(16))]
pub struct FFBIKEndEffector {
    pub item: crate::bindings::control_rig::FRigElementKey,
    pub(crate) __padding_end: [u8; 80],
}
impl FFBIKEndEffector {}
#[repr(C, align(16))]
pub struct FRigUnit_FullbodyIK {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub root: crate::bindings::control_rig::FRigElementKey,
    pub effectors: TArray<FFBIKEndEffector>,
    pub constraints: TArray<FFBIKConstraintOption>,
    pub solver_property: FSolverInput,
    pub motion_property: FMotionProcessInput,
    pub b_propagate_to_children: bool,
    pub debug_option: FFBIKDebugOption,
    pub(crate) __padding_end: [u8; 432],
}
impl FRigUnit_FullbodyIK {}
#[repr(transparent)]
pub struct EFBIKBoneLimitType(pub u8);
impl EFBIKBoneLimitType {
    pub const FREE: EFBIKBoneLimitType = EFBIKBoneLimitType(0);
    pub const LIMIT: EFBIKBoneLimitType = EFBIKBoneLimitType(1);
    pub const LOCKED: EFBIKBoneLimitType = EFBIKBoneLimitType(2);
}
#[repr(transparent)]
pub struct EPoleVectorOption(pub u8);
impl EPoleVectorOption {
    pub const DIRECTION: EPoleVectorOption = EPoleVectorOption(0);
    pub const LOCATION: EPoleVectorOption = EPoleVectorOption(1);
}
