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
    pub fn show_confirmation_dialog() -> EGooglePADErrorCode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_SHOW_CONFIRMATION_DIALOG,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::google_pad::UGooglePADFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_SHOW_CONFIRMATION_DIALOG,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EGooglePADErrorCode>().read() }
    }
    pub fn show_cellular_data_confirmation() -> EGooglePADErrorCode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_SHOW_CELLULAR_DATA_CONFIRMATION,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::google_pad::UGooglePADFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_SHOW_CELLULAR_DATA_CONFIRMATION,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EGooglePADErrorCode>().read() }
    }
    pub fn request_removal(name: FString) -> EGooglePADErrorCode {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_REQUEST_REMOVAL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = crate::bindings::google_pad::UGooglePADFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_REQUEST_REMOVAL,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<EGooglePADErrorCode>().read() }
    }
    pub fn request_info(asset_packs: TArray<FString>) -> EGooglePADErrorCode {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_REQUEST_INFO,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_packs,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::google_pad::UGooglePADFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_REQUEST_INFO,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<EGooglePADErrorCode>().read() }
    }
    pub fn request_download(asset_packs: TArray<FString>) -> EGooglePADErrorCode {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_REQUEST_DOWNLOAD,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_packs,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::google_pad::UGooglePADFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_REQUEST_DOWNLOAD,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<EGooglePADErrorCode>().read() }
    }
    pub fn release_download_state(state: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_RELEASE_DOWNLOAD_STATE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&state, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::google_pad::UGooglePADFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_RELEASE_DOWNLOAD_STATE,
                __buffer,
            )
        };
    }
    pub fn release_asset_pack_location(location: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_RELEASE_ASSET_PACK_LOCATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&location, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::google_pad::UGooglePADFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_RELEASE_ASSET_PACK_LOCATION,
                __buffer,
            )
        };
    }
    pub fn get_total_bytes_to_download(state: i32) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_TOTAL_BYTES_TO_DOWNLOAD,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&state, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::google_pad::UGooglePADFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_TOTAL_BYTES_TO_DOWNLOAD,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn get_storage_method(location: i32) -> EGooglePADStorageMethod {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_STORAGE_METHOD,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&location, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::google_pad::UGooglePADFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_STORAGE_METHOD,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<EGooglePADStorageMethod>().read() }
    }
    pub fn get_show_confirmation_dialog_status(
        status: &mut EGooglePADConfirmationDialogStatus,
    ) -> EGooglePADErrorCode {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_SHOW_CONFIRMATION_DIALOG_STATUS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                status,
                __buffer.add(0).cast::<EGooglePADConfirmationDialogStatus>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::google_pad::UGooglePADFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_SHOW_CONFIRMATION_DIALOG_STATUS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<EGooglePADConfirmationDialogStatus>().swap(status);
        }
        unsafe { __buffer.add(1).cast::<EGooglePADErrorCode>().read() }
    }
    pub fn get_show_cellular_data_confirmation_status(
        status: &mut EGooglePADCellularDataConfirmStatus,
    ) -> EGooglePADErrorCode {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_SHOW_CELLULAR_DATA_CONFIRMATION_STATUS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                status,
                __buffer.add(0).cast::<EGooglePADCellularDataConfirmStatus>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::google_pad::UGooglePADFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_SHOW_CELLULAR_DATA_CONFIRMATION_STATUS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<EGooglePADCellularDataConfirmStatus>().swap(status);
        }
        unsafe { __buffer.add(1).cast::<EGooglePADErrorCode>().read() }
    }
    pub fn get_download_status(state: i32) -> EGooglePADDownloadStatus {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_DOWNLOAD_STATUS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&state, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::google_pad::UGooglePADFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_DOWNLOAD_STATUS,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<EGooglePADDownloadStatus>().read() }
    }
    pub fn get_download_state(name: FString, state: &mut i32) -> EGooglePADErrorCode {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_DOWNLOAD_STATE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(state, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::google_pad::UGooglePADFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_DOWNLOAD_STATE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<i32>().swap(state);
        }
        unsafe { __buffer.add(20).cast::<EGooglePADErrorCode>().read() }
    }
    pub fn get_bytes_downloaded(state: i32) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_BYTES_DOWNLOADED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&state, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::google_pad::UGooglePADFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_BYTES_DOWNLOADED,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn get_assets_path(location: i32) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_ASSETS_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&location, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::google_pad::UGooglePADFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_ASSETS_PATH,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn get_asset_pack_location(
        name: FString,
        location: &mut i32,
    ) -> EGooglePADErrorCode {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_ASSET_PACK_LOCATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(location, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::google_pad::UGooglePADFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_GET_ASSET_PACK_LOCATION,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<i32>().swap(location);
        }
        unsafe { __buffer.add(20).cast::<EGooglePADErrorCode>().read() }
    }
    pub fn cancel_download(asset_packs: TArray<FString>) -> EGooglePADErrorCode {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_CANCEL_DOWNLOAD,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_packs,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::google_pad::UGooglePADFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::google_pad::U_GOOGLE_PAD_FUNCTION_LIBRARY_CANCEL_DOWNLOAD,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<EGooglePADErrorCode>().read() }
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
