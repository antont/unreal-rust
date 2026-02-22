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
    pub u_rust_extension_f_hit_result_new: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_core_test_f_string_copy: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_core_test_f_string: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_core_test_array_copy: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_core_test_array: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_core_new_object: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_core_f_name_none: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_core_create_test_array: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_type_def_add_numeric: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_rust_extension_f_hit_result_new: std::ptr::null_mut(),
            u_rust_extension_core_test_f_string_copy: std::ptr::null_mut(),
            u_rust_extension_core_test_f_string: std::ptr::null_mut(),
            u_rust_extension_core_test_array_copy: std::ptr::null_mut(),
            u_rust_extension_core_test_array: std::ptr::null_mut(),
            u_rust_extension_core_new_object: std::ptr::null_mut(),
            u_rust_extension_core_f_name_none: std::ptr::null_mut(),
            u_rust_extension_core_create_test_array: std::ptr::null_mut(),
            u_rust_extension_rust_type_def_add_numeric: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URustExtension_FHitResult::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("New"),
                &raw mut __FUNCTION_PTRS.u_rust_extension_f_hit_result_new,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URustExtension_Core::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TestFStringCopy"),
                &raw mut __FUNCTION_PTRS.u_rust_extension_core_test_f_string_copy,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TestFString"),
                &raw mut __FUNCTION_PTRS.u_rust_extension_core_test_f_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TestArrayCopy"),
                &raw mut __FUNCTION_PTRS.u_rust_extension_core_test_array_copy,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TestArray"),
                &raw mut __FUNCTION_PTRS.u_rust_extension_core_test_array,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NewObject"),
                &raw mut __FUNCTION_PTRS.u_rust_extension_core_new_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FNameNone"),
                &raw mut __FUNCTION_PTRS.u_rust_extension_core_f_name_none,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTestArray"),
                &raw mut __FUNCTION_PTRS.u_rust_extension_core_create_test_array,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URustExtension_RustTypeDef::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddNumeric"),
                &raw mut __FUNCTION_PTRS.u_rust_extension_rust_type_def_add_numeric,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FRustTypeInfo {
    pub(crate) __padding_end: [u8; 24],
}
impl FRustTypeInfo {}
#[repr(C, align(8))]
pub struct FRustType {
    pub(crate) __padding_end: [u8; 32],
}
impl FRustType {}
#[repr(C, align(8))]
pub struct FRustType_Numeric {
    pub(crate) __padding_end: [u8; 32],
}
impl FRustType_Numeric {}
#[repr(C, align(8))]
pub struct FRustTypeDef {
    pub(crate) __padding_end: [u8; 16],
}
impl FRustTypeDef {
    pub fn add_numeric(&mut self, def: &mut FRustTypeDef, type_info: FRustTypeInfo) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_gameplay::__FUNCTION_PTRS
                    .u_rust_extension_rust_type_def_add_numeric,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                def,
                __buffer.add(0).cast::<FRustTypeDef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &type_info,
                __buffer.add(16).cast::<FRustTypeInfo>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_gameplay::__FUNCTION_PTRS
                    .u_rust_extension_rust_type_def_add_numeric,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FRustTypeDef>().swap(def);
        }
        std::mem::forget(type_info);
    }
}
#[repr(C, align(8))]
pub struct URustExtension {
    __padding_end: [u8; 48],
}
impl URustExtension {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustExtension")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustExtension")
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
pub struct URustExtension_FHitResult {
    __padding_end: [u8; 48],
}
impl URustExtension_FHitResult {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustExtension_FHitResult")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustExtension_FHitResult")
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
    pub fn new() -> crate::bindings::engine::FHitResult {
        let mut __stack = crate::core_data::StackAlloc::<264>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_gameplay::__FUNCTION_PTRS
                    .u_rust_extension_f_hit_result_new,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_gameplay::URustExtension_FHitResult::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_gameplay::__FUNCTION_PTRS
                    .u_rust_extension_f_hit_result_new,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<crate::bindings::engine::FHitResult>().read() }
    }
}
#[repr(C, align(8))]
pub struct URustExtension_Core {
    __padding_end: [u8; 48],
}
impl URustExtension_Core {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustExtension_Core")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustExtension_Core")
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
    pub fn test_f_string_copy(str: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_gameplay::__FUNCTION_PTRS
                    .u_rust_extension_core_test_f_string_copy,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&str, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = crate::bindings::rust_gameplay::URustExtension_Core::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_gameplay::__FUNCTION_PTRS
                    .u_rust_extension_core_test_f_string_copy,
                __buffer,
            )
        };
        std::mem::forget(str);
    }
    pub fn test_f_string(str: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_gameplay::__FUNCTION_PTRS
                    .u_rust_extension_core_test_f_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&str, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = crate::bindings::rust_gameplay::URustExtension_Core::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_gameplay::__FUNCTION_PTRS
                    .u_rust_extension_core_test_f_string,
                __buffer,
            )
        };
        std::mem::forget(str);
    }
    pub fn test_array_copy(arr: TArray<i32>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_gameplay::__FUNCTION_PTRS
                    .u_rust_extension_core_test_array_copy,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &arr,
                __buffer.add(0).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_gameplay::URustExtension_Core::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_gameplay::__FUNCTION_PTRS
                    .u_rust_extension_core_test_array_copy,
                __buffer,
            )
        };
        std::mem::forget(arr);
    }
    pub fn test_array(arr: &TArray<i32>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_gameplay::__FUNCTION_PTRS
                    .u_rust_extension_core_test_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(arr, __buffer.add(0).cast::<TArray<i32>>(), 1);
        }
        let __object_ptr = crate::bindings::rust_gameplay::URustExtension_Core::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_gameplay::__FUNCTION_PTRS
                    .u_rust_extension_core_test_array,
                __buffer,
            )
        };
    }
    pub fn new_object(
        outer: UPtr<crate::bindings::core_u_object::UObject>,
        class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        name: FName,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_gameplay::__FUNCTION_PTRS
                    .u_rust_extension_core_new_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &outer,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer
                    .add(8)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(16).cast::<FName>(), 1);
        }
        let __object_ptr = crate::bindings::rust_gameplay::URustExtension_Core::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_gameplay::__FUNCTION_PTRS
                    .u_rust_extension_core_new_object,
                __buffer,
            )
        };
        std::mem::forget(outer);
        std::mem::forget(class);
        std::mem::forget(name);
        unsafe {
            __buffer
                .add(32)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn f_name_none() -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_gameplay::__FUNCTION_PTRS
                    .u_rust_extension_core_f_name_none,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_gameplay::URustExtension_Core::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_gameplay::__FUNCTION_PTRS
                    .u_rust_extension_core_f_name_none,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn create_test_array() -> TArray<i32> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_gameplay::__FUNCTION_PTRS
                    .u_rust_extension_core_create_test_array,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_gameplay::URustExtension_Core::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_gameplay::__FUNCTION_PTRS
                    .u_rust_extension_core_create_test_array,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<i32>>().read() }
    }
}
#[repr(C, align(8))]
pub struct URustExtension_RustTypeDef {
    __padding_end: [u8; 48],
}
impl URustExtension_RustTypeDef {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustExtension_RustTypeDef")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustExtension_RustTypeDef")
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
    pub fn add_numeric(&mut self, def: &mut FRustTypeDef, type_info: FRustTypeInfo) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_gameplay::__FUNCTION_PTRS
                    .u_rust_extension_rust_type_def_add_numeric,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                def,
                __buffer.add(0).cast::<FRustTypeDef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &type_info,
                __buffer.add(16).cast::<FRustTypeInfo>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_gameplay::__FUNCTION_PTRS
                    .u_rust_extension_rust_type_def_add_numeric,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FRustTypeDef>().swap(def);
        }
        std::mem::forget(type_info);
    }
}
