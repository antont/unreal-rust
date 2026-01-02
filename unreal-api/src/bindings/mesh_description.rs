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
impl FElementID {}
#[repr(C, align(4))]
pub struct FVertexID {
    __padding_end: [u8; 4],
}
impl FVertexID {}
#[repr(C, align(4))]
pub struct FVertexInstanceID {
    __padding_end: [u8; 4],
}
impl FVertexInstanceID {}
#[repr(C, align(4))]
pub struct FEdgeID {
    __padding_end: [u8; 4],
}
impl FEdgeID {}
#[repr(C, align(4))]
pub struct FUVID {
    __padding_end: [u8; 4],
}
impl FUVID {}
#[repr(C, align(4))]
pub struct FTriangleID {
    __padding_end: [u8; 4],
}
impl FTriangleID {}
#[repr(C, align(4))]
pub struct FPolygonGroupID {
    __padding_end: [u8; 4],
}
impl FPolygonGroupID {}
#[repr(C, align(4))]
pub struct FPolygonID {
    __padding_end: [u8; 4],
}
impl FPolygonID {}
#[repr(C, align(8))]
pub struct UMeshDescriptionBase {
    __padding_end: [u8; 760],
}
impl UMeshDescriptionBase {}
#[repr(C, align(8))]
pub struct UMeshDescriptionBaseBulkData {
    __padding_end: [u8; 72],
}
impl UMeshDescriptionBaseBulkData {}
