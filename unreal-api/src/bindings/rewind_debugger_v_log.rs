#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct URewindDebuggerVLogSettings {
    __padding_end: [u8; 192],
}
impl URewindDebuggerVLogSettings {}
#[repr(C, align(8))]
pub struct AVLogRenderingActor {
    __padding_end: [u8; 1336],
}
impl AVLogRenderingActor {}
