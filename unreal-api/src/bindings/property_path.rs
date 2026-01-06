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
pub static mut U_PROPERTY_PATH_TEST_OBJECT_SET_STRUCT_REF: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROPERTY_PATH_TEST_OBJECT_SET_STRUCT_CONST_REF: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROPERTY_PATH_TEST_OBJECT_SET_STRUCT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROPERTY_PATH_TEST_OBJECT_SET_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROPERTY_PATH_TEST_OBJECT_GET_STRUCT_REF: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROPERTY_PATH_TEST_OBJECT_GET_STRUCT_CONST_REF: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROPERTY_PATH_TEST_OBJECT_GET_STRUCT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROPERTY_PATH_TEST_OBJECT_GET_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPropertyPathTestObject::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStructRef"),
            &raw mut U_PROPERTY_PATH_TEST_OBJECT_SET_STRUCT_REF,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStructConstRef"),
            &raw mut U_PROPERTY_PATH_TEST_OBJECT_SET_STRUCT_CONST_REF,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStruct"),
            &raw mut U_PROPERTY_PATH_TEST_OBJECT_SET_STRUCT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFloat"),
            &raw mut U_PROPERTY_PATH_TEST_OBJECT_SET_FLOAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStructRef"),
            &raw mut U_PROPERTY_PATH_TEST_OBJECT_GET_STRUCT_REF,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStructConstRef"),
            &raw mut U_PROPERTY_PATH_TEST_OBJECT_GET_STRUCT_CONST_REF,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStruct"),
            &raw mut U_PROPERTY_PATH_TEST_OBJECT_GET_STRUCT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFloat"),
            &raw mut U_PROPERTY_PATH_TEST_OBJECT_GET_FLOAT,
        );
    }
}
#[repr(C, align(8))]
pub struct FPropertyPathTestInnerStruct {
    __padding_end: [u8; 40],
}
impl FPropertyPathTestInnerStruct {}
#[repr(C, align(8))]
pub struct FPropertyPathTestStruct {
    __padding_end: [u8; 96],
}
impl FPropertyPathTestStruct {}
#[repr(C, align(8))]
pub struct UPropertyPathTestObject {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub bool: bool,
    #[doc(hidden)]
    __padding_56: [u8; 4],
    pub integer: i32,
    pub string: FString,
    pub float: f32,
    pub _struct: FPropertyPathTestStruct,
    pub struct_ref: FPropertyPathTestStruct,
    pub struct_const_ref: FPropertyPathTestStruct,
    pub inner_object: UPtr<UPropertyPathTestObject>,
    __padding_end: [u8; 8],
}
impl UPropertyPathTestObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyPathTestObject")
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
pub struct EPropertyPathTestEnum(pub u8);
impl EPropertyPathTestEnum {
    pub const ONE: EPropertyPathTestEnum = EPropertyPathTestEnum(0);
    pub const TWO: EPropertyPathTestEnum = EPropertyPathTestEnum(1);
    pub const THREE: EPropertyPathTestEnum = EPropertyPathTestEnum(2);
    pub const FOUR: EPropertyPathTestEnum = EPropertyPathTestEnum(3);
}
