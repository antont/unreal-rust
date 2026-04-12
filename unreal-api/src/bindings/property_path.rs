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
    pub u_property_path_test_object_set_struct_ref: *mut crate::ffi::UFunctionOpague,
    pub u_property_path_test_object_set_struct_const_ref: *mut crate::ffi::UFunctionOpague,
    pub u_property_path_test_object_set_struct: *mut crate::ffi::UFunctionOpague,
    pub u_property_path_test_object_set_float: *mut crate::ffi::UFunctionOpague,
    pub u_property_path_test_object_get_struct_ref: *mut crate::ffi::UFunctionOpague,
    pub u_property_path_test_object_get_struct_const_ref: *mut crate::ffi::UFunctionOpague,
    pub u_property_path_test_object_get_struct: *mut crate::ffi::UFunctionOpague,
    pub u_property_path_test_object_get_float: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_property_path_test_object_set_struct_ref: std::ptr::null_mut(),
            u_property_path_test_object_set_struct_const_ref: std::ptr::null_mut(),
            u_property_path_test_object_set_struct: std::ptr::null_mut(),
            u_property_path_test_object_set_float: std::ptr::null_mut(),
            u_property_path_test_object_get_struct_ref: std::ptr::null_mut(),
            u_property_path_test_object_get_struct_const_ref: std::ptr::null_mut(),
            u_property_path_test_object_get_struct: std::ptr::null_mut(),
            u_property_path_test_object_get_float: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPropertyPathTestObject::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetStructRef"),
                &raw mut __FUNCTION_PTRS.u_property_path_test_object_set_struct_ref,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetStructConstRef"),
                &raw mut __FUNCTION_PTRS.u_property_path_test_object_set_struct_const_ref,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetStruct"),
                &raw mut __FUNCTION_PTRS.u_property_path_test_object_set_struct,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetFloat"),
                &raw mut __FUNCTION_PTRS.u_property_path_test_object_set_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetStructRef"),
                &raw mut __FUNCTION_PTRS.u_property_path_test_object_get_struct_ref,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetStructConstRef"),
                &raw mut __FUNCTION_PTRS.u_property_path_test_object_get_struct_const_ref,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetStruct"),
                &raw mut __FUNCTION_PTRS.u_property_path_test_object_get_struct,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFloat"),
                &raw mut __FUNCTION_PTRS.u_property_path_test_object_get_float,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FPropertyPathTestInnerStruct {
    pub(crate) __padding_end: [u8; 40],
}
impl FPropertyPathTestInnerStruct {}
#[repr(C, align(8))]
pub struct FPropertyPathTestStruct {
    pub(crate) __padding_end: [u8; 96],
}
impl FPropertyPathTestStruct {}
#[repr(C, align(8))]
pub struct UPropertyPathTestObject {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub bool: bool,
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 4],
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
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyPathTestObject")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyPathTestObject")
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
pub struct EPropertyPathTestEnum(pub u8);
impl EPropertyPathTestEnum {
    pub const ONE: EPropertyPathTestEnum = EPropertyPathTestEnum(0);
    pub const TWO: EPropertyPathTestEnum = EPropertyPathTestEnum(1);
    pub const THREE: EPropertyPathTestEnum = EPropertyPathTestEnum(2);
    pub const FOUR: EPropertyPathTestEnum = EPropertyPathTestEnum(3);
}
