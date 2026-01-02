#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct ULevelInstanceActorFactory {
    __padding_end: [u8; 144],
}
impl ULevelInstanceActorFactory {}
#[repr(C, align(8))]
pub struct ULevelInstanceEditorBehaviorSource {
    __padding_end: [u8; 72],
}
impl ULevelInstanceEditorBehaviorSource {}
#[repr(C, align(8))]
pub struct ULevelInstanceEditorMode {
    __padding_end: [u8; 336],
}
impl ULevelInstanceEditorMode {}
#[repr(C, align(8))]
pub struct ULevelInstanceEditorSettings {
    __padding_end: [u8; 144],
}
impl ULevelInstanceEditorSettings {}
#[repr(C, align(8))]
pub struct ULevelInstanceEditorPerProjectUserSettings {
    __padding_end: [u8; 112],
}
impl ULevelInstanceEditorPerProjectUserSettings {}
