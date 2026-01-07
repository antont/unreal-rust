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
pub static mut U_SOURCE_CONTROL_HELPERS_SYNC_FILES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_SYNC_FILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_REVERT_UNCHANGED_FILES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_REVERT_UNCHANGED_FILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_REVERT_FILES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_REVERT_FILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_REVERT_AND_RELOAD_PACKAGES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_QUERY_FILE_STATES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_QUERY_FILE_STATE_DELEGATE_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_QUERY_FILE_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_MARK_FILES_FOR_DELETE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_MARK_FILES_FOR_ADD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_MARK_FILE_FOR_DELETE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_MARK_FILE_FOR_ADD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_LAST_ERROR_MSG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_IS_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_IS_AVAILABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_CURRENT_PROVIDER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_COPY_FILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_CHECK_OUT_OR_ADD_FILES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_CHECK_OUT_OR_ADD_FILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_CHECK_OUT_FILES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_CHECK_OUT_FILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_CHECK_IN_FILES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_CHECK_IN_FILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_ASYNC_QUERY_FILE_STATES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SOURCE_CONTROL_HELPERS_ASYNC_QUERY_FILE_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USourceControlHelpers::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SyncFiles"),
            &raw mut U_SOURCE_CONTROL_HELPERS_SYNC_FILES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SyncFile"),
            &raw mut U_SOURCE_CONTROL_HELPERS_SYNC_FILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RevertUnchangedFiles"),
            &raw mut U_SOURCE_CONTROL_HELPERS_REVERT_UNCHANGED_FILES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RevertUnchangedFile"),
            &raw mut U_SOURCE_CONTROL_HELPERS_REVERT_UNCHANGED_FILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RevertFiles"),
            &raw mut U_SOURCE_CONTROL_HELPERS_REVERT_FILES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RevertFile"),
            &raw mut U_SOURCE_CONTROL_HELPERS_REVERT_FILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RevertAndReloadPackages"),
            &raw mut U_SOURCE_CONTROL_HELPERS_REVERT_AND_RELOAD_PACKAGES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("QueryFileStates"),
            &raw mut U_SOURCE_CONTROL_HELPERS_QUERY_FILE_STATES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("QueryFileStateDelegate__DelegateSignature"),
            &raw mut U_SOURCE_CONTROL_HELPERS_QUERY_FILE_STATE_DELEGATE_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("QueryFileState"),
            &raw mut U_SOURCE_CONTROL_HELPERS_QUERY_FILE_STATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MarkFilesForDelete"),
            &raw mut U_SOURCE_CONTROL_HELPERS_MARK_FILES_FOR_DELETE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MarkFilesForAdd"),
            &raw mut U_SOURCE_CONTROL_HELPERS_MARK_FILES_FOR_ADD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MarkFileForDelete"),
            &raw mut U_SOURCE_CONTROL_HELPERS_MARK_FILE_FOR_DELETE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MarkFileForAdd"),
            &raw mut U_SOURCE_CONTROL_HELPERS_MARK_FILE_FOR_ADD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LastErrorMsg"),
            &raw mut U_SOURCE_CONTROL_HELPERS_LAST_ERROR_MSG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsEnabled"),
            &raw mut U_SOURCE_CONTROL_HELPERS_IS_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAvailable"),
            &raw mut U_SOURCE_CONTROL_HELPERS_IS_AVAILABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CurrentProvider"),
            &raw mut U_SOURCE_CONTROL_HELPERS_CURRENT_PROVIDER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CopyFile"),
            &raw mut U_SOURCE_CONTROL_HELPERS_COPY_FILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckOutOrAddFiles"),
            &raw mut U_SOURCE_CONTROL_HELPERS_CHECK_OUT_OR_ADD_FILES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckOutOrAddFile"),
            &raw mut U_SOURCE_CONTROL_HELPERS_CHECK_OUT_OR_ADD_FILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckOutFiles"),
            &raw mut U_SOURCE_CONTROL_HELPERS_CHECK_OUT_FILES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckOutFile"),
            &raw mut U_SOURCE_CONTROL_HELPERS_CHECK_OUT_FILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckInFiles"),
            &raw mut U_SOURCE_CONTROL_HELPERS_CHECK_IN_FILES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckInFile"),
            &raw mut U_SOURCE_CONTROL_HELPERS_CHECK_IN_FILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AsyncQueryFileStates"),
            &raw mut U_SOURCE_CONTROL_HELPERS_ASYNC_QUERY_FILE_STATES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AsyncQueryFileState"),
            &raw mut U_SOURCE_CONTROL_HELPERS_ASYNC_QUERY_FILE_STATE,
        );
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_SYNC_FILES,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_SYNC_FILES,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_SYNC_FILE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_SYNC_FILE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_REVERT_UNCHANGED_FILES,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_REVERT_UNCHANGED_FILES,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_REVERT_UNCHANGED_FILE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_REVERT_UNCHANGED_FILE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_REVERT_FILES,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_REVERT_FILES,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_REVERT_FILE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_REVERT_FILE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_REVERT_AND_RELOAD_PACKAGES,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_REVERT_AND_RELOAD_PACKAGES,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_QUERY_FILE_STATES,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_QUERY_FILE_STATES,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_QUERY_FILE_STATE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_QUERY_FILE_STATE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_MARK_FILES_FOR_DELETE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_MARK_FILES_FOR_DELETE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_MARK_FILES_FOR_ADD,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_MARK_FILES_FOR_ADD,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_MARK_FILE_FOR_DELETE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_MARK_FILE_FOR_DELETE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_MARK_FILE_FOR_ADD,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_MARK_FILE_FOR_ADD,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_LAST_ERROR_MSG,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_LAST_ERROR_MSG,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_IS_ENABLED,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_IS_ENABLED,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_IS_AVAILABLE,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_IS_AVAILABLE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_CURRENT_PROVIDER,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::source_control::USourceControlHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_CURRENT_PROVIDER,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_COPY_FILE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_COPY_FILE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_CHECK_OUT_OR_ADD_FILES,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_CHECK_OUT_OR_ADD_FILES,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_CHECK_OUT_OR_ADD_FILE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_CHECK_OUT_OR_ADD_FILE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_CHECK_OUT_FILES,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_CHECK_OUT_FILES,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_CHECK_OUT_FILE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_CHECK_OUT_FILE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_CHECK_IN_FILES,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_CHECK_IN_FILES,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_CHECK_IN_FILE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_CHECK_IN_FILE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_ASYNC_QUERY_FILE_STATES,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_ASYNC_QUERY_FILE_STATES,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_ASYNC_QUERY_FILE_STATE,
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
                crate::bindings::source_control::U_SOURCE_CONTROL_HELPERS_ASYNC_QUERY_FILE_STATE,
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
