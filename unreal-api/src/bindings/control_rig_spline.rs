#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FControlRigSplineImpl {}
#[repr(C, align(8))]
pub struct FControlRigSpline {}
#[repr(C, align(8))]
pub struct FRigUnit_ControlRigSplineBase {}
#[repr(C, align(8))]
pub struct FRigUnit_ControlRigSplineFromPoints {
    pub points: TArray<FVector>,
    pub spline_mode: ESplineType,
    pub b_closed: bool,
    pub samples_per_segment: i32,
    pub compression: f32,
    pub stretch: f32,
    pub spline: FControlRigSpline,
}
#[repr(C, align(8))]
pub struct FRigUnit_ControlRigSplineFromTransforms {
    pub transforms: TArray<FTransform>,
    pub spline_mode: ESplineType,
    pub b_closed: bool,
    pub samples_per_segment: i32,
    pub compression: f32,
    pub stretch: f32,
    pub spline: FControlRigSpline,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetSplinePoints {
    pub points: TArray<FVector>,
    pub spline: FControlRigSpline,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetSplineTransforms {
    pub transforms: TArray<FTransform>,
    pub spline: FControlRigSpline,
}
#[repr(C, align(8))]
pub struct FRigUnit_PositionFromControlRigSpline {
    pub spline: FControlRigSpline,
    pub u: f32,
    pub position: FVector,
}
#[repr(C, align(16))]
pub struct FRigUnit_TransformFromControlRigSpline {
    pub spline: FControlRigSpline,
    pub up_vector: FVector,
    pub roll: f32,
    pub u: f32,
    pub transform: FTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_TransformFromControlRigSpline2 {
    pub spline: FControlRigSpline,
    pub u: f32,
    pub primary_axis: FVector,
    pub secondary_axis: FVector,
    pub transform: FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_TangentFromControlRigSpline {
    pub spline: FControlRigSpline,
    pub u: f32,
    pub tangent: FVector,
}
#[repr(C, align(8))]
pub struct FRigUnit_DrawControlRigSpline {
    pub spline: FControlRigSpline,
    pub color: FLinearColor,
    pub thickness: f32,
    pub detail: i32,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetLengthControlRigSpline {
    pub spline: FControlRigSpline,
    pub length: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetLengthAtParamControlRigSpline {
    pub spline: FControlRigSpline,
    pub u: f32,
    pub length: f32,
}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToSplineCurve {
    pub items: FRigElementKeyCollection,
    pub spline: FControlRigSpline,
    pub alignment: EControlRigCurveAlignment,
    pub minimum: f32,
    pub maximum: f32,
    pub sampling_precision: i32,
    pub primary_axis: FVector,
    pub secondary_axis: FVector,
    pub pole_vector_position: FVector,
    pub rotations: TArray<FRigUnit_FitChainToCurve_Rotation>,
    pub rotation_ease_type: ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_FitChainToCurve_DebugSettings,
    pub work_data: FRigUnit_FitChainToCurve_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToSplineCurveItemArray {
    pub items: TArray<FRigElementKey>,
    pub spline: FControlRigSpline,
    pub alignment: EControlRigCurveAlignment,
    pub minimum: f32,
    pub maximum: f32,
    pub sampling_precision: i32,
    pub primary_axis: FVector,
    pub secondary_axis: FVector,
    pub pole_vector_position: FVector,
    pub rotations: TArray<FRigUnit_FitChainToCurve_Rotation>,
    pub rotation_ease_type: ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_FitChainToCurve_DebugSettings,
    pub work_data: FRigUnit_FitChainToCurve_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_SplineConstraint_WorkData {
    pub chain_length: f32,
    pub item_transforms: TArray<FTransform>,
    pub item_segments: TArray<f32>,
    pub cached_items: TArray<FCachedRigElement>,
}
#[repr(C, align(8))]
pub struct FRigUnit_SplineConstraint {
    pub items: TArray<FRigElementKey>,
    pub spline: FControlRigSpline,
    pub alignment: EControlRigCurveAlignment,
    pub minimum: f32,
    pub maximum: f32,
    pub primary_axis: FVector,
    pub secondary_axis: FVector,
    pub b_propagate_to_children: bool,
    pub work_data: FRigUnit_SplineConstraint_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_FitSplineCurveToChain {
    pub items: FRigElementKeyCollection,
    pub spline: FControlRigSpline,
}
#[repr(C, align(8))]
pub struct FRigUnit_FitSplineCurveToChainItemArray {
    pub items: TArray<FRigElementKey>,
    pub spline: FControlRigSpline,
}
#[repr(C, align(8))]
pub struct FRigUnit_ClosestParameterFromControlRigSpline {
    pub spline: FControlRigSpline,
    pub position: FVector,
    pub u: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_ParameterAtPercentage {
    pub spline: FControlRigSpline,
    pub percentage: f32,
    pub u: f32,
}
