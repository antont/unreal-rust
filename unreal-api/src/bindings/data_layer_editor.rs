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
    __padding_end: [u8; 7],
}
impl FDataLayerCreationParameters {}
#[repr(C, align(8))]
pub struct UDataLayerEditorState {
    __padding_end: [u8; 80],
}
impl UDataLayerEditorState {}
#[repr(C, align(8))]
pub struct UActorEditorContextDataLayerState {
    __padding_end: [u8; 72],
}
impl UActorEditorContextDataLayerState {}
#[repr(C, align(8))]
pub struct UDataLayerEditorSubsystem {
    __padding_end: [u8; 344],
}
impl UDataLayerEditorSubsystem {}
#[repr(C, align(8))]
pub struct UDataLayerFactory {
    __padding_end: [u8; 136],
}
impl UDataLayerFactory {}
#[repr(C, align(8))]
pub struct UExternalDataLayerFactory {
    __padding_end: [u8; 136],
}
impl UExternalDataLayerFactory {}
