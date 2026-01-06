#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut U_CONTENT_BROWSER_DATA_SUBSYSTEM_GET_ITEMS_UNDER_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTENT_BROWSER_DATA_SUBSYSTEM_GET_ITEMS_AT_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTENT_BROWSER_DATA_SUBSYSTEM_GET_ITEM_AT_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTENT_BROWSER_DATA_SUBSYSTEM_GET_AVAILABLE_DATA_SOURCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTENT_BROWSER_DATA_SUBSYSTEM_GET_ACTIVE_DATA_SOURCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTENT_BROWSER_DATA_SUBSYSTEM_DEACTIVATE_DATA_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTENT_BROWSER_DATA_SUBSYSTEM_DEACTIVATE_ALL_DATA_SOURCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTENT_BROWSER_DATA_SUBSYSTEM_ACTIVATE_DATA_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTENT_BROWSER_DATA_SUBSYSTEM_ACTIVATE_ALL_DATA_SOURCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTENT_BROWSER_ITEM_LIBRARY_IS_FOLDER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTENT_BROWSER_ITEM_LIBRARY_IS_FILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTENT_BROWSER_ITEM_LIBRARY_GET_VIRTUAL_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTENT_BROWSER_ITEM_LIBRARY_GET_DISPLAY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTENT_BROWSER_ITEM_PATH_EXTENSIONS_SET_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTENT_BROWSER_ITEM_PATH_EXTENSIONS_MAKE_CONTENT_BROWSER_ITEM_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTENT_BROWSER_ITEM_PATH_EXTENSIONS_GET_VIRTUAL_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTENT_BROWSER_ITEM_PATH_EXTENSIONS_GET_INTERNAL_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTENT_BROWSER_ITEM_PATH_EXTENSIONS_BREAK_CONTENT_BROWSER_ITEM_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UContentBrowserDataSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetItemsUnderPath"),
            &raw mut U_CONTENT_BROWSER_DATA_SUBSYSTEM_GET_ITEMS_UNDER_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetItemsAtPath"),
            &raw mut U_CONTENT_BROWSER_DATA_SUBSYSTEM_GET_ITEMS_AT_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetItemAtPath"),
            &raw mut U_CONTENT_BROWSER_DATA_SUBSYSTEM_GET_ITEM_AT_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAvailableDataSources"),
            &raw mut U_CONTENT_BROWSER_DATA_SUBSYSTEM_GET_AVAILABLE_DATA_SOURCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActiveDataSources"),
            &raw mut U_CONTENT_BROWSER_DATA_SUBSYSTEM_GET_ACTIVE_DATA_SOURCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeactivateDataSource"),
            &raw mut U_CONTENT_BROWSER_DATA_SUBSYSTEM_DEACTIVATE_DATA_SOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeactivateAllDataSources"),
            &raw mut U_CONTENT_BROWSER_DATA_SUBSYSTEM_DEACTIVATE_ALL_DATA_SOURCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivateDataSource"),
            &raw mut U_CONTENT_BROWSER_DATA_SUBSYSTEM_ACTIVATE_DATA_SOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivateAllDataSources"),
            &raw mut U_CONTENT_BROWSER_DATA_SUBSYSTEM_ACTIVATE_ALL_DATA_SOURCES,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UContentBrowserItemLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsFolder"),
            &raw mut U_CONTENT_BROWSER_ITEM_LIBRARY_IS_FOLDER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsFile"),
            &raw mut U_CONTENT_BROWSER_ITEM_LIBRARY_IS_FILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVirtualPath"),
            &raw mut U_CONTENT_BROWSER_ITEM_LIBRARY_GET_VIRTUAL_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDisplayName"),
            &raw mut U_CONTENT_BROWSER_ITEM_LIBRARY_GET_DISPLAY_NAME,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UContentBrowserItemPathExtensions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPath"),
            &raw mut U_CONTENT_BROWSER_ITEM_PATH_EXTENSIONS_SET_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeContentBrowserItemPath"),
            &raw mut U_CONTENT_BROWSER_ITEM_PATH_EXTENSIONS_MAKE_CONTENT_BROWSER_ITEM_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVirtualPath"),
            &raw mut U_CONTENT_BROWSER_ITEM_PATH_EXTENSIONS_GET_VIRTUAL_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInternalPath"),
            &raw mut U_CONTENT_BROWSER_ITEM_PATH_EXTENSIONS_GET_INTERNAL_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BreakContentBrowserItemPath"),
            &raw mut U_CONTENT_BROWSER_ITEM_PATH_EXTENSIONS_BREAK_CONTENT_BROWSER_ITEM_PATH,
        );
    }
}
#[repr(C, align(8))]
pub struct FContentBrowserDataFilter {
    pub b_recursive_paths: bool,
    pub item_type_filter: EContentBrowserItemTypeFilter,
    pub item_category_filter: EContentBrowserItemCategoryFilter,
    pub item_attribute_filter: EContentBrowserItemAttributeFilter,
    __padding_end: [u8; 28],
}
impl FContentBrowserDataFilter {}
#[repr(C, align(8))]
pub struct FContentBrowserDataObjectFilter {
    pub object_names_to_include: TArray<FName>,
    pub object_names_to_exclude: TArray<FName>,
    pub b_on_disk_objects_only: bool,
    __padding_end: [u8; 167],
}
impl FContentBrowserDataObjectFilter {}
#[repr(C, align(8))]
pub struct FContentBrowserDataPackageFilter {
    pub package_names_to_include: TArray<FName>,
    pub package_names_to_exclude: TArray<FName>,
    pub package_paths_to_include: TArray<FName>,
    pub package_paths_to_exclude: TArray<FName>,
    pub b_recursive_package_paths_to_include: bool,
    pub b_recursive_package_paths_to_exclude: bool,
    __padding_end: [u8; 22],
}
impl FContentBrowserDataPackageFilter {}
#[repr(C, align(8))]
pub struct FContentBrowserDataClassFilter {
    pub class_names_to_include: TArray<FString>,
    pub class_names_to_exclude: TArray<FString>,
    pub b_recursive_class_names_to_include: bool,
    pub b_recursive_class_names_to_exclude: bool,
    __padding_end: [u8; 22],
}
impl FContentBrowserDataClassFilter {}
#[repr(C, align(8))]
pub struct FContentBrowserDataCollectionFilter {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub b_include_child_collections: bool,
    __padding_end: [u8; 7],
}
impl FContentBrowserDataCollectionFilter {}
#[repr(C, align(8))]
pub struct FContentBrowserItem {
    __padding_end: [u8; 96],
}
impl FContentBrowserItem {}
#[repr(C, align(4))]
pub struct FContentBrowserItemPath {
    __padding_end: [u8; 24],
}
impl FContentBrowserItemPath {}
#[repr(C, align(8))]
pub struct UContentBrowserDataMenuContext_AddNewMenu {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub selected_paths: TArray<FName>,
    pub b_contains_valid_package_path: bool,
    pub b_can_be_modified: bool,
    pub owner_domain: EContentBrowserDataMenuContext_AddNewMenuDomain,
    __padding_end: [u8; 37],
}
impl UContentBrowserDataMenuContext_AddNewMenu {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UContentBrowserDataMenuContext_AddNewMenu")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UContentBrowserDataMenuContext_FolderMenu {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub selected_items: TArray<FContentBrowserItem>,
    pub b_can_be_modified: bool,
    __padding_end: [u8; 23],
}
impl UContentBrowserDataMenuContext_FolderMenu {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UContentBrowserDataMenuContext_FolderMenu")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UContentBrowserDataMenuContext_FileMenu {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub selected_items: TArray<FContentBrowserItem>,
    #[doc(hidden)]
    __padding_96: [u8; 32],
    pub b_can_be_modified: bool,
    pub b_can_view: bool,
    pub b_has_cooked_packages: bool,
    pub b_contains_unsupported_assets: bool,
    __padding_end: [u8; 68],
}
impl UContentBrowserDataMenuContext_FileMenu {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UContentBrowserDataMenuContext_FileMenu")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UContentBrowserDataMenuContext_DragDropMenu {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub drop_target_item: FContentBrowserItem,
    pub dragged_items: TArray<FContentBrowserItem>,
    pub b_can_move: bool,
    pub b_can_copy: bool,
    __padding_end: [u8; 22],
}
impl UContentBrowserDataMenuContext_DragDropMenu {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UContentBrowserDataMenuContext_DragDropMenu")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UContentBrowserDataSource {
    __padding_end: [u8; 304],
}
impl UContentBrowserDataSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UContentBrowserDataSource")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UContentBrowserDataSubsystem {
    __padding_end: [u8; 560],
}
impl UContentBrowserDataSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UContentBrowserDataSubsystem")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UContentBrowserItemLibrary {
    __padding_end: [u8; 48],
}
impl UContentBrowserItemLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UContentBrowserItemLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UContentBrowserItemPathExtensions {
    __padding_end: [u8; 48],
}
impl UContentBrowserItemPathExtensions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UContentBrowserItemPathExtensions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
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
#[repr(transparent)]
pub struct EContentBrowserPathType(pub u8);
impl EContentBrowserPathType {
    pub const NONE: EContentBrowserPathType = EContentBrowserPathType(0);
    pub const INTERNAL: EContentBrowserPathType = EContentBrowserPathType(1);
    pub const VIRTUAL: EContentBrowserPathType = EContentBrowserPathType(2);
}
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
