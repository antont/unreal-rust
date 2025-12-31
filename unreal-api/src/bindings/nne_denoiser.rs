#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
    pub model_data: TSoftObjectPtr<crate::bindings::nne::UNNEModelData>,
    pub input_mapping: TSoftObjectPtr<crate::bindings::engine::UDataTable>,
    pub output_mapping: TSoftObjectPtr<crate::bindings::engine::UDataTable>,
    pub tiling_config: FTilingConfig,
}
pub struct UNNEDenoiserTemporalAsset {
    pub model_data: TSoftObjectPtr<crate::bindings::nne::UNNEModelData>,
    pub input_mapping: TSoftObjectPtr<crate::bindings::engine::UDataTable>,
    pub output_mapping: TSoftObjectPtr<crate::bindings::engine::UDataTable>,
    pub tiling_config: FTilingConfig,
}
pub struct UNNEDenoiserSettings {
    pub denoiser_asset: TSoftObjectPtr<UNNEDenoiserAsset>,
    pub maximum_tile_size_override: i32,
    pub runtime_type: EDenoiserRuntimeType,
    pub runtime_name: FString,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInputResourceName(pub u8);
impl EInputResourceName {
    pub const COLOR: EInputResourceName = EInputResourceName(0);
    pub const ALBEDO: EInputResourceName = EInputResourceName(1);
    pub const NORMAL: EInputResourceName = EInputResourceName(2);
    pub const OUTPUT: EInputResourceName = EInputResourceName(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EOutputResourceName(pub u8);
impl EOutputResourceName {
    pub const OUTPUT: EOutputResourceName = EOutputResourceName(0);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETemporalInputResourceName(pub u8);
impl ETemporalInputResourceName {
    pub const COLOR: ETemporalInputResourceName = ETemporalInputResourceName(0);
    pub const ALBEDO: ETemporalInputResourceName = ETemporalInputResourceName(1);
    pub const NORMAL: ETemporalInputResourceName = ETemporalInputResourceName(2);
    pub const FLOW: ETemporalInputResourceName = ETemporalInputResourceName(3);
    pub const OUTPUT: ETemporalInputResourceName = ETemporalInputResourceName(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETemporalOutputResourceName(pub u8);
impl ETemporalOutputResourceName {
    pub const OUTPUT: ETemporalOutputResourceName = ETemporalOutputResourceName(0);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDenoiserRuntimeType(pub u8);
impl EDenoiserRuntimeType {
    pub const CPU: EDenoiserRuntimeType = EDenoiserRuntimeType(0);
    pub const GPU: EDenoiserRuntimeType = EDenoiserRuntimeType(1);
    pub const RDG: EDenoiserRuntimeType = EDenoiserRuntimeType(2);
}
