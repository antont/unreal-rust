#![allow(clippy::all)]
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
    pub u_source_control_helpers_sync_files: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_sync_file: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_revert_unchanged_files: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_revert_unchanged_file: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_revert_files: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_revert_file: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_revert_and_reload_packages: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_query_file_states: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_query_file_state_delegate_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_query_file_state: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_mark_files_for_delete: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_mark_files_for_add: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_mark_file_for_delete: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_mark_file_for_add: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_last_error_msg: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_is_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_is_available: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_current_provider: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_copy_file: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_check_out_or_add_files: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_check_out_or_add_file: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_check_out_files: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_check_out_file: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_check_in_files: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_check_in_file: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_async_query_file_states: *mut crate::ffi::UFunctionOpague,
    pub u_source_control_helpers_async_query_file_state: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_source_control_helpers_sync_files: std::ptr::null_mut(),
            u_source_control_helpers_sync_file: std::ptr::null_mut(),
            u_source_control_helpers_revert_unchanged_files: std::ptr::null_mut(),
            u_source_control_helpers_revert_unchanged_file: std::ptr::null_mut(),
            u_source_control_helpers_revert_files: std::ptr::null_mut(),
            u_source_control_helpers_revert_file: std::ptr::null_mut(),
            u_source_control_helpers_revert_and_reload_packages: std::ptr::null_mut(),
            u_source_control_helpers_query_file_states: std::ptr::null_mut(),
            u_source_control_helpers_query_file_state_delegate_delegate_signature: std::ptr::null_mut(),
            u_source_control_helpers_query_file_state: std::ptr::null_mut(),
            u_source_control_helpers_mark_files_for_delete: std::ptr::null_mut(),
            u_source_control_helpers_mark_files_for_add: std::ptr::null_mut(),
            u_source_control_helpers_mark_file_for_delete: std::ptr::null_mut(),
            u_source_control_helpers_mark_file_for_add: std::ptr::null_mut(),
            u_source_control_helpers_last_error_msg: std::ptr::null_mut(),
            u_source_control_helpers_is_enabled: std::ptr::null_mut(),
            u_source_control_helpers_is_available: std::ptr::null_mut(),
            u_source_control_helpers_current_provider: std::ptr::null_mut(),
            u_source_control_helpers_copy_file: std::ptr::null_mut(),
            u_source_control_helpers_check_out_or_add_files: std::ptr::null_mut(),
            u_source_control_helpers_check_out_or_add_file: std::ptr::null_mut(),
            u_source_control_helpers_check_out_files: std::ptr::null_mut(),
            u_source_control_helpers_check_out_file: std::ptr::null_mut(),
            u_source_control_helpers_check_in_files: std::ptr::null_mut(),
            u_source_control_helpers_check_in_file: std::ptr::null_mut(),
            u_source_control_helpers_async_query_file_states: std::ptr::null_mut(),
            u_source_control_helpers_async_query_file_state: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USourceControlHelpers::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SyncFiles"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_sync_files,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SyncFile"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_sync_file,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RevertUnchangedFiles"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_revert_unchanged_files,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RevertUnchangedFile"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_revert_unchanged_file,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RevertFiles"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_revert_files,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RevertFile"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_revert_file,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RevertAndReloadPackages"),
                &raw mut __FUNCTION_PTRS
                    .u_source_control_helpers_revert_and_reload_packages,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("QueryFileStates"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_query_file_states,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("QueryFileStateDelegate__DelegateSignature"),
                &raw mut __FUNCTION_PTRS
                    .u_source_control_helpers_query_file_state_delegate_delegate_signature,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("QueryFileState"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_query_file_state,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MarkFilesForDelete"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_mark_files_for_delete,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MarkFilesForAdd"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_mark_files_for_add,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MarkFileForDelete"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_mark_file_for_delete,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MarkFileForAdd"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_mark_file_for_add,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("LastErrorMsg"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_last_error_msg,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsEnabled"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_is_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsAvailable"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_is_available,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CurrentProvider"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_current_provider,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CopyFile"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_copy_file,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CheckOutOrAddFiles"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_check_out_or_add_files,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CheckOutOrAddFile"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_check_out_or_add_file,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CheckOutFiles"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_check_out_files,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CheckOutFile"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_check_out_file,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CheckInFiles"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_check_in_files,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CheckInFile"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_check_in_file,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AsyncQueryFileStates"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_async_query_file_states,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AsyncQueryFileState"),
                &raw mut __FUNCTION_PTRS.u_source_control_helpers_async_query_file_state,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FSourceControlState {
    pub filename: FString,
    pub b_is_valid: bool,
    pub b_is_unknown: bool,
    pub b_can_check_in: bool,
    pub b_can_check_out: bool,
    pub b_is_checked_out: bool,
    pub b_is_current: bool,
    pub b_is_source_controlled: bool,
    pub b_is_added: bool,
    pub b_is_deleted: bool,
    pub b_is_ignored: bool,
    pub b_can_edit: bool,
    pub b_can_delete: bool,
    pub b_is_modified: bool,
    pub b_can_add: bool,
    pub b_is_conflicted: bool,
    pub b_can_revert: bool,
    pub b_is_checked_out_other: bool,
    pub checked_out_other: FString,
    pub b_is_checked_out_in_other_branch: bool,
    pub b_is_modified_in_other_branch: bool,
    pub previous_user: FString,
}
impl FSourceControlState {}
#[repr(C, align(8))]
pub struct USourceControlHelpers {
    __padding_end: [u8; 48],
}
impl USourceControlHelpers {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceControlHelpers")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceControlHelpers")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn sync_files(in_files: &TArray<FString>, b_silent: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_sync_files,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_files,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_sync_files,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn sync_file(in_file: FString, b_silent: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_sync_file,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_file,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_sync_file,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn revert_unchanged_files(in_files: &TArray<FString>, b_silent: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_revert_unchanged_files,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_files,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_revert_unchanged_files,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn revert_unchanged_file(in_file: FString, b_silent: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_revert_unchanged_file,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_file,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_revert_unchanged_file,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn revert_files(in_files: &TArray<FString>, b_silent: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_revert_files,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_files,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_revert_files,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn revert_file(in_file: FString, b_silent: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_revert_file,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_file,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_revert_file,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn revert_and_reload_packages(
        in_packages_to_revert: &TArray<FString>,
        b_revert_all: bool,
        b_reload_world: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_revert_and_reload_packages,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_packages_to_revert,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_revert_all,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_reload_world,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_revert_and_reload_packages,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn query_file_states(
        in_files: TArray<FString>,
        b_silent: bool,
    ) -> TArray<FSourceControlState> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_query_file_states,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_files,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_query_file_states,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<FSourceControlState>>().read() }
    }
    pub fn query_file_state(in_file: FString, b_silent: bool) -> FSourceControlState {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_query_file_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_file,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_query_file_state,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FSourceControlState>().read() }
    }
    pub fn mark_files_for_delete(in_files: &TArray<FString>, b_silent: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_mark_files_for_delete,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_files,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_mark_files_for_delete,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn mark_files_for_add(in_files: &TArray<FString>, b_silent: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_mark_files_for_add,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_files,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_mark_files_for_add,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn mark_file_for_delete(in_file: FString, b_silent: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_mark_file_for_delete,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_file,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_mark_file_for_delete,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn mark_file_for_add(in_file: FString, b_silent: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_mark_file_for_add,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_file,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_mark_file_for_add,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn last_error_msg() -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_last_error_msg,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_last_error_msg,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
    pub fn is_enabled() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_is_enabled,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_is_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_available() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_is_available,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_is_available,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn current_provider() -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_current_provider,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_current_provider,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn copy_file(
        in_source_file: FString,
        in_dest_file: FString,
        b_silent: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_copy_file,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_source_file,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_dest_file,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_copy_file,
                __buffer,
            )
        };
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
    pub fn check_out_or_add_files(in_files: &TArray<FString>, b_silent: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_check_out_or_add_files,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_files,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_check_out_or_add_files,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn check_out_or_add_file(in_file: FString, b_silent: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_check_out_or_add_file,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_file,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_check_out_or_add_file,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn check_out_files(in_files: &TArray<FString>, b_silent: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_check_out_files,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_files,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_check_out_files,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn check_out_file(in_file: FString, b_silent: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_check_out_file,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_file,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_check_out_file,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn check_in_files(
        in_files: &TArray<FString>,
        in_description: FString,
        b_silent: bool,
        b_keep_checked_out: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_check_in_files,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_files,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_description,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(32).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_keep_checked_out,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_check_in_files,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn check_in_file(
        in_file: FString,
        in_description: FString,
        b_silent: bool,
        b_keep_checked_out: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_check_in_file,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_file,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_description,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(32).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_keep_checked_out,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_check_in_file,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn async_query_file_states(
        file_state_callback: FAsyncQueryFileStates_FileStateCallback,
        in_files: TArray<FString>,
        b_silent: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_async_query_file_states,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &file_state_callback,
                __buffer.add(0).cast::<FAsyncQueryFileStates_FileStateCallback>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_files,
                __buffer.add(32).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(48).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_async_query_file_states,
                __buffer,
            )
        };
    }
    pub fn async_query_file_state(
        file_state_callback: FAsyncQueryFileState_FileStateCallback,
        in_file: FString,
        b_silent: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_async_query_file_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &file_state_callback,
                __buffer.add(0).cast::<FAsyncQueryFileState_FileStateCallback>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_file,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_silent, __buffer.add(48).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::__FUNCTION_PTRS
                    .u_source_control_helpers_async_query_file_state,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USourceControlPreferences {
    __padding_end: [u8; 216],
}
impl USourceControlPreferences {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceControlPreferences")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceControlPreferences")
            .copied()
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
pub struct FAsyncQueryFileState_FileStateCallback {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAsyncQueryFileStates_FileStateCallback {
    _opague: [u8; 32],
}
