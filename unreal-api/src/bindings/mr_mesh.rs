#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(1))]
pub struct FMRMeshConfiguration {}
pub struct UMockDataMeshTrackerComponent {
    pub on_mesh_tracker_updated: FMockDataMeshTrackerComponent_OnMeshTrackerUpdated,
    pub scan_world: bool,
    pub request_normals: bool,
    pub request_vertex_confidence: bool,
    pub vertex_color_mode: EMeshTrackerVertexColorMode,
    pub block_vertex_colors: TArray<FColor>,
    pub vertex_color_from_confidence_zero: FLinearColor,
    pub vertex_color_from_confidence_one: FLinearColor,
    pub update_interval: f32,
    pub mr_mesh: UPtr<UMRMeshComponent>,
}
pub struct UMRMeshBodyHolder {
    pub body_setup: UPtr<UBodySetup>,
    pub body_instance: FBodyInstance,
}
pub struct UMRMeshComponent {
    pub material: UPtr<UMaterialInterface>,
    pub wireframe_material: UPtr<UMaterialInterface>,
    pub b_create_mesh_proxy_sections: bool,
    pub b_update_nav_mesh_on_mesh_update: bool,
    pub b_never_create_collision_mesh: bool,
    pub body_holders: TArray<UPtr<UMRMeshBodyHolder>>,
}
pub struct UMeshReconstructorBase {}
