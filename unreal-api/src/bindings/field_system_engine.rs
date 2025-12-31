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
pub struct AFieldSystemActor {
    pub field_system_component: UPtr<UFieldSystemComponent>,
}
pub struct UFieldSystem {}
pub struct UFieldSystemComponent {
    pub field_system: UPtr<UFieldSystem>,
    pub b_is_world_field: bool,
    pub b_is_chaos_field: bool,
    pub supported_solvers: TArray<
        TSoftObjectPtr<crate::bindings::chaos_solver_engine::AChaosSolverActor>,
    >,
    pub construction_commands: FFieldObjectCommands,
    pub buffer_commands: FFieldObjectCommands,
}
pub struct UFieldSystemMetaData {}
pub struct UFieldSystemMetaDataIteration {
    pub iterations: i32,
}
pub struct UFieldSystemMetaDataProcessingResolution {
    pub resolution_type: crate::bindings::chaos::EFieldResolutionType,
}
pub struct UFieldSystemMetaDataFilter {
    pub filter_type: crate::bindings::chaos::EFieldFilterType,
    pub object_type: crate::bindings::chaos::EFieldObjectType,
    pub position_type: crate::bindings::chaos::EFieldPositionType,
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
    pub position: crate::bindings::core_u_object::FVector,
    pub interior_value: i32,
    pub exterior_value: i32,
    pub set_mask_condition: crate::bindings::chaos::ESetMaskConditionType,
}
pub struct UUniformScalar {
    pub magnitude: f32,
}
pub struct UWaveScalar {
    pub magnitude: f32,
    pub position: crate::bindings::core_u_object::FVector,
    pub wavelength: f32,
    pub period: f32,
    pub function: crate::bindings::chaos::EWaveFunctionType,
    pub falloff: crate::bindings::chaos::EFieldFalloffType,
}
pub struct URadialFalloff {
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub radius: f32,
    pub position: crate::bindings::core_u_object::FVector,
    pub falloff: crate::bindings::chaos::EFieldFalloffType,
}
pub struct UPlaneFalloff {
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub distance: f32,
    pub position: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub falloff: crate::bindings::chaos::EFieldFalloffType,
}
pub struct UBoxFalloff {
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub falloff: crate::bindings::chaos::EFieldFalloffType,
}
pub struct UNoiseField {
    pub min_range: f32,
    pub max_range: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
}
pub struct UUniformVector {
    pub magnitude: f32,
    pub direction: crate::bindings::core_u_object::FVector,
}
pub struct URadialVector {
    pub magnitude: f32,
    pub position: crate::bindings::core_u_object::FVector,
}
pub struct URandomVector {
    pub magnitude: f32,
}
pub struct UOperatorField {
    pub magnitude: f32,
    pub right_field: UPtr<UFieldNodeBase>,
    pub left_field: UPtr<UFieldNodeBase>,
    pub operation: crate::bindings::chaos::EFieldOperationType,
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
    pub operation: crate::bindings::chaos::EFieldCullingOperationType,
}
pub struct UReturnResultsTerminal {}
