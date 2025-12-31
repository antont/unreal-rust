#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
    pub group_size: crate::bindings::core_u_object::FIntVector,
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
    pub thread_count: crate::bindings::core_u_object::FUintVector,
}
pub struct UDispatchDataProvider {}
pub struct UComputeGraph {
    pub kernel_invocations: TArray<UPtr<UComputeKernel>>,
    pub data_interfaces: TArray<UPtr<UComputeDataInterface>>,
    pub graph_edges: TArray<FComputeGraphEdge>,
    pub bindings: TArray<TSubclassOf<crate::bindings::core_u_object::UObject>>,
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EShaderFundamentalType(pub u8);
impl EShaderFundamentalType {
    pub const BOOL: EShaderFundamentalType = EShaderFundamentalType(0);
    pub const INT: EShaderFundamentalType = EShaderFundamentalType(1);
    pub const UINT: EShaderFundamentalType = EShaderFundamentalType(2);
    pub const FLOAT: EShaderFundamentalType = EShaderFundamentalType(3);
    pub const STRUCT: EShaderFundamentalType = EShaderFundamentalType(4);
    pub const NONE: EShaderFundamentalType = EShaderFundamentalType(255);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EShaderFundamentalDimensionType(pub u8);
impl EShaderFundamentalDimensionType {
    pub const SCALAR: EShaderFundamentalDimensionType = EShaderFundamentalDimensionType(
        0,
    );
    pub const VECTOR: EShaderFundamentalDimensionType = EShaderFundamentalDimensionType(
        1,
    );
    pub const MATRIX: EShaderFundamentalDimensionType = EShaderFundamentalDimensionType(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EShaderParamBindingType(pub u8);
impl EShaderParamBindingType {
    pub const NONE: EShaderParamBindingType = EShaderParamBindingType(0);
    pub const CONSTANT_PARAMETER: EShaderParamBindingType = EShaderParamBindingType(1);
    pub const READ_ONLY_RESOURCE: EShaderParamBindingType = EShaderParamBindingType(2);
    pub const READ_WRITE_RESOURCE: EShaderParamBindingType = EShaderParamBindingType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EShaderResourceType(pub u8);
impl EShaderResourceType {
    pub const NONE: EShaderResourceType = EShaderResourceType(0);
    pub const TEXTURE1_D: EShaderResourceType = EShaderResourceType(1);
    pub const TEXTURE2_D: EShaderResourceType = EShaderResourceType(2);
    pub const TEXTURE3_D: EShaderResourceType = EShaderResourceType(3);
    pub const TEXTURE_CUBE: EShaderResourceType = EShaderResourceType(4);
    pub const BUFFER: EShaderResourceType = EShaderResourceType(5);
    pub const STRUCTURED_BUFFER: EShaderResourceType = EShaderResourceType(6);
    pub const BYTE_ADDRESS_BUFFER: EShaderResourceType = EShaderResourceType(7);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EShaderParamModifier(pub u8);
impl EShaderParamModifier {
    pub const NONE: EShaderParamModifier = EShaderParamModifier(0);
    pub const IN: EShaderParamModifier = EShaderParamModifier(1);
    pub const OUT: EShaderParamModifier = EShaderParamModifier(2);
    pub const IN_OUT: EShaderParamModifier = EShaderParamModifier(3);
}
