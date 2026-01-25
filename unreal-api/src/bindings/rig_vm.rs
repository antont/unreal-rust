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
    pub u_rig_vm_set_parameter_value_vector2_d: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_set_parameter_value_vector: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_set_parameter_value_transform: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_set_parameter_value_string: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_set_parameter_value_quat: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_set_parameter_value_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_set_parameter_value_int: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_set_parameter_value_float: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_set_parameter_value_double: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_set_parameter_value_bool: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_get_statistics: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_get_rig_vm_function_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_get_parameter_value_vector2_d: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_get_parameter_value_vector: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_get_parameter_value_transform: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_get_parameter_value_string: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_get_parameter_value_quat: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_get_parameter_value_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_get_parameter_value_int: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_get_parameter_value_float: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_get_parameter_value_double: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_get_parameter_value_bool: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_execute: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_add_rig_vm_function: *mut crate::ffi::UFunctionOpague,
    pub u_data_asset_link_set_data_asset: *mut crate::ffi::UFunctionOpague,
    pub u_data_asset_link_get_data_asset: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_user_workflow_options_requires_dialog: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_user_workflow_options_report_warning: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_user_workflow_options_report_info: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_user_workflow_options_report_error: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_user_workflow_options_is_valid: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_supports_event: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_set_variable_from_string: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_set_frames_per_second: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_set_delta_time: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_set_absolute_time: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_set_absolute_and_delta_time: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_request_run_once_event: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_request_init: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_remove_run_once_event: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_is_init_required: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_get_vm: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_get_variable_type: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_get_variable_as_string: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_get_supported_events: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_get_script_accessible_variables: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_get_extended_execute_context: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_get_delta_time: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_get_current_frames_per_second: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_get_absolute_time: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_find_rig_vm_hosts: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_execute_event: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_execute: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_host_can_execute: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_project_settings_get_tag: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_rig_vm_set_parameter_value_vector2_d: std::ptr::null_mut(),
            u_rig_vm_set_parameter_value_vector: std::ptr::null_mut(),
            u_rig_vm_set_parameter_value_transform: std::ptr::null_mut(),
            u_rig_vm_set_parameter_value_string: std::ptr::null_mut(),
            u_rig_vm_set_parameter_value_quat: std::ptr::null_mut(),
            u_rig_vm_set_parameter_value_name: std::ptr::null_mut(),
            u_rig_vm_set_parameter_value_int: std::ptr::null_mut(),
            u_rig_vm_set_parameter_value_float: std::ptr::null_mut(),
            u_rig_vm_set_parameter_value_double: std::ptr::null_mut(),
            u_rig_vm_set_parameter_value_bool: std::ptr::null_mut(),
            u_rig_vm_get_statistics: std::ptr::null_mut(),
            u_rig_vm_get_rig_vm_function_name: std::ptr::null_mut(),
            u_rig_vm_get_parameter_value_vector2_d: std::ptr::null_mut(),
            u_rig_vm_get_parameter_value_vector: std::ptr::null_mut(),
            u_rig_vm_get_parameter_value_transform: std::ptr::null_mut(),
            u_rig_vm_get_parameter_value_string: std::ptr::null_mut(),
            u_rig_vm_get_parameter_value_quat: std::ptr::null_mut(),
            u_rig_vm_get_parameter_value_name: std::ptr::null_mut(),
            u_rig_vm_get_parameter_value_int: std::ptr::null_mut(),
            u_rig_vm_get_parameter_value_float: std::ptr::null_mut(),
            u_rig_vm_get_parameter_value_double: std::ptr::null_mut(),
            u_rig_vm_get_parameter_value_bool: std::ptr::null_mut(),
            u_rig_vm_execute: std::ptr::null_mut(),
            u_rig_vm_add_rig_vm_function: std::ptr::null_mut(),
            u_data_asset_link_set_data_asset: std::ptr::null_mut(),
            u_data_asset_link_get_data_asset: std::ptr::null_mut(),
            u_rig_vm_user_workflow_options_requires_dialog: std::ptr::null_mut(),
            u_rig_vm_user_workflow_options_report_warning: std::ptr::null_mut(),
            u_rig_vm_user_workflow_options_report_info: std::ptr::null_mut(),
            u_rig_vm_user_workflow_options_report_error: std::ptr::null_mut(),
            u_rig_vm_user_workflow_options_is_valid: std::ptr::null_mut(),
            u_rig_vm_host_supports_event: std::ptr::null_mut(),
            u_rig_vm_host_set_variable_from_string: std::ptr::null_mut(),
            u_rig_vm_host_set_frames_per_second: std::ptr::null_mut(),
            u_rig_vm_host_set_delta_time: std::ptr::null_mut(),
            u_rig_vm_host_set_absolute_time: std::ptr::null_mut(),
            u_rig_vm_host_set_absolute_and_delta_time: std::ptr::null_mut(),
            u_rig_vm_host_request_run_once_event: std::ptr::null_mut(),
            u_rig_vm_host_request_init: std::ptr::null_mut(),
            u_rig_vm_host_remove_run_once_event: std::ptr::null_mut(),
            u_rig_vm_host_is_init_required: std::ptr::null_mut(),
            u_rig_vm_host_get_vm: std::ptr::null_mut(),
            u_rig_vm_host_get_variable_type: std::ptr::null_mut(),
            u_rig_vm_host_get_variable_as_string: std::ptr::null_mut(),
            u_rig_vm_host_get_supported_events: std::ptr::null_mut(),
            u_rig_vm_host_get_script_accessible_variables: std::ptr::null_mut(),
            u_rig_vm_host_get_extended_execute_context: std::ptr::null_mut(),
            u_rig_vm_host_get_delta_time: std::ptr::null_mut(),
            u_rig_vm_host_get_current_frames_per_second: std::ptr::null_mut(),
            u_rig_vm_host_get_absolute_time: std::ptr::null_mut(),
            u_rig_vm_host_find_rig_vm_hosts: std::ptr::null_mut(),
            u_rig_vm_host_execute_event: std::ptr::null_mut(),
            u_rig_vm_host_execute: std::ptr::null_mut(),
            u_rig_vm_host_can_execute: std::ptr::null_mut(),
            u_rig_vm_project_settings_get_tag: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVM::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetParameterValueVector2D"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_set_parameter_value_vector2_d,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetParameterValueVector"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_set_parameter_value_vector,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetParameterValueTransform"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_set_parameter_value_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetParameterValueString"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_set_parameter_value_string,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetParameterValueQuat"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_set_parameter_value_quat,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetParameterValueName"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_set_parameter_value_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetParameterValueInt"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_set_parameter_value_int,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetParameterValueFloat"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_set_parameter_value_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetParameterValueDouble"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_set_parameter_value_double,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetParameterValueBool"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_set_parameter_value_bool,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStatistics"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_get_statistics,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRigVMFunctionName"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_get_rig_vm_function_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParameterValueVector2D"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_get_parameter_value_vector2_d,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParameterValueVector"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_get_parameter_value_vector,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParameterValueTransform"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_get_parameter_value_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParameterValueString"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_get_parameter_value_string,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParameterValueQuat"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_get_parameter_value_quat,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParameterValueName"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_get_parameter_value_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParameterValueInt"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_get_parameter_value_int,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParameterValueFloat"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_get_parameter_value_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParameterValueDouble"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_get_parameter_value_double,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParameterValueBool"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_get_parameter_value_bool,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Execute"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_execute,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddRigVMFunction"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_add_rig_vm_function,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UDataAssetLink::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDataAsset"),
            &raw mut __FUNCTION_PTRS.u_data_asset_link_set_data_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDataAsset"),
            &raw mut __FUNCTION_PTRS.u_data_asset_link_get_data_asset,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMUserWorkflowOptions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequiresDialog"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_user_workflow_options_requires_dialog,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReportWarning"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_user_workflow_options_report_warning,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReportInfo"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_user_workflow_options_report_info,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReportError"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_user_workflow_options_report_error,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValid"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_user_workflow_options_is_valid,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMHost::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SupportsEvent"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_supports_event,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVariableFromString"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_set_variable_from_string,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFramesPerSecond"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_set_frames_per_second,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDeltaTime"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_set_delta_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAbsoluteTime"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_set_absolute_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAbsoluteAndDeltaTime"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_set_absolute_and_delta_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestRunOnceEvent"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_request_run_once_event,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestInit"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_request_init,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveRunOnceEvent"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_remove_run_once_event,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsInitRequired"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_is_init_required,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVM"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_get_vm,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVariableType"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_get_variable_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVariableAsString"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_get_variable_as_string,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSupportedEvents"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_get_supported_events,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetScriptAccessibleVariables"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_get_script_accessible_variables,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetExtendedExecuteContext"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_get_extended_execute_context,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDeltaTime"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_get_delta_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentFramesPerSecond"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_get_current_frames_per_second,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAbsoluteTime"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_get_absolute_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindRigVMHosts"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_find_rig_vm_hosts,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExecuteEvent"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_execute_event,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Execute"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_execute,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanExecute"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_host_can_execute,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMProjectSettings::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTag"),
            &raw mut __FUNCTION_PTRS.u_rig_vm_project_settings_get_tag,
        );
    }
}
#[repr(C, align(8))]
pub struct FRigVMUserWorkflow {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub title: FString,
    pub tooltip: FString,
    pub ty: ERigVMUserWorkflowType,
    #[doc(hidden)]
    pub(crate) __padding_72: [u8; 24],
    pub perform_dynamic_delegate: FRigVMUserWorkflow_PerformDynamicDelegate,
    pub options_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
impl FRigVMUserWorkflow {}
#[repr(C, align(16))]
pub struct FRigVMExtendedExecuteContext {
    pub(crate) __padding_end: [u8; 768],
}
impl FRigVMExtendedExecuteContext {}
#[repr(C, align(4))]
pub struct FRigVMDebugDrawSettings {
    pub depth_priority: crate::bindings::engine::ESceneDepthPriorityGroup,
    pub lifetime: f32,
}
impl FRigVMDebugDrawSettings {}
#[repr(C, align(8))]
pub struct FRigVMFunction_DebugBase {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub debug_draw_settings: FRigVMDebugDrawSettings,
}
impl FRigVMFunction_DebugBase {}
#[repr(C, align(8))]
pub struct FRigVMStructMutable {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub execute_pin: FRigVMExecutePin,
}
impl FRigVMStructMutable {}
#[repr(C, align(8))]
pub struct FRigVMExecutePin {
    pub(crate) __padding_end: [u8; 8],
}
impl FRigVMExecutePin {}
#[repr(C, align(8))]
pub struct FRigVMFunction_DebugBaseMutable {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub debug_draw_settings: FRigVMDebugDrawSettings,
}
impl FRigVMFunction_DebugBaseMutable {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathMutableBase {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathMutableBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_SimBaseMutable {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_SimBaseMutable {}
#[repr(C, align(8))]
pub struct FRigVMFunction_DampFloat {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub target: f32,
    pub smoothing_time: f32,
    pub result: f32,
}
impl FRigVMFunction_DampFloat {}
#[repr(C, align(16))]
pub struct FRigVMExecuteContext {
    pub(crate) __padding_end: [u8; 288],
}
impl FRigVMExecuteContext {}
#[repr(C, align(8))]
pub struct FRigVMFunction_DampVector {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub target: crate::bindings::core_u_object::FVector,
    pub smoothing_time: f32,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_DampVector {}
#[repr(C, align(16))]
pub struct FRigVMFunction_DampQuaternion {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub target: crate::bindings::core_u_object::FQuat,
    pub smoothing_time: f32,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigVMFunction_DampQuaternion {}
#[repr(C, align(8))]
pub struct FRigVMFunction_SpringDampFloat {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: f32,
    pub value_velocity: f32,
    pub target: f32,
    pub smoothing_time: f32,
}
impl FRigVMFunction_SpringDampFloat {}
#[repr(C, align(8))]
pub struct FRigVMFunction_SpringDampVector {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FVector,
    pub value_velocity: crate::bindings::core_u_object::FVector,
    pub target: crate::bindings::core_u_object::FVector,
    pub smoothing_time: f32,
}
impl FRigVMFunction_SpringDampVector {}
#[repr(C, align(16))]
pub struct FRigVMFunction_SpringDampQuat {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub value_velocity: crate::bindings::core_u_object::FVector,
    pub target: crate::bindings::core_u_object::FQuat,
    pub smoothing_time: f32,
}
impl FRigVMFunction_SpringDampQuat {}
#[repr(C, align(4))]
pub struct FRigVMMemoryStatistics {
    pub register_count: i32,
    pub data_bytes: i32,
    pub total_bytes: i32,
}
impl FRigVMMemoryStatistics {}
#[repr(C, align(4))]
pub struct FRigVMByteCodeStatistics {
    pub instruction_count: i32,
    pub data_bytes: i32,
}
impl FRigVMByteCodeStatistics {}
#[repr(C, align(4))]
pub struct FRigVMStatistics {
    pub bytes_for_cdo: i32,
    pub bytes_per_instance: i32,
    pub literal_memory: FRigVMMemoryStatistics,
    pub work_memory: FRigVMMemoryStatistics,
    pub debug_memory: FRigVMMemoryStatistics,
    pub bytes_for_caching: i32,
    pub byte_code: FRigVMByteCodeStatistics,
}
impl FRigVMStatistics {}
#[repr(C, align(8))]
pub struct FRigVMParameter {
    pub(crate) __padding_end: [u8; 64],
}
impl FRigVMParameter {}
#[repr(C, align(8))]
pub struct FRigVMExternalVariableDef {
    pub(crate) __padding_end: [u8; 56],
}
impl FRigVMExternalVariableDef {}
#[repr(C, align(8))]
pub struct FRigVMExternalVariable {
    pub(crate) __padding_end: [u8; 64],
}
impl FRigVMExternalVariable {}
#[repr(C, align(8))]
pub struct FRigVMFunctionCompilationData {
    pub(crate) __padding_end: [u8; 912],
}
impl FRigVMFunctionCompilationData {}
#[repr(C, align(8))]
pub struct FRigVMGraphFunctionArgument {
    pub name: FName,
    pub display_name: FName,
    pub cpp_type: FName,
    pub cpp_type_object: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    pub b_is_array: bool,
    pub direction: ERigVMPinDirection,
    pub default_value: FString,
    pub b_is_const: bool,
    pub(crate) __padding_end: [u8; 87],
}
impl FRigVMGraphFunctionArgument {}
#[repr(C, align(8))]
pub struct FRigVMGraphFunctionIdentifier {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub host_object: crate::bindings::core_u_object::FSoftObjectPath,
}
impl FRigVMGraphFunctionIdentifier {}
#[repr(C, align(8))]
pub struct FRigVMGraphFunctionHeader {
    pub library_pointer: FRigVMGraphFunctionIdentifier,
    pub variant: FRigVMVariant,
    pub name: FName,
    pub node_title: FString,
    pub node_color: crate::bindings::core_u_object::FLinearColor,
    #[doc(hidden)]
    pub(crate) __padding_192: [u8; 16],
    pub description: FString,
    pub category: FString,
    pub keywords: FString,
    pub arguments: TArray<FRigVMGraphFunctionArgument>,
    pub(crate) __padding_end: [u8; 272],
}
impl FRigVMGraphFunctionHeader {}
#[repr(C, align(8))]
pub struct FRigVMNodeLayout {
    pub categories: TArray<FRigVMPinCategory>,
    pub pin_index_in_category: TMap<FString, i32>,
    pub display_names: TMap<FString, FString>,
}
impl FRigVMNodeLayout {}
#[repr(C, align(8))]
pub struct FRigVMPinCategory {
    pub path: FString,
    pub elements: TArray<FString>,
    pub b_expanded_by_default: bool,
}
impl FRigVMPinCategory {}
#[repr(C, align(8))]
pub struct FRigVMVariant {
    pub guid: crate::bindings::core_u_object::FGuid,
    pub tags: TArray<FRigVMTag>,
}
impl FRigVMVariant {}
#[repr(C, align(8))]
pub struct FRigVMTag {
    pub name: FName,
    pub label: FString,
    pub tool_tip: FText,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub b_show_in_user_interface: bool,
    pub b_marks_subject_as_invalid: bool,
}
impl FRigVMTag {}
#[repr(C, align(8))]
pub struct FRigVMGraphFunctionData {
    pub(crate) __padding_end: [u8; 1488],
}
impl FRigVMGraphFunctionData {}
#[repr(C, align(8))]
pub struct FRigVMInstructionSetExecuteState {
    pub(crate) __padding_end: [u8; 80],
}
impl FRigVMInstructionSetExecuteState {}
#[repr(C, align(8))]
pub struct FRigVMVariantRef {
    pub object_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub variant: FRigVMVariant,
}
impl FRigVMVariantRef {}
#[repr(C, align(8))]
pub struct FRigVMFunction_AnimEasingType {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub ty: ERigVMAnimEasingType,
}
impl FRigVMFunction_AnimEasingType {}
#[repr(C, align(8))]
pub struct FRigVMFunction_AnimEasing {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub ty: ERigVMAnimEasingType,
    pub source_minimum: f32,
    pub source_maximum: f32,
    pub target_minimum: f32,
    pub target_maximum: f32,
    pub result: f32,
}
impl FRigVMFunction_AnimEasing {}
#[repr(C, align(8))]
pub struct FRigVMFunction_AnimEvalRichCurve {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub curve: crate::bindings::engine::FRuntimeFloatCurve,
    pub source_minimum: f32,
    pub source_maximum: f32,
    pub target_minimum: f32,
    pub target_maximum: f32,
    pub result: f32,
}
impl FRigVMFunction_AnimEvalRichCurve {}
#[repr(C, align(8))]
pub struct FRigVMFunction_AnimRichCurve {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub curve: crate::bindings::engine::FRuntimeFloatCurve,
}
impl FRigVMFunction_AnimRichCurve {}
#[repr(C, align(8))]
pub struct FRigVMFunction_GetDeltaTime {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub result: f32,
}
impl FRigVMFunction_GetDeltaTime {}
#[repr(C, align(8))]
pub struct FRigVMFunction_GetWorldTime {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub year: f32,
    pub month: f32,
    pub day: f32,
    pub week_day: f32,
    pub hours: f32,
    pub minutes: f32,
    pub seconds: f32,
    pub overall_seconds: f32,
}
impl FRigVMFunction_GetWorldTime {}
#[repr(C, align(8))]
pub struct FRigVMFunction_FramesToSeconds {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub frames: f32,
    pub seconds: f32,
}
impl FRigVMFunction_FramesToSeconds {}
#[repr(C, align(8))]
pub struct FRigVMFunction_SecondsToFrames {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub seconds: f32,
    pub frames: f32,
}
impl FRigVMFunction_SecondsToFrames {}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugLineNoSpace {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigVMFunction_DebugLineNoSpace {}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugLineStripNoSpace {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub points: TArray<crate::bindings::core_u_object::FVector>,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigVMFunction_DebugLineStripNoSpace {}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugPoint {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub vector: crate::bindings::core_u_object::FVector,
    pub mode: ERigUnitDebugPointMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub scale: f32,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigVMFunction_DebugPoint {}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugPointMutable {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub vector: crate::bindings::core_u_object::FVector,
    pub mode: ERigUnitDebugPointMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub scale: f32,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigVMFunction_DebugPointMutable {}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugRectangle {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub scale: f32,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigVMFunction_DebugRectangle {}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugRectangleNoSpace {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub scale: f32,
    pub thickness: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigVMFunction_DebugRectangleNoSpace {}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugArc {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub radius: f32,
    pub minimum_degrees: f32,
    pub maximum_degrees: f32,
    pub thickness: f32,
    pub detail: i32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigVMFunction_DebugArc {}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugArcNoSpace {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub radius: f32,
    pub minimum_degrees: f32,
    pub maximum_degrees: f32,
    pub thickness: f32,
    pub detail: i32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigVMFunction_DebugArcNoSpace {}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugBoxNoSpace {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub box_: crate::bindings::core_u_object::FBox,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigVMFunction_DebugBoxNoSpace {}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugTransformMutableNoSpace {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub mode: ERigUnitDebugTransformMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigVMFunction_DebugTransformMutableNoSpace {}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugTransformArrayMutableNoSpace {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub parent_indices: TArray<i32>,
    pub mode: ERigUnitDebugTransformMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigVMFunction_DebugTransformArrayMutableNoSpace {}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualDebugVector {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FVector,
    pub b_enabled: bool,
    pub mode: ERigUnitVisualDebugPointMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
}
impl FRigVMFunction_VisualDebugVector {}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualDebugVectorNoSpace {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FVector,
    pub b_enabled: bool,
    pub mode: ERigUnitVisualDebugPointMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
}
impl FRigVMFunction_VisualDebugVectorNoSpace {}
#[repr(C, align(16))]
pub struct FRigVMFunction_VisualDebugQuat {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
}
impl FRigVMFunction_VisualDebugQuat {}
#[repr(C, align(16))]
pub struct FRigVMFunction_VisualDebugQuatNoSpace {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
}
impl FRigVMFunction_VisualDebugQuatNoSpace {}
#[repr(C, align(16))]
pub struct FRigVMFunction_VisualDebugTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
}
impl FRigVMFunction_VisualDebugTransform {}
#[repr(C, align(16))]
pub struct FRigVMFunction_VisualDebugTransformNoSpace {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
}
impl FRigVMFunction_VisualDebugTransformNoSpace {}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogBase {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub text: FString,
    pub category: FName,
}
impl FRigVMFunction_VisualLogBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogText {
    pub(crate) __padding_end: [u8; 56],
}
impl FRigVMFunction_VisualLogText {}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogObject {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub object_color: crate::bindings::core_u_object::FLinearColor,
}
impl FRigVMFunction_VisualLogObject {}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogLocation {
    #[doc(hidden)]
    pub(crate) __padding_72: [u8; 72],
    pub location: crate::bindings::core_u_object::FVector,
    pub radius: f32,
}
impl FRigVMFunction_VisualLogLocation {}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogWireframeOptional {
    #[doc(hidden)]
    pub(crate) __padding_72: [u8; 72],
    pub b_wireframe: bool,
}
impl FRigVMFunction_VisualLogWireframeOptional {}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogSphere {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub center: crate::bindings::core_u_object::FVector,
    pub radius: f32,
}
impl FRigVMFunction_VisualLogSphere {}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogCone {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub origin: crate::bindings::core_u_object::FVector,
    pub direction: crate::bindings::core_u_object::FVector,
    pub length: f32,
    pub angle: f32,
}
impl FRigVMFunction_VisualLogCone {}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogCylinder {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub start: crate::bindings::core_u_object::FVector,
    pub end: crate::bindings::core_u_object::FVector,
    pub radius: f32,
}
impl FRigVMFunction_VisualLogCylinder {}
#[repr(C, align(16))]
pub struct FRigVMFunction_VisualLogCapsule {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub base: crate::bindings::core_u_object::FVector,
    pub half_height: f32,
    pub radius: f32,
    pub rotation: crate::bindings::core_u_object::FQuat,
}
impl FRigVMFunction_VisualLogCapsule {}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogBox {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub box_: crate::bindings::core_u_object::FBox,
}
impl FRigVMFunction_VisualLogBox {}
#[repr(C, align(16))]
pub struct FRigVMFunction_VisualLogOrientedBox {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub box_: crate::bindings::core_u_object::FBox,
    pub transform: crate::bindings::core_u_object::FTransform,
}
impl FRigVMFunction_VisualLogOrientedBox {}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogArrow {
    #[doc(hidden)]
    pub(crate) __padding_72: [u8; 72],
    pub segment_start: crate::bindings::core_u_object::FVector,
    pub segment_end: crate::bindings::core_u_object::FVector,
    pub arrow_head_size: f32,
}
impl FRigVMFunction_VisualLogArrow {}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogCircle {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub center: crate::bindings::core_u_object::FVector,
    pub up_axis: crate::bindings::core_u_object::FVector,
    pub radius: f32,
    pub thickness: f32,
}
impl FRigVMFunction_VisualLogCircle {}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogSegment {
    #[doc(hidden)]
    pub(crate) __padding_72: [u8; 72],
    pub segment_start: crate::bindings::core_u_object::FVector,
    pub segment_end: crate::bindings::core_u_object::FVector,
    pub thickness: f32,
}
impl FRigVMFunction_VisualLogSegment {}
#[repr(C, align(8))]
pub struct FRigVMFunction_IsHostBeingDebugged {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub result: bool,
}
impl FRigVMFunction_IsHostBeingDebugged {}
#[repr(C, align(16))]
pub struct FRigVMFunction_ForLoopCount {
    #[doc(hidden)]
    pub(crate) __padding_28: [u8; 28],
    pub count: i32,
    pub index: i32,
    pub ratio: f32,
    pub completed: FRigVMExecuteContext,
}
impl FRigVMFunction_ForLoopCount {}
#[repr(C, align(16))]
pub struct FRigVMFunction_Sequence {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub execute_context: FRigVMExecuteContext,
    pub a: FRigVMExecuteContext,
    pub b: FRigVMExecuteContext,
}
impl FRigVMFunction_Sequence {}
#[repr(C, align(8))]
pub struct FRigVMFunction_UserDefinedEvent {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub execute_pin: FRigVMExecutePin,
    pub event_name: FName,
}
impl FRigVMFunction_UserDefinedEvent {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolConstant {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: bool,
}
impl FRigVMFunction_MathBoolConstant {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolUnaryOp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: bool,
    pub result: bool,
}
impl FRigVMFunction_MathBoolUnaryOp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolBinaryOp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: bool,
    pub b: bool,
    pub result: bool,
}
impl FRigVMFunction_MathBoolBinaryOp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolBinaryAggregateOp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: bool,
    pub b: bool,
    pub result: bool,
}
impl FRigVMFunction_MathBoolBinaryAggregateOp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolMake {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: bool,
}
impl FRigVMFunction_MathBoolMake {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolConstTrue {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathBoolConstTrue {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolConstFalse {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathBoolConstFalse {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolNot {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathBoolNot {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolAnd {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathBoolAnd {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolNand {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathBoolNand {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolNand2 {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathBoolNand2 {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolOr {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathBoolOr {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolEquals {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: bool,
    pub b: bool,
    pub result: bool,
}
impl FRigVMFunction_MathBoolEquals {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolNotEquals {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: bool,
    pub b: bool,
    pub result: bool,
}
impl FRigVMFunction_MathBoolNotEquals {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolToggled {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: bool,
    pub toggled: bool,
}
impl FRigVMFunction_MathBoolToggled {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolFlipFlop {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub start_value: bool,
    pub duration: f32,
    pub result: bool,
}
impl FRigVMFunction_MathBoolFlipFlop {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolOnce {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub duration: f32,
    pub result: bool,
    pub(crate) __padding_end: [u8; 11],
}
impl FRigVMFunction_MathBoolOnce {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolToFloat {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: bool,
    pub result: f32,
}
impl FRigVMFunction_MathBoolToFloat {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolToInteger {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: bool,
    pub result: i32,
}
impl FRigVMFunction_MathBoolToInteger {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxFromArray {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub array: TArray<crate::bindings::core_u_object::FVector>,
    pub box_: crate::bindings::core_u_object::FBox,
    pub minimum: crate::bindings::core_u_object::FVector,
    pub maximum: crate::bindings::core_u_object::FVector,
    pub center: crate::bindings::core_u_object::FVector,
    pub size: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathBoxFromArray {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxIsValid {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub box_: crate::bindings::core_u_object::FBox,
    pub valid: bool,
}
impl FRigVMFunction_MathBoxIsValid {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxGetCenter {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub box_: crate::bindings::core_u_object::FBox,
    pub center: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathBoxGetCenter {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxGetSize {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub box_: crate::bindings::core_u_object::FBox,
    pub size: crate::bindings::core_u_object::FVector,
    pub extent: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathBoxGetSize {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxShift {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub box_: crate::bindings::core_u_object::FBox,
    pub amount: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FBox,
}
impl FRigVMFunction_MathBoxShift {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxMoveTo {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub box_: crate::bindings::core_u_object::FBox,
    pub center: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FBox,
}
impl FRigVMFunction_MathBoxMoveTo {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxExpand {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub box_: crate::bindings::core_u_object::FBox,
    pub amount: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FBox,
}
impl FRigVMFunction_MathBoxExpand {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathBoxTransform {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub box_: crate::bindings::core_u_object::FBox,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FBox,
}
impl FRigVMFunction_MathBoxTransform {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxGetDistance {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub box_: crate::bindings::core_u_object::FBox,
    pub position: crate::bindings::core_u_object::FVector,
    pub square: bool,
    pub valid: bool,
    pub distance: f32,
}
impl FRigVMFunction_MathBoxGetDistance {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxIsInside {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub box_: crate::bindings::core_u_object::FBox,
    pub position: crate::bindings::core_u_object::FVector,
    pub result: bool,
}
impl FRigVMFunction_MathBoxIsInside {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxGetVolume {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub box_: crate::bindings::core_u_object::FBox,
    pub volume: f32,
}
impl FRigVMFunction_MathBoxGetVolume {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorBinaryOp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: crate::bindings::core_u_object::FLinearColor,
    pub b: crate::bindings::core_u_object::FLinearColor,
    pub result: crate::bindings::core_u_object::FLinearColor,
}
impl FRigVMFunction_MathColorBinaryOp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorBinaryAggregateOp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: crate::bindings::core_u_object::FLinearColor,
    pub b: crate::bindings::core_u_object::FLinearColor,
    pub result: crate::bindings::core_u_object::FLinearColor,
}
impl FRigVMFunction_MathColorBinaryAggregateOp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorMake {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub result: crate::bindings::core_u_object::FLinearColor,
}
impl FRigVMFunction_MathColorMake {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorFromFloat {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub result: crate::bindings::core_u_object::FLinearColor,
}
impl FRigVMFunction_MathColorFromFloat {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorFromDouble {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f64,
    pub result: crate::bindings::core_u_object::FLinearColor,
}
impl FRigVMFunction_MathColorFromDouble {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorAdd {
    pub(crate) __padding_end: [u8; 56],
}
impl FRigVMFunction_MathColorAdd {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorSub {
    pub(crate) __padding_end: [u8; 56],
}
impl FRigVMFunction_MathColorSub {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorMul {
    pub(crate) __padding_end: [u8; 56],
}
impl FRigVMFunction_MathColorMul {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorLerp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: crate::bindings::core_u_object::FLinearColor,
    pub b: crate::bindings::core_u_object::FLinearColor,
    pub t: f32,
    pub result: crate::bindings::core_u_object::FLinearColor,
}
impl FRigVMFunction_MathColorLerp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleConstant {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f64,
}
impl FRigVMFunction_MathDoubleConstant {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleUnaryOp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f64,
    pub result: f64,
}
impl FRigVMFunction_MathDoubleUnaryOp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleBinaryOp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f64,
    pub b: f64,
    pub result: f64,
}
impl FRigVMFunction_MathDoubleBinaryOp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleBinaryAggregateOp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f64,
    pub b: f64,
    pub result: f64,
}
impl FRigVMFunction_MathDoubleBinaryAggregateOp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleMake {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f64,
}
impl FRigVMFunction_MathDoubleMake {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleConstPi {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathDoubleConstPi {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleConstHalfPi {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathDoubleConstHalfPi {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleConstTwoPi {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathDoubleConstTwoPi {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleConstE {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathDoubleConstE {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleAdd {
    pub(crate) __padding_end: [u8; 32],
}
impl FRigVMFunction_MathDoubleAdd {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleSub {
    pub(crate) __padding_end: [u8; 32],
}
impl FRigVMFunction_MathDoubleSub {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleMul {
    pub(crate) __padding_end: [u8; 32],
}
impl FRigVMFunction_MathDoubleMul {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleDiv {
    pub(crate) __padding_end: [u8; 32],
}
impl FRigVMFunction_MathDoubleDiv {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleMod {
    pub(crate) __padding_end: [u8; 32],
}
impl FRigVMFunction_MathDoubleMod {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleMin {
    pub(crate) __padding_end: [u8; 32],
}
impl FRigVMFunction_MathDoubleMin {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleMax {
    pub(crate) __padding_end: [u8; 32],
}
impl FRigVMFunction_MathDoubleMax {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoublePow {
    pub(crate) __padding_end: [u8; 32],
}
impl FRigVMFunction_MathDoublePow {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleSqrt {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathDoubleSqrt {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleNegate {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathDoubleNegate {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleAbs {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathDoubleAbs {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleFloor {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f64,
    pub result: f64,
    pub int: i32,
}
impl FRigVMFunction_MathDoubleFloor {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleCeil {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f64,
    pub result: f64,
    pub int: i32,
}
impl FRigVMFunction_MathDoubleCeil {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleRound {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f64,
    pub result: f64,
    pub int: i32,
}
impl FRigVMFunction_MathDoubleRound {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleToInt {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f64,
    pub result: i32,
}
impl FRigVMFunction_MathDoubleToInt {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleToFloat {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f64,
    pub result: f32,
}
impl FRigVMFunction_MathDoubleToFloat {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleSign {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathDoubleSign {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleClamp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f64,
    pub minimum: f64,
    pub maximum: f64,
    pub result: f64,
}
impl FRigVMFunction_MathDoubleClamp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleLerp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f64,
    pub b: f64,
    pub t: f64,
    pub result: f64,
}
impl FRigVMFunction_MathDoubleLerp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleRemap {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f64,
    pub source_minimum: f64,
    pub source_maximum: f64,
    pub target_minimum: f64,
    pub target_maximum: f64,
    pub b_clamp: bool,
    pub result: f64,
}
impl FRigVMFunction_MathDoubleRemap {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleEquals {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f64,
    pub b: f64,
    pub result: bool,
}
impl FRigVMFunction_MathDoubleEquals {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleNotEquals {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f64,
    pub b: f64,
    pub result: bool,
}
impl FRigVMFunction_MathDoubleNotEquals {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleGreater {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f64,
    pub b: f64,
    pub result: bool,
}
impl FRigVMFunction_MathDoubleGreater {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleLess {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f64,
    pub b: f64,
    pub result: bool,
}
impl FRigVMFunction_MathDoubleLess {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleGreaterEqual {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f64,
    pub b: f64,
    pub result: bool,
}
impl FRigVMFunction_MathDoubleGreaterEqual {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleLessEqual {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f64,
    pub b: f64,
    pub result: bool,
}
impl FRigVMFunction_MathDoubleLessEqual {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleIsNearlyZero {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f64,
    pub tolerance: f64,
    pub result: bool,
}
impl FRigVMFunction_MathDoubleIsNearlyZero {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleIsNearlyEqual {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f64,
    pub b: f64,
    pub tolerance: f64,
    pub result: bool,
}
impl FRigVMFunction_MathDoubleIsNearlyEqual {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleDeg {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathDoubleDeg {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleRad {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathDoubleRad {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleSin {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathDoubleSin {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleCos {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathDoubleCos {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleTan {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathDoubleTan {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleAsin {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathDoubleAsin {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleAcos {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathDoubleAcos {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleAtan {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathDoubleAtan {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleAtan2 {
    pub(crate) __padding_end: [u8; 32],
}
impl FRigVMFunction_MathDoubleAtan2 {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleLawOfCosine {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub alpha_angle: f64,
    pub beta_angle: f64,
    pub gamma_angle: f64,
    pub b_valid: bool,
}
impl FRigVMFunction_MathDoubleLawOfCosine {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleExponential {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathDoubleExponential {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleArraySum {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub array: TArray<f64>,
    pub sum: f64,
}
impl FRigVMFunction_MathDoubleArraySum {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleArrayAverage {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub array: TArray<f64>,
    pub average: f64,
}
impl FRigVMFunction_MathDoubleArrayAverage {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatConstant {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
}
impl FRigVMFunction_MathFloatConstant {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatUnaryOp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub result: f32,
}
impl FRigVMFunction_MathFloatUnaryOp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatBinaryOp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f32,
    pub b: f32,
    pub result: f32,
}
impl FRigVMFunction_MathFloatBinaryOp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatBinaryAggregateOp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f32,
    pub b: f32,
    pub result: f32,
}
impl FRigVMFunction_MathFloatBinaryAggregateOp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatMake {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
}
impl FRigVMFunction_MathFloatMake {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatConstPi {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathFloatConstPi {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatConstHalfPi {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathFloatConstHalfPi {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatConstTwoPi {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathFloatConstTwoPi {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatConstE {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathFloatConstE {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatAdd {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathFloatAdd {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatSub {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathFloatSub {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatMul {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathFloatMul {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatDiv {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathFloatDiv {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatMod {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathFloatMod {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatMin {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathFloatMin {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatMax {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathFloatMax {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatPow {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathFloatPow {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatSqrt {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathFloatSqrt {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatNegate {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathFloatNegate {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatAbs {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathFloatAbs {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatFloor {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub result: f32,
    pub int: i32,
}
impl FRigVMFunction_MathFloatFloor {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatCeil {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub result: f32,
    pub int: i32,
}
impl FRigVMFunction_MathFloatCeil {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatRound {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub result: f32,
    pub int: i32,
}
impl FRigVMFunction_MathFloatRound {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatToInt {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub result: i32,
}
impl FRigVMFunction_MathFloatToInt {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatToDouble {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub result: f64,
}
impl FRigVMFunction_MathFloatToDouble {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatSign {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathFloatSign {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatClamp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub minimum: f32,
    pub maximum: f32,
    pub result: f32,
}
impl FRigVMFunction_MathFloatClamp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatLerp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f32,
    pub b: f32,
    pub t: f32,
    pub result: f32,
}
impl FRigVMFunction_MathFloatLerp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatRemap {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub source_minimum: f32,
    pub source_maximum: f32,
    pub target_minimum: f32,
    pub target_maximum: f32,
    pub b_clamp: bool,
    pub result: f32,
}
impl FRigVMFunction_MathFloatRemap {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatEquals {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f32,
    pub b: f32,
    pub result: bool,
}
impl FRigVMFunction_MathFloatEquals {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatNotEquals {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f32,
    pub b: f32,
    pub result: bool,
}
impl FRigVMFunction_MathFloatNotEquals {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatGreater {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f32,
    pub b: f32,
    pub result: bool,
}
impl FRigVMFunction_MathFloatGreater {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatLess {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f32,
    pub b: f32,
    pub result: bool,
}
impl FRigVMFunction_MathFloatLess {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatGreaterEqual {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f32,
    pub b: f32,
    pub result: bool,
}
impl FRigVMFunction_MathFloatGreaterEqual {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatLessEqual {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f32,
    pub b: f32,
    pub result: bool,
}
impl FRigVMFunction_MathFloatLessEqual {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatIsNearlyZero {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub tolerance: f32,
    pub result: bool,
}
impl FRigVMFunction_MathFloatIsNearlyZero {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatIsNearlyEqual {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f32,
    pub b: f32,
    pub tolerance: f32,
    pub result: bool,
}
impl FRigVMFunction_MathFloatIsNearlyEqual {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatSelectBool {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub condition: bool,
    pub if_true: f32,
    pub if_false: f32,
    pub result: f32,
}
impl FRigVMFunction_MathFloatSelectBool {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatDeg {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathFloatDeg {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatRad {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathFloatRad {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatSin {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathFloatSin {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatCos {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathFloatCos {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatTan {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathFloatTan {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatAsin {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathFloatAsin {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatAcos {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathFloatAcos {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatAtan {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathFloatAtan {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatAtan2 {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathFloatAtan2 {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatLawOfCosine {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub alpha_angle: f32,
    pub beta_angle: f32,
    pub gamma_angle: f32,
    pub b_valid: bool,
}
impl FRigVMFunction_MathFloatLawOfCosine {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatExponential {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathFloatExponential {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatArraySum {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub array: TArray<f32>,
    pub sum: f32,
}
impl FRigVMFunction_MathFloatArraySum {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatArrayAverage {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub array: TArray<f32>,
    pub average: f32,
}
impl FRigVMFunction_MathFloatArrayAverage {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntUnaryOp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: i32,
    pub result: i32,
}
impl FRigVMFunction_MathIntUnaryOp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntBinaryOp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: i32,
    pub b: i32,
    pub result: i32,
}
impl FRigVMFunction_MathIntBinaryOp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntBinaryAggregateOp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: i32,
    pub b: i32,
    pub result: i32,
}
impl FRigVMFunction_MathIntBinaryAggregateOp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntMake {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: i32,
}
impl FRigVMFunction_MathIntMake {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntAdd {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathIntAdd {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntSub {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathIntSub {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntMul {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathIntMul {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntDiv {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathIntDiv {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntMod {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathIntMod {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntMin {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathIntMin {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntMax {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathIntMax {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntPow {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_MathIntPow {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntNegate {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathIntNegate {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntAbs {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathIntAbs {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntToFloat {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: i32,
    pub result: f32,
}
impl FRigVMFunction_MathIntToFloat {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntToDouble {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: i32,
    pub result: f64,
}
impl FRigVMFunction_MathIntToDouble {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntSign {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathIntSign {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntClamp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: i32,
    pub minimum: i32,
    pub maximum: i32,
    pub result: i32,
}
impl FRigVMFunction_MathIntClamp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntEquals {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: i32,
    pub b: i32,
    pub result: bool,
}
impl FRigVMFunction_MathIntEquals {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntNotEquals {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: i32,
    pub b: i32,
    pub result: bool,
}
impl FRigVMFunction_MathIntNotEquals {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntGreater {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: i32,
    pub b: i32,
    pub result: bool,
}
impl FRigVMFunction_MathIntGreater {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntLess {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: i32,
    pub b: i32,
    pub result: bool,
}
impl FRigVMFunction_MathIntLess {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntGreaterEqual {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: i32,
    pub b: i32,
    pub result: bool,
}
impl FRigVMFunction_MathIntGreaterEqual {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntLessEqual {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: i32,
    pub b: i32,
    pub result: bool,
}
impl FRigVMFunction_MathIntLessEqual {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntArraySum {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub array: TArray<i32>,
    pub sum: i32,
}
impl FRigVMFunction_MathIntArraySum {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntArrayAverage {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub array: TArray<i32>,
    pub average: i32,
}
impl FRigVMFunction_MathIntArrayAverage {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntToString {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub number: i32,
    pub padded_size: i32,
    pub result: FString,
}
impl FRigVMFunction_MathIntToString {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntToName {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub number: i32,
    pub padded_size: i32,
    pub result: FName,
}
impl FRigVMFunction_MathIntToName {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixUnaryOp {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FMatrix,
    pub result: crate::bindings::core_u_object::FMatrix,
}
impl FRigVMFunction_MathMatrixUnaryOp {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixBinaryOp {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub a: crate::bindings::core_u_object::FMatrix,
    pub b: crate::bindings::core_u_object::FMatrix,
    pub result: crate::bindings::core_u_object::FMatrix,
}
impl FRigVMFunction_MathMatrixBinaryOp {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixBinaryAggregateOp {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub a: crate::bindings::core_u_object::FMatrix,
    pub b: crate::bindings::core_u_object::FMatrix,
    pub result: crate::bindings::core_u_object::FMatrix,
}
impl FRigVMFunction_MathMatrixBinaryAggregateOp {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixToTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FMatrix,
    pub result: crate::bindings::core_u_object::FTransform,
}
impl FRigVMFunction_MathMatrixToTransform {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixFromTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FMatrix,
}
impl FRigVMFunction_MathMatrixFromTransform {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixFromTransformV2 {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FMatrix,
}
impl FRigVMFunction_MathMatrixFromTransformV2 {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixToVectors {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FMatrix,
    pub origin: crate::bindings::core_u_object::FVector,
    pub x: crate::bindings::core_u_object::FVector,
    pub y: crate::bindings::core_u_object::FVector,
    pub z: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathMatrixToVectors {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixFromVectors {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub origin: crate::bindings::core_u_object::FVector,
    pub x: crate::bindings::core_u_object::FVector,
    pub y: crate::bindings::core_u_object::FVector,
    pub z: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FMatrix,
}
impl FRigVMFunction_MathMatrixFromVectors {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixMul {
    pub(crate) __padding_end: [u8; 400],
}
impl FRigVMFunction_MathMatrixMul {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixInverse {
    pub(crate) __padding_end: [u8; 272],
}
impl FRigVMFunction_MathMatrixInverse {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionUnaryOp {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigVMFunction_MathQuaternionUnaryOp {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionBinaryOp {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub a: crate::bindings::core_u_object::FQuat,
    pub b: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigVMFunction_MathQuaternionBinaryOp {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionBinaryAggregateOp {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub a: crate::bindings::core_u_object::FQuat,
    pub b: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigVMFunction_MathQuaternionBinaryAggregateOp {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionMake {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigVMFunction_MathQuaternionMake {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionFromAxisAndAngle {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub axis: crate::bindings::core_u_object::FVector,
    pub angle: f32,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigVMFunction_MathQuaternionFromAxisAndAngle {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionFromEuler {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub euler: crate::bindings::core_u_object::FVector,
    pub rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigVMFunction_MathQuaternionFromEuler {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionFromRotator {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub rotator: crate::bindings::core_u_object::FRotator,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigVMFunction_MathQuaternionFromRotator {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionFromRotatorV2 {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FRotator,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigVMFunction_MathQuaternionFromRotatorV2 {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionFromTwoVectors {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigVMFunction_MathQuaternionFromTwoVectors {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionToAxisAndAngle {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub axis: crate::bindings::core_u_object::FVector,
    pub angle: f32,
}
impl FRigVMFunction_MathQuaternionToAxisAndAngle {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionToVectors {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub forward: crate::bindings::core_u_object::FVector,
    pub right: crate::bindings::core_u_object::FVector,
    pub up: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathQuaternionToVectors {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionScale {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub scale: f32,
}
impl FRigVMFunction_MathQuaternionScale {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionScaleV2 {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub factor: f32,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigVMFunction_MathQuaternionScaleV2 {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionToEuler {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathQuaternionToEuler {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionToRotator {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FRotator,
}
impl FRigVMFunction_MathQuaternionToRotator {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionMul {
    pub(crate) __padding_end: [u8; 112],
}
impl FRigVMFunction_MathQuaternionMul {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionInverse {
    pub(crate) __padding_end: [u8; 80],
}
impl FRigVMFunction_MathQuaternionInverse {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionSlerp {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub a: crate::bindings::core_u_object::FQuat,
    pub b: crate::bindings::core_u_object::FQuat,
    pub t: f32,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigVMFunction_MathQuaternionSlerp {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionEquals {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub a: crate::bindings::core_u_object::FQuat,
    pub b: crate::bindings::core_u_object::FQuat,
    pub result: bool,
}
impl FRigVMFunction_MathQuaternionEquals {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionNotEquals {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub a: crate::bindings::core_u_object::FQuat,
    pub b: crate::bindings::core_u_object::FQuat,
    pub result: bool,
}
impl FRigVMFunction_MathQuaternionNotEquals {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionSelectBool {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub condition: bool,
    pub if_true: crate::bindings::core_u_object::FQuat,
    pub if_false: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigVMFunction_MathQuaternionSelectBool {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionDot {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub a: crate::bindings::core_u_object::FQuat,
    pub b: crate::bindings::core_u_object::FQuat,
    pub result: f32,
}
impl FRigVMFunction_MathQuaternionDot {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionUnit {
    pub(crate) __padding_end: [u8; 80],
}
impl FRigVMFunction_MathQuaternionUnit {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionRotateVector {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub transform: crate::bindings::core_u_object::FQuat,
    pub vector: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathQuaternionRotateVector {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionGetAxis {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub quaternion: crate::bindings::core_u_object::FQuat,
    pub axis: crate::bindings::core_u_object::EAxis,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathQuaternionGetAxis {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionSwingTwist {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub input: crate::bindings::core_u_object::FQuat,
    pub twist_axis: crate::bindings::core_u_object::FVector,
    pub swing: crate::bindings::core_u_object::FQuat,
    pub twist: crate::bindings::core_u_object::FQuat,
}
impl FRigVMFunction_MathQuaternionSwingTwist {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathQuaternionRotationOrder {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
}
impl FRigVMFunction_MathQuaternionRotationOrder {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionMakeRelative {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub global: crate::bindings::core_u_object::FQuat,
    pub parent: crate::bindings::core_u_object::FQuat,
    pub local: crate::bindings::core_u_object::FQuat,
}
impl FRigVMFunction_MathQuaternionMakeRelative {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionMakeAbsolute {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub local: crate::bindings::core_u_object::FQuat,
    pub parent: crate::bindings::core_u_object::FQuat,
    pub global: crate::bindings::core_u_object::FQuat,
}
impl FRigVMFunction_MathQuaternionMakeAbsolute {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionMirrorTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub mirror_axis: crate::bindings::core_u_object::EAxis,
    pub axis_to_flip: crate::bindings::core_u_object::EAxis,
    pub central_transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigVMFunction_MathQuaternionMirrorTransform {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathRayIntersectRay {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: crate::bindings::core_u_object::FRay,
    pub b: crate::bindings::core_u_object::FRay,
    pub result: crate::bindings::core_u_object::FVector,
    pub distance: f32,
    pub ratio_a: f32,
    pub ratio_b: f32,
}
impl FRigVMFunction_MathRayIntersectRay {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathRayIntersectPlane {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub ray: crate::bindings::core_u_object::FRay,
    pub plane_point: crate::bindings::core_u_object::FVector,
    pub plane_normal: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
    pub distance: f32,
    pub ratio: f32,
}
impl FRigVMFunction_MathRayIntersectPlane {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathRayGetAt {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub ray: crate::bindings::core_u_object::FRay,
    pub ratio: f32,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathRayGetAt {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRayTransform {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub ray: crate::bindings::core_u_object::FRay,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FRay,
}
impl FRigVMFunction_MathRayTransform {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateQuatBase {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub input: crate::bindings::core_u_object::FQuat,
    pub distance_function: ERBFQuatDistanceType,
    pub smoothing_function: ERBFKernelType,
    pub smoothing_angle: f32,
    pub b_normalize_output: bool,
    pub twist_axis: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 136],
}
impl FRigVMFunction_MathRBFInterpolateQuatBase {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateVectorBase {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub input: crate::bindings::core_u_object::FVector,
    pub distance_function: ERBFVectorDistanceType,
    pub smoothing_function: ERBFKernelType,
    pub smoothing_radius: f32,
    pub b_normalize_output: bool,
    pub(crate) __padding_end: [u8; 135],
}
impl FRigVMFunction_MathRBFInterpolateVectorBase {}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateQuatFloat_Target {
    pub target: crate::bindings::core_u_object::FQuat,
    pub value: f32,
}
impl FMathRBFInterpolateQuatFloat_Target {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateQuatFloat {
    #[doc(hidden)]
    pub(crate) __padding_224: [u8; 224],
    pub targets: TArray<FMathRBFInterpolateQuatFloat_Target>,
    pub output: f32,
}
impl FRigVMFunction_MathRBFInterpolateQuatFloat {}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateQuatVector_Target {
    pub target: crate::bindings::core_u_object::FQuat,
    pub value: crate::bindings::core_u_object::FVector,
}
impl FMathRBFInterpolateQuatVector_Target {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateQuatVector {
    #[doc(hidden)]
    pub(crate) __padding_224: [u8; 224],
    pub targets: TArray<FMathRBFInterpolateQuatVector_Target>,
    pub output: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathRBFInterpolateQuatVector {}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateQuatColor_Target {
    pub target: crate::bindings::core_u_object::FQuat,
    pub value: crate::bindings::core_u_object::FLinearColor,
}
impl FMathRBFInterpolateQuatColor_Target {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateQuatColor {
    #[doc(hidden)]
    pub(crate) __padding_224: [u8; 224],
    pub targets: TArray<FMathRBFInterpolateQuatColor_Target>,
    pub output: crate::bindings::core_u_object::FLinearColor,
}
impl FRigVMFunction_MathRBFInterpolateQuatColor {}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateQuatQuat_Target {
    pub target: crate::bindings::core_u_object::FQuat,
    pub value: crate::bindings::core_u_object::FQuat,
}
impl FMathRBFInterpolateQuatQuat_Target {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateQuatQuat {
    #[doc(hidden)]
    pub(crate) __padding_224: [u8; 224],
    pub targets: TArray<FMathRBFInterpolateQuatQuat_Target>,
    pub output: crate::bindings::core_u_object::FQuat,
}
impl FRigVMFunction_MathRBFInterpolateQuatQuat {}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateQuatXform_Target {
    pub target: crate::bindings::core_u_object::FQuat,
    pub value: crate::bindings::core_u_object::FTransform,
}
impl FMathRBFInterpolateQuatXform_Target {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateQuatXform {
    #[doc(hidden)]
    pub(crate) __padding_224: [u8; 224],
    pub targets: TArray<FMathRBFInterpolateQuatXform_Target>,
    pub output: crate::bindings::core_u_object::FTransform,
}
impl FRigVMFunction_MathRBFInterpolateQuatXform {}
#[repr(C, align(8))]
pub struct FMathRBFInterpolateVectorFloat_Target {
    pub target: crate::bindings::core_u_object::FVector,
    pub value: f32,
}
impl FMathRBFInterpolateVectorFloat_Target {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateVectorFloat {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub targets: TArray<FMathRBFInterpolateVectorFloat_Target>,
    pub output: f32,
}
impl FRigVMFunction_MathRBFInterpolateVectorFloat {}
#[repr(C, align(8))]
pub struct FMathRBFInterpolateVectorVector_Target {
    pub target: crate::bindings::core_u_object::FVector,
    pub value: crate::bindings::core_u_object::FVector,
}
impl FMathRBFInterpolateVectorVector_Target {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateVectorVector {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub targets: TArray<FMathRBFInterpolateVectorVector_Target>,
    pub output: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathRBFInterpolateVectorVector {}
#[repr(C, align(8))]
pub struct FMathRBFInterpolateVectorColor_Target {
    pub target: crate::bindings::core_u_object::FVector,
    pub value: crate::bindings::core_u_object::FLinearColor,
}
impl FMathRBFInterpolateVectorColor_Target {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateVectorColor {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub targets: TArray<FMathRBFInterpolateVectorColor_Target>,
    pub output: crate::bindings::core_u_object::FLinearColor,
}
impl FRigVMFunction_MathRBFInterpolateVectorColor {}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateVectorQuat_Target {
    pub target: crate::bindings::core_u_object::FVector,
    pub value: crate::bindings::core_u_object::FQuat,
}
impl FMathRBFInterpolateVectorQuat_Target {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateVectorQuat {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub targets: TArray<FMathRBFInterpolateVectorQuat_Target>,
    pub output: crate::bindings::core_u_object::FQuat,
}
impl FRigVMFunction_MathRBFInterpolateVectorQuat {}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateVectorXform_Target {
    pub target: crate::bindings::core_u_object::FVector,
    pub value: crate::bindings::core_u_object::FTransform,
}
impl FMathRBFInterpolateVectorXform_Target {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateVectorXform {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub targets: TArray<FMathRBFInterpolateVectorXform_Target>,
    pub output: crate::bindings::core_u_object::FTransform,
}
impl FRigVMFunction_MathRBFInterpolateVectorXform {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathTransformMutableBase {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_MathTransformMutableBase {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformUnaryOp {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
}
impl FRigVMFunction_MathTransformUnaryOp {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformBinaryOp {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub a: crate::bindings::core_u_object::FTransform,
    pub b: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
}
impl FRigVMFunction_MathTransformBinaryOp {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformBinaryAggregateOp {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub a: crate::bindings::core_u_object::FTransform,
    pub b: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
}
impl FRigVMFunction_MathTransformBinaryAggregateOp {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformMake {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub translation: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FQuat,
    pub scale: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FTransform,
}
impl FRigVMFunction_MathTransformMake {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformFromEulerTransform {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub euler_transform: crate::bindings::animation_core::FEulerTransform,
    pub result: crate::bindings::core_u_object::FTransform,
}
impl FRigVMFunction_MathTransformFromEulerTransform {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformFromEulerTransformV2 {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::animation_core::FEulerTransform,
    pub result: crate::bindings::core_u_object::FTransform,
}
impl FRigVMFunction_MathTransformFromEulerTransformV2 {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformToEulerTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::animation_core::FEulerTransform,
}
impl FRigVMFunction_MathTransformToEulerTransform {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformToVectors {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FTransform,
    pub forward: crate::bindings::core_u_object::FVector,
    pub right: crate::bindings::core_u_object::FVector,
    pub up: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathTransformToVectors {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformMul {
    pub(crate) __padding_end: [u8; 304],
}
impl FRigVMFunction_MathTransformMul {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformMakeRelative {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub global: crate::bindings::core_u_object::FTransform,
    pub parent: crate::bindings::core_u_object::FTransform,
    pub local: crate::bindings::core_u_object::FTransform,
}
impl FRigVMFunction_MathTransformMakeRelative {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformMakeAbsolute {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub local: crate::bindings::core_u_object::FTransform,
    pub parent: crate::bindings::core_u_object::FTransform,
    pub global: crate::bindings::core_u_object::FTransform,
}
impl FRigVMFunction_MathTransformMakeAbsolute {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformAccumulateArray {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub target_space: ERigVMTransformSpace,
    pub root: crate::bindings::core_u_object::FTransform,
    pub parent_indices: TArray<i32>,
}
impl FRigVMFunction_MathTransformAccumulateArray {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformInverse {
    pub(crate) __padding_end: [u8; 208],
}
impl FRigVMFunction_MathTransformInverse {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformLerp {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub a: crate::bindings::core_u_object::FTransform,
    pub b: crate::bindings::core_u_object::FTransform,
    pub t: f32,
    pub result: crate::bindings::core_u_object::FTransform,
}
impl FRigVMFunction_MathTransformLerp {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformSelectBool {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub condition: bool,
    pub if_true: crate::bindings::core_u_object::FTransform,
    pub if_false: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
}
impl FRigVMFunction_MathTransformSelectBool {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformRotateVector {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub vector: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathTransformRotateVector {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformTransformVector {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub location: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathTransformTransformVector {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformFromSRT {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FVector,
    pub rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    pub scale: crate::bindings::core_u_object::FVector,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub euler_transform: crate::bindings::animation_core::FEulerTransform,
}
impl FRigVMFunction_MathTransformFromSRT {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathTransformArrayToSRT {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub translations: TArray<crate::bindings::core_u_object::FVector>,
    pub rotations: TArray<crate::bindings::core_u_object::FQuat>,
    pub scales: TArray<crate::bindings::core_u_object::FVector>,
}
impl FRigVMFunction_MathTransformArrayToSRT {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformClampSpatially {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FTransform,
    pub axis: crate::bindings::core_u_object::EAxis,
    pub ty: ERigVMClampSpatialMode,
    pub minimum: f32,
    pub maximum: f32,
    pub space: crate::bindings::core_u_object::FTransform,
    pub b_draw_debug: bool,
    pub debug_color: crate::bindings::core_u_object::FLinearColor,
    pub debug_thickness: f32,
    pub result: crate::bindings::core_u_object::FTransform,
}
impl FRigVMFunction_MathTransformClampSpatially {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformMirrorTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FTransform,
    pub mirror_axis: crate::bindings::core_u_object::EAxis,
    pub axis_to_flip: crate::bindings::core_u_object::EAxis,
    pub central_transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
}
impl FRigVMFunction_MathTransformMirrorTransform {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorUnaryOp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorUnaryOp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorBinaryOp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorBinaryOp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorBinaryAggregateOp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorBinaryAggregateOp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMake {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorMake {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorFromFloat {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorFromFloat {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorFromDouble {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f64,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorFromDouble {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorAdd {
    pub(crate) __padding_end: [u8; 80],
}
impl FRigVMFunction_MathVectorAdd {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorSub {
    pub(crate) __padding_end: [u8; 80],
}
impl FRigVMFunction_MathVectorSub {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMul {
    pub(crate) __padding_end: [u8; 80],
}
impl FRigVMFunction_MathVectorMul {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorScale {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub factor: f32,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorScale {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorDiv {
    pub(crate) __padding_end: [u8; 80],
}
impl FRigVMFunction_MathVectorDiv {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMod {
    pub(crate) __padding_end: [u8; 80],
}
impl FRigVMFunction_MathVectorMod {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMin {
    pub(crate) __padding_end: [u8; 80],
}
impl FRigVMFunction_MathVectorMin {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMax {
    pub(crate) __padding_end: [u8; 80],
}
impl FRigVMFunction_MathVectorMax {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorNegate {
    pub(crate) __padding_end: [u8; 56],
}
impl FRigVMFunction_MathVectorNegate {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorAbs {
    pub(crate) __padding_end: [u8; 56],
}
impl FRigVMFunction_MathVectorAbs {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorFloor {
    pub(crate) __padding_end: [u8; 56],
}
impl FRigVMFunction_MathVectorFloor {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorCeil {
    pub(crate) __padding_end: [u8; 56],
}
impl FRigVMFunction_MathVectorCeil {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorRound {
    pub(crate) __padding_end: [u8; 56],
}
impl FRigVMFunction_MathVectorRound {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorSign {
    pub(crate) __padding_end: [u8; 56],
}
impl FRigVMFunction_MathVectorSign {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorClamp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub minimum: crate::bindings::core_u_object::FVector,
    pub maximum: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorClamp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorLerp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub t: f32,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorLerp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorRemap {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub source_minimum: crate::bindings::core_u_object::FVector,
    pub source_maximum: crate::bindings::core_u_object::FVector,
    pub target_minimum: crate::bindings::core_u_object::FVector,
    pub target_maximum: crate::bindings::core_u_object::FVector,
    pub b_clamp: bool,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorRemap {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorEquals {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub result: bool,
}
impl FRigVMFunction_MathVectorEquals {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorNotEquals {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub result: bool,
}
impl FRigVMFunction_MathVectorNotEquals {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorIsNearlyZero {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub tolerance: f32,
    pub result: bool,
}
impl FRigVMFunction_MathVectorIsNearlyZero {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorIsNearlyEqual {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub tolerance: f32,
    pub result: bool,
}
impl FRigVMFunction_MathVectorIsNearlyEqual {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorSelectBool {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub condition: bool,
    pub if_true: crate::bindings::core_u_object::FVector,
    pub if_false: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorSelectBool {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorDeg {
    pub(crate) __padding_end: [u8; 56],
}
impl FRigVMFunction_MathVectorDeg {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorRad {
    pub(crate) __padding_end: [u8; 56],
}
impl FRigVMFunction_MathVectorRad {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorLengthSquared {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub result: f32,
}
impl FRigVMFunction_MathVectorLengthSquared {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorLength {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub result: f32,
}
impl FRigVMFunction_MathVectorLength {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorDistance {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub result: f32,
}
impl FRigVMFunction_MathVectorDistance {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorCross {
    pub(crate) __padding_end: [u8; 80],
}
impl FRigVMFunction_MathVectorCross {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorDot {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub result: f32,
}
impl FRigVMFunction_MathVectorDot {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorUnit {
    pub(crate) __padding_end: [u8; 56],
}
impl FRigVMFunction_MathVectorUnit {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorSetLength {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub length: f32,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorSetLength {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorClampLength {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub minimum_length: f32,
    pub maximum_length: f32,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorClampLength {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMirror {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorMirror {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorAngle {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub result: f32,
}
impl FRigVMFunction_MathVectorAngle {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorParallel {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub result: bool,
}
impl FRigVMFunction_MathVectorParallel {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorOrthogonal {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub result: bool,
}
impl FRigVMFunction_MathVectorOrthogonal {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorBezierFourPoint {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub bezier: FRigVMFourPointBezier,
    pub t: f32,
    pub result: crate::bindings::core_u_object::FVector,
    pub tangent: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorBezierFourPoint {}
#[repr(C, align(8))]
pub struct FRigVMFourPointBezier {
    pub(crate) __padding_end: [u8; 96],
}
impl FRigVMFourPointBezier {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMakeBezierFourPoint {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub bezier: FRigVMFourPointBezier,
}
impl FRigVMFunction_MathVectorMakeBezierFourPoint {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathVectorClampSpatially {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub axis: crate::bindings::core_u_object::EAxis,
    pub ty: ERigVMClampSpatialMode,
    pub minimum: f32,
    pub maximum: f32,
    pub space: crate::bindings::core_u_object::FTransform,
    pub b_draw_debug: bool,
    pub debug_color: crate::bindings::core_u_object::FLinearColor,
    pub debug_thickness: f32,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorClampSpatially {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntersectPlane {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub start: crate::bindings::core_u_object::FVector,
    pub direction: crate::bindings::core_u_object::FVector,
    pub plane_point: crate::bindings::core_u_object::FVector,
    pub plane_normal: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
    pub distance: f32,
}
impl FRigVMFunction_MathIntersectPlane {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDistanceToPlane {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub point: crate::bindings::core_u_object::FVector,
    pub plane_point: crate::bindings::core_u_object::FVector,
    pub plane_normal: crate::bindings::core_u_object::FVector,
    pub closest_point_on_plane: crate::bindings::core_u_object::FVector,
    pub signed_distance: f32,
}
impl FRigVMFunction_MathDistanceToPlane {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMakeRelative {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub global: crate::bindings::core_u_object::FVector,
    pub parent: crate::bindings::core_u_object::FVector,
    pub local: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorMakeRelative {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMakeAbsolute {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub local: crate::bindings::core_u_object::FVector,
    pub parent: crate::bindings::core_u_object::FVector,
    pub global: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorMakeAbsolute {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathVectorMirrorTransform {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub mirror_axis: crate::bindings::core_u_object::EAxis,
    pub axis_to_flip: crate::bindings::core_u_object::EAxis,
    pub central_transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorMirrorTransform {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorArraySum {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub array: TArray<crate::bindings::core_u_object::FVector>,
    pub sum: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorArraySum {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorArrayAverage {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub array: TArray<crate::bindings::core_u_object::FVector>,
    pub average: crate::bindings::core_u_object::FVector,
}
impl FRigVMFunction_MathVectorArrayAverage {}
#[repr(C, align(8))]
pub struct FRigVMFunction_NoiseFloat {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub speed: f32,
    pub frequency: f32,
    pub minimum: f32,
    pub maximum: f32,
    pub result: f32,
    pub(crate) __padding_end: [u8; 8],
}
impl FRigVMFunction_NoiseFloat {}
#[repr(C, align(8))]
pub struct FRigVMFunction_NoiseDouble {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f64,
    pub speed: f64,
    pub frequency: f64,
    pub minimum: f64,
    pub maximum: f64,
    pub result: f64,
    pub(crate) __padding_end: [u8; 8],
}
impl FRigVMFunction_NoiseDouble {}
#[repr(C, align(8))]
pub struct FRigVMFunction_NoiseVector {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub position: crate::bindings::core_u_object::FVector,
    pub speed: crate::bindings::core_u_object::FVector,
    pub frequency: crate::bindings::core_u_object::FVector,
    pub minimum: f32,
    pub maximum: f32,
    pub result: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_NoiseVector {}
#[repr(C, align(8))]
pub struct FRigVMFunction_NoiseVector2 {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub speed: crate::bindings::core_u_object::FVector,
    pub frequency: crate::bindings::core_u_object::FVector,
    pub minimum: f64,
    pub maximum: f64,
    pub result: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_NoiseVector2 {}
#[repr(C, align(8))]
pub struct FRigVMFunction_RandomFloat {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub seed: i32,
    pub minimum: f32,
    pub maximum: f32,
    pub duration: f32,
    pub result: f32,
    pub(crate) __padding_end: [u8; 20],
}
impl FRigVMFunction_RandomFloat {}
#[repr(C, align(8))]
pub struct FRigVMFunction_RandomVector {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub seed: i32,
    pub minimum: f32,
    pub maximum: f32,
    pub duration: f32,
    pub result: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMFunction_RandomVector {}
#[repr(C, align(8))]
pub struct FRigVMMirrorSettings {
    pub mirror_axis: crate::bindings::core_u_object::EAxis,
    pub axis_to_flip: crate::bindings::core_u_object::EAxis,
    pub search_string: FString,
    pub replace_string: FString,
}
impl FRigVMMirrorSettings {}
#[repr(C, align(8))]
pub struct FRigVMSimPoint {
    pub(crate) __padding_end: [u8; 64],
}
impl FRigVMSimPoint {}
#[repr(C, align(16))]
pub struct FRigVMFunction_ControlFlowBranch {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub execute_context: FRigVMExecuteContext,
    pub condition: bool,
    pub true_: FRigVMExecuteContext,
    pub false_: FRigVMExecuteContext,
    pub completed: FRigVMExecuteContext,
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunction_ControlFlowBranch {}
#[repr(C, align(8))]
pub struct FRigVMFunction_NameConcat {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: FName,
    pub b: FName,
    pub result: FName,
}
impl FRigVMFunction_NameConcat {}
#[repr(C, align(8))]
pub struct FRigVMFunction_NameTruncate {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub name: FName,
    pub count: i32,
    pub from_end: bool,
    pub remainder: FName,
    pub chopped: FName,
}
impl FRigVMFunction_NameTruncate {}
#[repr(C, align(8))]
pub struct FRigVMFunction_NameReplace {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub name: FName,
    pub old: FName,
    pub new: FName,
    pub result: FName,
}
impl FRigVMFunction_NameReplace {}
#[repr(C, align(8))]
pub struct FRigVMFunction_EndsWith {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub name: FName,
    pub ending: FName,
    pub result: bool,
}
impl FRigVMFunction_EndsWith {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StartsWith {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub name: FName,
    pub start: FName,
    pub result: bool,
}
impl FRigVMFunction_StartsWith {}
#[repr(C, align(8))]
pub struct FRigVMFunction_Contains {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub name: FName,
    pub search: FName,
    pub result: bool,
}
impl FRigVMFunction_Contains {}
#[repr(C, align(8))]
pub struct FRigVMFunction_IsNone {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub name: FName,
    pub result: bool,
}
impl FRigVMFunction_IsNone {}
#[repr(C, align(8))]
pub struct FRigVMFunction_IsNameValid {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: FName,
    pub result: bool,
}
impl FRigVMFunction_IsNameValid {}
#[repr(C, align(8))]
pub struct FRigVMFunction_GetNameNumericSuffix {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub name: FName,
    pub suffix: i32,
    pub success: bool,
}
impl FRigVMFunction_GetNameNumericSuffix {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringConcat {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: FString,
    pub b: FString,
    pub result: FString,
}
impl FRigVMFunction_StringConcat {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringTruncate {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub name: FString,
    pub count: i32,
    pub from_end: bool,
    pub remainder: FString,
    pub chopped: FString,
}
impl FRigVMFunction_StringTruncate {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringReplace {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub name: FString,
    pub old: FString,
    pub new: FString,
    pub result: FString,
}
impl FRigVMFunction_StringReplace {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringEndsWith {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub name: FString,
    pub ending: FString,
    pub result: bool,
}
impl FRigVMFunction_StringEndsWith {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringStartsWith {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub name: FString,
    pub start: FString,
    pub result: bool,
}
impl FRigVMFunction_StringStartsWith {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringContains {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub name: FString,
    pub search: FString,
    pub result: bool,
}
impl FRigVMFunction_StringContains {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringLength {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: FString,
    pub length: i32,
}
impl FRigVMFunction_StringLength {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringTrimWhitespace {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: FString,
    pub result: FString,
}
impl FRigVMFunction_StringTrimWhitespace {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringToUppercase {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: FString,
    pub result: FString,
}
impl FRigVMFunction_StringToUppercase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringToLowercase {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: FString,
    pub result: FString,
}
impl FRigVMFunction_StringToLowercase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringReverse {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: FString,
    pub reverse: FString,
}
impl FRigVMFunction_StringReverse {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringLeft {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: FString,
    pub count: i32,
    pub result: FString,
}
impl FRigVMFunction_StringLeft {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringRight {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: FString,
    pub count: i32,
    pub result: FString,
}
impl FRigVMFunction_StringRight {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringMiddle {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: FString,
    pub start: i32,
    pub count: i32,
    pub result: FString,
}
impl FRigVMFunction_StringMiddle {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringFind {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: FString,
    pub search: FString,
    pub found: bool,
    pub index: i32,
}
impl FRigVMFunction_StringFind {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringSplit {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: FString,
    pub separator: FString,
    pub result: TArray<FString>,
}
impl FRigVMFunction_StringSplit {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringJoin {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub values: TArray<FString>,
    pub separator: FString,
    pub result: FString,
}
impl FRigVMFunction_StringJoin {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringPadInteger {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: i32,
    pub digits: i32,
    pub result: FString,
}
impl FRigVMFunction_StringPadInteger {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringToInteger {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: FString,
    pub chop_left: bool,
    pub chop_right: bool,
    pub result: i32,
    pub success: bool,
}
impl FRigVMFunction_StringToInteger {}
#[repr(C, align(8))]
pub struct FRigVMFunction_AccumulateFloatAdd {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub increment: f32,
    pub initial_value: f32,
    pub b_integrate_delta_time: bool,
    pub result: f32,
    pub(crate) __padding_end: [u8; 8],
}
impl FRigVMFunction_AccumulateFloatAdd {}
#[repr(C, align(8))]
pub struct FRigVMFunction_AccumulateVectorAdd {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub increment: crate::bindings::core_u_object::FVector,
    pub initial_value: crate::bindings::core_u_object::FVector,
    pub b_integrate_delta_time: bool,
    pub result: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigVMFunction_AccumulateVectorAdd {}
#[repr(C, align(8))]
pub struct FRigVMFunction_AccumulateFloatMul {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub multiplier: f32,
    pub initial_value: f32,
    pub b_integrate_delta_time: bool,
    pub result: f32,
    pub(crate) __padding_end: [u8; 8],
}
impl FRigVMFunction_AccumulateFloatMul {}
#[repr(C, align(8))]
pub struct FRigVMFunction_AccumulateVectorMul {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub multiplier: crate::bindings::core_u_object::FVector,
    pub initial_value: crate::bindings::core_u_object::FVector,
    pub b_integrate_delta_time: bool,
    pub result: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigVMFunction_AccumulateVectorMul {}
#[repr(C, align(16))]
pub struct FRigVMFunction_AccumulateQuatMul {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub multiplier: crate::bindings::core_u_object::FQuat,
    pub initial_value: crate::bindings::core_u_object::FQuat,
    pub b_flip_order: bool,
    pub b_integrate_delta_time: bool,
    pub result: crate::bindings::core_u_object::FQuat,
    pub(crate) __padding_end: [u8; 48],
}
impl FRigVMFunction_AccumulateQuatMul {}
#[repr(C, align(16))]
pub struct FRigVMFunction_AccumulateTransformMul {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub multiplier: crate::bindings::core_u_object::FTransform,
    pub initial_value: crate::bindings::core_u_object::FTransform,
    pub b_flip_order: bool,
    pub b_integrate_delta_time: bool,
    pub result: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 112],
}
impl FRigVMFunction_AccumulateTransformMul {}
#[repr(C, align(8))]
pub struct FRigVMFunction_AccumulateFloatLerp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub target_value: f32,
    pub initial_value: f32,
    pub blend: f32,
    pub b_integrate_delta_time: bool,
    pub result: f32,
    pub(crate) __padding_end: [u8; 12],
}
impl FRigVMFunction_AccumulateFloatLerp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_AccumulateVectorLerp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub target_value: crate::bindings::core_u_object::FVector,
    pub initial_value: crate::bindings::core_u_object::FVector,
    pub blend: f32,
    pub b_integrate_delta_time: bool,
    pub result: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigVMFunction_AccumulateVectorLerp {}
#[repr(C, align(16))]
pub struct FRigVMFunction_AccumulateQuatLerp {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub target_value: crate::bindings::core_u_object::FQuat,
    pub initial_value: crate::bindings::core_u_object::FQuat,
    pub blend: f32,
    pub b_integrate_delta_time: bool,
    pub result: crate::bindings::core_u_object::FQuat,
    pub(crate) __padding_end: [u8; 48],
}
impl FRigVMFunction_AccumulateQuatLerp {}
#[repr(C, align(16))]
pub struct FRigVMFunction_AccumulateTransformLerp {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub target_value: crate::bindings::core_u_object::FTransform,
    pub initial_value: crate::bindings::core_u_object::FTransform,
    pub blend: f32,
    pub b_integrate_delta_time: bool,
    pub result: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 112],
}
impl FRigVMFunction_AccumulateTransformLerp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_AccumulateFloatRange {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub minimum: f32,
    pub maximum: f32,
    pub(crate) __padding_end: [u8; 12],
}
impl FRigVMFunction_AccumulateFloatRange {}
#[repr(C, align(8))]
pub struct FRigVMFunction_AccumulateVectorRange {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub minimum: crate::bindings::core_u_object::FVector,
    pub maximum: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 56],
}
impl FRigVMFunction_AccumulateVectorRange {}
#[repr(C, align(8))]
pub struct FRigVMFunction_AlphaInterp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub scale: f32,
    pub bias: f32,
    pub b_map_range: bool,
    pub in_range: crate::bindings::engine::FInputRange,
    pub out_range: crate::bindings::engine::FInputRange,
    pub b_clamp_result: bool,
    pub clamp_min: f32,
    pub clamp_max: f32,
    pub b_interp_result: bool,
    pub interp_speed_increasing: f32,
    pub interp_speed_decreasing: f32,
    pub result: f32,
    pub(crate) __padding_end: [u8; 52],
}
impl FRigVMFunction_AlphaInterp {}
#[repr(C, align(8))]
pub struct FRigVMFunction_AlphaInterpVector {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub scale: f32,
    pub bias: f32,
    pub b_map_range: bool,
    pub in_range: crate::bindings::engine::FInputRange,
    pub out_range: crate::bindings::engine::FInputRange,
    pub b_clamp_result: bool,
    pub clamp_min: f32,
    pub clamp_max: f32,
    pub b_interp_result: bool,
    pub interp_speed_increasing: f32,
    pub interp_speed_decreasing: f32,
    pub result: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 48],
}
impl FRigVMFunction_AlphaInterpVector {}
#[repr(C, align(16))]
pub struct FRigVMFunction_AlphaInterpQuat {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub scale: f32,
    pub bias: f32,
    pub b_map_range: bool,
    pub in_range: crate::bindings::engine::FInputRange,
    pub out_range: crate::bindings::engine::FInputRange,
    pub b_clamp_result: bool,
    pub clamp_min: f32,
    pub clamp_max: f32,
    pub b_interp_result: bool,
    pub interp_speed_increasing: f32,
    pub interp_speed_decreasing: f32,
    pub result: crate::bindings::core_u_object::FQuat,
    pub(crate) __padding_end: [u8; 48],
}
impl FRigVMFunction_AlphaInterpQuat {}
#[repr(C, align(8))]
pub struct FRigVMFunction_DeltaFromPreviousFloat {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub delta: f32,
    pub previous_value: f32,
    pub(crate) __padding_end: [u8; 12],
}
impl FRigVMFunction_DeltaFromPreviousFloat {}
#[repr(C, align(8))]
pub struct FRigVMFunction_DeltaFromPreviousVector {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub delta: crate::bindings::core_u_object::FVector,
    pub previous_value: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigVMFunction_DeltaFromPreviousVector {}
#[repr(C, align(16))]
pub struct FRigVMFunction_DeltaFromPreviousQuat {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub delta: crate::bindings::core_u_object::FQuat,
    pub previous_value: crate::bindings::core_u_object::FQuat,
    pub(crate) __padding_end: [u8; 48],
}
impl FRigVMFunction_DeltaFromPreviousQuat {}
#[repr(C, align(16))]
pub struct FRigVMFunction_DeltaFromPreviousTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FTransform,
    pub delta: crate::bindings::core_u_object::FTransform,
    pub previous_value: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 112],
}
impl FRigVMFunction_DeltaFromPreviousTransform {}
#[repr(C, align(8))]
pub struct FRigVMFunction_KalmanFloat {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub buffer_size: i32,
    pub result: f32,
    pub(crate) __padding_end: [u8; 28],
}
impl FRigVMFunction_KalmanFloat {}
#[repr(C, align(8))]
pub struct FRigVMFunction_KalmanVector {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub buffer_size: i32,
    pub result: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 24],
}
impl FRigVMFunction_KalmanVector {}
#[repr(C, align(16))]
pub struct FRigVMFunction_KalmanTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FTransform,
    pub buffer_size: i32,
    pub result: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigVMFunction_KalmanTransform {}
#[repr(C, align(8))]
pub struct FRigVMFunction_Timeline {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub speed: f32,
    pub time: f32,
    pub(crate) __padding_end: [u8; 8],
}
impl FRigVMFunction_Timeline {}
#[repr(C, align(8))]
pub struct FRigVMFunction_TimeLoop {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub speed: f32,
    pub duration: f32,
    pub normalize: bool,
    pub absolute: f32,
    pub relative: f32,
    pub flip_flop: f32,
    pub even: bool,
    pub(crate) __padding_end: [u8; 23],
}
impl FRigVMFunction_TimeLoop {}
#[repr(C, align(8))]
pub struct FRigVMFunction_TimeOffsetFloat {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub seconds_ago: f32,
    pub buffer_size: i32,
    pub time_range: f32,
    pub result: f32,
    pub(crate) __padding_end: [u8; 44],
}
impl FRigVMFunction_TimeOffsetFloat {}
#[repr(C, align(8))]
pub struct FRigVMFunction_TimeOffsetVector {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub seconds_ago: f32,
    pub buffer_size: i32,
    pub time_range: f32,
    pub result: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 40],
}
impl FRigVMFunction_TimeOffsetVector {}
#[repr(C, align(16))]
pub struct FRigVMFunction_TimeOffsetTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FTransform,
    pub seconds_ago: f32,
    pub buffer_size: i32,
    pub time_range: f32,
    pub result: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 48],
}
impl FRigVMFunction_TimeOffsetTransform {}
#[repr(C, align(8))]
pub struct FRigVMFunction_VerletIntegrateVector {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub target: crate::bindings::core_u_object::FVector,
    pub strength: f32,
    pub damp: f32,
    pub blend: f32,
    pub force: crate::bindings::core_u_object::FVector,
    pub position: crate::bindings::core_u_object::FVector,
    pub velocity: crate::bindings::core_u_object::FVector,
    pub acceleration: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 72],
}
impl FRigVMFunction_VerletIntegrateVector {}
#[repr(C, align(8))]
pub struct URigVMBlueprintGeneratedClass {
    __padding_end: [u8; 1728],
}
impl URigVMBlueprintGeneratedClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMBlueprintGeneratedClass")
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
pub struct URigVM {
    __padding_end: [u8; 1320],
}
impl URigVM {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("URigVM").unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_parameter_value_vector2_d(
        &mut self,
        in_parameter_name: &FName,
        in_value: &crate::bindings::core_u_object::FVector2D,
        in_array_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_set_parameter_value_vector2_d,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_index,
                __buffer.add(32).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_set_parameter_value_vector2_d,
                __buffer,
            )
        };
    }
    pub fn set_parameter_value_vector(
        &mut self,
        in_parameter_name: &FName,
        in_value: &crate::bindings::core_u_object::FVector,
        in_array_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_set_parameter_value_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_index,
                __buffer.add(40).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_set_parameter_value_vector,
                __buffer,
            )
        };
    }
    pub fn set_parameter_value_transform(
        &mut self,
        in_parameter_name: &FName,
        in_value: &crate::bindings::core_u_object::FTransform,
        in_array_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<116>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_set_parameter_value_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_index,
                __buffer.add(112).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_set_parameter_value_transform,
                __buffer,
            )
        };
    }
    pub fn set_parameter_value_string(
        &mut self,
        in_parameter_name: &FName,
        in_value: FString,
        in_array_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_set_parameter_value_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_index,
                __buffer.add(32).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_set_parameter_value_string,
                __buffer,
            )
        };
    }
    pub fn set_parameter_value_quat(
        &mut self,
        in_parameter_name: &FName,
        in_value: &crate::bindings::core_u_object::FQuat,
        in_array_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<52>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_set_parameter_value_quat,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_index,
                __buffer.add(48).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_set_parameter_value_quat,
                __buffer,
            )
        };
    }
    pub fn set_parameter_value_name(
        &mut self,
        in_parameter_name: &FName,
        in_value: &FName,
        in_array_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_set_parameter_value_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(in_value, __buffer.add(12).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_index,
                __buffer.add(24).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_set_parameter_value_name,
                __buffer,
            )
        };
    }
    pub fn set_parameter_value_int(
        &mut self,
        in_parameter_name: &FName,
        in_value: i32,
        in_array_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_set_parameter_value_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_index,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_set_parameter_value_int,
                __buffer,
            )
        };
    }
    pub fn set_parameter_value_float(
        &mut self,
        in_parameter_name: &FName,
        in_value: f32,
        in_array_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_set_parameter_value_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(12).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_index,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_set_parameter_value_float,
                __buffer,
            )
        };
    }
    pub fn set_parameter_value_double(
        &mut self,
        in_parameter_name: &FName,
        in_value: f64,
        in_array_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_set_parameter_value_double,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(16).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_index,
                __buffer.add(24).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_set_parameter_value_double,
                __buffer,
            )
        };
    }
    pub fn set_parameter_value_bool(
        &mut self,
        in_parameter_name: &FName,
        in_value: bool,
        in_array_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_set_parameter_value_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(12).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_index,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_set_parameter_value_bool,
                __buffer,
            )
        };
    }
    pub fn get_statistics(&self) -> FRigVMStatistics {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_get_statistics,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_get_statistics,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FRigVMStatistics>().read() }
    }
    pub fn get_parameter_value_vector2_d(
        &mut self,
        in_parameter_name: &FName,
        in_array_index: i32,
    ) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_get_parameter_value_vector2_d,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_get_parameter_value_vector2_d,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn get_parameter_value_vector(
        &mut self,
        in_parameter_name: &FName,
        in_array_index: i32,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_get_parameter_value_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_get_parameter_value_vector,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_parameter_value_transform(
        &mut self,
        in_parameter_name: &FName,
        in_array_index: i32,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_get_parameter_value_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_get_parameter_value_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_parameter_value_string(
        &mut self,
        in_parameter_name: &FName,
        in_array_index: i32,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_get_parameter_value_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_get_parameter_value_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn get_parameter_value_quat(
        &mut self,
        in_parameter_name: &FName,
        in_array_index: i32,
    ) -> crate::bindings::core_u_object::FQuat {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_get_parameter_value_quat,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_get_parameter_value_quat,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FQuat>().read()
        }
    }
    pub fn get_parameter_value_name(
        &mut self,
        in_parameter_name: &FName,
        in_array_index: i32,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_get_parameter_value_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_get_parameter_value_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FName>().read() }
    }
    pub fn get_parameter_value_int(
        &mut self,
        in_parameter_name: &FName,
        in_array_index: i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_get_parameter_value_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_get_parameter_value_int,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn get_parameter_value_float(
        &mut self,
        in_parameter_name: &FName,
        in_array_index: i32,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_get_parameter_value_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_get_parameter_value_float,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn get_parameter_value_double(
        &mut self,
        in_parameter_name: &FName,
        in_array_index: i32,
    ) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_get_parameter_value_double,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_get_parameter_value_double,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f64>().read() }
    }
    pub fn get_parameter_value_bool(
        &mut self,
        in_parameter_name: &FName,
        in_array_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_get_parameter_value_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_get_parameter_value_bool,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn execute(
        &mut self,
        context: &mut FRigVMExtendedExecuteContext,
        in_entry_name: &FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<781>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_execute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FRigVMExtendedExecuteContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_entry_name,
                __buffer.add(768).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_execute,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FRigVMExtendedExecuteContext>().swap(context);
        }
        unsafe { __buffer.add(780).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UNameSpacedUserData {
    __padding_end: [u8; 264],
}
impl UNameSpacedUserData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNameSpacedUserData")
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
pub struct UDataAssetLink {
    #[doc(hidden)]
    pub(crate) __padding_264: [u8; 264],
    pub data_asset: TSoftObjectPtr<crate::bindings::engine::UDataAsset>,
    __padding_end: [u8; 8],
}
impl UDataAssetLink {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataAssetLink")
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
    pub fn set_data_asset(
        &mut self,
        in_data_asset: TSoftObjectPtr<crate::bindings::engine::UDataAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_data_asset_link_set_data_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_data_asset,
                __buffer
                    .add(0)
                    .cast::<TSoftObjectPtr<crate::bindings::engine::UDataAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_data_asset_link_set_data_asset,
                __buffer,
            )
        };
    }
    pub fn get_data_asset(&self) -> TSoftObjectPtr<crate::bindings::engine::UDataAsset> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_data_asset_link_get_data_asset,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_data_asset_link_get_data_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TSoftObjectPtr<crate::bindings::engine::UDataAsset>>()
                .read()
        }
    }
}
pub struct IRigVMGraphFunctionHost {}
#[repr(C, align(8))]
pub struct URigVMGraphFunctionHost {
    __padding_end: [u8; 48],
}
impl URigVMGraphFunctionHost {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMGraphFunctionHost")
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
pub struct URigVMMemoryStorageGeneratorClass {
    __padding_end: [u8; 688],
}
impl URigVMMemoryStorageGeneratorClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMMemoryStorageGeneratorClass")
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
pub struct URigVMMemoryStorage {
    __padding_end: [u8; 48],
}
impl URigVMMemoryStorage {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMMemoryStorage")
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
pub struct URigVMNativized {
    __padding_end: [u8; 1360],
}
impl URigVMNativized {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMNativized")
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
pub struct URigVMUserWorkflowOptions {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub subject: UPtr<crate::bindings::core_u_object::UObject>,
    pub workflow: FRigVMUserWorkflow,
    __padding_end: [u8; 24],
}
impl URigVMUserWorkflowOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMUserWorkflowOptions")
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
    pub fn requires_dialog(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_user_workflow_options_requires_dialog,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_user_workflow_options_requires_dialog,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn report_warning(&mut self, in_message: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_user_workflow_options_report_warning,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_message,
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
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_user_workflow_options_report_warning,
                __buffer,
            )
        };
    }
    pub fn report_info(&mut self, in_message: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_user_workflow_options_report_info,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_message,
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
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_user_workflow_options_report_info,
                __buffer,
            )
        };
    }
    pub fn report_error(&mut self, in_message: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_user_workflow_options_report_error,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_message,
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
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_user_workflow_options_report_error,
                __buffer,
            )
        };
    }
    pub fn is_valid(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_user_workflow_options_is_valid,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_user_workflow_options_is_valid,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(16))]
pub struct URigVMHost {
    __padding_end: [u8; 1712],
}
impl URigVMHost {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMHost")
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
    pub fn supports_event(&self, in_event_name: &FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_supports_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_event_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_supports_event,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn set_variable_from_string(
        &mut self,
        in_variable_name: &FName,
        in_value: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_host_set_variable_from_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
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
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_host_set_variable_from_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_frames_per_second(&mut self, in_frames_per_second: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_host_set_frames_per_second,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_frames_per_second,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_host_set_frames_per_second,
                __buffer,
            )
        };
    }
    pub fn set_delta_time(&mut self, in_delta_time: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_set_delta_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_delta_time,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_set_delta_time,
                __buffer,
            )
        };
    }
    pub fn set_absolute_time(
        &mut self,
        in_absolute_time: f32,
        in_set_delta_time_zero: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_set_absolute_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_absolute_time,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_set_delta_time_zero,
                __buffer.add(4).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_set_absolute_time,
                __buffer,
            )
        };
    }
    pub fn set_absolute_and_delta_time(
        &mut self,
        in_absolute_time: f32,
        in_delta_time: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_host_set_absolute_and_delta_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_absolute_time,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_delta_time,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_host_set_absolute_and_delta_time,
                __buffer,
            )
        };
    }
    pub fn request_run_once_event(
        &mut self,
        in_event_name: &FName,
        in_event_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_host_request_run_once_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_event_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_event_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_host_request_run_once_event,
                __buffer,
            )
        };
    }
    pub fn request_init(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_request_init,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_request_init,
                __buffer,
            )
        };
    }
    pub fn remove_run_once_event(&mut self, in_event_name: &FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_host_remove_run_once_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_event_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_host_remove_run_once_event,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn is_init_required(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_is_init_required,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_is_init_required,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_vm(&mut self) -> UPtr<URigVM> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_get_vm,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_get_vm,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVM>>().read() }
    }
    pub fn get_variable_type(&self, in_variable_name: &FName) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_get_variable_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_get_variable_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FName>().read() }
    }
    pub fn get_variable_as_string(&self, in_variable_name: &FName) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_host_get_variable_as_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_host_get_variable_as_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn get_supported_events(&self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_host_get_supported_events,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_host_get_supported_events,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn get_script_accessible_variables(&self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_host_get_script_accessible_variables,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_host_get_script_accessible_variables,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn get_extended_execute_context(&mut self) -> FRigVMExtendedExecuteContext {
        let mut __stack = crate::core_data::StackAlloc::<768>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_host_get_extended_execute_context,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_host_get_extended_execute_context,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FRigVMExtendedExecuteContext>().read() }
    }
    pub fn get_delta_time(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_get_delta_time,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_get_delta_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_current_frames_per_second(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_host_get_current_frames_per_second,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_host_get_current_frames_per_second,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_absolute_time(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_get_absolute_time,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_get_absolute_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn find_rig_vm_hosts(
        outer: UPtr<crate::bindings::core_u_object::UObject>,
        optional_class: TSubclassOf<URigVMHost>,
    ) -> TArray<UPtr<URigVMHost>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_find_rig_vm_hosts,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &outer,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_class,
                __buffer.add(8).cast::<TSubclassOf<URigVMHost>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rig_vm::URigVMHost::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_find_rig_vm_hosts,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TArray<UPtr<URigVMHost>>>().read() }
    }
    pub fn execute_event(&mut self, in_event_name: &FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_execute_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_event_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_execute_event,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn execute(&mut self, in_event_name: &FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_execute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_event_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_execute,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn can_execute(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_can_execute,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS.u_rig_vm_host_can_execute,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMEditorSettings {
    __padding_end: [u8; 112],
}
impl URigVMEditorSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEditorSettings")
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
pub struct URigVMProjectSettings {
    __padding_end: [u8; 120],
}
impl URigVMProjectSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMProjectSettings")
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
    pub fn get_tag(&self, in_tag_name: FName) -> FRigVMTag {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_project_settings_get_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_tag_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm::__FUNCTION_PTRS
                    .u_rig_vm_project_settings_get_tag,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FRigVMTag>().read() }
    }
}
#[repr(C, align(8))]
pub struct FRigVMUserWorkflow_PerformDynamicDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FGetSupportedWorkflows_PerformDynamicDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FGetWorkflows_PerformDynamicDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FMakeOptionsForWorkflow_PerformDynamicDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FPerformUserWorkflow_PerformDynamicDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FProvideWorkflows_PerformDynamicDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FRigVMUserWorkflowOptions_PerformDynamicDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FRigVMUserWorkflowProvider__DelegateSignature_PerformDynamicDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCallPython_PerformDynamicDelegate {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct ERigVMUserWorkflowType(pub u8);
impl ERigVMUserWorkflowType {
    pub const INVALID: ERigVMUserWorkflowType = ERigVMUserWorkflowType(0);
    pub const NODE_CONTEXT: ERigVMUserWorkflowType = ERigVMUserWorkflowType(1);
    pub const PIN_CONTEXT: ERigVMUserWorkflowType = ERigVMUserWorkflowType(2);
    pub const ON_PIN_DEFAULT_CHANGED: ERigVMUserWorkflowType = ERigVMUserWorkflowType(4);
    pub const NODE_CONTEXT_BUTTON: ERigVMUserWorkflowType = ERigVMUserWorkflowType(8);
    pub const ALL: ERigVMUserWorkflowType = ERigVMUserWorkflowType(15);
}
#[repr(transparent)]
pub struct ERigVMDrawSettings(pub u8);
impl ERigVMDrawSettings {
    pub const POINTS: ERigVMDrawSettings = ERigVMDrawSettings(0);
    pub const LINES: ERigVMDrawSettings = ERigVMDrawSettings(1);
    pub const LINE_STRIP: ERigVMDrawSettings = ERigVMDrawSettings(2);
    pub const DYNAMIC_MESH: ERigVMDrawSettings = ERigVMDrawSettings(3);
}
#[repr(transparent)]
pub struct ERigVMParameterType(pub u8);
impl ERigVMParameterType {
    pub const INPUT: ERigVMParameterType = ERigVMParameterType(0);
    pub const OUTPUT: ERigVMParameterType = ERigVMParameterType(1);
    pub const INVALID: ERigVMParameterType = ERigVMParameterType(2);
}
#[repr(transparent)]
pub struct ERigVMOpCode(pub u8);
impl ERigVMOpCode {
    pub const EXECUTE_0_OPERANDS: ERigVMOpCode = ERigVMOpCode(0);
    pub const EXECUTE_1_OPERANDS: ERigVMOpCode = ERigVMOpCode(1);
    pub const EXECUTE_2_OPERANDS: ERigVMOpCode = ERigVMOpCode(2);
    pub const EXECUTE_3_OPERANDS: ERigVMOpCode = ERigVMOpCode(3);
    pub const EXECUTE_4_OPERANDS: ERigVMOpCode = ERigVMOpCode(4);
    pub const EXECUTE_5_OPERANDS: ERigVMOpCode = ERigVMOpCode(5);
    pub const EXECUTE_6_OPERANDS: ERigVMOpCode = ERigVMOpCode(6);
    pub const EXECUTE_7_OPERANDS: ERigVMOpCode = ERigVMOpCode(7);
    pub const EXECUTE_8_OPERANDS: ERigVMOpCode = ERigVMOpCode(8);
    pub const EXECUTE_9_OPERANDS: ERigVMOpCode = ERigVMOpCode(9);
    pub const EXECUTE_10_OPERANDS: ERigVMOpCode = ERigVMOpCode(10);
    pub const EXECUTE_11_OPERANDS: ERigVMOpCode = ERigVMOpCode(11);
    pub const EXECUTE_12_OPERANDS: ERigVMOpCode = ERigVMOpCode(12);
    pub const EXECUTE_13_OPERANDS: ERigVMOpCode = ERigVMOpCode(13);
    pub const EXECUTE_14_OPERANDS: ERigVMOpCode = ERigVMOpCode(14);
    pub const EXECUTE_15_OPERANDS: ERigVMOpCode = ERigVMOpCode(15);
    pub const EXECUTE_16_OPERANDS: ERigVMOpCode = ERigVMOpCode(16);
    pub const EXECUTE_17_OPERANDS: ERigVMOpCode = ERigVMOpCode(17);
    pub const EXECUTE_18_OPERANDS: ERigVMOpCode = ERigVMOpCode(18);
    pub const EXECUTE_19_OPERANDS: ERigVMOpCode = ERigVMOpCode(19);
    pub const EXECUTE_20_OPERANDS: ERigVMOpCode = ERigVMOpCode(20);
    pub const EXECUTE_21_OPERANDS: ERigVMOpCode = ERigVMOpCode(21);
    pub const EXECUTE_22_OPERANDS: ERigVMOpCode = ERigVMOpCode(22);
    pub const EXECUTE_23_OPERANDS: ERigVMOpCode = ERigVMOpCode(23);
    pub const EXECUTE_24_OPERANDS: ERigVMOpCode = ERigVMOpCode(24);
    pub const EXECUTE_25_OPERANDS: ERigVMOpCode = ERigVMOpCode(25);
    pub const EXECUTE_26_OPERANDS: ERigVMOpCode = ERigVMOpCode(26);
    pub const EXECUTE_27_OPERANDS: ERigVMOpCode = ERigVMOpCode(27);
    pub const EXECUTE_28_OPERANDS: ERigVMOpCode = ERigVMOpCode(28);
    pub const EXECUTE_29_OPERANDS: ERigVMOpCode = ERigVMOpCode(29);
    pub const EXECUTE_30_OPERANDS: ERigVMOpCode = ERigVMOpCode(30);
    pub const EXECUTE_31_OPERANDS: ERigVMOpCode = ERigVMOpCode(31);
    pub const EXECUTE_32_OPERANDS: ERigVMOpCode = ERigVMOpCode(32);
    pub const EXECUTE_33_OPERANDS: ERigVMOpCode = ERigVMOpCode(33);
    pub const EXECUTE_34_OPERANDS: ERigVMOpCode = ERigVMOpCode(34);
    pub const EXECUTE_35_OPERANDS: ERigVMOpCode = ERigVMOpCode(35);
    pub const EXECUTE_36_OPERANDS: ERigVMOpCode = ERigVMOpCode(36);
    pub const EXECUTE_37_OPERANDS: ERigVMOpCode = ERigVMOpCode(37);
    pub const EXECUTE_38_OPERANDS: ERigVMOpCode = ERigVMOpCode(38);
    pub const EXECUTE_39_OPERANDS: ERigVMOpCode = ERigVMOpCode(39);
    pub const EXECUTE_40_OPERANDS: ERigVMOpCode = ERigVMOpCode(40);
    pub const EXECUTE_41_OPERANDS: ERigVMOpCode = ERigVMOpCode(41);
    pub const EXECUTE_42_OPERANDS: ERigVMOpCode = ERigVMOpCode(42);
    pub const EXECUTE_43_OPERANDS: ERigVMOpCode = ERigVMOpCode(43);
    pub const EXECUTE_44_OPERANDS: ERigVMOpCode = ERigVMOpCode(44);
    pub const EXECUTE_45_OPERANDS: ERigVMOpCode = ERigVMOpCode(45);
    pub const EXECUTE_46_OPERANDS: ERigVMOpCode = ERigVMOpCode(46);
    pub const EXECUTE_47_OPERANDS: ERigVMOpCode = ERigVMOpCode(47);
    pub const EXECUTE_48_OPERANDS: ERigVMOpCode = ERigVMOpCode(48);
    pub const EXECUTE_49_OPERANDS: ERigVMOpCode = ERigVMOpCode(49);
    pub const EXECUTE_50_OPERANDS: ERigVMOpCode = ERigVMOpCode(50);
    pub const EXECUTE_51_OPERANDS: ERigVMOpCode = ERigVMOpCode(51);
    pub const EXECUTE_52_OPERANDS: ERigVMOpCode = ERigVMOpCode(52);
    pub const EXECUTE_53_OPERANDS: ERigVMOpCode = ERigVMOpCode(53);
    pub const EXECUTE_54_OPERANDS: ERigVMOpCode = ERigVMOpCode(54);
    pub const EXECUTE_55_OPERANDS: ERigVMOpCode = ERigVMOpCode(55);
    pub const EXECUTE_56_OPERANDS: ERigVMOpCode = ERigVMOpCode(56);
    pub const EXECUTE_57_OPERANDS: ERigVMOpCode = ERigVMOpCode(57);
    pub const EXECUTE_58_OPERANDS: ERigVMOpCode = ERigVMOpCode(58);
    pub const EXECUTE_59_OPERANDS: ERigVMOpCode = ERigVMOpCode(59);
    pub const EXECUTE_60_OPERANDS: ERigVMOpCode = ERigVMOpCode(60);
    pub const EXECUTE_61_OPERANDS: ERigVMOpCode = ERigVMOpCode(61);
    pub const EXECUTE_62_OPERANDS: ERigVMOpCode = ERigVMOpCode(62);
    pub const EXECUTE_63_OPERANDS: ERigVMOpCode = ERigVMOpCode(63);
    pub const EXECUTE_64_OPERANDS: ERigVMOpCode = ERigVMOpCode(64);
    pub const ZERO: ERigVMOpCode = ERigVMOpCode(65);
    pub const BOOL_FALSE: ERigVMOpCode = ERigVMOpCode(66);
    pub const BOOL_TRUE: ERigVMOpCode = ERigVMOpCode(67);
    pub const COPY: ERigVMOpCode = ERigVMOpCode(68);
    pub const INCREMENT: ERigVMOpCode = ERigVMOpCode(69);
    pub const DECREMENT: ERigVMOpCode = ERigVMOpCode(70);
    pub const EQUALS: ERigVMOpCode = ERigVMOpCode(71);
    pub const NOT_EQUALS: ERigVMOpCode = ERigVMOpCode(72);
    pub const JUMP_ABSOLUTE: ERigVMOpCode = ERigVMOpCode(73);
    pub const JUMP_FORWARD: ERigVMOpCode = ERigVMOpCode(74);
    pub const JUMP_BACKWARD: ERigVMOpCode = ERigVMOpCode(75);
    pub const JUMP_ABSOLUTE_IF: ERigVMOpCode = ERigVMOpCode(76);
    pub const JUMP_FORWARD_IF: ERigVMOpCode = ERigVMOpCode(77);
    pub const JUMP_BACKWARD_IF: ERigVMOpCode = ERigVMOpCode(78);
    pub const CHANGE_TYPE: ERigVMOpCode = ERigVMOpCode(79);
    pub const EXIT: ERigVMOpCode = ERigVMOpCode(80);
    pub const BEGIN_BLOCK: ERigVMOpCode = ERigVMOpCode(81);
    pub const END_BLOCK: ERigVMOpCode = ERigVMOpCode(82);
    pub const ARRAY_RESET: ERigVMOpCode = ERigVMOpCode(83);
    pub const ARRAY_GET_NUM: ERigVMOpCode = ERigVMOpCode(84);
    pub const ARRAY_SET_NUM: ERigVMOpCode = ERigVMOpCode(85);
    pub const ARRAY_GET_AT_INDEX: ERigVMOpCode = ERigVMOpCode(86);
    pub const ARRAY_SET_AT_INDEX: ERigVMOpCode = ERigVMOpCode(87);
    pub const ARRAY_ADD: ERigVMOpCode = ERigVMOpCode(88);
    pub const ARRAY_INSERT: ERigVMOpCode = ERigVMOpCode(89);
    pub const ARRAY_REMOVE: ERigVMOpCode = ERigVMOpCode(90);
    pub const ARRAY_FIND: ERigVMOpCode = ERigVMOpCode(91);
    pub const ARRAY_APPEND: ERigVMOpCode = ERigVMOpCode(92);
    pub const ARRAY_CLONE: ERigVMOpCode = ERigVMOpCode(93);
    pub const ARRAY_ITERATOR: ERigVMOpCode = ERigVMOpCode(94);
    pub const ARRAY_UNION: ERigVMOpCode = ERigVMOpCode(95);
    pub const ARRAY_DIFFERENCE: ERigVMOpCode = ERigVMOpCode(96);
    pub const ARRAY_INTERSECTION: ERigVMOpCode = ERigVMOpCode(97);
    pub const ARRAY_REVERSE: ERigVMOpCode = ERigVMOpCode(98);
    pub const INVOKE_ENTRY: ERigVMOpCode = ERigVMOpCode(99);
    pub const JUMP_TO_BRANCH: ERigVMOpCode = ERigVMOpCode(100);
    pub const EXECUTE: ERigVMOpCode = ERigVMOpCode(101);
    pub const RUN_INSTRUCTIONS: ERigVMOpCode = ERigVMOpCode(102);
    pub const SETUP_TRAITS: ERigVMOpCode = ERigVMOpCode(103);
    pub const INVALID: ERigVMOpCode = ERigVMOpCode(104);
    pub const FIRST_ARRAY_OP_CODE: ERigVMOpCode = ERigVMOpCode(83);
    pub const LAST_ARRAY_OP_CODE: ERigVMOpCode = ERigVMOpCode(98);
}
#[repr(transparent)]
pub struct ERigVMMemoryType(pub u8);
impl ERigVMMemoryType {
    pub const WORK: ERigVMMemoryType = ERigVMMemoryType(0);
    pub const LITERAL: ERigVMMemoryType = ERigVMMemoryType(1);
    pub const EXTERNAL: ERigVMMemoryType = ERigVMMemoryType(2);
    pub const DEBUG: ERigVMMemoryType = ERigVMMemoryType(3);
    pub const INVALID: ERigVMMemoryType = ERigVMMemoryType(4);
}
#[repr(transparent)]
pub struct ERigVMPinDirection(pub u8);
impl ERigVMPinDirection {
    pub const INPUT: ERigVMPinDirection = ERigVMPinDirection(0);
    pub const OUTPUT: ERigVMPinDirection = ERigVMPinDirection(1);
    pub const IO: ERigVMPinDirection = ERigVMPinDirection(2);
    pub const VISIBLE: ERigVMPinDirection = ERigVMPinDirection(3);
    pub const HIDDEN: ERigVMPinDirection = ERigVMPinDirection(4);
    pub const INVALID: ERigVMPinDirection = ERigVMPinDirection(5);
}
#[repr(transparent)]
pub struct ERigVMRegisterType(pub u8);
impl ERigVMRegisterType {
    pub const PLAIN: ERigVMRegisterType = ERigVMRegisterType(0);
    pub const STRING: ERigVMRegisterType = ERigVMRegisterType(1);
    pub const NAME: ERigVMRegisterType = ERigVMRegisterType(2);
    pub const STRUCT: ERigVMRegisterType = ERigVMRegisterType(3);
    pub const INVALID: ERigVMRegisterType = ERigVMRegisterType(4);
}
#[repr(transparent)]
pub struct ERigVMAnimEasingType(pub u8);
impl ERigVMAnimEasingType {
    pub const LINEAR: ERigVMAnimEasingType = ERigVMAnimEasingType(0);
    pub const QUADRATIC_EASE_IN: ERigVMAnimEasingType = ERigVMAnimEasingType(1);
    pub const QUADRATIC_EASE_OUT: ERigVMAnimEasingType = ERigVMAnimEasingType(2);
    pub const QUADRATIC_EASE_IN_OUT: ERigVMAnimEasingType = ERigVMAnimEasingType(3);
    pub const CUBIC_EASE_IN: ERigVMAnimEasingType = ERigVMAnimEasingType(4);
    pub const CUBIC_EASE_OUT: ERigVMAnimEasingType = ERigVMAnimEasingType(5);
    pub const CUBIC_EASE_IN_OUT: ERigVMAnimEasingType = ERigVMAnimEasingType(6);
    pub const QUARTIC_EASE_IN: ERigVMAnimEasingType = ERigVMAnimEasingType(7);
    pub const QUARTIC_EASE_OUT: ERigVMAnimEasingType = ERigVMAnimEasingType(8);
    pub const QUARTIC_EASE_IN_OUT: ERigVMAnimEasingType = ERigVMAnimEasingType(9);
    pub const QUINTIC_EASE_IN: ERigVMAnimEasingType = ERigVMAnimEasingType(10);
    pub const QUINTIC_EASE_OUT: ERigVMAnimEasingType = ERigVMAnimEasingType(11);
    pub const QUINTIC_EASE_IN_OUT: ERigVMAnimEasingType = ERigVMAnimEasingType(12);
    pub const SINE_EASE_IN: ERigVMAnimEasingType = ERigVMAnimEasingType(13);
    pub const SINE_EASE_OUT: ERigVMAnimEasingType = ERigVMAnimEasingType(14);
    pub const SINE_EASE_IN_OUT: ERigVMAnimEasingType = ERigVMAnimEasingType(15);
    pub const CIRCULAR_EASE_IN: ERigVMAnimEasingType = ERigVMAnimEasingType(16);
    pub const CIRCULAR_EASE_OUT: ERigVMAnimEasingType = ERigVMAnimEasingType(17);
    pub const CIRCULAR_EASE_IN_OUT: ERigVMAnimEasingType = ERigVMAnimEasingType(18);
    pub const EXPONENTIAL_EASE_IN: ERigVMAnimEasingType = ERigVMAnimEasingType(19);
    pub const EXPONENTIAL_EASE_OUT: ERigVMAnimEasingType = ERigVMAnimEasingType(20);
    pub const EXPONENTIAL_EASE_IN_OUT: ERigVMAnimEasingType = ERigVMAnimEasingType(21);
    pub const ELASTIC_EASE_IN: ERigVMAnimEasingType = ERigVMAnimEasingType(22);
    pub const ELASTIC_EASE_OUT: ERigVMAnimEasingType = ERigVMAnimEasingType(23);
    pub const ELASTIC_EASE_IN_OUT: ERigVMAnimEasingType = ERigVMAnimEasingType(24);
    pub const BACK_EASE_IN: ERigVMAnimEasingType = ERigVMAnimEasingType(25);
    pub const BACK_EASE_OUT: ERigVMAnimEasingType = ERigVMAnimEasingType(26);
    pub const BACK_EASE_IN_OUT: ERigVMAnimEasingType = ERigVMAnimEasingType(27);
    pub const BOUNCE_EASE_IN: ERigVMAnimEasingType = ERigVMAnimEasingType(28);
    pub const BOUNCE_EASE_OUT: ERigVMAnimEasingType = ERigVMAnimEasingType(29);
    pub const BOUNCE_EASE_IN_OUT: ERigVMAnimEasingType = ERigVMAnimEasingType(30);
}
#[repr(transparent)]
pub struct ERigUnitDebugPointMode(pub u8);
impl ERigUnitDebugPointMode {
    pub const POINT: ERigUnitDebugPointMode = ERigUnitDebugPointMode(0);
    pub const VECTOR: ERigUnitDebugPointMode = ERigUnitDebugPointMode(1);
    pub const MAX: ERigUnitDebugPointMode = ERigUnitDebugPointMode(2);
}
#[repr(transparent)]
pub struct ERigUnitDebugTransformMode(pub u8);
impl ERigUnitDebugTransformMode {
    pub const POINT: ERigUnitDebugTransformMode = ERigUnitDebugTransformMode(0);
    pub const AXES: ERigUnitDebugTransformMode = ERigUnitDebugTransformMode(1);
    pub const BOX: ERigUnitDebugTransformMode = ERigUnitDebugTransformMode(2);
    pub const MAX: ERigUnitDebugTransformMode = ERigUnitDebugTransformMode(3);
}
#[repr(transparent)]
pub struct ERigUnitVisualDebugPointMode(pub u8);
impl ERigUnitVisualDebugPointMode {
    pub const POINT: ERigUnitVisualDebugPointMode = ERigUnitVisualDebugPointMode(0);
    pub const VECTOR: ERigUnitVisualDebugPointMode = ERigUnitVisualDebugPointMode(1);
    pub const MAX: ERigUnitVisualDebugPointMode = ERigUnitVisualDebugPointMode(2);
}
#[repr(transparent)]
pub struct ERBFQuatDistanceType(pub u8);
impl ERBFQuatDistanceType {
    pub const EUCLIDEAN: ERBFQuatDistanceType = ERBFQuatDistanceType(0);
    pub const ARC_LENGTH: ERBFQuatDistanceType = ERBFQuatDistanceType(1);
    pub const SWING_ANGLE: ERBFQuatDistanceType = ERBFQuatDistanceType(2);
    pub const TWIST_ANGLE: ERBFQuatDistanceType = ERBFQuatDistanceType(3);
}
#[repr(transparent)]
pub struct ERBFKernelType(pub u8);
impl ERBFKernelType {
    pub const GAUSSIAN: ERBFKernelType = ERBFKernelType(0);
    pub const EXPONENTIAL: ERBFKernelType = ERBFKernelType(1);
    pub const LINEAR: ERBFKernelType = ERBFKernelType(2);
    pub const CUBIC: ERBFKernelType = ERBFKernelType(3);
    pub const QUINTIC: ERBFKernelType = ERBFKernelType(4);
}
#[repr(transparent)]
pub struct ERBFVectorDistanceType(pub u8);
impl ERBFVectorDistanceType {
    pub const EUCLIDEAN: ERBFVectorDistanceType = ERBFVectorDistanceType(0);
    pub const MANHATTAN: ERBFVectorDistanceType = ERBFVectorDistanceType(1);
    pub const ARC_LENGTH: ERBFVectorDistanceType = ERBFVectorDistanceType(2);
}
#[repr(transparent)]
pub struct ERigVMTransformSpace(pub u8);
impl ERigVMTransformSpace {
    pub const LOCAL_SPACE: ERigVMTransformSpace = ERigVMTransformSpace(0);
    pub const GLOBAL_SPACE: ERigVMTransformSpace = ERigVMTransformSpace(1);
    pub const MAX: ERigVMTransformSpace = ERigVMTransformSpace(2);
}
#[repr(transparent)]
pub struct ERigVMClampSpatialMode(pub u8);
impl ERigVMClampSpatialMode {
    pub const PLANE: ERigVMClampSpatialMode = ERigVMClampSpatialMode(0);
    pub const CYLINDER: ERigVMClampSpatialMode = ERigVMClampSpatialMode(1);
    pub const SPHERE: ERigVMClampSpatialMode = ERigVMClampSpatialMode(2);
    pub const CAPSULE: ERigVMClampSpatialMode = ERigVMClampSpatialMode(3);
}
#[repr(transparent)]
pub struct ERigVMSimPointIntegrateType(pub u8);
impl ERigVMSimPointIntegrateType {
    pub const VERLET: ERigVMSimPointIntegrateType = ERigVMSimPointIntegrateType(0);
    pub const SEMI_EXPLICIT_EULER: ERigVMSimPointIntegrateType = ERigVMSimPointIntegrateType(
        1,
    );
}
