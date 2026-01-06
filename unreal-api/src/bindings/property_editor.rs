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
pub static mut U_EDIT_CONDITION_TEST_OBJECT_VOID_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDIT_CONDITION_TEST_OBJECT_STATIC_VOID_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDIT_CONDITION_TEST_OBJECT_STATIC_GET_WEAK_OBJECT_PTR_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDIT_CONDITION_TEST_OBJECT_STATIC_GET_U_OBJECT_PTR_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDIT_CONDITION_TEST_OBJECT_STATIC_GET_UINT_BITFIELD_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDIT_CONDITION_TEST_OBJECT_STATIC_GET_SOFT_CLASS_PTR_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDIT_CONDITION_TEST_OBJECT_STATIC_GET_INTEGER_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDIT_CONDITION_TEST_OBJECT_STATIC_GET_ENUM_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDIT_CONDITION_TEST_OBJECT_STATIC_GET_DOUBLE_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDIT_CONDITION_TEST_OBJECT_STATIC_GET_BYTE_ENUM_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDIT_CONDITION_TEST_OBJECT_STATIC_GET_BOOL_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDIT_CONDITION_TEST_OBJECT_GET_WEAK_OBJECT_PTR_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDIT_CONDITION_TEST_OBJECT_GET_U_OBJECT_PTR_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDIT_CONDITION_TEST_OBJECT_GET_UINT_BITFIELD_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDIT_CONDITION_TEST_OBJECT_GET_SOFT_CLASS_PTR_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDIT_CONDITION_TEST_OBJECT_GET_INTEGER_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDIT_CONDITION_TEST_OBJECT_GET_ENUM_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDIT_CONDITION_TEST_OBJECT_GET_DOUBLE_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDIT_CONDITION_TEST_OBJECT_GET_BYTE_ENUM_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDIT_CONDITION_TEST_OBJECT_GET_BOOL_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditConditionTestObject::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("VoidFunction"),
            &raw mut U_EDIT_CONDITION_TEST_OBJECT_VOID_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StaticVoidFunction"),
            &raw mut U_EDIT_CONDITION_TEST_OBJECT_STATIC_VOID_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StaticGetWeakObjectPtrFunction"),
            &raw mut U_EDIT_CONDITION_TEST_OBJECT_STATIC_GET_WEAK_OBJECT_PTR_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StaticGetUObjectPtrFunction"),
            &raw mut U_EDIT_CONDITION_TEST_OBJECT_STATIC_GET_U_OBJECT_PTR_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StaticGetUintBitfieldFunction"),
            &raw mut U_EDIT_CONDITION_TEST_OBJECT_STATIC_GET_UINT_BITFIELD_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StaticGetSoftClassPtrFunction"),
            &raw mut U_EDIT_CONDITION_TEST_OBJECT_STATIC_GET_SOFT_CLASS_PTR_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StaticGetIntegerFunction"),
            &raw mut U_EDIT_CONDITION_TEST_OBJECT_STATIC_GET_INTEGER_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StaticGetEnumFunction"),
            &raw mut U_EDIT_CONDITION_TEST_OBJECT_STATIC_GET_ENUM_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StaticGetDoubleFunction"),
            &raw mut U_EDIT_CONDITION_TEST_OBJECT_STATIC_GET_DOUBLE_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StaticGetByteEnumFunction"),
            &raw mut U_EDIT_CONDITION_TEST_OBJECT_STATIC_GET_BYTE_ENUM_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StaticGetBoolFunction"),
            &raw mut U_EDIT_CONDITION_TEST_OBJECT_STATIC_GET_BOOL_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWeakObjectPtrFunction"),
            &raw mut U_EDIT_CONDITION_TEST_OBJECT_GET_WEAK_OBJECT_PTR_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUObjectPtrFunction"),
            &raw mut U_EDIT_CONDITION_TEST_OBJECT_GET_U_OBJECT_PTR_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUintBitfieldFunction"),
            &raw mut U_EDIT_CONDITION_TEST_OBJECT_GET_UINT_BITFIELD_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSoftClassPtrFunction"),
            &raw mut U_EDIT_CONDITION_TEST_OBJECT_GET_SOFT_CLASS_PTR_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIntegerFunction"),
            &raw mut U_EDIT_CONDITION_TEST_OBJECT_GET_INTEGER_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEnumFunction"),
            &raw mut U_EDIT_CONDITION_TEST_OBJECT_GET_ENUM_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDoubleFunction"),
            &raw mut U_EDIT_CONDITION_TEST_OBJECT_GET_DOUBLE_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetByteEnumFunction"),
            &raw mut U_EDIT_CONDITION_TEST_OBJECT_GET_BYTE_ENUM_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoolFunction"),
            &raw mut U_EDIT_CONDITION_TEST_OBJECT_GET_BOOL_FUNCTION,
        );
    }
}
#[repr(C, align(8))]
pub struct UDetailRowMenuContext {
    __padding_end: [u8; 112],
}
impl UDetailRowMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDetailRowMenuContext")
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
pub struct UDetailRowMenuContextPrivate {
    __padding_end: [u8; 64],
}
impl UDetailRowMenuContextPrivate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDetailRowMenuContextPrivate")
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
pub struct UDetailsViewPropertyHandleTestValueClass {
    __padding_end: [u8; 48],
}
impl UDetailsViewPropertyHandleTestValueClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDetailsViewPropertyHandleTestValueClass")
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
pub struct UDetailsViewPropertyHandleTestClass {
    __padding_end: [u8; 112],
}
impl UDetailsViewPropertyHandleTestClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDetailsViewPropertyHandleTestClass")
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
pub struct UDetailsConfig {
    __padding_end: [u8; 128],
}
impl UDetailsConfig {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDetailsConfig")
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
pub struct UEditConditionTestObject {
    __padding_end: [u8; 144],
}
impl UEditConditionTestObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditConditionTestObject")
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
pub struct UPropertyEditorSinglePropertyTestClass {
    __padding_end: [u8; 72],
}
impl UPropertyEditorSinglePropertyTestClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyEditorSinglePropertyTestClass")
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
