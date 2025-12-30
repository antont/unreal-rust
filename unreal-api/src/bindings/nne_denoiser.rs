#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FTilingConfig {
    pub alignment: i32,
    pub overlap: i32,
    pub max_size: i32,
    pub min_size: i32,
}
#[repr(C, align(8))]
pub struct FNNEDenoiserBaseMappingData {
    pub tensor_index: i32,
    pub tensor_channel: i32,
    pub resource_channel: i32,
}
#[repr(C, align(8))]
pub struct FNNEDenoiserInputMappingData {
    pub resource: EInputResourceName,
}
#[repr(C, align(8))]
pub struct FNNEDenoiserOutputMappingData {
    pub resource: EOutputResourceName,
}
#[repr(C, align(8))]
pub struct FNNEDenoiserTemporalInputMappingData {
    pub resource: ETemporalInputResourceName,
    pub frame_index: i32,
}
#[repr(C, align(8))]
pub struct FNNEDenoiserTemporalOutputMappingData {
    pub resource: ETemporalOutputResourceName,
}
pub struct UNNEDenoiserAsset {
    pub model_data: TSoftObjectPtr<UNNEModelData>,
    pub input_mapping: TSoftObjectPtr<UDataTable>,
    pub output_mapping: TSoftObjectPtr<UDataTable>,
    pub tiling_config: FTilingConfig,
}
pub struct UNNEDenoiserTemporalAsset {
    pub model_data: TSoftObjectPtr<UNNEModelData>,
    pub input_mapping: TSoftObjectPtr<UDataTable>,
    pub output_mapping: TSoftObjectPtr<UDataTable>,
    pub tiling_config: FTilingConfig,
}
pub struct UNNEDenoiserSettings {
    pub denoiser_asset: TSoftObjectPtr<UNNEDenoiserAsset>,
    pub maximum_tile_size_override: i32,
    pub runtime_type: EDenoiserRuntimeType,
    pub runtime_name: FString,
}
