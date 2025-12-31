#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
    pub left_bound: crate::bindings::core_u_object::FFrameNumber,
    pub right_bound: crate::bindings::core_u_object::FFrameNumber,
    pub scale_center_x: crate::bindings::core_u_object::FFrameNumber,
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMultiScalePivotType(pub u8);
impl EMultiScalePivotType {
    pub const AVERAGE: EMultiScalePivotType = EMultiScalePivotType(0);
    pub const BOUND_CENTER: EMultiScalePivotType = EMultiScalePivotType(1);
    pub const FIRST_KEY: EMultiScalePivotType = EMultiScalePivotType(2);
    pub const LAST_KEY: EMultiScalePivotType = EMultiScalePivotType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EToolTransformInterpType(pub u8);
impl EToolTransformInterpType {
    pub const LINEAR: EToolTransformInterpType = EToolTransformInterpType(0);
    pub const SINUSOIDAL: EToolTransformInterpType = EToolTransformInterpType(1);
    pub const CUBIC: EToolTransformInterpType = EToolTransformInterpType(2);
    pub const CIRCULAR_IN: EToolTransformInterpType = EToolTransformInterpType(3);
    pub const CIRCULAR_OUT: EToolTransformInterpType = EToolTransformInterpType(4);
    pub const EXP_IN: EToolTransformInterpType = EToolTransformInterpType(5);
    pub const EXP_OUT: EToolTransformInterpType = EToolTransformInterpType(6);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECurveEditorFFTFilterType(pub u8);
impl ECurveEditorFFTFilterType {
    pub const LOWPASS: ECurveEditorFFTFilterType = ECurveEditorFFTFilterType(0);
    pub const HIGHPASS: ECurveEditorFFTFilterType = ECurveEditorFFTFilterType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECurveEditorFFTFilterClass(pub u8);
impl ECurveEditorFFTFilterClass {
    pub const BUTTERWORTH: ECurveEditorFFTFilterClass = ECurveEditorFFTFilterClass(0);
    pub const CHEBYSHEV: ECurveEditorFFTFilterClass = ECurveEditorFFTFilterClass(1);
}
