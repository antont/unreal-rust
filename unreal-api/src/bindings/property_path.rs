#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FPropertyPathTestInnerStruct {
    __padding_end: [u8; 40],
}
impl FPropertyPathTestInnerStruct {}
#[repr(C, align(8))]
pub struct FPropertyPathTestStruct {
    __padding_end: [u8; 96],
}
impl FPropertyPathTestStruct {}
#[repr(C, align(8))]
pub struct UPropertyPathTestObject {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub bool: bool,
    #[doc(hidden)]
    __padding_56: [u8; 4],
    pub integer: i32,
    pub string: FString,
    pub float: f32,
    pub _struct: FPropertyPathTestStruct,
    pub struct_ref: FPropertyPathTestStruct,
    pub struct_const_ref: FPropertyPathTestStruct,
    pub inner_object: UPtr<UPropertyPathTestObject>,
    __padding_end: [u8; 8],
}
impl UPropertyPathTestObject {}
#[repr(transparent)]
pub struct EPropertyPathTestEnum(pub u8);
impl EPropertyPathTestEnum {
    pub const ONE: EPropertyPathTestEnum = EPropertyPathTestEnum(0);
    pub const TWO: EPropertyPathTestEnum = EPropertyPathTestEnum(1);
    pub const THREE: EPropertyPathTestEnum = EPropertyPathTestEnum(2);
    pub const FOUR: EPropertyPathTestEnum = EPropertyPathTestEnum(3);
}
