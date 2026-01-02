#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UPaintBrushSettings {
    __padding_end: [u8; 72],
}
impl UPaintBrushSettings {}
#[repr(C, align(8))]
pub struct UMeshPaintSettings {
    __padding_end: [u8; 56],
}
impl UMeshPaintSettings {}
#[repr(C, align(8))]
pub struct UVertexColorImportOptions {
    __padding_end: [u8; 152],
}
impl UVertexColorImportOptions {}
#[repr(transparent)]
pub struct EMeshPaintColorViewMode(pub u8);
impl EMeshPaintColorViewMode {
    pub const NORMAL: EMeshPaintColorViewMode = EMeshPaintColorViewMode(0);
    pub const RGB: EMeshPaintColorViewMode = EMeshPaintColorViewMode(1);
    pub const ALPHA: EMeshPaintColorViewMode = EMeshPaintColorViewMode(2);
    pub const RED: EMeshPaintColorViewMode = EMeshPaintColorViewMode(3);
    pub const GREEN: EMeshPaintColorViewMode = EMeshPaintColorViewMode(4);
    pub const BLUE: EMeshPaintColorViewMode = EMeshPaintColorViewMode(5);
}
