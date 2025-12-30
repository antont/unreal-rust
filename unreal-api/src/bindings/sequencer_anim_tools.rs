#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UBaseSequencerAnimTool {}
pub struct IBaseSequencerAnimTool {}
pub struct UMotionTrailToolBuilder {}
pub struct UMotionTrailTool {
    pub transform_proxy: UPtr<UTransformProxy>,
    pub transform_gizmo: UPtr<UCombinedTransformGizmo>,
    pub trs_gizmo: UPtr<UTransformGizmo>,
    pub left_click_behavior: UPtr<USingleClickInputBehavior>,
    pub right_click_behavior: UPtr<USingleClickInputBehavior>,
}
pub struct USequencerPivotToolBuilder {}
pub struct USequencerPivotTool {
    pub click_behavior: UPtr<USingleClickInputBehavior>,
    pub transform_proxy: UPtr<UTransformProxy>,
    pub transform_gizmo: UPtr<UCombinedTransformGizmo>,
    pub trs_gizmo: UPtr<UTransformGizmo>,
}
pub struct USequencerToolsEditMode {}
