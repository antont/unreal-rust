#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UTranslationPickerSettings {
    __padding_end: [u8; 56],
}
impl UTranslationPickerSettings {}
#[repr(C, align(8))]
pub struct UTranslationUnit {
    __padding_end: [u8; 216],
}
impl UTranslationUnit {}
