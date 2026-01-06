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
pub static mut U_MOBILE_INSTALLED_CONTENT_MOUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOBILE_INSTALLED_CONTENT_GET_INSTALLED_CONTENT_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOBILE_INSTALLED_CONTENT_GET_DISK_FREE_SPACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOBILE_PENDING_CONTENT_START_INSTALL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOBILE_PENDING_CONTENT_GET_TOTAL_DOWNLOADED_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOBILE_PENDING_CONTENT_GET_REQUIRED_DISK_SPACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOBILE_PENDING_CONTENT_GET_INSTALL_PROGRESS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOBILE_PENDING_CONTENT_GET_DOWNLOAD_STATUS_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOBILE_PENDING_CONTENT_GET_DOWNLOAD_SPEED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOBILE_PENDING_CONTENT_GET_DOWNLOAD_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOBILE_PATCHING_LIBRARY_REQUEST_CONTENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOBILE_PATCHING_LIBRARY_HAS_ACTIVE_WI_FI_CONNECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOBILE_PATCHING_LIBRARY_GET_SUPPORTED_PLATFORM_NAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOBILE_PATCHING_LIBRARY_GET_INSTALLED_CONTENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOBILE_PATCHING_LIBRARY_GET_ACTIVE_DEVICE_PROFILE_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMobileInstalledContent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Mount"),
            &raw mut U_MOBILE_INSTALLED_CONTENT_MOUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInstalledContentSize"),
            &raw mut U_MOBILE_INSTALLED_CONTENT_GET_INSTALLED_CONTENT_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDiskFreeSpace"),
            &raw mut U_MOBILE_INSTALLED_CONTENT_GET_DISK_FREE_SPACE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMobilePendingContent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartInstall"),
            &raw mut U_MOBILE_PENDING_CONTENT_START_INSTALL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTotalDownloadedSize"),
            &raw mut U_MOBILE_PENDING_CONTENT_GET_TOTAL_DOWNLOADED_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRequiredDiskSpace"),
            &raw mut U_MOBILE_PENDING_CONTENT_GET_REQUIRED_DISK_SPACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInstallProgress"),
            &raw mut U_MOBILE_PENDING_CONTENT_GET_INSTALL_PROGRESS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDownloadStatusText"),
            &raw mut U_MOBILE_PENDING_CONTENT_GET_DOWNLOAD_STATUS_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDownloadSpeed"),
            &raw mut U_MOBILE_PENDING_CONTENT_GET_DOWNLOAD_SPEED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDownloadSize"),
            &raw mut U_MOBILE_PENDING_CONTENT_GET_DOWNLOAD_SIZE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMobilePatchingLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestContent"),
            &raw mut U_MOBILE_PATCHING_LIBRARY_REQUEST_CONTENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasActiveWiFiConnection"),
            &raw mut U_MOBILE_PATCHING_LIBRARY_HAS_ACTIVE_WI_FI_CONNECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSupportedPlatformNames"),
            &raw mut U_MOBILE_PATCHING_LIBRARY_GET_SUPPORTED_PLATFORM_NAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInstalledContent"),
            &raw mut U_MOBILE_PATCHING_LIBRARY_GET_INSTALLED_CONTENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActiveDeviceProfileName"),
            &raw mut U_MOBILE_PATCHING_LIBRARY_GET_ACTIVE_DEVICE_PROFILE_NAME,
        );
    }
}
#[repr(C, align(8))]
pub struct UMobileInstalledContent {
    __padding_end: [u8; 80],
}
impl UMobileInstalledContent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMobileInstalledContent")
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
pub struct UMobilePendingContent {
    __padding_end: [u8; 144],
}
impl UMobilePendingContent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMobilePendingContent")
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
pub struct UMobilePatchingLibrary {
    __padding_end: [u8; 48],
}
impl UMobilePatchingLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMobilePatchingLibrary")
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
pub struct FStartInstall_OnSucceeded {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FStartInstall_OnFailed {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FRequestContent_OnSucceeded {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FRequestContent_OnFailed {
    _opague: [u8; 32],
}
