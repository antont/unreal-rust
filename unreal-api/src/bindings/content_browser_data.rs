#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FContentBrowserDataFilter {
    pub b_recursive_paths: bool,
    pub item_type_filter: EContentBrowserItemTypeFilter,
    pub item_category_filter: EContentBrowserItemCategoryFilter,
    pub item_attribute_filter: EContentBrowserItemAttributeFilter,
}
#[repr(C, align(8))]
pub struct FContentBrowserDataObjectFilter {
    pub object_names_to_include: TArray<FName>,
    pub object_names_to_exclude: TArray<FName>,
    pub b_on_disk_objects_only: bool,
}
#[repr(C, align(8))]
pub struct FContentBrowserDataPackageFilter {
    pub package_names_to_include: TArray<FName>,
    pub package_names_to_exclude: TArray<FName>,
    pub package_paths_to_include: TArray<FName>,
    pub package_paths_to_exclude: TArray<FName>,
    pub b_recursive_package_paths_to_include: bool,
    pub b_recursive_package_paths_to_exclude: bool,
}
#[repr(C, align(8))]
pub struct FContentBrowserDataClassFilter {
    pub class_names_to_include: TArray<FString>,
    pub class_names_to_exclude: TArray<FString>,
    pub b_recursive_class_names_to_include: bool,
    pub b_recursive_class_names_to_exclude: bool,
}
#[repr(C, align(8))]
pub struct FContentBrowserDataCollectionFilter {
    pub b_include_child_collections: bool,
}
#[repr(C, align(8))]
pub struct FContentBrowserDataUnsupportedClassFilter {}
#[repr(C, align(8))]
pub struct FContentBrowserDataLegacyFilter {}
#[repr(C, align(8))]
pub struct FContentBrowserCompiledSubsystemFilter {}
#[repr(C, align(8))]
pub struct FContentBrowserCompiledVirtualFolderFilter {}
#[repr(C, align(8))]
pub struct FContentBrowserItem {}
#[repr(C, align(4))]
pub struct FContentBrowserItemPath {}
pub struct UContentBrowserDataMenuContext_AddNewMenu {
    pub selected_paths: TArray<FName>,
    pub b_contains_valid_package_path: bool,
    pub b_can_be_modified: bool,
    pub owner_domain: EContentBrowserDataMenuContext_AddNewMenuDomain,
}
pub struct UContentBrowserDataMenuContext_FolderMenu {
    pub selected_items: TArray<FContentBrowserItem>,
    pub b_can_be_modified: bool,
}
pub struct UContentBrowserDataMenuContext_FileMenu {
    pub selected_items: TArray<FContentBrowserItem>,
    pub b_can_be_modified: bool,
    pub b_can_view: bool,
    pub b_has_cooked_packages: bool,
    pub b_contains_unsupported_assets: bool,
}
pub struct UContentBrowserDataMenuContext_DragDropMenu {
    pub drop_target_item: FContentBrowserItem,
    pub dragged_items: TArray<FContentBrowserItem>,
    pub b_can_move: bool,
    pub b_can_copy: bool,
}
pub struct UContentBrowserDataSource {}
pub struct UContentBrowserDataSubsystem {
    pub enabled_data_sources: TArray<FName>,
}
pub struct UContentBrowserItemLibrary {}
pub struct UContentBrowserItemPathExtensions {}
