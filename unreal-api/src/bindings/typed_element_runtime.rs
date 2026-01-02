#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FTypedElementSelectionOptions {
    pub b_allow_hidden: bool,
    pub b_allow_groups: bool,
    pub b_allow_legacy_notifications: bool,
    pub b_warn_if_locked: bool,
    pub b_allow_sub_root_selection: bool,
    pub child_element_inclusion_method: ETypedElementChildInclusionMethod,
    __padding_end: [u8; 14],
}
impl FTypedElementSelectionOptions {}
#[repr(C, align(8))]
pub struct FTypedElementSelectionSetState {
    __padding_end: [u8; 24],
}
impl FTypedElementSelectionSetState {}
#[repr(C, align(4))]
pub struct FTypedElementIsSelectedOptions {
    pub b_allow_indirect: bool,
    __padding_end: [u8; 15],
}
impl FTypedElementIsSelectedOptions {}
#[repr(C, align(4))]
pub struct FTypedElementSelectionNormalizationOptions {
    pub b_expand_groups: bool,
    pub b_follow_attachment: bool,
    __padding_end: [u8; 14],
}
impl FTypedElementSelectionNormalizationOptions {}
#[repr(C, align(1))]
pub struct FTypedElementAssetDataReferencedOptions {
    pub b_only_top_level_asset: bool,
}
impl FTypedElementAssetDataReferencedOptions {}
#[repr(C, align(8))]
pub struct UTypedElementSelectionSetLibrary {
    __padding_end: [u8; 48],
}
impl UTypedElementSelectionSetLibrary {}
#[repr(C, align(8))]
pub struct UTypedElementSelectionSet {
    __padding_end: [u8; 2256],
}
impl UTypedElementSelectionSet {}
pub struct UTypedElementAssetDataInterface {}
pub struct ITypedElementAssetDataInterface {}
pub struct UTypedElementHierarchyInterface {}
pub struct ITypedElementHierarchyInterface {}
pub struct UTypedElementObjectInterface {}
pub struct ITypedElementObjectInterface {}
pub struct UTypedElementPrimitiveCustomDataInterface {}
pub struct ITypedElementPrimitiveCustomDataInterface {}
pub struct UTypedElementSelectionInterface {}
pub struct ITypedElementSelectionInterface {}
#[repr(transparent)]
pub struct FTypedElementSelectionSet_OnPreSelectionChange {
    _opague: u8,
}
#[repr(transparent)]
pub struct FTypedElementSelectionSet_OnSelectionChange {
    _opague: u8,
}
#[repr(transparent)]
pub struct ETypedElementChildInclusionMethod(pub u8);
impl ETypedElementChildInclusionMethod {
    pub const NONE: ETypedElementChildInclusionMethod = ETypedElementChildInclusionMethod(
        0,
    );
    pub const IMMEDIATE: ETypedElementChildInclusionMethod = ETypedElementChildInclusionMethod(
        1,
    );
    pub const RECURSIVE: ETypedElementChildInclusionMethod = ETypedElementChildInclusionMethod(
        2,
    );
}
#[repr(transparent)]
pub struct ETypedElementSelectionMethod(pub u8);
impl ETypedElementSelectionMethod {
    pub const PRIMARY: ETypedElementSelectionMethod = ETypedElementSelectionMethod(0);
    pub const SECONDARY: ETypedElementSelectionMethod = ETypedElementSelectionMethod(1);
    pub const FROM_SECONDARY: ETypedElementSelectionMethod = ETypedElementSelectionMethod(
        2,
    );
}
