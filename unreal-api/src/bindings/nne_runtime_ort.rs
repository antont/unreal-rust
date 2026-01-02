#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UNNERuntimeORTCpu {
    __padding_end: [u8; 80],
}
impl UNNERuntimeORTCpu {}
#[repr(C, align(8))]
pub struct UNNERuntimeORTDmlProxy {
    __padding_end: [u8; 64],
}
impl UNNERuntimeORTDmlProxy {}
#[repr(C, align(8))]
pub struct UNNERuntimeORTDml_GPU_RDG_NPU {
    __padding_end: [u8; 88],
}
impl UNNERuntimeORTDml_GPU_RDG_NPU {}
#[repr(C, align(8))]
pub struct UNNERuntimeORTDml_GPU_RDG {
    __padding_end: [u8; 80],
}
impl UNNERuntimeORTDml_GPU_RDG {}
#[repr(C, align(8))]
pub struct UNNERuntimeORTDml_GPU_NPU {
    __padding_end: [u8; 80],
}
impl UNNERuntimeORTDml_GPU_NPU {}
#[repr(C, align(8))]
pub struct UNNERuntimeORTDml_RDG_NPU {
    __padding_end: [u8; 80],
}
impl UNNERuntimeORTDml_RDG_NPU {}
#[repr(C, align(8))]
pub struct UNNERuntimeORTDml_GPU {
    __padding_end: [u8; 72],
}
impl UNNERuntimeORTDml_GPU {}
#[repr(C, align(8))]
pub struct UNNERuntimeORTDml_RDG {
    __padding_end: [u8; 72],
}
impl UNNERuntimeORTDml_RDG {}
#[repr(C, align(8))]
pub struct UNNERuntimeORTDml_NPU {
    __padding_end: [u8; 72],
}
impl UNNERuntimeORTDml_NPU {}
#[repr(C, align(8))]
pub struct UNNERuntimeORTSettings {
    __padding_end: [u8; 136],
}
impl UNNERuntimeORTSettings {}
#[repr(transparent)]
pub struct EExecutionMode(pub u8);
impl EExecutionMode {
    pub const SEQUENTIAL: EExecutionMode = EExecutionMode(0);
    pub const PARALLEL: EExecutionMode = EExecutionMode(1);
}
