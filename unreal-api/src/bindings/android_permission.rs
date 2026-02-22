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
    pub u_android_permission_function_library_check_permission: *mut crate::ffi::UFunctionOpague,
    pub u_android_permission_function_library_acquire_permissions: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_android_permission_function_library_check_permission: std::ptr::null_mut(),
            u_android_permission_function_library_acquire_permissions: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAndroidPermissionFunctionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CheckPermission"),
                &raw mut __FUNCTION_PTRS
                    .u_android_permission_function_library_check_permission,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AcquirePermissions"),
                &raw mut __FUNCTION_PTRS
                    .u_android_permission_function_library_acquire_permissions,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct UAndroidPermissionCallbackProxy {
    __padding_end: [u8; 104],
}
impl UAndroidPermissionCallbackProxy {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAndroidPermissionCallbackProxy")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAndroidPermissionCallbackProxy")
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
#[repr(C, align(8))]
pub struct UAndroidPermissionFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UAndroidPermissionFunctionLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAndroidPermissionFunctionLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAndroidPermissionFunctionLibrary")
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
    pub fn check_permission(permission: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::android_permission::__FUNCTION_PTRS
                    .u_android_permission_function_library_check_permission,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &permission,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::android_permission::UAndroidPermissionFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::android_permission::__FUNCTION_PTRS
                    .u_android_permission_function_library_check_permission,
                __buffer,
            )
        };
        std::mem::forget(permission);
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn acquire_permissions(
        permissions: &TArray<FString>,
    ) -> UPtr<UAndroidPermissionCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::android_permission::__FUNCTION_PTRS
                    .u_android_permission_function_library_acquire_permissions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                permissions,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::android_permission::UAndroidPermissionFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::android_permission::__FUNCTION_PTRS
                    .u_android_permission_function_library_acquire_permissions,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<UPtr<UAndroidPermissionCallbackProxy>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct FAndroidPermissionCallbackProxy_OnPermissionsGrantedDynamicDelegate {
    _opague: [u8; 24],
}
