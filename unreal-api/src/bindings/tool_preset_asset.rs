#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FInteractiveToolPresetDefinition {
    pub stored_properties: FString,
    pub label: FString,
    pub tooltip: FString,
}
#[repr(C, align(16))]
pub struct FInteractiveToolPresetStore {
    pub named_presets: TArray<FInteractiveToolPresetDefinition>,
    pub tool_label: FText,
    pub tool_icon: crate::bindings::slate_core::FSlateBrush,
}
pub struct UAssetDefinition_InteractiveToolsPresetCollectionAsset {}
pub struct UInteractiveToolsPresetCollectionAsset {
    pub per_tool_presets: TMap<FString, FInteractiveToolPresetStore>,
    pub collection_label: FText,
}
pub struct UInteractiveToolsPresetCollectionAssetFactory {}
pub struct UToolPresetAssetSubsystem {
    pub default_collection: UPtr<UInteractiveToolsPresetCollectionAsset>,
}
