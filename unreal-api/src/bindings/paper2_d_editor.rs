#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FTileSetImportMapping {
    pub source_name: FString,
    pub imported_tile_set: TWeakObjectPtr<UPaperTileSet>,
    pub imported_texture: TWeakObjectPtr<UTexture>,
}
pub struct UPaperSpriteAtlasFactory {}
pub struct UPaperExtractSpritesSettings {
    pub sprite_extract_mode: ESpriteExtractMode,
    pub outline_color: FLinearColor,
    pub viewport_texture_tint: FLinearColor,
    pub background_color: FLinearColor,
    pub naming_template: FString,
    pub naming_start_index: i32,
}
pub struct UPaperExtractSpriteGridSettings {
    pub cell_width: i32,
    pub cell_height: i32,
    pub num_cells_x: i32,
    pub num_cells_y: i32,
    pub margin_x: i32,
    pub margin_y: i32,
    pub spacing_x: i32,
    pub spacing_y: i32,
}
pub struct UFlipbookEditorSettings {
    pub background_color: FColor,
    pub b_show_grid_by_default: bool,
}
pub struct UPaperFlipbookActorFactory {}
pub struct UPaperFlipbookFactory {}
pub struct UPaperSpriteThumbnailRenderer {}
pub struct UPaperFlipbookThumbnailRenderer {}
pub struct UPaperImporterSettings {
    pub b_pick_best_material_when_creating_sprites: bool,
    pub b_pick_best_material_when_creating_tile_maps: bool,
    pub b_analysis_can_use_opaque: bool,
    pub default_pixels_per_unreal_unit: f32,
    pub normal_map_texture_suffixes: TArray<FString>,
    pub base_map_texture_suffixes: TArray<FString>,
    pub default_sprite_texture_group: TextureGroup,
    pub b_override_texture_compression: bool,
    pub default_sprite_texture_compression: TextureCompressionSettings,
    pub unlit_default_masked_material_name: FSoftObjectPath,
    pub unlit_default_translucent_material_name: FSoftObjectPath,
    pub unlit_default_opaque_material_name: FSoftObjectPath,
    pub lit_default_masked_material_name: FSoftObjectPath,
    pub lit_default_translucent_material_name: FSoftObjectPath,
    pub lit_default_opaque_material_name: FSoftObjectPath,
}
pub struct UPaperSpriteActorFactory {}
pub struct UPaperSpriteFactory {}
pub struct UPaperTileMapFactory {}
pub struct UPaperTileMapPromotionFactory {
    pub asset_to_rename: UPtr<UPaperTileMap>,
}
pub struct UPaperTileSetFactory {}
pub struct UPaperTileSetThumbnailRenderer {}
pub struct USpriteEditorSettings {
    pub background_color: FColor,
    pub b_show_grid_by_default: bool,
}
pub struct UTerrainSplineActorFactory {}
pub struct UTileMapActorFactory {}
pub struct UTileMapAssetImportData {
    pub tile_set_map: TArray<FTileSetImportMapping>,
}
pub struct UTileMapEditorSettings {
    pub default_background_color: FColor,
    pub b_show_grid_by_default: bool,
    pub default_tile_grid_color: FColor,
    pub default_multi_tile_grid_color: FColor,
    pub default_multi_tile_grid_width: i32,
    pub default_multi_tile_grid_height: i32,
    pub default_multi_tile_grid_offset_x: i32,
    pub default_multi_tile_grid_offset_y: i32,
    pub default_layer_grid_color: FColor,
}
pub struct UTileSetEditorSettings {
    pub default_background_color: FColor,
    pub b_show_grid_by_default: bool,
    pub extrusion_amount: i32,
    pub b_pad_to_power_of2: bool,
    pub b_fill_with_transparent_black: bool,
}
pub struct UTileSheetPaddingFactory {
    pub source_tile_set: UPtr<UPaperTileSet>,
    pub extrusion_amount: i32,
    pub b_pad_to_power_of2: bool,
    pub b_fill_with_transparent_black: bool,
}
