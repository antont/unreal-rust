#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FContentSourceColumn {}
#[repr(C, align(8))]
pub struct FAssetViewInstanceConfig {
    pub thumbnail_size: u8,
    pub view_type: u8,
    pub hidden_columns: TArray<FName>,
    pub list_hidden_columns: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FPathViewConfig {
    pub b_expanded: bool,
    pub plugin_filters: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FContentBrowserInstanceConfig {
    pub path_view: FPathViewConfig,
    pub b_show_favorites: bool,
    pub b_favorites_expanded: bool,
    pub b_sources_expanded: bool,
    pub b_filter_recursively: bool,
    pub b_show_folders: bool,
    pub b_show_empty_folders: bool,
    pub b_show_engine_content: bool,
    pub b_show_developer_content: bool,
    pub b_show_localized_content: bool,
    pub b_show_plugin_content: bool,
    pub b_show_cpp_folders: bool,
    pub b_search_classes: bool,
    pub b_search_asset_paths: bool,
    pub b_search_collections: bool,
    pub b_show_asset_access_specifier: bool,
}
#[repr(C, align(4))]
pub struct FContentBrowserPluginSettings {
    pub plugin_name: FName,
    pub root_folder_sort_priority: f32,
}
#[repr(C, align(8))]
pub struct FTextFilterKeyValueHandlerEntry {
    pub key: FName,
    pub handler_class: TSoftObjectPtr<UClass>,
}
pub struct UContentBrowserFrontEndFilterExtension {}
pub struct UContentBrowserPathViewContextMenuContext {}
pub struct UTextFilterKeyValueHandler {}
pub struct UTextFilterValueHandler {}
pub struct UAssetViewConfig {
    pub instances: TMap<FName, FAssetViewInstanceConfig>,
}
pub struct UContentBrowserConfig {
    pub favorites: TSet<FString>,
    pub instances: TMap<FName, FContentBrowserInstanceConfig>,
}
pub struct UContentBrowserCollectionProjectSettings {
    pub excluded_collections_from_view: TArray<FName>,
}
pub struct UContentBrowserAssetContextMenuContext {
    pub common_asset_definition: UPtr<UAssetDefinition>,
    pub selected_assets: TArray<FAssetData>,
    pub common_class: TSubclassOf<UObject>,
    pub b_can_be_modified: bool,
    pub b_can_view: bool,
    pub b_has_cooked_packages: bool,
    pub b_contains_unsupported_assets: bool,
}
pub struct UContentBrowserAssetViewContextMenuContext {}
pub struct UContentBrowserAssetSortingContextMenuContext {}
pub struct UContentBrowserMenuContext {}
pub struct UContentBrowserFolderContext {
    pub b_can_be_modified: bool,
    pub b_no_folder_on_disk: bool,
    pub num_asset_paths: i32,
    pub num_class_paths: i32,
    pub selected_package_paths: TArray<FString>,
}
pub struct UContentBrowserAddNewContextMenuContext {}
pub struct UContentBrowserToolbarMenuContext {}
pub struct UContentSourcesViewMenuContext {}
pub struct UTextFilterKeyValueHandlers {
    pub text_filter_key_value_handlers: TArray<FTextFilterKeyValueHandlerEntry>,
}
pub struct UTextFilterValueHandlers {
    pub text_filter_value_handlers: TArray<TSoftObjectPtr<UClass>>,
}
