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
    pub fn get_config_data_text_with_default(
        entry_name: FString,
        default_value: FText,
    ) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_TEXT_WITH_DEFAULT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &entry_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &default_value,
                __buffer.add(16).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::global_configuration_data::UGlobalConfigurationDataBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_TEXT_WITH_DEFAULT,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FText>().read() }
    }
    pub fn get_config_data_text(entry_name: FString, value_out: &mut FText) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_TEXT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &entry_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value_out,
                __buffer.add(16).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::global_configuration_data::UGlobalConfigurationDataBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_TEXT,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FText>().swap(value_out);
        }
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_config_data_struct(
        entry_name: FString,
        struct_type: UPtr<crate::bindings::core_u_object::UScriptStruct>,
        value_out: &mut crate::bindings::core_u_object::FInstancedStruct,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_STRUCT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &entry_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &struct_type,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value_out,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::core_u_object::FInstancedStruct>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::global_configuration_data::UGlobalConfigurationDataBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_STRUCT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FInstancedStruct>()
                .swap(value_out);
        }
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn get_config_data_string_with_default(
        entry_name: FString,
        default_value: FString,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_STRING_WITH_DEFAULT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &entry_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &default_value,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::global_configuration_data::UGlobalConfigurationDataBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_STRING_WITH_DEFAULT,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FString>().read() }
    }
    pub fn get_config_data_string(entry_name: FString, value_out: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_STRING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &entry_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value_out,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::global_configuration_data::UGlobalConfigurationDataBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_STRING,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FString>().swap(value_out);
        }
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_config_data_object(
        entry_name: FString,
        value_in_out: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_OBJECT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &entry_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value_in_out,
                __buffer.add(16).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::global_configuration_data::UGlobalConfigurationDataBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_OBJECT,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_config_data_int_with_default(
        entry_name: FString,
        default_value: i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_INT_WITH_DEFAULT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &entry_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &default_value,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::global_configuration_data::UGlobalConfigurationDataBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_INT_WITH_DEFAULT,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<i32>().read() }
    }
    pub fn get_config_data_int(entry_name: FString, value_out: &mut i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_INT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &entry_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value_out, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::global_configuration_data::UGlobalConfigurationDataBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_INT,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<i32>().swap(value_out);
        }
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_config_data_float_with_default(
        entry_name: FString,
        default_value: f32,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_FLOAT_WITH_DEFAULT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &entry_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &default_value,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::global_configuration_data::UGlobalConfigurationDataBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_FLOAT_WITH_DEFAULT,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<f32>().read() }
    }
    pub fn get_config_data_float(entry_name: FString, value_out: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_FLOAT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &entry_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value_out, __buffer.add(16).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::global_configuration_data::UGlobalConfigurationDataBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_FLOAT,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<f32>().swap(value_out);
        }
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_config_data_bool_with_default(
        entry_name: FString,
        b_default_value: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_BOOL_WITH_DEFAULT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &entry_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_default_value,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::global_configuration_data::UGlobalConfigurationDataBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_BOOL_WITH_DEFAULT,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn get_config_data_bool(entry_name: FString, b_value_out: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_BOOL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &entry_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_value_out,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::global_configuration_data::UGlobalConfigurationDataBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::global_configuration_data::U_GLOBAL_CONFIGURATION_DATA_BLUEPRINT_LIBRARY_GET_CONFIG_DATA_BOOL,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(b_value_out);
        }
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
}
