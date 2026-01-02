#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UOpenVDBImportOptionsObject {
    __padding_end: [u8; 216],
}
impl UOpenVDBImportOptionsObject {}
#[repr(C, align(8))]
pub struct USparseVolumeTextureFactory {
    __padding_end: [u8; 168],
}
impl USparseVolumeTextureFactory {}
