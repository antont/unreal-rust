#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_asset_registry_helpers_to_soft_object_path: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_helpers_sorting_predicate_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_helpers_sort_by_predicate: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_helpers_sort_by_asset_name: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_helpers_set_filter_tags_and_values: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_helpers_is_valid: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_helpers_is_u_asset: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_helpers_is_redirector: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_helpers_is_asset_loaded: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_helpers_is_asset_cooked: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_helpers_get_tag_value: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_helpers_get_full_name: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_helpers_get_export_text_name: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_helpers_get_derived_class_asset_data: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_helpers_get_class: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_helpers_get_blueprint_assets: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_helpers_get_asset_registry: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_helpers_get_asset: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_helpers_find_asset_native_class: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_helpers_create_asset_data: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_helpers_asset_has_editor_only_data: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_wait_for_package: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_wait_for_completion: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_use_filter_to_exclude_assets: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_search_all_assets: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_scan_paths_synchronous: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_scan_modified_asset_files: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_scan_files_synchronous: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_run_assets_through_filter: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_prioritize_search_path: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_k2_get_referencers: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_k2_get_dependencies: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_k2_get_asset_by_object_path: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_is_search_async: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_is_search_all_assets: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_is_loading_assets: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_has_assets: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_get_sub_paths: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_get_in_memory_assets: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_get_derived_class_names: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_get_assets_by_paths: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_get_assets_by_path: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_get_assets_by_package_name: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_get_assets_by_class: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_get_assets: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_get_asset_by_object_path: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_get_ancestor_class_names: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_get_all_cached_paths: *mut crate::ffi::UFunctionOpague,
    pub u_asset_registry_get_all_assets: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_asset_registry_helpers_to_soft_object_path: std::ptr::null_mut(),
            u_asset_registry_helpers_sorting_predicate_delegate_signature: std::ptr::null_mut(),
            u_asset_registry_helpers_sort_by_predicate: std::ptr::null_mut(),
            u_asset_registry_helpers_sort_by_asset_name: std::ptr::null_mut(),
            u_asset_registry_helpers_set_filter_tags_and_values: std::ptr::null_mut(),
            u_asset_registry_helpers_is_valid: std::ptr::null_mut(),
            u_asset_registry_helpers_is_u_asset: std::ptr::null_mut(),
            u_asset_registry_helpers_is_redirector: std::ptr::null_mut(),
            u_asset_registry_helpers_is_asset_loaded: std::ptr::null_mut(),
            u_asset_registry_helpers_is_asset_cooked: std::ptr::null_mut(),
            u_asset_registry_helpers_get_tag_value: std::ptr::null_mut(),
            u_asset_registry_helpers_get_full_name: std::ptr::null_mut(),
            u_asset_registry_helpers_get_export_text_name: std::ptr::null_mut(),
            u_asset_registry_helpers_get_derived_class_asset_data: std::ptr::null_mut(),
            u_asset_registry_helpers_get_class: std::ptr::null_mut(),
            u_asset_registry_helpers_get_blueprint_assets: std::ptr::null_mut(),
            u_asset_registry_helpers_get_asset_registry: std::ptr::null_mut(),
            u_asset_registry_helpers_get_asset: std::ptr::null_mut(),
            u_asset_registry_helpers_find_asset_native_class: std::ptr::null_mut(),
            u_asset_registry_helpers_create_asset_data: std::ptr::null_mut(),
            u_asset_registry_helpers_asset_has_editor_only_data: std::ptr::null_mut(),
            u_asset_registry_wait_for_package: std::ptr::null_mut(),
            u_asset_registry_wait_for_completion: std::ptr::null_mut(),
            u_asset_registry_use_filter_to_exclude_assets: std::ptr::null_mut(),
            u_asset_registry_search_all_assets: std::ptr::null_mut(),
            u_asset_registry_scan_paths_synchronous: std::ptr::null_mut(),
            u_asset_registry_scan_modified_asset_files: std::ptr::null_mut(),
            u_asset_registry_scan_files_synchronous: std::ptr::null_mut(),
            u_asset_registry_run_assets_through_filter: std::ptr::null_mut(),
            u_asset_registry_prioritize_search_path: std::ptr::null_mut(),
            u_asset_registry_k2_get_referencers: std::ptr::null_mut(),
            u_asset_registry_k2_get_dependencies: std::ptr::null_mut(),
            u_asset_registry_k2_get_asset_by_object_path: std::ptr::null_mut(),
            u_asset_registry_is_search_async: std::ptr::null_mut(),
            u_asset_registry_is_search_all_assets: std::ptr::null_mut(),
            u_asset_registry_is_loading_assets: std::ptr::null_mut(),
            u_asset_registry_has_assets: std::ptr::null_mut(),
            u_asset_registry_get_sub_paths: std::ptr::null_mut(),
            u_asset_registry_get_in_memory_assets: std::ptr::null_mut(),
            u_asset_registry_get_derived_class_names: std::ptr::null_mut(),
            u_asset_registry_get_assets_by_paths: std::ptr::null_mut(),
            u_asset_registry_get_assets_by_path: std::ptr::null_mut(),
            u_asset_registry_get_assets_by_package_name: std::ptr::null_mut(),
            u_asset_registry_get_assets_by_class: std::ptr::null_mut(),
            u_asset_registry_get_assets: std::ptr::null_mut(),
            u_asset_registry_get_asset_by_object_path: std::ptr::null_mut(),
            u_asset_registry_get_ancestor_class_names: std::ptr::null_mut(),
            u_asset_registry_get_all_cached_paths: std::ptr::null_mut(),
            u_asset_registry_get_all_assets: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAssetRegistryHelpers::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ToSoftObjectPath"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_helpers_to_soft_object_path,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SortingPredicate__DelegateSignature"),
            &raw mut __FUNCTION_PTRS
                .u_asset_registry_helpers_sorting_predicate_delegate_signature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SortByPredicate"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_helpers_sort_by_predicate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SortByAssetName"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_helpers_sort_by_asset_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterTagsAndValues"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_helpers_set_filter_tags_and_values,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValid"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_helpers_is_valid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsUAsset"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_helpers_is_u_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRedirector"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_helpers_is_redirector,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAssetLoaded"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_helpers_is_asset_loaded,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAssetCooked"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_helpers_is_asset_cooked,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTagValue"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_helpers_get_tag_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFullName"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_helpers_get_full_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetExportTextName"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_helpers_get_export_text_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDerivedClassAssetData"),
            &raw mut __FUNCTION_PTRS
                .u_asset_registry_helpers_get_derived_class_asset_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetClass"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_helpers_get_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlueprintAssets"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_helpers_get_blueprint_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetRegistry"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_helpers_get_asset_registry,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAsset"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_helpers_get_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindAssetNativeClass"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_helpers_find_asset_native_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateAssetData"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_helpers_create_asset_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssetHasEditorOnlyData"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_helpers_asset_has_editor_only_data,
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
            &raw mut __FUNCTION_PTRS.u_asset_registry_wait_for_package,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WaitForCompletion"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_wait_for_completion,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UseFilterToExcludeAssets"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_use_filter_to_exclude_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SearchAllAssets"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_search_all_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScanPathsSynchronous"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_scan_paths_synchronous,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScanModifiedAssetFiles"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_scan_modified_asset_files,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScanFilesSynchronous"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_scan_files_synchronous,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RunAssetsThroughFilter"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_run_assets_through_filter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PrioritizeSearchPath"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_prioritize_search_path,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetReferencers"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_k2_get_referencers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetDependencies"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_k2_get_dependencies,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetAssetByObjectPath"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_k2_get_asset_by_object_path,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsSearchAsync"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_is_search_async,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsSearchAllAssets"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_is_search_all_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLoadingAssets"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_is_loading_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasAssets"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_has_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSubPaths"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_get_sub_paths,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInMemoryAssets"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_get_in_memory_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDerivedClassNames"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_get_derived_class_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetsByPaths"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_get_assets_by_paths,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetsByPath"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_get_assets_by_path,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetsByPackageName"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_get_assets_by_package_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetsByClass"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_get_assets_by_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssets"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_get_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetByObjectPath"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_get_asset_by_object_path,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAncestorClassNames"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_get_ancestor_class_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllCachedPaths"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_get_all_cached_paths,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllAssets"),
            &raw mut __FUNCTION_PTRS.u_asset_registry_get_all_assets,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_to_soft_object_path,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_to_soft_object_path,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_sort_by_predicate,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_sort_by_predicate,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_sort_by_asset_name,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_sort_by_asset_name,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_set_filter_tags_and_values,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_set_filter_tags_and_values,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_is_valid,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_is_valid,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_is_u_asset,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_is_u_asset,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_is_redirector,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_is_redirector,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_is_asset_loaded,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_is_asset_loaded,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_is_asset_cooked,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_is_asset_cooked,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_get_tag_value,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_get_tag_value,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_get_full_name,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_get_full_name,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_get_export_text_name,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_get_export_text_name,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_get_derived_class_asset_data,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_get_derived_class_asset_data,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_get_class,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_get_class,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_get_blueprint_assets,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_get_blueprint_assets,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_get_asset_registry,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::asset_registry::UAssetRegistryHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_get_asset_registry,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_get_asset,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_get_asset,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_find_asset_native_class,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_find_asset_native_class,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_create_asset_data,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_create_asset_data,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_asset_has_editor_only_data,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_helpers_asset_has_editor_only_data,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_wait_for_package,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_wait_for_package,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_wait_for_completion,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_wait_for_completion,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_use_filter_to_exclude_assets,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_use_filter_to_exclude_assets,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_search_all_assets,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_search_all_assets,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_scan_paths_synchronous,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_scan_paths_synchronous,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_scan_modified_asset_files,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_scan_modified_asset_files,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_scan_files_synchronous,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_scan_files_synchronous,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_run_assets_through_filter,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_run_assets_through_filter,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_prioritize_search_path,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_prioritize_search_path,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_k2_get_referencers,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_k2_get_referencers,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_k2_get_dependencies,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_k2_get_dependencies,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_k2_get_asset_by_object_path,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_k2_get_asset_by_object_path,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_is_search_async,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_is_search_async,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_is_search_all_assets,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_is_search_all_assets,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_is_loading_assets,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_is_loading_assets,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_has_assets,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_has_assets,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_sub_paths,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_sub_paths,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_in_memory_assets,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_in_memory_assets,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_derived_class_names,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_derived_class_names,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_assets_by_paths,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_assets_by_paths,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_assets_by_path,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_assets_by_path,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_assets_by_package_name,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_assets_by_package_name,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_assets_by_class,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_assets_by_class,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_assets,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_assets,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_asset_by_object_path,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_asset_by_object_path,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_ancestor_class_names,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_ancestor_class_names,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_all_cached_paths,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_all_cached_paths,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_all_assets,
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
                crate::bindings::asset_registry::__FUNCTION_PTRS
                    .u_asset_registry_get_all_assets,
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
