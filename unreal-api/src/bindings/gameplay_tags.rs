#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FGameplayTag {
    __padding_end: [u8; 12],
}
impl FGameplayTag {}
#[repr(C, align(8))]
pub struct FGameplayTagContainer {
    pub gameplay_tags: TArray<FGameplayTag>,
    __padding_end: [u8; 16],
}
impl FGameplayTagContainer {}
#[repr(C, align(8))]
pub struct FGameplayTagQuery {
    __padding_end: [u8; 72],
}
impl FGameplayTagQuery {}
#[repr(C, align(8))]
pub struct FGameplayTagTableRow {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub tag: FName,
    pub dev_comment: FString,
}
impl FGameplayTagTableRow {}
#[repr(C, align(8))]
pub struct FRestrictedGameplayTagTableRow {
    #[doc(hidden)]
    __padding_40: [u8; 40],
    pub b_allow_non_restricted_children: bool,
    __padding_end: [u8; 7],
}
impl FRestrictedGameplayTagTableRow {}
#[repr(C, align(8))]
pub struct UBlueprintGameplayTagLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintGameplayTagLibrary {}
pub struct UGameplayTagAssetInterface {}
pub struct IGameplayTagAssetInterface {}
#[repr(C, align(8))]
pub struct UEditableGameplayTagQuery {
    __padding_end: [u8; 160],
}
impl UEditableGameplayTagQuery {}
#[repr(C, align(8))]
pub struct UEditableGameplayTagQueryExpression {
    __padding_end: [u8; 48],
}
impl UEditableGameplayTagQueryExpression {}
#[repr(C, align(8))]
pub struct UEditableGameplayTagQueryExpression_AnyTagsMatch {
    __padding_end: [u8; 80],
}
impl UEditableGameplayTagQueryExpression_AnyTagsMatch {}
#[repr(C, align(8))]
pub struct UEditableGameplayTagQueryExpression_AllTagsMatch {
    __padding_end: [u8; 80],
}
impl UEditableGameplayTagQueryExpression_AllTagsMatch {}
#[repr(C, align(8))]
pub struct UEditableGameplayTagQueryExpression_NoTagsMatch {
    __padding_end: [u8; 80],
}
impl UEditableGameplayTagQueryExpression_NoTagsMatch {}
#[repr(C, align(8))]
pub struct UEditableGameplayTagQueryExpression_AnyTagsExactMatch {
    __padding_end: [u8; 80],
}
impl UEditableGameplayTagQueryExpression_AnyTagsExactMatch {}
#[repr(C, align(8))]
pub struct UEditableGameplayTagQueryExpression_AllTagsExactMatch {
    __padding_end: [u8; 80],
}
impl UEditableGameplayTagQueryExpression_AllTagsExactMatch {}
#[repr(C, align(8))]
pub struct UEditableGameplayTagQueryExpression_AnyExprMatch {
    __padding_end: [u8; 64],
}
impl UEditableGameplayTagQueryExpression_AnyExprMatch {}
#[repr(C, align(8))]
pub struct UEditableGameplayTagQueryExpression_AllExprMatch {
    __padding_end: [u8; 64],
}
impl UEditableGameplayTagQueryExpression_AllExprMatch {}
#[repr(C, align(8))]
pub struct UEditableGameplayTagQueryExpression_NoExprMatch {
    __padding_end: [u8; 64],
}
impl UEditableGameplayTagQueryExpression_NoExprMatch {}
#[repr(C, align(8))]
pub struct UGameplayTagsManager {
    __padding_end: [u8; 1248],
}
impl UGameplayTagsManager {}
#[repr(C, align(8))]
pub struct UGameplayTagsList {
    __padding_end: [u8; 96],
}
impl UGameplayTagsList {}
#[repr(C, align(8))]
pub struct URestrictedGameplayTagsList {
    __padding_end: [u8; 80],
}
impl URestrictedGameplayTagsList {}
#[repr(C, align(8))]
pub struct UGameplayTagsSettings {
    __padding_end: [u8; 256],
}
impl UGameplayTagsSettings {}
#[repr(C, align(8))]
pub struct UGameplayTagsDeveloperSettings {
    __padding_end: [u8; 136],
}
impl UGameplayTagsDeveloperSettings {}
#[repr(transparent)]
pub struct EGameplayTagSourceType(pub u8);
impl EGameplayTagSourceType {
    pub const NATIVE: EGameplayTagSourceType = EGameplayTagSourceType(0);
    pub const DEFAULT_TAG_LIST: EGameplayTagSourceType = EGameplayTagSourceType(1);
    pub const TAG_LIST: EGameplayTagSourceType = EGameplayTagSourceType(2);
    pub const RESTRICTED_TAG_LIST: EGameplayTagSourceType = EGameplayTagSourceType(3);
    pub const DATA_TABLE: EGameplayTagSourceType = EGameplayTagSourceType(4);
    pub const INVALID: EGameplayTagSourceType = EGameplayTagSourceType(5);
}
#[repr(transparent)]
pub struct EGameplayContainerMatchType(pub u8);
impl EGameplayContainerMatchType {
    pub const ANY: EGameplayContainerMatchType = EGameplayContainerMatchType(0);
    pub const ALL: EGameplayContainerMatchType = EGameplayContainerMatchType(1);
}
