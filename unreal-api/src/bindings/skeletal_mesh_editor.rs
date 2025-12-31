#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct USkeletalMeshEditorContextMenuContext {}
pub struct USkeletalMeshEditorUISubsystem {}
pub struct USkeletalMeshEditorSubsystem {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESkelSubSysQueryCurvesMetatdataNamesFilter(pub u8);
impl ESkelSubSysQueryCurvesMetatdataNamesFilter {
    pub const ALL: ESkelSubSysQueryCurvesMetatdataNamesFilter = ESkelSubSysQueryCurvesMetatdataNamesFilter(
        0,
    );
    pub const MORPH_TARGET_ONLY: ESkelSubSysQueryCurvesMetatdataNamesFilter = ESkelSubSysQueryCurvesMetatdataNamesFilter(
        1,
    );
    pub const MATERIAL_ONLY: ESkelSubSysQueryCurvesMetatdataNamesFilter = ESkelSubSysQueryCurvesMetatdataNamesFilter(
        2,
    );
}
