#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAxis {
    pub axis: crate::bindings::core_u_object::FVector,
    pub b_in_local_space: bool,
}
#[repr(C, align(8))]
pub struct FNodeChain {
    pub nodes: TArray<FName>,
}
#[repr(C, align(4))]
pub struct FNodeObject {
    pub name: FName,
    pub parent_name: FName,
}
#[repr(C, align(8))]
pub struct FNodeHierarchyData {
    pub nodes: TArray<FNodeObject>,
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub node_name_to_index_mapping: TMap<FName, i32>,
}
#[repr(C, align(8))]
pub struct FNodeHierarchyWithUserData {
    pub hierarchy: FNodeHierarchyData,
}
#[repr(C, align(16))]
pub struct FCCDIKChainLink {}
#[repr(C, align(1))]
pub struct FFilterOptionPerAxis {
    pub b_x: bool,
    pub b_y: bool,
    pub b_z: bool,
}
#[repr(C, align(1))]
pub struct FTransformFilter {
    pub translation_filter: FFilterOptionPerAxis,
    pub rotation_filter: FFilterOptionPerAxis,
    pub scale_filter: FFilterOptionPerAxis,
}
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
#[repr(C, align(16))]
pub struct FConstraintOffset {
    pub translation: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FQuat,
    pub scale: crate::bindings::core_u_object::FVector,
    pub parent: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(4))]
pub struct FTransformConstraint {
    pub operator: FConstraintDescription,
    pub source_node: FName,
    pub target_node: FName,
    pub weight: f32,
    pub b_maintain_offset: bool,
}
#[repr(C, align(8))]
pub struct FConstraintDescriptionEx {
    pub axes_filter_option: FFilterOptionPerAxis,
}
#[repr(C, align(8))]
pub struct FTransformConstraintDescription {
    pub transform_type: ETransformConstraintType,
}
#[repr(C, align(8))]
pub struct FAimConstraintDescription {
    pub look_at_axis: FAxis,
    pub look_up_axis: FAxis,
    pub b_use_look_up: bool,
    pub look_up_target: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FConstraintDescriptor {
    pub ty: EConstraintType,
}
#[repr(C, align(16))]
pub struct FConstraintData {
    pub constraint: FConstraintDescriptor,
    pub weight: f32,
    pub b_maintain_offset: bool,
    pub offset: crate::bindings::core_u_object::FTransform,
    pub current_transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FEulerTransform {
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub scale: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FFABRIKChainLink {}
#[repr(C, align(16))]
pub struct FTransformNoScale {
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FQuat,
}
pub struct UAnimationDataSourceRegistry {
    pub data_sources: TMap<
        FName,
        TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
    >,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETransformConstraintType(pub u8);
impl ETransformConstraintType {
    pub const TRANSLATION: ETransformConstraintType = ETransformConstraintType(0);
    pub const ROTATION: ETransformConstraintType = ETransformConstraintType(1);
    pub const SCALE: ETransformConstraintType = ETransformConstraintType(2);
    pub const PARENT: ETransformConstraintType = ETransformConstraintType(3);
    pub const LOOK_AT: ETransformConstraintType = ETransformConstraintType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EConstraintType(pub u8);
impl EConstraintType {
    pub const TRANSFORM: EConstraintType = EConstraintType(0);
    pub const AIM: EConstraintType = EConstraintType(1);
    pub const MAX: EConstraintType = EConstraintType(2);
}
#[allow(non_camel_case_types)]
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
