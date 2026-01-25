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
    pub u_content_browser_data_subsystem_get_items_under_path: *mut crate::ffi::UFunctionOpague,
    pub u_content_browser_data_subsystem_get_items_at_path: *mut crate::ffi::UFunctionOpague,
    pub u_content_browser_data_subsystem_get_item_at_path: *mut crate::ffi::UFunctionOpague,
    pub u_content_browser_data_subsystem_get_available_data_sources: *mut crate::ffi::UFunctionOpague,
    pub u_content_browser_data_subsystem_get_active_data_sources: *mut crate::ffi::UFunctionOpague,
    pub u_content_browser_data_subsystem_deactivate_data_source: *mut crate::ffi::UFunctionOpague,
    pub u_content_browser_data_subsystem_deactivate_all_data_sources: *mut crate::ffi::UFunctionOpague,
    pub u_content_browser_data_subsystem_activate_data_source: *mut crate::ffi::UFunctionOpague,
    pub u_content_browser_data_subsystem_activate_all_data_sources: *mut crate::ffi::UFunctionOpague,
    pub u_content_browser_item_library_is_folder: *mut crate::ffi::UFunctionOpague,
    pub u_content_browser_item_library_is_file: *mut crate::ffi::UFunctionOpague,
    pub u_content_browser_item_library_get_virtual_path: *mut crate::ffi::UFunctionOpague,
    pub u_content_browser_item_library_get_display_name: *mut crate::ffi::UFunctionOpague,
    pub u_content_browser_item_path_extensions_set_path: *mut crate::ffi::UFunctionOpague,
    pub u_content_browser_item_path_extensions_make_content_browser_item_path: *mut crate::ffi::UFunctionOpague,
    pub u_content_browser_item_path_extensions_get_virtual_path: *mut crate::ffi::UFunctionOpague,
    pub u_content_browser_item_path_extensions_get_internal_path: *mut crate::ffi::UFunctionOpague,
    pub u_content_browser_item_path_extensions_break_content_browser_item_path: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_content_browser_data_subsystem_get_items_under_path: std::ptr::null_mut(),
            u_content_browser_data_subsystem_get_items_at_path: std::ptr::null_mut(),
            u_content_browser_data_subsystem_get_item_at_path: std::ptr::null_mut(),
            u_content_browser_data_subsystem_get_available_data_sources: std::ptr::null_mut(),
            u_content_browser_data_subsystem_get_active_data_sources: std::ptr::null_mut(),
            u_content_browser_data_subsystem_deactivate_data_source: std::ptr::null_mut(),
            u_content_browser_data_subsystem_deactivate_all_data_sources: std::ptr::null_mut(),
            u_content_browser_data_subsystem_activate_data_source: std::ptr::null_mut(),
            u_content_browser_data_subsystem_activate_all_data_sources: std::ptr::null_mut(),
            u_content_browser_item_library_is_folder: std::ptr::null_mut(),
            u_content_browser_item_library_is_file: std::ptr::null_mut(),
            u_content_browser_item_library_get_virtual_path: std::ptr::null_mut(),
            u_content_browser_item_library_get_display_name: std::ptr::null_mut(),
            u_content_browser_item_path_extensions_set_path: std::ptr::null_mut(),
            u_content_browser_item_path_extensions_make_content_browser_item_path: std::ptr::null_mut(),
            u_content_browser_item_path_extensions_get_virtual_path: std::ptr::null_mut(),
            u_content_browser_item_path_extensions_get_internal_path: std::ptr::null_mut(),
            u_content_browser_item_path_extensions_break_content_browser_item_path: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UContentBrowserDataSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetItemsUnderPath"),
            &raw mut __FUNCTION_PTRS
                .u_content_browser_data_subsystem_get_items_under_path,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetItemsAtPath"),
            &raw mut __FUNCTION_PTRS.u_content_browser_data_subsystem_get_items_at_path,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetItemAtPath"),
            &raw mut __FUNCTION_PTRS.u_content_browser_data_subsystem_get_item_at_path,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAvailableDataSources"),
            &raw mut __FUNCTION_PTRS
                .u_content_browser_data_subsystem_get_available_data_sources,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActiveDataSources"),
            &raw mut __FUNCTION_PTRS
                .u_content_browser_data_subsystem_get_active_data_sources,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeactivateDataSource"),
            &raw mut __FUNCTION_PTRS
                .u_content_browser_data_subsystem_deactivate_data_source,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeactivateAllDataSources"),
            &raw mut __FUNCTION_PTRS
                .u_content_browser_data_subsystem_deactivate_all_data_sources,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivateDataSource"),
            &raw mut __FUNCTION_PTRS
                .u_content_browser_data_subsystem_activate_data_source,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivateAllDataSources"),
            &raw mut __FUNCTION_PTRS
                .u_content_browser_data_subsystem_activate_all_data_sources,
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
            &raw mut __FUNCTION_PTRS.u_content_browser_item_library_is_folder,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsFile"),
            &raw mut __FUNCTION_PTRS.u_content_browser_item_library_is_file,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVirtualPath"),
            &raw mut __FUNCTION_PTRS.u_content_browser_item_library_get_virtual_path,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDisplayName"),
            &raw mut __FUNCTION_PTRS.u_content_browser_item_library_get_display_name,
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
            &raw mut __FUNCTION_PTRS.u_content_browser_item_path_extensions_set_path,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeContentBrowserItemPath"),
            &raw mut __FUNCTION_PTRS
                .u_content_browser_item_path_extensions_make_content_browser_item_path,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVirtualPath"),
            &raw mut __FUNCTION_PTRS
                .u_content_browser_item_path_extensions_get_virtual_path,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInternalPath"),
            &raw mut __FUNCTION_PTRS
                .u_content_browser_item_path_extensions_get_internal_path,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BreakContentBrowserItemPath"),
            &raw mut __FUNCTION_PTRS
                .u_content_browser_item_path_extensions_break_content_browser_item_path,
        );
    }
}
#[repr(C, align(8))]
pub struct FContentBrowserDataFilter {
    pub b_recursive_paths: bool,
    pub item_type_filter: EContentBrowserItemTypeFilter,
    pub item_category_filter: EContentBrowserItemCategoryFilter,
    pub item_attribute_filter: EContentBrowserItemAttributeFilter,
    pub(crate) __padding_end: [u8; 28],
}
impl FContentBrowserDataFilter {}
#[repr(C, align(8))]
pub struct FContentBrowserDataObjectFilter {
    pub object_names_to_include: TArray<FName>,
    pub object_names_to_exclude: TArray<FName>,
    pub b_on_disk_objects_only: bool,
    pub(crate) __padding_end: [u8; 167],
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
    pub(crate) __padding_end: [u8; 22],
}
impl FContentBrowserDataPackageFilter {}
#[repr(C, align(8))]
pub struct FContentBrowserDataClassFilter {
    pub class_names_to_include: TArray<FString>,
    pub class_names_to_exclude: TArray<FString>,
    pub b_recursive_class_names_to_include: bool,
    pub b_recursive_class_names_to_exclude: bool,
    pub(crate) __padding_end: [u8; 22],
}
impl FContentBrowserDataClassFilter {}
#[repr(C, align(8))]
pub struct FContentBrowserDataCollectionFilter {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub b_include_child_collections: bool,
}
impl FContentBrowserDataCollectionFilter {}
#[repr(C, align(8))]
pub struct FContentBrowserItem {
    pub(crate) __padding_end: [u8; 96],
}
impl FContentBrowserItem {}
#[repr(C, align(4))]
pub struct FContentBrowserItemPath {
    pub(crate) __padding_end: [u8; 24],
}
impl FContentBrowserItemPath {}
#[repr(C, align(8))]
pub struct UContentBrowserDataMenuContext_AddNewMenu {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
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
    pub(crate) __padding_48: [u8; 48],
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
    pub(crate) __padding_48: [u8; 48],
    pub selected_items: TArray<FContentBrowserItem>,
    #[doc(hidden)]
    pub(crate) __padding_96: [u8; 32],
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
    pub(crate) __padding_48: [u8; 48],
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
    pub fn get_items_under_path(
        &self,
        in_path: FName,
        in_filter: &FContentBrowserDataFilter,
    ) -> TArray<FContentBrowserItem> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_data_subsystem_get_items_under_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_path, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_filter,
                __buffer.add(16).cast::<FContentBrowserDataFilter>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_data_subsystem_get_items_under_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<TArray<FContentBrowserItem>>().read() }
    }
    pub fn get_items_at_path(
        &self,
        in_path: FName,
        in_item_type_filter: EContentBrowserItemTypeFilter,
    ) -> TArray<FContentBrowserItem> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_data_subsystem_get_items_at_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_path, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item_type_filter,
                __buffer.add(12).cast::<EContentBrowserItemTypeFilter>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_data_subsystem_get_items_at_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TArray<FContentBrowserItem>>().read() }
    }
    pub fn get_item_at_path(
        &self,
        in_path: FName,
        in_item_type_filter: EContentBrowserItemTypeFilter,
    ) -> FContentBrowserItem {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_data_subsystem_get_item_at_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_path, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item_type_filter,
                __buffer.add(12).cast::<EContentBrowserItemTypeFilter>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_data_subsystem_get_item_at_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FContentBrowserItem>().read() }
    }
    pub fn get_available_data_sources(&self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_data_subsystem_get_available_data_sources,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_data_subsystem_get_available_data_sources,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn get_active_data_sources(&self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_data_subsystem_get_active_data_sources,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_data_subsystem_get_active_data_sources,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn deactivate_data_source(&mut self, name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_data_subsystem_deactivate_data_source,
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
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_data_subsystem_deactivate_data_source,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn deactivate_all_data_sources(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_data_subsystem_deactivate_all_data_sources,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_data_subsystem_deactivate_all_data_sources,
                __buffer,
            )
        };
    }
    pub fn activate_data_source(&mut self, name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_data_subsystem_activate_data_source,
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
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_data_subsystem_activate_data_source,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn activate_all_data_sources(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_data_subsystem_activate_all_data_sources,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_data_subsystem_activate_all_data_sources,
                __buffer,
            )
        };
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
    pub fn is_folder(item: &FContentBrowserItem) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_item_library_is_folder,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                item,
                __buffer.add(0).cast::<FContentBrowserItem>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::content_browser_data::UContentBrowserItemLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_item_library_is_folder,
                __buffer,
            )
        };
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn is_file(item: &FContentBrowserItem) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_item_library_is_file,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                item,
                __buffer.add(0).cast::<FContentBrowserItem>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::content_browser_data::UContentBrowserItemLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_item_library_is_file,
                __buffer,
            )
        };
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn get_virtual_path(item: &FContentBrowserItem) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<108>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_item_library_get_virtual_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                item,
                __buffer.add(0).cast::<FContentBrowserItem>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::content_browser_data::UContentBrowserItemLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_item_library_get_virtual_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(96).cast::<FName>().read() }
    }
    pub fn get_display_name(item: &FContentBrowserItem) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_item_library_get_display_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                item,
                __buffer.add(0).cast::<FContentBrowserItem>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::content_browser_data::UContentBrowserItemLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_item_library_get_display_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(96).cast::<FText>().read() }
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
    pub fn set_path(
        item_path: &mut FContentBrowserItemPath,
        in_path: FName,
        in_path_type: EContentBrowserPathType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<37>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_item_path_extensions_set_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                item_path,
                __buffer.add(0).cast::<FContentBrowserItemPath>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_path, __buffer.add(24).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_path_type,
                __buffer.add(36).cast::<EContentBrowserPathType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::content_browser_data::UContentBrowserItemPathExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_item_path_extensions_set_path,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FContentBrowserItemPath>().swap(item_path);
        }
    }
    pub fn make_content_browser_item_path(
        in_path: FName,
        in_path_type: EContentBrowserPathType,
    ) -> FContentBrowserItemPath {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_item_path_extensions_make_content_browser_item_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_path, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_path_type,
                __buffer.add(12).cast::<EContentBrowserPathType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::content_browser_data::UContentBrowserItemPathExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_item_path_extensions_make_content_browser_item_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FContentBrowserItemPath>().read() }
    }
    pub fn get_virtual_path(item_path: &FContentBrowserItemPath) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_item_path_extensions_get_virtual_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                item_path,
                __buffer.add(0).cast::<FContentBrowserItemPath>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::content_browser_data::UContentBrowserItemPathExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_item_path_extensions_get_virtual_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FName>().read() }
    }
    pub fn get_internal_path(item_path: &FContentBrowserItemPath) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_item_path_extensions_get_internal_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                item_path,
                __buffer.add(0).cast::<FContentBrowserItemPath>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::content_browser_data::UContentBrowserItemPathExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_item_path_extensions_get_internal_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FName>().read() }
    }
    pub fn break_content_browser_item_path(
        item_path: &FContentBrowserItemPath,
        virtual_path: &mut FName,
        internal_path: &mut FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_item_path_extensions_break_content_browser_item_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                item_path,
                __buffer.add(0).cast::<FContentBrowserItemPath>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                virtual_path,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                internal_path,
                __buffer.add(36).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::content_browser_data::UContentBrowserItemPathExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::content_browser_data::__FUNCTION_PTRS
                    .u_content_browser_item_path_extensions_break_content_browser_item_path,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<FName>().swap(virtual_path);
        }
        unsafe {
            __buffer.add(36).cast::<FName>().swap(internal_path);
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
