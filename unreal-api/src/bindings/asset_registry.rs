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
pub static mut U_ASSET_REGISTRY_HELPERS_TO_SOFT_OBJECT_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HELPERS_SORTING_PREDICATE_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HELPERS_SORT_BY_PREDICATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HELPERS_SORT_BY_ASSET_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HELPERS_SET_FILTER_TAGS_AND_VALUES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HELPERS_IS_VALID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HELPERS_IS_U_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HELPERS_IS_REDIRECTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HELPERS_IS_ASSET_LOADED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HELPERS_IS_ASSET_COOKED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HELPERS_GET_TAG_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HELPERS_GET_FULL_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HELPERS_GET_EXPORT_TEXT_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HELPERS_GET_DERIVED_CLASS_ASSET_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HELPERS_GET_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HELPERS_GET_BLUEPRINT_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HELPERS_GET_ASSET_REGISTRY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HELPERS_GET_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HELPERS_FIND_ASSET_NATIVE_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HELPERS_CREATE_ASSET_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HELPERS_ASSET_HAS_EDITOR_ONLY_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_WAIT_FOR_PACKAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_WAIT_FOR_COMPLETION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_USE_FILTER_TO_EXCLUDE_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_SEARCH_ALL_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_SCAN_PATHS_SYNCHRONOUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_SCAN_MODIFIED_ASSET_FILES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_SCAN_FILES_SYNCHRONOUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_RUN_ASSETS_THROUGH_FILTER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_PRIORITIZE_SEARCH_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_K2_GET_REFERENCERS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_K2_GET_DEPENDENCIES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_K2_GET_ASSET_BY_OBJECT_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_IS_SEARCH_ASYNC: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_IS_SEARCH_ALL_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_IS_LOADING_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_HAS_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_GET_SUB_PATHS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_GET_IN_MEMORY_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_GET_DERIVED_CLASS_NAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_GET_ASSETS_BY_PATHS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_GET_ASSETS_BY_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_GET_ASSETS_BY_PACKAGE_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_GET_ASSETS_BY_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_GET_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_GET_ASSET_BY_OBJECT_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_GET_ANCESTOR_CLASS_NAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_GET_ALL_CACHED_PATHS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_REGISTRY_GET_ALL_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAssetRegistryHelpers::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ToSoftObjectPath"),
            &raw mut U_ASSET_REGISTRY_HELPERS_TO_SOFT_OBJECT_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SortingPredicate__DelegateSignature"),
            &raw mut U_ASSET_REGISTRY_HELPERS_SORTING_PREDICATE_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SortByPredicate"),
            &raw mut U_ASSET_REGISTRY_HELPERS_SORT_BY_PREDICATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SortByAssetName"),
            &raw mut U_ASSET_REGISTRY_HELPERS_SORT_BY_ASSET_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterTagsAndValues"),
            &raw mut U_ASSET_REGISTRY_HELPERS_SET_FILTER_TAGS_AND_VALUES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValid"),
            &raw mut U_ASSET_REGISTRY_HELPERS_IS_VALID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsUAsset"),
            &raw mut U_ASSET_REGISTRY_HELPERS_IS_U_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRedirector"),
            &raw mut U_ASSET_REGISTRY_HELPERS_IS_REDIRECTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAssetLoaded"),
            &raw mut U_ASSET_REGISTRY_HELPERS_IS_ASSET_LOADED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAssetCooked"),
            &raw mut U_ASSET_REGISTRY_HELPERS_IS_ASSET_COOKED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTagValue"),
            &raw mut U_ASSET_REGISTRY_HELPERS_GET_TAG_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFullName"),
            &raw mut U_ASSET_REGISTRY_HELPERS_GET_FULL_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetExportTextName"),
            &raw mut U_ASSET_REGISTRY_HELPERS_GET_EXPORT_TEXT_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDerivedClassAssetData"),
            &raw mut U_ASSET_REGISTRY_HELPERS_GET_DERIVED_CLASS_ASSET_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetClass"),
            &raw mut U_ASSET_REGISTRY_HELPERS_GET_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlueprintAssets"),
            &raw mut U_ASSET_REGISTRY_HELPERS_GET_BLUEPRINT_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetRegistry"),
            &raw mut U_ASSET_REGISTRY_HELPERS_GET_ASSET_REGISTRY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAsset"),
            &raw mut U_ASSET_REGISTRY_HELPERS_GET_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindAssetNativeClass"),
            &raw mut U_ASSET_REGISTRY_HELPERS_FIND_ASSET_NATIVE_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateAssetData"),
            &raw mut U_ASSET_REGISTRY_HELPERS_CREATE_ASSET_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssetHasEditorOnlyData"),
            &raw mut U_ASSET_REGISTRY_HELPERS_ASSET_HAS_EDITOR_ONLY_DATA,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAssetRegistry::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WaitForPackage"),
            &raw mut U_ASSET_REGISTRY_WAIT_FOR_PACKAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WaitForCompletion"),
            &raw mut U_ASSET_REGISTRY_WAIT_FOR_COMPLETION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UseFilterToExcludeAssets"),
            &raw mut U_ASSET_REGISTRY_USE_FILTER_TO_EXCLUDE_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SearchAllAssets"),
            &raw mut U_ASSET_REGISTRY_SEARCH_ALL_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScanPathsSynchronous"),
            &raw mut U_ASSET_REGISTRY_SCAN_PATHS_SYNCHRONOUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScanModifiedAssetFiles"),
            &raw mut U_ASSET_REGISTRY_SCAN_MODIFIED_ASSET_FILES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScanFilesSynchronous"),
            &raw mut U_ASSET_REGISTRY_SCAN_FILES_SYNCHRONOUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RunAssetsThroughFilter"),
            &raw mut U_ASSET_REGISTRY_RUN_ASSETS_THROUGH_FILTER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PrioritizeSearchPath"),
            &raw mut U_ASSET_REGISTRY_PRIORITIZE_SEARCH_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetReferencers"),
            &raw mut U_ASSET_REGISTRY_K2_GET_REFERENCERS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetDependencies"),
            &raw mut U_ASSET_REGISTRY_K2_GET_DEPENDENCIES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetAssetByObjectPath"),
            &raw mut U_ASSET_REGISTRY_K2_GET_ASSET_BY_OBJECT_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsSearchAsync"),
            &raw mut U_ASSET_REGISTRY_IS_SEARCH_ASYNC,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsSearchAllAssets"),
            &raw mut U_ASSET_REGISTRY_IS_SEARCH_ALL_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLoadingAssets"),
            &raw mut U_ASSET_REGISTRY_IS_LOADING_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasAssets"),
            &raw mut U_ASSET_REGISTRY_HAS_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSubPaths"),
            &raw mut U_ASSET_REGISTRY_GET_SUB_PATHS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInMemoryAssets"),
            &raw mut U_ASSET_REGISTRY_GET_IN_MEMORY_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDerivedClassNames"),
            &raw mut U_ASSET_REGISTRY_GET_DERIVED_CLASS_NAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetsByPaths"),
            &raw mut U_ASSET_REGISTRY_GET_ASSETS_BY_PATHS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetsByPath"),
            &raw mut U_ASSET_REGISTRY_GET_ASSETS_BY_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetsByPackageName"),
            &raw mut U_ASSET_REGISTRY_GET_ASSETS_BY_PACKAGE_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetsByClass"),
            &raw mut U_ASSET_REGISTRY_GET_ASSETS_BY_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssets"),
            &raw mut U_ASSET_REGISTRY_GET_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetByObjectPath"),
            &raw mut U_ASSET_REGISTRY_GET_ASSET_BY_OBJECT_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAncestorClassNames"),
            &raw mut U_ASSET_REGISTRY_GET_ANCESTOR_CLASS_NAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllCachedPaths"),
            &raw mut U_ASSET_REGISTRY_GET_ALL_CACHED_PATHS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllAssets"),
            &raw mut U_ASSET_REGISTRY_GET_ALL_ASSETS,
        );
    }
}
#[repr(C, align(8))]
pub struct FTagAndValue {
    pub tag: FName,
    pub value: FString,
}
impl FTagAndValue {}
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
impl FAssetRegistryDependencyOptions {}
#[repr(C, align(8))]
pub struct UAssetRegistryHelpers {
    __padding_end: [u8; 48],
}
impl UAssetRegistryHelpers {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetRegistryHelpers")
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
pub struct IAssetRegistry {}
#[repr(C, align(8))]
pub struct UAssetRegistry {
    __padding_end: [u8; 48],
}
impl UAssetRegistry {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetRegistry")
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
pub struct UAssetRegistryImpl {
    __padding_end: [u8; 5880],
}
impl UAssetRegistryImpl {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetRegistryImpl")
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
pub struct FSortByPredicate_SortingPredicate {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct EAssetRegistrySortOrder(pub u8);
impl EAssetRegistrySortOrder {
    pub const ASCENDING: EAssetRegistrySortOrder = EAssetRegistrySortOrder(0);
    pub const DESCENDING: EAssetRegistrySortOrder = EAssetRegistrySortOrder(1);
}
