#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FDataflowGroomSolverProxy {
    __padding_end: [u8; 344],
}
impl FDataflowGroomSolverProxy {}
#[repr(C, align(16))]
pub struct UGroomSolverComponent {
    __padding_end: [u8; 2304],
}
impl UGroomSolverComponent {}
