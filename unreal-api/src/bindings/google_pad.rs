#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UGooglePADFunctionLibrary {}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
