#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UCookerStats {
    __padding_end: [u8; 88],
}
impl UCookerStats {}
#[repr(C, align(8))]
pub struct ULightingBuildInfo {
    __padding_end: [u8; 88],
}
impl ULightingBuildInfo {}
#[repr(C, align(8))]
pub struct UPrimitiveStats {
    __padding_end: [u8; 176],
}
impl UPrimitiveStats {}
#[repr(C, align(8))]
pub struct UShaderCookerStats {
    __padding_end: [u8; 136],
}
impl UShaderCookerStats {}
#[repr(C, align(8))]
pub struct UStaticMeshLightingInfo {
    __padding_end: [u8; 152],
}
impl UStaticMeshLightingInfo {}
#[repr(C, align(8))]
pub struct UTextureStats {
    __padding_end: [u8; 184],
}
impl UTextureStats {}
