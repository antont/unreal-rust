#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FDataLayerCreationParameters {
    pub data_layer_asset: UPtr<UDataLayerAsset>,
    pub world_data_layers: UPtr<AWorldDataLayers>,
    pub b_is_private: bool,
}
pub struct UDataLayerEditorState {
    pub not_loaded_data_layers: TArray<UPtr<UDataLayerAsset>>,
    pub loaded_data_layers: TArray<UPtr<UDataLayerAsset>>,
}
pub struct UActorEditorContextDataLayerState {
    pub external_data_layer_asset: UPtr<UExternalDataLayerAsset>,
    pub data_layer_assets: TArray<TSoftObjectPtr<UDataLayerAsset>>,
}
pub struct UDataLayerEditorSubsystem {}
pub struct UDataLayerFactory {}
pub struct UExternalDataLayerFactory {}
