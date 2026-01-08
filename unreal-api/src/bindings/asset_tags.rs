#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_asset_tags_subsystem_reparent_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_rename_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_remove_assets_from_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_remove_asset_ptrs_from_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_remove_asset_ptr_from_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_remove_asset_from_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_remove_asset_datas_from_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_remove_asset_data_from_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_k2_remove_assets_from_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_k2_remove_asset_from_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_k2_get_collections_containing_asset: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_k2_add_asset_to_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_k2_add_assets_to_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_get_collections_containing_asset_ptr: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_get_collections_containing_asset_data: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_get_collections_containing_asset: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_get_collections: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_get_assets_in_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_empty_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_destroy_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_create_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_collection_exists: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_add_asset_to_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_add_assets_to_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_add_asset_ptr_to_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_add_asset_ptrs_to_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_add_asset_data_to_collection: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tags_subsystem_add_asset_datas_to_collection: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_asset_tags_subsystem_reparent_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_rename_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_remove_assets_from_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_remove_asset_ptrs_from_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_remove_asset_ptr_from_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_remove_asset_from_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_remove_asset_datas_from_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_remove_asset_data_from_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_k2_remove_assets_from_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_k2_remove_asset_from_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_k2_get_collections_containing_asset: std::ptr::null_mut(),
            u_asset_tags_subsystem_k2_add_asset_to_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_k2_add_assets_to_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_get_collections_containing_asset_ptr: std::ptr::null_mut(),
            u_asset_tags_subsystem_get_collections_containing_asset_data: std::ptr::null_mut(),
            u_asset_tags_subsystem_get_collections_containing_asset: std::ptr::null_mut(),
            u_asset_tags_subsystem_get_collections: std::ptr::null_mut(),
            u_asset_tags_subsystem_get_assets_in_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_empty_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_destroy_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_create_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_collection_exists: std::ptr::null_mut(),
            u_asset_tags_subsystem_add_asset_to_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_add_assets_to_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_add_asset_ptr_to_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_add_asset_ptrs_to_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_add_asset_data_to_collection: std::ptr::null_mut(),
            u_asset_tags_subsystem_add_asset_datas_to_collection: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAssetTagsSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReparentCollection"),
            &raw mut __FUNCTION_PTRS.u_asset_tags_subsystem_reparent_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameCollection"),
            &raw mut __FUNCTION_PTRS.u_asset_tags_subsystem_rename_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAssetsFromCollection"),
            &raw mut __FUNCTION_PTRS.u_asset_tags_subsystem_remove_assets_from_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAssetPtrsFromCollection"),
            &raw mut __FUNCTION_PTRS
                .u_asset_tags_subsystem_remove_asset_ptrs_from_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAssetPtrFromCollection"),
            &raw mut __FUNCTION_PTRS
                .u_asset_tags_subsystem_remove_asset_ptr_from_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAssetFromCollection"),
            &raw mut __FUNCTION_PTRS.u_asset_tags_subsystem_remove_asset_from_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAssetDatasFromCollection"),
            &raw mut __FUNCTION_PTRS
                .u_asset_tags_subsystem_remove_asset_datas_from_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAssetDataFromCollection"),
            &raw mut __FUNCTION_PTRS
                .u_asset_tags_subsystem_remove_asset_data_from_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_RemoveAssetsFromCollection"),
            &raw mut __FUNCTION_PTRS
                .u_asset_tags_subsystem_k2_remove_assets_from_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_RemoveAssetFromCollection"),
            &raw mut __FUNCTION_PTRS
                .u_asset_tags_subsystem_k2_remove_asset_from_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetCollectionsContainingAsset"),
            &raw mut __FUNCTION_PTRS
                .u_asset_tags_subsystem_k2_get_collections_containing_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_AddAssetToCollection"),
            &raw mut __FUNCTION_PTRS.u_asset_tags_subsystem_k2_add_asset_to_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_AddAssetsToCollection"),
            &raw mut __FUNCTION_PTRS.u_asset_tags_subsystem_k2_add_assets_to_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCollectionsContainingAssetPtr"),
            &raw mut __FUNCTION_PTRS
                .u_asset_tags_subsystem_get_collections_containing_asset_ptr,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCollectionsContainingAssetData"),
            &raw mut __FUNCTION_PTRS
                .u_asset_tags_subsystem_get_collections_containing_asset_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCollectionsContainingAsset"),
            &raw mut __FUNCTION_PTRS
                .u_asset_tags_subsystem_get_collections_containing_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCollections"),
            &raw mut __FUNCTION_PTRS.u_asset_tags_subsystem_get_collections,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetsInCollection"),
            &raw mut __FUNCTION_PTRS.u_asset_tags_subsystem_get_assets_in_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EmptyCollection"),
            &raw mut __FUNCTION_PTRS.u_asset_tags_subsystem_empty_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DestroyCollection"),
            &raw mut __FUNCTION_PTRS.u_asset_tags_subsystem_destroy_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateCollection"),
            &raw mut __FUNCTION_PTRS.u_asset_tags_subsystem_create_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CollectionExists"),
            &raw mut __FUNCTION_PTRS.u_asset_tags_subsystem_collection_exists,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAssetToCollection"),
            &raw mut __FUNCTION_PTRS.u_asset_tags_subsystem_add_asset_to_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAssetsToCollection"),
            &raw mut __FUNCTION_PTRS.u_asset_tags_subsystem_add_assets_to_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAssetPtrToCollection"),
            &raw mut __FUNCTION_PTRS.u_asset_tags_subsystem_add_asset_ptr_to_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAssetPtrsToCollection"),
            &raw mut __FUNCTION_PTRS.u_asset_tags_subsystem_add_asset_ptrs_to_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAssetDataToCollection"),
            &raw mut __FUNCTION_PTRS.u_asset_tags_subsystem_add_asset_data_to_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAssetDatasToCollection"),
            &raw mut __FUNCTION_PTRS.u_asset_tags_subsystem_add_asset_datas_to_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_reparent_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_reparent_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_rename_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_rename_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_remove_assets_from_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_remove_assets_from_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_remove_asset_ptrs_from_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_remove_asset_ptrs_from_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_remove_asset_ptr_from_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_remove_asset_ptr_from_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_remove_asset_from_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_remove_asset_from_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_remove_asset_datas_from_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_remove_asset_datas_from_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_remove_asset_data_from_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_remove_asset_data_from_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_k2_remove_assets_from_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_k2_remove_assets_from_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_k2_remove_asset_from_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_k2_remove_asset_from_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_k2_get_collections_containing_asset,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_k2_get_collections_containing_asset,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_k2_add_asset_to_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_k2_add_asset_to_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_k2_add_assets_to_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_k2_add_assets_to_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_get_collections_containing_asset_ptr,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_get_collections_containing_asset_ptr,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_get_collections_containing_asset_data,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_get_collections_containing_asset_data,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_get_collections_containing_asset,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_get_collections_containing_asset,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_get_collections,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_get_collections,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_get_assets_in_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_get_assets_in_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_empty_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_empty_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_destroy_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_destroy_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_create_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_create_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_collection_exists,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_collection_exists,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_add_asset_to_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_add_asset_to_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_add_assets_to_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_add_assets_to_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_add_asset_ptr_to_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_add_asset_ptr_to_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_add_asset_ptrs_to_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_add_asset_ptrs_to_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_add_asset_data_to_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_add_asset_data_to_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_add_asset_datas_to_collection,
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
                crate::bindings::asset_tags::__FUNCTION_PTRS
                    .u_asset_tags_subsystem_add_asset_datas_to_collection,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
}
