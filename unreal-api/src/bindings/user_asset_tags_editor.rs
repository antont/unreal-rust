#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAssetDefinition_TaggedAssetBrowserConfiguration {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_TaggedAssetBrowserConfiguration {}
#[repr(C, align(8))]
pub struct UAssetEditor_TaggedAssetBrowserConfiguration {
    __padding_end: [u8; 72],
}
impl UAssetEditor_TaggedAssetBrowserConfiguration {}
#[repr(C, align(8))]
pub struct ULocalFavoriteUserAssetTagsConfig {
    __padding_end: [u8; 136],
}
impl ULocalFavoriteUserAssetTagsConfig {}
#[repr(C, align(8))]
pub struct UProjectUserAssetTagSettings {
    __padding_end: [u8; 184],
}
impl UProjectUserAssetTagSettings {}
#[repr(C, align(8))]
pub struct UTaggedAssetBrowserConfig {
    __padding_end: [u8; 168],
}
impl UTaggedAssetBrowserConfig {}
#[repr(C, align(8))]
pub struct UTaggedAssetBrowserConfiguration {
    __padding_end: [u8; 120],
}
impl UTaggedAssetBrowserConfiguration {}
#[repr(C, align(8))]
pub struct UTaggedAssetBrowserConfigurationFactory {
    __padding_end: [u8; 136],
}
impl UTaggedAssetBrowserConfigurationFactory {}
#[repr(C, align(8))]
pub struct UTaggedAssetBrowserConfigurationHierarchyViewModel {
    __padding_end: [u8; 512],
}
impl UTaggedAssetBrowserConfigurationHierarchyViewModel {}
#[repr(C, align(8))]
pub struct UTaggedAssetBrowserFilterBase {
    __padding_end: [u8; 264],
}
impl UTaggedAssetBrowserFilterBase {}
#[repr(C, align(8))]
pub struct UTaggedAssetBrowserFilterRoot {
    __padding_end: [u8; 208],
}
impl UTaggedAssetBrowserFilterRoot {}
#[repr(C, align(16))]
pub struct UTaggedAssetBrowserSection {
    __padding_end: [u8; 480],
}
impl UTaggedAssetBrowserSection {}
#[repr(C, align(8))]
pub struct UTaggedAssetBrowserFilter_All {
    __padding_end: [u8; 264],
}
impl UTaggedAssetBrowserFilter_All {}
#[repr(C, align(8))]
pub struct UTaggedAssetBrowserFilter_UserAssetTag {
    __padding_end: [u8; 280],
}
impl UTaggedAssetBrowserFilter_UserAssetTag {}
#[repr(C, align(8))]
pub struct UTaggedAssetBrowserFilter_UserAssetTagCollection {
    __padding_end: [u8; 296],
}
impl UTaggedAssetBrowserFilter_UserAssetTagCollection {}
#[repr(C, align(8))]
pub struct UTaggedAssetBrowserFilter_Recent {
    __padding_end: [u8; 304],
}
impl UTaggedAssetBrowserFilter_Recent {}
#[repr(C, align(8))]
pub struct UTaggedAssetBrowserFilter_Directories {
    __padding_end: [u8; 280],
}
impl UTaggedAssetBrowserFilter_Directories {}
#[repr(C, align(8))]
pub struct UTaggedAssetBrowserFilter_Class {
    __padding_end: [u8; 280],
}
impl UTaggedAssetBrowserFilter_Class {}
#[repr(C, align(8))]
pub struct UUserAssetTagEditorContext {
    __padding_end: [u8; 64],
}
impl UUserAssetTagEditorContext {}
#[repr(C, align(8))]
pub struct UTaggedAssetBrowserMenuContext {
    __padding_end: [u8; 64],
}
impl UTaggedAssetBrowserMenuContext {}
#[repr(C, align(8))]
pub struct UUserAssetTagProvider {
    __padding_end: [u8; 48],
}
impl UUserAssetTagProvider {}
#[repr(C, align(8))]
pub struct UUserAssetTagProvider_LocalAssetTypeFavorites {
    __padding_end: [u8; 48],
}
impl UUserAssetTagProvider_LocalAssetTypeFavorites {}
#[repr(C, align(8))]
pub struct UUserAssetTagProvider_Project {
    __padding_end: [u8; 48],
}
impl UUserAssetTagProvider_Project {}
#[repr(C, align(8))]
pub struct UUserAssetTagsEditorConfig {
    __padding_end: [u8; 136],
}
impl UUserAssetTagsEditorConfig {}
#[repr(transparent)]
pub struct EUserAssetTagProviderMenuType(pub i32);
impl EUserAssetTagProviderMenuType {
    pub const SECTION: EUserAssetTagProviderMenuType = EUserAssetTagProviderMenuType(0);
    pub const SUB_MENU: EUserAssetTagProviderMenuType = EUserAssetTagProviderMenuType(1);
}
