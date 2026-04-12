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
    pub u_dataflow_blueprint_library_regenerate_asset_from_dataflow: *mut crate::ffi::UFunctionOpague,
    pub u_dataflow_blueprint_library_override_dataflow_variable_object_array: *mut crate::ffi::UFunctionOpague,
    pub u_dataflow_blueprint_library_override_dataflow_variable_object: *mut crate::ffi::UFunctionOpague,
    pub u_dataflow_blueprint_library_override_dataflow_variable_int_array: *mut crate::ffi::UFunctionOpague,
    pub u_dataflow_blueprint_library_override_dataflow_variable_int: *mut crate::ffi::UFunctionOpague,
    pub u_dataflow_blueprint_library_override_dataflow_variable_float_array: *mut crate::ffi::UFunctionOpague,
    pub u_dataflow_blueprint_library_override_dataflow_variable_float: *mut crate::ffi::UFunctionOpague,
    pub u_dataflow_blueprint_library_override_dataflow_variable_bool_array: *mut crate::ffi::UFunctionOpague,
    pub u_dataflow_blueprint_library_override_dataflow_variable_bool: *mut crate::ffi::UFunctionOpague,
    pub u_dataflow_blueprint_library_evaluate_terminal_node_by_name: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_dataflow_blueprint_library_regenerate_asset_from_dataflow: std::ptr::null_mut(),
            u_dataflow_blueprint_library_override_dataflow_variable_object_array: std::ptr::null_mut(),
            u_dataflow_blueprint_library_override_dataflow_variable_object: std::ptr::null_mut(),
            u_dataflow_blueprint_library_override_dataflow_variable_int_array: std::ptr::null_mut(),
            u_dataflow_blueprint_library_override_dataflow_variable_int: std::ptr::null_mut(),
            u_dataflow_blueprint_library_override_dataflow_variable_float_array: std::ptr::null_mut(),
            u_dataflow_blueprint_library_override_dataflow_variable_float: std::ptr::null_mut(),
            u_dataflow_blueprint_library_override_dataflow_variable_bool_array: std::ptr::null_mut(),
            u_dataflow_blueprint_library_override_dataflow_variable_bool: std::ptr::null_mut(),
            u_dataflow_blueprint_library_evaluate_terminal_node_by_name: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UDataflowBlueprintLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RegenerateAssetFromDataflow"),
                &raw mut __FUNCTION_PTRS
                    .u_dataflow_blueprint_library_regenerate_asset_from_dataflow,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OverrideDataflowVariableObjectArray"),
                &raw mut __FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_object_array,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OverrideDataflowVariableObject"),
                &raw mut __FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OverrideDataflowVariableIntArray"),
                &raw mut __FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_int_array,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OverrideDataflowVariableInt"),
                &raw mut __FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_int,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OverrideDataflowVariableFloatArray"),
                &raw mut __FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_float_array,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OverrideDataflowVariableFloat"),
                &raw mut __FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OverrideDataflowVariableBoolArray"),
                &raw mut __FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_bool_array,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OverrideDataflowVariableBool"),
                &raw mut __FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_bool,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EvaluateTerminalNodeByName"),
                &raw mut __FUNCTION_PTRS
                    .u_dataflow_blueprint_library_evaluate_terminal_node_by_name,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FStringValuePair {
    pub key: FString,
    pub value: FString,
}
impl FStringValuePair {}
#[repr(C, align(8))]
pub struct UDataflowBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UDataflowBlueprintLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowBlueprintLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowBlueprintLibrary")
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
    pub fn regenerate_asset_from_dataflow(
        asset_to_regenerate: UPtr<crate::bindings::core_u_object::UObject>,
        b_regenerate_dependent_assets: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::dataflow_engine::__FUNCTION_PTRS
                    .u_dataflow_blueprint_library_regenerate_asset_from_dataflow,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_to_regenerate,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_regenerate_dependent_assets,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::dataflow_engine::UDataflowBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::dataflow_engine::__FUNCTION_PTRS
                    .u_dataflow_blueprint_library_regenerate_asset_from_dataflow,
                __buffer,
            )
        };
        std::mem::forget(asset_to_regenerate);
        std::mem::forget(b_regenerate_dependent_assets);
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn override_dataflow_variable_object_array(
        asset: UPtr<crate::bindings::core_u_object::UObject>,
        variable_name: FName,
        variable_array_value: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::dataflow_engine::__FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_object_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                variable_array_value,
                __buffer
                    .add(24)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::dataflow_engine::UDataflowBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::dataflow_engine::__FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_object_array,
                __buffer,
            )
        };
        std::mem::forget(asset);
        std::mem::forget(variable_name);
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn override_dataflow_variable_object(
        asset: UPtr<crate::bindings::core_u_object::UObject>,
        variable_name: FName,
        variable_value: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::dataflow_engine::__FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable_value,
                __buffer.add(24).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::dataflow_engine::UDataflowBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::dataflow_engine::__FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_object,
                __buffer,
            )
        };
        std::mem::forget(asset);
        std::mem::forget(variable_name);
        std::mem::forget(variable_value);
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn override_dataflow_variable_int_array(
        asset: UPtr<crate::bindings::core_u_object::UObject>,
        variable_name: FName,
        variable_array_value: &TArray<i32>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::dataflow_engine::__FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_int_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                variable_array_value,
                __buffer.add(24).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::dataflow_engine::UDataflowBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::dataflow_engine::__FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_int_array,
                __buffer,
            )
        };
        std::mem::forget(asset);
        std::mem::forget(variable_name);
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn override_dataflow_variable_int(
        asset: UPtr<crate::bindings::core_u_object::UObject>,
        variable_name: FName,
        variable_value: i64,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::dataflow_engine::__FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable_value,
                __buffer.add(24).cast::<i64>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::dataflow_engine::UDataflowBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::dataflow_engine::__FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_int,
                __buffer,
            )
        };
        std::mem::forget(asset);
        std::mem::forget(variable_name);
        std::mem::forget(variable_value);
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn override_dataflow_variable_float_array(
        asset: UPtr<crate::bindings::core_u_object::UObject>,
        variable_name: FName,
        variable_array_value: &TArray<f32>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::dataflow_engine::__FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_float_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                variable_array_value,
                __buffer.add(24).cast::<TArray<f32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::dataflow_engine::UDataflowBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::dataflow_engine::__FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_float_array,
                __buffer,
            )
        };
        std::mem::forget(asset);
        std::mem::forget(variable_name);
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn override_dataflow_variable_float(
        asset: UPtr<crate::bindings::core_u_object::UObject>,
        variable_name: FName,
        variable_value: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::dataflow_engine::__FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable_value,
                __buffer.add(20).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::dataflow_engine::UDataflowBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::dataflow_engine::__FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_float,
                __buffer,
            )
        };
        std::mem::forget(asset);
        std::mem::forget(variable_name);
        std::mem::forget(variable_value);
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn override_dataflow_variable_bool_array(
        asset: UPtr<crate::bindings::core_u_object::UObject>,
        variable_name: FName,
        variable_array_value: &TArray<bool>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::dataflow_engine::__FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_bool_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                variable_array_value,
                __buffer.add(24).cast::<TArray<bool>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::dataflow_engine::UDataflowBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::dataflow_engine::__FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_bool_array,
                __buffer,
            )
        };
        std::mem::forget(asset);
        std::mem::forget(variable_name);
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn override_dataflow_variable_bool(
        asset: UPtr<crate::bindings::core_u_object::UObject>,
        variable_name: FName,
        variable_value: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<22>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::dataflow_engine::__FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable_value,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::dataflow_engine::UDataflowBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::dataflow_engine::__FUNCTION_PTRS
                    .u_dataflow_blueprint_library_override_dataflow_variable_bool,
                __buffer,
            )
        };
        std::mem::forget(asset);
        std::mem::forget(variable_name);
        std::mem::forget(variable_value);
        unsafe { __buffer.add(21).cast::<bool>().read() }
    }
    pub fn evaluate_terminal_node_by_name(
        dataflow: UPtr<UDataflow>,
        terminal_node_name: FName,
        result_asset: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::dataflow_engine::__FUNCTION_PTRS
                    .u_dataflow_blueprint_library_evaluate_terminal_node_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &dataflow,
                __buffer.add(0).cast::<UPtr<UDataflow>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &terminal_node_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &result_asset,
                __buffer.add(24).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::dataflow_engine::UDataflowBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::dataflow_engine::__FUNCTION_PTRS
                    .u_dataflow_blueprint_library_evaluate_terminal_node_by_name,
                __buffer,
            )
        };
        std::mem::forget(dataflow);
        std::mem::forget(terminal_node_name);
        std::mem::forget(result_asset);
    }
}
pub struct IDataflowContentOwner {}
#[repr(C, align(8))]
pub struct UDataflowContentOwner {
    __padding_end: [u8; 48],
}
impl UDataflowContentOwner {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowContentOwner")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowContentOwner")
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
pub struct UDataflowContextObject {
    __padding_end: [u8; 128],
}
impl UDataflowContextObject {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowContextObject")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowContextObject")
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
pub struct UDataflowBaseContent {
    __padding_end: [u8; 208],
}
impl UDataflowBaseContent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowBaseContent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowBaseContent")
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
pub struct UDataflowSkeletalContent {
    __padding_end: [u8; 232],
}
impl UDataflowSkeletalContent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowSkeletalContent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowSkeletalContent")
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
#[repr(C, align(16))]
pub struct UDataflowDebugDrawComponent {
    __padding_end: [u8; 1616],
}
impl UDataflowDebugDrawComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowDebugDrawComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowDebugDrawComponent")
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
pub struct UDataflowEdNode {
    __padding_end: [u8; 256],
}
impl UDataflowEdNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEdNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEdNode")
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
pub struct IDataflowInstanceInterface {}
#[repr(C, align(8))]
pub struct UDataflowInstanceInterface {
    __padding_end: [u8; 48],
}
impl UDataflowInstanceInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowInstanceInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowInstanceInterface")
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
pub struct UDataflowMesh {
    __padding_end: [u8; 72],
}
impl UDataflowMesh {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowMesh")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowMesh")
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
#[repr(C, align(16))]
pub struct UDataflow {
    __padding_end: [u8; 720],
}
impl UDataflow {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflow")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UDataflow").copied()
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
pub struct UDataflowSubGraph {
    __padding_end: [u8; 216],
}
impl UDataflowSubGraph {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowSubGraph")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowSubGraph")
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
pub struct EDataflowDebugDrawRenderType(pub u8);
impl EDataflowDebugDrawRenderType {
    pub const WIREFRAME: EDataflowDebugDrawRenderType = EDataflowDebugDrawRenderType(0);
    pub const SHADED: EDataflowDebugDrawRenderType = EDataflowDebugDrawRenderType(1);
}
#[repr(transparent)]
pub struct EDataflowSphereCoveringColorMethod(pub u8);
impl EDataflowSphereCoveringColorMethod {
    pub const SINGLE: EDataflowSphereCoveringColorMethod = EDataflowSphereCoveringColorMethod(
        0,
    );
    pub const COLOR_BY_RADIUS: EDataflowSphereCoveringColorMethod = EDataflowSphereCoveringColorMethod(
        1,
    );
    pub const RANDOM: EDataflowSphereCoveringColorMethod = EDataflowSphereCoveringColorMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EDataflowType(pub u8);
impl EDataflowType {
    pub const CONSTRUCTION: EDataflowType = EDataflowType(0);
    pub const SIMULATION: EDataflowType = EDataflowType(1);
}
