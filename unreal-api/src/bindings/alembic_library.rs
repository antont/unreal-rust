#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FAbcCompressionSettings {
    pub b_merge_meshes: bool,
    pub b_bake_matrix_animation: bool,
    pub base_calculation_type: EBaseCalculationType,
    pub percentage_of_total_bases: f32,
    pub max_number_of_bases: i32,
    pub minimum_number_of_vertex_influence_percentage: f32,
}
#[repr(C, align(4))]
pub struct FAbcSamplingSettings {
    pub sampling_type: EAlembicSamplingType,
    pub frame_steps: i32,
    pub time_steps: f32,
    pub frame_start: i32,
    pub frame_end: i32,
    pub b_skip_empty: bool,
}
#[repr(C, align(4))]
pub struct FAbcNormalGenerationSettings {
    pub b_force_one_smoothing_group_per_object: bool,
    pub hard_edge_angle_threshold: f32,
    pub b_recompute_normals: bool,
    pub b_ignore_degenerate_triangles: bool,
    pub b_skip_computing_tangents: bool,
}
#[repr(C, align(1))]
pub struct FAbcMaterialSettings {
    pub b_create_materials: bool,
    pub b_find_materials: bool,
}
#[repr(C, align(1))]
pub struct FAbcStaticMeshSettings {
    pub b_merge_meshes: bool,
    pub b_propagate_matrix_transformations: bool,
    pub b_generate_lightmap_u_vs: bool,
}
#[repr(C, align(8))]
pub struct FAbcConversionSettings {
    pub preset: EAbcConversionPreset,
    pub b_flip_u: bool,
    pub b_flip_v: bool,
    pub scale: FVector,
    pub rotation: FVector,
}
#[repr(C, align(4))]
pub struct FAbcGeometryCacheSettings {
    pub b_flatten_tracks: bool,
    pub b_store_imported_vertex_numbers: bool,
    pub b_apply_constant_topology_optimizations: bool,
    pub b_calculate_motion_vectors_during_import_deprecated: bool,
    pub motion_vectors: EAbcGeometryCacheMotionVectorsImport,
    pub b_optimize_index_buffers: bool,
    pub compressed_position_precision: f32,
    pub compressed_texture_coordinates_number_of_bits: i32,
}
pub struct UAbcAssetImportData {
    pub track_names: TArray<FString>,
    pub sampling_settings: FAbcSamplingSettings,
    pub normal_generation_settings: FAbcNormalGenerationSettings,
    pub material_settings: FAbcMaterialSettings,
    pub compression_settings: FAbcCompressionSettings,
    pub static_mesh_settings: FAbcStaticMeshSettings,
    pub geometry_cache_settings: FAbcGeometryCacheSettings,
    pub conversion_settings: FAbcConversionSettings,
}
pub struct UAbcImportSettings {
    pub import_type: EAlembicImportType,
    pub sampling_settings: FAbcSamplingSettings,
    pub normal_generation_settings: FAbcNormalGenerationSettings,
    pub material_settings: FAbcMaterialSettings,
    pub compression_settings: FAbcCompressionSettings,
    pub static_mesh_settings: FAbcStaticMeshSettings,
    pub geometry_cache_settings: FAbcGeometryCacheSettings,
    pub conversion_settings: FAbcConversionSettings,
}
pub struct UAlembicTestCommandlet {}
