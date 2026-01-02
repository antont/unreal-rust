#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAssetDefinition_InterchangeSceneImportAsset {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_InterchangeSceneImportAsset {}
#[repr(C, align(8))]
pub struct UInterchangeEditorScriptLibrary {
    __padding_end: [u8; 48],
}
impl UInterchangeEditorScriptLibrary {}
#[repr(C, align(8))]
pub struct UInterchangeFbxAssetImportDataConverter {
    __padding_end: [u8; 48],
}
impl UInterchangeFbxAssetImportDataConverter {}
