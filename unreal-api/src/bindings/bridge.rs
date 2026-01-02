#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UNodePort {
    __padding_end: [u8; 72],
}
impl UNodePort {}
#[repr(C, align(8))]
pub struct UBrowserBinding {
    __padding_end: [u8; 440],
}
impl UBrowserBinding {}
