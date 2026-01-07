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
    pub fn mount(&mut self, pak_order: i32, mount_point: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mobile_patching_utils::U_MOBILE_INSTALLED_CONTENT_MOUNT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&pak_order, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mount_point,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::U_MOBILE_INSTALLED_CONTENT_MOUNT,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_installed_content_size(&mut self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mobile_patching_utils::U_MOBILE_INSTALLED_CONTENT_GET_INSTALLED_CONTENT_SIZE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::U_MOBILE_INSTALLED_CONTENT_GET_INSTALLED_CONTENT_SIZE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_disk_free_space(&mut self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mobile_patching_utils::U_MOBILE_INSTALLED_CONTENT_GET_DISK_FREE_SPACE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::U_MOBILE_INSTALLED_CONTENT_GET_DISK_FREE_SPACE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
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
    pub fn start_install(
        &mut self,
        on_succeeded: FStartInstall_OnSucceeded,
        on_failed: FStartInstall_OnFailed,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mobile_patching_utils::U_MOBILE_PENDING_CONTENT_START_INSTALL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_succeeded,
                __buffer.add(0).cast::<FStartInstall_OnSucceeded>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_failed,
                __buffer.add(32).cast::<FStartInstall_OnFailed>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::U_MOBILE_PENDING_CONTENT_START_INSTALL,
                __buffer,
            )
        };
    }
    pub fn get_total_downloaded_size(&mut self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mobile_patching_utils::U_MOBILE_PENDING_CONTENT_GET_TOTAL_DOWNLOADED_SIZE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::U_MOBILE_PENDING_CONTENT_GET_TOTAL_DOWNLOADED_SIZE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_required_disk_space(&mut self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mobile_patching_utils::U_MOBILE_PENDING_CONTENT_GET_REQUIRED_DISK_SPACE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::U_MOBILE_PENDING_CONTENT_GET_REQUIRED_DISK_SPACE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_install_progress(&mut self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mobile_patching_utils::U_MOBILE_PENDING_CONTENT_GET_INSTALL_PROGRESS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::U_MOBILE_PENDING_CONTENT_GET_INSTALL_PROGRESS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_download_status_text(&mut self) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mobile_patching_utils::U_MOBILE_PENDING_CONTENT_GET_DOWNLOAD_STATUS_TEXT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::U_MOBILE_PENDING_CONTENT_GET_DOWNLOAD_STATUS_TEXT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
    pub fn get_download_speed(&mut self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mobile_patching_utils::U_MOBILE_PENDING_CONTENT_GET_DOWNLOAD_SPEED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::U_MOBILE_PENDING_CONTENT_GET_DOWNLOAD_SPEED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_download_size(&mut self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mobile_patching_utils::U_MOBILE_PENDING_CONTENT_GET_DOWNLOAD_SIZE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::U_MOBILE_PENDING_CONTENT_GET_DOWNLOAD_SIZE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
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
    pub fn request_content(
        remote_manifest_url: FString,
        cloud_url: FString,
        install_directory: FString,
        on_succeeded: FRequestContent_OnSucceeded,
        on_failed: FRequestContent_OnFailed,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mobile_patching_utils::U_MOBILE_PATCHING_LIBRARY_REQUEST_CONTENT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &remote_manifest_url,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &cloud_url,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &install_directory,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_succeeded,
                __buffer.add(48).cast::<FRequestContent_OnSucceeded>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_failed,
                __buffer.add(80).cast::<FRequestContent_OnFailed>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mobile_patching_utils::UMobilePatchingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::U_MOBILE_PATCHING_LIBRARY_REQUEST_CONTENT,
                __buffer,
            )
        };
    }
    pub fn has_active_wi_fi_connection() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mobile_patching_utils::U_MOBILE_PATCHING_LIBRARY_HAS_ACTIVE_WI_FI_CONNECTION,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::mobile_patching_utils::UMobilePatchingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::U_MOBILE_PATCHING_LIBRARY_HAS_ACTIVE_WI_FI_CONNECTION,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_supported_platform_names() -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mobile_patching_utils::U_MOBILE_PATCHING_LIBRARY_GET_SUPPORTED_PLATFORM_NAMES,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::mobile_patching_utils::UMobilePatchingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::U_MOBILE_PATCHING_LIBRARY_GET_SUPPORTED_PLATFORM_NAMES,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
    pub fn get_installed_content(
        install_directory: FString,
    ) -> UPtr<UMobileInstalledContent> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mobile_patching_utils::U_MOBILE_PATCHING_LIBRARY_GET_INSTALLED_CONTENT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &install_directory,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mobile_patching_utils::UMobilePatchingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::U_MOBILE_PATCHING_LIBRARY_GET_INSTALLED_CONTENT,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UMobileInstalledContent>>().read() }
    }
    pub fn get_active_device_profile_name() -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mobile_patching_utils::U_MOBILE_PATCHING_LIBRARY_GET_ACTIVE_DEVICE_PROFILE_NAME,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::mobile_patching_utils::UMobilePatchingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::U_MOBILE_PATCHING_LIBRARY_GET_ACTIVE_DEVICE_PROFILE_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
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
