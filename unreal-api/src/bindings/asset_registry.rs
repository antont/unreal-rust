#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FTagAndValue {
    pub tag: FName,
    pub value: FString,
}
#[repr(C, align(1))]
pub struct FAssetRegistryDependencyOptions {
    pub b_include_soft_package_references: bool,
    pub b_include_hard_package_references: bool,
    pub b_include_game_package_references: bool,
    pub b_include_editor_only_package_references: bool,
    pub b_include_searchable_names: bool,
    pub b_include_soft_management_references: bool,
    pub b_include_hard_management_references: bool,
}
pub struct UAssetRegistryHelpers {}
pub struct UAssetRegistry {}
pub struct IAssetRegistry {}
pub struct UAssetRegistryImpl {}
pub struct FSortByPredicate_SortingPredicate;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAssetRegistrySortOrder(pub u8);
impl EAssetRegistrySortOrder {
    pub const ASCENDING: EAssetRegistrySortOrder = EAssetRegistrySortOrder(0);
    pub const DESCENDING: EAssetRegistrySortOrder = EAssetRegistrySortOrder(1);
}
