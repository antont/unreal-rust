#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FElementID {
    pub id_value: i32,
}
#[repr(C, align(4))]
pub struct FVertexID {}
#[repr(C, align(4))]
pub struct FVertexInstanceID {}
#[repr(C, align(4))]
pub struct FEdgeID {}
#[repr(C, align(4))]
pub struct FUVID {}
#[repr(C, align(4))]
pub struct FTriangleID {}
#[repr(C, align(4))]
pub struct FPolygonGroupID {}
#[repr(C, align(4))]
pub struct FPolygonID {}
pub struct UMeshDescriptionBase {}
pub struct UMeshDescriptionBaseBulkData {
    pub preallocated_mesh_description: UPtr<UMeshDescriptionBase>,
    pub mesh_description: UPtr<UMeshDescriptionBase>,
}
