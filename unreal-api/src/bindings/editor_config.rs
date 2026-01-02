#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UEditorConfigBase {
    __padding_end: [u8; 48],
}
impl UEditorConfigBase {}
#[repr(C, align(8))]
pub struct UEditorConfigSubsystem {
    __padding_end: [u8; 216],
}
impl UEditorConfigSubsystem {}
#[repr(C, align(8))]
pub struct UEditorMetadataOverrides {
    __padding_end: [u8; 152],
}
impl UEditorMetadataOverrides {}
#[repr(C, align(8))]
pub struct UEditorConfigTestObject {
    __padding_end: [u8; 112],
}
impl UEditorConfigTestObject {}
#[repr(transparent)]
pub struct EEditorConfigTestEnum(pub u8);
impl EEditorConfigTestEnum {
    pub const ZERO: EEditorConfigTestEnum = EEditorConfigTestEnum(0);
    pub const ONE: EEditorConfigTestEnum = EEditorConfigTestEnum(1);
    pub const TWO: EEditorConfigTestEnum = EEditorConfigTestEnum(2);
}
