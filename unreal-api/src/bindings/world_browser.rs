#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UEditorLevelFolders {
    __padding_end: [u8; 128],
}
impl UEditorLevelFolders {}
#[repr(C, align(8))]
pub struct UWorldTileDetails {
    __padding_end: [u8; 1816],
}
impl UWorldTileDetails {}
#[repr(C, align(8))]
pub struct UWorldBrowserConfig {
    __padding_end: [u8; 128],
}
impl UWorldBrowserConfig {}
