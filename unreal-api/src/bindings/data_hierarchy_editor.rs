#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UDataHierarchyEditorMenuContext {
    __padding_end: [u8; 64],
}
impl UDataHierarchyEditorMenuContext {}
#[repr(C, align(8))]
pub struct UHierarchyDataRefreshContext {
    __padding_end: [u8; 48],
}
impl UHierarchyDataRefreshContext {}
#[repr(C, align(8))]
pub struct UHierarchyElement {
    __padding_end: [u8; 192],
}
impl UHierarchyElement {}
#[repr(C, align(8))]
pub struct UHierarchySection {
    __padding_end: [u8; 224],
}
impl UHierarchySection {}
#[repr(C, align(8))]
pub struct UHierarchyRoot {
    __padding_end: [u8; 208],
}
impl UHierarchyRoot {}
#[repr(C, align(8))]
pub struct UHierarchyItem {
    __padding_end: [u8; 192],
}
impl UHierarchyItem {}
#[repr(C, align(8))]
pub struct UHierarchyCategory {
    __padding_end: [u8; 232],
}
impl UHierarchyCategory {}
#[repr(C, align(8))]
pub struct UHierarchyMenuContext {
    __padding_end: [u8; 72],
}
impl UHierarchyMenuContext {}
#[repr(C, align(8))]
pub struct UDataHierarchyViewModelBase {
    __padding_end: [u8; 488],
}
impl UDataHierarchyViewModelBase {}
