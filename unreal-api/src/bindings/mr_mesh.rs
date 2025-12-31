#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FMRMeshConfiguration {}
pub struct UMockDataMeshTrackerComponent {
    pub on_mesh_tracker_updated: FMockDataMeshTrackerComponent_OnMeshTrackerUpdated,
    pub scan_world: bool,
    pub request_normals: bool,
    pub request_vertex_confidence: bool,
    pub vertex_color_mode: EMeshTrackerVertexColorMode,
    pub block_vertex_colors: TArray<crate::bindings::core_u_object::FColor>,
    pub vertex_color_from_confidence_zero: crate::bindings::core_u_object::FLinearColor,
    pub vertex_color_from_confidence_one: crate::bindings::core_u_object::FLinearColor,
    pub update_interval: f32,
    pub mr_mesh: UPtr<UMRMeshComponent>,
}
pub struct UMRMeshBodyHolder {
    pub body_setup: UPtr<crate::bindings::engine::UBodySetup>,
    pub body_instance: crate::bindings::engine::FBodyInstance,
}
pub struct UMRMeshComponent {
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub wireframe_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub b_create_mesh_proxy_sections: bool,
    pub b_update_nav_mesh_on_mesh_update: bool,
    pub b_never_create_collision_mesh: bool,
    pub body_holders: TArray<UPtr<UMRMeshBodyHolder>>,
}
pub struct UMeshReconstructorBase {}
pub struct FMockDataMeshTrackerComponent_OnMeshTrackerUpdated;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshTrackerVertexColorMode(pub u8);
impl EMeshTrackerVertexColorMode {
    pub const NONE: EMeshTrackerVertexColorMode = EMeshTrackerVertexColorMode(0);
    pub const CONFIDENCE: EMeshTrackerVertexColorMode = EMeshTrackerVertexColorMode(1);
    pub const BLOCK: EMeshTrackerVertexColorMode = EMeshTrackerVertexColorMode(2);
}
