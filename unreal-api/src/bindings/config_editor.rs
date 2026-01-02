#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UPropertyConfigFileDisplayRow {
    __padding_end: [u8; 144],
}
impl UPropertyConfigFileDisplayRow {}
#[repr(C, align(8))]
pub struct UConfigHierarchyPropertyView {
    __padding_end: [u8; 112],
}
impl UConfigHierarchyPropertyView {}
