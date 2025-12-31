#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct USpeedTreeImportFactory {}
pub struct UReimportSpeedTreeFactory {}
pub struct USpeedTreeImportData {
    pub tree_scale: f32,
    pub import_geometry_type: EImportGeometryType,
    pub lod_type: EImportLODType,
    pub flags_104: u8,
    pub flags_105: u8,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EImportGeometryType(pub u8);
impl EImportGeometryType {
    pub const IGT_3D: EImportGeometryType = EImportGeometryType(0);
    pub const IGT_BILLBOARDS: EImportGeometryType = EImportGeometryType(1);
    pub const IGT_BOTH: EImportGeometryType = EImportGeometryType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EImportLODType(pub u8);
impl EImportLODType {
    pub const ILT_PAINTED_FOLIAGE: EImportLODType = EImportLODType(0);
    pub const ILT_INDIVIDUAL_ACTORS: EImportLODType = EImportLODType(1);
}
