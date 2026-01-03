#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UMeshPaintingToolProperties {
    __padding_end: [u8; 256],
}
impl UMeshPaintingToolProperties {}
#[repr(C, align(8))]
pub struct UMeshPaintingSubsystem {
    __padding_end: [u8; 256],
}
impl UMeshPaintingSubsystem {}
pub struct IMeshPaintSelectionInterface {}
#[repr(C, align(8))]
pub struct UMeshPaintSelectionInterface {
    __padding_end: [u8; 48],
}
impl UMeshPaintSelectionInterface {}
#[repr(C, align(8))]
pub struct UMeshPaintSelectionMechanic {
    __padding_end: [u8; 96],
}
impl UMeshPaintSelectionMechanic {}
#[repr(C, align(8))]
pub struct UVertexAdapterClickToolBuilder {
    __padding_end: [u8; 48],
}
impl UVertexAdapterClickToolBuilder {}
#[repr(C, align(8))]
pub struct UTextureColorAdapterClickToolBuilder {
    __padding_end: [u8; 48],
}
impl UTextureColorAdapterClickToolBuilder {}
#[repr(C, align(8))]
pub struct UTextureAssetAdapterClickToolBuilder {
    __padding_end: [u8; 48],
}
impl UTextureAssetAdapterClickToolBuilder {}
#[repr(C, align(8))]
pub struct UMeshClickTool {
    __padding_end: [u8; 224],
}
impl UMeshClickTool {}
#[repr(C, align(8))]
pub struct UVertexAdapterClickTool {
    __padding_end: [u8; 224],
}
impl UVertexAdapterClickTool {}
#[repr(C, align(8))]
pub struct UTextureColorAdapterClickTool {
    __padding_end: [u8; 224],
}
impl UTextureColorAdapterClickTool {}
#[repr(C, align(8))]
pub struct UTextureAssetAdapterClickTool {
    __padding_end: [u8; 224],
}
impl UTextureAssetAdapterClickTool {}
#[repr(C, align(8))]
pub struct UMeshTextureColorPaintingToolBuilder {
    __padding_end: [u8; 56],
}
impl UMeshTextureColorPaintingToolBuilder {}
#[repr(C, align(8))]
pub struct UMeshTextureAssetPaintingToolBuilder {
    __padding_end: [u8; 56],
}
impl UMeshTextureAssetPaintingToolBuilder {}
#[repr(C, align(8))]
pub struct UMeshTexturePaintingToolProperties {
    __padding_end: [u8; 288],
}
impl UMeshTexturePaintingToolProperties {}
#[repr(C, align(8))]
pub struct UMeshTextureColorPaintingToolProperties {
    __padding_end: [u8; 296],
}
impl UMeshTextureColorPaintingToolProperties {}
#[repr(C, align(8))]
pub struct UMeshTextureAssetPaintingToolProperties {
    __padding_end: [u8; 304],
}
impl UMeshTextureAssetPaintingToolProperties {}
#[repr(C, align(8))]
pub struct UMeshTexturePaintingTool {
    __padding_end: [u8; 1320],
}
impl UMeshTexturePaintingTool {}
#[repr(C, align(8))]
pub struct UMeshTextureColorPaintingTool {
    __padding_end: [u8; 1336],
}
impl UMeshTextureColorPaintingTool {}
#[repr(C, align(8))]
pub struct UMeshTextureAssetPaintingTool {
    __padding_end: [u8; 1328],
}
impl UMeshTextureAssetPaintingTool {}
#[repr(C, align(8))]
pub struct UMeshVertexColorPaintingToolBuilder {
    __padding_end: [u8; 48],
}
impl UMeshVertexColorPaintingToolBuilder {}
#[repr(C, align(8))]
pub struct UMeshVertexWeightPaintingToolBuilder {
    __padding_end: [u8; 48],
}
impl UMeshVertexWeightPaintingToolBuilder {}
#[repr(C, align(8))]
pub struct UMeshVertexPaintingToolProperties {
    __padding_end: [u8; 272],
}
impl UMeshVertexPaintingToolProperties {}
#[repr(C, align(8))]
pub struct UMeshVertexColorPaintingToolProperties {
    __padding_end: [u8; 280],
}
impl UMeshVertexColorPaintingToolProperties {}
#[repr(C, align(8))]
pub struct UMeshVertexWeightPaintingToolProperties {
    __padding_end: [u8; 280],
}
impl UMeshVertexWeightPaintingToolProperties {}
#[repr(C, align(8))]
pub struct UMeshVertexPaintingTool {
    __padding_end: [u8; 1200],
}
impl UMeshVertexPaintingTool {}
#[repr(C, align(8))]
pub struct UMeshVertexColorPaintingTool {
    __padding_end: [u8; 1208],
}
impl UMeshVertexColorPaintingTool {}
#[repr(C, align(8))]
pub struct UMeshVertexWeightPaintingTool {
    __padding_end: [u8; 1208],
}
impl UMeshVertexWeightPaintingTool {}
#[repr(C, align(8))]
pub struct UTexturePaintToolset {
    __padding_end: [u8; 48],
}
impl UTexturePaintToolset {}
#[repr(transparent)]
pub struct EMeshPaintWeightTypes(pub u8);
impl EMeshPaintWeightTypes {
    pub const ALPHA_LERP: EMeshPaintWeightTypes = EMeshPaintWeightTypes(2);
    pub const RGB: EMeshPaintWeightTypes = EMeshPaintWeightTypes(3);
    pub const ARGB: EMeshPaintWeightTypes = EMeshPaintWeightTypes(4);
    pub const ONE_MINUS_ARGB: EMeshPaintWeightTypes = EMeshPaintWeightTypes(5);
}
#[repr(transparent)]
pub struct EMeshPaintTextureIndex(pub u8);
impl EMeshPaintTextureIndex {
    pub const TEXTURE_ONE: EMeshPaintTextureIndex = EMeshPaintTextureIndex(0);
    pub const TEXTURE_TWO: EMeshPaintTextureIndex = EMeshPaintTextureIndex(1);
    pub const TEXTURE_THREE: EMeshPaintTextureIndex = EMeshPaintTextureIndex(2);
    pub const TEXTURE_FOUR: EMeshPaintTextureIndex = EMeshPaintTextureIndex(3);
    pub const TEXTURE_FIVE: EMeshPaintTextureIndex = EMeshPaintTextureIndex(4);
}
#[repr(transparent)]
pub struct EMeshPaintDataColorViewMode(pub u8);
impl EMeshPaintDataColorViewMode {
    pub const NORMAL: EMeshPaintDataColorViewMode = EMeshPaintDataColorViewMode(0);
    pub const RGB: EMeshPaintDataColorViewMode = EMeshPaintDataColorViewMode(1);
    pub const ALPHA: EMeshPaintDataColorViewMode = EMeshPaintDataColorViewMode(2);
    pub const RED: EMeshPaintDataColorViewMode = EMeshPaintDataColorViewMode(3);
    pub const GREEN: EMeshPaintDataColorViewMode = EMeshPaintDataColorViewMode(4);
    pub const BLUE: EMeshPaintDataColorViewMode = EMeshPaintDataColorViewMode(5);
}
