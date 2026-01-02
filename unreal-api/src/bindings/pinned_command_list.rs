#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UPinnedCommandListSettings {
    __padding_end: [u8; 64],
}
impl UPinnedCommandListSettings {}
#[repr(transparent)]
pub struct EPinnedCommandListType(pub u8);
impl EPinnedCommandListType {
    pub const COMMAND: EPinnedCommandListType = EPinnedCommandListType(0);
    pub const CUSTOM_WIDGET: EPinnedCommandListType = EPinnedCommandListType(1);
}
