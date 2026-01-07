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
    pub fn to_soft_object_path(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> crate::bindings::core_u_object::FSoftObjectPath {
        let mut __stack = crate::core_data::StackAlloc::<192>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_TO_SOFT_OBJECT_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_TO_SOFT_OBJECT_PATH,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(152)
                .cast::<crate::bindings::core_u_object::FSoftObjectPath>()
                .read()
        }
    }
    pub fn sort_by_predicate(
        assets: &mut TArray<crate::bindings::core_u_object::FAssetData>,
        sorting_predicate: FSortByPredicate_SortingPredicate,
        sort_order: EAssetRegistrySortOrder,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_SORT_BY_PREDICATE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                assets,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sorting_predicate,
                __buffer.add(16).cast::<FSortByPredicate_SortingPredicate>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sort_order,
                __buffer.add(48).cast::<EAssetRegistrySortOrder>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_SORT_BY_PREDICATE,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .swap(assets);
        }
    }
    pub fn sort_by_asset_name(
        assets: &mut TArray<crate::bindings::core_u_object::FAssetData>,
        sort_order: EAssetRegistrySortOrder,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_SORT_BY_ASSET_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                assets,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sort_order,
                __buffer.add(16).cast::<EAssetRegistrySortOrder>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_SORT_BY_ASSET_NAME,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .swap(assets);
        }
    }
    pub fn set_filter_tags_and_values(
        in_filter: &crate::bindings::core_u_object::FARFilter,
        in_tags_and_values: &TArray<FTagAndValue>,
    ) -> crate::bindings::core_u_object::FARFilter {
        let mut __stack = crate::core_data::StackAlloc::<720>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_SET_FILTER_TAGS_AND_VALUES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_filter,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FARFilter>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_tags_and_values,
                __buffer.add(352).cast::<TArray<FTagAndValue>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_SET_FILTER_TAGS_AND_VALUES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(368).cast::<crate::bindings::core_u_object::FARFilter>().read()
        }
    }
    pub fn is_valid(in_asset_data: &crate::bindings::core_u_object::FAssetData) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<153>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_IS_VALID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_IS_VALID,
                __buffer,
            )
        };
        unsafe { __buffer.add(152).cast::<bool>().read() }
    }
    pub fn is_u_asset(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<153>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_IS_U_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_IS_U_ASSET,
                __buffer,
            )
        };
        unsafe { __buffer.add(152).cast::<bool>().read() }
    }
    pub fn is_redirector(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<153>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_IS_REDIRECTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_IS_REDIRECTOR,
                __buffer,
            )
        };
        unsafe { __buffer.add(152).cast::<bool>().read() }
    }
    pub fn is_asset_loaded(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<153>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_IS_ASSET_LOADED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_IS_ASSET_LOADED,
                __buffer,
            )
        };
        unsafe { __buffer.add(152).cast::<bool>().read() }
    }
    pub fn is_asset_cooked(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<153>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_IS_ASSET_COOKED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_IS_ASSET_COOKED,
                __buffer,
            )
        };
        unsafe { __buffer.add(152).cast::<bool>().read() }
    }
    pub fn get_tag_value(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
        in_tag_name: &FName,
        out_tag_value: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<185>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_GET_TAG_VALUE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_tag_name,
                __buffer.add(152).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_tag_value,
                __buffer.add(168).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_GET_TAG_VALUE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(168).cast::<FString>().swap(out_tag_value);
        }
        unsafe { __buffer.add(184).cast::<bool>().read() }
    }
    pub fn get_full_name(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<168>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_GET_FULL_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_GET_FULL_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(152).cast::<FString>().read() }
    }
    pub fn get_export_text_name(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<168>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_GET_EXPORT_TEXT_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_GET_EXPORT_TEXT_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(152).cast::<FString>().read() }
    }
    pub fn get_derived_class_asset_data(
        in_base_classes: &TArray<crate::bindings::core_u_object::FTopLevelAssetPath>,
        out_derived_class_asset_data: &mut TArray<
            crate::bindings::core_u_object::FAssetData,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_GET_DERIVED_CLASS_ASSET_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_base_classes,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<crate::bindings::core_u_object::FTopLevelAssetPath>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_derived_class_asset_data,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_GET_DERIVED_CLASS_ASSET_DATA,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .swap(out_derived_class_asset_data);
        }
    }
    pub fn get_class(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_GET_CLASS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_GET_CLASS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(152)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_blueprint_assets(
        in_filter: &crate::bindings::core_u_object::FARFilter,
        out_asset_data: &mut TArray<crate::bindings::core_u_object::FAssetData>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<368>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_GET_BLUEPRINT_ASSETS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_filter,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FARFilter>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_asset_data,
                __buffer
                    .add(352)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_GET_BLUEPRINT_ASSETS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(352)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .swap(out_asset_data);
        }
    }
    pub fn get_asset_registry() -> TScriptInterface<UAssetRegistry> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_GET_ASSET_REGISTRY,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_GET_ASSET_REGISTRY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TScriptInterface<UAssetRegistry>>().read() }
    }
    pub fn get_asset(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_GET_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_GET_ASSET,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(152)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn find_asset_native_class(
        asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_FIND_ASSET_NATIVE_CLASS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_FIND_ASSET_NATIVE_CLASS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(152)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn create_asset_data(
        in_asset: UPtr<crate::bindings::core_u_object::UObject>,
        b_allow_blueprint_class: bool,
    ) -> crate::bindings::core_u_object::FAssetData {
        let mut __stack = crate::core_data::StackAlloc::<168>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_CREATE_ASSET_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_allow_blueprint_class,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_CREATE_ASSET_DATA,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FAssetData>().read()
        }
    }
    pub fn asset_has_editor_only_data(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<153>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_ASSET_HAS_EDITOR_ONLY_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HELPERS_ASSET_HAS_EDITOR_ONLY_DATA,
                __buffer,
            )
        };
        unsafe { __buffer.add(152).cast::<bool>().read() }
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
    pub fn wait_for_package(&mut self, package_name: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_WAIT_FOR_PACKAGE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &package_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_WAIT_FOR_PACKAGE,
                __buffer,
            )
        };
    }
    pub fn wait_for_completion(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_WAIT_FOR_COMPLETION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_WAIT_FOR_COMPLETION,
                __buffer,
            )
        };
    }
    pub fn use_filter_to_exclude_assets(
        &self,
        asset_data_list: &mut TArray<crate::bindings::core_u_object::FAssetData>,
        filter: &crate::bindings::core_u_object::FARFilter,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<368>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_USE_FILTER_TO_EXCLUDE_ASSETS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_data_list,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                filter,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FARFilter>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_USE_FILTER_TO_EXCLUDE_ASSETS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .swap(asset_data_list);
        }
    }
    pub fn search_all_assets(&mut self, b_synchronous_search: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_SEARCH_ALL_ASSETS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_synchronous_search,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_SEARCH_ALL_ASSETS,
                __buffer,
            )
        };
    }
    pub fn scan_paths_synchronous(
        &mut self,
        in_paths: &TArray<FString>,
        b_force_rescan: bool,
        b_ignore_deny_list_scan_filters: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_SCAN_PATHS_SYNCHRONOUS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_paths,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_rescan,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_ignore_deny_list_scan_filters,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_SCAN_PATHS_SYNCHRONOUS,
                __buffer,
            )
        };
    }
    pub fn scan_modified_asset_files(&mut self, in_file_paths: &TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_SCAN_MODIFIED_ASSET_FILES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_file_paths,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_SCAN_MODIFIED_ASSET_FILES,
                __buffer,
            )
        };
    }
    pub fn scan_files_synchronous(
        &mut self,
        in_file_paths: &TArray<FString>,
        b_force_rescan: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_SCAN_FILES_SYNCHRONOUS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_file_paths,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_rescan,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_SCAN_FILES_SYNCHRONOUS,
                __buffer,
            )
        };
    }
    pub fn run_assets_through_filter(
        &self,
        asset_data_list: &mut TArray<crate::bindings::core_u_object::FAssetData>,
        filter: &crate::bindings::core_u_object::FARFilter,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<368>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_RUN_ASSETS_THROUGH_FILTER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_data_list,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                filter,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FARFilter>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_RUN_ASSETS_THROUGH_FILTER,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .swap(asset_data_list);
        }
    }
    pub fn prioritize_search_path(&mut self, path_to_prioritize: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_PRIORITIZE_SEARCH_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &path_to_prioritize,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_PRIORITIZE_SEARCH_PATH,
                __buffer,
            )
        };
    }
    pub fn get_referencers(
        &self,
        package_name: FName,
        reference_options: &FAssetRegistryDependencyOptions,
        out_referencers: &mut TArray<FName>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_K2_GET_REFERENCERS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &package_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                reference_options,
                __buffer.add(12).cast::<FAssetRegistryDependencyOptions>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_referencers,
                __buffer.add(24).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_K2_GET_REFERENCERS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<TArray<FName>>().swap(out_referencers);
        }
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn get_dependencies(
        &self,
        package_name: FName,
        dependency_options: &FAssetRegistryDependencyOptions,
        out_dependencies: &mut TArray<FName>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_K2_GET_DEPENDENCIES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &package_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                dependency_options,
                __buffer.add(12).cast::<FAssetRegistryDependencyOptions>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_dependencies,
                __buffer.add(24).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_K2_GET_DEPENDENCIES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<TArray<FName>>().swap(out_dependencies);
        }
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn k2_get_asset_by_object_path(
        &self,
        object_path: &crate::bindings::core_u_object::FSoftObjectPath,
        b_include_only_on_disk_assets: bool,
        b_skip_ar_filtered_assets: bool,
    ) -> crate::bindings::core_u_object::FAssetData {
        let mut __stack = crate::core_data::StackAlloc::<200>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_K2_GET_ASSET_BY_OBJECT_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                object_path,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::core_u_object::FSoftObjectPath>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_only_on_disk_assets,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_skip_ar_filtered_assets,
                __buffer.add(41).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_K2_GET_ASSET_BY_OBJECT_PATH,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(48).cast::<crate::bindings::core_u_object::FAssetData>().read()
        }
    }
    pub fn is_search_async(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_IS_SEARCH_ASYNC,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_IS_SEARCH_ASYNC,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_search_all_assets(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_IS_SEARCH_ALL_ASSETS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_IS_SEARCH_ALL_ASSETS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_loading_assets(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_IS_LOADING_ASSETS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_IS_LOADING_ASSETS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_assets(&self, package_path: FName, b_recursive: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HAS_ASSETS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &package_path,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_HAS_ASSETS,
                __buffer,
            )
        };
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
    pub fn get_sub_paths(
        &self,
        in_base_path: FString,
        out_path_list: &mut TArray<FString>,
        b_in_recurse: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_SUB_PATHS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_base_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_path_list,
                __buffer.add(16).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_recurse,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_SUB_PATHS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<FString>>().swap(out_path_list);
        }
    }
    pub fn get_in_memory_assets(
        &self,
        filter: &crate::bindings::core_u_object::FARFilter,
        out_asset_data: &mut TArray<crate::bindings::core_u_object::FAssetData>,
        b_skip_ar_filtered_assets: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<370>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_IN_MEMORY_ASSETS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                filter,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FARFilter>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_asset_data,
                __buffer
                    .add(352)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_skip_ar_filtered_assets,
                __buffer.add(368).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_IN_MEMORY_ASSETS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(352)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .swap(out_asset_data);
        }
        unsafe { __buffer.add(369).cast::<bool>().read() }
    }
    pub fn get_derived_class_names(
        &self,
        class_names: &TArray<crate::bindings::core_u_object::FTopLevelAssetPath>,
        excluded_class_names: &TSet<crate::bindings::core_u_object::FTopLevelAssetPath>,
        out_derived_class_names: &mut TSet<
            crate::bindings::core_u_object::FTopLevelAssetPath,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<176>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_DERIVED_CLASS_NAMES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                class_names,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<crate::bindings::core_u_object::FTopLevelAssetPath>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                excluded_class_names,
                __buffer
                    .add(16)
                    .cast::<TSet<crate::bindings::core_u_object::FTopLevelAssetPath>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_derived_class_names,
                __buffer
                    .add(96)
                    .cast::<TSet<crate::bindings::core_u_object::FTopLevelAssetPath>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_DERIVED_CLASS_NAMES,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(96)
                .cast::<TSet<crate::bindings::core_u_object::FTopLevelAssetPath>>()
                .swap(out_derived_class_names);
        }
    }
    pub fn get_assets_by_paths(
        &self,
        package_paths: TArray<FName>,
        out_asset_data: &mut TArray<crate::bindings::core_u_object::FAssetData>,
        b_recursive: bool,
        b_include_only_on_disk_assets: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_ASSETS_BY_PATHS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &package_paths,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_asset_data,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_only_on_disk_assets,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_ASSETS_BY_PATHS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .swap(out_asset_data);
        }
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn get_assets_by_path(
        &self,
        package_path: FName,
        out_asset_data: &mut TArray<crate::bindings::core_u_object::FAssetData>,
        b_recursive: bool,
        b_include_only_on_disk_assets: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_ASSETS_BY_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &package_path,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_asset_data,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_only_on_disk_assets,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_ASSETS_BY_PATH,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .swap(out_asset_data);
        }
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn get_assets_by_package_name(
        &self,
        package_name: FName,
        out_asset_data: &mut TArray<crate::bindings::core_u_object::FAssetData>,
        b_include_only_on_disk_assets: bool,
        b_skip_ar_filtered_assets: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_ASSETS_BY_PACKAGE_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &package_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_asset_data,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_only_on_disk_assets,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_skip_ar_filtered_assets,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_ASSETS_BY_PACKAGE_NAME,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .swap(out_asset_data);
        }
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn get_assets_by_class(
        &self,
        class_path_name: crate::bindings::core_u_object::FTopLevelAssetPath,
        out_asset_data: &mut TArray<crate::bindings::core_u_object::FAssetData>,
        b_search_sub_classes: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<42>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_ASSETS_BY_CLASS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class_path_name,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::core_u_object::FTopLevelAssetPath>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_asset_data,
                __buffer
                    .add(24)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_search_sub_classes,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_ASSETS_BY_CLASS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .swap(out_asset_data);
        }
        unsafe { __buffer.add(41).cast::<bool>().read() }
    }
    pub fn get_assets(
        &self,
        filter: &crate::bindings::core_u_object::FARFilter,
        out_asset_data: &mut TArray<crate::bindings::core_u_object::FAssetData>,
        b_skip_ar_filtered_assets: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<370>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_ASSETS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                filter,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FARFilter>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_asset_data,
                __buffer
                    .add(352)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_skip_ar_filtered_assets,
                __buffer.add(368).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_ASSETS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(352)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .swap(out_asset_data);
        }
        unsafe { __buffer.add(369).cast::<bool>().read() }
    }
    pub fn get_asset_by_object_path(
        &self,
        object_path: FName,
        b_include_only_on_disk_assets: bool,
    ) -> crate::bindings::core_u_object::FAssetData {
        let mut __stack = crate::core_data::StackAlloc::<168>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_ASSET_BY_OBJECT_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_path,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_only_on_disk_assets,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_ASSET_BY_OBJECT_PATH,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FAssetData>().read()
        }
    }
    pub fn get_ancestor_class_names(
        &self,
        class_path_name: crate::bindings::core_u_object::FTopLevelAssetPath,
        out_ancestor_class_names: &mut TArray<
            crate::bindings::core_u_object::FTopLevelAssetPath,
        >,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_ANCESTOR_CLASS_NAMES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class_path_name,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::core_u_object::FTopLevelAssetPath>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_ancestor_class_names,
                __buffer
                    .add(24)
                    .cast::<
                        TArray<crate::bindings::core_u_object::FTopLevelAssetPath>,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_ANCESTOR_CLASS_NAMES,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::core_u_object::FTopLevelAssetPath>>()
                .swap(out_ancestor_class_names);
        }
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn get_all_cached_paths(&self, out_path_list: &mut TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_ALL_CACHED_PATHS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_path_list,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_ALL_CACHED_PATHS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FString>>().swap(out_path_list);
        }
    }
    pub fn get_all_assets(
        &self,
        out_asset_data: &mut TArray<crate::bindings::core_u_object::FAssetData>,
        b_include_only_on_disk_assets: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_ALL_ASSETS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_asset_data,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_only_on_disk_assets,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::U_ASSET_REGISTRY_GET_ALL_ASSETS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .swap(out_asset_data);
        }
        unsafe { __buffer.add(17).cast::<bool>().read() }
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
