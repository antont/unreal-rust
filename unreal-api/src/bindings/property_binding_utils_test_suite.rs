#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UPropertyBindingUtilsTest_PropertyObjectInstanced {
    __padding_end: [u8; 72],
}
impl UPropertyBindingUtilsTest_PropertyObjectInstanced {}
#[repr(C, align(8))]
pub struct UPropertyBindingUtilsTest_PropertyObjectInstancedWithB {
    __padding_end: [u8; 80],
}
impl UPropertyBindingUtilsTest_PropertyObjectInstancedWithB {}
#[repr(C, align(8))]
pub struct UPropertyBindingUtilsTest_PropertyObject {
    __padding_end: [u8; 152],
}
impl UPropertyBindingUtilsTest_PropertyObject {}
