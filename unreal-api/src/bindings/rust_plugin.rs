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
    pub u_rust_extension_rust_class_def_add_numeric: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_rust_extension_rust_class_def_register: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_new: std::ptr::null_mut(),
            u_rust_extension_rust_class_def_add_numeric: std::ptr::null_mut(),
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
                unreal_ffi::Utf8Str::from("AddNumeric"),
                &raw mut __FUNCTION_PTRS.u_rust_extension_rust_class_def_add_numeric,
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
    pub fn add_numeric(def: &mut FRustClassDef, name: FString, offset: i32) {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_add_numeric,
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
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_add_numeric,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FRustClassDef>().swap(def);
        }
        std::mem::forget(name);
        std::mem::forget(offset);
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
pub struct URustType_Numeric {
    __padding_end: [u8; 48],
}
impl URustType_Numeric {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Numeric")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustType_Numeric")
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
    pub fn add_numeric(def: &mut FRustClassDef, name: FString, offset: i32) {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_add_numeric,
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
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_RustClassDef::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_rust_class_def_add_numeric,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FRustClassDef>().swap(def);
        }
        std::mem::forget(name);
        std::mem::forget(offset);
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
