#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UStateTreeTest_PropertyObjectInstanced {
    __padding_end: [u8; 88],
}
impl UStateTreeTest_PropertyObjectInstanced {}
#[repr(C, align(8))]
pub struct UStateTreeTest_PropertyObjectInstancedWithB {
    __padding_end: [u8; 96],
}
impl UStateTreeTest_PropertyObjectInstancedWithB {}
#[repr(C, align(8))]
pub struct UStateTreeTest_PropertyObject {
    __padding_end: [u8; 152],
}
impl UStateTreeTest_PropertyObject {}
#[repr(C, align(8))]
pub struct UStateTreeTest_PropertyObject2 {
    __padding_end: [u8; 48],
}
impl UStateTreeTest_PropertyObject2 {}
#[repr(C, align(8))]
pub struct UStateTreeTestSchema {
    __padding_end: [u8; 56],
}
impl UStateTreeTestSchema {}
#[repr(C, align(8))]
pub struct UStateTreeTestSchema2 {
    __padding_end: [u8; 48],
}
impl UStateTreeTestSchema2 {}
