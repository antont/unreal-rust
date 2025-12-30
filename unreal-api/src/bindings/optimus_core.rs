#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FOptimus_ShaderBinding {
    pub name: FName,
    pub data_type: FOptimusDataTypeRef,
}
#[repr(C, align(8))]
pub struct FOptimusDataTypeRef {
    pub type_name: FName,
    pub type_object: TSoftObjectPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FOptimusParameterBinding {
    pub name: FOptimusValidatedName,
    pub data_type: FOptimusDataTypeRef,
    pub data_domain: FOptimusDataDomain,
    pub b_support_atomic_if_compatible_data_type: bool,
    pub b_support_read: bool,
}
#[repr(C, align(8))]
pub struct FOptimusDataDomain {
    pub ty: EOptimusDataDomainType,
    pub dimension_names: TArray<FName>,
    pub multiplier: i32,
    pub expression: FString,
    pub level_names_deprecated: TArray<FName>,
}
#[repr(C, align(4))]
pub struct FOptimusValidatedName {
    pub name: FName,
}
#[repr(C, align(8))]
pub struct FOptimusParameterBindingArray {
    pub inner_array: TArray<FOptimusParameterBinding>,
}
#[repr(C, align(4))]
pub struct FOptimusFunctionNodeGraphHeaderWithGuid {
    pub function_graph_guid: FGuid,
    pub function_name: FName,
    pub category: FName,
}
#[repr(C, align(8))]
pub struct FOptimusFunctionNodeGraphHeaderWithGuidArray {
    pub headers: TArray<FOptimusFunctionNodeGraphHeaderWithGuid>,
}
#[repr(C, align(8))]
pub struct FOptimusShaderText {
    pub declarations: FString,
    pub shader_text: FString,
}
#[repr(C, align(4))]
pub struct FOptimusValueIdentifier {
    pub ty: EOptimusValueType,
    pub name: FName,
}
#[repr(C, align(8))]
pub struct FOptimusValueDescription {
    pub data_type: FOptimusDataTypeRef,
    pub value_usage: EOptimusValueUsage,
    pub value: FOptimusValueContainerStruct,
    pub shader_value: FShaderValueContainer,
}
#[repr(C, align(8))]
pub struct FOptimusValueContainerStruct {
    pub value: FInstancedPropertyBag,
}
#[repr(C, align(8))]
pub struct FOptimusDataInterfacePropertyOverrideInfo {
    pub pin_name_to_value_id_map: TMap<FName, FOptimusValueIdentifier>,
}
#[repr(C, align(8))]
pub struct FOptimusAction {}
#[repr(C, align(8))]
pub struct FOptimusCompoundAction {}
#[repr(C, align(8))]
pub struct FOptimusComponentBindingAction_AddBinding {}
#[repr(C, align(8))]
pub struct FOptimusComponentBindingAction_RemoveBinding {}
#[repr(C, align(8))]
pub struct FOptimusComponentBindingAction_RenameBinding {}
#[repr(C, align(8))]
pub struct FOptimusComponentBindingAction_SetComponentSource {}
#[repr(C, align(8))]
pub struct FOptimusNodeAction_RenameNode {}
#[repr(C, align(8))]
pub struct FOptimusNodeAction_MoveNode {}
#[repr(C, align(8))]
pub struct FOptimusCommentNodeAction_ResizeNode {}
#[repr(C, align(8))]
pub struct FOptimusNodeAction_SetPinValue {}
#[repr(C, align(8))]
pub struct FOptimusNodeAction_SetPinName {}
#[repr(C, align(8))]
pub struct FOptimusNodeAction_SetPinType {}
#[repr(C, align(8))]
pub struct FOptimusNodeAction_SetPinDataDomain {}
#[repr(C, align(8))]
pub struct FOptimusNodeAction_AddRemovePin {}
#[repr(C, align(8))]
pub struct FOptimusNodeAction_AddPin {}
#[repr(C, align(8))]
pub struct FOptimusNodeAction_AddGroupingPin {}
#[repr(C, align(8))]
pub struct FOptimusNodeAction_RemovePin {}
#[repr(C, align(8))]
pub struct FOptimusNodeAction_MovePin {}
#[repr(C, align(16))]
pub struct FOptimusNodeGraphAction_AddGraph {}
#[repr(C, align(8))]
pub struct FOptimusNodeGraphAction_RemoveGraph {}
#[repr(C, align(8))]
pub struct FOptimusNodeGraphAction_RenameGraph {}
#[repr(C, align(16))]
pub struct FOptimusNodeGraphAction_AddNode {}
#[repr(C, align(16))]
pub struct FOptimusNodeGraphAction_DuplicateNode {}
#[repr(C, align(8))]
pub struct FOptimusNodeGraphAction_RemoveNode {}
#[repr(C, align(8))]
pub struct FOptimusNodeGraphAction_AddRemoveNodePair {}
#[repr(C, align(8))]
pub struct FOptimusNodeGraphAction_AddNodePair {}
#[repr(C, align(8))]
pub struct FOptimusNodeGraphAction_RemoveNodePair {}
#[repr(C, align(8))]
pub struct FOptimusNodeGraphAction_AddRemoveLink {}
#[repr(C, align(8))]
pub struct FOptimusNodeGraphAction_AddLink {}
#[repr(C, align(8))]
pub struct FOptimusNodeGraphAction_RemoveLink {}
#[repr(C, align(8))]
pub struct FOptimusNodeGraphAction_ConnectAdderPin {}
#[repr(C, align(8))]
pub struct FOptimusNodeGraphAction_PackageKernelFunction {}
#[repr(C, align(8))]
pub struct FOptimusNodeGraphAction_UnpackageKernelFunction {}
#[repr(C, align(8))]
pub struct FOptimusResourceAction_AddResource {}
#[repr(C, align(8))]
pub struct FOptimusResourceAction_RemoveResource {}
#[repr(C, align(8))]
pub struct FOptimusResourceAction_RenameResource {}
#[repr(C, align(8))]
pub struct FOptimusResourceAction_SetDataType {}
#[repr(C, align(8))]
pub struct FOptimusResourceAction_SetDataDomain {}
#[repr(C, align(8))]
pub struct FOptimusVariableAction_AddVariable {}
#[repr(C, align(8))]
pub struct FOptimusVariableAction_RemoveVariable {}
#[repr(C, align(8))]
pub struct FOptimusVariableAction_RenameVariable {}
#[repr(C, align(8))]
pub struct FOptimusVariableAction_SetDataType {}
#[repr(C, align(8))]
pub struct FRigVMTrait_OptimusDeformer {
    pub deformer_graph: TSoftObjectPtr<UOptimusDeformer>,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_OptimusDeformerSettings {
    pub execution_phase: EOptimusDeformerExecutionPhase,
    pub execution_group: i32,
    pub deform_child_components: bool,
    pub exclude_child_components_with_tag: FName,
}
#[repr(C, align(8))]
pub struct FRigUnit_AddOptimusDeformer {
    pub deformer_instance_guid: FGuid,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_OptimusVariableBase {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerIntVariable {
    pub value: i32,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerIntArrayVariable {
    pub value: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt2Variable {
    pub value: FIntPoint,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt2ArrayVariable {
    pub value: TArray<FIntPoint>,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt3Variable {
    pub value: FIntVector,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt3ArrayVariable {
    pub value: TArray<FIntVector>,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt4Variable {
    pub value: FIntVector4,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt4ArrayVariable {
    pub value: TArray<FIntVector4>,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerFloatVariable {
    pub value: f64,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerFloatArrayVariable {
    pub value: TArray<f64>,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerVector2Variable {
    pub value: FVector2D,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerVector2ArrayVariable {
    pub value: TArray<FVector2D>,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerVectorVariable {
    pub value: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerVectorArrayVariable {
    pub value: TArray<FVector>,
}
#[repr(C, align(16))]
pub struct FRigVMTrait_SetDeformerVector4Variable {
    pub value: FVector4,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerVector4ArrayVariable {
    pub value: TArray<FVector4>,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerLinearColorVariable {
    pub value: FLinearColor,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerLinearColorArrayVariable {
    pub value: TArray<FLinearColor>,
}
#[repr(C, align(16))]
pub struct FRigVMTrait_SetDeformerQuatVariable {
    pub value: FQuat,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerQuatArrayVariable {
    pub value: TArray<FQuat>,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerRotatorVariable {
    pub value: FRotator,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerRotatorArrayVariable {
    pub value: TArray<FRotator>,
}
#[repr(C, align(16))]
pub struct FRigVMTrait_SetDeformerTransformVariable {
    pub value: FTransform,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerTransformArrayVariable {
    pub value: TArray<FTransform>,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerNameVariable {
    pub value: FName,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerNameArrayVariable {
    pub value: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerBoolVariable {
    pub value: bool,
}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerBoolArrayVariable {
    pub value: TArray<bool>,
}
#[repr(C, align(8))]
pub struct FOptimusAnimAttributeBufferDescription {
    pub name: FString,
    pub data_type: FOptimusDataTypeRef,
    pub default_value_struct: FOptimusValueContainerStruct,
    pub hlsl_id: FString,
    pub pin_name: FName,
}
#[repr(C, align(8))]
pub struct FOptimusAnimAttributeBufferArray {
    pub inner_array: TArray<FOptimusAnimAttributeBufferDescription>,
}
#[repr(C, align(8))]
pub struct FOptimusAnimAttributeDescription {
    pub name: FString,
    pub bone_name: FName,
    pub data_type: FOptimusDataTypeRef,
    pub default_value_struct: FOptimusValueContainerStruct,
    pub hlsl_id: FString,
    pub pin_name: FName,
    pub default_value_deprecated: UPtr<UOptimusValueContainer>,
}
#[repr(C, align(8))]
pub struct FOptimusAnimAttributeArray {
    pub inner_array: TArray<FOptimusAnimAttributeDescription>,
}
#[repr(C, align(4))]
pub struct FOptimusDebugDrawParameters {
    pub b_force_enable: bool,
    pub max_line_count: i32,
    pub max_triangle_count: i32,
    pub max_character_count: i32,
    pub font_size: i32,
}
#[repr(C, align(8))]
pub struct FOptimusGraphVariableDescription {
    pub name: FString,
    pub value_type: FShaderValueTypeHandle,
    pub value_id: FOptimusValueIdentifier,
    pub offset: i32,
    pub value_deprecated: TArray<u8>,
    pub shader_value_deprecated: FShaderValueContainer,
    pub source_object_deprecated: TSoftObjectPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FOptimusNode_ComponentSource_DuplicationInfo {
    pub binding_name: FName,
    pub component_type: TSubclassOf<UOptimusComponentSource>,
}
#[repr(C, align(8))]
pub struct FOptimusSecondaryInputBindingsGroup {
    pub group_name: FOptimusValidatedName,
    pub binding_array: FOptimusParameterBindingArray,
}
#[repr(C, align(8))]
pub struct FOptimusNode_GetVariable_DuplicationInfo {
    pub variable_name: FName,
    pub data_type: FOptimusDataTypeRef,
    pub default_value: FString,
}
#[repr(C, align(8))]
pub struct FOptimusPinPairInfo {
    pub input_pin_path: TArray<FName>,
    pub output_pin_path: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FOptimusLoopTerminalInfo {
    pub count: i32,
    pub bindings: FOptimusParameterBindingArray,
}
#[repr(C, align(8))]
pub struct FOptimusNode_ResourceAccessorBase_DuplicationInfo {
    pub resource_name: FName,
    pub data_type: FOptimusDataTypeRef,
    pub data_domain: FOptimusDataDomain,
}
#[repr(C, align(4))]
pub struct FOptimusConstantIdentifier {
    pub node_path: FName,
    pub group_name: FName,
    pub constant_name: FName,
}
#[repr(C, align(8))]
pub struct FOptimusConstantDefinition {
    pub referenced_constant: FOptimusConstantIdentifier,
    pub expression: FString,
}
#[repr(C, align(8))]
pub struct FOptimusConstant {
    pub identifier: FOptimusConstantIdentifier,
    pub definition: FOptimusConstantDefinition,
    pub component_binding_index: i32,
    pub ty: EOptimusConstantType,
}
#[repr(C, align(4))]
pub struct FOptimusConstantIndex {
    pub kernel_index: i32,
    pub ty: EOptimusConstantType,
    pub constant_index: i32,
}
#[repr(C, align(8))]
pub struct FOptimusKernelConstantContainer {
    pub input_constants: TArray<FOptimusConstant>,
    pub output_constants: TArray<FOptimusConstant>,
    pub group_name_to_binding_index: TMap<FName, i32>,
}
#[repr(C, align(8))]
pub struct FOptimusConstantContainer {
    pub kernel_containers: TArray<FOptimusKernelConstantContainer>,
}
#[repr(C, align(8))]
pub struct FOptimusDataType {
    pub type_name: FName,
    pub display_name: FText,
    pub shader_value_type: FShaderValueTypeHandle,
    pub shader_value_size: i32,
    pub type_category: FName,
    pub type_object: TWeakObjectPtr<UObject>,
    pub b_has_custom_pin_color: bool,
    pub custom_pin_color: FLinearColor,
    pub usage_flags: EOptimusDataTypeUsageFlags,
    pub type_flags: EOptimusDataTypeFlags,
}
#[repr(C, align(8))]
pub struct FOptimusComputeGraphInfo {
    pub graph_type: EOptimusNodeGraphType,
    pub graph_name: FName,
    pub compute_graph: UPtr<UOptimusComputeGraph>,
}
#[repr(C, align(8))]
pub struct FOptimusDeformerInstanceExecInfo {
    pub graph_name: FName,
    pub graph_type: EOptimusNodeGraphType,
    pub compute_graph: UPtr<UComputeGraph>,
    pub compute_graph_instance: FComputeGraphInstance,
}
#[repr(C, align(4))]
pub struct FOptimusDeformerInstanceComponentBinding {
    pub provider_name: FName,
    pub component_name: FName,
}
#[repr(C, align(8))]
pub struct FOptimusExecutionDomain {
    pub ty: EOptimusExecutionDomainType,
    pub name: FName,
    pub expression: FString,
}
#[repr(C, align(8))]
pub struct FOptimusFunctionGraphIdentifier {
    pub asset: UPtr<UOptimusDeformer>,
    pub guid: FGuid,
}
#[repr(C, align(8))]
pub struct FOptimusFunctionNodeGraphHeader {
    pub graph_path: TSoftObjectPtr<UOptimusFunctionNodeGraph>,
    pub function_name: FName,
    pub category: FName,
}
#[repr(C, align(8))]
pub struct FOptimusFunctionNodeGraphHeaderArray {
    pub headers: TArray<FOptimusFunctionNodeGraphHeader>,
}
#[repr(C, align(8))]
pub struct FOptimusFunctionReferenceNodeSet {
    pub nodes: TSet<TSoftObjectPtr<UOptimusNode_FunctionReference>>,
}
#[repr(C, align(8))]
pub struct FOptimusFunctionReferenceData {
    pub function_references: TMap<FSoftObjectPath, FOptimusFunctionReferenceNodeSet>,
}
#[repr(C, align(8))]
pub struct FOptimusVariableMetaDataEntry {
    pub key: FName,
    pub value: FString,
}
pub struct UOptimusAlternativeSelectedObjectProvider {}
pub struct IOptimusAlternativeSelectedObjectProvider {}
pub struct UOptimusComponentBindingProvider {}
pub struct IOptimusComponentBindingProvider {}
pub struct UOptimusComponentBindingReceiver {}
pub struct IOptimusComponentBindingReceiver {}
pub struct UOptimusComputeKernelDataInterface {}
pub struct IOptimusComputeKernelDataInterface {}
pub struct UOptimusComputeKernelProvider {}
pub struct IOptimusComputeKernelProvider {}
pub struct UOptimusDataInterfaceProvider {}
pub struct IOptimusDataInterfaceProvider {}
pub struct UOptimusDeformerAssetPathAccessor {}
pub struct IOptimusDeformerAssetPathAccessor {}
pub struct UOptimusDeformerGeometryReadbackProvider {}
pub struct IOptimusDeformerGeometryReadbackProvider {}
pub struct UOptimusDeformerInstanceAccessor {}
pub struct IOptimusDeformerInstanceAccessor {}
pub struct UOptimusDeprecatedExecutionDataInterface {}
pub struct IOptimusDeprecatedExecutionDataInterface {}
pub struct UOptimusExecutionDomainProvider {}
pub struct IOptimusExecutionDomainProvider {}
pub struct UOptimusGeneratedClassDefiner {}
pub struct IOptimusGeneratedClassDefiner {}
pub struct UOptimusNodeAdderPinProvider {}
pub struct IOptimusNodeAdderPinProvider {}
pub struct UOptimusNodeFunctionLibraryOwner {}
pub struct IOptimusNodeFunctionLibraryOwner {}
pub struct UOptimusNodeGraphCollectionOwner {}
pub struct IOptimusNodeGraphCollectionOwner {}
pub struct UOptimusNodeGraphProvider {}
pub struct IOptimusNodeGraphProvider {}
pub struct UOptimusNodePairProvider {}
pub struct IOptimusNodePairProvider {}
pub struct UOptimusNodePinRouter {}
pub struct IOptimusNodePinRouter {}
pub struct UOptimusNodeSubGraphReferencer {}
pub struct IOptimusNodeSubGraphReferencer {}
pub struct UOptimusNonCollapsibleNode {}
pub struct IOptimusNonCollapsibleNode {}
pub struct UOptimusNonCopyableNode {}
pub struct IOptimusNonCopyableNode {}
pub struct UOptimusOutputBufferWriter {}
pub struct IOptimusOutputBufferWriter {}
pub struct UOptimusParameterBindingProvider {}
pub struct IOptimusParameterBindingProvider {}
pub struct UOptimusPathResolver {}
pub struct IOptimusPathResolver {}
pub struct UOptimusPersistentBufferProvider {}
pub struct IOptimusPersistentBufferProvider {}
pub struct UOptimusPinMutabilityDefiner {}
pub struct IOptimusPinMutabilityDefiner {}
pub struct UOptimusPropertyPinProvider {}
pub struct IOptimusPropertyPinProvider {}
pub struct UOptimusShaderTextProvider {}
pub struct IOptimusShaderTextProvider {}
pub struct UOptimusUnnamedNodePinProvider {}
pub struct IOptimusUnnamedNodePinProvider {}
pub struct UOptimusValueProvider {}
pub struct IOptimusValueProvider {}
pub struct UOptimusKernelSource {
    pub source: FString,
}
pub struct UOptimusComponentSource {}
pub struct UOptimusSceneComponentSource {}
pub struct UOptimusSkinnedMeshComponentSource {}
pub struct UOptimusSkeletalMeshComponentSource {}
pub struct UOptimusComputeDataInterface {}
pub struct UOptimusAdvancedSkeletonDataInterface {
    pub skin_weight_profile: FName,
    pub b_enable_layered_skinning: bool,
    pub attribute_buffer_array: FOptimusAnimAttributeBufferArray,
}
pub struct UOptimusAdvancedSkeletonDataProvider {}
pub struct UOptimusAnimAttributeDataInterface {
    pub attribute_array: FOptimusAnimAttributeArray,
}
pub struct UOptimusAnimAttributeDataProvider {}
pub struct UOptimusClothDataInterface {}
pub struct UOptimusClothDataProvider {}
pub struct UOptimusConnectivityDataInterface {}
pub struct UOptimusConnectivityDataProvider {}
pub struct UOptimusCopyKernelDataInterface {
    pub component_source_binding: TWeakObjectPtr<UOptimusComponentSourceBinding>,
    pub num_threads_expression: FString,
}
pub struct UOptimusCopyKernelDataProvider {}
pub struct UOptimusCustomComputeKernelDataInterface {
    pub component_source_binding: TWeakObjectPtr<UOptimusComponentSourceBinding>,
    pub num_threads_expression: FString,
    pub execution_domain_constant_identifier_deprecated: FOptimusConstantIdentifier,
}
pub struct UOptimusCustomComputeKernelDataProvider {}
pub struct UOptimusDebugDrawDataInterface {
    pub b_is_supported: bool,
    pub debug_draw_parameters: FOptimusDebugDrawParameters,
}
pub struct UOptimusDebugDrawDataProvider {}
pub struct UOptimusDuplicateVerticesDataInterface {}
pub struct UOptimusDuplicateVerticesDataProvider {}
pub struct UOptimusGraphDataInterface {
    pub variables: TArray<FOptimusGraphVariableDescription>,
    pub parameter_buffer_size: i32,
}
pub struct UOptimusGraphDataProvider {
    pub variables: TArray<FOptimusGraphVariableDescription>,
}
pub struct UOptimusHalfEdgeDataInterface {}
pub struct UOptimusHalfEdgeDataProvider {}
pub struct UOptimusLoopTerminalDataInterface {
    pub index: u32,
    pub count: u32,
}
pub struct UOptimusLoopTerminalDataProvider {}
pub struct UOptimusMorphTargetDataInterface {}
pub struct UOptimusMorphTargetDataProvider {}
pub struct UOptimusRawBufferDataInterface {
    pub value_type: FShaderValueTypeHandle,
    pub data_domain: FOptimusDataDomain,
    pub component_source_binding: TWeakObjectPtr<UOptimusComponentSourceBinding>,
    pub domain_constant_identifier_deprecated: FOptimusConstantIdentifier,
}
pub struct UOptimusTransientBufferDataInterface {
    pub b_zero_init_for_atomic_writes: bool,
}
pub struct UOptimusImplicitPersistentBufferDataInterface {
    pub b_zero_init_for_atomic_writes: bool,
}
pub struct UOptimusPersistentBufferDataInterface {
    pub resource_name: FName,
}
pub struct UOptimusRawBufferDataProvider {}
pub struct UOptimusTransientBufferDataProvider {}
pub struct UOptimusImplicitPersistentBufferDataProvider {}
pub struct UOptimusPersistentBufferDataProvider {}
pub struct UOptimusSceneDataInterface {}
pub struct UOptimusSceneDataProvider {}
pub struct UOptimusSkeletonDataInterface {}
pub struct UOptimusSkeletonDataProvider {}
pub struct UOptimusSkinnedMeshDataInterface {}
pub struct UOptimusSkinnedMeshDataProvider {}
pub struct UDEPRECATED_OptimusSkinnedMeshExecDataInterface {
    pub domain: EOptimusSkinnedMeshExecDomain,
}
pub struct UOptimusSkinnedMeshReadDataInterface {}
pub struct UOptimusSkinnedMeshReadDataProvider {}
pub struct UOptimusSkinnedMeshVertexAttributeDataInterface {
    pub attribute_name: FName,
    pub default_value: f32,
}
pub struct UOptimusSkinnedMeshVertexAttributeDataProvider {
    pub attribute_name: FName,
    pub default_value: f32,
}
pub struct UOptimusSkinnedMeshWriteDataInterface {}
pub struct UOptimusSkinnedMeshWriteDataProvider {}
pub struct UOptimusSkinWeightsAsVertexMaskDataInterface {
    pub skin_weight_profile: FName,
    pub bone_names: TArray<FName>,
    pub expand_towards_root: i32,
    pub expand_towards_leaf: i32,
    pub b_debug_draw_included_bones: bool,
    pub debug_draw_color: FColor,
}
pub struct UOptimusSkinWeightsAsVertexMaskDataProvider {}
pub struct UOptimusNode {
    pub display_name: FText,
    pub graph_position: FVector2D,
    pub pins: TArray<UPtr<UOptimusNodePin>>,
    pub expanded_pins: TSet<FName>,
    pub diagnostic_level: EOptimusDiagnosticLevel,
}
pub struct UOptimusNode_DataInterface {
    pub data_interface_class: TSubclassOf<UObject>,
    pub data_interface_data: UPtr<UOptimusComputeDataInterface>,
}
pub struct UOptimusNode_AnimAttributeDataInterface {}
pub struct UOptimusNode_ComponentSource {
    pub binding: UPtr<UOptimusComponentSourceBinding>,
    pub duplication_info: FOptimusNode_ComponentSource_DuplicationInfo,
}
pub struct UOptimusNode_ComputeKernelBase {}
pub struct UOptimusNode_ComputeKernelFunctionGeneratorClass {
    pub category: FName,
    pub kernel_name: FName,
    pub execution_domain: FOptimusExecutionDomain,
    pub group_size: FIntVector,
    pub input_bindings: TArray<FOptimusParameterBinding>,
    pub output_bindings: TArray<FOptimusParameterBinding>,
    pub shader_source: FString,
}
pub struct UOptimusNode_ComputeKernelFunction {}
pub struct UOptimusNode_ConstantValueGeneratorClass {
    pub data_type: FOptimusDataTypeRef,
}
pub struct UOptimusNode_ConstantValue {}
pub struct UOptimusNode_CustomComputeKernel {
    pub category: FName,
    pub kernel_name: FOptimusValidatedName,
    pub execution_domain: FOptimusExecutionDomain,
    pub group_size: FIntVector,
    pub parameters_deprecated: TArray<FOptimus_ShaderBinding>,
    pub input_bindings_deprecated: TArray<FOptimusParameterBinding>,
    pub output_bindings_deprecated: TArray<FOptimusParameterBinding>,
    pub input_binding_array: FOptimusParameterBindingArray,
    pub output_binding_array: FOptimusParameterBindingArray,
    pub secondary_input_binding_groups: TArray<FOptimusSecondaryInputBindingsGroup>,
    pub additional_sources: TArray<UPtr<UComputeSource>>,
    pub shader_source: FOptimusShaderText,
}
pub struct UOptimusNode_FunctionReference {
    pub function_graph_identifier: FOptimusFunctionGraphIdentifier,
    pub default_component_pin: TWeakObjectPtr<UOptimusNodePin>,
    pub resolved_function_graph: TWeakObjectPtr<UOptimusFunctionNodeGraph>,
    pub function_graph_deprecated: TSoftObjectPtr<UOptimusFunctionNodeGraph>,
}
pub struct UOptimusNode_ResourceAccessorBase {
    pub resource_desc: TWeakObjectPtr<UOptimusResourceDescription>,
    pub write_type_deprecated: EOptimusBufferWriteType,
    pub duplication_info: FOptimusNode_ResourceAccessorBase_DuplicationInfo,
}
pub struct UOptimusNode_GetResource {}
pub struct UOptimusNode_GetVariable {
    pub variable_desc: TWeakObjectPtr<UOptimusVariableDescription>,
    pub duplication_info: FOptimusNode_GetVariable_DuplicationInfo,
}
pub struct UOptimusNode_GraphTerminal {
    pub terminal_type: EOptimusTerminalType,
    pub default_component_pin: TWeakObjectPtr<UOptimusNodePin>,
}
pub struct UOptimusNode_LoopTerminal {
    pub terminal_type: EOptimusTerminalType,
    pub loop_info: FOptimusLoopTerminalInfo,
    pub index_pin: UPtr<UOptimusNodePin>,
    pub count_pin: UPtr<UOptimusNodePin>,
    pub pin_pair_infos: TArray<FOptimusPinPairInfo>,
}
pub struct UOptimusNode_Resource {}
pub struct UOptimusNode_SetResource {}
pub struct UOptimusNode_SubGraphReference {
    pub sub_graph_name: FName,
    pub default_component_pin: TWeakObjectPtr<UOptimusNodePin>,
}
pub struct UOptimusActionStack {
    pub transacted_action_index: i32,
}
pub struct UOptimusComponentSourceBinding {
    pub binding_name: FName,
    pub component_type: TSubclassOf<UOptimusComponentSource>,
    pub component_tags: TArray<FName>,
    pub b_is_primary_binding: bool,
}
pub struct UOptimusComputeGraph {
    pub kernel_to_node: TArray<TSoftObjectPtr<UOptimusNode>>,
}
pub struct UOptimusComponentSourceBindingContainer {
    pub bindings: TArray<UPtr<UOptimusComponentSourceBinding>>,
}
pub struct UOptimusVariableContainer {
    pub descriptions: TArray<UPtr<UOptimusVariableDescription>>,
}
pub struct UOptimusResourceContainer {
    pub descriptions: TArray<UPtr<UOptimusResourceDescription>>,
}
pub struct UOptimusDeformer {
    pub mesh: UPtr<USkeletalMesh>,
    pub compute_graphs: TArray<FOptimusComputeGraphInfo>,
    pub data_interface_property_override_map: TMap<
        TWeakObjectPtr<UComputeDataInterface>,
        FOptimusDataInterfacePropertyOverrideInfo,
    >,
    pub value_map: TMap<FOptimusValueIdentifier, FOptimusValueDescription>,
    pub action_stack: UPtr<UOptimusActionStack>,
    pub status: EOptimusDeformerStatus,
    pub graphs: TArray<UPtr<UOptimusNodeGraph>>,
    pub bindings: UPtr<UOptimusComponentSourceBindingContainer>,
    pub variables: UPtr<UOptimusVariableContainer>,
    pub resources: UPtr<UOptimusResourceContainer>,
}
pub struct UOptimusDeformerDynamicInstanceManager {
    pub default_instance: UPtr<UOptimusDeformerInstance>,
    pub guid_to_rig_deformer_instance_map: TMap<FGuid, UPtr<UOptimusDeformerInstance>>,
    pub guid_to_instance_map: TMap<FGuid, UPtr<UOptimusDeformerInstance>>,
}
pub struct UOptimusDeformerInstanceSettings {
    pub deformer: TWeakObjectPtr<UOptimusDeformer>,
    pub bindings: TArray<FOptimusDeformerInstanceComponentBinding>,
}
pub struct UOptimusDeformerInstance {
    pub mesh_component: TWeakObjectPtr<UMeshComponent>,
    pub instance_settings: TWeakObjectPtr<UOptimusDeformerInstanceSettings>,
    pub compute_graph_exec_infos: TArray<FOptimusDeformerInstanceExecInfo>,
}
pub struct UOptimusNodeGraph {
    pub graph_type: EOptimusNodeGraphType,
    pub b_view_location_set: bool,
    pub view_location: FVector2D,
    pub view_zoom: f32,
    pub nodes: TArray<UPtr<UOptimusNode>>,
    pub links: TArray<UPtr<UOptimusNodeLink>>,
    pub node_pairs: TArray<UPtr<UOptimusNodePair>>,
    pub sub_graphs: TArray<UPtr<UOptimusNodeGraph>>,
}
pub struct UOptimusNodeSubGraph {
    pub input_bindings: FOptimusParameterBindingArray,
    pub output_bindings: FOptimusParameterBindingArray,
}
pub struct UOptimusFunctionNodeGraph {
    pub category: FName,
    pub access_specifier: FName,
    pub guid: FGuid,
}
pub struct UOptimusNodeLink {
    pub node_output_pin: UPtr<UOptimusNodePin>,
    pub node_input_pin: UPtr<UOptimusNodePin>,
}
pub struct UOptimusNodePair {
    pub first: UPtr<UOptimusNode>,
    pub second: UPtr<UOptimusNode>,
}
pub struct UOptimusNodePin {
    pub b_is_grouping_pin: bool,
    pub direction: EOptimusNodePinDirection,
    pub storage_type_deprecated: EOptimusNodePinStorageType,
    pub data_domain: FOptimusDataDomain,
    pub data_type: FOptimusDataTypeRef,
    pub sub_pins: TArray<UPtr<UOptimusNodePin>>,
}
pub struct UOptimusNode_Comment {
    pub comment_color: FLinearColor,
    pub font_size: i32,
    pub comment: FString,
    pub b_bubble_visible: bool,
    pub b_color_bubble: bool,
    pub size: FDeprecateSlateVector2D,
}
pub struct UOptimusResourceDescription {
    pub resource_name: FName,
    pub data_type: FOptimusDataTypeRef,
    pub component_binding: TWeakObjectPtr<UOptimusComponentSourceBinding>,
    pub data_domain: FOptimusDataDomain,
    pub data_interface: UPtr<UOptimusPersistentBufferDataInterface>,
}
pub struct UOptimusSource {
    pub source_text: FString,
}
pub struct UOptimusValueContainerGeneratorClass {
    pub data_type: FOptimusDataTypeRef,
}
pub struct UOptimusValueContainer {}
pub struct UOptimusVariableDescription {
    pub guid: FGuid,
    pub variable_name: FName,
    pub data_type: FOptimusDataTypeRef,
    pub default_value_struct: FOptimusValueContainerStruct,
    pub cached_shader_value: FShaderValueContainer,
    pub value_data_deprecated: TArray<u8>,
    pub default_value_deprecated: UPtr<UOptimusValueContainer>,
}
