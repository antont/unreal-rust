#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UCurveEditorFFTFilter {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub cutoff_frequency: f32,
    pub ty: ECurveEditorFFTFilterType,
    pub response: ECurveEditorFFTFilterClass,
    pub order: i32,
    __padding_end: [u8; 4],
}
impl UCurveEditorFFTFilter {}
#[repr(C, align(8))]
pub struct UCurveEditorTools_LatticeUndoObject {
    __padding_end: [u8; 248],
}
impl UCurveEditorTools_LatticeUndoObject {}
#[repr(C, align(8))]
pub struct UCurveEditorRetimeToolData {
    __padding_end: [u8; 64],
}
impl UCurveEditorRetimeToolData {}
#[repr(transparent)]
pub struct EMultiScalePivotType(pub u8);
impl EMultiScalePivotType {
    pub const AVERAGE: EMultiScalePivotType = EMultiScalePivotType(0);
    pub const BOUND_CENTER: EMultiScalePivotType = EMultiScalePivotType(1);
    pub const FIRST_KEY: EMultiScalePivotType = EMultiScalePivotType(2);
    pub const LAST_KEY: EMultiScalePivotType = EMultiScalePivotType(3);
}
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
#[repr(transparent)]
pub struct ECurveEditorFFTFilterType(pub u8);
impl ECurveEditorFFTFilterType {
    pub const LOWPASS: ECurveEditorFFTFilterType = ECurveEditorFFTFilterType(0);
    pub const HIGHPASS: ECurveEditorFFTFilterType = ECurveEditorFFTFilterType(1);
}
#[repr(transparent)]
pub struct ECurveEditorFFTFilterClass(pub u8);
impl ECurveEditorFFTFilterClass {
    pub const BUTTERWORTH: ECurveEditorFFTFilterClass = ECurveEditorFFTFilterClass(0);
    pub const CHEBYSHEV: ECurveEditorFFTFilterClass = ECurveEditorFFTFilterClass(1);
}
