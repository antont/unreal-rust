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
    pub fn reparent_collection(&mut self, name: FName, new_parent_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_REPARENT_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_parent_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_REPARENT_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn rename_collection(&mut self, name: FName, new_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_RENAME_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_RENAME_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn remove_assets_from_collection(
        &mut self,
        name: FName,
        asset_path_names: &TArray<FName>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSETS_FROM_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_path_names,
                __buffer.add(16).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSETS_FROM_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn remove_asset_ptrs_from_collection(
        &mut self,
        name: FName,
        asset_ptrs: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSET_PTRS_FROM_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_ptrs,
                __buffer
                    .add(16)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSET_PTRS_FROM_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn remove_asset_ptr_from_collection(
        &mut self,
        name: FName,
        asset_ptr: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSET_PTR_FROM_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_ptr,
                __buffer.add(16).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSET_PTR_FROM_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn remove_asset_from_collection(
        &mut self,
        name: FName,
        asset_path_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSET_FROM_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSET_FROM_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn remove_asset_datas_from_collection(
        &mut self,
        name: FName,
        asset_datas: &TArray<crate::bindings::core_u_object::FAssetData>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSET_DATAS_FROM_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_datas,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSET_DATAS_FROM_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn remove_asset_data_from_collection(
        &mut self,
        name: FName,
        asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<169>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSET_DATA_FROM_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_data,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_REMOVE_ASSET_DATA_FROM_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(168).cast::<bool>().read() }
    }
    pub fn k2_remove_assets_from_collection(
        &mut self,
        name: FName,
        asset_paths: &TArray<crate::bindings::core_u_object::FSoftObjectPath>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_K2_REMOVE_ASSETS_FROM_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_paths,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FSoftObjectPath>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_K2_REMOVE_ASSETS_FROM_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn k2_remove_asset_from_collection(
        &mut self,
        name: FName,
        asset_path: &crate::bindings::core_u_object::FSoftObjectPath,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_K2_REMOVE_ASSET_FROM_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_path,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::core_u_object::FSoftObjectPath>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_K2_REMOVE_ASSET_FROM_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn k2_get_collections_containing_asset(
        &mut self,
        asset_path: &crate::bindings::core_u_object::FSoftObjectPath,
    ) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_K2_GET_COLLECTIONS_CONTAINING_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_path,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::core_u_object::FSoftObjectPath>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_K2_GET_COLLECTIONS_CONTAINING_ASSET,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<TArray<FName>>().read() }
    }
    pub fn k2_add_asset_to_collection(
        &mut self,
        name: FName,
        asset_path: &crate::bindings::core_u_object::FSoftObjectPath,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_K2_ADD_ASSET_TO_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_path,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::core_u_object::FSoftObjectPath>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_K2_ADD_ASSET_TO_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn k2_add_assets_to_collection(
        &mut self,
        name: FName,
        asset_paths: &TArray<crate::bindings::core_u_object::FSoftObjectPath>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_K2_ADD_ASSETS_TO_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_paths,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FSoftObjectPath>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_K2_ADD_ASSETS_TO_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_collections_containing_asset_ptr(
        &mut self,
        asset_ptr: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_GET_COLLECTIONS_CONTAINING_ASSET_PTR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_ptr,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_GET_COLLECTIONS_CONTAINING_ASSET_PTR,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<FName>>().read() }
    }
    pub fn get_collections_containing_asset_data(
        &mut self,
        asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<168>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_GET_COLLECTIONS_CONTAINING_ASSET_DATA,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_GET_COLLECTIONS_CONTAINING_ASSET_DATA,
                __buffer,
            )
        };
        unsafe { __buffer.add(152).cast::<TArray<FName>>().read() }
    }
    pub fn get_collections_containing_asset(
        &mut self,
        asset_path_name: FName,
    ) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_GET_COLLECTIONS_CONTAINING_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_GET_COLLECTIONS_CONTAINING_ASSET,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TArray<FName>>().read() }
    }
    pub fn get_collections(&mut self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_GET_COLLECTIONS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_GET_COLLECTIONS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn get_assets_in_collection(
        &mut self,
        name: FName,
    ) -> TArray<crate::bindings::core_u_object::FAssetData> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_GET_ASSETS_IN_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_GET_ASSETS_IN_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .read()
        }
    }
    pub fn empty_collection(&mut self, name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_EMPTY_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_EMPTY_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn destroy_collection(&mut self, name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_DESTROY_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_DESTROY_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn create_collection(
        &mut self,
        name: FName,
        share_type: crate::bindings::engine::ECollectionScriptingShareType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_CREATE_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &share_type,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::engine::ECollectionScriptingShareType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_CREATE_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
    pub fn collection_exists(&mut self, name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_COLLECTION_EXISTS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_COLLECTION_EXISTS,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn add_asset_to_collection(
        &mut self,
        name: FName,
        asset_path_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_ADD_ASSET_TO_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_ADD_ASSET_TO_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn add_assets_to_collection(
        &mut self,
        name: FName,
        asset_path_names: &TArray<FName>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_ADD_ASSETS_TO_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_path_names,
                __buffer.add(16).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_ADD_ASSETS_TO_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn add_asset_ptr_to_collection(
        &mut self,
        name: FName,
        asset_ptr: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_ADD_ASSET_PTR_TO_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_ptr,
                __buffer.add(16).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_ADD_ASSET_PTR_TO_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn add_asset_ptrs_to_collection(
        &mut self,
        name: FName,
        asset_ptrs: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_ADD_ASSET_PTRS_TO_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_ptrs,
                __buffer
                    .add(16)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_ADD_ASSET_PTRS_TO_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn add_asset_data_to_collection(
        &mut self,
        name: FName,
        asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<169>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_ADD_ASSET_DATA_TO_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_data,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_ADD_ASSET_DATA_TO_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(168).cast::<bool>().read() }
    }
    pub fn add_asset_datas_to_collection(
        &mut self,
        name: FName,
        asset_datas: &TArray<crate::bindings::core_u_object::FAssetData>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_ADD_ASSET_DATAS_TO_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_datas,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::U_ASSET_TAGS_SUBSYSTEM_ADD_ASSET_DATAS_TO_COLLECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
}
