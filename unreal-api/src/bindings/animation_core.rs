#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FFilterOptionPerAxis {
    pub b_x: bool,
    pub b_y: bool,
    pub b_z: bool,
}
impl FFilterOptionPerAxis {}
#[repr(C, align(1))]
pub struct FTransformFilter {
    pub translation_filter: FFilterOptionPerAxis,
    pub rotation_filter: FFilterOptionPerAxis,
    pub scale_filter: FFilterOptionPerAxis,
}
impl FTransformFilter {}
#[repr(C, align(1))]
pub struct FConstraintDescription {
    pub b_translation: bool,
    pub b_rotation: bool,
    pub b_scale: bool,
    pub b_parent: bool,
    pub translation_axes: FFilterOptionPerAxis,
    pub rotation_axes: FFilterOptionPerAxis,
    pub scale_axes: FFilterOptionPerAxis,
}
impl FConstraintDescription {}
#[repr(C, align(4))]
pub struct FTransformConstraint {
    pub operator: FConstraintDescription,
    pub source_node: FName,
    pub target_node: FName,
    pub weight: f32,
    pub b_maintain_offset: bool,
    __padding_end: [u8; 3],
}
impl FTransformConstraint {}
#[repr(C, align(8))]
pub struct FEulerTransform {
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub scale: crate::bindings::core_u_object::FVector,
}
impl FEulerTransform {}
#[repr(C, align(16))]
pub struct FTransformNoScale {
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FQuat,
}
impl FTransformNoScale {}
#[repr(C, align(8))]
pub struct UAnimationDataSourceRegistry {
    __padding_end: [u8; 128],
}
impl UAnimationDataSourceRegistry {}
#[repr(transparent)]
pub struct ETransformConstraintType(pub u8);
impl ETransformConstraintType {
    pub const TRANSLATION: ETransformConstraintType = ETransformConstraintType(0);
    pub const ROTATION: ETransformConstraintType = ETransformConstraintType(1);
    pub const SCALE: ETransformConstraintType = ETransformConstraintType(2);
    pub const PARENT: ETransformConstraintType = ETransformConstraintType(3);
    pub const LOOK_AT: ETransformConstraintType = ETransformConstraintType(4);
}
#[repr(transparent)]
pub struct EConstraintType(pub u8);
impl EConstraintType {
    pub const TRANSFORM: EConstraintType = EConstraintType(0);
    pub const AIM: EConstraintType = EConstraintType(1);
    pub const MAX: EConstraintType = EConstraintType(2);
}
#[repr(transparent)]
pub struct EEulerRotationOrder(pub u8);
impl EEulerRotationOrder {
    pub const XYZ: EEulerRotationOrder = EEulerRotationOrder(0);
    pub const XZY: EEulerRotationOrder = EEulerRotationOrder(1);
    pub const YXZ: EEulerRotationOrder = EEulerRotationOrder(2);
    pub const YZX: EEulerRotationOrder = EEulerRotationOrder(3);
    pub const ZXY: EEulerRotationOrder = EEulerRotationOrder(4);
    pub const ZYX: EEulerRotationOrder = EEulerRotationOrder(5);
}
