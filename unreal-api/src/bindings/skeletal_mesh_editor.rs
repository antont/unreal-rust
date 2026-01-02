#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct USkeletalMeshEditorContextMenuContext {
    __padding_end: [u8; 56],
}
impl USkeletalMeshEditorContextMenuContext {}
#[repr(C, align(8))]
pub struct USkeletalMeshEditorUISubsystem {
    __padding_end: [u8; 56],
}
impl USkeletalMeshEditorUISubsystem {}
#[repr(C, align(8))]
pub struct USkeletalMeshEditorSubsystem {
    __padding_end: [u8; 56],
}
impl USkeletalMeshEditorSubsystem {}
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
