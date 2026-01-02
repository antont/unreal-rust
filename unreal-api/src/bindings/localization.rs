#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct ULocalizationSettings {
    __padding_end: [u8; 96],
}
impl ULocalizationSettings {}
#[repr(C, align(8))]
pub struct ULocalizationTargetSet {
    __padding_end: [u8; 64],
}
impl ULocalizationTargetSet {}
#[repr(C, align(8))]
pub struct ULocalizationTarget {
    __padding_end: [u8; 512],
}
impl ULocalizationTarget {}
#[repr(C, align(8))]
pub struct UUserGeneratedContentLocalizationSettings {
    __padding_end: [u8; 72],
}
impl UUserGeneratedContentLocalizationSettings {}
#[repr(transparent)]
pub struct ELocalizationGatherPathRoot(pub u8);
impl ELocalizationGatherPathRoot {
    pub const AUTO: ELocalizationGatherPathRoot = ELocalizationGatherPathRoot(0);
    pub const ENGINE: ELocalizationGatherPathRoot = ELocalizationGatherPathRoot(1);
    pub const PROJECT: ELocalizationGatherPathRoot = ELocalizationGatherPathRoot(2);
}
#[repr(transparent)]
pub struct ELocalizedTextCollapseMode(pub u8);
impl ELocalizedTextCollapseMode {
    pub const IDENTICAL_TEXT_ID_AND_SOURCE: ELocalizedTextCollapseMode = ELocalizedTextCollapseMode(
        0,
    );
    pub const IDENTICAL_PACKAGE_ID_TEXT_ID_AND_SOURCE: ELocalizedTextCollapseMode = ELocalizedTextCollapseMode(
        1,
    );
    pub const IDENTICAL_NAMESPACE_AND_SOURCE: ELocalizedTextCollapseMode = ELocalizedTextCollapseMode(
        2,
    );
}
#[repr(transparent)]
pub struct EPortableObjectFormat(pub u8);
impl EPortableObjectFormat {
    pub const UNREAL: EPortableObjectFormat = EPortableObjectFormat(0);
    pub const CROWDIN: EPortableObjectFormat = EPortableObjectFormat(1);
}
#[repr(transparent)]
pub struct ELocalizationTargetConflictStatus(pub u8);
impl ELocalizationTargetConflictStatus {
    pub const UNKNOWN: ELocalizationTargetConflictStatus = ELocalizationTargetConflictStatus(
        0,
    );
    pub const CONFLICTS_PRESENT: ELocalizationTargetConflictStatus = ELocalizationTargetConflictStatus(
        1,
    );
    pub const CLEAR: ELocalizationTargetConflictStatus = ELocalizationTargetConflictStatus(
        2,
    );
}
