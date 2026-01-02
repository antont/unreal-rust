#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UBagTestObject1 {
    __padding_end: [u8; 48],
}
impl UBagTestObject1 {}
#[repr(C, align(8))]
pub struct UBagTestObject2 {
    __padding_end: [u8; 48],
}
impl UBagTestObject2 {}
#[repr(C, align(8))]
pub struct UBagTestObject1Derived {
    __padding_end: [u8; 48],
}
impl UBagTestObject1Derived {}
#[repr(C, align(8))]
pub struct UTestObjectWithPropertyBag {
    __padding_end: [u8; 64],
}
impl UTestObjectWithPropertyBag {}
#[repr(C, align(8))]
pub struct UTestObjectWithInstanceStruct {
    __padding_end: [u8; 64],
}
impl UTestObjectWithInstanceStruct {}
