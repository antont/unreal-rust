#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FDynamicMeshChangeInfo {
    pub ty: EDynamicMeshChangeType,
    pub flags: EDynamicMeshAttributeChangeFlags,
    pub b_is_revert_change: bool,
}
pub struct UDynamicMeshProcessorBlueprint {
    pub b_requires_game_thread: bool,
}
pub struct UMeshCommandChangeTarget {}
pub struct IMeshCommandChangeTarget {}
pub struct UMeshReplacementCommandChangeTarget {}
pub struct IMeshReplacementCommandChangeTarget {}
pub struct UMeshVertexCommandChangeTarget {}
pub struct IMeshVertexCommandChangeTarget {}
pub struct UBaseDynamicMeshComponent {
    pub b_explicit_show_wireframe: bool,
    pub wireframe_color: FLinearColor,
    pub color_mode: EDynamicMeshComponentColorOverrideMode,
    pub constant_color: FColor,
    pub color_space_mode: EDynamicMeshVertexColorTransformMode,
    pub b_two_sided: bool,
    pub b_enable_flat_shading: bool,
    pub b_enable_view_mode_overrides: bool,
    pub override_render_material: UPtr<UMaterialInterface>,
    pub secondary_render_material: UPtr<UMaterialInterface>,
    pub wireframe_material_override: UPtr<UMaterialInterface>,
    pub secondary_wireframe_material_override: UPtr<UMaterialInterface>,
    pub b_enable_raytracing: bool,
    pub draw_path: EDynamicMeshDrawPath,
    pub distance_field_mode_deprecated: EDynamicMeshComponentDistanceFieldMode,
    pub base_materials: TArray<UPtr<UMaterialInterface>>,
}
pub struct UDynamicMeshComponent {
    pub mesh_object: UPtr<UDynamicMesh>,
    pub b_allows_geometry_selection: bool,
    pub tangents_type: EDynamicMeshComponentTangentsMode,
    pub collision_type: ECollisionTraceFlag,
    pub b_use_async_cooking: bool,
    pub b_enable_complex_collision: bool,
    pub b_defer_collision_updates: bool,
    pub b_disable_mesh_uv_hit_results: bool,
    pub mesh_body_setup: UPtr<UBodySetup>,
    pub agg_geom: FKAggregateGeom,
    pub async_body_setup_queue: TArray<UPtr<UBodySetup>>,
}
pub struct ADynamicMeshActor {
    pub dynamic_mesh_component: UPtr<UDynamicMeshComponent>,
    pub b_enable_compute_mesh_pool: bool,
    pub dynamic_mesh_pool: UPtr<UDynamicMeshPool>,
}
pub struct UDynamicMeshGenerator {}
pub struct UDynamicMesh {
    pub mesh_modified_bp_event: FDynamicMesh_MeshModifiedBPEvent,
    pub mesh_generator: UPtr<UDynamicMeshGenerator>,
    pub b_enable_mesh_generator: bool,
}
pub struct UDynamicMeshPool {
    pub cached_meshes: TArray<UPtr<UDynamicMesh>>,
    pub all_created_meshes: TArray<UPtr<UDynamicMesh>>,
}
