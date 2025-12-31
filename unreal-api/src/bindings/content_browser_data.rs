#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EContentBrowserItemTypeFilter(pub u8);
impl EContentBrowserItemTypeFilter {
    pub const INCLUDE_NONE: EContentBrowserItemTypeFilter = EContentBrowserItemTypeFilter(
        0,
    );
    pub const INCLUDE_FOLDERS: EContentBrowserItemTypeFilter = EContentBrowserItemTypeFilter(
        1,
    );
    pub const INCLUDE_FILES: EContentBrowserItemTypeFilter = EContentBrowserItemTypeFilter(
        2,
    );
    pub const INCLUDE_ALL: EContentBrowserItemTypeFilter = EContentBrowserItemTypeFilter(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EContentBrowserItemCategoryFilter(pub u8);
impl EContentBrowserItemCategoryFilter {
    pub const INCLUDE_NONE: EContentBrowserItemCategoryFilter = EContentBrowserItemCategoryFilter(
        0,
    );
    pub const INCLUDE_ASSETS: EContentBrowserItemCategoryFilter = EContentBrowserItemCategoryFilter(
        1,
    );
    pub const INCLUDE_CLASSES: EContentBrowserItemCategoryFilter = EContentBrowserItemCategoryFilter(
        2,
    );
    pub const INCLUDE_COLLECTIONS: EContentBrowserItemCategoryFilter = EContentBrowserItemCategoryFilter(
        4,
    );
    pub const INCLUDE_REDIRECTORS: EContentBrowserItemCategoryFilter = EContentBrowserItemCategoryFilter(
        8,
    );
    pub const INCLUDE_MISC: EContentBrowserItemCategoryFilter = EContentBrowserItemCategoryFilter(
        16,
    );
    pub const INCLUDE_ALL: EContentBrowserItemCategoryFilter = EContentBrowserItemCategoryFilter(
        31,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EContentBrowserItemAttributeFilter(pub u8);
impl EContentBrowserItemAttributeFilter {
    pub const INCLUDE_NONE: EContentBrowserItemAttributeFilter = EContentBrowserItemAttributeFilter(
        0,
    );
    pub const INCLUDE_PROJECT: EContentBrowserItemAttributeFilter = EContentBrowserItemAttributeFilter(
        1,
    );
    pub const INCLUDE_ENGINE: EContentBrowserItemAttributeFilter = EContentBrowserItemAttributeFilter(
        2,
    );
    pub const INCLUDE_PLUGINS: EContentBrowserItemAttributeFilter = EContentBrowserItemAttributeFilter(
        4,
    );
    pub const INCLUDE_DEVELOPER: EContentBrowserItemAttributeFilter = EContentBrowserItemAttributeFilter(
        8,
    );
    pub const INCLUDE_LOCALIZED: EContentBrowserItemAttributeFilter = EContentBrowserItemAttributeFilter(
        16,
    );
    pub const INCLUDE_ALL: EContentBrowserItemAttributeFilter = EContentBrowserItemAttributeFilter(
        31,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EContentBrowserPathType(pub u8);
impl EContentBrowserPathType {
    pub const NONE: EContentBrowserPathType = EContentBrowserPathType(0);
    pub const INTERNAL: EContentBrowserPathType = EContentBrowserPathType(1);
    pub const VIRTUAL: EContentBrowserPathType = EContentBrowserPathType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EContentBrowserDataMenuContext_AddNewMenuDomain(pub u8);
impl EContentBrowserDataMenuContext_AddNewMenuDomain {
    pub const TOOLBAR: EContentBrowserDataMenuContext_AddNewMenuDomain = EContentBrowserDataMenuContext_AddNewMenuDomain(
        0,
    );
    pub const ASSET_VIEW: EContentBrowserDataMenuContext_AddNewMenuDomain = EContentBrowserDataMenuContext_AddNewMenuDomain(
        1,
    );
    pub const PATH_VIEW: EContentBrowserDataMenuContext_AddNewMenuDomain = EContentBrowserDataMenuContext_AddNewMenuDomain(
        2,
    );
}
