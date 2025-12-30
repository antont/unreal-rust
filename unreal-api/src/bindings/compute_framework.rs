#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FComputeGraphEdge {
    pub kernel_index: i32,
    pub kernel_binding_index: i32,
    pub data_interface_index: i32,
    pub data_interface_binding_index: i32,
    pub b_kernel_input: bool,
    pub binding_function_name_override: FString,
    pub binding_function_namespace: FString,
}
#[repr(C, align(8))]
pub struct FComputeGraphInstance {
    pub data_providers: TArray<UPtr<UComputeDataProvider>>,
}
#[repr(C, align(8))]
pub struct FComputeKernelPermutationBool {
    pub name: FString,
    pub value: bool,
}
#[repr(C, align(8))]
pub struct FComputeKernelPermutationSet {
    pub boolean_options: TArray<FComputeKernelPermutationBool>,
}
#[repr(C, align(8))]
pub struct FComputeKernelDefinition {
    pub symbol: FString,
    pub define: FString,
}
#[repr(C, align(8))]
pub struct FComputeKernelDefinitionSet {
    pub defines: TArray<FComputeKernelDefinition>,
}
#[repr(C, align(8))]
pub struct FComputeKernelPermutationVector {
    pub permutations: TMap<FString, u32>,
    pub bit_count: u32,
}
#[repr(C, align(8))]
pub struct FShaderValueTypeHandle {}
#[repr(C, align(8))]
pub struct FArrayShaderValue {
    pub array_of_values: TArray<u8>,
}
#[repr(C, align(8))]
pub struct FShaderValueContainer {
    pub shader_value: TArray<u8>,
    pub array_list: TArray<FArrayShaderValue>,
}
#[repr(C, align(8))]
pub struct FShaderValueType {
    pub ty: EShaderFundamentalType,
    pub dimension_type: EShaderFundamentalDimensionType,
    pub name: FName,
    pub b_is_dynamic_array: bool,
}
#[repr(C, align(8))]
pub struct FShaderParamTypeDefinition {
    pub type_declaration: FString,
    pub name: FString,
    pub value_type: FShaderValueTypeHandle,
    pub array_element_count: u16,
    pub binding_type: EShaderParamBindingType,
    pub resource_type: EShaderResourceType,
    pub modifier: EShaderParamModifier,
}
#[repr(C, align(8))]
pub struct FShaderFunctionDefinition {
    pub name: FString,
    pub param_types: TArray<FShaderParamTypeDefinition>,
    pub b_has_return_type: bool,
}
pub struct UComputeDataInterface {}
pub struct UComputeKernelSource {
    pub entry_point: FString,
    pub group_size: FIntVector,
    pub permutation_set: FComputeKernelPermutationSet,
    pub definitions_set: FComputeKernelDefinitionSet,
    pub additional_sources: TArray<UPtr<UComputeSource>>,
    pub external_inputs: TArray<FShaderFunctionDefinition>,
    pub external_outputs: TArray<FShaderFunctionDefinition>,
}
pub struct UComputeKernelSourceWithText {
    pub source_text: FString,
}
pub struct UComputeSource {
    pub additional_sources: TArray<UPtr<UComputeSource>>,
}
pub struct UComputeDataInterfaceBuffer {
    pub value_type: FShaderValueTypeHandle,
    pub element_count: i32,
    pub b_allow_read_write: bool,
    pub b_clear_before_use: bool,
}
pub struct UComputeDataProvider {}
pub struct UBufferDataProvider {}
pub struct UComputeDataInterfaceDispatch {
    pub thread_count: FUintVector,
}
pub struct UDispatchDataProvider {}
pub struct UComputeGraph {
    pub kernel_invocations: TArray<UPtr<UComputeKernel>>,
    pub data_interfaces: TArray<UPtr<UComputeDataInterface>>,
    pub graph_edges: TArray<FComputeGraphEdge>,
    pub bindings: TArray<TSubclassOf<UObject>>,
    pub data_interface_to_binding: TArray<i32>,
}
pub struct UComputeGraphComponent {
    pub compute_graph: UPtr<UComputeGraph>,
    pub compute_graph_instance: FComputeGraphInstance,
}
pub struct UComputeGraphFromText {
    pub graph_source_text: FString,
}
pub struct UComputeKernel {
    pub kernel_source: UPtr<UComputeKernelSource>,
    pub kernel_flags: i32,
}
