#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FOverlayItem {
    pub start_time: FTimespan,
    pub end_time: FTimespan,
    pub text: FString,
    pub position: FVector2D,
}
pub struct UOverlays {}
pub struct UBasicOverlays {
    pub overlays: TArray<FOverlayItem>,
    pub asset_import_data: UPtr<UAssetImportData>,
}
pub struct ULocalizedOverlays {
    pub default_overlays: UPtr<UBasicOverlays>,
    pub locale_to_overlays_map: TMap<FString, UPtr<UBasicOverlays>>,
    pub asset_import_data: UPtr<UAssetImportData>,
}
