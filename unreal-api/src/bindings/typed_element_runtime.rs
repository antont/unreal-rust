#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FTypedElementSelectionOptions {
    pub b_allow_hidden: bool,
    pub b_allow_groups: bool,
    pub b_allow_legacy_notifications: bool,
    pub b_warn_if_locked: bool,
    pub b_allow_sub_root_selection: bool,
    pub child_element_inclusion_method: ETypedElementChildInclusionMethod,
}
#[repr(C, align(8))]
pub struct FTypedElementSelectionSetState {
    pub created_from_selection_set: TWeakObjectPtr<UTypedElementSelectionSet>,
}
#[repr(C, align(4))]
pub struct FTypedElementIsSelectedOptions {
    pub b_allow_indirect: bool,
}
#[repr(C, align(4))]
pub struct FTypedElementSelectionNormalizationOptions {
    pub b_expand_groups: bool,
    pub b_follow_attachment: bool,
}
#[repr(C, align(1))]
pub struct FTypedElementAssetDataReferencedOptions {
    pub b_only_top_level_asset: bool,
}
pub struct UTypedElementSelectionSetLibrary {}
pub struct UTypedElementSelectionSet {
    pub on_pre_selection_change: FTypedElementSelectionSet_OnPreSelectionChange,
    pub on_selection_change: FTypedElementSelectionSet_OnSelectionChange,
}
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
