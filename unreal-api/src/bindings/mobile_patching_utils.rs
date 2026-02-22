#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(forgetting_copy_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_mobile_installed_content_mount: *mut crate::ffi::UFunctionOpague,
    pub u_mobile_installed_content_get_installed_content_size: *mut crate::ffi::UFunctionOpague,
    pub u_mobile_installed_content_get_disk_free_space: *mut crate::ffi::UFunctionOpague,
    pub u_mobile_pending_content_start_install: *mut crate::ffi::UFunctionOpague,
    pub u_mobile_pending_content_get_total_downloaded_size: *mut crate::ffi::UFunctionOpague,
    pub u_mobile_pending_content_get_required_disk_space: *mut crate::ffi::UFunctionOpague,
    pub u_mobile_pending_content_get_install_progress: *mut crate::ffi::UFunctionOpague,
    pub u_mobile_pending_content_get_download_status_text: *mut crate::ffi::UFunctionOpague,
    pub u_mobile_pending_content_get_download_speed: *mut crate::ffi::UFunctionOpague,
    pub u_mobile_pending_content_get_download_size: *mut crate::ffi::UFunctionOpague,
    pub u_mobile_patching_library_request_content: *mut crate::ffi::UFunctionOpague,
    pub u_mobile_patching_library_has_active_wi_fi_connection: *mut crate::ffi::UFunctionOpague,
    pub u_mobile_patching_library_get_supported_platform_names: *mut crate::ffi::UFunctionOpague,
    pub u_mobile_patching_library_get_installed_content: *mut crate::ffi::UFunctionOpague,
    pub u_mobile_patching_library_get_active_device_profile_name: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_mobile_installed_content_mount: std::ptr::null_mut(),
            u_mobile_installed_content_get_installed_content_size: std::ptr::null_mut(),
            u_mobile_installed_content_get_disk_free_space: std::ptr::null_mut(),
            u_mobile_pending_content_start_install: std::ptr::null_mut(),
            u_mobile_pending_content_get_total_downloaded_size: std::ptr::null_mut(),
            u_mobile_pending_content_get_required_disk_space: std::ptr::null_mut(),
            u_mobile_pending_content_get_install_progress: std::ptr::null_mut(),
            u_mobile_pending_content_get_download_status_text: std::ptr::null_mut(),
            u_mobile_pending_content_get_download_speed: std::ptr::null_mut(),
            u_mobile_pending_content_get_download_size: std::ptr::null_mut(),
            u_mobile_patching_library_request_content: std::ptr::null_mut(),
            u_mobile_patching_library_has_active_wi_fi_connection: std::ptr::null_mut(),
            u_mobile_patching_library_get_supported_platform_names: std::ptr::null_mut(),
            u_mobile_patching_library_get_installed_content: std::ptr::null_mut(),
            u_mobile_patching_library_get_active_device_profile_name: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMobileInstalledContent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Mount"),
                &raw mut __FUNCTION_PTRS.u_mobile_installed_content_mount,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInstalledContentSize"),
                &raw mut __FUNCTION_PTRS
                    .u_mobile_installed_content_get_installed_content_size,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDiskFreeSpace"),
                &raw mut __FUNCTION_PTRS.u_mobile_installed_content_get_disk_free_space,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMobilePendingContent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartInstall"),
                &raw mut __FUNCTION_PTRS.u_mobile_pending_content_start_install,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTotalDownloadedSize"),
                &raw mut __FUNCTION_PTRS
                    .u_mobile_pending_content_get_total_downloaded_size,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRequiredDiskSpace"),
                &raw mut __FUNCTION_PTRS.u_mobile_pending_content_get_required_disk_space,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInstallProgress"),
                &raw mut __FUNCTION_PTRS.u_mobile_pending_content_get_install_progress,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDownloadStatusText"),
                &raw mut __FUNCTION_PTRS
                    .u_mobile_pending_content_get_download_status_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDownloadSpeed"),
                &raw mut __FUNCTION_PTRS.u_mobile_pending_content_get_download_speed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDownloadSize"),
                &raw mut __FUNCTION_PTRS.u_mobile_pending_content_get_download_size,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMobilePatchingLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RequestContent"),
                &raw mut __FUNCTION_PTRS.u_mobile_patching_library_request_content,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasActiveWiFiConnection"),
                &raw mut __FUNCTION_PTRS
                    .u_mobile_patching_library_has_active_wi_fi_connection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSupportedPlatformNames"),
                &raw mut __FUNCTION_PTRS
                    .u_mobile_patching_library_get_supported_platform_names,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInstalledContent"),
                &raw mut __FUNCTION_PTRS.u_mobile_patching_library_get_installed_content,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActiveDeviceProfileName"),
                &raw mut __FUNCTION_PTRS
                    .u_mobile_patching_library_get_active_device_profile_name,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct UMobileInstalledContent {
    __padding_end: [u8; 80],
}
impl UMobileInstalledContent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMobileInstalledContent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMobileInstalledContent")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
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
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_installed_content_mount,
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
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_installed_content_mount,
                __buffer,
            )
        };
        std::mem::forget(pak_order);
        std::mem::forget(mount_point);
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
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_installed_content_get_installed_content_size,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_installed_content_get_installed_content_size,
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
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_installed_content_get_disk_free_space,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_installed_content_get_disk_free_space,
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
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMobilePendingContent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMobilePendingContent")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
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
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_pending_content_start_install,
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
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_pending_content_start_install,
                __buffer,
            )
        };
        std::mem::forget(on_succeeded);
        std::mem::forget(on_failed);
    }
    pub fn get_total_downloaded_size(&mut self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_pending_content_get_total_downloaded_size,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_pending_content_get_total_downloaded_size,
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
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_pending_content_get_required_disk_space,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_pending_content_get_required_disk_space,
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
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_pending_content_get_install_progress,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_pending_content_get_install_progress,
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
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_pending_content_get_download_status_text,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_pending_content_get_download_status_text,
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
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_pending_content_get_download_speed,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_pending_content_get_download_speed,
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
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_pending_content_get_download_size,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_pending_content_get_download_size,
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
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMobilePatchingLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMobilePatchingLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
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
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_patching_library_request_content,
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
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_patching_library_request_content,
                __buffer,
            )
        };
        std::mem::forget(remote_manifest_url);
        std::mem::forget(cloud_url);
        std::mem::forget(install_directory);
        std::mem::forget(on_succeeded);
        std::mem::forget(on_failed);
    }
    pub fn has_active_wi_fi_connection() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_patching_library_has_active_wi_fi_connection,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::mobile_patching_utils::UMobilePatchingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_patching_library_has_active_wi_fi_connection,
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
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_patching_library_get_supported_platform_names,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::mobile_patching_utils::UMobilePatchingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_patching_library_get_supported_platform_names,
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
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_patching_library_get_installed_content,
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
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_patching_library_get_installed_content,
                __buffer,
            )
        };
        std::mem::forget(install_directory);
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
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_patching_library_get_active_device_profile_name,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::mobile_patching_utils::UMobilePatchingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mobile_patching_utils::__FUNCTION_PTRS
                    .u_mobile_patching_library_get_active_device_profile_name,
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
