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
pub static mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_TEXT_WITH_DEFAULT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_STRUCT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_STRING_WITH_DEFAULT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_INT_WITH_DEFAULT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_INT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_FLOAT_WITH_DEFAULT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_BOOL_WITH_DEFAULT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_BOOL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGlobalConfigurationDataBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConfigDataTextWithDefault"),
            &raw mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_TEXT_WITH_DEFAULT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConfigDataText"),
            &raw mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConfigDataStruct"),
            &raw mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_STRUCT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConfigDataStringWithDefault"),
            &raw mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_STRING_WITH_DEFAULT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConfigDataString"),
            &raw mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_STRING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConfigDataObject"),
            &raw mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConfigDataIntWithDefault"),
            &raw mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_INT_WITH_DEFAULT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConfigDataInt"),
            &raw mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_INT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConfigDataFloatWithDefault"),
            &raw mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_FLOAT_WITH_DEFAULT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConfigDataFloat"),
            &raw mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_FLOAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConfigDataBoolWithDefault"),
            &raw mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_BOOL_WITH_DEFAULT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConfigDataBool"),
            &raw mut U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_BOOL,
        );
    }
}
#[repr(C, align(8))]
pub struct UGlobalConfigurationDataBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UGlobalConfigurationDataBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGlobalConfigurationDataBlueprintLibrary")
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
