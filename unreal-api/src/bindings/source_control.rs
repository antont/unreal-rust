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
