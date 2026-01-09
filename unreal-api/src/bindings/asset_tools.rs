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
    pub u_asset_tools_rename_referencing_soft_object_paths: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_rename_assets_with_dialog: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_rename_assets: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_open_editor_for_assets: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_migrate_packages: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_is_asset_read_only: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_import_asset_tasks: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_import_assets_with_dialog: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_import_assets_automated: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_find_soft_references_to_object: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_export_assets_with_dialog: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_export_assets: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_duplicate_asset_with_dialog_and_title: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_duplicate_asset_with_dialog: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_duplicate_asset: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_diff_assets: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_diff_against_depot: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_create_unique_asset_name: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_create_asset_with_dialog_async: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_create_asset_with_dialog: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_create_asset_async: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_create_asset: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_begin_advanced_copy_packages: *mut crate::ffi::UFunctionOpague,
    pub u_asset_tools_helpers_get_asset_tools: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_asset_tools_rename_referencing_soft_object_paths: std::ptr::null_mut(),
            u_asset_tools_rename_assets_with_dialog: std::ptr::null_mut(),
            u_asset_tools_rename_assets: std::ptr::null_mut(),
            u_asset_tools_open_editor_for_assets: std::ptr::null_mut(),
            u_asset_tools_migrate_packages: std::ptr::null_mut(),
            u_asset_tools_is_asset_read_only: std::ptr::null_mut(),
            u_asset_tools_import_asset_tasks: std::ptr::null_mut(),
            u_asset_tools_import_assets_with_dialog: std::ptr::null_mut(),
            u_asset_tools_import_assets_automated: std::ptr::null_mut(),
            u_asset_tools_find_soft_references_to_object: std::ptr::null_mut(),
            u_asset_tools_export_assets_with_dialog: std::ptr::null_mut(),
            u_asset_tools_export_assets: std::ptr::null_mut(),
            u_asset_tools_duplicate_asset_with_dialog_and_title: std::ptr::null_mut(),
            u_asset_tools_duplicate_asset_with_dialog: std::ptr::null_mut(),
            u_asset_tools_duplicate_asset: std::ptr::null_mut(),
            u_asset_tools_diff_assets: std::ptr::null_mut(),
            u_asset_tools_diff_against_depot: std::ptr::null_mut(),
            u_asset_tools_create_unique_asset_name: std::ptr::null_mut(),
            u_asset_tools_create_asset_with_dialog_async: std::ptr::null_mut(),
            u_asset_tools_create_asset_with_dialog: std::ptr::null_mut(),
            u_asset_tools_create_asset_async: std::ptr::null_mut(),
            u_asset_tools_create_asset: std::ptr::null_mut(),
            u_asset_tools_begin_advanced_copy_packages: std::ptr::null_mut(),
            u_asset_tools_helpers_get_asset_tools: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAssetTools::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameReferencingSoftObjectPaths"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_rename_referencing_soft_object_paths,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameAssetsWithDialog"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_rename_assets_with_dialog,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameAssets"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_rename_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OpenEditorForAssets"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_open_editor_for_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MigratePackages"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_migrate_packages,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAssetReadOnly"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_is_asset_read_only,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ImportAssetTasks"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_import_asset_tasks,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ImportAssetsWithDialog"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_import_assets_with_dialog,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ImportAssetsAutomated"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_import_assets_automated,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindSoftReferencesToObject"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_find_soft_references_to_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportAssetsWithDialog"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_export_assets_with_dialog,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportAssets"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_export_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateAssetWithDialogAndTitle"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_duplicate_asset_with_dialog_and_title,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateAssetWithDialog"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_duplicate_asset_with_dialog,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateAsset"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_duplicate_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DiffAssets"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_diff_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DiffAgainstDepot"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_diff_against_depot,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateUniqueAssetName"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_create_unique_asset_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateAssetWithDialogAsync"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_create_asset_with_dialog_async,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateAssetWithDialog"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_create_asset_with_dialog,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateAssetAsync"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_create_asset_async,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateAsset"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_create_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BeginAdvancedCopyPackages"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_begin_advanced_copy_packages,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAssetToolsHelpers::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetTools"),
            &raw mut __FUNCTION_PTRS.u_asset_tools_helpers_get_asset_tools,
        );
    }
}
#[repr(C, align(8))]
pub struct FAssetRenameData {
    pub asset: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
    pub new_package_path: FString,
    pub new_name: FString,
    pub(crate) __padding_end: [u8; 88],
}
impl FAssetRenameData {}
#[repr(C, align(8))]
pub struct FMigrationOptions {
    pub b_prompt: bool,
    pub b_ignore_dependencies: bool,
    pub asset_conflict: EAssetMigrationConflict,
    pub orphan_folder: FString,
}
impl FMigrationOptions {}
#[repr(C, align(8))]
pub struct UAssetToolsSettings {
    __padding_end: [u8; 120],
}
impl UAssetToolsSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetToolsSettings")
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
pub struct IAssetTools {}
#[repr(C, align(8))]
pub struct UAssetTools {
    __padding_end: [u8; 48],
}
impl UAssetTools {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetTools")
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
    pub fn rename_referencing_soft_object_paths(
        &mut self,
        packages_to_check: TArray<UPtr<crate::bindings::core_u_object::UPackage>>,
        asset_redirector_map: &TMap<
            crate::bindings::core_u_object::FSoftObjectPath,
            crate::bindings::core_u_object::FSoftObjectPath,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_rename_referencing_soft_object_paths,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &packages_to_check,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UPackage>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_redirector_map,
                __buffer
                    .add(16)
                    .cast::<
                        TMap<
                            crate::bindings::core_u_object::FSoftObjectPath,
                            crate::bindings::core_u_object::FSoftObjectPath,
                        >,
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
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_rename_referencing_soft_object_paths,
                __buffer,
            )
        };
    }
    pub fn rename_assets_with_dialog(
        &mut self,
        assets_and_names: &TArray<FAssetRenameData>,
        b_auto_checkout: bool,
    ) -> EAssetRenameResult {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_rename_assets_with_dialog,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                assets_and_names,
                __buffer.add(0).cast::<TArray<FAssetRenameData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_auto_checkout,
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
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_rename_assets_with_dialog,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<EAssetRenameResult>().read() }
    }
    pub fn rename_assets(
        &mut self,
        assets_and_names: &TArray<FAssetRenameData>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_rename_assets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                assets_and_names,
                __buffer.add(0).cast::<TArray<FAssetRenameData>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_rename_assets,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn open_editor_for_assets(
        &mut self,
        assets: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_open_editor_for_assets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                assets,
                __buffer
                    .add(0)
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
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_open_editor_for_assets,
                __buffer,
            )
        };
    }
    pub fn migrate_packages(
        &self,
        package_names_to_migrate: &TArray<FName>,
        destination_path: FString,
        options: &FMigrationOptions,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_migrate_packages,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                package_names_to_migrate,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer.add(32).cast::<FMigrationOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_migrate_packages,
                __buffer,
            )
        };
    }
    pub fn is_asset_read_only(
        &self,
        asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<153>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_is_asset_read_only,
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
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_is_asset_read_only,
                __buffer,
            )
        };
        unsafe { __buffer.add(152).cast::<bool>().read() }
    }
    pub fn import_asset_tasks(
        &mut self,
        import_tasks: &TArray<UPtr<crate::bindings::unreal_ed::UAssetImportTask>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_import_asset_tasks,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                import_tasks,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<UPtr<crate::bindings::unreal_ed::UAssetImportTask>>,
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
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_import_asset_tasks,
                __buffer,
            )
        };
    }
    pub fn import_assets_with_dialog(
        &mut self,
        destination_path: FString,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_import_assets_with_dialog,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_path,
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
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_import_assets_with_dialog,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
    pub fn import_assets_automated(
        &mut self,
        import_data: UPtr<crate::bindings::unreal_ed::UAutomatedAssetImportData>,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_import_assets_automated,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &import_data,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::unreal_ed::UAutomatedAssetImportData>,
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
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_import_assets_automated,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
    pub fn find_soft_references_to_object(
        &mut self,
        target_object: crate::bindings::core_u_object::FSoftObjectPath,
        referencing_objects: &mut TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_find_soft_references_to_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_object,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::core_u_object::FSoftObjectPath>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                referencing_objects,
                __buffer
                    .add(40)
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
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_find_soft_references_to_object,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .swap(referencing_objects);
        }
    }
    pub fn export_assets_with_dialog(
        &mut self,
        assets_to_export: &TArray<FString>,
        b_prompt_for_individual_filenames: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_export_assets_with_dialog,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                assets_to_export,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_prompt_for_individual_filenames,
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
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_export_assets_with_dialog,
                __buffer,
            )
        };
    }
    pub fn export_assets(
        &mut self,
        assets_to_export: &TArray<FString>,
        export_path: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_export_assets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                assets_to_export,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &export_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_export_assets,
                __buffer,
            )
        };
    }
    pub fn duplicate_asset_with_dialog_and_title(
        &mut self,
        asset_name: FString,
        package_path: FString,
        original_object: UPtr<crate::bindings::core_u_object::UObject>,
        dialog_title: FText,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_duplicate_asset_with_dialog_and_title,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &package_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &original_object,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &dialog_title,
                __buffer.add(40).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_duplicate_asset_with_dialog_and_title,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn duplicate_asset_with_dialog(
        &mut self,
        asset_name: FString,
        package_path: FString,
        original_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_duplicate_asset_with_dialog,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &package_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &original_object,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_duplicate_asset_with_dialog,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn duplicate_asset(
        &mut self,
        asset_name: FString,
        package_path: FString,
        original_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_duplicate_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &package_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &original_object,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_duplicate_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn diff_assets(
        &self,
        old_asset: UPtr<crate::bindings::core_u_object::UObject>,
        new_asset: UPtr<crate::bindings::core_u_object::UObject>,
        old_revision: &crate::bindings::asset_definition::FRevisionInfo,
        new_revision: &crate::bindings::asset_definition::FRevisionInfo,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS.u_asset_tools_diff_assets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &old_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_asset,
                __buffer.add(8).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                old_revision,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::asset_definition::FRevisionInfo>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_revision,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::asset_definition::FRevisionInfo>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tools::__FUNCTION_PTRS.u_asset_tools_diff_assets,
                __buffer,
            )
        };
    }
    pub fn diff_against_depot(
        &self,
        in_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_package_path: FString,
        in_package_name: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_diff_against_depot,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_package_path,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_package_name,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_diff_against_depot,
                __buffer,
            )
        };
    }
    pub fn create_unique_asset_name(
        &mut self,
        in_base_package_name: FString,
        in_suffix: FString,
        out_package_name: &mut FString,
        out_asset_name: &mut FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_create_unique_asset_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_base_package_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_suffix,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_package_name,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_asset_name,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_create_unique_asset_name,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<FString>().swap(out_package_name);
        }
        unsafe {
            __buffer.add(48).cast::<FString>().swap(out_asset_name);
        }
    }
    pub fn create_asset_with_dialog_async(
        &mut self,
        asset_name: FString,
        package_path: FString,
        asset_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        factory: UPtr<crate::bindings::unreal_ed::UFactory>,
        on_complete: FCreateAssetWithDialogAsync_OnComplete,
        on_cancelled: FCreateAssetWithDialogAsync_OnCancelled,
        calling_context: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<125>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_create_asset_with_dialog_async,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &package_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_class,
                __buffer
                    .add(32)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &factory,
                __buffer.add(40).cast::<UPtr<crate::bindings::unreal_ed::UFactory>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_complete,
                __buffer.add(48).cast::<FCreateAssetWithDialogAsync_OnComplete>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_cancelled,
                __buffer.add(80).cast::<FCreateAssetWithDialogAsync_OnCancelled>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &calling_context,
                __buffer.add(112).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_create_asset_with_dialog_async,
                __buffer,
            )
        };
        unsafe { __buffer.add(124).cast::<bool>().read() }
    }
    pub fn create_asset_with_dialog(
        &mut self,
        asset_name: FString,
        package_path: FString,
        asset_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        factory: UPtr<crate::bindings::unreal_ed::UFactory>,
        calling_context: FName,
        b_call_configure_properties: bool,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_create_asset_with_dialog,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &package_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_class,
                __buffer
                    .add(32)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &factory,
                __buffer.add(40).cast::<UPtr<crate::bindings::unreal_ed::UFactory>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &calling_context,
                __buffer.add(48).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_call_configure_properties,
                __buffer.add(60).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_create_asset_with_dialog,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(64)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn create_asset_async(
        &mut self,
        asset_name: FString,
        package_path: FString,
        asset_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        factory: UPtr<crate::bindings::unreal_ed::UFactory>,
        on_complete: FCreateAssetAsync_OnComplete,
        on_cancelled: FCreateAssetAsync_OnCancelled,
        calling_context: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<125>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_create_asset_async,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &package_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_class,
                __buffer
                    .add(32)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &factory,
                __buffer.add(40).cast::<UPtr<crate::bindings::unreal_ed::UFactory>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_complete,
                __buffer.add(48).cast::<FCreateAssetAsync_OnComplete>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_cancelled,
                __buffer.add(80).cast::<FCreateAssetAsync_OnCancelled>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &calling_context,
                __buffer.add(112).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_create_asset_async,
                __buffer,
            )
        };
        unsafe { __buffer.add(124).cast::<bool>().read() }
    }
    pub fn create_asset(
        &mut self,
        asset_name: FString,
        package_path: FString,
        asset_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        factory: UPtr<crate::bindings::unreal_ed::UFactory>,
        calling_context: FName,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS.u_asset_tools_create_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &package_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_class,
                __buffer
                    .add(32)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &factory,
                __buffer.add(40).cast::<UPtr<crate::bindings::unreal_ed::UFactory>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &calling_context,
                __buffer.add(48).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tools::__FUNCTION_PTRS.u_asset_tools_create_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(64)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn begin_advanced_copy_packages(
        &self,
        input_names_to_copy: &TArray<FName>,
        target_path: FString,
        on_copy_complete: &FBeginAdvancedCopyPackages_OnCopyComplete,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_begin_advanced_copy_packages,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                input_names_to_copy,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                on_copy_complete,
                __buffer.add(32).cast::<FBeginAdvancedCopyPackages_OnCopyComplete>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_begin_advanced_copy_packages,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UAssetToolsHelpers {
    __padding_end: [u8; 48],
}
impl UAssetToolsHelpers {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetToolsHelpers")
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
    pub fn get_asset_tools() -> TScriptInterface<UAssetTools> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_helpers_get_asset_tools,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::asset_tools::UAssetToolsHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::asset_tools::__FUNCTION_PTRS
                    .u_asset_tools_helpers_get_asset_tools,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TScriptInterface<UAssetTools>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAdvancedCopyCustomization {
    __padding_end: [u8; 424],
}
impl UAdvancedCopyCustomization {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAdvancedCopyCustomization")
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
pub struct UAssetDefinition_AssetTypeActionsProxy {
    __padding_end: [u8; 112],
}
impl UAssetDefinition_AssetTypeActionsProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_AssetTypeActionsProxy")
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
pub struct UAssetToolsImpl {
    __padding_end: [u8; 1064],
}
impl UAssetToolsImpl {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetToolsImpl")
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
pub struct FBeginAdvancedCopyPackages_OnCopyComplete {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCreateAssetAsync_OnComplete {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCreateAssetAsync_OnCancelled {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCreateAssetWithDialogAsync_OnComplete {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCreateAssetWithDialogAsync_OnCancelled {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct EAssetMigrationConflict(pub u8);
impl EAssetMigrationConflict {
    pub const SKIP: EAssetMigrationConflict = EAssetMigrationConflict(0);
    pub const OVERWRITE: EAssetMigrationConflict = EAssetMigrationConflict(1);
    pub const CANCEL: EAssetMigrationConflict = EAssetMigrationConflict(2);
}
#[repr(transparent)]
pub struct EAssetRenameResult(pub u8);
impl EAssetRenameResult {
    pub const FAILURE: EAssetRenameResult = EAssetRenameResult(0);
    pub const SUCCESS: EAssetRenameResult = EAssetRenameResult(1);
    pub const PENDING: EAssetRenameResult = EAssetRenameResult(2);
}
#[repr(transparent)]
pub struct EAssetTypeActivationOpenedMethod(pub u8);
impl EAssetTypeActivationOpenedMethod {
    pub const EDIT: EAssetTypeActivationOpenedMethod = EAssetTypeActivationOpenedMethod(
        0,
    );
    pub const VIEW: EAssetTypeActivationOpenedMethod = EAssetTypeActivationOpenedMethod(
        1,
    );
}
