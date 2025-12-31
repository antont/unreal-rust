#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAddSolverDeformerDataflowNode {
    pub physics_solvers: TArray<
        crate::bindings::dataflow_simulation::FDataflowSimulationProperty,
    >,
    pub simulation_time: crate::bindings::dataflow_simulation::FDataflowSimulationTime,
    pub mesh_deformer: UPtr<crate::bindings::optimus_core::UOptimusDeformer>,
    pub deformer_numeric_inputs: TArray<
        crate::bindings::dataflow_core::FDataflowNumericTypes,
    >,
    pub deformer_vector_inputs: TArray<
        crate::bindings::dataflow_core::FDataflowVectorTypes,
    >,
    pub deformer_string_inputs: TArray<
        crate::bindings::dataflow_core::FDataflowStringTypes,
    >,
    pub deformer_bool_inputs: TArray<crate::bindings::dataflow_core::FDataflowBoolTypes>,
    pub deformer_transform_inputs: TArray<
        crate::bindings::dataflow_core::FDataflowTransformTypes,
    >,
    pub deformer_numeric_arrays: TArray<
        crate::bindings::dataflow_core::FDataflowNumericArrayTypes,
    >,
    pub deformer_vector_arrays: TArray<
        crate::bindings::dataflow_core::FDataflowVectorArrayTypes,
    >,
    pub deformer_string_arrays: TArray<
        crate::bindings::dataflow_core::FDataflowStringArrayTypes,
    >,
    pub deformer_bool_arrays: TArray<
        crate::bindings::dataflow_core::FDataflowBoolArrayTypes,
    >,
    pub deformer_transform_arrays: TArray<
        crate::bindings::dataflow_core::FDataflowTransformArrayTypes,
    >,
}
#[repr(C, align(8))]
pub struct FGroomSolverSettings {
    pub max_lod_distance: f32,
    pub min_lod_distance: f32,
    pub max_points_count: f32,
}
#[repr(C, align(8))]
pub struct FDataflowGroomSolverProxy {}
pub struct UGroomSolverComponent {
    pub solver_settings: FGroomSolverSettings,
    pub simulation_asset: crate::bindings::dataflow_simulation::FDataflowSimulationAsset,
    pub groom_components: TSet<
        UPtr<crate::bindings::hair_strands_core::UGroomComponent>,
    >,
    pub mesh_deformer: UPtr<crate::bindings::engine::UMeshDeformer>,
    pub deformer_instance: UPtr<crate::bindings::engine::UMeshDeformerInstance>,
    pub deformer_settings: UPtr<crate::bindings::engine::UMeshDeformerInstanceSettings>,
    pub collision_components: TMap<UPtr<crate::bindings::engine::UMeshComponent>, i32>,
}
