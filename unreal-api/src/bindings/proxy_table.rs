#![allow(clippy::all)]
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
    pub u_proxy_table_function_library_make_lookup_proxy_with_override_table: *mut crate::ffi::UFunctionOpague,
    pub u_proxy_table_function_library_make_lookup_proxy: *mut crate::ffi::UFunctionOpague,
    pub u_proxy_table_function_library_evaluate_proxy_table: *mut crate::ffi::UFunctionOpague,
    pub u_proxy_table_function_library_evaluate_proxy_asset: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_proxy_table_function_library_make_lookup_proxy_with_override_table: std::ptr::null_mut(),
            u_proxy_table_function_library_make_lookup_proxy: std::ptr::null_mut(),
            u_proxy_table_function_library_evaluate_proxy_table: std::ptr::null_mut(),
            u_proxy_table_function_library_evaluate_proxy_asset: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UProxyTableFunctionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeLookupProxyWithOverrideTable"),
                &raw mut __FUNCTION_PTRS
                    .u_proxy_table_function_library_make_lookup_proxy_with_override_table,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeLookupProxy"),
                &raw mut __FUNCTION_PTRS.u_proxy_table_function_library_make_lookup_proxy,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EvaluateProxyTable"),
                &raw mut __FUNCTION_PTRS
                    .u_proxy_table_function_library_evaluate_proxy_table,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EvaluateProxyAsset"),
                &raw mut __FUNCTION_PTRS
                    .u_proxy_table_function_library_evaluate_proxy_asset,
            );
        }
    }
}
pub struct IChooserParameterProxyTable {}
#[repr(C, align(8))]
pub struct UChooserParameterProxyTable {
    __padding_end: [u8; 48],
}
impl UChooserParameterProxyTable {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChooserParameterProxyTable")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChooserParameterProxyTable")
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
pub struct UProxyAsset {
    __padding_end: [u8; 192],
}
impl UProxyAsset {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProxyAsset")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProxyAsset")
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
pub struct UProxyTable {
    __padding_end: [u8; 176],
}
impl UProxyTable {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProxyTable")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProxyTable")
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
pub struct UProxyTableFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UProxyTableFunctionLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProxyTableFunctionLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProxyTableFunctionLibrary")
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
    pub fn make_lookup_proxy_with_override_table(
        proxy: UPtr<UProxyAsset>,
        proxy_table: UPtr<UProxyTable>,
    ) -> crate::bindings::core_u_object::FInstancedStruct {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::proxy_table::__FUNCTION_PTRS
                    .u_proxy_table_function_library_make_lookup_proxy_with_override_table,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &proxy,
                __buffer.add(0).cast::<UPtr<UProxyAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &proxy_table,
                __buffer.add(8).cast::<UPtr<UProxyTable>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::proxy_table::UProxyTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::proxy_table::__FUNCTION_PTRS
                    .u_proxy_table_function_library_make_lookup_proxy_with_override_table,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FInstancedStruct>()
                .read()
        }
    }
    pub fn make_lookup_proxy(
        proxy: UPtr<UProxyAsset>,
    ) -> crate::bindings::core_u_object::FInstancedStruct {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::proxy_table::__FUNCTION_PTRS
                    .u_proxy_table_function_library_make_lookup_proxy,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &proxy,
                __buffer.add(0).cast::<UPtr<UProxyAsset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::proxy_table::UProxyTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::proxy_table::__FUNCTION_PTRS
                    .u_proxy_table_function_library_make_lookup_proxy,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::core_u_object::FInstancedStruct>()
                .read()
        }
    }
    pub fn evaluate_proxy_table(
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
        proxy_table: UPtr<UProxyTable>,
        key: FName,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::proxy_table::__FUNCTION_PTRS
                    .u_proxy_table_function_library_evaluate_proxy_table,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &proxy_table,
                __buffer.add(8).cast::<UPtr<UProxyTable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&key, __buffer.add(16).cast::<FName>(), 1);
        }
        let __object_ptr = crate::bindings::proxy_table::UProxyTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::proxy_table::__FUNCTION_PTRS
                    .u_proxy_table_function_library_evaluate_proxy_table,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn evaluate_proxy_asset(
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
        proxy: UPtr<UProxyAsset>,
        object_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::proxy_table::__FUNCTION_PTRS
                    .u_proxy_table_function_library_evaluate_proxy_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &proxy,
                __buffer.add(8).cast::<UPtr<UProxyAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_class,
                __buffer
                    .add(16)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::proxy_table::UProxyTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::proxy_table::__FUNCTION_PTRS
                    .u_proxy_table_function_library_evaluate_proxy_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
}
