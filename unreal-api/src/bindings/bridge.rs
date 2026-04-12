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
    pub u_node_port_is_node_running: *mut crate::ffi::UFunctionOpague,
    pub u_node_port_get_node_port: *mut crate::ffi::UFunctionOpague,
    pub u_browser_binding_start_node_process: *mut crate::ffi::UFunctionOpague,
    pub u_browser_binding_show_login_dialog: *mut crate::ffi::UFunctionOpague,
    pub u_browser_binding_show_dialog: *mut crate::ffi::UFunctionOpague,
    pub u_browser_binding_send_success: *mut crate::ffi::UFunctionOpague,
    pub u_browser_binding_send_failure: *mut crate::ffi::UFunctionOpague,
    pub u_browser_binding_save_auth_token: *mut crate::ffi::UFunctionOpague,
    pub u_browser_binding_restart_node_process: *mut crate::ffi::UFunctionOpague,
    pub u_browser_binding_open_megascans_plugin_settings: *mut crate::ffi::UFunctionOpague,
    pub u_browser_binding_open_external_url: *mut crate::ffi::UFunctionOpague,
    pub u_browser_binding_on_exit_callback: *mut crate::ffi::UFunctionOpague,
    pub u_browser_binding_on_dropped_callback: *mut crate::ffi::UFunctionOpague,
    pub u_browser_binding_on_drop_discarded_callback: *mut crate::ffi::UFunctionOpague,
    pub u_browser_binding_on_bulk_export_metahumans_callback: *mut crate::ffi::UFunctionOpague,
    pub u_browser_binding_logout: *mut crate::ffi::UFunctionOpague,
    pub u_browser_binding_get_project_path: *mut crate::ffi::UFunctionOpague,
    pub u_browser_binding_get_auth_token: *mut crate::ffi::UFunctionOpague,
    pub u_browser_binding_export_data_to_ms_plugin: *mut crate::ffi::UFunctionOpague,
    pub u_browser_binding_drag_started: *mut crate::ffi::UFunctionOpague,
    pub u_browser_binding_dialog_success_callback: *mut crate::ffi::UFunctionOpague,
    pub u_browser_binding_dialog_fail_callback: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_node_port_is_node_running: std::ptr::null_mut(),
            u_node_port_get_node_port: std::ptr::null_mut(),
            u_browser_binding_start_node_process: std::ptr::null_mut(),
            u_browser_binding_show_login_dialog: std::ptr::null_mut(),
            u_browser_binding_show_dialog: std::ptr::null_mut(),
            u_browser_binding_send_success: std::ptr::null_mut(),
            u_browser_binding_send_failure: std::ptr::null_mut(),
            u_browser_binding_save_auth_token: std::ptr::null_mut(),
            u_browser_binding_restart_node_process: std::ptr::null_mut(),
            u_browser_binding_open_megascans_plugin_settings: std::ptr::null_mut(),
            u_browser_binding_open_external_url: std::ptr::null_mut(),
            u_browser_binding_on_exit_callback: std::ptr::null_mut(),
            u_browser_binding_on_dropped_callback: std::ptr::null_mut(),
            u_browser_binding_on_drop_discarded_callback: std::ptr::null_mut(),
            u_browser_binding_on_bulk_export_metahumans_callback: std::ptr::null_mut(),
            u_browser_binding_logout: std::ptr::null_mut(),
            u_browser_binding_get_project_path: std::ptr::null_mut(),
            u_browser_binding_get_auth_token: std::ptr::null_mut(),
            u_browser_binding_export_data_to_ms_plugin: std::ptr::null_mut(),
            u_browser_binding_drag_started: std::ptr::null_mut(),
            u_browser_binding_dialog_success_callback: std::ptr::null_mut(),
            u_browser_binding_dialog_fail_callback: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UNodePort::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsNodeRunning"),
                &raw mut __FUNCTION_PTRS.u_node_port_is_node_running,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNodePort"),
                &raw mut __FUNCTION_PTRS.u_node_port_get_node_port,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBrowserBinding::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartNodeProcess"),
                &raw mut __FUNCTION_PTRS.u_browser_binding_start_node_process,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ShowLoginDialog"),
                &raw mut __FUNCTION_PTRS.u_browser_binding_show_login_dialog,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ShowDialog"),
                &raw mut __FUNCTION_PTRS.u_browser_binding_show_dialog,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SendSuccess"),
                &raw mut __FUNCTION_PTRS.u_browser_binding_send_success,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SendFailure"),
                &raw mut __FUNCTION_PTRS.u_browser_binding_send_failure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SaveAuthToken"),
                &raw mut __FUNCTION_PTRS.u_browser_binding_save_auth_token,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RestartNodeProcess"),
                &raw mut __FUNCTION_PTRS.u_browser_binding_restart_node_process,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OpenMegascansPluginSettings"),
                &raw mut __FUNCTION_PTRS.u_browser_binding_open_megascans_plugin_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OpenExternalUrl"),
                &raw mut __FUNCTION_PTRS.u_browser_binding_open_external_url,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnExitCallback"),
                &raw mut __FUNCTION_PTRS.u_browser_binding_on_exit_callback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnDroppedCallback"),
                &raw mut __FUNCTION_PTRS.u_browser_binding_on_dropped_callback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnDropDiscardedCallback"),
                &raw mut __FUNCTION_PTRS.u_browser_binding_on_drop_discarded_callback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnBulkExportMetahumansCallback"),
                &raw mut __FUNCTION_PTRS
                    .u_browser_binding_on_bulk_export_metahumans_callback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Logout"),
                &raw mut __FUNCTION_PTRS.u_browser_binding_logout,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetProjectPath"),
                &raw mut __FUNCTION_PTRS.u_browser_binding_get_project_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAuthToken"),
                &raw mut __FUNCTION_PTRS.u_browser_binding_get_auth_token,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExportDataToMSPlugin"),
                &raw mut __FUNCTION_PTRS.u_browser_binding_export_data_to_ms_plugin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DragStarted"),
                &raw mut __FUNCTION_PTRS.u_browser_binding_drag_started,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DialogSuccessCallback"),
                &raw mut __FUNCTION_PTRS.u_browser_binding_dialog_success_callback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DialogFailCallback"),
                &raw mut __FUNCTION_PTRS.u_browser_binding_dialog_fail_callback,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct UNodePort {
    __padding_end: [u8; 72],
}
impl UNodePort {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNodePort")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UNodePort").copied()
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
}
#[repr(C, align(8))]
pub struct UBrowserBinding {
    __padding_end: [u8; 440],
}
impl UBrowserBinding {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBrowserBinding")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBrowserBinding")
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
}
