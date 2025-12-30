#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FSimulationStageMetaData {
    pub simulation_stage_name: FName,
    pub enabled_binding: FName,
    pub element_count: FIntVector,
    pub element_count_x_binding: FName,
    pub element_count_y_binding: FName,
    pub element_count_z_binding: FName,
    pub iteration_source_type: ENiagaraIterationSource,
    pub iteration_data_interface: FName,
    pub iteration_direct_binding: FName,
    pub execute_behavior: ENiagaraSimStageExecuteBehavior,
    pub flags_104: u8,
    pub particle_iteration_state_binding: FName,
    pub particle_iteration_state_range: FIntPoint,
    pub output_destinations: TArray<FName>,
    pub input_data_interfaces: TArray<FName>,
    pub num_iterations: i32,
    pub num_iterations_binding: FName,
    pub gpu_dispatch_type: ENiagaraGpuDispatchType,
    pub gpu_direct_dispatch_element_type: ENiagaraDirectDispatchElementType,
    pub gpu_dispatch_num_threads: FIntVector,
}
#[repr(C, align(8))]
pub struct FNiagaraCompileEvent {
    pub severity: FNiagaraCompileEventSeverity,
    pub message: FString,
    pub short_description: FString,
    pub node_guid: FGuid,
    pub pin_guid: FGuid,
    pub stack_guids: TArray<FGuid>,
    pub source: FNiagaraCompileEventSource,
}
#[repr(C, align(8))]
pub struct FNiagaraDataInterfaceGeneratedFunction {
    pub variadic_inputs: TArray<FNiagaraVariableCommonReference>,
    pub variadic_outputs: TArray<FNiagaraVariableCommonReference>,
}
#[repr(C, align(8))]
pub struct FNiagaraDataInterfaceGPUParamInfo {
    pub data_interface_hlsl_symbol: FString,
    pub di_class_name: FString,
    pub shader_parameters_offset: u32,
    pub generated_functions: TArray<FNiagaraDataInterfaceGeneratedFunction>,
}
#[repr(C, align(8))]
pub struct FNiagaraShaderScriptExternalConstant {
    pub ty: FName,
    pub name: FString,
}
#[repr(C, align(8))]
pub struct FNiagaraShaderScriptParametersMetadata {
    pub data_interface_param_info: TArray<FNiagaraDataInterfaceGPUParamInfo>,
    pub loose_metadata_names: TArray<FString>,
    pub b_external_constants_interpolated: bool,
    pub external_constants: TArray<FNiagaraShaderScriptExternalConstant>,
}
pub struct UNiagaraScriptBase {}
