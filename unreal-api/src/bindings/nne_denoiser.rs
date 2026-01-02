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
impl FTilingConfig {}
#[repr(C, align(8))]
pub struct FNNEDenoiserBaseMappingData {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub tensor_index: i32,
    pub tensor_channel: i32,
    pub resource_channel: i32,
    __padding_end: [u8; 4],
}
impl FNNEDenoiserBaseMappingData {}
#[repr(C, align(8))]
pub struct FNNEDenoiserInputMappingData {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub resource: EInputResourceName,
    __padding_end: [u8; 7],
}
impl FNNEDenoiserInputMappingData {}
#[repr(C, align(8))]
pub struct FNNEDenoiserOutputMappingData {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub resource: EOutputResourceName,
    __padding_end: [u8; 7],
}
impl FNNEDenoiserOutputMappingData {}
#[repr(C, align(8))]
pub struct FNNEDenoiserTemporalInputMappingData {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub resource: ETemporalInputResourceName,
    pub frame_index: i32,
}
impl FNNEDenoiserTemporalInputMappingData {}
#[repr(C, align(8))]
pub struct FNNEDenoiserTemporalOutputMappingData {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub resource: ETemporalOutputResourceName,
    __padding_end: [u8; 7],
}
impl FNNEDenoiserTemporalOutputMappingData {}
#[repr(C, align(8))]
pub struct UNNEDenoiserAsset {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub model_data: TSoftObjectPtr<crate::bindings::nne::UNNEModelData>,
    pub input_mapping: TSoftObjectPtr<crate::bindings::engine::UDataTable>,
    pub output_mapping: TSoftObjectPtr<crate::bindings::engine::UDataTable>,
    pub tiling_config: FTilingConfig,
}
impl UNNEDenoiserAsset {}
#[repr(C, align(8))]
pub struct UNNEDenoiserTemporalAsset {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub model_data: TSoftObjectPtr<crate::bindings::nne::UNNEModelData>,
    pub input_mapping: TSoftObjectPtr<crate::bindings::engine::UDataTable>,
    pub output_mapping: TSoftObjectPtr<crate::bindings::engine::UDataTable>,
    pub tiling_config: FTilingConfig,
}
impl UNNEDenoiserTemporalAsset {}
#[repr(C, align(8))]
pub struct UNNEDenoiserSettings {
    __padding_end: [u8; 224],
}
impl UNNEDenoiserSettings {}
#[repr(transparent)]
pub struct EInputResourceName(pub u8);
impl EInputResourceName {
    pub const COLOR: EInputResourceName = EInputResourceName(0);
    pub const ALBEDO: EInputResourceName = EInputResourceName(1);
    pub const NORMAL: EInputResourceName = EInputResourceName(2);
    pub const OUTPUT: EInputResourceName = EInputResourceName(3);
}
#[repr(transparent)]
pub struct EOutputResourceName(pub u8);
impl EOutputResourceName {
    pub const OUTPUT: EOutputResourceName = EOutputResourceName(0);
}
#[repr(transparent)]
pub struct ETemporalInputResourceName(pub u8);
impl ETemporalInputResourceName {
    pub const COLOR: ETemporalInputResourceName = ETemporalInputResourceName(0);
    pub const ALBEDO: ETemporalInputResourceName = ETemporalInputResourceName(1);
    pub const NORMAL: ETemporalInputResourceName = ETemporalInputResourceName(2);
    pub const FLOW: ETemporalInputResourceName = ETemporalInputResourceName(3);
    pub const OUTPUT: ETemporalInputResourceName = ETemporalInputResourceName(4);
}
#[repr(transparent)]
pub struct ETemporalOutputResourceName(pub u8);
impl ETemporalOutputResourceName {
    pub const OUTPUT: ETemporalOutputResourceName = ETemporalOutputResourceName(0);
}
#[repr(transparent)]
pub struct EDenoiserRuntimeType(pub u8);
impl EDenoiserRuntimeType {
    pub const CPU: EDenoiserRuntimeType = EDenoiserRuntimeType(0);
    pub const GPU: EDenoiserRuntimeType = EDenoiserRuntimeType(1);
    pub const RDG: EDenoiserRuntimeType = EDenoiserRuntimeType(2);
}
