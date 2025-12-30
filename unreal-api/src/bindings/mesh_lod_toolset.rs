#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FGenerateStaticMeshLODProcessSettings {
    pub mesh_generator: EGenerateStaticMeshLODProcess_MeshGeneratorModes,
    pub solidify_voxel_resolution: i32,
    pub winding_threshold: f32,
    pub closure_distance: f32,
    pub b_prefilter_vertices: bool,
    pub prefilter_grid_resolution: i32,
}
#[repr(C, align(4))]
pub struct FGenerateStaticMeshLODProcess_PreprocessSettings {
    pub filter_group_layer: FName,
    pub thicken_weight_map_name: FName,
    pub thicken_amount: f32,
    pub b_is_thicken_amount_enabled: bool,
}
#[repr(C, align(4))]
pub struct FGenerateStaticMeshLODProcess_SimplifySettings {
    pub method: EGenerateStaticMeshLODProcess_SimplifyMethod,
    pub target_count: i32,
    pub target_percentage: f32,
    pub tolerance: f32,
}
#[repr(C, align(4))]
pub struct FGenerateStaticMeshLODProcess_NormalsSettings {
    pub method: EGenerateStaticMeshLODProcess_NormalsMethod,
    pub angle: f32,
}
#[repr(C, align(4))]
pub struct FGenerateStaticMeshLODProcess_UVSettings_PatchBuilder {
    pub curvature_alignment: f32,
    pub smoothing_steps: i32,
    pub smoothing_alpha: f32,
}
#[repr(C, align(4))]
pub struct FGenerateStaticMeshLODProcess_UVSettings {
    pub uv_method: EGenerateStaticMeshLODProcess_AutoUVMethod,
    pub num_uv_atlas_charts: i32,
    pub num_initial_patches: i32,
    pub merging_threshold: f32,
    pub max_angle_deviation: f32,
    pub patch_builder: FGenerateStaticMeshLODProcess_UVSettings_PatchBuilder,
}
#[repr(C, align(4))]
pub struct FGenerateStaticMeshLODProcess_TextureSettings {
    pub bake_resolution: EGenerateStaticMeshLODBakeResolution,
    pub bake_thickness: f32,
    pub b_combine_textures: bool,
}
#[repr(C, align(4))]
pub struct FGenerateStaticMeshLODProcess_CollisionSettings {
    pub collision_group_layer_name: FName,
    pub collision_type: EGenerateStaticMeshLODSimpleCollisionGeometryType,
    pub convex_triangle_count: i32,
    pub b_prefilter_vertices: bool,
    pub prefilter_grid_resolution: i32,
    pub b_simplify_polygons: bool,
    pub hull_tolerance: f32,
    pub sweep_axis: EGenerateStaticMeshLODProjectedHullAxisMode,
}
#[repr(C, align(8))]
pub struct FGenerateStaticMeshLOD_TextureConfig {
    pub texture: UPtr<UTexture2D>,
    pub constraint: EGenerateStaticMeshLOD_BakeConstraint,
}
#[repr(C, align(8))]
pub struct FGenerateStaticMeshLOD_MaterialConfig {
    pub material: UPtr<UMaterialInterface>,
    pub constraint: EGenerateStaticMeshLOD_BakeConstraint,
}
#[repr(C, align(4))]
pub struct FLODManagerLODInfo {
    pub vertex_count: i32,
    pub triangle_count: i32,
}
pub struct UAssetDefinition_StaticMeshLODGenerationSettings {}
pub struct UGenerateStaticMeshLODProcess {
    pub source_static_mesh: UPtr<UStaticMesh>,
    pub all_derived_textures: TSet<UPtr<UTexture2D>>,
    pub derived_normal_map_tex: UPtr<UTexture2D>,
    pub derived_multi_texture_bake_result: UPtr<UTexture2D>,
}
pub struct UGenerateStaticMeshLODAssetToolBuilder {}
pub struct UGenerateStaticMeshLODAssetToolOutputProperties {
    pub output_mode: EGenerateLODAssetOutputMode,
    pub new_asset_name: FString,
    pub b_save_input_as_hi_res_source: bool,
    pub generated_suffix: FString,
    pub b_show_output_mode: bool,
}
pub struct UGenerateStaticMeshLODAssetToolPresetProperties {
    pub preset: TWeakObjectPtr<UStaticMeshLODGenerationSettings>,
}
pub struct UGenerateStaticMeshLODAssetToolProperties {
    pub preprocessing: FGenerateStaticMeshLODProcess_PreprocessSettings,
    pub mesh_generation: FGenerateStaticMeshLODProcessSettings,
    pub simplification: FGenerateStaticMeshLODProcess_SimplifySettings,
    pub normals: FGenerateStaticMeshLODProcess_NormalsSettings,
    pub texture_baking: FGenerateStaticMeshLODProcess_TextureSettings,
    pub uv_generation: FGenerateStaticMeshLODProcess_UVSettings,
    pub simple_collision: FGenerateStaticMeshLODProcess_CollisionSettings,
    pub collision_group_layer_name: FName,
    pub group_layers_list: TArray<FString>,
}
pub struct UGenerateStaticMeshLODAssetToolTextureProperties {
    pub materials: TArray<FGenerateStaticMeshLOD_MaterialConfig>,
    pub textures: TArray<FGenerateStaticMeshLOD_TextureConfig>,
    pub preview_textures: TArray<UPtr<UTexture2D>>,
}
pub struct UGenerateStaticMeshLODAssetTool {
    pub output_properties: UPtr<UGenerateStaticMeshLODAssetToolOutputProperties>,
    pub basic_properties: UPtr<UGenerateStaticMeshLODAssetToolProperties>,
    pub preset_properties: UPtr<UGenerateStaticMeshLODAssetToolPresetProperties>,
    pub texture_properties: UPtr<UGenerateStaticMeshLODAssetToolTextureProperties>,
    pub collision_viz_settings: UPtr<UCollisionGeometryVisualizationProperties>,
    pub preview_with_background_compute: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub preview_textures: TArray<UPtr<UTexture2D>>,
    pub preview_materials: TArray<UPtr<UMaterialInterface>>,
    pub object_data: UPtr<UPhysicsObjectToolPropertySet>,
    pub collision_preview: UPtr<UPreviewGeometry>,
    pub generate_process: UPtr<UGenerateStaticMeshLODProcess>,
}
pub struct UStaticMeshLODGenerationSettings {
    pub preprocessing: FGenerateStaticMeshLODProcess_PreprocessSettings,
    pub mesh_generation: FGenerateStaticMeshLODProcessSettings,
    pub simplification: FGenerateStaticMeshLODProcess_SimplifySettings,
    pub normals: FGenerateStaticMeshLODProcess_NormalsSettings,
    pub texture_baking: FGenerateStaticMeshLODProcess_TextureSettings,
    pub uv_generation: FGenerateStaticMeshLODProcess_UVSettings,
    pub simple_collision: FGenerateStaticMeshLODProcess_CollisionSettings,
}
pub struct UStaticMeshLODGenerationSettingsFactory {}
pub struct ULODManagerToolBuilder {}
pub struct ULODManagerLODProperties {
    pub source_lo_ds: TArray<FLODManagerLODInfo>,
    pub hi_res_source: TArray<FLODManagerLODInfo>,
    pub render_lo_ds: TArray<FLODManagerLODInfo>,
    pub b_nanite_enabled: bool,
    pub keep_triangle_percent: f32,
    pub materials: TArray<FStaticMaterial>,
}
pub struct ULODManagerPreviewLODProperties {
    pub visible_lod: FString,
    pub lod_names_list: TArray<FString>,
    pub b_show_seams: bool,
}
pub struct ULODManagerActionPropertySet {}
pub struct ULODManagerHiResSourceModelActions {}
pub struct ULODManagerMaterialActions {}
pub struct ULODManagerToolChangeTarget {}
pub struct ILODManagerToolChangeTarget {}
pub struct ULODManagerTool {
    pub lod_info_properties: UPtr<ULODManagerLODProperties>,
    pub lod_preview_properties: UPtr<ULODManagerPreviewLODProperties>,
    pub hi_res_source_model_actions: UPtr<ULODManagerHiResSourceModelActions>,
    pub material_actions: UPtr<ULODManagerMaterialActions>,
    pub lod_preview: UPtr<UPreviewMesh>,
    pub lod_preview_lines: UPtr<UPreviewGeometry>,
}
