#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FSimulationStageMetaData {
    pub simulation_stage_name: FName,
    pub enabled_binding: FName,
    pub element_count: crate::bindings::core_u_object::FIntVector,
    pub element_count_x_binding: FName,
    pub element_count_y_binding: FName,
    pub element_count_z_binding: FName,
    pub iteration_source_type: crate::bindings::niagara_core::ENiagaraIterationSource,
    pub iteration_data_interface: FName,
    pub iteration_direct_binding: FName,
    pub execute_behavior: ENiagaraSimStageExecuteBehavior,
    pub flags_104: u8,
    pub particle_iteration_state_binding: FName,
    pub particle_iteration_state_range: crate::bindings::core_u_object::FIntPoint,
    pub output_destinations: TArray<FName>,
    pub input_data_interfaces: TArray<FName>,
    pub num_iterations: i32,
    pub num_iterations_binding: FName,
    pub gpu_dispatch_type: ENiagaraGpuDispatchType,
    pub gpu_direct_dispatch_element_type: ENiagaraDirectDispatchElementType,
    pub gpu_dispatch_num_threads: crate::bindings::core_u_object::FIntVector,
}
#[repr(C, align(8))]
pub struct FNiagaraCompileEvent {
    pub severity: FNiagaraCompileEventSeverity,
    pub message: FString,
    pub short_description: FString,
    pub node_guid: crate::bindings::core_u_object::FGuid,
    pub pin_guid: crate::bindings::core_u_object::FGuid,
    pub stack_guids: TArray<crate::bindings::core_u_object::FGuid>,
    pub source: FNiagaraCompileEventSource,
}
#[repr(C, align(8))]
pub struct FNiagaraDataInterfaceGeneratedFunction {
    pub variadic_inputs: TArray<
        crate::bindings::niagara_core::FNiagaraVariableCommonReference,
    >,
    pub variadic_outputs: TArray<
        crate::bindings::niagara_core::FNiagaraVariableCommonReference,
    >,
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENiagaraSimStageExecuteBehavior(pub u8);
impl ENiagaraSimStageExecuteBehavior {
    pub const ALWAYS: ENiagaraSimStageExecuteBehavior = ENiagaraSimStageExecuteBehavior(
        0,
    );
    pub const ON_SIMULATION_RESET: ENiagaraSimStageExecuteBehavior = ENiagaraSimStageExecuteBehavior(
        1,
    );
    pub const NOT_ON_SIMULATION_RESET: ENiagaraSimStageExecuteBehavior = ENiagaraSimStageExecuteBehavior(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENiagaraGpuDispatchType(pub u8);
impl ENiagaraGpuDispatchType {
    pub const ONE_D: ENiagaraGpuDispatchType = ENiagaraGpuDispatchType(0);
    pub const TWO_D: ENiagaraGpuDispatchType = ENiagaraGpuDispatchType(1);
    pub const THREE_D: ENiagaraGpuDispatchType = ENiagaraGpuDispatchType(2);
    pub const CUSTOM: ENiagaraGpuDispatchType = ENiagaraGpuDispatchType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENiagaraDirectDispatchElementType(pub u8);
impl ENiagaraDirectDispatchElementType {
    pub const NUM_THREADS: ENiagaraDirectDispatchElementType = ENiagaraDirectDispatchElementType(
        0,
    );
    pub const NUM_THREADS_NO_CLIPPING: ENiagaraDirectDispatchElementType = ENiagaraDirectDispatchElementType(
        1,
    );
    pub const NUM_GROUPS: ENiagaraDirectDispatchElementType = ENiagaraDirectDispatchElementType(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct FNiagaraCompileEventSeverity(pub u8);
impl FNiagaraCompileEventSeverity {
    pub const LOG: FNiagaraCompileEventSeverity = FNiagaraCompileEventSeverity(0);
    pub const DISPLAY: FNiagaraCompileEventSeverity = FNiagaraCompileEventSeverity(1);
    pub const WARNING: FNiagaraCompileEventSeverity = FNiagaraCompileEventSeverity(2);
    pub const ERROR: FNiagaraCompileEventSeverity = FNiagaraCompileEventSeverity(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct FNiagaraCompileEventSource(pub u8);
impl FNiagaraCompileEventSource {
    pub const UNSET: FNiagaraCompileEventSource = FNiagaraCompileEventSource(0);
    pub const SCRIPT_DEPENDENCY: FNiagaraCompileEventSource = FNiagaraCompileEventSource(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENiagaraMipMapGenerationType(pub u8);
impl ENiagaraMipMapGenerationType {
    pub const UNFILTERED: ENiagaraMipMapGenerationType = ENiagaraMipMapGenerationType(0);
    pub const LINEAR: ENiagaraMipMapGenerationType = ENiagaraMipMapGenerationType(1);
    pub const BLUR1: ENiagaraMipMapGenerationType = ENiagaraMipMapGenerationType(2);
    pub const BLUR2: ENiagaraMipMapGenerationType = ENiagaraMipMapGenerationType(3);
    pub const BLUR3: ENiagaraMipMapGenerationType = ENiagaraMipMapGenerationType(4);
    pub const BLUR4: ENiagaraMipMapGenerationType = ENiagaraMipMapGenerationType(5);
}
