#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UTestSettings {
    __padding_end: [u8; 56],
}
impl UTestSettings {}
#[repr(C, align(8))]
pub struct UTedsSettingsEditorSubsystem {
    __padding_end: [u8; 104],
}
impl UTedsSettingsEditorSubsystem {}
#[repr(C, align(8))]
pub struct UTedsSettingsFactory {
    __padding_end: [u8; 48],
}
impl UTedsSettingsFactory {}
