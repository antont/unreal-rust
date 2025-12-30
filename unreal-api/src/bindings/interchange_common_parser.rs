#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FInterchangeCurveKey {
    pub interp_mode: EInterchangeCurveInterpMode,
    pub tangent_mode: EInterchangeCurveTangentMode,
    pub tangent_weight_mode: EInterchangeCurveTangentWeightMode,
    pub time: f32,
    pub value: f32,
    pub arrive_tangent: f32,
    pub arrive_tangent_weight: f32,
    pub leave_tangent: f32,
    pub leave_tangent_weight: f32,
}
#[repr(C, align(8))]
pub struct FInterchangeCurve {
    pub keys: TArray<FInterchangeCurveKey>,
}
#[repr(C, align(8))]
pub struct FInterchangeStepCurve {
    pub key_times: TArray<f32>,
}
