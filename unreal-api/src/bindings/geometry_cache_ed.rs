#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UActorFactoryGeometryCache {
    __padding_end: [u8; 144],
}
impl UActorFactoryGeometryCache {}
#[repr(C, align(8))]
pub struct UAssetDefinition_GeometryCache {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_GeometryCache {}
#[repr(C, align(8))]
pub struct UGeometryCacheThumbnailRenderer {
    __padding_end: [u8; 64],
}
impl UGeometryCacheThumbnailRenderer {}
