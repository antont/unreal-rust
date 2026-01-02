#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UNewPluginDescriptorData {
    __padding_end: [u8; 104],
}
impl UNewPluginDescriptorData {}
#[repr(C, align(8))]
pub struct UPluginMetadataObject {
    __padding_end: [u8; 304],
}
impl UPluginMetadataObject {}
