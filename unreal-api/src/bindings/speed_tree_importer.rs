#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct USpeedTreeImportFactory {}
pub struct UReimportSpeedTreeFactory {}
pub struct USpeedTreeImportData {
    pub tree_scale: f32,
    pub import_geometry_type: EImportGeometryType,
    pub lod_type: EImportLODType,
    pub flags_104: u8,
    pub flags_105: u8,
}
