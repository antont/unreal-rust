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
pub static mut U_NODE_PORT_IS_NODE_RUNNING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NODE_PORT_GET_NODE_PORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BROWSER_BINDING_START_NODE_PROCESS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BROWSER_BINDING_SHOW_LOGIN_DIALOG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BROWSER_BINDING_SHOW_DIALOG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BROWSER_BINDING_SEND_SUCCESS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BROWSER_BINDING_SEND_FAILURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BROWSER_BINDING_SAVE_AUTH_TOKEN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BROWSER_BINDING_RESTART_NODE_PROCESS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BROWSER_BINDING_OPEN_MEGASCANS_PLUGIN_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BROWSER_BINDING_OPEN_EXTERNAL_URL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BROWSER_BINDING_ON_EXIT_CALLBACK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BROWSER_BINDING_ON_DROPPED_CALLBACK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BROWSER_BINDING_ON_DROP_DISCARDED_CALLBACK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BROWSER_BINDING_ON_BULK_EXPORT_METAHUMANS_CALLBACK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BROWSER_BINDING_LOGOUT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BROWSER_BINDING_GET_PROJECT_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BROWSER_BINDING_GET_AUTH_TOKEN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BROWSER_BINDING_EXPORT_DATA_TO_MS_PLUGIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BROWSER_BINDING_DRAG_STARTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BROWSER_BINDING_DIALOG_SUCCESS_CALLBACK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BROWSER_BINDING_DIALOG_FAIL_CALLBACK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNodePort::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsNodeRunning"),
            &raw mut U_NODE_PORT_IS_NODE_RUNNING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodePort"),
            &raw mut U_NODE_PORT_GET_NODE_PORT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBrowserBinding::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartNodeProcess"),
            &raw mut U_BROWSER_BINDING_START_NODE_PROCESS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShowLoginDialog"),
            &raw mut U_BROWSER_BINDING_SHOW_LOGIN_DIALOG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShowDialog"),
            &raw mut U_BROWSER_BINDING_SHOW_DIALOG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SendSuccess"),
            &raw mut U_BROWSER_BINDING_SEND_SUCCESS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SendFailure"),
            &raw mut U_BROWSER_BINDING_SEND_FAILURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveAuthToken"),
            &raw mut U_BROWSER_BINDING_SAVE_AUTH_TOKEN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RestartNodeProcess"),
            &raw mut U_BROWSER_BINDING_RESTART_NODE_PROCESS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OpenMegascansPluginSettings"),
            &raw mut U_BROWSER_BINDING_OPEN_MEGASCANS_PLUGIN_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OpenExternalUrl"),
            &raw mut U_BROWSER_BINDING_OPEN_EXTERNAL_URL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnExitCallback"),
            &raw mut U_BROWSER_BINDING_ON_EXIT_CALLBACK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnDroppedCallback"),
            &raw mut U_BROWSER_BINDING_ON_DROPPED_CALLBACK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnDropDiscardedCallback"),
            &raw mut U_BROWSER_BINDING_ON_DROP_DISCARDED_CALLBACK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnBulkExportMetahumansCallback"),
            &raw mut U_BROWSER_BINDING_ON_BULK_EXPORT_METAHUMANS_CALLBACK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Logout"),
            &raw mut U_BROWSER_BINDING_LOGOUT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetProjectPath"),
            &raw mut U_BROWSER_BINDING_GET_PROJECT_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAuthToken"),
            &raw mut U_BROWSER_BINDING_GET_AUTH_TOKEN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportDataToMSPlugin"),
            &raw mut U_BROWSER_BINDING_EXPORT_DATA_TO_MS_PLUGIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DragStarted"),
            &raw mut U_BROWSER_BINDING_DRAG_STARTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DialogSuccessCallback"),
            &raw mut U_BROWSER_BINDING_DIALOG_SUCCESS_CALLBACK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DialogFailCallback"),
            &raw mut U_BROWSER_BINDING_DIALOG_FAIL_CALLBACK,
        );
    }
}
#[repr(C, align(8))]
pub struct UNodePort {
    __padding_end: [u8; 72],
}
impl UNodePort {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNodePort")
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
pub struct UBrowserBinding {
    __padding_end: [u8; 440],
}
impl UBrowserBinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBrowserBinding")
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
