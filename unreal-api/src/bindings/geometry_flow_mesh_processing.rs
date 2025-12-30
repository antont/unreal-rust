#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FMeshNormalsSettings {
    pub normals_type: EGeometryFlow_ComputeNormalsType,
    pub b_invert: bool,
    pub b_area_weighted: bool,
    pub b_angle_weighted: bool,
    pub angle_threshold_deg: f64,
}
#[repr(C, align(4))]
pub struct FMeshSimplifySettings {
    pub simplify_type: EGeometryFlow_MeshSimplifyType,
    pub target_type: EGeomtryFlow_MeshSimplifyTargetType,
    pub target_count: i32,
    pub target_fraction: f32,
    pub geometric_tolerance: f32,
    pub b_discard_attributes: bool,
    pub b_prevent_normal_flips: bool,
    pub b_preserve_sharp_edges: bool,
    pub b_allow_seam_collapse: bool,
    pub b_allow_seam_splits: bool,
    pub mesh_boundary_constraints: EGeometryFlow_EdgeRefineFlags,
    pub group_border_constraints: EGeometryFlow_EdgeRefineFlags,
    pub material_border_constraints: EGeometryFlow_EdgeRefineFlags,
}
#[repr(C, align(4))]
pub struct FMeshSolidifySettings {
    pub voxel_resolution: i32,
    pub winding_threshold: f32,
    pub surface_converge_steps: i32,
    pub extend_bounds: f32,
}
#[repr(C, align(4))]
pub struct FMeshTangentsSettings {
    pub tangents_type: EGeometryFlow_ComputeTangentsType,
    pub uv_layer: i32,
}
#[repr(C, align(8))]
pub struct FVoxMorphologyOpSettings {
    pub voxel_resolution: i32,
    pub distance: f64,
}
#[repr(C, align(4))]
pub struct FMeshMakeBakingCacheSettings {
    pub dimensions: FIntPoint,
    pub uv_layer: i32,
    pub thickness: f32,
}
#[repr(C, align(8))]
pub struct FBakeMeshTextureImageSettings {
    pub detail_uv_layer: i32,
    pub max_distance: f64,
}
#[repr(C, align(8))]
pub struct FBakeMeshMultiTextureSettings {}
#[repr(C, align(8))]
pub struct FBakeMeshNormalMapSettings {
    pub max_distance: f64,
}
#[repr(C, align(4))]
pub struct FGenerateConvexHullMeshSettings {
    pub b_prefilter_vertices: bool,
    pub prefilter_grid_resolution: i32,
}
#[repr(C, align(8))]
pub struct FMeshMakeCleanGeometrySettings {
    pub fill_holes_edge_count_thresh: i32,
    pub fill_holes_estimated_area_fraction: f64,
    pub b_discard_all_attributes: bool,
    pub b_clear_u_vs: bool,
    pub b_clear_normals: bool,
    pub b_clear_tangents: bool,
    pub b_clear_vertex_colors: bool,
    pub b_clear_material_i_ds: bool,
    pub b_output_mesh_vertex_normals: bool,
    pub b_output_overlay_vertex_normals: bool,
}
#[repr(C, align(4))]
pub struct FMeshNormalFlowSettings {
    pub max_remesh_iterations: i32,
    pub num_extra_projection_iterations: i32,
    pub b_flips: bool,
    pub b_splits: bool,
    pub b_collapses: bool,
    pub smoothing_type: EGeometryFlow_SmoothTypes,
    pub smoothing_strength: f32,
}
#[repr(C, align(4))]
pub struct FMeshRecalculateUVsSettings {
    pub unwrap_type: EGeometryFlow_RecalculateUVsUnwrapType,
    pub uv_layer: i32,
}
#[repr(C, align(4))]
pub struct FMeshRepackUVsSettings {
    pub uv_layer: i32,
    pub texture_resolution: i32,
    pub gutter_size: i32,
    pub b_allow_flips: bool,
    pub uv_scale: FVector2f,
    pub uv_translation: FVector2f,
}
#[repr(C, align(4))]
pub struct FMeshThickenSettings {}
#[repr(C, align(4))]
pub struct FGenerateConvexHullSettings {
    pub simplify_to_triangle_count: i32,
    pub b_prefilter_vertices: bool,
    pub prefilter_grid_resolution: i32,
}
#[repr(C, align(4))]
pub struct FGenerateSweptHullSettings {
    pub b_simplify_polygons: bool,
    pub sweep_axis: EGeometryFlow_ProjectedHullAxisMode,
    pub hull_tolerance: f32,
}
#[repr(C, align(4))]
pub struct FGenerateSimpleCollisionSettings {
    pub convex_hull_settings: FGenerateConvexHullSettings,
    pub swept_hull_settings: FGenerateSweptHullSettings,
}
