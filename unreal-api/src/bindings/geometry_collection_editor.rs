#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UActorFactoryGeometryCollection {
    __padding_end: [u8; 144],
}
impl UActorFactoryGeometryCollection {}
#[repr(C, align(8))]
pub struct UAssetDefinition_GeometryCollection {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_GeometryCollection {}
#[repr(C, align(8))]
pub struct UGeometryCollectionCacheFactory {
    __padding_end: [u8; 160],
}
impl UGeometryCollectionCacheFactory {}
#[repr(C, align(8))]
pub struct UGeometryCollectionFactory {
    __padding_end: [u8; 136],
}
impl UGeometryCollectionFactory {}
#[repr(C, align(8))]
pub struct UGeometryCollectionThumbnailRenderer {
    __padding_end: [u8; 64],
}
impl UGeometryCollectionThumbnailRenderer {}
