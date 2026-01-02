#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAIGraph {
    __padding_end: [u8; 200],
}
impl UAIGraph {}
#[repr(C, align(8))]
pub struct UAIGraphNode {
    __padding_end: [u8; 344],
}
impl UAIGraphNode {}
#[repr(C, align(8))]
pub struct UAIGraphSchema {
    __padding_end: [u8; 48],
}
impl UAIGraphSchema {}
#[repr(C, align(8))]
pub struct UK2Node_AIMoveTo {
    __padding_end: [u8; 240],
}
impl UK2Node_AIMoveTo {}
