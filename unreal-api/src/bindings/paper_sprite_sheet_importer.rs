#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UPaperSpriteSheet {
    pub sprite_names: TArray<FString>,
    pub sprites: TArray<TSoftObjectPtr<crate::bindings::paper2_d::UPaperSprite>>,
    pub texture_name: FString,
    pub texture: UPtr<crate::bindings::engine::UTexture2D>,
    pub normal_map_texture_name: FString,
    pub normal_map_texture: UPtr<crate::bindings::engine::UTexture2D>,
    pub asset_import_data: UPtr<crate::bindings::engine::UAssetImportData>,
}
pub struct UPaperSpriteSheetImportFactory {}
pub struct UPaperSpriteSheetReimportFactory {}
