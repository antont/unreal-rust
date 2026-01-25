#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_android_file_server_bp_library_stop_file_server: *mut crate::ffi::UFunctionOpague,
    pub u_android_file_server_bp_library_start_file_server: *mut crate::ffi::UFunctionOpague,
    pub u_android_file_server_bp_library_is_file_server_running: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_android_file_server_bp_library_stop_file_server: std::ptr::null_mut(),
            u_android_file_server_bp_library_start_file_server: std::ptr::null_mut(),
            u_android_file_server_bp_library_is_file_server_running: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAndroidFileServerBPLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopFileServer"),
            &raw mut __FUNCTION_PTRS.u_android_file_server_bp_library_stop_file_server,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartFileServer"),
            &raw mut __FUNCTION_PTRS.u_android_file_server_bp_library_start_file_server,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsFileServerRunning"),
            &raw mut __FUNCTION_PTRS
                .u_android_file_server_bp_library_is_file_server_running,
        );
    }
}
#[repr(C, align(8))]
pub struct UAndroidFileServerBPLibrary {
    __padding_end: [u8; 48],
}
impl UAndroidFileServerBPLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAndroidFileServerBPLibrary")
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
    pub fn stop_file_server(b_usb: bool, b_network: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::android_file_server::__FUNCTION_PTRS
                    .u_android_file_server_bp_library_stop_file_server,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_usb, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_network, __buffer.add(1).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::android_file_server::UAndroidFileServerBPLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::android_file_server::__FUNCTION_PTRS
                    .u_android_file_server_bp_library_stop_file_server,
                __buffer,
            )
        };
        unsafe { __buffer.add(2).cast::<bool>().read() }
    }
    pub fn start_file_server(b_usb: bool, b_network: bool, port: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::android_file_server::__FUNCTION_PTRS
                    .u_android_file_server_bp_library_start_file_server,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_usb, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_network, __buffer.add(1).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&port, __buffer.add(4).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::android_file_server::UAndroidFileServerBPLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::android_file_server::__FUNCTION_PTRS
                    .u_android_file_server_bp_library_start_file_server,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_file_server_running() -> EAFSActiveType {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::android_file_server::__FUNCTION_PTRS
                    .u_android_file_server_bp_library_is_file_server_running,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::android_file_server::UAndroidFileServerBPLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::android_file_server::__FUNCTION_PTRS
                    .u_android_file_server_bp_library_is_file_server_running,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EAFSActiveType>().read() }
    }
}
#[repr(transparent)]
pub struct EAFSActiveType(pub u8);
impl EAFSActiveType {
    pub const NONE: EAFSActiveType = EAFSActiveType(0);
    pub const USB_ONLY: EAFSActiveType = EAFSActiveType(1);
    pub const NETWORK_ONLY: EAFSActiveType = EAFSActiveType(2);
    pub const COMBINED: EAFSActiveType = EAFSActiveType(3);
}
