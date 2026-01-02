#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UOptimusDeformerFactory {
    __padding_end: [u8; 136],
}
impl UOptimusDeformerFactory {}
#[repr(C, align(8))]
pub struct UOptimusEditorGraph {
    __padding_end: [u8; 456],
}
impl UOptimusEditorGraph {}
#[repr(C, align(8))]
pub struct UOptimusEditorGraphNode {
    __padding_end: [u8; 464],
}
impl UOptimusEditorGraphNode {}
#[repr(C, align(8))]
pub struct UOptimusEditorGraphNode_Comment {
    __padding_end: [u8; 344],
}
impl UOptimusEditorGraphNode_Comment {}
#[repr(C, align(8))]
pub struct UOptimusEditorGraphSchema {
    __padding_end: [u8; 48],
}
impl UOptimusEditorGraphSchema {}
#[repr(C, align(8))]
pub struct UOptimusSourceFactory {
    __padding_end: [u8; 136],
}
impl UOptimusSourceFactory {}
