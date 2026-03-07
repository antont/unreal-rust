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
    pub u_rust_extension_rust_class_def_register: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_new: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_weak_object: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_u_object: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_u_int8: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_u_int64: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_u_int32: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_u_int16: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_u_class: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_struct: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_soft_object: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_soft_class: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_set: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_map: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_lazy_object: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_int8: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_int64: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_int32: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_int16: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_f_text: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_f_string: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_f_name: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_float: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_enum: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_double: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_bool: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_create_type_array: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_rust_class_def_add_property: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_rust_extension_rust_class_def_register: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_new: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_weak_object: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_u_object: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_u_int8: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_u_int64: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_u_int32: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_u_int16: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_u_class: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_struct: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_soft_object: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_soft_class: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_set: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_map: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_lazy_object: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_int8: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_int64: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_int32: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_int16: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_f_text: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_f_string: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_f_name: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_float: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_enum: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_double: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_bool: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_create_type_array: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_add_property: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URustExtension_RustClassDef::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Register"),
                &raw mut __FUNCTION_PTRS.u_rust_extension_rust_class_def_register,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("New"),
                &raw mut __FUNCTION_PTRS.u_rust_extension_rust_class_def_new,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeWeakObject"),
                &raw mut __FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_weak_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeUObject"),
                &raw mut __FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeUInt8"),
                &raw mut __FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_int8,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeUInt64"),
                &raw mut __FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_int64,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeUInt32"),
                &raw mut __FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_int32,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeUInt16"),
                &raw mut __FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_int16,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeUClass"),
                &raw mut __FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeStruct"),
                &raw mut __FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_struct,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeSoftObject"),
                &raw mut __FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_soft_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeSoftClass"),
                &raw mut __FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_soft_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeSet"),
                &raw mut __FUNCTION_PTRS.u_rust_extension_rust_class_def_create_type_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeMap"),
                &raw mut __FUNCTION_PTRS.u_rust_extension_rust_class_def_create_type_map,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeLazyObject"),
                &raw mut __FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_lazy_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeInt8"),
                &raw mut __FUNCTION_PTRS.u_rust_extension_rust_class_def_create_type_int8,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeInt64"),
                &raw mut __FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_int64,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeInt32"),
                &raw mut __FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_int32,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeInt16"),
                &raw mut __FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_int16,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeFText"),
                &raw mut __FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_f_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeFString"),
                &raw mut __FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_f_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeFName"),
                &raw mut __FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_f_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeFloat"),
                &raw mut __FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeEnum"),
                &raw mut __FUNCTION_PTRS.u_rust_extension_rust_class_def_create_type_enum,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeDouble"),
                &raw mut __FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_double,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeBool"),
                &raw mut __FUNCTION_PTRS.u_rust_extension_rust_class_def_create_type_bool,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTypeArray"),
                &raw mut __FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_array,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddProperty"),
                &raw mut __FUNCTION_PTRS.u_rust_extension_rust_class_def_add_property,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FRustClassDef {
    pub(crate) __padding_end: [u8; 40],
}
impl FRustClassDef {
    pub fn register(def: &FRustClassDef) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_register,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                def,
                __buffer.add(0).cast::<FRustClassDef>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_register,
                __buffer,
            )
        };
    }
    pub fn new(name: FString, size: i32) -> FRustClassDef {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_new,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&size, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_new,
                __buffer,
            )
        };
        std::mem::forget(name);
        std::mem::forget(size);
        unsafe { __buffer.add(24).cast::<FRustClassDef>().read() }
    }
    pub fn create_type_weak_object(
        property_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_weak_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_weak_object,
                __buffer,
            )
        };
        std::mem::forget(property_class);
        unsafe { __buffer.add(8).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_u_object(
        property_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_object,
                __buffer,
            )
        };
        std::mem::forget(property_class);
        unsafe { __buffer.add(8).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_u_int8() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_int8,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_int8,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_u_int64() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_int64,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_int64,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_u_int32() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_int32,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_int32,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_u_int16() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_int16,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_int16,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_u_class(
        meta_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &meta_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_class,
                __buffer,
            )
        };
        std::mem::forget(meta_class);
        unsafe { __buffer.add(8).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_struct(
        script_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    ) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_struct,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &script_struct,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_struct,
                __buffer,
            )
        };
        std::mem::forget(script_struct);
        unsafe { __buffer.add(8).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_soft_object(
        property_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_soft_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_soft_object,
                __buffer,
            )
        };
        std::mem::forget(property_class);
        unsafe { __buffer.add(8).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_soft_class(
        meta_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_soft_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &meta_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_soft_class,
                __buffer,
            )
        };
        std::mem::forget(meta_class);
        unsafe { __buffer.add(8).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_set(element_type: UPtr<URustType>) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_set,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_type,
                __buffer.add(0).cast::<UPtr<URustType>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_set,
                __buffer,
            )
        };
        std::mem::forget(element_type);
        unsafe { __buffer.add(8).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_map(
        key_type: UPtr<URustType>,
        value_type: UPtr<URustType>,
    ) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_map,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &key_type,
                __buffer.add(0).cast::<UPtr<URustType>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value_type,
                __buffer.add(8).cast::<UPtr<URustType>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_map,
                __buffer,
            )
        };
        std::mem::forget(key_type);
        std::mem::forget(value_type);
        unsafe { __buffer.add(16).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_lazy_object(
        property_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_lazy_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_lazy_object,
                __buffer,
            )
        };
        std::mem::forget(property_class);
        unsafe { __buffer.add(8).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_int8() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_int8,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_int8,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_int64() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_int64,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_int64,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_int32() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_int32,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_int32,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_int16() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_int16,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_int16,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_f_text() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_f_text,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_f_text,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_f_string() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_f_string,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_f_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_f_name() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_f_name,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_f_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_float() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_float,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_float,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_enum(
        enum_: UPtr<crate::bindings::core_u_object::UEnum>,
    ) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_enum,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &enum_,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UEnum>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_enum,
                __buffer,
            )
        };
        std::mem::forget(enum_);
        unsafe { __buffer.add(8).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_double() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_double,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_double,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_bool() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_bool,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_bool,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_array(inner_type: UPtr<URustType>) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &inner_type,
                __buffer.add(0).cast::<UPtr<URustType>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_array,
                __buffer,
            )
        };
        std::mem::forget(inner_type);
        unsafe { __buffer.add(8).cast::<UPtr<URustType>>().read() }
    }
    pub fn add_property(
        def: &mut FRustClassDef,
        name: FString,
        offset: i32,
        ty: UPtr<URustType>,
        flags: i64,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_add_property,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                def,
                __buffer.add(0).cast::<FRustClassDef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(40).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&offset, __buffer.add(56).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ty,
                __buffer.add(64).cast::<UPtr<URustType>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&flags, __buffer.add(72).cast::<i64>(), 1);
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_add_property,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FRustClassDef>().swap(def);
        }
        std::mem::forget(name);
        std::mem::forget(offset);
        std::mem::forget(ty);
        std::mem::forget(flags);
    }
}
#[repr(C, align(8))]
pub struct URustType {
    __padding_end: [u8; 48],
}
impl URustType {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("URustType").copied()
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
pub struct URustType_Bool {
    __padding_end: [u8; 48],
}
impl URustType_Bool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Bool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Bool")
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
pub struct URustType_Int8 {
    __padding_end: [u8; 48],
}
impl URustType_Int8 {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Int8")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Int8")
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
pub struct URustType_Int16 {
    __padding_end: [u8; 48],
}
impl URustType_Int16 {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Int16")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Int16")
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
pub struct URustType_Int32 {
    __padding_end: [u8; 48],
}
impl URustType_Int32 {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Int32")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Int32")
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
pub struct URustType_Int64 {
    __padding_end: [u8; 48],
}
impl URustType_Int64 {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Int64")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Int64")
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
pub struct URustType_UInt8 {
    __padding_end: [u8; 48],
}
impl URustType_UInt8 {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_UInt8")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_UInt8")
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
pub struct URustType_UInt16 {
    __padding_end: [u8; 48],
}
impl URustType_UInt16 {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_UInt16")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_UInt16")
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
pub struct URustType_UInt32 {
    __padding_end: [u8; 48],
}
impl URustType_UInt32 {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_UInt32")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_UInt32")
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
pub struct URustType_UInt64 {
    __padding_end: [u8; 48],
}
impl URustType_UInt64 {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_UInt64")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_UInt64")
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
pub struct URustType_Float {
    __padding_end: [u8; 48],
}
impl URustType_Float {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Float")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Float")
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
pub struct URustType_Double {
    __padding_end: [u8; 48],
}
impl URustType_Double {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Double")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Double")
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
pub struct URustType_FName {
    __padding_end: [u8; 48],
}
impl URustType_FName {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_FName")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_FName")
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
pub struct URustType_FString {
    __padding_end: [u8; 48],
}
impl URustType_FString {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_FString")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_FString")
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
pub struct URustType_FText {
    __padding_end: [u8; 48],
}
impl URustType_FText {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_FText")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_FText")
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
pub struct URustType_UObject {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub property_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
impl URustType_UObject {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_UObject")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_UObject")
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
pub struct URustType_UClass {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub meta_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
impl URustType_UClass {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_UClass")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_UClass")
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
pub struct URustType_SoftObject {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub property_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
impl URustType_SoftObject {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_SoftObject")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_SoftObject")
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
pub struct URustType_SoftClass {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub meta_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
impl URustType_SoftClass {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_SoftClass")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_SoftClass")
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
pub struct URustType_WeakObject {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub property_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
impl URustType_WeakObject {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_WeakObject")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_WeakObject")
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
pub struct URustType_LazyObject {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub property_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
impl URustType_LazyObject {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_LazyObject")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_LazyObject")
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
pub struct URustType_Struct {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub script_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
}
impl URustType_Struct {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Struct")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Struct")
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
pub struct URustType_Enum {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub enum_: UPtr<crate::bindings::core_u_object::UEnum>,
}
impl URustType_Enum {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Enum")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Enum")
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
pub struct URustType_Array {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub inner_type: UPtr<URustType>,
}
impl URustType_Array {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Array")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Array")
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
pub struct URustType_Set {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub element_type: UPtr<URustType>,
}
impl URustType_Set {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Set")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Set")
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
pub struct URustType_Map {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub key_type: UPtr<URustType>,
    pub value_type: UPtr<URustType>,
}
impl URustType_Map {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Map")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Map")
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
pub struct URustExtension_RustClassDef {
    __padding_end: [u8; 48],
}
impl URustExtension_RustClassDef {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustExtension_RustClassDef")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustExtension_RustClassDef")
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
    pub fn register(def: &FRustClassDef) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_register,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                def,
                __buffer.add(0).cast::<FRustClassDef>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_register,
                __buffer,
            )
        };
    }
    pub fn new(name: FString, size: i32) -> FRustClassDef {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_new,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&size, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_new,
                __buffer,
            )
        };
        std::mem::forget(name);
        std::mem::forget(size);
        unsafe { __buffer.add(24).cast::<FRustClassDef>().read() }
    }
    pub fn create_type_weak_object(
        property_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_weak_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_weak_object,
                __buffer,
            )
        };
        std::mem::forget(property_class);
        unsafe { __buffer.add(8).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_u_object(
        property_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_object,
                __buffer,
            )
        };
        std::mem::forget(property_class);
        unsafe { __buffer.add(8).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_u_int8() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_int8,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_int8,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_u_int64() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_int64,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_int64,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_u_int32() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_int32,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_int32,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_u_int16() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_int16,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_int16,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_u_class(
        meta_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &meta_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_u_class,
                __buffer,
            )
        };
        std::mem::forget(meta_class);
        unsafe { __buffer.add(8).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_struct(
        script_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    ) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_struct,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &script_struct,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_struct,
                __buffer,
            )
        };
        std::mem::forget(script_struct);
        unsafe { __buffer.add(8).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_soft_object(
        property_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_soft_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_soft_object,
                __buffer,
            )
        };
        std::mem::forget(property_class);
        unsafe { __buffer.add(8).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_soft_class(
        meta_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_soft_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &meta_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_soft_class,
                __buffer,
            )
        };
        std::mem::forget(meta_class);
        unsafe { __buffer.add(8).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_set(element_type: UPtr<URustType>) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_set,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_type,
                __buffer.add(0).cast::<UPtr<URustType>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_set,
                __buffer,
            )
        };
        std::mem::forget(element_type);
        unsafe { __buffer.add(8).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_map(
        key_type: UPtr<URustType>,
        value_type: UPtr<URustType>,
    ) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_map,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &key_type,
                __buffer.add(0).cast::<UPtr<URustType>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value_type,
                __buffer.add(8).cast::<UPtr<URustType>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_map,
                __buffer,
            )
        };
        std::mem::forget(key_type);
        std::mem::forget(value_type);
        unsafe { __buffer.add(16).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_lazy_object(
        property_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_lazy_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_lazy_object,
                __buffer,
            )
        };
        std::mem::forget(property_class);
        unsafe { __buffer.add(8).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_int8() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_int8,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_int8,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_int64() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_int64,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_int64,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_int32() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_int32,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_int32,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_int16() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_int16,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_int16,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_f_text() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_f_text,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_f_text,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_f_string() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_f_string,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_f_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_f_name() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_f_name,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_f_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_float() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_float,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_float,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_enum(
        enum_: UPtr<crate::bindings::core_u_object::UEnum>,
    ) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_enum,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &enum_,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UEnum>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_enum,
                __buffer,
            )
        };
        std::mem::forget(enum_);
        unsafe { __buffer.add(8).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_double() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_double,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_double,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_bool() -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_bool,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_bool,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URustType>>().read() }
    }
    pub fn create_type_array(inner_type: UPtr<URustType>) -> UPtr<URustType> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &inner_type,
                __buffer.add(0).cast::<UPtr<URustType>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_create_type_array,
                __buffer,
            )
        };
        std::mem::forget(inner_type);
        unsafe { __buffer.add(8).cast::<UPtr<URustType>>().read() }
    }
    pub fn add_property(
        def: &mut FRustClassDef,
        name: FString,
        offset: i32,
        ty: UPtr<URustType>,
        flags: i64,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_add_property,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                def,
                __buffer.add(0).cast::<FRustClassDef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(40).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&offset, __buffer.add(56).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ty,
                __buffer.add(64).cast::<UPtr<URustType>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&flags, __buffer.add(72).cast::<i64>(), 1);
        }
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_add_property,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FRustClassDef>().swap(def);
        }
        std::mem::forget(name);
        std::mem::forget(offset);
        std::mem::forget(ty);
        std::mem::forget(flags);
    }
}
#[repr(C, align(8))]
pub struct ARustGameModeBase {
    __padding_end: [u8; 1328],
}
impl ARustGameModeBase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ARustGameModeBase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ARustGameModeBase")
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
pub struct URustEditorSubsystem {
    __padding_end: [u8; 64],
}
impl URustEditorSubsystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustEditorSubsystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustEditorSubsystem")
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
