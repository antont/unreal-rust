#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct UCachedMotionAnalysisProperties {
    __padding_end: [u8; 592],
}
impl UCachedMotionAnalysisProperties {}
#[repr(C, align(16))]
pub struct ULocomotionAnalysisProperties {
    __padding_end: [u8; 416],
}
impl ULocomotionAnalysisProperties {}
#[repr(C, align(16))]
pub struct URootMotionAnalysisProperties {
    __padding_end: [u8; 416],
}
impl URootMotionAnalysisProperties {}
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
