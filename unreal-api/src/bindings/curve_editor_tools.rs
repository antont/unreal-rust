#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FCurveEditorRetimeAnchor {
    pub value_in_seconds: f64,
    pub b_is_selected: bool,
}
#[repr(C, align(4))]
pub struct FMultiScaleToolOptions {
    pub x_scale: f32,
    pub y_scale: f32,
    pub pivot_type: EMultiScalePivotType,
}
#[repr(C, align(4))]
pub struct FTransformToolOptions {
    pub upper_bound: f32,
    pub lower_bound: f32,
    pub left_bound: FFrameNumber,
    pub right_bound: FFrameNumber,
    pub scale_center_x: FFrameNumber,
    pub scale_center_y: f32,
    pub falloff_interp_type: EToolTransformInterpType,
}
pub struct UCurveEditorFFTFilter {
    pub cutoff_frequency: f32,
    pub ty: ECurveEditorFFTFilterType,
    pub response: ECurveEditorFFTFilterClass,
    pub order: i32,
}
pub struct UCurveEditorTools_LatticeUndoObject {}
pub struct UCurveEditorRetimeToolData {
    pub retiming_anchors: TArray<FCurveEditorRetimeAnchor>,
}
