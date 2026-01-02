#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UCQTestSettings {
    __padding_end: [u8; 120],
}
impl UCQTestSettings {}
#[repr(C, align(8))]
pub struct UTestGameInstance {
    __padding_end: [u8; 528],
}
impl UTestGameInstance {}
