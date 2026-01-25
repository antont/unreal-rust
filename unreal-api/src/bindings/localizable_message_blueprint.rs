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
    pub u_localizable_message_library_reset_localizable_message: *mut crate::ffi::UFunctionOpague,
    pub u_localizable_message_library_is_empty_localizable_message: *mut crate::ffi::UFunctionOpague,
    pub u_localizable_message_library_equal_equal_localizable_message: *mut crate::ffi::UFunctionOpague,
    pub u_localizable_message_library_conv_localizable_message_to_text: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_localizable_message_library_reset_localizable_message: std::ptr::null_mut(),
            u_localizable_message_library_is_empty_localizable_message: std::ptr::null_mut(),
            u_localizable_message_library_equal_equal_localizable_message: std::ptr::null_mut(),
            u_localizable_message_library_conv_localizable_message_to_text: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULocalizableMessageLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Reset_LocalizableMessage"),
            &raw mut __FUNCTION_PTRS
                .u_localizable_message_library_reset_localizable_message,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsEmpty_LocalizableMessage"),
            &raw mut __FUNCTION_PTRS
                .u_localizable_message_library_is_empty_localizable_message,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EqualEqual_LocalizableMessage"),
            &raw mut __FUNCTION_PTRS
                .u_localizable_message_library_equal_equal_localizable_message,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_LocalizableMessageToText"),
            &raw mut __FUNCTION_PTRS
                .u_localizable_message_library_conv_localizable_message_to_text,
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
    pub fn reset_localizable_message(
        message: &mut crate::bindings::localizable_message::FLocalizableMessage,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::localizable_message_blueprint::__FUNCTION_PTRS
                    .u_localizable_message_library_reset_localizable_message,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                message,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::localizable_message::FLocalizableMessage>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::localizable_message_blueprint::ULocalizableMessageLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::localizable_message_blueprint::__FUNCTION_PTRS
                    .u_localizable_message_library_reset_localizable_message,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::localizable_message::FLocalizableMessage>()
                .swap(message);
        }
    }
    pub fn is_empty_localizable_message(
        message: &crate::bindings::localizable_message::FLocalizableMessage,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::localizable_message_blueprint::__FUNCTION_PTRS
                    .u_localizable_message_library_is_empty_localizable_message,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                message,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::localizable_message::FLocalizableMessage>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::localizable_message_blueprint::ULocalizableMessageLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::localizable_message_blueprint::__FUNCTION_PTRS
                    .u_localizable_message_library_is_empty_localizable_message,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn equal_equal_localizable_message(
        a: &crate::bindings::localizable_message::FLocalizableMessage,
        b: &crate::bindings::localizable_message::FLocalizableMessage,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::localizable_message_blueprint::__FUNCTION_PTRS
                    .u_localizable_message_library_equal_equal_localizable_message,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                a,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::localizable_message::FLocalizableMessage>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::localizable_message::FLocalizableMessage>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::localizable_message_blueprint::ULocalizableMessageLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::localizable_message_blueprint::__FUNCTION_PTRS
                    .u_localizable_message_library_equal_equal_localizable_message,
                __buffer,
            )
        };
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn conv_localizable_message_to_text(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        message: &crate::bindings::localizable_message::FLocalizableMessage,
    ) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::localizable_message_blueprint::__FUNCTION_PTRS
                    .u_localizable_message_library_conv_localizable_message_to_text,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                message,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::localizable_message::FLocalizableMessage>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::localizable_message_blueprint::ULocalizableMessageLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::localizable_message_blueprint::__FUNCTION_PTRS
                    .u_localizable_message_library_conv_localizable_message_to_text,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<FText>().read() }
    }
}
