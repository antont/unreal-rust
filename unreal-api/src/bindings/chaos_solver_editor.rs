#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UActorFactoryChaosSolver {
    __padding_end: [u8; 144],
}
impl UActorFactoryChaosSolver {}
#[repr(C, align(8))]
pub struct UChaosSolverFactory {
    __padding_end: [u8; 136],
}
impl UChaosSolverFactory {}
