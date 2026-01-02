#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UClothPainterSettings {
    __padding_end: [u8; 136],
}
impl UClothPainterSettings {}
#[repr(C, align(8))]
pub struct UClothingAssetExporter {
    __padding_end: [u8; 48],
}
impl UClothingAssetExporter {}
#[repr(C, align(8))]
pub struct UClothPaintTool_BrushSettings {
    __padding_end: [u8; 56],
}
impl UClothPaintTool_BrushSettings {}
#[repr(C, align(8))]
pub struct UClothPaintTool_GradientSettings {
    __padding_end: [u8; 64],
}
impl UClothPaintTool_GradientSettings {}
#[repr(C, align(8))]
pub struct UClothPaintTool_SmoothSettings {
    __padding_end: [u8; 56],
}
impl UClothPaintTool_SmoothSettings {}
#[repr(C, align(8))]
pub struct UClothPaintTool_FillSettings {
    __padding_end: [u8; 56],
}
impl UClothPaintTool_FillSettings {}
#[repr(transparent)]
pub struct ESourceColorChannel(pub u8);
impl ESourceColorChannel {
    pub const RED: ESourceColorChannel = ESourceColorChannel(0);
    pub const GREEN: ESourceColorChannel = ESourceColorChannel(1);
    pub const BLUE: ESourceColorChannel = ESourceColorChannel(2);
    pub const ALPHA: ESourceColorChannel = ESourceColorChannel(3);
}
