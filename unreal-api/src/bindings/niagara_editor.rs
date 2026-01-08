#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_niagara_baker_function_library_capture_niagara_to_static_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_function_on_pasted_function_call_node_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_try_set_local_value_as_int: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_try_get_local_value_as_int: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_try_get_local_value_as_float: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_try_get_input_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_get_type_name: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_create_vec4_local_value_input: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_create_vec3_local_value_input: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_create_vec2_local_value_input: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_create_struct_local_value_input: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_create_quat_local_value_input: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_create_matrix_local_value_input: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_create_linked_value_input: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_create_int_local_value_input: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_create_float_local_value_input: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_create_expression_value_input: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_create_enum_local_value_input: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_create_enum_linked_value_input: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_create_dynamic_value_input: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_create_data_value_input: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_create_color_local_value_input: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_clipboard_editor_scripting_utilities_create_bool_local_value_input: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_python_script_module_input_is_set: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_python_script_module_input_is_local_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_python_script_module_input_is_linked_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_python_script_module_input_as_vec4: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_python_script_module_input_as_vec3: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_python_script_module_input_as_vec2: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_python_script_module_input_as_quat: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_python_script_module_input_as_linked_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_python_script_module_input_as_int: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_python_script_module_input_as_float: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_python_script_module_input_as_enum: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_python_script_module_input_as_color: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_python_script_module_input_as_bool: *mut crate::ffi::UFunctionOpague,
    pub u_upgrade_niagara_script_results_set_vec4_input: *mut crate::ffi::UFunctionOpague,
    pub u_upgrade_niagara_script_results_set_vec3_input: *mut crate::ffi::UFunctionOpague,
    pub u_upgrade_niagara_script_results_set_vec2_input: *mut crate::ffi::UFunctionOpague,
    pub u_upgrade_niagara_script_results_set_quat_input: *mut crate::ffi::UFunctionOpague,
    pub u_upgrade_niagara_script_results_set_new_input: *mut crate::ffi::UFunctionOpague,
    pub u_upgrade_niagara_script_results_set_linked_input: *mut crate::ffi::UFunctionOpague,
    pub u_upgrade_niagara_script_results_set_int_input: *mut crate::ffi::UFunctionOpague,
    pub u_upgrade_niagara_script_results_set_float_input: *mut crate::ffi::UFunctionOpague,
    pub u_upgrade_niagara_script_results_set_enum_input_from_int: *mut crate::ffi::UFunctionOpague,
    pub u_upgrade_niagara_script_results_set_enum_input: *mut crate::ffi::UFunctionOpague,
    pub u_upgrade_niagara_script_results_set_color_input: *mut crate::ffi::UFunctionOpague,
    pub u_upgrade_niagara_script_results_set_bool_input: *mut crate::ffi::UFunctionOpague,
    pub u_upgrade_niagara_script_results_reset_to_default: *mut crate::ffi::UFunctionOpague,
    pub u_upgrade_niagara_script_results_get_old_input: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_python_module_get_object: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_python_emitter_set_properties: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_python_emitter_has_module: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_python_emitter_get_properties: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_python_emitter_get_object: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_python_emitter_get_modules: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_python_emitter_get_module: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_niagara_baker_function_library_capture_niagara_to_static_mesh: std::ptr::null_mut(),
            u_niagara_clipboard_function_on_pasted_function_call_node_delegate_signature: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_try_set_local_value_as_int: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_try_get_local_value_as_int: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_try_get_local_value_as_float: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_try_get_input_by_name: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_get_type_name: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_create_vec4_local_value_input: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_create_vec3_local_value_input: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_create_vec2_local_value_input: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_create_struct_local_value_input: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_create_quat_local_value_input: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_create_matrix_local_value_input: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_create_linked_value_input: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_create_int_local_value_input: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_create_float_local_value_input: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_create_expression_value_input: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_create_enum_local_value_input: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_create_enum_linked_value_input: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_create_dynamic_value_input: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_create_data_value_input: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_create_color_local_value_input: std::ptr::null_mut(),
            u_niagara_clipboard_editor_scripting_utilities_create_bool_local_value_input: std::ptr::null_mut(),
            u_niagara_python_script_module_input_is_set: std::ptr::null_mut(),
            u_niagara_python_script_module_input_is_local_value: std::ptr::null_mut(),
            u_niagara_python_script_module_input_is_linked_value: std::ptr::null_mut(),
            u_niagara_python_script_module_input_as_vec4: std::ptr::null_mut(),
            u_niagara_python_script_module_input_as_vec3: std::ptr::null_mut(),
            u_niagara_python_script_module_input_as_vec2: std::ptr::null_mut(),
            u_niagara_python_script_module_input_as_quat: std::ptr::null_mut(),
            u_niagara_python_script_module_input_as_linked_value: std::ptr::null_mut(),
            u_niagara_python_script_module_input_as_int: std::ptr::null_mut(),
            u_niagara_python_script_module_input_as_float: std::ptr::null_mut(),
            u_niagara_python_script_module_input_as_enum: std::ptr::null_mut(),
            u_niagara_python_script_module_input_as_color: std::ptr::null_mut(),
            u_niagara_python_script_module_input_as_bool: std::ptr::null_mut(),
            u_upgrade_niagara_script_results_set_vec4_input: std::ptr::null_mut(),
            u_upgrade_niagara_script_results_set_vec3_input: std::ptr::null_mut(),
            u_upgrade_niagara_script_results_set_vec2_input: std::ptr::null_mut(),
            u_upgrade_niagara_script_results_set_quat_input: std::ptr::null_mut(),
            u_upgrade_niagara_script_results_set_new_input: std::ptr::null_mut(),
            u_upgrade_niagara_script_results_set_linked_input: std::ptr::null_mut(),
            u_upgrade_niagara_script_results_set_int_input: std::ptr::null_mut(),
            u_upgrade_niagara_script_results_set_float_input: std::ptr::null_mut(),
            u_upgrade_niagara_script_results_set_enum_input_from_int: std::ptr::null_mut(),
            u_upgrade_niagara_script_results_set_enum_input: std::ptr::null_mut(),
            u_upgrade_niagara_script_results_set_color_input: std::ptr::null_mut(),
            u_upgrade_niagara_script_results_set_bool_input: std::ptr::null_mut(),
            u_upgrade_niagara_script_results_reset_to_default: std::ptr::null_mut(),
            u_upgrade_niagara_script_results_get_old_input: std::ptr::null_mut(),
            u_niagara_python_module_get_object: std::ptr::null_mut(),
            u_niagara_python_emitter_set_properties: std::ptr::null_mut(),
            u_niagara_python_emitter_has_module: std::ptr::null_mut(),
            u_niagara_python_emitter_get_properties: std::ptr::null_mut(),
            u_niagara_python_emitter_get_object: std::ptr::null_mut(),
            u_niagara_python_emitter_get_modules: std::ptr::null_mut(),
            u_niagara_python_emitter_get_module: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraBakerFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CaptureNiagaraToStaticMesh"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_baker_function_library_capture_niagara_to_static_mesh,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraClipboardFunction::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnPastedFunctionCallNode__DelegateSignature"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_function_on_pasted_function_call_node_delegate_signature,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraClipboardEditorScriptingUtilities::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TrySetLocalValueAsInt"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_try_set_local_value_as_int,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TryGetLocalValueAsInt"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_try_get_local_value_as_int,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TryGetLocalValueAsFloat"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_try_get_local_value_as_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TryGetInputByName"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_try_get_input_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTypeName"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_get_type_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateVec4LocalValueInput"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_create_vec4_local_value_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateVec3LocalValueInput"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_create_vec3_local_value_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateVec2LocalValueInput"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_create_vec2_local_value_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateStructLocalValueInput"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_create_struct_local_value_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateQuatLocalValueInput"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_create_quat_local_value_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMatrixLocalValueInput"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_create_matrix_local_value_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateLinkedValueInput"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_create_linked_value_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateIntLocalValueInput"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_create_int_local_value_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateFloatLocalValueInput"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_create_float_local_value_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateExpressionValueInput"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_create_expression_value_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateEnumLocalValueInput"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_create_enum_local_value_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateEnumLinkedValueInput"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_create_enum_linked_value_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateDynamicValueInput"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_create_dynamic_value_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateDataValueInput"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_create_data_value_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateColorLocalValueInput"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_create_color_local_value_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateBoolLocalValueInput"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_clipboard_editor_scripting_utilities_create_bool_local_value_input,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraPythonScriptModuleInput::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsSet"),
            &raw mut __FUNCTION_PTRS.u_niagara_python_script_module_input_is_set,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLocalValue"),
            &raw mut __FUNCTION_PTRS.u_niagara_python_script_module_input_is_local_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLinkedValue"),
            &raw mut __FUNCTION_PTRS.u_niagara_python_script_module_input_is_linked_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AsVec4"),
            &raw mut __FUNCTION_PTRS.u_niagara_python_script_module_input_as_vec4,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AsVec3"),
            &raw mut __FUNCTION_PTRS.u_niagara_python_script_module_input_as_vec3,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AsVec2"),
            &raw mut __FUNCTION_PTRS.u_niagara_python_script_module_input_as_vec2,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AsQuat"),
            &raw mut __FUNCTION_PTRS.u_niagara_python_script_module_input_as_quat,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AsLinkedValue"),
            &raw mut __FUNCTION_PTRS.u_niagara_python_script_module_input_as_linked_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AsInt"),
            &raw mut __FUNCTION_PTRS.u_niagara_python_script_module_input_as_int,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AsFloat"),
            &raw mut __FUNCTION_PTRS.u_niagara_python_script_module_input_as_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AsEnum"),
            &raw mut __FUNCTION_PTRS.u_niagara_python_script_module_input_as_enum,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AsColor"),
            &raw mut __FUNCTION_PTRS.u_niagara_python_script_module_input_as_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AsBool"),
            &raw mut __FUNCTION_PTRS.u_niagara_python_script_module_input_as_bool,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUpgradeNiagaraScriptResults::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVec4Input"),
            &raw mut __FUNCTION_PTRS.u_upgrade_niagara_script_results_set_vec4_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVec3Input"),
            &raw mut __FUNCTION_PTRS.u_upgrade_niagara_script_results_set_vec3_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVec2Input"),
            &raw mut __FUNCTION_PTRS.u_upgrade_niagara_script_results_set_vec2_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetQuatInput"),
            &raw mut __FUNCTION_PTRS.u_upgrade_niagara_script_results_set_quat_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNewInput"),
            &raw mut __FUNCTION_PTRS.u_upgrade_niagara_script_results_set_new_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLinkedInput"),
            &raw mut __FUNCTION_PTRS.u_upgrade_niagara_script_results_set_linked_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIntInput"),
            &raw mut __FUNCTION_PTRS.u_upgrade_niagara_script_results_set_int_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFloatInput"),
            &raw mut __FUNCTION_PTRS.u_upgrade_niagara_script_results_set_float_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnumInputFromInt"),
            &raw mut __FUNCTION_PTRS
                .u_upgrade_niagara_script_results_set_enum_input_from_int,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnumInput"),
            &raw mut __FUNCTION_PTRS.u_upgrade_niagara_script_results_set_enum_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetColorInput"),
            &raw mut __FUNCTION_PTRS.u_upgrade_niagara_script_results_set_color_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBoolInput"),
            &raw mut __FUNCTION_PTRS.u_upgrade_niagara_script_results_set_bool_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetToDefault"),
            &raw mut __FUNCTION_PTRS.u_upgrade_niagara_script_results_reset_to_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOldInput"),
            &raw mut __FUNCTION_PTRS.u_upgrade_niagara_script_results_get_old_input,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraPythonModule::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetObject"),
            &raw mut __FUNCTION_PTRS.u_niagara_python_module_get_object,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraPythonEmitter::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetProperties"),
            &raw mut __FUNCTION_PTRS.u_niagara_python_emitter_set_properties,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasModule"),
            &raw mut __FUNCTION_PTRS.u_niagara_python_emitter_has_module,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetProperties"),
            &raw mut __FUNCTION_PTRS.u_niagara_python_emitter_get_properties,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetObject"),
            &raw mut __FUNCTION_PTRS.u_niagara_python_emitter_get_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetModules"),
            &raw mut __FUNCTION_PTRS.u_niagara_python_emitter_get_modules,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetModule"),
            &raw mut __FUNCTION_PTRS.u_niagara_python_emitter_get_module,
        );
    }
}
#[repr(C, align(8))]
pub struct UActorFactoryNiagara {
    __padding_end: [u8; 144],
}
impl UActorFactoryNiagara {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryNiagara")
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
pub struct UNiagaraScriptFactoryNew {
    __padding_end: [u8; 136],
}
impl UNiagaraScriptFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraScriptFactoryNew")
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
pub struct UNiagaraModuleScriptFactory {
    __padding_end: [u8; 136],
}
impl UNiagaraModuleScriptFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraModuleScriptFactory")
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
pub struct UNiagaraFunctionScriptFactory {
    __padding_end: [u8; 136],
}
impl UNiagaraFunctionScriptFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraFunctionScriptFactory")
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
pub struct UNiagaraDynamicInputScriptFactory {
    __padding_end: [u8; 136],
}
impl UNiagaraDynamicInputScriptFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDynamicInputScriptFactory")
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
pub struct UAssetDefinition_NiagaraDataChannel {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_NiagaraDataChannel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_NiagaraDataChannel")
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
pub struct UAssetDefinition_NiagaraEffectType {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_NiagaraEffectType {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_NiagaraEffectType")
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
pub struct UAssetDefinition_NiagaraEmitter {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_NiagaraEmitter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_NiagaraEmitter")
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
pub struct UAssetDefinition_NiagaraParameterCollection {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_NiagaraParameterCollection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_NiagaraParameterCollection")
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
pub struct UAssetDefinition_NiagaraParameterCollectionInstance {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_NiagaraParameterCollectionInstance {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_NiagaraParameterCollectionInstance")
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
pub struct UAssetDefinition_NiagaraParameterDefinitions {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_NiagaraParameterDefinitions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_NiagaraParameterDefinitions")
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
pub struct UAssetDefinition_NiagaraScript {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_NiagaraScript {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_NiagaraScript")
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
pub struct UAssetDefinition_NiagaraSimCache {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_NiagaraSimCache {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_NiagaraSimCache")
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
pub struct UAssetDefinition_NiagaraStatelessEmitterTemplate {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_NiagaraStatelessEmitterTemplate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_NiagaraStatelessEmitterTemplate")
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
pub struct UAssetDefinition_NiagaraSystem {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_NiagaraSystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_NiagaraSystem")
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
pub struct UAssetDefinition_NiagaraValidationRuleSet {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_NiagaraValidationRuleSet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_NiagaraValidationRuleSet")
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
pub struct UNiagaraDumpByteCodeCommandlet {
    __padding_end: [u8; 208],
}
impl UNiagaraDumpByteCodeCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDumpByteCodeCommandlet")
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
pub struct UNiagaraDumpModuleInfoCommandlet {
    __padding_end: [u8; 184],
}
impl UNiagaraDumpModuleInfoCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDumpModuleInfoCommandlet")
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
pub struct UNiagaraScriptValidationCommandlet {
    __padding_end: [u8; 184],
}
impl UNiagaraScriptValidationCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraScriptValidationCommandlet")
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
pub struct UNiagaraStatelessAuditCommandlet {
    __padding_end: [u8; 240],
}
impl UNiagaraStatelessAuditCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessAuditCommandlet")
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
pub struct UNiagaraSystemAuditCommandlet {
    __padding_end: [u8; 1440],
}
impl UNiagaraSystemAuditCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSystemAuditCommandlet")
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
pub struct UNiagaraTraversalCacheAuditCommandlet {
    __padding_end: [u8; 160],
}
impl UNiagaraTraversalCacheAuditCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraTraversalCacheAuditCommandlet")
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
pub struct UNiagaraFavoriteActionsConfig {
    __padding_end: [u8; 128],
}
impl UNiagaraFavoriteActionsConfig {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraFavoriteActionsConfig")
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
pub struct UNiagaraObjectAssetHelper {
    __padding_end: [u8; 88],
}
impl UNiagaraObjectAssetHelper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraObjectAssetHelper")
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
pub struct UFNiagaraMemoryBufferSimCacheVisualizerSettings {
    __padding_end: [u8; 64],
}
impl UFNiagaraMemoryBufferSimCacheVisualizerSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFNiagaraMemoryBufferSimCacheVisualizerSettings")
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
pub struct UEdGraphSchema_Niagara {
    __padding_end: [u8; 48],
}
impl UEdGraphSchema_Niagara {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEdGraphSchema_Niagara")
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
pub struct UEdGraphSchema_NiagaraSystemOverview {
    __padding_end: [u8; 48],
}
impl UEdGraphSchema_NiagaraSystemOverview {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEdGraphSchema_NiagaraSystemOverview")
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
pub struct UNiagaraBakerFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UNiagaraBakerFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraBakerFunctionLibrary")
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
    pub fn capture_niagara_to_static_mesh(
        component_to_capture: UPtr<crate::bindings::niagara::UNiagaraComponent>,
        static_mesh_output: UPtr<crate::bindings::engine::UStaticMesh>,
        readback_parameters: crate::bindings::niagara::FNiagaraRendererReadbackParameters,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_baker_function_library_capture_niagara_to_static_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component_to_capture,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::niagara::UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh_output,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &readback_parameters,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::niagara::FNiagaraRendererReadbackParameters,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraBakerFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_baker_function_library_capture_niagara_to_static_mesh,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UNiagaraBakerStaticMeshFactoryNew {
    __padding_end: [u8; 136],
}
impl UNiagaraBakerStaticMeshFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraBakerStaticMeshFactoryNew")
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
pub struct UNiagaraClipboardFunctionInput {
    __padding_end: [u8; 240],
}
impl UNiagaraClipboardFunctionInput {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraClipboardFunctionInput")
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
pub struct UNiagaraClipboardRenderer {
    __padding_end: [u8; 112],
}
impl UNiagaraClipboardRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraClipboardRenderer")
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
pub struct UNiagaraClipboardFunction {
    __padding_end: [u8; 288],
}
impl UNiagaraClipboardFunction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraClipboardFunction")
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
pub struct UNiagaraClipboardContent {
    __padding_end: [u8; 240],
}
impl UNiagaraClipboardContent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraClipboardContent")
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
pub struct UNiagaraClipboardEditorScriptingUtilities {
    __padding_end: [u8; 48],
}
impl UNiagaraClipboardEditorScriptingUtilities {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraClipboardEditorScriptingUtilities")
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
    pub fn try_set_local_value_as_int(
        in_input: UPtr<UNiagaraClipboardFunctionInput>,
        b_out_succeeded: &mut bool,
        in_value: i32,
        b_loose_typing: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_try_set_local_value_as_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input,
                __buffer.add(0).cast::<UPtr<UNiagaraClipboardFunctionInput>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_succeeded,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_loose_typing,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_try_set_local_value_as_int,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<bool>().swap(b_out_succeeded);
        }
    }
    pub fn try_get_local_value_as_int(
        in_input: UPtr<UNiagaraClipboardFunctionInput>,
        b_out_succeeded: &mut bool,
        out_value: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_try_get_local_value_as_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input,
                __buffer.add(0).cast::<UPtr<UNiagaraClipboardFunctionInput>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_succeeded,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(12).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_try_get_local_value_as_int,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<bool>().swap(b_out_succeeded);
        }
        unsafe {
            __buffer.add(12).cast::<i32>().swap(out_value);
        }
    }
    pub fn try_get_local_value_as_float(
        in_input: UPtr<UNiagaraClipboardFunctionInput>,
        b_out_succeeded: &mut bool,
        out_value: &mut f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_try_get_local_value_as_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input,
                __buffer.add(0).cast::<UPtr<UNiagaraClipboardFunctionInput>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_succeeded,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(12).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_try_get_local_value_as_float,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<bool>().swap(b_out_succeeded);
        }
        unsafe {
            __buffer.add(12).cast::<f32>().swap(out_value);
        }
    }
    pub fn try_get_input_by_name(
        in_inputs: &TArray<UPtr<UNiagaraClipboardFunctionInput>>,
        in_input_name: FName,
        b_out_succeeded: &mut bool,
        out_input: &mut UPtr<UNiagaraClipboardFunctionInput>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_try_get_input_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_inputs,
                __buffer.add(0).cast::<TArray<UPtr<UNiagaraClipboardFunctionInput>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_succeeded,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_input,
                __buffer.add(32).cast::<UPtr<UNiagaraClipboardFunctionInput>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_try_get_input_by_name,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(28).cast::<bool>().swap(b_out_succeeded);
        }
        unsafe {
            __buffer
                .add(32)
                .cast::<UPtr<UNiagaraClipboardFunctionInput>>()
                .swap(out_input);
        }
    }
    pub fn get_type_name(in_input: UPtr<UNiagaraClipboardFunctionInput>) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_get_type_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input,
                __buffer.add(0).cast::<UPtr<UNiagaraClipboardFunctionInput>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_get_type_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FName>().read() }
    }
    pub fn create_vec4_local_value_input(
        in_outer: UPtr<crate::bindings::core_u_object::UObject>,
        in_input_name: FName,
        b_in_has_edit_condition: bool,
        b_in_edit_condition_value: bool,
        in_vec4_value: crate::bindings::core_u_object::FVector4f,
    ) -> UPtr<UNiagaraClipboardFunctionInput> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_vec4_local_value_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_outer,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_has_edit_condition,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_edit_condition_value,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_vec4_value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector4f>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_vec4_local_value_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<UNiagaraClipboardFunctionInput>>().read() }
    }
    pub fn create_vec3_local_value_input(
        in_outer: UPtr<crate::bindings::core_u_object::UObject>,
        in_input_name: FName,
        b_in_has_edit_condition: bool,
        b_in_edit_condition_value: bool,
        in_vec3_value: crate::bindings::core_u_object::FVector,
    ) -> UPtr<UNiagaraClipboardFunctionInput> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_vec3_local_value_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_outer,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_has_edit_condition,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_edit_condition_value,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_vec3_value,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_vec3_local_value_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<UNiagaraClipboardFunctionInput>>().read() }
    }
    pub fn create_vec2_local_value_input(
        in_outer: UPtr<crate::bindings::core_u_object::UObject>,
        in_input_name: FName,
        b_in_has_edit_condition: bool,
        b_in_edit_condition_value: bool,
        in_vec2_value: crate::bindings::core_u_object::FVector2D,
    ) -> UPtr<UNiagaraClipboardFunctionInput> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_vec2_local_value_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_outer,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_has_edit_condition,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_edit_condition_value,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_vec2_value,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_vec2_local_value_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<UPtr<UNiagaraClipboardFunctionInput>>().read() }
    }
    pub fn create_struct_local_value_input(
        in_outer: UPtr<crate::bindings::core_u_object::UObject>,
        in_input_name: FName,
        b_in_has_edit_condition: bool,
        b_in_edit_condition_value: bool,
        in_struct_value: UPtr<crate::bindings::core_u_object::UUserDefinedStruct>,
    ) -> UPtr<UNiagaraClipboardFunctionInput> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_struct_local_value_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_outer,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_has_edit_condition,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_edit_condition_value,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_struct_value,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::core_u_object::UUserDefinedStruct>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_struct_local_value_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<UPtr<UNiagaraClipboardFunctionInput>>().read() }
    }
    pub fn create_quat_local_value_input(
        in_outer: UPtr<crate::bindings::core_u_object::UObject>,
        in_input_name: FName,
        b_in_has_edit_condition: bool,
        b_in_edit_condition_value: bool,
        in_quat_value: crate::bindings::core_u_object::FQuat4f,
    ) -> UPtr<UNiagaraClipboardFunctionInput> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_quat_local_value_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_outer,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_has_edit_condition,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_edit_condition_value,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_quat_value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FQuat4f>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_quat_local_value_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<UNiagaraClipboardFunctionInput>>().read() }
    }
    pub fn create_matrix_local_value_input(
        in_outer: UPtr<crate::bindings::core_u_object::UObject>,
        in_input_name: FName,
        b_in_has_edit_condition: bool,
        b_in_edit_condition_value: bool,
        in_matrix_value: crate::bindings::core_u_object::FMatrix44f,
    ) -> UPtr<UNiagaraClipboardFunctionInput> {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_matrix_local_value_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_outer,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_has_edit_condition,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_edit_condition_value,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_matrix_value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FMatrix44f>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_matrix_local_value_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(96).cast::<UPtr<UNiagaraClipboardFunctionInput>>().read() }
    }
    pub fn create_linked_value_input(
        in_outer: UPtr<crate::bindings::core_u_object::UObject>,
        in_input_name: FName,
        in_input_type_name: FName,
        b_in_has_edit_condition: bool,
        b_in_edit_condition_value: bool,
        in_linked_value: FName,
    ) -> UPtr<UNiagaraClipboardFunctionInput> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_linked_value_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_outer,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_type_name,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_has_edit_condition,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_edit_condition_value,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_linked_value,
                __buffer.add(36).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_linked_value_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<UNiagaraClipboardFunctionInput>>().read() }
    }
    pub fn create_int_local_value_input(
        in_outer: UPtr<crate::bindings::core_u_object::UObject>,
        in_input_name: FName,
        b_in_has_edit_condition: bool,
        b_in_edit_condition_value: bool,
        in_local_value: i32,
    ) -> UPtr<UNiagaraClipboardFunctionInput> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_int_local_value_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_outer,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_has_edit_condition,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_edit_condition_value,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_local_value,
                __buffer.add(24).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_int_local_value_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<UPtr<UNiagaraClipboardFunctionInput>>().read() }
    }
    pub fn create_float_local_value_input(
        in_outer: UPtr<crate::bindings::core_u_object::UObject>,
        in_input_name: FName,
        b_in_has_edit_condition: bool,
        b_in_edit_condition_value: bool,
        in_local_value: f32,
    ) -> UPtr<UNiagaraClipboardFunctionInput> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_float_local_value_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_outer,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_has_edit_condition,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_edit_condition_value,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_local_value,
                __buffer.add(24).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_float_local_value_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<UPtr<UNiagaraClipboardFunctionInput>>().read() }
    }
    pub fn create_expression_value_input(
        in_outer: UPtr<crate::bindings::core_u_object::UObject>,
        in_input_name: FName,
        in_input_type_name: FName,
        b_in_has_edit_condition: bool,
        b_in_edit_condition_value: bool,
        in_expression_value: FString,
    ) -> UPtr<UNiagaraClipboardFunctionInput> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_expression_value_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_outer,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_type_name,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_has_edit_condition,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_edit_condition_value,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_expression_value,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_expression_value_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<UPtr<UNiagaraClipboardFunctionInput>>().read() }
    }
    pub fn create_enum_local_value_input(
        in_outer: UPtr<crate::bindings::core_u_object::UObject>,
        in_input_name: FName,
        b_in_has_edit_condition: bool,
        b_in_edit_codition_value: bool,
        in_enum_type: UPtr<crate::bindings::engine::UUserDefinedEnum>,
        in_enum_value: i32,
    ) -> UPtr<UNiagaraClipboardFunctionInput> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_enum_local_value_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_outer,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_has_edit_condition,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_edit_codition_value,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_enum_type,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::engine::UUserDefinedEnum>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_enum_value,
                __buffer.add(32).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_enum_local_value_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<UPtr<UNiagaraClipboardFunctionInput>>().read() }
    }
    pub fn create_enum_linked_value_input(
        in_outer: UPtr<crate::bindings::core_u_object::UObject>,
        in_input_name: FName,
        b_in_has_edit_condition: bool,
        b_in_edit_codition_value: bool,
        in_enum_type: UPtr<crate::bindings::engine::UUserDefinedEnum>,
        in_linked_value: FName,
    ) -> UPtr<UNiagaraClipboardFunctionInput> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_enum_linked_value_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_outer,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_has_edit_condition,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_edit_codition_value,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_enum_type,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::engine::UUserDefinedEnum>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_linked_value,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_enum_linked_value_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<UNiagaraClipboardFunctionInput>>().read() }
    }
    pub fn create_dynamic_value_input(
        in_outer: UPtr<crate::bindings::core_u_object::UObject>,
        in_input_name: FName,
        in_input_type_name: FName,
        b_in_has_edit_condition: bool,
        b_in_edit_condition_value: bool,
        in_dynamic_value_name: FString,
        in_dynamic_value: UPtr<crate::bindings::niagara::UNiagaraScript>,
    ) -> UPtr<UNiagaraClipboardFunctionInput> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_dynamic_value_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_outer,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_type_name,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_has_edit_condition,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_edit_condition_value,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_dynamic_value_name,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_dynamic_value,
                __buffer
                    .add(56)
                    .cast::<UPtr<crate::bindings::niagara::UNiagaraScript>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_dynamic_value_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<UPtr<UNiagaraClipboardFunctionInput>>().read() }
    }
    pub fn create_data_value_input(
        in_outer: UPtr<crate::bindings::core_u_object::UObject>,
        in_input_name: FName,
        b_in_has_edit_condition: bool,
        b_in_edit_condition_value: bool,
        in_data_value: UPtr<crate::bindings::niagara::UNiagaraDataInterface>,
    ) -> UPtr<UNiagaraClipboardFunctionInput> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_data_value_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_outer,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_has_edit_condition,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_edit_condition_value,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_data_value,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::niagara::UNiagaraDataInterface>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_data_value_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<UPtr<UNiagaraClipboardFunctionInput>>().read() }
    }
    pub fn create_color_local_value_input(
        in_outer: UPtr<crate::bindings::core_u_object::UObject>,
        in_input_name: FName,
        b_in_has_edit_condition: bool,
        b_in_edit_condition_value: bool,
        in_color_value: crate::bindings::core_u_object::FLinearColor,
    ) -> UPtr<UNiagaraClipboardFunctionInput> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_color_local_value_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_outer,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_has_edit_condition,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_edit_condition_value,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_color_value,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_color_local_value_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<UPtr<UNiagaraClipboardFunctionInput>>().read() }
    }
    pub fn create_bool_local_value_input(
        in_outer: UPtr<crate::bindings::core_u_object::UObject>,
        in_input_name: FName,
        b_in_has_edit_condition: bool,
        b_in_edit_condition_value: bool,
        in_bool_value: bool,
    ) -> UPtr<UNiagaraClipboardFunctionInput> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_bool_local_value_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_outer,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_has_edit_condition,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_edit_condition_value,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_bool_value,
                __buffer.add(22).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara_editor::UNiagaraClipboardEditorScriptingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_clipboard_editor_scripting_utilities_create_bool_local_value_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UNiagaraClipboardFunctionInput>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraConvertInPlaceEmitterAndSystemState {
    __padding_end: [u8; 48],
}
impl UNiagaraConvertInPlaceEmitterAndSystemState {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraConvertInPlaceEmitterAndSystemState")
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
pub struct UNiagaraDataChannelAssetFactoryNew {
    __padding_end: [u8; 136],
}
impl UNiagaraDataChannelAssetFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataChannelAssetFactoryNew")
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
pub struct UNiagaraEditorParametersAdapter {
    __padding_end: [u8; 64],
}
impl UNiagaraEditorParametersAdapter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraEditorParametersAdapter")
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
pub struct UNiagaraEditorSettings {
    __padding_end: [u8; 1488],
}
impl UNiagaraEditorSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraEditorSettings")
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
pub struct UNiagaraEffectTypeFactoryNew {
    __padding_end: [u8; 136],
}
impl UNiagaraEffectTypeFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraEffectTypeFactoryNew")
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
pub struct UNiagaraEmitterEditorData {
    __padding_end: [u8; 248],
}
impl UNiagaraEmitterEditorData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraEmitterEditorData")
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
pub struct UNiagaraEmitterFactoryNew {
    __padding_end: [u8; 152],
}
impl UNiagaraEmitterFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraEmitterFactoryNew")
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
pub struct UNiagaraGraph {
    __padding_end: [u8; 792],
}
impl UNiagaraGraph {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraGraph")
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
pub struct UNiagaraMessageData {
    __padding_end: [u8; 48],
}
impl UNiagaraMessageData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraMessageData")
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
pub struct UNiagaraMessageDataText {
    __padding_end: [u8; 96],
}
impl UNiagaraMessageDataText {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraMessageDataText")
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
pub struct UNiagaraNode {
    __padding_end: [u8; 256],
}
impl UNiagaraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNode")
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
pub struct UNiagaraNodeWithDynamicPins {
    __padding_end: [u8; 256],
}
impl UNiagaraNodeWithDynamicPins {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeWithDynamicPins")
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
pub struct UNiagaraNodeFunctionCall {
    __padding_end: [u8; 1192],
}
impl UNiagaraNodeFunctionCall {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeFunctionCall")
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
pub struct UNiagaraNodeAssignment {
    __padding_end: [u8; 1376],
}
impl UNiagaraNodeAssignment {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeAssignment")
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
pub struct UNiagaraNodeConvert {
    __padding_end: [u8; 392],
}
impl UNiagaraNodeConvert {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeConvert")
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
pub struct UNiagaraNodeCustomHlsl {
    __padding_end: [u8; 1264],
}
impl UNiagaraNodeCustomHlsl {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeCustomHlsl")
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
pub struct UNiagaraNodeDataSetBase {
    __padding_end: [u8; 312],
}
impl UNiagaraNodeDataSetBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeDataSetBase")
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
pub struct UNiagaraNodeEmitter {
    __padding_end: [u8; 344],
}
impl UNiagaraNodeEmitter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeEmitter")
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
pub struct UNiagaraNodeIf {
    __padding_end: [u8; 304],
}
impl UNiagaraNodeIf {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeIf")
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
pub struct UNiagaraNodeInput {
    __padding_end: [u8; 360],
}
impl UNiagaraNodeInput {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeInput")
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
pub struct UNiagaraNodeOp {
    __padding_end: [u8; 296],
}
impl UNiagaraNodeOp {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeOp")
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
pub struct UNiagaraNodeOutput {
    __padding_end: [u8; 296],
}
impl UNiagaraNodeOutput {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeOutput")
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
pub struct UNiagaraNodeOutputTag {
    __padding_end: [u8; 272],
}
impl UNiagaraNodeOutputTag {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeOutputTag")
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
pub struct UNiagaraNodeParameterMapBase {
    __padding_end: [u8; 280],
}
impl UNiagaraNodeParameterMapBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeParameterMapBase")
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
pub struct UNiagaraNodeParameterMapSet {
    __padding_end: [u8; 280],
}
impl UNiagaraNodeParameterMapSet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeParameterMapSet")
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
pub struct UNiagaraNodeParameterMapFor {
    __padding_end: [u8; 280],
}
impl UNiagaraNodeParameterMapFor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeParameterMapFor")
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
pub struct UNiagaraNodeParameterMapForWithContinue {
    __padding_end: [u8; 280],
}
impl UNiagaraNodeParameterMapForWithContinue {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeParameterMapForWithContinue")
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
pub struct UNiagaraNodeParameterMapForIndex {
    __padding_end: [u8; 256],
}
impl UNiagaraNodeParameterMapForIndex {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeParameterMapForIndex")
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
pub struct UNiagaraNodeParameterMapGet {
    __padding_end: [u8; 360],
}
impl UNiagaraNodeParameterMapGet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeParameterMapGet")
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
pub struct UNiagaraNodeReadDataSet {
    __padding_end: [u8; 312],
}
impl UNiagaraNodeReadDataSet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeReadDataSet")
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
pub struct UNiagaraNodeReroute {
    __padding_end: [u8; 264],
}
impl UNiagaraNodeReroute {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeReroute")
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
pub struct UNiagaraNodeUsageSelector {
    __padding_end: [u8; 312],
}
impl UNiagaraNodeUsageSelector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeUsageSelector")
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
pub struct UNiagaraNodeSelect {
    __padding_end: [u8; 376],
}
impl UNiagaraNodeSelect {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeSelect")
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
pub struct UNiagaraNodeSimTargetSelector {
    __padding_end: [u8; 312],
}
impl UNiagaraNodeSimTargetSelector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeSimTargetSelector")
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
pub struct UNiagaraNodeStaticSwitch {
    __padding_end: [u8; 384],
}
impl UNiagaraNodeStaticSwitch {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeStaticSwitch")
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
pub struct UNiagaraNodeWriteDataSet {
    __padding_end: [u8; 328],
}
impl UNiagaraNodeWriteDataSet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraNodeWriteDataSet")
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
pub struct UNiagaraOutliner {
    __padding_end: [u8; 288],
}
impl UNiagaraOutliner {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraOutliner")
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
pub struct UNiagaraOverviewNode {
    __padding_end: [u8; 224],
}
impl UNiagaraOverviewNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraOverviewNode")
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
pub struct UNiagaraParameterCollectionFactoryNew {
    __padding_end: [u8; 136],
}
impl UNiagaraParameterCollectionFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraParameterCollectionFactoryNew")
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
pub struct UNiagaraParameterCollectionInstanceFactoryNew {
    __padding_end: [u8; 144],
}
impl UNiagaraParameterCollectionInstanceFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraParameterCollectionInstanceFactoryNew")
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
pub struct UNiagaraParameterDefinitions {
    __padding_end: [u8; 136],
}
impl UNiagaraParameterDefinitions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraParameterDefinitions")
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
pub struct UNiagaraParameterDefinitionsFactory {
    __padding_end: [u8; 136],
}
impl UNiagaraParameterDefinitionsFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraParameterDefinitionsFactory")
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
pub struct UNiagaraScriptSource {
    __padding_end: [u8; 128],
}
impl UNiagaraScriptSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraScriptSource")
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
pub struct UNiagaraScriptVariable {
    __padding_end: [u8; 640],
}
impl UNiagaraScriptVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraScriptVariable")
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
pub struct UNiagaraSimCacheFactoryNew {
    __padding_end: [u8; 136],
}
impl UNiagaraSimCacheFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSimCacheFactoryNew")
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
pub struct UNiagaraStackEditorData {
    __padding_end: [u8; 920],
}
impl UNiagaraStackEditorData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackEditorData")
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
pub struct UNiagaraStatelessEmitterTemplateFactoryNew {
    __padding_end: [u8; 136],
}
impl UNiagaraStatelessEmitterTemplateFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessEmitterTemplateFactoryNew")
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
pub struct UNiagaraSystemEditorFolder {
    __padding_end: [u8; 96],
}
impl UNiagaraSystemEditorFolder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSystemEditorFolder")
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
#[repr(C, align(16))]
pub struct UNiagaraSystemEditorData {
    __padding_end: [u8; 368],
}
impl UNiagaraSystemEditorData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSystemEditorData")
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
pub struct UNiagaraSystemFactoryNew {
    __padding_end: [u8; 160],
}
impl UNiagaraSystemFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSystemFactoryNew")
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
pub struct UNiagaraThumbnailRendererBase {
    __padding_end: [u8; 48],
}
impl UNiagaraThumbnailRendererBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraThumbnailRendererBase")
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
pub struct UNiagaraEmitterThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UNiagaraEmitterThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraEmitterThumbnailRenderer")
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
pub struct UNiagaraSystemThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UNiagaraSystemThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSystemThumbnailRenderer")
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
pub struct UNiagaraValidationRule_NoWarmupTime {
    __padding_end: [u8; 56],
}
impl UNiagaraValidationRule_NoWarmupTime {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_NoWarmupTime")
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
pub struct UNiagaraValidationRule_NoEvents {
    __padding_end: [u8; 160],
}
impl UNiagaraValidationRule_NoEvents {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_NoEvents")
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
pub struct UNiagaraValidationRule_FixedGPUBoundsSet {
    __padding_end: [u8; 56],
}
impl UNiagaraValidationRule_FixedGPUBoundsSet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_FixedGPUBoundsSet")
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
pub struct UNiagaraValidationRule_EmitterCount {
    __padding_end: [u8; 80],
}
impl UNiagaraValidationRule_EmitterCount {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_EmitterCount")
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
pub struct UNiagaraValidationRule_RendererCount {
    __padding_end: [u8; 80],
}
impl UNiagaraValidationRule_RendererCount {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_RendererCount")
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
pub struct UNiagaraValidationRule_BannedRenderers {
    __padding_end: [u8; 176],
}
impl UNiagaraValidationRule_BannedRenderers {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_BannedRenderers")
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
pub struct UNiagaraValidationRule_Lightweight {
    __padding_end: [u8; 168],
}
impl UNiagaraValidationRule_Lightweight {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_Lightweight")
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
pub struct UNiagaraValidationRule_BannedModules {
    __padding_end: [u8; 176],
}
impl UNiagaraValidationRule_BannedModules {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_BannedModules")
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
pub struct UNiagaraValidationRule_BannedDataInterfaces {
    __padding_end: [u8; 176],
}
impl UNiagaraValidationRule_BannedDataInterfaces {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_BannedDataInterfaces")
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
pub struct UNiagaraValidationRule_RendererSortingEnabled {
    __padding_end: [u8; 160],
}
impl UNiagaraValidationRule_RendererSortingEnabled {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_RendererSortingEnabled")
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
pub struct UNiagaraValidationRule_GpuUsage {
    __padding_end: [u8; 160],
}
impl UNiagaraValidationRule_GpuUsage {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_GpuUsage")
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
pub struct UNiagaraValidationRule_RibbonRenderer {
    __padding_end: [u8; 160],
}
impl UNiagaraValidationRule_RibbonRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_RibbonRenderer")
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
pub struct UNiagaraValidationRule_InvalidEffectType {
    __padding_end: [u8; 56],
}
impl UNiagaraValidationRule_InvalidEffectType {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_InvalidEffectType")
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
pub struct UNiagaraValidationRule_HasEffectType {
    __padding_end: [u8; 64],
}
impl UNiagaraValidationRule_HasEffectType {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_HasEffectType")
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
pub struct UDEPRECATED_NiagaraValidationRule_CheckDeprecatedEmitters {
    __padding_end: [u8; 64],
}
impl UDEPRECATED_NiagaraValidationRule_CheckDeprecatedEmitters {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_NiagaraValidationRule_CheckDeprecatedEmitters")
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
pub struct UNiagaraValidationRule_LWC {
    __padding_end: [u8; 56],
}
impl UNiagaraValidationRule_LWC {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_LWC")
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
pub struct UNiagaraValidationRule_NoOpaqueRenderMaterial {
    __padding_end: [u8; 56],
}
impl UNiagaraValidationRule_NoOpaqueRenderMaterial {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_NoOpaqueRenderMaterial")
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
pub struct UNiagaraValidationRule_NoFixedDeltaTime {
    __padding_end: [u8; 56],
}
impl UNiagaraValidationRule_NoFixedDeltaTime {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_NoFixedDeltaTime")
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
pub struct UNiagaraValidationRule_SimulationStageBudget {
    __padding_end: [u8; 80],
}
impl UNiagaraValidationRule_SimulationStageBudget {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_SimulationStageBudget")
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
pub struct UNiagaraValidationRule_TickDependencyCheck {
    __padding_end: [u8; 80],
}
impl UNiagaraValidationRule_TickDependencyCheck {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_TickDependencyCheck")
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
pub struct UNiagaraValidationRule_UserDataInterfaces {
    __padding_end: [u8; 104],
}
impl UNiagaraValidationRule_UserDataInterfaces {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_UserDataInterfaces")
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
pub struct UNiagaraValidationRule_SingletonModule {
    __padding_end: [u8; 64],
}
impl UNiagaraValidationRule_SingletonModule {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_SingletonModule")
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
pub struct UNiagaraValidationRule_NoMapForOnCpu {
    __padding_end: [u8; 144],
}
impl UNiagaraValidationRule_NoMapForOnCpu {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_NoMapForOnCpu")
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
pub struct UNiagaraValidationRule_ModuleSimTargetRestriction {
    __padding_end: [u8; 64],
}
impl UNiagaraValidationRule_ModuleSimTargetRestriction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_ModuleSimTargetRestriction")
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
pub struct UNiagaraValidationRule_MaterialUsage {
    __padding_end: [u8; 64],
}
impl UNiagaraValidationRule_MaterialUsage {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule_MaterialUsage")
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
pub struct UNiagaraValidationRuleSetFactoryNew {
    __padding_end: [u8; 136],
}
impl UNiagaraValidationRuleSetFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRuleSetFactoryNew")
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
pub struct UNiagaraVersionMetaData {
    __padding_end: [u8; 152],
}
impl UNiagaraVersionMetaData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraVersionMetaData")
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
pub struct UMovieSceneNiagaraEmitterSectionBase {
    __padding_end: [u8; 400],
}
impl UMovieSceneNiagaraEmitterSectionBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNiagaraEmitterSectionBase")
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
pub struct UMovieSceneNiagaraEmitterTrack {
    __padding_end: [u8; 496],
}
impl UMovieSceneNiagaraEmitterTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNiagaraEmitterTrack")
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
pub struct UNiagaraSequence {
    __padding_end: [u8; 144],
}
impl UNiagaraSequence {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSequence")
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
pub struct UNiagaraSequencerTrackFilter {
    __padding_end: [u8; 48],
}
impl UNiagaraSequencerTrackFilter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSequencerTrackFilter")
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
pub struct UMovieSceneNiagaraEmitterSection {
    __padding_end: [u8; 1832],
}
impl UMovieSceneNiagaraEmitterSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNiagaraEmitterSection")
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
pub struct UNiagaraPythonScriptModuleInput {
    __padding_end: [u8; 56],
}
impl UNiagaraPythonScriptModuleInput {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraPythonScriptModuleInput")
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
    pub fn is_set(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_is_set,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_is_set,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_local_value(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_is_local_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_is_local_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_linked_value(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_is_linked_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_is_linked_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn as_vec4(&self) -> crate::bindings::core_u_object::FVector4 {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_as_vec4,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_as_vec4,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector4>().read()
        }
    }
    pub fn as_vec3(&self) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_as_vec3,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_as_vec3,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn as_vec2(&self) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_as_vec2,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_as_vec2,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn as_quat(&self) -> crate::bindings::core_u_object::FQuat {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_as_quat,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_as_quat,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<crate::bindings::core_u_object::FQuat>().read() }
    }
    pub fn as_linked_value(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_as_linked_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_as_linked_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn as_int(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_as_int,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_as_int,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn as_float(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_as_float,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_as_float,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn as_enum(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_as_enum,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_as_enum,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn as_color(&self) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_as_color,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_as_color,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>().read()
        }
    }
    pub fn as_bool(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_as_bool,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_script_module_input_as_bool,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UUpgradeNiagaraScriptResults {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub b_cancelled_by_python_error: bool,
    pub old_inputs: TArray<UPtr<UNiagaraPythonScriptModuleInput>>,
    pub new_inputs: TArray<UPtr<UNiagaraPythonScriptModuleInput>>,
    __padding_end: [u8; 8],
}
impl UUpgradeNiagaraScriptResults {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUpgradeNiagaraScriptResults")
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
    pub fn set_vec4_input(
        &mut self,
        input_name: FString,
        value: crate::bindings::core_u_object::FVector4,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_vec4_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_vec4_input,
                __buffer,
            )
        };
    }
    pub fn set_vec3_input(
        &mut self,
        input_name: FString,
        value: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_vec3_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_vec3_input,
                __buffer,
            )
        };
    }
    pub fn set_vec2_input(
        &mut self,
        input_name: FString,
        value: crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_vec2_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_vec2_input,
                __buffer,
            )
        };
    }
    pub fn set_quat_input(
        &mut self,
        input_name: FString,
        value: crate::bindings::core_u_object::FQuat,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_quat_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_quat_input,
                __buffer,
            )
        };
    }
    pub fn set_new_input(
        &mut self,
        input_name: FString,
        value: UPtr<UNiagaraPythonScriptModuleInput>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_new_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(16).cast::<UPtr<UNiagaraPythonScriptModuleInput>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_new_input,
                __buffer,
            )
        };
    }
    pub fn set_linked_input(&mut self, input_name: FString, value: FString) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_linked_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(16).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_linked_input,
                __buffer,
            )
        };
    }
    pub fn set_int_input(&mut self, input_name: FString, value: i32) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_int_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_int_input,
                __buffer,
            )
        };
    }
    pub fn set_float_input(&mut self, input_name: FString, value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_float_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(16).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_float_input,
                __buffer,
            )
        };
    }
    pub fn set_enum_input_from_int(&mut self, input_name: FString, value: i32) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_enum_input_from_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_enum_input_from_int,
                __buffer,
            )
        };
    }
    pub fn set_enum_input(&mut self, input_name: FString, value: FString) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_enum_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(16).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_enum_input,
                __buffer,
            )
        };
    }
    pub fn set_color_input(
        &mut self,
        input_name: FString,
        value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_color_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_color_input,
                __buffer,
            )
        };
    }
    pub fn set_bool_input(&mut self, input_name: FString, value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_bool_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_set_bool_input,
                __buffer,
            )
        };
    }
    pub fn reset_to_default(&mut self, input_name: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_reset_to_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_reset_to_default,
                __buffer,
            )
        };
    }
    pub fn get_old_input(
        &mut self,
        input_name: FString,
    ) -> UPtr<UNiagaraPythonScriptModuleInput> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_get_old_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_upgrade_niagara_script_results_get_old_input,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<UPtr<UNiagaraPythonScriptModuleInput>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraPythonModule {
    __padding_end: [u8; 56],
}
impl UNiagaraPythonModule {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraPythonModule")
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
    pub fn get_object(&self) -> UPtr<UNiagaraStackModuleItem> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_module_get_object,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_module_get_object,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UNiagaraStackModuleItem>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraPythonEmitter {
    __padding_end: [u8; 64],
}
impl UNiagaraPythonEmitter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraPythonEmitter")
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
    pub fn set_properties(
        &mut self,
        data: crate::bindings::niagara::FVersionedNiagaraEmitterData,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1640>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_emitter_set_properties,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::niagara::FVersionedNiagaraEmitterData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_emitter_set_properties,
                __buffer,
            )
        };
    }
    pub fn has_module(&self, module_name: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_emitter_has_module,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &module_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_emitter_has_module,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_properties(
        &self,
    ) -> crate::bindings::niagara::FVersionedNiagaraEmitterData {
        let mut __stack = crate::core_data::StackAlloc::<1640>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_emitter_get_properties,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_emitter_get_properties,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::niagara::FVersionedNiagaraEmitterData>()
                .read()
        }
    }
    pub fn get_object(&mut self) -> UPtr<crate::bindings::niagara::UNiagaraEmitter> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_emitter_get_object,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_emitter_get_object,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::niagara::UNiagaraEmitter>>()
                .read()
        }
    }
    pub fn get_modules(&self) -> TArray<UPtr<UNiagaraPythonModule>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_emitter_get_modules,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_emitter_get_modules,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<UNiagaraPythonModule>>>().read() }
    }
    pub fn get_module(&self, module_name: FString) -> UPtr<UNiagaraPythonModule> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_emitter_get_module,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &module_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara_editor::__FUNCTION_PTRS
                    .u_niagara_python_emitter_get_module,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UNiagaraPythonModule>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UUpgradeNiagaraEmitterContext {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub b_cancelled_by_python_error: bool,
    pub old_emitter: UPtr<UNiagaraPythonEmitter>,
    pub new_emitter: UPtr<UNiagaraPythonEmitter>,
    __padding_end: [u8; 16],
}
impl UUpgradeNiagaraEmitterContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUpgradeNiagaraEmitterContext")
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
pub struct UNiagaraHierarchyScriptParameterRefreshContext {
    __padding_end: [u8; 56],
}
impl UNiagaraHierarchyScriptParameterRefreshContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraHierarchyScriptParameterRefreshContext")
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
pub struct UNiagaraHierarchyScriptParameter {
    __padding_end: [u8; 192],
}
impl UNiagaraHierarchyScriptParameter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraHierarchyScriptParameter")
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
pub struct UNiagaraHierarchyScriptCategory {
    __padding_end: [u8; 232],
}
impl UNiagaraHierarchyScriptCategory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraHierarchyScriptCategory")
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
pub struct UNiagaraScriptParametersHierarchyViewModel {
    __padding_end: [u8; 504],
}
impl UNiagaraScriptParametersHierarchyViewModel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraScriptParametersHierarchyViewModel")
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
pub struct UNiagaraHierarchySummaryDataRefreshContext {
    __padding_end: [u8; 80],
}
impl UNiagaraHierarchySummaryDataRefreshContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraHierarchySummaryDataRefreshContext")
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
pub struct UNiagaraHierarchyModule {
    __padding_end: [u8; 192],
}
impl UNiagaraHierarchyModule {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraHierarchyModule")
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
pub struct UNiagaraHierarchyModuleInput {
    __padding_end: [u8; 224],
}
impl UNiagaraHierarchyModuleInput {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraHierarchyModuleInput")
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
pub struct UNiagaraHierarchyAssignmentInput {
    __padding_end: [u8; 208],
}
impl UNiagaraHierarchyAssignmentInput {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraHierarchyAssignmentInput")
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
pub struct UNiagaraHierarchyEmitterProperties {
    __padding_end: [u8; 192],
}
impl UNiagaraHierarchyEmitterProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraHierarchyEmitterProperties")
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
pub struct UNiagaraHierarchyRenderer {
    __padding_end: [u8; 192],
}
impl UNiagaraHierarchyRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraHierarchyRenderer")
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
pub struct UNiagaraHierarchyEventHandler {
    __padding_end: [u8; 192],
}
impl UNiagaraHierarchyEventHandler {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraHierarchyEventHandler")
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
pub struct UNiagaraHierarchyEventHandlerProperties {
    __padding_end: [u8; 192],
}
impl UNiagaraHierarchyEventHandlerProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraHierarchyEventHandlerProperties")
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
pub struct UNiagaraHierarchySimStage {
    __padding_end: [u8; 192],
}
impl UNiagaraHierarchySimStage {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraHierarchySimStage")
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
pub struct UNiagaraHierarchySimStageProperties {
    __padding_end: [u8; 192],
}
impl UNiagaraHierarchySimStageProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraHierarchySimStageProperties")
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
pub struct UNiagaraHierarchyObjectProperty {
    __padding_end: [u8; 192],
}
impl UNiagaraHierarchyObjectProperty {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraHierarchyObjectProperty")
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
pub struct UNiagaraSummaryViewViewModel {
    __padding_end: [u8; 584],
}
impl UNiagaraSummaryViewViewModel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSummaryViewViewModel")
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
pub struct UNiagaraHierarchyUserParameterRefreshContext {
    __padding_end: [u8; 56],
}
impl UNiagaraHierarchyUserParameterRefreshContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraHierarchyUserParameterRefreshContext")
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
pub struct UNiagaraHierarchyUserParameter {
    __padding_end: [u8; 200],
}
impl UNiagaraHierarchyUserParameter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraHierarchyUserParameter")
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
pub struct UNiagaraUserParametersHierarchyViewModel {
    __padding_end: [u8; 504],
}
impl UNiagaraUserParametersHierarchyViewModel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraUserParametersHierarchyViewModel")
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
pub struct UNiagaraCurveSelectionViewModel {
    __padding_end: [u8; 152],
}
impl UNiagaraCurveSelectionViewModel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraCurveSelectionViewModel")
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
pub struct UNiagaraScratchPadViewModel {
    __padding_end: [u8; 328],
}
impl UNiagaraScratchPadViewModel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraScratchPadViewModel")
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
pub struct UNiagaraScripStatsViewModelSettings {
    __padding_end: [u8; 64],
}
impl UNiagaraScripStatsViewModelSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraScripStatsViewModelSettings")
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
pub struct UNiagaraSystemEditorDocumentsViewModel {
    __padding_end: [u8; 208],
}
impl UNiagaraSystemEditorDocumentsViewModel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSystemEditorDocumentsViewModel")
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
pub struct UNiagaraSystemScalabilityViewModel {
    __padding_end: [u8; 160],
}
impl UNiagaraSystemScalabilityViewModel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSystemScalabilityViewModel")
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
pub struct UNiagaraSystemSelectionViewModel {
    __padding_end: [u8; 256],
}
impl UNiagaraSystemSelectionViewModel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSystemSelectionViewModel")
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
pub struct UNiagaraStackEntry {
    __padding_end: [u8; 616],
}
impl UNiagaraStackEntry {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackEntry")
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
pub struct UNiagaraStackCommentCollection {
    __padding_end: [u8; 616],
}
impl UNiagaraStackCommentCollection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackCommentCollection")
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
pub struct UNiagaraStackItemGroup {
    __padding_end: [u8; 696],
}
impl UNiagaraStackItemGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackItemGroup")
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
pub struct UNiagaraStackEmitterPropertiesGroup {
    __padding_end: [u8; 720],
}
impl UNiagaraStackEmitterPropertiesGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackEmitterPropertiesGroup")
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
pub struct UNiagaraStackItem {
    __padding_end: [u8; 704],
}
impl UNiagaraStackItem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackItem")
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
pub struct UNiagaraStackEmitterPropertiesItem {
    __padding_end: [u8; 744],
}
impl UNiagaraStackEmitterPropertiesItem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackEmitterPropertiesItem")
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
pub struct UNiagaraStackEmitterSummaryItem {
    __padding_end: [u8; 736],
}
impl UNiagaraStackEmitterSummaryItem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackEmitterSummaryItem")
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
pub struct UNiagaraStackEmitterSummaryGroup {
    __padding_end: [u8; 704],
}
impl UNiagaraStackEmitterSummaryGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackEmitterSummaryGroup")
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
pub struct UNiagaraStackSpacer {
    __padding_end: [u8; 656],
}
impl UNiagaraStackSpacer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackSpacer")
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
pub struct UNiagaraStackErrorItem {
    __padding_end: [u8; 776],
}
impl UNiagaraStackErrorItem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackErrorItem")
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
pub struct UNiagaraStackErrorItemLongDescription {
    __padding_end: [u8; 720],
}
impl UNiagaraStackErrorItemLongDescription {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackErrorItemLongDescription")
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
pub struct UNiagaraStackErrorItemFix {
    __padding_end: [u8; 816],
}
impl UNiagaraStackErrorItemFix {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackErrorItemFix")
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
pub struct UNiagaraStackErrorItemDismiss {
    __padding_end: [u8; 816],
}
impl UNiagaraStackErrorItemDismiss {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackErrorItemDismiss")
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
pub struct UNiagaraStackEventWrapper {
    __padding_end: [u8; 88],
}
impl UNiagaraStackEventWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackEventWrapper")
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
pub struct UNiagaraStackEventHandlerPropertiesItem {
    __padding_end: [u8; 768],
}
impl UNiagaraStackEventHandlerPropertiesItem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackEventHandlerPropertiesItem")
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
pub struct UNiagaraStackScriptItemGroup {
    __padding_end: [u8; 784],
}
impl UNiagaraStackScriptItemGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackScriptItemGroup")
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
pub struct UNiagaraStackEventScriptItemGroup {
    __padding_end: [u8; 840],
}
impl UNiagaraStackEventScriptItemGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackEventScriptItemGroup")
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
pub struct UNiagaraStackItemContent {
    __padding_end: [u8; 640],
}
impl UNiagaraStackItemContent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackItemContent")
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
pub struct UNiagaraStackFunctionInput {
    __padding_end: [u8; 3008],
}
impl UNiagaraStackFunctionInput {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackFunctionInput")
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
pub struct UNiagaraStackCategory {
    __padding_end: [u8; 656],
}
impl UNiagaraStackCategory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackCategory")
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
pub struct UNiagaraStackScriptHierarchyCategory {
    __padding_end: [u8; 840],
}
impl UNiagaraStackScriptHierarchyCategory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackScriptHierarchyCategory")
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
pub struct UNiagaraStackSummaryCategory {
    __padding_end: [u8; 672],
}
impl UNiagaraStackSummaryCategory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackSummaryCategory")
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
pub struct UNiagaraStackItemTextContent {
    __padding_end: [u8; 656],
}
impl UNiagaraStackItemTextContent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackItemTextContent")
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
pub struct UNiagaraStackItemFooter {
    __padding_end: [u8; 664],
}
impl UNiagaraStackItemFooter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackItemFooter")
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
pub struct UNiagaraStackItemGroupFooter {
    __padding_end: [u8; 616],
}
impl UNiagaraStackItemGroupFooter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackItemGroupFooter")
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
pub struct UNiagaraStackModuleItem {
    __padding_end: [u8; 896],
}
impl UNiagaraStackModuleItem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackModuleItem")
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
pub struct UNiagaraStackModuleItemLinkedInputCollection {
    __padding_end: [u8; 664],
}
impl UNiagaraStackModuleItemLinkedInputCollection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackModuleItemLinkedInputCollection")
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
pub struct UNiagaraStackModuleItemOutput {
    __padding_end: [u8; 736],
}
impl UNiagaraStackModuleItemOutput {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackModuleItemOutput")
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
pub struct UNiagaraStackModuleItemOutputCollection {
    __padding_end: [u8; 664],
}
impl UNiagaraStackModuleItemOutputCollection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackModuleItemOutputCollection")
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
pub struct UNiagaraStackNote {
    __padding_end: [u8; 680],
}
impl UNiagaraStackNote {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackNote")
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
pub struct UNiagaraStackObject {
    __padding_end: [u8; 848],
}
impl UNiagaraStackObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackObject")
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
pub struct UNiagaraStackPropertyRow {
    __padding_end: [u8; 744],
}
impl UNiagaraStackPropertyRow {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackPropertyRow")
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
pub struct UNiagaraStackRendererItem {
    __padding_end: [u8; 776],
}
impl UNiagaraStackRendererItem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackRendererItem")
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
pub struct UNiagaraStackRenderItemGroup {
    __padding_end: [u8; 728],
}
impl UNiagaraStackRenderItemGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackRenderItemGroup")
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
pub struct UNiagaraStackRoot {
    __padding_end: [u8; 624],
}
impl UNiagaraStackRoot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackRoot")
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
pub struct UNiagaraStackScriptHierarchyRoot {
    __padding_end: [u8; 832],
}
impl UNiagaraStackScriptHierarchyRoot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackScriptHierarchyRoot")
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
pub struct UNiagaraStackSelection {
    __padding_end: [u8; 632],
}
impl UNiagaraStackSelection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackSelection")
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
pub struct UNiagaraStackSimulationStagePropertiesItem {
    __padding_end: [u8; 728],
}
impl UNiagaraStackSimulationStagePropertiesItem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackSimulationStagePropertiesItem")
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
pub struct UNiagaraStackSimulationStageGroup {
    __padding_end: [u8; 848],
}
impl UNiagaraStackSimulationStageGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackSimulationStageGroup")
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
pub struct UNiagaraStackStatelessEmitterGroup {
    __padding_end: [u8; 720],
}
impl UNiagaraStackStatelessEmitterGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackStatelessEmitterGroup")
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
pub struct UNiagaraStackStatelessEmitterObjectItem {
    __padding_end: [u8; 768],
}
impl UNiagaraStackStatelessEmitterObjectItem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackStatelessEmitterObjectItem")
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
pub struct UNiagaraStackStatelessEmitterSimulateGroup {
    __padding_end: [u8; 720],
}
impl UNiagaraStackStatelessEmitterSimulateGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackStatelessEmitterSimulateGroup")
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
pub struct UNiagaraStackStatelessModuleItem {
    __padding_end: [u8; 760],
}
impl UNiagaraStackStatelessModuleItem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackStatelessModuleItem")
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
pub struct UNiagaraStackStatelessEmitterSpawnGroup {
    __padding_end: [u8; 720],
}
impl UNiagaraStackStatelessEmitterSpawnGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackStatelessEmitterSpawnGroup")
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
pub struct UNiagaraStackStatelessEmitterSpawnItem {
    __padding_end: [u8; 808],
}
impl UNiagaraStackStatelessEmitterSpawnItem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackStatelessEmitterSpawnItem")
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
pub struct UNiagaraStackValueCollection {
    __padding_end: [u8; 856],
}
impl UNiagaraStackValueCollection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackValueCollection")
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
pub struct UNiagaraStackSummaryViewCollection {
    __padding_end: [u8; 880],
}
impl UNiagaraStackSummaryViewCollection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackSummaryViewCollection")
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
pub struct UNiagaraStackSystemPropertiesItem {
    __padding_end: [u8; 728],
}
impl UNiagaraStackSystemPropertiesItem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackSystemPropertiesItem")
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
pub struct UNiagaraStackSystemPropertiesGroup {
    __padding_end: [u8; 696],
}
impl UNiagaraStackSystemPropertiesGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackSystemPropertiesGroup")
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
pub struct UNiagaraStackSystemUserParametersGroup {
    __padding_end: [u8; 712],
}
impl UNiagaraStackSystemUserParametersGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackSystemUserParametersGroup")
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
pub struct UNiagaraStackViewModel {
    __padding_end: [u8; 488],
}
impl UNiagaraStackViewModel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStackViewModel")
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
pub struct UVolumeCacheFactory {
    __padding_end: [u8; 136],
}
impl UVolumeCacheFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVolumeCacheFactory")
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
pub struct UNiagaraDataChannelReadModuleData {
    __padding_end: [u8; 64],
}
impl UNiagaraDataChannelReadModuleData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataChannelReadModuleData")
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
pub struct UNiagaraDataChannelSpawnModuleData {
    __padding_end: [u8; 64],
}
impl UNiagaraDataChannelSpawnModuleData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataChannelSpawnModuleData")
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
pub struct UNiagaraDataChannelWriteModuleData {
    __padding_end: [u8; 72],
}
impl UNiagaraDataChannelWriteModuleData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataChannelWriteModuleData")
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
pub struct FNiagaraClipboardFunction_OnPastedFunctionCallNodeDelegate {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct ENiagaraNamespaceMetadataOptions(pub i32);
impl ENiagaraNamespaceMetadataOptions {
    pub const HIDE_IN_SCRIPT: ENiagaraNamespaceMetadataOptions = ENiagaraNamespaceMetadataOptions(
        0,
    );
    pub const HIDE_IN_SYSTEM: ENiagaraNamespaceMetadataOptions = ENiagaraNamespaceMetadataOptions(
        1,
    );
    pub const ADVANCED_IN_SCRIPT: ENiagaraNamespaceMetadataOptions = ENiagaraNamespaceMetadataOptions(
        2,
    );
    pub const ADVANCED_IN_SYSTEM: ENiagaraNamespaceMetadataOptions = ENiagaraNamespaceMetadataOptions(
        3,
    );
    pub const PREVENT_EDITING_NAMESPACE: ENiagaraNamespaceMetadataOptions = ENiagaraNamespaceMetadataOptions(
        4,
    );
    pub const PREVENT_EDITING_NAMESPACE_MODIFIER: ENiagaraNamespaceMetadataOptions = ENiagaraNamespaceMetadataOptions(
        5,
    );
    pub const PREVENT_EDITING_NAME: ENiagaraNamespaceMetadataOptions = ENiagaraNamespaceMetadataOptions(
        6,
    );
    pub const PREVENT_CREATING_IN_SYSTEM_EDITOR: ENiagaraNamespaceMetadataOptions = ENiagaraNamespaceMetadataOptions(
        7,
    );
    pub const HIDE_IN_DEFINITIONS: ENiagaraNamespaceMetadataOptions = ENiagaraNamespaceMetadataOptions(
        8,
    );
}
#[repr(transparent)]
pub struct ENiagaraMessageSeverity(pub u8);
impl ENiagaraMessageSeverity {
    pub const CRITICAL_ERROR: ENiagaraMessageSeverity = ENiagaraMessageSeverity(0);
    pub const ERROR: ENiagaraMessageSeverity = ENiagaraMessageSeverity(1);
    pub const PERFORMANCE_WARNING: ENiagaraMessageSeverity = ENiagaraMessageSeverity(2);
    pub const WARNING: ENiagaraMessageSeverity = ENiagaraMessageSeverity(3);
    pub const INFO: ENiagaraMessageSeverity = ENiagaraMessageSeverity(4);
}
#[repr(transparent)]
pub struct ENiagaraStaticSwitchType(pub u8);
impl ENiagaraStaticSwitchType {
    pub const BOOL: ENiagaraStaticSwitchType = ENiagaraStaticSwitchType(0);
    pub const INTEGER: ENiagaraStaticSwitchType = ENiagaraStaticSwitchType(1);
    pub const ENUM: ENiagaraStaticSwitchType = ENiagaraStaticSwitchType(2);
}
#[repr(transparent)]
pub struct ENiagaraOutlinerViewModes(pub u8);
impl ENiagaraOutlinerViewModes {
    pub const STATE: ENiagaraOutlinerViewModes = ENiagaraOutlinerViewModes(0);
    pub const PERFORMANCE: ENiagaraOutlinerViewModes = ENiagaraOutlinerViewModes(1);
    pub const DEBUG: ENiagaraOutlinerViewModes = ENiagaraOutlinerViewModes(2);
}
#[repr(transparent)]
pub struct ENiagaraOutlinerSortMode(pub u8);
impl ENiagaraOutlinerSortMode {
    pub const AUTO: ENiagaraOutlinerSortMode = ENiagaraOutlinerSortMode(0);
    pub const FILTER_MATCHES: ENiagaraOutlinerSortMode = ENiagaraOutlinerSortMode(1);
    pub const AVERAGE_TIME: ENiagaraOutlinerSortMode = ENiagaraOutlinerSortMode(2);
    pub const MAX_TIME: ENiagaraOutlinerSortMode = ENiagaraOutlinerSortMode(3);
}
#[repr(transparent)]
pub struct ENiagaraOutlinerTimeUnits(pub u8);
impl ENiagaraOutlinerTimeUnits {
    pub const MICROSECONDS: ENiagaraOutlinerTimeUnits = ENiagaraOutlinerTimeUnits(0);
    pub const MILLISECONDS: ENiagaraOutlinerTimeUnits = ENiagaraOutlinerTimeUnits(1);
    pub const SECONDS: ENiagaraOutlinerTimeUnits = ENiagaraOutlinerTimeUnits(2);
}
#[repr(transparent)]
pub struct ENDIMemoryBufferViewType(pub i32);
impl ENDIMemoryBufferViewType {
    pub const FLOAT: ENDIMemoryBufferViewType = ENDIMemoryBufferViewType(0);
    pub const INTEGER: ENDIMemoryBufferViewType = ENDIMemoryBufferViewType(1);
    pub const UNSIGNED_INTEGER: ENDIMemoryBufferViewType = ENDIMemoryBufferViewType(2);
    pub const HEX: ENDIMemoryBufferViewType = ENDIMemoryBufferViewType(3);
}
#[repr(transparent)]
pub struct ENiagaraClipboardFunctionInputValueMode(pub i32);
impl ENiagaraClipboardFunctionInputValueMode {
    pub const LOCAL: ENiagaraClipboardFunctionInputValueMode = ENiagaraClipboardFunctionInputValueMode(
        0,
    );
    pub const LINKED: ENiagaraClipboardFunctionInputValueMode = ENiagaraClipboardFunctionInputValueMode(
        1,
    );
    pub const DATA: ENiagaraClipboardFunctionInputValueMode = ENiagaraClipboardFunctionInputValueMode(
        2,
    );
    pub const OBJECT_ASSET: ENiagaraClipboardFunctionInputValueMode = ENiagaraClipboardFunctionInputValueMode(
        3,
    );
    pub const EXPRESSION: ENiagaraClipboardFunctionInputValueMode = ENiagaraClipboardFunctionInputValueMode(
        4,
    );
    pub const DYNAMIC: ENiagaraClipboardFunctionInputValueMode = ENiagaraClipboardFunctionInputValueMode(
        5,
    );
    pub const RESET_TO_DEFAULT: ENiagaraClipboardFunctionInputValueMode = ENiagaraClipboardFunctionInputValueMode(
        6,
    );
}
#[repr(transparent)]
pub struct ENiagaraClipboardFunctionScriptMode(pub i32);
impl ENiagaraClipboardFunctionScriptMode {
    pub const SCRIPT_ASSET: ENiagaraClipboardFunctionScriptMode = ENiagaraClipboardFunctionScriptMode(
        0,
    );
    pub const ASSIGNMENT: ENiagaraClipboardFunctionScriptMode = ENiagaraClipboardFunctionScriptMode(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraCategoryExpandState(pub u8);
impl ENiagaraCategoryExpandState {
    pub const DEFAULT: ENiagaraCategoryExpandState = ENiagaraCategoryExpandState(0);
    pub const DEFAULT_EXPAND_MODIFIED: ENiagaraCategoryExpandState = ENiagaraCategoryExpandState(
        1,
    );
    pub const COLLAPSE_ALL: ENiagaraCategoryExpandState = ENiagaraCategoryExpandState(2);
    pub const EXPAND_ALL: ENiagaraCategoryExpandState = ENiagaraCategoryExpandState(3);
}
#[repr(transparent)]
pub struct ENiagaraAddDefaultsTrackMode(pub u8);
impl ENiagaraAddDefaultsTrackMode {
    pub const NO_SUBTRACKS: ENiagaraAddDefaultsTrackMode = ENiagaraAddDefaultsTrackMode(
        0,
    );
    pub const COMPONENT_TRACK_ONLY: ENiagaraAddDefaultsTrackMode = ENiagaraAddDefaultsTrackMode(
        1,
    );
    pub const LIFECYCLE_TRACK: ENiagaraAddDefaultsTrackMode = ENiagaraAddDefaultsTrackMode(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraStackEntryInlineDisplayMode(pub i32);
impl ENiagaraStackEntryInlineDisplayMode {
    pub const EXPRESSION: ENiagaraStackEntryInlineDisplayMode = ENiagaraStackEntryInlineDisplayMode(
        0,
    );
    pub const GRAPH_HORIZONTAL: ENiagaraStackEntryInlineDisplayMode = ENiagaraStackEntryInlineDisplayMode(
        1,
    );
    pub const GRAPH_VERTICAL: ENiagaraStackEntryInlineDisplayMode = ENiagaraStackEntryInlineDisplayMode(
        2,
    );
    pub const GRAPH_HYBRID: ENiagaraStackEntryInlineDisplayMode = ENiagaraStackEntryInlineDisplayMode(
        3,
    );
    pub const NONE: ENiagaraStackEntryInlineDisplayMode = ENiagaraStackEntryInlineDisplayMode(
        4,
    );
}
#[repr(transparent)]
pub struct ENiagaraDataChanneSpawnModuleMode(pub u8);
impl ENiagaraDataChanneSpawnModuleMode {
    pub const CONDITIONAL_SPAWN: ENiagaraDataChanneSpawnModuleMode = ENiagaraDataChanneSpawnModuleMode(
        0,
    );
    pub const DIRECT_SPAWN: ENiagaraDataChanneSpawnModuleMode = ENiagaraDataChanneSpawnModuleMode(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraDataChanneWriteModuleMode(pub u8);
impl ENiagaraDataChanneWriteModuleMode {
    pub const APPEND_NEW_ELEMENT: ENiagaraDataChanneWriteModuleMode = ENiagaraDataChanneWriteModuleMode(
        0,
    );
    pub const WRITE_TO_EXISTING_ELEMENT: ENiagaraDataChanneWriteModuleMode = ENiagaraDataChanneWriteModuleMode(
        1,
    );
}
