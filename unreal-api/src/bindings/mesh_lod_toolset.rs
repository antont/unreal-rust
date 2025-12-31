#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
    pub texture: UPtr<crate::bindings::engine::UTexture2D>,
    pub constraint: EGenerateStaticMeshLOD_BakeConstraint,
}
#[repr(C, align(8))]
pub struct FGenerateStaticMeshLOD_MaterialConfig {
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub constraint: EGenerateStaticMeshLOD_BakeConstraint,
}
#[repr(C, align(4))]
pub struct FLODManagerLODInfo {
    pub vertex_count: i32,
    pub triangle_count: i32,
}
pub struct UAssetDefinition_StaticMeshLODGenerationSettings {}
pub struct UGenerateStaticMeshLODProcess {
    pub source_static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub all_derived_textures: TSet<UPtr<crate::bindings::engine::UTexture2D>>,
    pub derived_normal_map_tex: UPtr<crate::bindings::engine::UTexture2D>,
    pub derived_multi_texture_bake_result: UPtr<crate::bindings::engine::UTexture2D>,
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
    pub preview_textures: TArray<UPtr<crate::bindings::engine::UTexture2D>>,
}
pub struct UGenerateStaticMeshLODAssetTool {
    pub output_properties: UPtr<UGenerateStaticMeshLODAssetToolOutputProperties>,
    pub basic_properties: UPtr<UGenerateStaticMeshLODAssetToolProperties>,
    pub preset_properties: UPtr<UGenerateStaticMeshLODAssetToolPresetProperties>,
    pub texture_properties: UPtr<UGenerateStaticMeshLODAssetToolTextureProperties>,
    pub collision_viz_settings: UPtr<
        crate::bindings::mesh_modeling_tools_exp::UCollisionGeometryVisualizationProperties,
    >,
    pub preview_with_background_compute: UPtr<
        crate::bindings::modeling_components::UMeshOpPreviewWithBackgroundCompute,
    >,
    pub preview_textures: TArray<UPtr<crate::bindings::engine::UTexture2D>>,
    pub preview_materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub object_data: UPtr<
        crate::bindings::mesh_modeling_tools_exp::UPhysicsObjectToolPropertySet,
    >,
    pub collision_preview: UPtr<crate::bindings::modeling_components::UPreviewGeometry>,
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
    pub materials: TArray<crate::bindings::engine::FStaticMaterial>,
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
    pub lod_preview: UPtr<crate::bindings::modeling_components::UPreviewMesh>,
    pub lod_preview_lines: UPtr<crate::bindings::modeling_components::UPreviewGeometry>,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGenerateStaticMeshLODProcess_MeshGeneratorModes(pub u8);
impl EGenerateStaticMeshLODProcess_MeshGeneratorModes {
    pub const SOLIDIFY: EGenerateStaticMeshLODProcess_MeshGeneratorModes = EGenerateStaticMeshLODProcess_MeshGeneratorModes(
        0,
    );
    pub const SOLIDIFY_AND_CLOSE: EGenerateStaticMeshLODProcess_MeshGeneratorModes = EGenerateStaticMeshLODProcess_MeshGeneratorModes(
        1,
    );
    pub const CLEAN_AND_SIMPLIFY: EGenerateStaticMeshLODProcess_MeshGeneratorModes = EGenerateStaticMeshLODProcess_MeshGeneratorModes(
        2,
    );
    pub const CONVEX_HULL: EGenerateStaticMeshLODProcess_MeshGeneratorModes = EGenerateStaticMeshLODProcess_MeshGeneratorModes(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGenerateStaticMeshLODProcess_SimplifyMethod(pub u8);
impl EGenerateStaticMeshLODProcess_SimplifyMethod {
    pub const TRIANGLE_COUNT: EGenerateStaticMeshLODProcess_SimplifyMethod = EGenerateStaticMeshLODProcess_SimplifyMethod(
        0,
    );
    pub const VERTEX_COUNT: EGenerateStaticMeshLODProcess_SimplifyMethod = EGenerateStaticMeshLODProcess_SimplifyMethod(
        1,
    );
    pub const TRIANGLE_PERCENTAGE: EGenerateStaticMeshLODProcess_SimplifyMethod = EGenerateStaticMeshLODProcess_SimplifyMethod(
        2,
    );
    pub const GEOMETRIC_TOLERANCE: EGenerateStaticMeshLODProcess_SimplifyMethod = EGenerateStaticMeshLODProcess_SimplifyMethod(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGenerateStaticMeshLODProcess_NormalsMethod(pub u8);
impl EGenerateStaticMeshLODProcess_NormalsMethod {
    pub const FROM_ANGLE_THRESHOLD: EGenerateStaticMeshLODProcess_NormalsMethod = EGenerateStaticMeshLODProcess_NormalsMethod(
        0,
    );
    pub const PER_VERTEX: EGenerateStaticMeshLODProcess_NormalsMethod = EGenerateStaticMeshLODProcess_NormalsMethod(
        1,
    );
    pub const PER_TRIANGLE: EGenerateStaticMeshLODProcess_NormalsMethod = EGenerateStaticMeshLODProcess_NormalsMethod(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGenerateStaticMeshLODProcess_AutoUVMethod(pub u8);
impl EGenerateStaticMeshLODProcess_AutoUVMethod {
    pub const PATCH_BUILDER: EGenerateStaticMeshLODProcess_AutoUVMethod = EGenerateStaticMeshLODProcess_AutoUVMethod(
        0,
    );
    pub const UV_ATLAS: EGenerateStaticMeshLODProcess_AutoUVMethod = EGenerateStaticMeshLODProcess_AutoUVMethod(
        1,
    );
    pub const X_ATLAS: EGenerateStaticMeshLODProcess_AutoUVMethod = EGenerateStaticMeshLODProcess_AutoUVMethod(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGenerateStaticMeshLODBakeResolution(pub i32);
impl EGenerateStaticMeshLODBakeResolution {
    pub const RESOLUTION16: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        16,
    );
    pub const RESOLUTION32: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        32,
    );
    pub const RESOLUTION64: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        64,
    );
    pub const RESOLUTION128: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        128,
    );
    pub const RESOLUTION256: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        256,
    );
    pub const RESOLUTION512: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        512,
    );
    pub const RESOLUTION1024: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        1024,
    );
    pub const RESOLUTION2048: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        2048,
    );
    pub const RESOLUTION4096: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        4096,
    );
    pub const RESOLUTION8192: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        8192,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGenerateStaticMeshLODSimpleCollisionGeometryType(pub u8);
impl EGenerateStaticMeshLODSimpleCollisionGeometryType {
    pub const ALIGNED_BOXES: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        0,
    );
    pub const ORIENTED_BOXES: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        1,
    );
    pub const MINIMAL_SPHERES: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        2,
    );
    pub const CAPSULES: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        3,
    );
    pub const CONVEX_HULLS: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        4,
    );
    pub const SWEPT_HULLS: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        5,
    );
    pub const MIN_VOLUME: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        6,
    );
    pub const NONE: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        7,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGenerateStaticMeshLODProjectedHullAxisMode(pub u8);
impl EGenerateStaticMeshLODProjectedHullAxisMode {
    pub const X: EGenerateStaticMeshLODProjectedHullAxisMode = EGenerateStaticMeshLODProjectedHullAxisMode(
        0,
    );
    pub const Y: EGenerateStaticMeshLODProjectedHullAxisMode = EGenerateStaticMeshLODProjectedHullAxisMode(
        1,
    );
    pub const Z: EGenerateStaticMeshLODProjectedHullAxisMode = EGenerateStaticMeshLODProjectedHullAxisMode(
        2,
    );
    pub const SMALLEST_BOX_DIMENSION: EGenerateStaticMeshLODProjectedHullAxisMode = EGenerateStaticMeshLODProjectedHullAxisMode(
        3,
    );
    pub const SMALLEST_VOLUME: EGenerateStaticMeshLODProjectedHullAxisMode = EGenerateStaticMeshLODProjectedHullAxisMode(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGenerateStaticMeshLOD_BakeConstraint(pub i32);
impl EGenerateStaticMeshLOD_BakeConstraint {
    pub const NO_CONSTRAINT: EGenerateStaticMeshLOD_BakeConstraint = EGenerateStaticMeshLOD_BakeConstraint(
        0,
    );
    pub const DO_NOT_BAKE: EGenerateStaticMeshLOD_BakeConstraint = EGenerateStaticMeshLOD_BakeConstraint(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGenerateLODAssetOutputMode(pub u8);
impl EGenerateLODAssetOutputMode {
    pub const CREATE_NEW_ASSET: EGenerateLODAssetOutputMode = EGenerateLODAssetOutputMode(
        0,
    );
    pub const UPDATE_EXISTING_ASSET: EGenerateLODAssetOutputMode = EGenerateLODAssetOutputMode(
        1,
    );
}
