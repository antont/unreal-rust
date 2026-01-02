#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UContentBrowserFrontEndFilterExtension {
    __padding_end: [u8; 48],
}
impl UContentBrowserFrontEndFilterExtension {}
#[repr(C, align(8))]
pub struct UContentBrowserPathViewContextMenuContext {
    __padding_end: [u8; 80],
}
impl UContentBrowserPathViewContextMenuContext {}
#[repr(C, align(8))]
pub struct UTextFilterKeyValueHandler {
    __padding_end: [u8; 48],
}
impl UTextFilterKeyValueHandler {}
#[repr(C, align(8))]
pub struct UTextFilterValueHandler {
    __padding_end: [u8; 48],
}
impl UTextFilterValueHandler {}
#[repr(C, align(8))]
pub struct UAssetViewConfig {
    __padding_end: [u8; 128],
}
impl UAssetViewConfig {}
#[repr(C, align(8))]
pub struct UContentBrowserConfig {
    __padding_end: [u8; 208],
}
impl UContentBrowserConfig {}
#[repr(C, align(8))]
pub struct UContentBrowserCollectionProjectSettings {
    __padding_end: [u8; 120],
}
impl UContentBrowserCollectionProjectSettings {}
#[repr(C, align(8))]
pub struct UContentBrowserAssetContextMenuContext {
    #[doc(hidden)]
    __padding_88: [u8; 88],
    pub selected_assets: TArray<crate::bindings::core_u_object::FAssetData>,
    #[doc(hidden)]
    __padding_115: [u8; 11],
    pub b_contains_unsupported_assets: bool,
    __padding_end: [u8; 4],
}
impl UContentBrowserAssetContextMenuContext {}
#[repr(C, align(8))]
pub struct UContentBrowserAssetViewContextMenuContext {
    __padding_end: [u8; 80],
}
impl UContentBrowserAssetViewContextMenuContext {}
#[repr(C, align(8))]
pub struct UContentBrowserAssetSortingContextMenuContext {
    __padding_end: [u8; 96],
}
impl UContentBrowserAssetSortingContextMenuContext {}
#[repr(C, align(8))]
pub struct UContentBrowserMenuContext {
    __padding_end: [u8; 64],
}
impl UContentBrowserMenuContext {}
#[repr(C, align(8))]
pub struct UContentBrowserFolderContext {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub b_can_be_modified: bool,
    pub b_no_folder_on_disk: bool,
    pub num_asset_paths: i32,
    pub num_class_paths: i32,
    pub selected_package_paths: TArray<FString>,
    __padding_end: [u8; 24],
}
impl UContentBrowserFolderContext {}
#[repr(C, align(8))]
pub struct UContentBrowserAddNewContextMenuContext {
    __padding_end: [u8; 64],
}
impl UContentBrowserAddNewContextMenuContext {}
#[repr(C, align(8))]
pub struct UContentBrowserToolbarMenuContext {
    __padding_end: [u8; 88],
}
impl UContentBrowserToolbarMenuContext {}
#[repr(C, align(8))]
pub struct UContentSourcesViewMenuContext {
    __padding_end: [u8; 64],
}
impl UContentSourcesViewMenuContext {}
#[repr(C, align(8))]
pub struct UTextFilterKeyValueHandlers {
    __padding_end: [u8; 64],
}
impl UTextFilterKeyValueHandlers {}
#[repr(C, align(8))]
pub struct UTextFilterValueHandlers {
    __padding_end: [u8; 64],
}
impl UTextFilterValueHandlers {}
