#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UMegascansSettings {
    __padding_end: [u8; 56],
}
impl UMegascansSettings {}
#[repr(C, align(8))]
pub struct UMaterialBlendSettings {
    __padding_end: [u8; 80],
}
impl UMaterialBlendSettings {}
#[repr(C, align(8))]
pub struct UMaterialAssetSettings {
    __padding_end: [u8; 96],
}
impl UMaterialAssetSettings {}
#[repr(C, align(8))]
pub struct UMaterialPresetsSettings {
    __padding_end: [u8; 120],
}
impl UMaterialPresetsSettings {}
#[repr(C, align(8))]
pub struct UVersionInfoHandler {
    __padding_end: [u8; 64],
}
impl UVersionInfoHandler {}
