#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UCachedMotionAnalysisProperties {}
pub struct ULocomotionAnalysisProperties {
    pub function_axis: EAnalysisLocomotionAxis,
    pub primary_bone_socket: FBoneSocketTarget,
    pub secondary_bone_socket: FBoneSocketTarget,
    pub character_facing_axis: EAnalysisLinearAxis,
    pub character_up_axis: EAnalysisLinearAxis,
}
pub struct URootMotionAnalysisProperties {
    pub function_axis: EAnalysisRootMotionAxis,
    pub character_facing_axis: EAnalysisLinearAxis,
    pub character_up_axis: EAnalysisLinearAxis,
}
