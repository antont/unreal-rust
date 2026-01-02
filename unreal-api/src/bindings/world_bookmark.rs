#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAssetDefinition_WorldBookmark {
    __padding_end: [u8; 80],
}
impl UAssetDefinition_WorldBookmark {}
#[repr(C, align(8))]
pub struct UWorldBookmarkBrowserSettings {
    __padding_end: [u8; 112],
}
impl UWorldBookmarkBrowserSettings {}
#[repr(C, align(8))]
pub struct UWorldBookmark {
    __padding_end: [u8; 208],
}
impl UWorldBookmark {}
#[repr(C, align(8))]
pub struct UWorldBookmarkEditorSettings {
    __padding_end: [u8; 120],
}
impl UWorldBookmarkEditorSettings {}
#[repr(C, align(8))]
pub struct UWorldBookmarkEditorPerProjectUserSettings {
    __padding_end: [u8; 160],
}
impl UWorldBookmarkEditorPerProjectUserSettings {}
#[repr(C, align(8))]
pub struct UWorldBookmarkFactory {
    __padding_end: [u8; 136],
}
impl UWorldBookmarkFactory {}
#[repr(transparent)]
pub struct EWorldBookmarkBrowserViewMode(pub u8);
impl EWorldBookmarkBrowserViewMode {
    pub const LIST_VIEW: EWorldBookmarkBrowserViewMode = EWorldBookmarkBrowserViewMode(
        0,
    );
    pub const TREE_VIEW: EWorldBookmarkBrowserViewMode = EWorldBookmarkBrowserViewMode(
        1,
    );
}
