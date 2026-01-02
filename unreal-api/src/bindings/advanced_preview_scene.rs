#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UDefaultEditorProfiles {
    __padding_end: [u8; 64],
}
impl UDefaultEditorProfiles {}
#[repr(C, align(8))]
pub struct ULocalProfiles {
    __padding_end: [u8; 64],
}
impl ULocalProfiles {}
#[repr(C, align(8))]
pub struct USharedProfiles {
    __padding_end: [u8; 64],
}
impl USharedProfiles {}
#[repr(C, align(8))]
pub struct UAssetViewerSettings {
    __padding_end: [u8; 184],
}
impl UAssetViewerSettings {}
