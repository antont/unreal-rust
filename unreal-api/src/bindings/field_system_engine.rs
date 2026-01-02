#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FFieldObjectCommands {
    pub target_names: TArray<FName>,
    pub root_nodes: TArray<UPtr<UFieldNodeBase>>,
    pub meta_datas: TArray<UPtr<UFieldSystemMetaData>>,
}
impl FFieldObjectCommands {}
#[repr(C, align(8))]
pub struct AFieldSystemActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub field_system_component: UPtr<UFieldSystemComponent>,
}
impl AFieldSystemActor {}
#[repr(C, align(8))]
pub struct UFieldSystem {
    __padding_end: [u8; 64],
}
impl UFieldSystem {}
#[repr(C, align(16))]
pub struct UFieldSystemComponent {
    #[doc(hidden)]
    __padding_1504: [u8; 1504],
    pub field_system: UPtr<UFieldSystem>,
    __padding_end: [u8; 200],
}
impl UFieldSystemComponent {}
#[repr(C, align(8))]
pub struct UFieldSystemMetaData {
    __padding_end: [u8; 240],
}
impl UFieldSystemMetaData {}
#[repr(C, align(8))]
pub struct UFieldSystemMetaDataIteration {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub iterations: i32,
    __padding_end: [u8; 4],
}
impl UFieldSystemMetaDataIteration {}
#[repr(C, align(8))]
pub struct UFieldSystemMetaDataProcessingResolution {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub resolution_type: crate::bindings::chaos::EFieldResolutionType,
    __padding_end: [u8; 7],
}
impl UFieldSystemMetaDataProcessingResolution {}
#[repr(C, align(8))]
pub struct UFieldSystemMetaDataFilter {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub filter_type: crate::bindings::chaos::EFieldFilterType,
    pub object_type: crate::bindings::chaos::EFieldObjectType,
    pub position_type: crate::bindings::chaos::EFieldPositionType,
    __padding_end: [u8; 5],
}
impl UFieldSystemMetaDataFilter {}
#[repr(C, align(8))]
pub struct UFieldNodeBase {
    __padding_end: [u8; 240],
}
impl UFieldNodeBase {}
#[repr(C, align(8))]
pub struct UFieldNodeInt {
    __padding_end: [u8; 240],
}
impl UFieldNodeInt {}
#[repr(C, align(8))]
pub struct UFieldNodeFloat {
    __padding_end: [u8; 240],
}
impl UFieldNodeFloat {}
#[repr(C, align(8))]
pub struct UFieldNodeVector {
    __padding_end: [u8; 240],
}
impl UFieldNodeVector {}
#[repr(C, align(8))]
pub struct UUniformInteger {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: i32,
    __padding_end: [u8; 4],
}
impl UUniformInteger {}
#[repr(C, align(8))]
pub struct URadialIntMask {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub radius: f32,
    pub position: crate::bindings::core_u_object::FVector,
    pub interior_value: i32,
    pub exterior_value: i32,
    pub set_mask_condition: crate::bindings::chaos::ESetMaskConditionType,
    __padding_end: [u8; 7],
}
impl URadialIntMask {}
#[repr(C, align(8))]
pub struct UUniformScalar {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    __padding_end: [u8; 4],
}
impl UUniformScalar {}
#[repr(C, align(8))]
pub struct UWaveScalar {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub position: crate::bindings::core_u_object::FVector,
    pub wavelength: f32,
    pub period: f32,
    pub function: crate::bindings::chaos::EWaveFunctionType,
    pub falloff: crate::bindings::chaos::EFieldFalloffType,
    __padding_end: [u8; 6],
}
impl UWaveScalar {}
#[repr(C, align(8))]
pub struct URadialFalloff {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub radius: f32,
    pub position: crate::bindings::core_u_object::FVector,
    pub falloff: crate::bindings::chaos::EFieldFalloffType,
    __padding_end: [u8; 7],
}
impl URadialFalloff {}
#[repr(C, align(8))]
pub struct UPlaneFalloff {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub distance: f32,
    pub position: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub falloff: crate::bindings::chaos::EFieldFalloffType,
    __padding_end: [u8; 7],
}
impl UPlaneFalloff {}
#[repr(C, align(16))]
pub struct UBoxFalloff {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub falloff: crate::bindings::chaos::EFieldFalloffType,
    __padding_end: [u8; 15],
}
impl UBoxFalloff {}
#[repr(C, align(16))]
pub struct UNoiseField {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub min_range: f32,
    pub max_range: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
}
impl UNoiseField {}
#[repr(C, align(8))]
pub struct UUniformVector {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub direction: crate::bindings::core_u_object::FVector,
}
impl UUniformVector {}
#[repr(C, align(8))]
pub struct URadialVector {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub position: crate::bindings::core_u_object::FVector,
}
impl URadialVector {}
#[repr(C, align(8))]
pub struct URandomVector {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    __padding_end: [u8; 4],
}
impl URandomVector {}
#[repr(C, align(8))]
pub struct UOperatorField {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub right_field: UPtr<UFieldNodeBase>,
    pub left_field: UPtr<UFieldNodeBase>,
    pub operation: crate::bindings::chaos::EFieldOperationType,
    __padding_end: [u8; 7],
}
impl UOperatorField {}
#[repr(C, align(8))]
pub struct UToIntegerField {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub float_field: UPtr<UFieldNodeFloat>,
}
impl UToIntegerField {}
#[repr(C, align(8))]
pub struct UToFloatField {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub int_field: UPtr<UFieldNodeInt>,
}
impl UToFloatField {}
#[repr(C, align(8))]
pub struct UCullingField {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub culling: UPtr<UFieldNodeBase>,
    pub field: UPtr<UFieldNodeBase>,
    pub operation: crate::bindings::chaos::EFieldCullingOperationType,
    __padding_end: [u8; 7],
}
impl UCullingField {}
#[repr(C, align(8))]
pub struct UReturnResultsTerminal {
    __padding_end: [u8; 240],
}
impl UReturnResultsTerminal {}
