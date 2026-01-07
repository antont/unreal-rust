#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FControlRigSpline {
    __padding_end: [u8; 24],
}
impl FControlRigSpline {}
#[repr(C, align(8))]
pub struct FRigUnit_ControlRigSplineBase {
    __padding_end: [u8; 8],
}
impl FRigUnit_ControlRigSplineBase {}
#[repr(C, align(8))]
pub struct FRigUnit_ControlRigSplineFromPoints {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub points: TArray<crate::bindings::core_u_object::FVector>,
    pub spline_mode: ESplineType,
    pub b_closed: bool,
    pub samples_per_segment: i32,
    pub compression: f32,
    pub stretch: f32,
    pub spline: FControlRigSpline,
}
impl FRigUnit_ControlRigSplineFromPoints {}
#[repr(C, align(8))]
pub struct FRigUnit_ControlRigSplineFromTransforms {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub spline_mode: ESplineType,
    pub b_closed: bool,
    pub samples_per_segment: i32,
    pub compression: f32,
    pub stretch: f32,
    pub spline: FControlRigSpline,
}
impl FRigUnit_ControlRigSplineFromTransforms {}
#[repr(C, align(8))]
pub struct FRigUnit_SetSplinePoints {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub points: TArray<crate::bindings::core_u_object::FVector>,
    pub spline: FControlRigSpline,
}
impl FRigUnit_SetSplinePoints {}
#[repr(C, align(8))]
pub struct FRigUnit_SetSplineTransforms {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub spline: FControlRigSpline,
}
impl FRigUnit_SetSplineTransforms {}
#[repr(C, align(8))]
pub struct FRigUnit_PositionFromControlRigSpline {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub spline: FControlRigSpline,
    pub u: f32,
    pub position: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_PositionFromControlRigSpline {}
#[repr(C, align(16))]
pub struct FRigUnit_TransformFromControlRigSpline {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub spline: FControlRigSpline,
    pub up_vector: crate::bindings::core_u_object::FVector,
    pub roll: f32,
    pub u: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_TransformFromControlRigSpline {}
#[repr(C, align(16))]
pub struct FRigUnit_TransformFromControlRigSpline2 {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub spline: FControlRigSpline,
    pub u: f32,
    pub primary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis: crate::bindings::core_u_object::FVector,
    pub transform: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_TransformFromControlRigSpline2 {}
#[repr(C, align(8))]
pub struct FRigUnit_TangentFromControlRigSpline {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub spline: FControlRigSpline,
    pub u: f32,
    pub tangent: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_TangentFromControlRigSpline {}
#[repr(C, align(8))]
pub struct FRigUnit_DrawControlRigSpline {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub spline: FControlRigSpline,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub detail: i32,
}
impl FRigUnit_DrawControlRigSpline {}
#[repr(C, align(8))]
pub struct FRigUnit_GetLengthControlRigSpline {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub spline: FControlRigSpline,
    pub length: f32,
}
impl FRigUnit_GetLengthControlRigSpline {}
#[repr(C, align(8))]
pub struct FRigUnit_GetLengthAtParamControlRigSpline {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub spline: FControlRigSpline,
    pub u: f32,
    pub length: f32,
}
impl FRigUnit_GetLengthAtParamControlRigSpline {}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToSplineCurve {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub items: crate::bindings::control_rig::FRigElementKeyCollection,
    pub spline: FControlRigSpline,
    pub alignment: crate::bindings::control_rig::EControlRigCurveAlignment,
    pub minimum: f32,
    pub maximum: f32,
    pub sampling_precision: i32,
    pub primary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis: crate::bindings::core_u_object::FVector,
    pub pole_vector_position: crate::bindings::core_u_object::FVector,
    pub rotations: TArray<
        crate::bindings::control_rig::FRigUnit_FitChainToCurve_Rotation,
    >,
    pub rotation_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub debug_settings: crate::bindings::control_rig::FRigUnit_FitChainToCurve_DebugSettings,
    __padding_end: [u8; 160],
}
impl FRigUnit_FitChainToSplineCurve {}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToSplineCurveItemArray {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub items: TArray<crate::bindings::control_rig::FRigElementKey>,
    pub spline: FControlRigSpline,
    pub alignment: crate::bindings::control_rig::EControlRigCurveAlignment,
    pub minimum: f32,
    pub maximum: f32,
    pub sampling_precision: i32,
    pub primary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis: crate::bindings::core_u_object::FVector,
    pub pole_vector_position: crate::bindings::core_u_object::FVector,
    pub rotations: TArray<
        crate::bindings::control_rig::FRigUnit_FitChainToCurve_Rotation,
    >,
    pub rotation_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub debug_settings: crate::bindings::control_rig::FRigUnit_FitChainToCurve_DebugSettings,
    __padding_end: [u8; 160],
}
impl FRigUnit_FitChainToSplineCurveItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_SplineConstraint {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub items: TArray<crate::bindings::control_rig::FRigElementKey>,
    pub spline: FControlRigSpline,
    pub alignment: crate::bindings::control_rig::EControlRigCurveAlignment,
    pub minimum: f32,
    pub maximum: f32,
    pub primary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis: crate::bindings::core_u_object::FVector,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 63],
}
impl FRigUnit_SplineConstraint {}
#[repr(C, align(8))]
pub struct FRigUnit_FitSplineCurveToChain {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub items: crate::bindings::control_rig::FRigElementKeyCollection,
    pub spline: FControlRigSpline,
}
impl FRigUnit_FitSplineCurveToChain {}
#[repr(C, align(8))]
pub struct FRigUnit_FitSplineCurveToChainItemArray {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub items: TArray<crate::bindings::control_rig::FRigElementKey>,
    pub spline: FControlRigSpline,
}
impl FRigUnit_FitSplineCurveToChainItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_ClosestParameterFromControlRigSpline {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub spline: FControlRigSpline,
    pub position: crate::bindings::core_u_object::FVector,
    pub u: f32,
}
impl FRigUnit_ClosestParameterFromControlRigSpline {}
#[repr(C, align(8))]
pub struct FRigUnit_ParameterAtPercentage {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub spline: FControlRigSpline,
    pub percentage: f32,
    pub u: f32,
}
impl FRigUnit_ParameterAtPercentage {}
#[repr(transparent)]
pub struct ESplineType(pub u8);
impl ESplineType {
    pub const B_SPLINE: ESplineType = ESplineType(0);
    pub const HERMITE: ESplineType = ESplineType(1);
    pub const MAX: ESplineType = ESplineType(2);
}
