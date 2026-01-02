#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FMRMeshConfiguration {
    __padding_end: [u8; 1],
}
impl FMRMeshConfiguration {}
#[repr(C, align(16))]
pub struct UMockDataMeshTrackerComponent {
    #[doc(hidden)]
    __padding_680: [u8; 680],
    pub scan_world: bool,
    pub request_normals: bool,
    pub request_vertex_confidence: bool,
    pub vertex_color_mode: EMeshTrackerVertexColorMode,
    pub block_vertex_colors: TArray<crate::bindings::core_u_object::FColor>,
    pub vertex_color_from_confidence_zero: crate::bindings::core_u_object::FLinearColor,
    pub vertex_color_from_confidence_one: crate::bindings::core_u_object::FLinearColor,
    pub update_interval: f32,
    __padding_end: [u8; 44],
}
impl UMockDataMeshTrackerComponent {}
#[repr(C, align(8))]
pub struct UMRMeshBodyHolder {
    __padding_end: [u8; 600],
}
impl UMRMeshBodyHolder {}
#[repr(C, align(16))]
pub struct UMRMeshComponent {
    __padding_end: [u8; 1648],
}
impl UMRMeshComponent {}
#[repr(C, align(8))]
pub struct UMeshReconstructorBase {
    __padding_end: [u8; 48],
}
impl UMeshReconstructorBase {}
#[repr(transparent)]
pub struct FMockDataMeshTrackerComponent_OnMeshTrackerUpdated {
    _opague: u8,
}
#[repr(transparent)]
pub struct EMeshTrackerVertexColorMode(pub u8);
impl EMeshTrackerVertexColorMode {
    pub const NONE: EMeshTrackerVertexColorMode = EMeshTrackerVertexColorMode(0);
    pub const CONFIDENCE: EMeshTrackerVertexColorMode = EMeshTrackerVertexColorMode(1);
    pub const BLOCK: EMeshTrackerVertexColorMode = EMeshTrackerVertexColorMode(2);
}
