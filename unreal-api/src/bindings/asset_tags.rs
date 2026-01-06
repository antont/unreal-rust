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
pub static mut U_ASSET_TAGS_SUBSYSTEM_REPARENT_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_RENAME_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSETS_FROM_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSET_PTRS_FROM_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSET_PTR_FROM_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSET_FROM_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSET_DATAS_FROM_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSET_DATA_FROM_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_K2_REMOVE_ASSETS_FROM_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_K2_REMOVE_ASSET_FROM_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_K2_GET_COLLECTIONS_CONTAINING_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_K2_ADD_ASSET_TO_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_K2_ADD_ASSETS_TO_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_GET_COLLECTIONS_CONTAINING_ASSET_PTR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_GET_COLLECTIONS_CONTAINING_ASSET_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_GET_COLLECTIONS_CONTAINING_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_GET_COLLECTIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_GET_ASSETS_IN_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_EMPTY_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_DESTROY_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_CREATE_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_COLLECTION_EXISTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_ADD_ASSET_TO_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_ADD_ASSETS_TO_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_ADD_ASSET_PTR_TO_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_ADD_ASSET_PTRS_TO_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_ADD_ASSET_DATA_TO_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TAGS_SUBSYSTEM_ADD_ASSET_DATAS_TO_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAssetTagsSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReparentCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_REPARENT_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_RENAME_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAssetsFromCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSETS_FROM_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAssetPtrsFromCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSET_PTRS_FROM_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAssetPtrFromCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSET_PTR_FROM_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAssetFromCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSET_FROM_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAssetDatasFromCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSET_DATAS_FROM_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAssetDataFromCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSET_DATA_FROM_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_RemoveAssetsFromCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_K2_REMOVE_ASSETS_FROM_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_RemoveAssetFromCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_K2_REMOVE_ASSET_FROM_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetCollectionsContainingAsset"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_K2_GET_COLLECTIONS_CONTAINING_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_AddAssetToCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_K2_ADD_ASSET_TO_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_AddAssetsToCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_K2_ADD_ASSETS_TO_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCollectionsContainingAssetPtr"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_GET_COLLECTIONS_CONTAINING_ASSET_PTR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCollectionsContainingAssetData"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_GET_COLLECTIONS_CONTAINING_ASSET_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCollectionsContainingAsset"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_GET_COLLECTIONS_CONTAINING_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCollections"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_GET_COLLECTIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetsInCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_GET_ASSETS_IN_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EmptyCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_EMPTY_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DestroyCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_DESTROY_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_CREATE_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CollectionExists"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_COLLECTION_EXISTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAssetToCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_ADD_ASSET_TO_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAssetsToCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_ADD_ASSETS_TO_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAssetPtrToCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_ADD_ASSET_PTR_TO_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAssetPtrsToCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_ADD_ASSET_PTRS_TO_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAssetDataToCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_ADD_ASSET_DATA_TO_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAssetDatasToCollection"),
            &raw mut U_ASSET_TAGS_SUBSYSTEM_ADD_ASSET_DATAS_TO_COLLECTION,
        );
    }
}
#[repr(C, align(8))]
pub struct UAssetTagsSubsystem {
    __padding_end: [u8; 56],
}
impl UAssetTagsSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetTagsSubsystem")
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
