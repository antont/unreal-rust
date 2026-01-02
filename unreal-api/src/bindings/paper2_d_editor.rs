#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UPaperSpriteAtlasFactory {
    __padding_end: [u8; 136],
}
impl UPaperSpriteAtlasFactory {}
#[repr(C, align(8))]
pub struct UPaperExtractSpritesSettings {
    __padding_end: [u8; 128],
}
impl UPaperExtractSpritesSettings {}
#[repr(C, align(8))]
pub struct UPaperExtractSpriteGridSettings {
    __padding_end: [u8; 80],
}
impl UPaperExtractSpriteGridSettings {}
#[repr(C, align(8))]
pub struct UFlipbookEditorSettings {
    __padding_end: [u8; 56],
}
impl UFlipbookEditorSettings {}
#[repr(C, align(8))]
pub struct UPaperFlipbookActorFactory {
    __padding_end: [u8; 144],
}
impl UPaperFlipbookActorFactory {}
#[repr(C, align(8))]
pub struct UPaperFlipbookFactory {
    __padding_end: [u8; 152],
}
impl UPaperFlipbookFactory {}
#[repr(C, align(8))]
pub struct UPaperSpriteThumbnailRenderer {
    __padding_end: [u8; 56],
}
impl UPaperSpriteThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UPaperFlipbookThumbnailRenderer {
    __padding_end: [u8; 56],
}
impl UPaperFlipbookThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UPaperImporterSettings {
    __padding_end: [u8; 336],
}
impl UPaperImporterSettings {}
#[repr(C, align(8))]
pub struct UPaperSpriteActorFactory {
    __padding_end: [u8; 144],
}
impl UPaperSpriteActorFactory {}
#[repr(C, align(8))]
pub struct UPaperSpriteFactory {
    __padding_end: [u8; 168],
}
impl UPaperSpriteFactory {}
#[repr(C, align(8))]
pub struct UPaperTileMapFactory {
    __padding_end: [u8; 144],
}
impl UPaperTileMapFactory {}
#[repr(C, align(8))]
pub struct UPaperTileMapPromotionFactory {
    __padding_end: [u8; 144],
}
impl UPaperTileMapPromotionFactory {}
#[repr(C, align(8))]
pub struct UPaperTileSetFactory {
    __padding_end: [u8; 144],
}
impl UPaperTileSetFactory {}
#[repr(C, align(8))]
pub struct UPaperTileSetThumbnailRenderer {
    __padding_end: [u8; 56],
}
impl UPaperTileSetThumbnailRenderer {}
#[repr(C, align(8))]
pub struct USpriteEditorSettings {
    __padding_end: [u8; 56],
}
impl USpriteEditorSettings {}
#[repr(C, align(8))]
pub struct UTerrainSplineActorFactory {
    __padding_end: [u8; 144],
}
impl UTerrainSplineActorFactory {}
#[repr(C, align(8))]
pub struct UTileMapActorFactory {
    __padding_end: [u8; 144],
}
impl UTileMapActorFactory {}
#[repr(C, align(8))]
pub struct UTileMapAssetImportData {
    __padding_end: [u8; 112],
}
impl UTileMapAssetImportData {}
#[repr(C, align(8))]
pub struct UTileMapEditorSettings {
    __padding_end: [u8; 88],
}
impl UTileMapEditorSettings {}
#[repr(C, align(8))]
pub struct UTileSetEditorSettings {
    __padding_end: [u8; 64],
}
impl UTileSetEditorSettings {}
#[repr(C, align(8))]
pub struct UTileSheetPaddingFactory {
    __padding_end: [u8; 152],
}
impl UTileSheetPaddingFactory {}
#[repr(transparent)]
pub struct ESpriteExtractMode(pub u8);
impl ESpriteExtractMode {
    pub const AUTO: ESpriteExtractMode = ESpriteExtractMode(0);
    pub const GRID: ESpriteExtractMode = ESpriteExtractMode(1);
}
