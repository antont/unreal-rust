#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UBlueprintEditorLibrary {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAssetSaveVersionComparisonResults(pub u8);
impl EAssetSaveVersionComparisonResults {
    pub const INVALID_COMPARISON: EAssetSaveVersionComparisonResults = EAssetSaveVersionComparisonResults(
        0,
    );
    pub const IDENTICAL: EAssetSaveVersionComparisonResults = EAssetSaveVersionComparisonResults(
        1,
    );
    pub const NEWER: EAssetSaveVersionComparisonResults = EAssetSaveVersionComparisonResults(
        2,
    );
    pub const OLDER: EAssetSaveVersionComparisonResults = EAssetSaveVersionComparisonResults(
        3,
    );
}
