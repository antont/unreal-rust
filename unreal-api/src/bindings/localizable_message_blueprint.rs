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
pub static mut U_LOCALIZABLE_MESSAGE_LIBRARY_RESET_LOCALIZABLE_MESSAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LOCALIZABLE_MESSAGE_LIBRARY_IS_EMPTY_LOCALIZABLE_MESSAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LOCALIZABLE_MESSAGE_LIBRARY_EQUAL_EQUAL_LOCALIZABLE_MESSAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LOCALIZABLE_MESSAGE_LIBRARY_CONV_LOCALIZABLE_MESSAGE_TO_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULocalizableMessageLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Reset_LocalizableMessage"),
            &raw mut U_LOCALIZABLE_MESSAGE_LIBRARY_RESET_LOCALIZABLE_MESSAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsEmpty_LocalizableMessage"),
            &raw mut U_LOCALIZABLE_MESSAGE_LIBRARY_IS_EMPTY_LOCALIZABLE_MESSAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EqualEqual_LocalizableMessage"),
            &raw mut U_LOCALIZABLE_MESSAGE_LIBRARY_EQUAL_EQUAL_LOCALIZABLE_MESSAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_LocalizableMessageToText"),
            &raw mut U_LOCALIZABLE_MESSAGE_LIBRARY_CONV_LOCALIZABLE_MESSAGE_TO_TEXT,
        );
    }
}
#[repr(C, align(8))]
pub struct ULocalizableMessageLibrary {
    __padding_end: [u8; 48],
}
impl ULocalizableMessageLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULocalizableMessageLibrary")
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
