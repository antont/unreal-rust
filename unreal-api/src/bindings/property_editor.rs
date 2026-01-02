#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UDetailRowMenuContext {
    __padding_end: [u8; 112],
}
impl UDetailRowMenuContext {}
#[repr(C, align(8))]
pub struct UDetailRowMenuContextPrivate {
    __padding_end: [u8; 64],
}
impl UDetailRowMenuContextPrivate {}
#[repr(C, align(8))]
pub struct UDetailsViewPropertyHandleTestValueClass {
    __padding_end: [u8; 48],
}
impl UDetailsViewPropertyHandleTestValueClass {}
#[repr(C, align(8))]
pub struct UDetailsViewPropertyHandleTestClass {
    __padding_end: [u8; 112],
}
impl UDetailsViewPropertyHandleTestClass {}
#[repr(C, align(8))]
pub struct UDetailsConfig {
    __padding_end: [u8; 128],
}
impl UDetailsConfig {}
#[repr(C, align(8))]
pub struct UEditConditionTestObject {
    __padding_end: [u8; 144],
}
impl UEditConditionTestObject {}
#[repr(C, align(8))]
pub struct UPropertyEditorSinglePropertyTestClass {
    __padding_end: [u8; 72],
}
impl UPropertyEditorSinglePropertyTestClass {}
#[repr(transparent)]
pub struct EditConditionByteEnum(pub u8);
impl EditConditionByteEnum {
    pub const FIRST: EditConditionByteEnum = EditConditionByteEnum(15);
    pub const SECOND: EditConditionByteEnum = EditConditionByteEnum(31);
}
#[repr(transparent)]
pub struct EditConditionTestEnum(pub i32);
impl EditConditionTestEnum {
    pub const FIRST: EditConditionTestEnum = EditConditionTestEnum(15);
    pub const SECOND: EditConditionTestEnum = EditConditionTestEnum(31);
}
