#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UCachedMotionAnalysisProperties {}
pub struct ULocomotionAnalysisProperties {
    pub function_axis: EAnalysisLocomotionAxis,
    pub primary_bone_socket: crate::bindings::engine::FBoneSocketTarget,
    pub secondary_bone_socket: crate::bindings::engine::FBoneSocketTarget,
    pub character_facing_axis: crate::bindings::persona::EAnalysisLinearAxis,
    pub character_up_axis: crate::bindings::persona::EAnalysisLinearAxis,
}
pub struct URootMotionAnalysisProperties {
    pub function_axis: EAnalysisRootMotionAxis,
    pub character_facing_axis: crate::bindings::persona::EAnalysisLinearAxis,
    pub character_up_axis: crate::bindings::persona::EAnalysisLinearAxis,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAnalysisLocomotionAxis(pub u8);
impl EAnalysisLocomotionAxis {
    pub const SPEED: EAnalysisLocomotionAxis = EAnalysisLocomotionAxis(0);
    pub const DIRECTION: EAnalysisLocomotionAxis = EAnalysisLocomotionAxis(1);
    pub const FORWARD_SPEED: EAnalysisLocomotionAxis = EAnalysisLocomotionAxis(2);
    pub const RIGHTWARD_SPEED: EAnalysisLocomotionAxis = EAnalysisLocomotionAxis(3);
    pub const UPWARD_SPEED: EAnalysisLocomotionAxis = EAnalysisLocomotionAxis(4);
    pub const FORWARD_SLOPE: EAnalysisLocomotionAxis = EAnalysisLocomotionAxis(5);
    pub const RIGHTWARD_SLOPE: EAnalysisLocomotionAxis = EAnalysisLocomotionAxis(6);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAnalysisRootMotionAxis(pub u8);
impl EAnalysisRootMotionAxis {
    pub const SPEED: EAnalysisRootMotionAxis = EAnalysisRootMotionAxis(0);
    pub const DIRECTION: EAnalysisRootMotionAxis = EAnalysisRootMotionAxis(1);
    pub const FORWARD_SPEED: EAnalysisRootMotionAxis = EAnalysisRootMotionAxis(2);
    pub const RIGHTWARD_SPEED: EAnalysisRootMotionAxis = EAnalysisRootMotionAxis(3);
    pub const UPWARD_SPEED: EAnalysisRootMotionAxis = EAnalysisRootMotionAxis(4);
    pub const FORWARD_SLOPE: EAnalysisRootMotionAxis = EAnalysisRootMotionAxis(5);
    pub const RIGHTWARD_SLOPE: EAnalysisRootMotionAxis = EAnalysisRootMotionAxis(6);
}
