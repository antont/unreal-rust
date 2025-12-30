#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FAddSolverDeformerDataflowNode {
    pub physics_solvers: TArray<FDataflowSimulationProperty>,
    pub simulation_time: FDataflowSimulationTime,
    pub mesh_deformer: UPtr<UOptimusDeformer>,
    pub deformer_numeric_inputs: TArray<FDataflowNumericTypes>,
    pub deformer_vector_inputs: TArray<FDataflowVectorTypes>,
    pub deformer_string_inputs: TArray<FDataflowStringTypes>,
    pub deformer_bool_inputs: TArray<FDataflowBoolTypes>,
    pub deformer_transform_inputs: TArray<FDataflowTransformTypes>,
    pub deformer_numeric_arrays: TArray<FDataflowNumericArrayTypes>,
    pub deformer_vector_arrays: TArray<FDataflowVectorArrayTypes>,
    pub deformer_string_arrays: TArray<FDataflowStringArrayTypes>,
    pub deformer_bool_arrays: TArray<FDataflowBoolArrayTypes>,
    pub deformer_transform_arrays: TArray<FDataflowTransformArrayTypes>,
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
    pub simulation_asset: FDataflowSimulationAsset,
    pub groom_components: TSet<UPtr<UGroomComponent>>,
    pub mesh_deformer: UPtr<UMeshDeformer>,
    pub deformer_instance: UPtr<UMeshDeformerInstance>,
    pub deformer_settings: UPtr<UMeshDeformerInstanceSettings>,
    pub collision_components: TMap<UPtr<UMeshComponent>, i32>,
}
