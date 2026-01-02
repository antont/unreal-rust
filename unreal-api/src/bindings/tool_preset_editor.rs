#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UToolPresetUserSettings {
    __padding_end: [u8; 136],
}
impl UToolPresetUserSettings {}
#[repr(C, align(8))]
pub struct UToolPresetProjectSettings {
    __padding_end: [u8; 184],
}
impl UToolPresetProjectSettings {}
