#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FOverlayItem {
    pub start_time: crate::bindings::core_u_object::FTimespan,
    pub end_time: crate::bindings::core_u_object::FTimespan,
    pub text: FString,
    pub position: crate::bindings::core_u_object::FVector2D,
}
pub struct UOverlays {}
pub struct UBasicOverlays {
    pub overlays: TArray<FOverlayItem>,
    pub asset_import_data: UPtr<crate::bindings::engine::UAssetImportData>,
}
pub struct ULocalizedOverlays {
    pub default_overlays: UPtr<UBasicOverlays>,
    pub locale_to_overlays_map: TMap<FString, UPtr<UBasicOverlays>>,
    pub asset_import_data: UPtr<crate::bindings::engine::UAssetImportData>,
}
