#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct USpeedTreeImportFactory {
    __padding_end: [u8; 232],
}
impl USpeedTreeImportFactory {}
#[repr(C, align(8))]
pub struct UReimportSpeedTreeFactory {
    __padding_end: [u8; 272],
}
impl UReimportSpeedTreeFactory {}
#[repr(C, align(8))]
pub struct USpeedTreeImportData {
    __padding_end: [u8; 112],
}
impl USpeedTreeImportData {}
#[repr(transparent)]
pub struct EImportGeometryType(pub u8);
impl EImportGeometryType {
    pub const IGT_3D: EImportGeometryType = EImportGeometryType(0);
    pub const IGT_BILLBOARDS: EImportGeometryType = EImportGeometryType(1);
    pub const IGT_BOTH: EImportGeometryType = EImportGeometryType(2);
}
#[repr(transparent)]
pub struct EImportLODType(pub u8);
impl EImportLODType {
    pub const ILT_PAINTED_FOLIAGE: EImportLODType = EImportLODType(0);
    pub const ILT_INDIVIDUAL_ACTORS: EImportLODType = EImportLODType(1);
}
