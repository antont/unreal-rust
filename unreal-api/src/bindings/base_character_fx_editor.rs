#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UBaseCharacterFXEditor {
    __padding_end: [u8; 80],
}
impl UBaseCharacterFXEditor {}
#[repr(C, align(8))]
pub struct UBaseCharacterFXEditorMode {
    __padding_end: [u8; 344],
}
impl UBaseCharacterFXEditorMode {}
#[repr(C, align(8))]
pub struct UBaseCharacterFXEditorUISubsystem {
    __padding_end: [u8; 56],
}
impl UBaseCharacterFXEditorUISubsystem {}
