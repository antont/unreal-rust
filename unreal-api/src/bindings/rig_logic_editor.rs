#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UDNAAssetImportUI {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    __padding_end: [u8; 32],
}
impl UDNAAssetImportUI {}
#[repr(C, align(8))]
pub struct UDNAAssetImportFactory {
    __padding_end: [u8; 184],
}
impl UDNAAssetImportFactory {}
#[repr(C, align(8))]
pub struct UDNAImporterLibrary {
    __padding_end: [u8; 48],
}
impl UDNAImporterLibrary {}
