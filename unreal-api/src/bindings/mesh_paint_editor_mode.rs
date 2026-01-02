#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UImportVertexColorOptions {
    __padding_end: [u8; 152],
}
impl UImportVertexColorOptions {}
#[repr(C, align(8))]
pub struct UMeshPaintMode {
    __padding_end: [u8; 360],
}
impl UMeshPaintMode {}
#[repr(C, align(8))]
pub struct UMeshPaintModeSettings {
    __padding_end: [u8; 64],
}
impl UMeshPaintModeSettings {}
