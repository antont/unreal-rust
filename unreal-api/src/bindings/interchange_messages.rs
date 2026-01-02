#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UInterchangeResultMeshWarning {
    __padding_end: [u8; 136],
}
impl UInterchangeResultMeshWarning {}
#[repr(C, align(8))]
pub struct UInterchangeResultTextureDisplay {
    __padding_end: [u8; 152],
}
impl UInterchangeResultTextureDisplay {}
#[repr(C, align(8))]
pub struct UInterchangeResultTextureWarning {
    __padding_end: [u8; 136],
}
impl UInterchangeResultTextureWarning {}
#[repr(C, align(8))]
pub struct UInterchangeResultMeshError {
    __padding_end: [u8; 136],
}
impl UInterchangeResultMeshError {}
#[repr(C, align(8))]
pub struct UInterchangeResultMeshWarning_Generic {
    __padding_end: [u8; 152],
}
impl UInterchangeResultMeshWarning_Generic {}
#[repr(C, align(8))]
pub struct UInterchangeResultMeshError_Generic {
    __padding_end: [u8; 152],
}
impl UInterchangeResultMeshError_Generic {}
#[repr(C, align(8))]
pub struct UInterchangeResultMeshWarning_TooManyUVs {
    __padding_end: [u8; 144],
}
impl UInterchangeResultMeshWarning_TooManyUVs {}
#[repr(C, align(8))]
pub struct UInterchangeResultTextureDisplay_TextureFileDoNotExist {
    __padding_end: [u8; 168],
}
impl UInterchangeResultTextureDisplay_TextureFileDoNotExist {}
