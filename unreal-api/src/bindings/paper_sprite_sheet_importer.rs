#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UPaperSpriteSheet {
    pub sprite_names: TArray<FString>,
    pub sprites: TArray<TSoftObjectPtr<UPaperSprite>>,
    pub texture_name: FString,
    pub texture: UPtr<UTexture2D>,
    pub normal_map_texture_name: FString,
    pub normal_map_texture: UPtr<UTexture2D>,
    pub asset_import_data: UPtr<UAssetImportData>,
}
pub struct UPaperSpriteSheetImportFactory {}
pub struct UPaperSpriteSheetReimportFactory {}
