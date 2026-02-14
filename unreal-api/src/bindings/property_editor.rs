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
    pub u_edit_condition_test_object_void_function: *mut crate::ffi::UFunctionOpague,
    pub u_edit_condition_test_object_static_void_function: *mut crate::ffi::UFunctionOpague,
    pub u_edit_condition_test_object_static_get_weak_object_ptr_function: *mut crate::ffi::UFunctionOpague,
    pub u_edit_condition_test_object_static_get_u_object_ptr_function: *mut crate::ffi::UFunctionOpague,
    pub u_edit_condition_test_object_static_get_uint_bitfield_function: *mut crate::ffi::UFunctionOpague,
    pub u_edit_condition_test_object_static_get_soft_class_ptr_function: *mut crate::ffi::UFunctionOpague,
    pub u_edit_condition_test_object_static_get_integer_function: *mut crate::ffi::UFunctionOpague,
    pub u_edit_condition_test_object_static_get_enum_function: *mut crate::ffi::UFunctionOpague,
    pub u_edit_condition_test_object_static_get_double_function: *mut crate::ffi::UFunctionOpague,
    pub u_edit_condition_test_object_static_get_byte_enum_function: *mut crate::ffi::UFunctionOpague,
    pub u_edit_condition_test_object_static_get_bool_function: *mut crate::ffi::UFunctionOpague,
    pub u_edit_condition_test_object_get_weak_object_ptr_function: *mut crate::ffi::UFunctionOpague,
    pub u_edit_condition_test_object_get_u_object_ptr_function: *mut crate::ffi::UFunctionOpague,
    pub u_edit_condition_test_object_get_uint_bitfield_function: *mut crate::ffi::UFunctionOpague,
    pub u_edit_condition_test_object_get_soft_class_ptr_function: *mut crate::ffi::UFunctionOpague,
    pub u_edit_condition_test_object_get_integer_function: *mut crate::ffi::UFunctionOpague,
    pub u_edit_condition_test_object_get_enum_function: *mut crate::ffi::UFunctionOpague,
    pub u_edit_condition_test_object_get_double_function: *mut crate::ffi::UFunctionOpague,
    pub u_edit_condition_test_object_get_byte_enum_function: *mut crate::ffi::UFunctionOpague,
    pub u_edit_condition_test_object_get_bool_function: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_edit_condition_test_object_void_function: std::ptr::null_mut(),
            u_edit_condition_test_object_static_void_function: std::ptr::null_mut(),
            u_edit_condition_test_object_static_get_weak_object_ptr_function: std::ptr::null_mut(),
            u_edit_condition_test_object_static_get_u_object_ptr_function: std::ptr::null_mut(),
            u_edit_condition_test_object_static_get_uint_bitfield_function: std::ptr::null_mut(),
            u_edit_condition_test_object_static_get_soft_class_ptr_function: std::ptr::null_mut(),
            u_edit_condition_test_object_static_get_integer_function: std::ptr::null_mut(),
            u_edit_condition_test_object_static_get_enum_function: std::ptr::null_mut(),
            u_edit_condition_test_object_static_get_double_function: std::ptr::null_mut(),
            u_edit_condition_test_object_static_get_byte_enum_function: std::ptr::null_mut(),
            u_edit_condition_test_object_static_get_bool_function: std::ptr::null_mut(),
            u_edit_condition_test_object_get_weak_object_ptr_function: std::ptr::null_mut(),
            u_edit_condition_test_object_get_u_object_ptr_function: std::ptr::null_mut(),
            u_edit_condition_test_object_get_uint_bitfield_function: std::ptr::null_mut(),
            u_edit_condition_test_object_get_soft_class_ptr_function: std::ptr::null_mut(),
            u_edit_condition_test_object_get_integer_function: std::ptr::null_mut(),
            u_edit_condition_test_object_get_enum_function: std::ptr::null_mut(),
            u_edit_condition_test_object_get_double_function: std::ptr::null_mut(),
            u_edit_condition_test_object_get_byte_enum_function: std::ptr::null_mut(),
            u_edit_condition_test_object_get_bool_function: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UEditConditionTestObject::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("VoidFunction"),
                &raw mut __FUNCTION_PTRS.u_edit_condition_test_object_void_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StaticVoidFunction"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_condition_test_object_static_void_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StaticGetWeakObjectPtrFunction"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_condition_test_object_static_get_weak_object_ptr_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StaticGetUObjectPtrFunction"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_condition_test_object_static_get_u_object_ptr_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StaticGetUintBitfieldFunction"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_condition_test_object_static_get_uint_bitfield_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StaticGetSoftClassPtrFunction"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_condition_test_object_static_get_soft_class_ptr_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StaticGetIntegerFunction"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_condition_test_object_static_get_integer_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StaticGetEnumFunction"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_condition_test_object_static_get_enum_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StaticGetDoubleFunction"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_condition_test_object_static_get_double_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StaticGetByteEnumFunction"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_condition_test_object_static_get_byte_enum_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StaticGetBoolFunction"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_condition_test_object_static_get_bool_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWeakObjectPtrFunction"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_condition_test_object_get_weak_object_ptr_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUObjectPtrFunction"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_condition_test_object_get_u_object_ptr_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUintBitfieldFunction"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_condition_test_object_get_uint_bitfield_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSoftClassPtrFunction"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_condition_test_object_get_soft_class_ptr_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetIntegerFunction"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_condition_test_object_get_integer_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEnumFunction"),
                &raw mut __FUNCTION_PTRS.u_edit_condition_test_object_get_enum_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDoubleFunction"),
                &raw mut __FUNCTION_PTRS.u_edit_condition_test_object_get_double_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetByteEnumFunction"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_condition_test_object_get_byte_enum_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBoolFunction"),
                &raw mut __FUNCTION_PTRS.u_edit_condition_test_object_get_bool_function,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct UDetailRowMenuContext {
    __padding_end: [u8; 112],
}
impl UDetailRowMenuContext {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDetailRowMenuContext")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDetailRowMenuContext")
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
pub struct UDetailRowMenuContextPrivate {
    __padding_end: [u8; 64],
}
impl UDetailRowMenuContextPrivate {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDetailRowMenuContextPrivate")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDetailRowMenuContextPrivate")
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
pub struct UDetailsViewPropertyHandleTestValueClass {
    __padding_end: [u8; 48],
}
impl UDetailsViewPropertyHandleTestValueClass {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDetailsViewPropertyHandleTestValueClass")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDetailsViewPropertyHandleTestValueClass")
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
pub struct UDetailsViewPropertyHandleTestClass {
    __padding_end: [u8; 112],
}
impl UDetailsViewPropertyHandleTestClass {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDetailsViewPropertyHandleTestClass")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDetailsViewPropertyHandleTestClass")
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
pub struct UDetailsConfig {
    __padding_end: [u8; 128],
}
impl UDetailsConfig {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDetailsConfig")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDetailsConfig")
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
pub struct UEditConditionTestObject {
    __padding_end: [u8; 144],
}
impl UEditConditionTestObject {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditConditionTestObject")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditConditionTestObject")
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
pub struct UPropertyEditorSinglePropertyTestClass {
    __padding_end: [u8; 72],
}
impl UPropertyEditorSinglePropertyTestClass {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyEditorSinglePropertyTestClass")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyEditorSinglePropertyTestClass")
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
#[repr(transparent)]
pub struct EditConditionByteEnum(pub u8);
impl EditConditionByteEnum {
    pub const FIRST: EditConditionByteEnum = EditConditionByteEnum(15);
    pub const SECOND: EditConditionByteEnum = EditConditionByteEnum(31);
}
#[repr(transparent)]
pub struct EditConditionTestEnum(pub i32);
impl EditConditionTestEnum {
    pub const FIRST: EditConditionTestEnum = EditConditionTestEnum(15);
    pub const SECOND: EditConditionTestEnum = EditConditionTestEnum(31);
}
