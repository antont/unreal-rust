#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FInteractiveToolPresetDefinition {
    __padding_end: [u8; 48],
}
impl FInteractiveToolPresetDefinition {}
#[repr(C, align(16))]
pub struct FInteractiveToolPresetStore {
    __padding_end: [u8; 240],
}
impl FInteractiveToolPresetStore {}
#[repr(C, align(8))]
pub struct UAssetDefinition_InteractiveToolsPresetCollectionAsset {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_InteractiveToolsPresetCollectionAsset {}
#[repr(C, align(8))]
pub struct UInteractiveToolsPresetCollectionAsset {
    __padding_end: [u8; 144],
}
impl UInteractiveToolsPresetCollectionAsset {}
#[repr(C, align(8))]
pub struct UInteractiveToolsPresetCollectionAssetFactory {
    __padding_end: [u8; 136],
}
impl UInteractiveToolsPresetCollectionAssetFactory {}
#[repr(C, align(8))]
pub struct UToolPresetAssetSubsystem {
    __padding_end: [u8; 64],
}
impl UToolPresetAssetSubsystem {}
