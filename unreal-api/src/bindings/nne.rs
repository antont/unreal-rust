#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct INNERuntime {}
#[repr(C, align(8))]
pub struct UNNERuntime {
    __padding_end: [u8; 48],
}
impl UNNERuntime {}
pub struct INNERuntimeCPU {}
#[repr(C, align(8))]
pub struct UNNERuntimeCPU {
    __padding_end: [u8; 48],
}
impl UNNERuntimeCPU {}
pub struct INNERuntimeGPU {}
#[repr(C, align(8))]
pub struct UNNERuntimeGPU {
    __padding_end: [u8; 48],
}
impl UNNERuntimeGPU {}
pub struct INNERuntimeNPU {}
#[repr(C, align(8))]
pub struct UNNERuntimeNPU {
    __padding_end: [u8; 48],
}
impl UNNERuntimeNPU {}
pub struct INNERuntimeRDG {}
#[repr(C, align(8))]
pub struct UNNERuntimeRDG {
    __padding_end: [u8; 48],
}
impl UNNERuntimeRDG {}
#[repr(C, align(8))]
pub struct UNNEModelData {
    __padding_end: [u8; 280],
}
impl UNNEModelData {}
