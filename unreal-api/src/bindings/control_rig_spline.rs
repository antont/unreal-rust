#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FControlRigSplineImpl {}
#[repr(C, align(8))]
pub struct FControlRigSpline {}
#[repr(C, align(8))]
pub struct FRigUnit_ControlRigSplineBase {}
#[repr(C, align(8))]
pub struct FRigUnit_ControlRigSplineFromPoints {
    pub points: TArray<crate::bindings::core_u_object::FVector>,
    pub spline_mode: ESplineType,
    pub b_closed: bool,
    pub samples_per_segment: i32,
    pub compression: f32,
    pub stretch: f32,
    pub spline: FControlRigSpline,
}
#[repr(C, align(8))]
pub struct FRigUnit_ControlRigSplineFromTransforms {
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub spline_mode: ESplineType,
    pub b_closed: bool,
    pub samples_per_segment: i32,
    pub compression: f32,
    pub stretch: f32,
    pub spline: FControlRigSpline,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetSplinePoints {
    pub points: TArray<crate::bindings::core_u_object::FVector>,
    pub spline: FControlRigSpline,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetSplineTransforms {
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub spline: FControlRigSpline,
}
#[repr(C, align(8))]
pub struct FRigUnit_PositionFromControlRigSpline {
    pub spline: FControlRigSpline,
    pub u: f32,
    pub position: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FRigUnit_TransformFromControlRigSpline {
    pub spline: FControlRigSpline,
    pub up_vector: crate::bindings::core_u_object::FVector,
    pub roll: f32,
    pub u: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_TransformFromControlRigSpline2 {
    pub spline: FControlRigSpline,
    pub u: f32,
    pub primary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis: crate::bindings::core_u_object::FVector,
    pub transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_TangentFromControlRigSpline {
    pub spline: FControlRigSpline,
    pub u: f32,
    pub tangent: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigUnit_DrawControlRigSpline {
    pub spline: FControlRigSpline,
    pub color: crate::bindings::core_u_object::FLinearColor,
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
    pub work_data: crate::bindings::control_rig::FRigUnit_FitChainToCurve_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToSplineCurveItemArray {
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
    pub work_data: crate::bindings::control_rig::FRigUnit_FitChainToCurve_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_SplineConstraint_WorkData {
    pub chain_length: f32,
    pub item_transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub item_segments: TArray<f32>,
    pub cached_items: TArray<crate::bindings::control_rig::FCachedRigElement>,
}
#[repr(C, align(8))]
pub struct FRigUnit_SplineConstraint {
    pub items: TArray<crate::bindings::control_rig::FRigElementKey>,
    pub spline: FControlRigSpline,
    pub alignment: crate::bindings::control_rig::EControlRigCurveAlignment,
    pub minimum: f32,
    pub maximum: f32,
    pub primary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis: crate::bindings::core_u_object::FVector,
    pub b_propagate_to_children: bool,
    pub work_data: FRigUnit_SplineConstraint_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_FitSplineCurveToChain {
    pub items: crate::bindings::control_rig::FRigElementKeyCollection,
    pub spline: FControlRigSpline,
}
#[repr(C, align(8))]
pub struct FRigUnit_FitSplineCurveToChainItemArray {
    pub items: TArray<crate::bindings::control_rig::FRigElementKey>,
    pub spline: FControlRigSpline,
}
#[repr(C, align(8))]
pub struct FRigUnit_ClosestParameterFromControlRigSpline {
    pub spline: FControlRigSpline,
    pub position: crate::bindings::core_u_object::FVector,
    pub u: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_ParameterAtPercentage {
    pub spline: FControlRigSpline,
    pub percentage: f32,
    pub u: f32,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESplineType(pub u8);
impl ESplineType {
    pub const B_SPLINE: ESplineType = ESplineType(0);
    pub const HERMITE: ESplineType = ESplineType(1);
    pub const MAX: ESplineType = ESplineType(2);
}
