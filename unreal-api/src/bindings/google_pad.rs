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
pub static mut U_GOOGLE_PAD_FUNCTION_LIBRARY_SHOW_CONFIRMATION_DIALOG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GOOGLE_PAD_FUNCTION_LIBRARY_SHOW_CELLULAR_DATA_CONFIRMATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GOOGLE_PAD_FUNCTION_LIBRARY_REQUEST_REMOVAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GOOGLE_PAD_FUNCTION_LIBRARY_REQUEST_INFO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GOOGLE_PAD_FUNCTION_LIBRARY_REQUEST_DOWNLOAD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GOOGLE_PAD_FUNCTION_LIBRARY_RELEASE_DOWNLOAD_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GOOGLE_PAD_FUNCTION_LIBRARY_RELEASE_ASSET_PACK_LOCATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_TOTAL_BYTES_TO_DOWNLOAD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_STORAGE_METHOD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_SHOW_CONFIRMATION_DIALOG_STATUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_SHOW_CELLULAR_DATA_CONFIRMATION_STATUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_DOWNLOAD_STATUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_DOWNLOAD_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_BYTES_DOWNLOADED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_ASSETS_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_ASSET_PACK_LOCATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GOOGLE_PAD_FUNCTION_LIBRARY_CANCEL_DOWNLOAD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGooglePADFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShowConfirmationDialog"),
            &raw mut U_GOOGLE_PAD_FUNCTION_LIBRARY_SHOW_CONFIRMATION_DIALOG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShowCellularDataConfirmation"),
            &raw mut U_GOOGLE_PAD_FUNCTION_LIBRARY_SHOW_CELLULAR_DATA_CONFIRMATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestRemoval"),
            &raw mut U_GOOGLE_PAD_FUNCTION_LIBRARY_REQUEST_REMOVAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestInfo"),
            &raw mut U_GOOGLE_PAD_FUNCTION_LIBRARY_REQUEST_INFO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestDownload"),
            &raw mut U_GOOGLE_PAD_FUNCTION_LIBRARY_REQUEST_DOWNLOAD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReleaseDownloadState"),
            &raw mut U_GOOGLE_PAD_FUNCTION_LIBRARY_RELEASE_DOWNLOAD_STATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReleaseAssetPackLocation"),
            &raw mut U_GOOGLE_PAD_FUNCTION_LIBRARY_RELEASE_ASSET_PACK_LOCATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTotalBytesToDownload"),
            &raw mut U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_TOTAL_BYTES_TO_DOWNLOAD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStorageMethod"),
            &raw mut U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_STORAGE_METHOD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetShowConfirmationDialogStatus"),
            &raw mut U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_SHOW_CONFIRMATION_DIALOG_STATUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetShowCellularDataConfirmationStatus"),
            &raw mut U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_SHOW_CELLULAR_DATA_CONFIRMATION_STATUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDownloadStatus"),
            &raw mut U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_DOWNLOAD_STATUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDownloadState"),
            &raw mut U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_DOWNLOAD_STATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBytesDownloaded"),
            &raw mut U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_BYTES_DOWNLOADED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetsPath"),
            &raw mut U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_ASSETS_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetPackLocation"),
            &raw mut U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_ASSET_PACK_LOCATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CancelDownload"),
            &raw mut U_GOOGLE_PAD_FUNCTION_LIBRARY_CANCEL_DOWNLOAD,
        );
    }
}
#[repr(C, align(8))]
pub struct UGooglePADFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UGooglePADFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGooglePADFunctionLibrary")
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
#[repr(transparent)]
pub struct EGooglePADErrorCode(pub u8);
impl EGooglePADErrorCode {
    pub const ASSET_PACK_NO_ERROR: EGooglePADErrorCode = EGooglePADErrorCode(0);
    pub const ASSET_PACK_APP_UNAVAILABLE: EGooglePADErrorCode = EGooglePADErrorCode(1);
    pub const ASSET_PACK_UNAVAILABLE: EGooglePADErrorCode = EGooglePADErrorCode(2);
    pub const ASSET_PACK_INVALID_REQUEST: EGooglePADErrorCode = EGooglePADErrorCode(3);
    pub const ASSET_PACK_DOWNLOAD_NOT_FOUND: EGooglePADErrorCode = EGooglePADErrorCode(
        4,
    );
    pub const ASSET_PACK_API_NOT_AVAILABLE: EGooglePADErrorCode = EGooglePADErrorCode(5);
    pub const ASSET_PACK_NETWORK_ERROR: EGooglePADErrorCode = EGooglePADErrorCode(6);
    pub const ASSET_PACK_ACCESS_DENIED: EGooglePADErrorCode = EGooglePADErrorCode(7);
    pub const ASSET_PACK_INSUFFICIENT_STORAGE: EGooglePADErrorCode = EGooglePADErrorCode(
        8,
    );
    pub const ASSET_PACK_PLAY_STORE_NOT_FOUND: EGooglePADErrorCode = EGooglePADErrorCode(
        9,
    );
    pub const ASSET_PACK_NETWORK_UNRESTRICTED: EGooglePADErrorCode = EGooglePADErrorCode(
        10,
    );
    pub const ASSET_PACK_INTERNAL_ERROR: EGooglePADErrorCode = EGooglePADErrorCode(11);
    pub const ASSET_PACK_INITIALIZATION_NEEDED: EGooglePADErrorCode = EGooglePADErrorCode(
        12,
    );
    pub const ASSET_PACK_INITIALIZATION_FAILED: EGooglePADErrorCode = EGooglePADErrorCode(
        13,
    );
    pub const ASSET_PACK_APP_NOT_OWNED: EGooglePADErrorCode = EGooglePADErrorCode(14);
    pub const ASSET_PACK_CONFIRMATION_NOT_REQUIRED: EGooglePADErrorCode = EGooglePADErrorCode(
        15,
    );
    pub const ASSET_PACK_UNRECOGNIZED_INSTALLATION: EGooglePADErrorCode = EGooglePADErrorCode(
        16,
    );
}
#[repr(transparent)]
pub struct EGooglePADDownloadStatus(pub u8);
impl EGooglePADDownloadStatus {
    pub const ASSET_PACK_UNKNOWN: EGooglePADDownloadStatus = EGooglePADDownloadStatus(0);
    pub const ASSET_PACK_DOWNLOAD_PENDING: EGooglePADDownloadStatus = EGooglePADDownloadStatus(
        1,
    );
    pub const ASSET_PACK_DOWNLOADING: EGooglePADDownloadStatus = EGooglePADDownloadStatus(
        2,
    );
    pub const ASSET_PACK_TRANSFERRING: EGooglePADDownloadStatus = EGooglePADDownloadStatus(
        3,
    );
    pub const ASSET_PACK_DOWNLOAD_COMPLETED: EGooglePADDownloadStatus = EGooglePADDownloadStatus(
        4,
    );
    pub const ASSET_PACK_DOWNLOAD_FAILED: EGooglePADDownloadStatus = EGooglePADDownloadStatus(
        5,
    );
    pub const ASSET_PACK_DOWNLOAD_CANCELED: EGooglePADDownloadStatus = EGooglePADDownloadStatus(
        6,
    );
    pub const ASSET_PACK_WAITING_FOR_WIFI: EGooglePADDownloadStatus = EGooglePADDownloadStatus(
        7,
    );
    pub const ASSET_PACK_NOT_INSTALLED: EGooglePADDownloadStatus = EGooglePADDownloadStatus(
        8,
    );
    pub const ASSET_PACK_INFO_PENDING: EGooglePADDownloadStatus = EGooglePADDownloadStatus(
        9,
    );
    pub const ASSET_PACK_INFO_FAILED: EGooglePADDownloadStatus = EGooglePADDownloadStatus(
        10,
    );
    pub const ASSET_PACK_REMOVAL_PENDING: EGooglePADDownloadStatus = EGooglePADDownloadStatus(
        11,
    );
    pub const ASSET_PACK_REMOVAL_FAILED: EGooglePADDownloadStatus = EGooglePADDownloadStatus(
        12,
    );
    pub const ASSET_PACK_REQUIRES_USER_CONFIRMATION: EGooglePADDownloadStatus = EGooglePADDownloadStatus(
        13,
    );
}
#[repr(transparent)]
pub struct EGooglePADCellularDataConfirmStatus(pub u8);
impl EGooglePADCellularDataConfirmStatus {
    pub const ASSET_PACK_CONFIRM_UNKNOWN: EGooglePADCellularDataConfirmStatus = EGooglePADCellularDataConfirmStatus(
        0,
    );
    pub const ASSET_PACK_CONFIRM_PENDING: EGooglePADCellularDataConfirmStatus = EGooglePADCellularDataConfirmStatus(
        1,
    );
    pub const ASSET_PACK_CONFIRM_USER_APPROVED: EGooglePADCellularDataConfirmStatus = EGooglePADCellularDataConfirmStatus(
        2,
    );
    pub const ASSET_PACK_CONFIRM_USER_CANCELED: EGooglePADCellularDataConfirmStatus = EGooglePADCellularDataConfirmStatus(
        3,
    );
}
#[repr(transparent)]
pub struct EGooglePADConfirmationDialogStatus(pub u8);
impl EGooglePADConfirmationDialogStatus {
    pub const ASSET_PACK_CONFIRMATION_DIALOG_UNKNOWN: EGooglePADConfirmationDialogStatus = EGooglePADConfirmationDialogStatus(
        0,
    );
    pub const ASSET_PACK_CONFIRMATION_DIALOG_PENDING: EGooglePADConfirmationDialogStatus = EGooglePADConfirmationDialogStatus(
        1,
    );
    pub const ASSET_PACK_CONFIRMATION_DIALOG_APPROVED: EGooglePADConfirmationDialogStatus = EGooglePADConfirmationDialogStatus(
        2,
    );
    pub const ASSET_PACK_CONFIRMATION_DIALOG_CANCELED: EGooglePADConfirmationDialogStatus = EGooglePADConfirmationDialogStatus(
        3,
    );
}
#[repr(transparent)]
pub struct EGooglePADStorageMethod(pub u8);
impl EGooglePADStorageMethod {
    pub const ASSET_PACK_STORAGE_FILES: EGooglePADStorageMethod = EGooglePADStorageMethod(
        0,
    );
    pub const ASSET_PACK_STORAGE_APK: EGooglePADStorageMethod = EGooglePADStorageMethod(
        1,
    );
    pub const ASSET_PACK_STORAGE_UNKNOWN: EGooglePADStorageMethod = EGooglePADStorageMethod(
        2,
    );
    pub const ASSET_PACK_STORAGE_NOT_INSTALLED: EGooglePADStorageMethod = EGooglePADStorageMethod(
        3,
    );
}
