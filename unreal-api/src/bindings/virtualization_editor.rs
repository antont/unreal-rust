#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UCheckForVirtualizedContentCommandlet {
    __padding_end: [u8; 136],
}
impl UCheckForVirtualizedContentCommandlet {}
#[repr(C, align(8))]
pub struct UGenerateMountPointPayloadManifestCommandlet {
    __padding_end: [u8; 160],
}
impl UGenerateMountPointPayloadManifestCommandlet {}
#[repr(C, align(8))]
pub struct UGeneratePayloadManifestCommandlet {
    __padding_end: [u8; 144],
}
impl UGeneratePayloadManifestCommandlet {}
#[repr(C, align(8))]
pub struct UPrecachePayloadsCommandlet {
    __padding_end: [u8; 136],
}
impl UPrecachePayloadsCommandlet {}
#[repr(C, align(8))]
pub struct URehydrateProjectCommandlet {
    __padding_end: [u8; 144],
}
impl URehydrateProjectCommandlet {}
#[repr(C, align(8))]
pub struct UValidateVirtualizedContentCommandlet {
    __padding_end: [u8; 136],
}
impl UValidateVirtualizedContentCommandlet {}
#[repr(C, align(8))]
pub struct UVirtualizeProjectCommandlet {
    __padding_end: [u8; 144],
}
impl UVirtualizeProjectCommandlet {}
