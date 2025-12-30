#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FFieldObjectCommands {
    pub target_names: TArray<FName>,
    pub root_nodes: TArray<UPtr<UFieldNodeBase>>,
    pub meta_datas: TArray<UPtr<UFieldSystemMetaData>>,
}
pub struct AFieldSystemActor {
    pub field_system_component: UPtr<UFieldSystemComponent>,
}
pub struct UFieldSystem {}
pub struct UFieldSystemComponent {
    pub field_system: UPtr<UFieldSystem>,
    pub b_is_world_field: bool,
    pub b_is_chaos_field: bool,
    pub supported_solvers: TArray<TSoftObjectPtr<AChaosSolverActor>>,
    pub construction_commands: FFieldObjectCommands,
    pub buffer_commands: FFieldObjectCommands,
}
pub struct UFieldSystemMetaData {}
pub struct UFieldSystemMetaDataIteration {
    pub iterations: i32,
}
pub struct UFieldSystemMetaDataProcessingResolution {
    pub resolution_type: EFieldResolutionType,
}
pub struct UFieldSystemMetaDataFilter {
    pub filter_type: EFieldFilterType,
    pub object_type: EFieldObjectType,
    pub position_type: EFieldPositionType,
}
pub struct UFieldNodeBase {}
pub struct UFieldNodeInt {}
pub struct UFieldNodeFloat {}
pub struct UFieldNodeVector {}
pub struct UUniformInteger {
    pub magnitude: i32,
}
pub struct URadialIntMask {
    pub radius: f32,
    pub position: FVector,
    pub interior_value: i32,
    pub exterior_value: i32,
    pub set_mask_condition: ESetMaskConditionType,
}
pub struct UUniformScalar {
    pub magnitude: f32,
}
pub struct UWaveScalar {
    pub magnitude: f32,
    pub position: FVector,
    pub wavelength: f32,
    pub period: f32,
    pub function: EWaveFunctionType,
    pub falloff: EFieldFalloffType,
}
pub struct URadialFalloff {
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub radius: f32,
    pub position: FVector,
    pub falloff: EFieldFalloffType,
}
pub struct UPlaneFalloff {
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub distance: f32,
    pub position: FVector,
    pub normal: FVector,
    pub falloff: EFieldFalloffType,
}
pub struct UBoxFalloff {
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub transform: FTransform,
    pub falloff: EFieldFalloffType,
}
pub struct UNoiseField {
    pub min_range: f32,
    pub max_range: f32,
    pub transform: FTransform,
}
pub struct UUniformVector {
    pub magnitude: f32,
    pub direction: FVector,
}
pub struct URadialVector {
    pub magnitude: f32,
    pub position: FVector,
}
pub struct URandomVector {
    pub magnitude: f32,
}
pub struct UOperatorField {
    pub magnitude: f32,
    pub right_field: UPtr<UFieldNodeBase>,
    pub left_field: UPtr<UFieldNodeBase>,
    pub operation: EFieldOperationType,
}
pub struct UToIntegerField {
    pub float_field: UPtr<UFieldNodeFloat>,
}
pub struct UToFloatField {
    pub int_field: UPtr<UFieldNodeInt>,
}
pub struct UCullingField {
    pub culling: UPtr<UFieldNodeBase>,
    pub field: UPtr<UFieldNodeBase>,
    pub operation: EFieldCullingOperationType,
}
pub struct UReturnResultsTerminal {}
