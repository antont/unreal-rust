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
    pub u_py_test_interface_func_interface: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_child_interface_func_interface_child: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_other_interface_func_interface_other: *mut crate::ffi::UFunctionOpague,
    pub u_editor_python_scripting_library_set_keep_python_script_alive: *mut crate::ffi::UFunctionOpague,
    pub u_editor_python_scripting_library_get_keep_python_script_alive: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_struct_library_set_bool_mutable_via_ref: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_struct_library_set_bool_mutable: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_struct_library_legacy_is_bool_set: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_struct_library_is_bool_set: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_struct_library_get_constant_value: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_struct_library_clear_bool_mutable_via_ref: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_struct_library_clear_bool_mutable: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_struct_library_add_str: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_struct_library_add_int: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_struct_library_add_float: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_return_set: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_return_map: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_return_field_path: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_return_array: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_multicast_delegate_property_callback: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_legacy_func_taking_py_test_struct: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_get_constant_value: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_func_taking_py_test_struct_default: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_func_taking_py_test_struct: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_func_taking_py_test_delegate: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_func_taking_py_test_child_struct: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_func_taking_field_path: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_func_blueprint_native_ref: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_func_blueprint_native: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_func_blueprint_implementable_packed_getter: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_func_blueprint_implementable: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_emit_script_warning: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_emit_script_error: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_delegate_property_callback: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_call_func_blueprint_native_ref: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_call_func_blueprint_native: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_call_func_blueprint_implementable_packed_getter: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_call_func_blueprint_implementable: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_library_is_bool_set: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_object_library_get_other_constant_value: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_struct_delegate_on_name_collision_delegate_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_vector_delegate_on_name_collision_delegate_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_type_hint_get_string_const: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_type_hint_get_int_const: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_type_hint_check_tuple_return_type: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_type_hint_check_text_type_hints: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_type_hint_check_struct_type_hints: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_type_hint_check_string_type_hints: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_type_hint_check_static_function: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_type_hint_check_set_type_hints: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_type_hint_check_object_type_hints: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_type_hint_check_name_type_hints: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_type_hint_check_map_type_hints: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_type_hint_check_integer_type_hints: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_type_hint_check_float_type_hints: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_type_hint_check_field_path_type_hints: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_type_hint_check_enum_type_hints: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_type_hint_check_delegate_type_hints: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_type_hint_check_bool_type_hints: *mut crate::ffi::UFunctionOpague,
    pub u_py_test_type_hint_check_array_type_hints: *mut crate::ffi::UFunctionOpague,
    pub u_python_script_library_is_python_initialized: *mut crate::ffi::UFunctionOpague,
    pub u_python_script_library_is_python_configured: *mut crate::ffi::UFunctionOpague,
    pub u_python_script_library_is_python_available: *mut crate::ffi::UFunctionOpague,
    pub u_python_script_library_force_enable_python_at_runtime: *mut crate::ffi::UFunctionOpague,
    pub u_python_script_library_execute_python_script: *mut crate::ffi::UFunctionOpague,
    pub u_python_script_library_execute_python_command_ex: *mut crate::ffi::UFunctionOpague,
    pub u_python_script_library_execute_python_command: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_py_test_interface_func_interface: std::ptr::null_mut(),
            u_py_test_child_interface_func_interface_child: std::ptr::null_mut(),
            u_py_test_other_interface_func_interface_other: std::ptr::null_mut(),
            u_editor_python_scripting_library_set_keep_python_script_alive: std::ptr::null_mut(),
            u_editor_python_scripting_library_get_keep_python_script_alive: std::ptr::null_mut(),
            u_py_test_struct_library_set_bool_mutable_via_ref: std::ptr::null_mut(),
            u_py_test_struct_library_set_bool_mutable: std::ptr::null_mut(),
            u_py_test_struct_library_legacy_is_bool_set: std::ptr::null_mut(),
            u_py_test_struct_library_is_bool_set: std::ptr::null_mut(),
            u_py_test_struct_library_get_constant_value: std::ptr::null_mut(),
            u_py_test_struct_library_clear_bool_mutable_via_ref: std::ptr::null_mut(),
            u_py_test_struct_library_clear_bool_mutable: std::ptr::null_mut(),
            u_py_test_struct_library_add_str: std::ptr::null_mut(),
            u_py_test_struct_library_add_int: std::ptr::null_mut(),
            u_py_test_struct_library_add_float: std::ptr::null_mut(),
            u_py_test_object_return_set: std::ptr::null_mut(),
            u_py_test_object_return_map: std::ptr::null_mut(),
            u_py_test_object_return_field_path: std::ptr::null_mut(),
            u_py_test_object_return_array: std::ptr::null_mut(),
            u_py_test_object_multicast_delegate_property_callback: std::ptr::null_mut(),
            u_py_test_object_legacy_func_taking_py_test_struct: std::ptr::null_mut(),
            u_py_test_object_get_constant_value: std::ptr::null_mut(),
            u_py_test_object_func_taking_py_test_struct_default: std::ptr::null_mut(),
            u_py_test_object_func_taking_py_test_struct: std::ptr::null_mut(),
            u_py_test_object_func_taking_py_test_delegate: std::ptr::null_mut(),
            u_py_test_object_func_taking_py_test_child_struct: std::ptr::null_mut(),
            u_py_test_object_func_taking_field_path: std::ptr::null_mut(),
            u_py_test_object_func_blueprint_native_ref: std::ptr::null_mut(),
            u_py_test_object_func_blueprint_native: std::ptr::null_mut(),
            u_py_test_object_func_blueprint_implementable_packed_getter: std::ptr::null_mut(),
            u_py_test_object_func_blueprint_implementable: std::ptr::null_mut(),
            u_py_test_object_emit_script_warning: std::ptr::null_mut(),
            u_py_test_object_emit_script_error: std::ptr::null_mut(),
            u_py_test_object_delegate_property_callback: std::ptr::null_mut(),
            u_py_test_object_call_func_blueprint_native_ref: std::ptr::null_mut(),
            u_py_test_object_call_func_blueprint_native: std::ptr::null_mut(),
            u_py_test_object_call_func_blueprint_implementable_packed_getter: std::ptr::null_mut(),
            u_py_test_object_call_func_blueprint_implementable: std::ptr::null_mut(),
            u_py_test_object_library_is_bool_set: std::ptr::null_mut(),
            u_py_test_object_library_get_other_constant_value: std::ptr::null_mut(),
            u_py_test_struct_delegate_on_name_collision_delegate_delegate_signature: std::ptr::null_mut(),
            u_py_test_vector_delegate_on_name_collision_delegate_delegate_signature: std::ptr::null_mut(),
            u_py_test_type_hint_get_string_const: std::ptr::null_mut(),
            u_py_test_type_hint_get_int_const: std::ptr::null_mut(),
            u_py_test_type_hint_check_tuple_return_type: std::ptr::null_mut(),
            u_py_test_type_hint_check_text_type_hints: std::ptr::null_mut(),
            u_py_test_type_hint_check_struct_type_hints: std::ptr::null_mut(),
            u_py_test_type_hint_check_string_type_hints: std::ptr::null_mut(),
            u_py_test_type_hint_check_static_function: std::ptr::null_mut(),
            u_py_test_type_hint_check_set_type_hints: std::ptr::null_mut(),
            u_py_test_type_hint_check_object_type_hints: std::ptr::null_mut(),
            u_py_test_type_hint_check_name_type_hints: std::ptr::null_mut(),
            u_py_test_type_hint_check_map_type_hints: std::ptr::null_mut(),
            u_py_test_type_hint_check_integer_type_hints: std::ptr::null_mut(),
            u_py_test_type_hint_check_float_type_hints: std::ptr::null_mut(),
            u_py_test_type_hint_check_field_path_type_hints: std::ptr::null_mut(),
            u_py_test_type_hint_check_enum_type_hints: std::ptr::null_mut(),
            u_py_test_type_hint_check_delegate_type_hints: std::ptr::null_mut(),
            u_py_test_type_hint_check_bool_type_hints: std::ptr::null_mut(),
            u_py_test_type_hint_check_array_type_hints: std::ptr::null_mut(),
            u_python_script_library_is_python_initialized: std::ptr::null_mut(),
            u_python_script_library_is_python_configured: std::ptr::null_mut(),
            u_python_script_library_is_python_available: std::ptr::null_mut(),
            u_python_script_library_force_enable_python_at_runtime: std::ptr::null_mut(),
            u_python_script_library_execute_python_script: std::ptr::null_mut(),
            u_python_script_library_execute_python_command_ex: std::ptr::null_mut(),
            u_python_script_library_execute_python_command: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPyTestInterface::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncInterface"),
            &raw mut __FUNCTION_PTRS.u_py_test_interface_func_interface,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPyTestChildInterface::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncInterfaceChild"),
            &raw mut __FUNCTION_PTRS.u_py_test_child_interface_func_interface_child,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPyTestOtherInterface::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncInterfaceOther"),
            &raw mut __FUNCTION_PTRS.u_py_test_other_interface_func_interface_other,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditorPythonScriptingLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetKeepPythonScriptAlive"),
            &raw mut __FUNCTION_PTRS
                .u_editor_python_scripting_library_set_keep_python_script_alive,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeepPythonScriptAlive"),
            &raw mut __FUNCTION_PTRS
                .u_editor_python_scripting_library_get_keep_python_script_alive,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPyTestStructLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBoolMutableViaRef"),
            &raw mut __FUNCTION_PTRS.u_py_test_struct_library_set_bool_mutable_via_ref,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBoolMutable"),
            &raw mut __FUNCTION_PTRS.u_py_test_struct_library_set_bool_mutable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LegacyIsBoolSet"),
            &raw mut __FUNCTION_PTRS.u_py_test_struct_library_legacy_is_bool_set,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsBoolSet"),
            &raw mut __FUNCTION_PTRS.u_py_test_struct_library_is_bool_set,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConstantValue"),
            &raw mut __FUNCTION_PTRS.u_py_test_struct_library_get_constant_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearBoolMutableViaRef"),
            &raw mut __FUNCTION_PTRS.u_py_test_struct_library_clear_bool_mutable_via_ref,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearBoolMutable"),
            &raw mut __FUNCTION_PTRS.u_py_test_struct_library_clear_bool_mutable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddStr"),
            &raw mut __FUNCTION_PTRS.u_py_test_struct_library_add_str,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddInt"),
            &raw mut __FUNCTION_PTRS.u_py_test_struct_library_add_int,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddFloat"),
            &raw mut __FUNCTION_PTRS.u_py_test_struct_library_add_float,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPyTestObject::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReturnSet"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_return_set,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReturnMap"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_return_map,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReturnFieldPath"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_return_field_path,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReturnArray"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_return_array,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MulticastDelegatePropertyCallback"),
            &raw mut __FUNCTION_PTRS
                .u_py_test_object_multicast_delegate_property_callback,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LegacyFuncTakingPyTestStruct"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_legacy_func_taking_py_test_struct,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConstantValue"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_get_constant_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncTakingPyTestStructDefault"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_func_taking_py_test_struct_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncTakingPyTestStruct"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_func_taking_py_test_struct,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncTakingPyTestDelegate"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_func_taking_py_test_delegate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncTakingPyTestChildStruct"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_func_taking_py_test_child_struct,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncTakingFieldPath"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_func_taking_field_path,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncBlueprintNativeRef"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_func_blueprint_native_ref,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncBlueprintNative"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_func_blueprint_native,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncBlueprintImplementablePackedGetter"),
            &raw mut __FUNCTION_PTRS
                .u_py_test_object_func_blueprint_implementable_packed_getter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FuncBlueprintImplementable"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_func_blueprint_implementable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EmitScriptWarning"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_emit_script_warning,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EmitScriptError"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_emit_script_error,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DelegatePropertyCallback"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_delegate_property_callback,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CallFuncBlueprintNativeRef"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_call_func_blueprint_native_ref,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CallFuncBlueprintNative"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_call_func_blueprint_native,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CallFuncBlueprintImplementablePackedGetter"),
            &raw mut __FUNCTION_PTRS
                .u_py_test_object_call_func_blueprint_implementable_packed_getter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CallFuncBlueprintImplementable"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_call_func_blueprint_implementable,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPyTestObjectLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsBoolSet"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_library_is_bool_set,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOtherConstantValue"),
            &raw mut __FUNCTION_PTRS.u_py_test_object_library_get_other_constant_value,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPyTestStructDelegate::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnNameCollisionDelegate__DelegateSignature"),
            &raw mut __FUNCTION_PTRS
                .u_py_test_struct_delegate_on_name_collision_delegate_delegate_signature,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPyTestVectorDelegate::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnNameCollisionDelegate__DelegateSignature"),
            &raw mut __FUNCTION_PTRS
                .u_py_test_vector_delegate_on_name_collision_delegate_delegate_signature,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPyTestTypeHint::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStringConst"),
            &raw mut __FUNCTION_PTRS.u_py_test_type_hint_get_string_const,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIntConst"),
            &raw mut __FUNCTION_PTRS.u_py_test_type_hint_get_int_const,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckTupleReturnType"),
            &raw mut __FUNCTION_PTRS.u_py_test_type_hint_check_tuple_return_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckTextTypeHints"),
            &raw mut __FUNCTION_PTRS.u_py_test_type_hint_check_text_type_hints,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckStructTypeHints"),
            &raw mut __FUNCTION_PTRS.u_py_test_type_hint_check_struct_type_hints,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckStringTypeHints"),
            &raw mut __FUNCTION_PTRS.u_py_test_type_hint_check_string_type_hints,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckStaticFunction"),
            &raw mut __FUNCTION_PTRS.u_py_test_type_hint_check_static_function,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckSetTypeHints"),
            &raw mut __FUNCTION_PTRS.u_py_test_type_hint_check_set_type_hints,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckObjectTypeHints"),
            &raw mut __FUNCTION_PTRS.u_py_test_type_hint_check_object_type_hints,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckNameTypeHints"),
            &raw mut __FUNCTION_PTRS.u_py_test_type_hint_check_name_type_hints,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckMapTypeHints"),
            &raw mut __FUNCTION_PTRS.u_py_test_type_hint_check_map_type_hints,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckIntegerTypeHints"),
            &raw mut __FUNCTION_PTRS.u_py_test_type_hint_check_integer_type_hints,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckFloatTypeHints"),
            &raw mut __FUNCTION_PTRS.u_py_test_type_hint_check_float_type_hints,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckFieldPathTypeHints"),
            &raw mut __FUNCTION_PTRS.u_py_test_type_hint_check_field_path_type_hints,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckEnumTypeHints"),
            &raw mut __FUNCTION_PTRS.u_py_test_type_hint_check_enum_type_hints,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckDelegateTypeHints"),
            &raw mut __FUNCTION_PTRS.u_py_test_type_hint_check_delegate_type_hints,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckBoolTypeHints"),
            &raw mut __FUNCTION_PTRS.u_py_test_type_hint_check_bool_type_hints,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckArrayTypeHints"),
            &raw mut __FUNCTION_PTRS.u_py_test_type_hint_check_array_type_hints,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPythonScriptLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPythonInitialized"),
            &raw mut __FUNCTION_PTRS.u_python_script_library_is_python_initialized,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPythonConfigured"),
            &raw mut __FUNCTION_PTRS.u_python_script_library_is_python_configured,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPythonAvailable"),
            &raw mut __FUNCTION_PTRS.u_python_script_library_is_python_available,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ForceEnablePythonAtRuntime"),
            &raw mut __FUNCTION_PTRS
                .u_python_script_library_force_enable_python_at_runtime,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExecutePythonScript"),
            &raw mut __FUNCTION_PTRS.u_python_script_library_execute_python_script,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExecutePythonCommandEx"),
            &raw mut __FUNCTION_PTRS.u_python_script_library_execute_python_command_ex,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExecutePythonCommand"),
            &raw mut __FUNCTION_PTRS.u_python_script_library_execute_python_command,
        );
    }
}
#[repr(C, align(8))]
pub struct FPyTestStruct {
    pub bool: bool,
    pub int: i32,
    pub float: f32,
    pub enum_: EPyTestEnum,
    pub string: FString,
    pub name: FName,
    pub text: FText,
    pub field_path: TFieldPath<FProperty>,
    pub struct_field_path: TFieldPath<FStructProperty>,
    pub string_array: TArray<FString>,
    pub string_set: TSet<FString>,
    pub string_int_map: TMap<FString, i32>,
    #[doc(hidden)]
    pub(crate) __padding_342: [u8; 6],
    pub bool_mutable: bool,
}
impl FPyTestStruct {}
#[repr(C, align(8))]
pub struct FPythonLogOutputEntry {
    pub ty: EPythonLogOutputType,
    pub output: FString,
}
impl FPythonLogOutputEntry {}
#[repr(C, align(8))]
pub struct FPyTestChildStruct {
    pub(crate) __padding_end: [u8; 344],
}
impl FPyTestChildStruct {}
#[repr(C, align(4))]
pub struct FPyTestClassSparseData {
    pub int_from_sparse_data: i32,
}
impl FPyTestClassSparseData {}
pub struct IPyTestInterface {}
#[repr(C, align(8))]
pub struct UPyTestInterface {
    __padding_end: [u8; 48],
}
impl UPyTestInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPyTestInterface")
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
    pub fn func_interface(&self, in_value: i32) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_interface_func_interface,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_interface_func_interface,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
}
pub struct IPyTestChildInterface {}
#[repr(C, align(8))]
pub struct UPyTestChildInterface {
    __padding_end: [u8; 48],
}
impl UPyTestChildInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPyTestChildInterface")
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
    pub fn func_interface_child(&self, in_value: i32) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_child_interface_func_interface_child,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_child_interface_func_interface_child,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
}
pub struct IPyTestOtherInterface {}
#[repr(C, align(8))]
pub struct UPyTestOtherInterface {
    __padding_end: [u8; 48],
}
impl UPyTestOtherInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPyTestOtherInterface")
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
    pub fn func_interface_other(&self, in_value: i32) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_other_interface_func_interface_other,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_other_interface_func_interface_other,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UEditorPythonScriptingLibrary {
    __padding_end: [u8; 48],
}
impl UEditorPythonScriptingLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorPythonScriptingLibrary")
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
    pub fn set_keep_python_script_alive(b_new_keep_alive: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_editor_python_scripting_library_set_keep_python_script_alive,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_new_keep_alive,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::python_script_plugin::UEditorPythonScriptingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_editor_python_scripting_library_set_keep_python_script_alive,
                __buffer,
            )
        };
    }
    pub fn get_keep_python_script_alive() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_editor_python_scripting_library_get_keep_python_script_alive,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::python_script_plugin::UEditorPythonScriptingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_editor_python_scripting_library_get_keep_python_script_alive,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UK2Node_ExecutePythonScript {
    __padding_end: [u8; 368],
}
impl UK2Node_ExecutePythonScript {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UK2Node_ExecutePythonScript")
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
pub struct UPyTestStructLibrary {
    __padding_end: [u8; 48],
}
impl UPyTestStructLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPyTestStructLibrary")
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
    pub fn set_bool_mutable_via_ref(in_struct: &mut FPyTestStruct) {
        let mut __stack = crate::core_data::StackAlloc::<344>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_struct_library_set_bool_mutable_via_ref,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_struct,
                __buffer.add(0).cast::<FPyTestStruct>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestStructLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_struct_library_set_bool_mutable_via_ref,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FPyTestStruct>().swap(in_struct);
        }
    }
    pub fn set_bool_mutable(in_struct: &FPyTestStruct) {
        let mut __stack = crate::core_data::StackAlloc::<344>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_struct_library_set_bool_mutable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_struct,
                __buffer.add(0).cast::<FPyTestStruct>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestStructLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_struct_library_set_bool_mutable,
                __buffer,
            )
        };
    }
    pub fn legacy_is_bool_set(in_struct: &FPyTestStruct) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<345>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_struct_library_legacy_is_bool_set,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_struct,
                __buffer.add(0).cast::<FPyTestStruct>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestStructLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_struct_library_legacy_is_bool_set,
                __buffer,
            )
        };
        unsafe { __buffer.add(344).cast::<bool>().read() }
    }
    pub fn is_bool_set(in_struct: &FPyTestStruct) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<345>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_struct_library_is_bool_set,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_struct,
                __buffer.add(0).cast::<FPyTestStruct>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestStructLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_struct_library_is_bool_set,
                __buffer,
            )
        };
        unsafe { __buffer.add(344).cast::<bool>().read() }
    }
    pub fn get_constant_value() -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_struct_library_get_constant_value,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestStructLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_struct_library_get_constant_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn clear_bool_mutable_via_ref(in_struct: &mut FPyTestStruct) {
        let mut __stack = crate::core_data::StackAlloc::<344>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_struct_library_clear_bool_mutable_via_ref,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_struct,
                __buffer.add(0).cast::<FPyTestStruct>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestStructLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_struct_library_clear_bool_mutable_via_ref,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FPyTestStruct>().swap(in_struct);
        }
    }
    pub fn clear_bool_mutable(in_struct: &FPyTestStruct) {
        let mut __stack = crate::core_data::StackAlloc::<344>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_struct_library_clear_bool_mutable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_struct,
                __buffer.add(0).cast::<FPyTestStruct>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestStructLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_struct_library_clear_bool_mutable,
                __buffer,
            )
        };
    }
    pub fn add_str(in_struct: &FPyTestStruct, in_value: FString) -> FPyTestStruct {
        let mut __stack = crate::core_data::StackAlloc::<704>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_struct_library_add_str,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_struct,
                __buffer.add(0).cast::<FPyTestStruct>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(344).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestStructLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_struct_library_add_str,
                __buffer,
            )
        };
        unsafe { __buffer.add(360).cast::<FPyTestStruct>().read() }
    }
    pub fn add_int(in_struct: &FPyTestStruct, in_value: i32) -> FPyTestStruct {
        let mut __stack = crate::core_data::StackAlloc::<696>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_struct_library_add_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_struct,
                __buffer.add(0).cast::<FPyTestStruct>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(344).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestStructLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_struct_library_add_int,
                __buffer,
            )
        };
        unsafe { __buffer.add(352).cast::<FPyTestStruct>().read() }
    }
    pub fn add_float(in_struct: &FPyTestStruct, in_value: f32) -> FPyTestStruct {
        let mut __stack = crate::core_data::StackAlloc::<696>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_struct_library_add_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_struct,
                __buffer.add(0).cast::<FPyTestStruct>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(344).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestStructLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_struct_library_add_float,
                __buffer,
            )
        };
        unsafe { __buffer.add(352).cast::<FPyTestStruct>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPyTestObject {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub bool: bool,
    pub int: i32,
    pub float: f32,
    pub enum_: EPyTestEnum,
    pub string: FString,
    pub name: FName,
    pub text: FText,
    pub field_path: TFieldPath<FProperty>,
    pub struct_field_path: TFieldPath<FStructProperty>,
    pub string_array: TArray<FString>,
    pub string_set: TSet<FString>,
    pub string_int_map: TMap<FString, i32>,
    pub delegate: FPyTestObject_Delegate,
    #[doc(hidden)]
    pub(crate) __padding_456: [u8; 24],
    pub _struct: FPyTestStruct,
    pub struct_array: TArray<FPyTestStruct>,
    pub child_struct: FPyTestChildStruct,
    __padding_end: [u8; 8],
}
impl UPyTestObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPyTestObject")
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
    pub fn return_set() -> TSet<i32> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_return_set,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestObject::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_return_set,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TSet<i32>>().read() }
    }
    pub fn return_map() -> TMap<i32, bool> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_return_map,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestObject::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_return_map,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TMap<i32, bool>>().read() }
    }
    pub fn return_field_path() -> TFieldPath<FProperty> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_return_field_path,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestObject::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_return_field_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TFieldPath<FProperty>>().read() }
    }
    pub fn return_array() -> TArray<i32> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_return_array,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestObject::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_return_array,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<i32>>().read() }
    }
    pub fn multicast_delegate_property_callback(&self, in_str: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_multicast_delegate_property_callback,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_str, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_multicast_delegate_property_callback,
                __buffer,
            )
        };
    }
    pub fn legacy_func_taking_py_test_struct(&self, in_struct: &FPyTestStruct) {
        let mut __stack = crate::core_data::StackAlloc::<344>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_legacy_func_taking_py_test_struct,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_struct,
                __buffer.add(0).cast::<FPyTestStruct>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_legacy_func_taking_py_test_struct,
                __buffer,
            )
        };
    }
    pub fn get_constant_value() -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_get_constant_value,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestObject::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_get_constant_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn func_taking_py_test_struct_default(&mut self, in_struct: &FPyTestStruct) {
        let mut __stack = crate::core_data::StackAlloc::<344>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_func_taking_py_test_struct_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_struct,
                __buffer.add(0).cast::<FPyTestStruct>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_func_taking_py_test_struct_default,
                __buffer,
            )
        };
    }
    pub fn func_taking_py_test_struct(&self, in_struct: &FPyTestStruct) {
        let mut __stack = crate::core_data::StackAlloc::<344>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_func_taking_py_test_struct,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_struct,
                __buffer.add(0).cast::<FPyTestStruct>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_func_taking_py_test_struct,
                __buffer,
            )
        };
    }
    pub fn func_taking_py_test_delegate(
        &self,
        in_delegate: &FFuncTakingPyTestDelegate_InDelegate,
        in_value: i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_func_taking_py_test_delegate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_delegate,
                __buffer.add(0).cast::<FFuncTakingPyTestDelegate_InDelegate>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(32).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_func_taking_py_test_delegate,
                __buffer,
            )
        };
        unsafe { __buffer.add(36).cast::<i32>().read() }
    }
    pub fn func_taking_py_test_child_struct(&self, in_struct: &FPyTestChildStruct) {
        let mut __stack = crate::core_data::StackAlloc::<344>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_func_taking_py_test_child_struct,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_struct,
                __buffer.add(0).cast::<FPyTestChildStruct>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_func_taking_py_test_child_struct,
                __buffer,
            )
        };
    }
    pub fn func_taking_field_path(&mut self, in_field_path: &TFieldPath<FProperty>) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_func_taking_field_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_field_path,
                __buffer.add(0).cast::<TFieldPath<FProperty>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_func_taking_field_path,
                __buffer,
            )
        };
    }
    pub fn func_blueprint_native_ref(&self, in_out_struct: &mut FPyTestStruct) {
        let mut __stack = crate::core_data::StackAlloc::<344>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_func_blueprint_native_ref,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_struct,
                __buffer.add(0).cast::<FPyTestStruct>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_func_blueprint_native_ref,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FPyTestStruct>().swap(in_out_struct);
        }
    }
    pub fn func_blueprint_native(&self, in_value: i32) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_func_blueprint_native,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_func_blueprint_native,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn func_blueprint_implementable_packed_getter(
        &self,
        out_value: &mut i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_func_blueprint_implementable_packed_getter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_func_blueprint_implementable_packed_getter,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(out_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn func_blueprint_implementable(&self, in_value: i32) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_func_blueprint_implementable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_func_blueprint_implementable,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn emit_script_warning() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_emit_script_warning,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestObject::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_emit_script_warning,
                __buffer,
            )
        };
    }
    pub fn emit_script_error() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_emit_script_error,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestObject::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_emit_script_error,
                __buffer,
            )
        };
    }
    pub fn delegate_property_callback(&self, in_value: i32) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_delegate_property_callback,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_delegate_property_callback,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn call_func_blueprint_native_ref(&self, in_out_struct: &mut FPyTestStruct) {
        let mut __stack = crate::core_data::StackAlloc::<344>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_call_func_blueprint_native_ref,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_struct,
                __buffer.add(0).cast::<FPyTestStruct>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_call_func_blueprint_native_ref,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FPyTestStruct>().swap(in_out_struct);
        }
    }
    pub fn call_func_blueprint_native(&self, in_value: i32) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_call_func_blueprint_native,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_call_func_blueprint_native,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn call_func_blueprint_implementable_packed_getter(
        &self,
        out_value: &mut i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_call_func_blueprint_implementable_packed_getter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_call_func_blueprint_implementable_packed_getter,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(out_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn call_func_blueprint_implementable(&self, in_value: i32) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_call_func_blueprint_implementable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_call_func_blueprint_implementable,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPyTestChildObject {
    __padding_end: [u8; 1168],
}
impl UPyTestChildObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPyTestChildObject")
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
pub struct UDEPRECATED_LegacyPyTestObject {
    __padding_end: [u8; 1168],
}
impl UDEPRECATED_LegacyPyTestObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_LegacyPyTestObject")
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
pub struct UPyTestObjectLibrary {
    __padding_end: [u8; 48],
}
impl UPyTestObjectLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPyTestObjectLibrary")
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
    pub fn is_bool_set(in_obj: UPtr<UPyTestObject>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_library_is_bool_set,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_obj,
                __buffer.add(0).cast::<UPtr<UPyTestObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestObjectLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_library_is_bool_set,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_other_constant_value() -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_library_get_other_constant_value,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestObjectLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_object_library_get_other_constant_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPyTestStructDelegate {
    __padding_end: [u8; 72],
}
impl UPyTestStructDelegate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPyTestStructDelegate")
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
pub struct UPyTestVectorDelegate {
    __padding_end: [u8; 72],
}
impl UPyTestVectorDelegate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPyTestVectorDelegate")
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
pub struct UPyTestTypeHint {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub bool_prop: bool,
    pub int_prop: i32,
    pub float_prop: f32,
    pub enum_prop: EPyTestEnum,
    pub string_prop: FString,
    pub name_prop: FName,
    pub text_prop: FText,
    pub field_path_prop: TFieldPath<FProperty>,
    pub struct_prop: FPyTestStruct,
    pub object_prop: UPtr<UPyTestObject>,
    pub str_array_prop: TArray<FString>,
    pub name_array_prop: TArray<FName>,
    pub text_array_prop: TArray<FText>,
    pub object_array_prop: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub set_prop: TSet<FString>,
    pub map_prop: TMap<i32, FString>,
    pub delegate_prop: FPyTestTypeHint_DelegateProp,
    #[doc(hidden)]
    pub(crate) __padding_792: [u8; 24],
    pub slate_tick_delegate: FPyTestTypeHint_SlateTickDelegate,
}
impl UPyTestTypeHint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPyTestTypeHint")
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
    pub fn get_string_const() -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_get_string_const,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestTypeHint::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_get_string_const,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_int_const() -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_get_int_const,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestTypeHint::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_get_int_const,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn check_tuple_return_type(in_out_string: &mut FString) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_tuple_return_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_string,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestTypeHint::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_tuple_return_type,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(in_out_string);
        }
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn check_text_type_hints(&mut self, param1: &FText, param2: &FText) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_text_type_hints,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(param1, __buffer.add(0).cast::<FText>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(param2, __buffer.add(16).cast::<FText>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_text_type_hints,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FText>().read() }
    }
    pub fn check_struct_type_hints(
        &mut self,
        param1: &FPyTestStruct,
        param2: &FPyTestStruct,
    ) -> FPyTestStruct {
        let mut __stack = crate::core_data::StackAlloc::<1032>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_struct_type_hints,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                param1,
                __buffer.add(0).cast::<FPyTestStruct>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                param2,
                __buffer.add(344).cast::<FPyTestStruct>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_struct_type_hints,
                __buffer,
            )
        };
        unsafe { __buffer.add(688).cast::<FPyTestStruct>().read() }
    }
    pub fn check_string_type_hints(
        &mut self,
        param1: FString,
        param2: FString,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_string_type_hints,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&param1, __buffer.add(0).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &param2,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_string_type_hints,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FString>().read() }
    }
    pub fn check_static_function(
        param1: bool,
        param2: i32,
        param3: f64,
        param4: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_static_function,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&param1, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&param2, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&param3, __buffer.add(8).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &param4,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::python_script_plugin::UPyTestTypeHint::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_static_function,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn check_set_type_hints(
        &mut self,
        param1: &TSet<FString>,
        param2: &TSet<FName>,
        param3: &TSet<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> TSet<FName> {
        let mut __stack = crate::core_data::StackAlloc::<320>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_set_type_hints,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                param1,
                __buffer.add(0).cast::<TSet<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                param2,
                __buffer.add(80).cast::<TSet<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                param3,
                __buffer
                    .add(160)
                    .cast::<TSet<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_set_type_hints,
                __buffer,
            )
        };
        unsafe { __buffer.add(240).cast::<TSet<FName>>().read() }
    }
    pub fn check_object_type_hints(
        &mut self,
        param1: UPtr<UPyTestObject>,
        param4: UPtr<UPyTestObject>,
    ) -> UPtr<UPyTestObject> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_object_type_hints,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &param1,
                __buffer.add(0).cast::<UPtr<UPyTestObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &param4,
                __buffer.add(8).cast::<UPtr<UPyTestObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_object_type_hints,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UPyTestObject>>().read() }
    }
    pub fn check_name_type_hints(&mut self, param1: &FName, param2: &FName) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_name_type_hints,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(param1, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(param2, __buffer.add(12).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_name_type_hints,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FName>().read() }
    }
    pub fn check_map_type_hints(
        &mut self,
        param1: &TMap<i32, FString>,
        param2: &TMap<i32, FName>,
        param3: &TMap<i32, FText>,
        param4: &TMap<i32, UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> TMap<FString, UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<400>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_map_type_hints,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                param1,
                __buffer.add(0).cast::<TMap<i32, FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                param2,
                __buffer.add(80).cast::<TMap<i32, FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                param3,
                __buffer.add(160).cast::<TMap<i32, FText>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                param4,
                __buffer
                    .add(240)
                    .cast::<TMap<i32, UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_map_type_hints,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(320)
                .cast::<TMap<FString, UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
    pub fn check_integer_type_hints(
        &mut self,
        param1: u8,
        param2: i32,
        param3: i64,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_integer_type_hints,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&param1, __buffer.add(0).cast::<u8>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&param2, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&param3, __buffer.add(8).cast::<i64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_integer_type_hints,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn check_float_type_hints(
        &mut self,
        param1: f32,
        param2: f64,
        param3: f32,
        param4: f64,
    ) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_float_type_hints,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&param1, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&param2, __buffer.add(8).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&param3, __buffer.add(16).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&param4, __buffer.add(24).cast::<f64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_float_type_hints,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<f64>().read() }
    }
    pub fn check_field_path_type_hints(
        &mut self,
        param1: TFieldPath<FProperty>,
    ) -> TFieldPath<FProperty> {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_field_path_type_hints,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &param1,
                __buffer.add(0).cast::<TFieldPath<FProperty>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_field_path_type_hints,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<TFieldPath<FProperty>>().read() }
    }
    pub fn check_enum_type_hints(
        &mut self,
        param1: EPyTestEnum,
        param2: EPyTestEnum,
    ) -> EPyTestEnum {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_enum_type_hints,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &param1,
                __buffer.add(0).cast::<EPyTestEnum>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &param2,
                __buffer.add(1).cast::<EPyTestEnum>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_enum_type_hints,
                __buffer,
            )
        };
        unsafe { __buffer.add(2).cast::<EPyTestEnum>().read() }
    }
    pub fn check_delegate_type_hints(
        &mut self,
        param1: &FCheckDelegateTypeHints_Param1,
    ) -> FCheckDelegateTypeHints_ReturnValue {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_delegate_type_hints,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                param1,
                __buffer.add(0).cast::<FCheckDelegateTypeHints_Param1>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_delegate_type_hints,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FCheckDelegateTypeHints_ReturnValue>().read() }
    }
    pub fn check_bool_type_hints(
        &mut self,
        b_param1: bool,
        b_param2: bool,
        b_param3: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_bool_type_hints,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_param1, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_param2, __buffer.add(1).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_param3, __buffer.add(2).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_bool_type_hints,
                __buffer,
            )
        };
        unsafe { __buffer.add(3).cast::<bool>().read() }
    }
    pub fn check_array_type_hints(
        &mut self,
        param1: &TArray<FString>,
        param2: &TArray<FName>,
        param3: &TArray<FText>,
        param4: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> TArray<FText> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_array_type_hints,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                param1,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                param2,
                __buffer.add(16).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                param3,
                __buffer.add(32).cast::<TArray<FText>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                param4,
                __buffer
                    .add(48)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_py_test_type_hint_check_array_type_hints,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<TArray<FText>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPythonOnlineDocsCommandlet {
    __padding_end: [u8; 136],
}
impl UPythonOnlineDocsCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonOnlineDocsCommandlet")
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
pub struct UPythonScriptCommandlet {
    __padding_end: [u8; 136],
}
impl UPythonScriptCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonScriptCommandlet")
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
pub struct UPythonScriptLibrary {
    __padding_end: [u8; 48],
}
impl UPythonScriptLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonScriptLibrary")
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
    pub fn is_python_initialized() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_python_script_library_is_python_initialized,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::python_script_plugin::UPythonScriptLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_python_script_library_is_python_initialized,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_python_configured() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_python_script_library_is_python_configured,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::python_script_plugin::UPythonScriptLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_python_script_library_is_python_configured,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_python_available() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_python_script_library_is_python_available,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::python_script_plugin::UPythonScriptLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_python_script_library_is_python_available,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn force_enable_python_at_runtime() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_python_script_library_force_enable_python_at_runtime,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::python_script_plugin::UPythonScriptLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_python_script_library_force_enable_python_at_runtime,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn execute_python_script(
        python_script: FString,
        python_inputs: &TArray<FString>,
        python_outputs: &TArray<FString>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_python_script_library_execute_python_script,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &python_script,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                python_inputs,
                __buffer.add(16).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                python_outputs,
                __buffer.add(32).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::python_script_plugin::UPythonScriptLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_python_script_library_execute_python_script,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn execute_python_command_ex(
        python_command: FString,
        command_result: &mut FString,
        log_output: &mut TArray<FPythonLogOutputEntry>,
        execution_mode: EPythonCommandExecutionMode,
        file_execution_scope: EPythonFileExecutionScope,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<51>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_python_script_library_execute_python_command_ex,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &python_command,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                command_result,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                log_output,
                __buffer.add(32).cast::<TArray<FPythonLogOutputEntry>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &execution_mode,
                __buffer.add(48).cast::<EPythonCommandExecutionMode>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &file_execution_scope,
                __buffer.add(49).cast::<EPythonFileExecutionScope>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::python_script_plugin::UPythonScriptLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_python_script_library_execute_python_command_ex,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FString>().swap(command_result);
        }
        unsafe {
            __buffer.add(32).cast::<TArray<FPythonLogOutputEntry>>().swap(log_output);
        }
        unsafe { __buffer.add(50).cast::<bool>().read() }
    }
    pub fn execute_python_command(python_command: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_python_script_library_execute_python_command,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &python_command,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::python_script_plugin::UPythonScriptLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::python_script_plugin::__FUNCTION_PTRS
                    .u_python_script_library_execute_python_command,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPythonScriptPluginSettings {
    __padding_end: [u8; 232],
}
impl UPythonScriptPluginSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonScriptPluginSettings")
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
pub struct UPythonScriptPluginUserSettings {
    __padding_end: [u8; 112],
}
impl UPythonScriptPluginUserSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonScriptPluginUserSettings")
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
pub struct IPythonResourceOwner {}
#[repr(C, align(8))]
pub struct UPythonResourceOwner {
    __padding_end: [u8; 48],
}
impl UPythonResourceOwner {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonResourceOwner")
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
pub struct UPythonObjectHandle {
    __padding_end: [u8; 64],
}
impl UPythonObjectHandle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonObjectHandle")
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
pub struct UPythonCallableForDelegate {
    __padding_end: [u8; 64],
}
impl UPythonCallableForDelegate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonCallableForDelegate")
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
pub struct UPythonGeneratedEnum {
    __padding_end: [u8; 216],
}
impl UPythonGeneratedEnum {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonGeneratedEnum")
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
pub struct UPythonGeneratedClass {
    __padding_end: [u8; 1048],
}
impl UPythonGeneratedClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonGeneratedClass")
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
pub struct UPythonGeneratedStruct {
    __padding_end: [u8; 1040],
}
impl UPythonGeneratedStruct {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPythonGeneratedStruct")
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
pub struct FFuncTakingPyTestDelegate_InDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCheckDelegateTypeHints_Param1 {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCheckDelegateTypeHints_ReturnValue {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FPyTestObject_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FPyTestObject_MulticastDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPyTestStructDelegate_OnNameCollisionTestDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPyTestVectorDelegate_OnNameCollisionTestDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPyTestTypeHint_DelegateProp {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FPyTestTypeHint_MulticastDelegateProp {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPyTestTypeHint_SlateTickDelegate {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct EPyTestEnum(pub u8);
impl EPyTestEnum {
    pub const ONE: EPyTestEnum = EPyTestEnum(0);
    pub const TWO: EPyTestEnum = EPyTestEnum(1);
}
#[repr(transparent)]
pub struct EPythonLogOutputType(pub u8);
impl EPythonLogOutputType {
    pub const INFO: EPythonLogOutputType = EPythonLogOutputType(0);
    pub const WARNING: EPythonLogOutputType = EPythonLogOutputType(1);
    pub const ERROR: EPythonLogOutputType = EPythonLogOutputType(2);
}
#[repr(transparent)]
pub struct EPythonCommandExecutionMode(pub u8);
impl EPythonCommandExecutionMode {
    pub const EXECUTE_FILE: EPythonCommandExecutionMode = EPythonCommandExecutionMode(0);
    pub const EXECUTE_STATEMENT: EPythonCommandExecutionMode = EPythonCommandExecutionMode(
        1,
    );
    pub const EVALUATE_STATEMENT: EPythonCommandExecutionMode = EPythonCommandExecutionMode(
        2,
    );
}
#[repr(transparent)]
pub struct EPythonFileExecutionScope(pub u8);
impl EPythonFileExecutionScope {
    pub const PRIVATE: EPythonFileExecutionScope = EPythonFileExecutionScope(0);
    pub const PUBLIC: EPythonFileExecutionScope = EPythonFileExecutionScope(1);
}
#[repr(transparent)]
pub struct EPythonEnabledOverrideState(pub u8);
impl EPythonEnabledOverrideState {
    pub const NONE: EPythonEnabledOverrideState = EPythonEnabledOverrideState(0);
    pub const ENABLE: EPythonEnabledOverrideState = EPythonEnabledOverrideState(1);
    pub const DISABLE: EPythonEnabledOverrideState = EPythonEnabledOverrideState(2);
}
#[repr(transparent)]
pub struct ETypeHintingMode(pub u8);
impl ETypeHintingMode {
    pub const OFF: ETypeHintingMode = ETypeHintingMode(0);
    pub const AUTO_COMPLETION: ETypeHintingMode = ETypeHintingMode(1);
    pub const TYPE_CHECKER: ETypeHintingMode = ETypeHintingMode(2);
}
