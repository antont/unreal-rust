#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FDataLayerCreationParameters {
    pub data_layer_asset: UPtr<crate::bindings::engine::UDataLayerAsset>,
    pub world_data_layers: UPtr<crate::bindings::engine::AWorldDataLayers>,
    pub b_is_private: bool,
}
pub struct UDataLayerEditorState {
    pub not_loaded_data_layers: TArray<UPtr<crate::bindings::engine::UDataLayerAsset>>,
    pub loaded_data_layers: TArray<UPtr<crate::bindings::engine::UDataLayerAsset>>,
}
pub struct UActorEditorContextDataLayerState {
    pub external_data_layer_asset: UPtr<
        crate::bindings::engine::UExternalDataLayerAsset,
    >,
    pub data_layer_assets: TArray<
        TSoftObjectPtr<crate::bindings::engine::UDataLayerAsset>,
    >,
}
pub struct UDataLayerEditorSubsystem {}
pub struct UDataLayerFactory {}
pub struct UExternalDataLayerFactory {}
