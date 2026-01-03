#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FOptimusDataTypeRef {
    __padding_end: [u8; 64],
}
impl FOptimusDataTypeRef {}
#[repr(C, align(8))]
pub struct FRigVMTrait_OptimusDeformerSettings {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub execution_phase: EOptimusDeformerExecutionPhase,
    pub execution_group: i32,
    pub deform_child_components: bool,
    pub exclude_child_components_with_tag: FName,
}
impl FRigVMTrait_OptimusDeformerSettings {}
#[repr(C, align(8))]
pub struct FRigUnit_AddOptimusDeformer {
    __padding_end: [u8; 32],
}
impl FRigUnit_AddOptimusDeformer {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerIntVariable {
    __padding_end: [u8; 32],
}
impl FRigVMTrait_SetDeformerIntVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerIntArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerIntArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt2Variable {
    __padding_end: [u8; 32],
}
impl FRigVMTrait_SetDeformerInt2Variable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt2ArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerInt2ArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt3Variable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerInt3Variable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt3ArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerInt3ArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt4Variable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerInt4Variable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerInt4ArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerInt4ArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerFloatVariable {
    __padding_end: [u8; 32],
}
impl FRigVMTrait_SetDeformerFloatVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerFloatArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerFloatArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerVector2Variable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerVector2Variable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerVector2ArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerVector2ArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerVectorVariable {
    __padding_end: [u8; 48],
}
impl FRigVMTrait_SetDeformerVectorVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerVectorArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerVectorArrayVariable {}
#[repr(C, align(16))]
pub struct FRigVMTrait_SetDeformerVector4Variable {
    __padding_end: [u8; 64],
}
impl FRigVMTrait_SetDeformerVector4Variable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerVector4ArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerVector4ArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerLinearColorVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerLinearColorVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerLinearColorArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerLinearColorArrayVariable {}
#[repr(C, align(16))]
pub struct FRigVMTrait_SetDeformerQuatVariable {
    __padding_end: [u8; 64],
}
impl FRigVMTrait_SetDeformerQuatVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerQuatArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerQuatArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerRotatorVariable {
    __padding_end: [u8; 48],
}
impl FRigVMTrait_SetDeformerRotatorVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerRotatorArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerRotatorArrayVariable {}
#[repr(C, align(16))]
pub struct FRigVMTrait_SetDeformerTransformVariable {
    __padding_end: [u8; 128],
}
impl FRigVMTrait_SetDeformerTransformVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerTransformArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerTransformArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerNameVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerNameVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerNameArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerNameArrayVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerBoolVariable {
    __padding_end: [u8; 32],
}
impl FRigVMTrait_SetDeformerBoolVariable {}
#[repr(C, align(8))]
pub struct FRigVMTrait_SetDeformerBoolArrayVariable {
    __padding_end: [u8; 40],
}
impl FRigVMTrait_SetDeformerBoolArrayVariable {}
#[repr(C, align(4))]
pub struct FOptimusDeformerInstanceComponentBinding {
    __padding_end: [u8; 24],
}
impl FOptimusDeformerInstanceComponentBinding {}
pub struct IOptimusAlternativeSelectedObjectProvider {}
#[repr(C, align(8))]
pub struct UOptimusAlternativeSelectedObjectProvider {
    __padding_end: [u8; 48],
}
impl UOptimusAlternativeSelectedObjectProvider {}
pub struct IOptimusComponentBindingProvider {}
#[repr(C, align(8))]
pub struct UOptimusComponentBindingProvider {
    __padding_end: [u8; 48],
}
impl UOptimusComponentBindingProvider {}
pub struct IOptimusComponentBindingReceiver {}
#[repr(C, align(8))]
pub struct UOptimusComponentBindingReceiver {
    __padding_end: [u8; 48],
}
impl UOptimusComponentBindingReceiver {}
pub struct IOptimusComputeKernelDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusComputeKernelDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusComputeKernelDataInterface {}
pub struct IOptimusComputeKernelProvider {}
#[repr(C, align(8))]
pub struct UOptimusComputeKernelProvider {
    __padding_end: [u8; 48],
}
impl UOptimusComputeKernelProvider {}
pub struct IOptimusDataInterfaceProvider {}
#[repr(C, align(8))]
pub struct UOptimusDataInterfaceProvider {
    __padding_end: [u8; 48],
}
impl UOptimusDataInterfaceProvider {}
pub struct IOptimusDeformerAssetPathAccessor {}
#[repr(C, align(8))]
pub struct UOptimusDeformerAssetPathAccessor {
    __padding_end: [u8; 48],
}
impl UOptimusDeformerAssetPathAccessor {}
pub struct IOptimusDeformerGeometryReadbackProvider {}
#[repr(C, align(8))]
pub struct UOptimusDeformerGeometryReadbackProvider {
    __padding_end: [u8; 48],
}
impl UOptimusDeformerGeometryReadbackProvider {}
pub struct IOptimusDeformerInstanceAccessor {}
#[repr(C, align(8))]
pub struct UOptimusDeformerInstanceAccessor {
    __padding_end: [u8; 48],
}
impl UOptimusDeformerInstanceAccessor {}
pub struct IOptimusDeprecatedExecutionDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusDeprecatedExecutionDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusDeprecatedExecutionDataInterface {}
pub struct IOptimusExecutionDomainProvider {}
#[repr(C, align(8))]
pub struct UOptimusExecutionDomainProvider {
    __padding_end: [u8; 48],
}
impl UOptimusExecutionDomainProvider {}
pub struct IOptimusGeneratedClassDefiner {}
#[repr(C, align(8))]
pub struct UOptimusGeneratedClassDefiner {
    __padding_end: [u8; 48],
}
impl UOptimusGeneratedClassDefiner {}
pub struct IOptimusNodeAdderPinProvider {}
#[repr(C, align(8))]
pub struct UOptimusNodeAdderPinProvider {
    __padding_end: [u8; 48],
}
impl UOptimusNodeAdderPinProvider {}
pub struct IOptimusNodeFunctionLibraryOwner {}
#[repr(C, align(8))]
pub struct UOptimusNodeFunctionLibraryOwner {
    __padding_end: [u8; 48],
}
impl UOptimusNodeFunctionLibraryOwner {}
pub struct IOptimusNodeGraphCollectionOwner {}
#[repr(C, align(8))]
pub struct UOptimusNodeGraphCollectionOwner {
    __padding_end: [u8; 48],
}
impl UOptimusNodeGraphCollectionOwner {}
pub struct IOptimusNodeGraphProvider {}
#[repr(C, align(8))]
pub struct UOptimusNodeGraphProvider {
    __padding_end: [u8; 48],
}
impl UOptimusNodeGraphProvider {}
pub struct IOptimusNodePairProvider {}
#[repr(C, align(8))]
pub struct UOptimusNodePairProvider {
    __padding_end: [u8; 48],
}
impl UOptimusNodePairProvider {}
pub struct IOptimusNodePinRouter {}
#[repr(C, align(8))]
pub struct UOptimusNodePinRouter {
    __padding_end: [u8; 48],
}
impl UOptimusNodePinRouter {}
pub struct IOptimusNodeSubGraphReferencer {}
#[repr(C, align(8))]
pub struct UOptimusNodeSubGraphReferencer {
    __padding_end: [u8; 48],
}
impl UOptimusNodeSubGraphReferencer {}
pub struct IOptimusNonCollapsibleNode {}
#[repr(C, align(8))]
pub struct UOptimusNonCollapsibleNode {
    __padding_end: [u8; 48],
}
impl UOptimusNonCollapsibleNode {}
pub struct IOptimusNonCopyableNode {}
#[repr(C, align(8))]
pub struct UOptimusNonCopyableNode {
    __padding_end: [u8; 48],
}
impl UOptimusNonCopyableNode {}
pub struct IOptimusOutputBufferWriter {}
#[repr(C, align(8))]
pub struct UOptimusOutputBufferWriter {
    __padding_end: [u8; 48],
}
impl UOptimusOutputBufferWriter {}
pub struct IOptimusParameterBindingProvider {}
#[repr(C, align(8))]
pub struct UOptimusParameterBindingProvider {
    __padding_end: [u8; 48],
}
impl UOptimusParameterBindingProvider {}
pub struct IOptimusPathResolver {}
#[repr(C, align(8))]
pub struct UOptimusPathResolver {
    __padding_end: [u8; 48],
}
impl UOptimusPathResolver {}
pub struct IOptimusPersistentBufferProvider {}
#[repr(C, align(8))]
pub struct UOptimusPersistentBufferProvider {
    __padding_end: [u8; 48],
}
impl UOptimusPersistentBufferProvider {}
pub struct IOptimusPinMutabilityDefiner {}
#[repr(C, align(8))]
pub struct UOptimusPinMutabilityDefiner {
    __padding_end: [u8; 48],
}
impl UOptimusPinMutabilityDefiner {}
pub struct IOptimusPropertyPinProvider {}
#[repr(C, align(8))]
pub struct UOptimusPropertyPinProvider {
    __padding_end: [u8; 48],
}
impl UOptimusPropertyPinProvider {}
pub struct IOptimusShaderTextProvider {}
#[repr(C, align(8))]
pub struct UOptimusShaderTextProvider {
    __padding_end: [u8; 48],
}
impl UOptimusShaderTextProvider {}
pub struct IOptimusUnnamedNodePinProvider {}
#[repr(C, align(8))]
pub struct UOptimusUnnamedNodePinProvider {
    __padding_end: [u8; 48],
}
impl UOptimusUnnamedNodePinProvider {}
pub struct IOptimusValueProvider {}
#[repr(C, align(8))]
pub struct UOptimusValueProvider {
    __padding_end: [u8; 48],
}
impl UOptimusValueProvider {}
#[repr(C, align(8))]
pub struct UOptimusKernelSource {
    __padding_end: [u8; 176],
}
impl UOptimusKernelSource {}
#[repr(C, align(8))]
pub struct UOptimusComponentSource {
    __padding_end: [u8; 48],
}
impl UOptimusComponentSource {}
#[repr(C, align(8))]
pub struct UOptimusSceneComponentSource {
    __padding_end: [u8; 48],
}
impl UOptimusSceneComponentSource {}
#[repr(C, align(8))]
pub struct UOptimusSkinnedMeshComponentSource {
    __padding_end: [u8; 48],
}
impl UOptimusSkinnedMeshComponentSource {}
#[repr(C, align(8))]
pub struct UOptimusSkeletalMeshComponentSource {
    __padding_end: [u8; 48],
}
impl UOptimusSkeletalMeshComponentSource {}
#[repr(C, align(8))]
pub struct UOptimusComputeDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusComputeDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusAdvancedSkeletonDataInterface {
    __padding_end: [u8; 152],
}
impl UOptimusAdvancedSkeletonDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusAdvancedSkeletonDataProvider {
    __padding_end: [u8; 232],
}
impl UOptimusAdvancedSkeletonDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusAnimAttributeDataInterface {
    __padding_end: [u8; 112],
}
impl UOptimusAnimAttributeDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusAnimAttributeDataProvider {
    __padding_end: [u8; 80],
}
impl UOptimusAnimAttributeDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusClothDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusClothDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusClothDataProvider {
    __padding_end: [u8; 56],
}
impl UOptimusClothDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusConnectivityDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusConnectivityDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusConnectivityDataProvider {
    __padding_end: [u8; 72],
}
impl UOptimusConnectivityDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusCopyKernelDataInterface {
    __padding_end: [u8; 80],
}
impl UOptimusCopyKernelDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusCopyKernelDataProvider {
    __padding_end: [u8; 216],
}
impl UOptimusCopyKernelDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusCustomComputeKernelDataInterface {
    __padding_end: [u8; 120],
}
impl UOptimusCustomComputeKernelDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusCustomComputeKernelDataProvider {
    __padding_end: [u8; 216],
}
impl UOptimusCustomComputeKernelDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusDebugDrawDataInterface {
    __padding_end: [u8; 72],
}
impl UOptimusDebugDrawDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusDebugDrawDataProvider {
    __padding_end: [u8; 80],
}
impl UOptimusDebugDrawDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusDuplicateVerticesDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusDuplicateVerticesDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusDuplicateVerticesDataProvider {
    __padding_end: [u8; 56],
}
impl UOptimusDuplicateVerticesDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusGraphDataInterface {
    __padding_end: [u8; 72],
}
impl UOptimusGraphDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusGraphDataProvider {
    __padding_end: [u8; 104],
}
impl UOptimusGraphDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusHalfEdgeDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusHalfEdgeDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusHalfEdgeDataProvider {
    __padding_end: [u8; 192],
}
impl UOptimusHalfEdgeDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusLoopTerminalDataInterface {
    __padding_end: [u8; 56],
}
impl UOptimusLoopTerminalDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusLoopTerminalDataProvider {
    __padding_end: [u8; 56],
}
impl UOptimusLoopTerminalDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusMorphTargetDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusMorphTargetDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusMorphTargetDataProvider {
    __padding_end: [u8; 56],
}
impl UOptimusMorphTargetDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusRawBufferDataInterface {
    __padding_end: [u8; 168],
}
impl UOptimusRawBufferDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusTransientBufferDataInterface {
    __padding_end: [u8; 176],
}
impl UOptimusTransientBufferDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusImplicitPersistentBufferDataInterface {
    __padding_end: [u8; 176],
}
impl UOptimusImplicitPersistentBufferDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusPersistentBufferDataInterface {
    __padding_end: [u8; 184],
}
impl UOptimusPersistentBufferDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusRawBufferDataProvider {
    __padding_end: [u8; 224],
}
impl UOptimusRawBufferDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusTransientBufferDataProvider {
    __padding_end: [u8; 232],
}
impl UOptimusTransientBufferDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusImplicitPersistentBufferDataProvider {
    __padding_end: [u8; 264],
}
impl UOptimusImplicitPersistentBufferDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusPersistentBufferDataProvider {
    __padding_end: [u8; 264],
}
impl UOptimusPersistentBufferDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusSceneDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusSceneDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusSceneDataProvider {
    __padding_end: [u8; 56],
}
impl UOptimusSceneDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusSkeletonDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusSkeletonDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusSkeletonDataProvider {
    __padding_end: [u8; 56],
}
impl UOptimusSkeletonDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusSkinnedMeshDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusSkinnedMeshDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusSkinnedMeshDataProvider {
    __padding_end: [u8; 56],
}
impl UOptimusSkinnedMeshDataProvider {}
#[repr(C, align(8))]
pub struct UDEPRECATED_OptimusSkinnedMeshExecDataInterface {
    __padding_end: [u8; 64],
}
impl UDEPRECATED_OptimusSkinnedMeshExecDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusSkinnedMeshReadDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusSkinnedMeshReadDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusSkinnedMeshReadDataProvider {
    __padding_end: [u8; 88],
}
impl UOptimusSkinnedMeshReadDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusSkinnedMeshVertexAttributeDataInterface {
    __padding_end: [u8; 64],
}
impl UOptimusSkinnedMeshVertexAttributeDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusSkinnedMeshVertexAttributeDataProvider {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub attribute_name: FName,
    pub default_value: f32,
    __padding_end: [u8; 16],
}
impl UOptimusSkinnedMeshVertexAttributeDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusSkinnedMeshWriteDataInterface {
    __padding_end: [u8; 56],
}
impl UOptimusSkinnedMeshWriteDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusSkinnedMeshWriteDataProvider {
    __padding_end: [u8; 96],
}
impl UOptimusSkinnedMeshWriteDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusSkinWeightsAsVertexMaskDataInterface {
    __padding_end: [u8; 168],
}
impl UOptimusSkinWeightsAsVertexMaskDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusSkinWeightsAsVertexMaskDataProvider {
    __padding_end: [u8; 232],
}
impl UOptimusSkinWeightsAsVertexMaskDataProvider {}
#[repr(C, align(8))]
pub struct UOptimusNode {
    __padding_end: [u8; 272],
}
impl UOptimusNode {}
#[repr(C, align(8))]
pub struct UOptimusNode_DataInterface {
    __padding_end: [u8; 320],
}
impl UOptimusNode_DataInterface {}
#[repr(C, align(8))]
pub struct UOptimusNode_AnimAttributeDataInterface {
    __padding_end: [u8; 320],
}
impl UOptimusNode_AnimAttributeDataInterface {}
#[repr(C, align(8))]
pub struct UOptimusNode_ComponentSource {
    __padding_end: [u8; 320],
}
impl UOptimusNode_ComponentSource {}
#[repr(C, align(8))]
pub struct UOptimusNode_ComputeKernelBase {
    __padding_end: [u8; 280],
}
impl UOptimusNode_ComputeKernelBase {}
#[repr(C, align(8))]
pub struct UOptimusNode_ComputeKernelFunctionGeneratorClass {
    __padding_end: [u8; 744],
}
impl UOptimusNode_ComputeKernelFunctionGeneratorClass {}
#[repr(C, align(8))]
pub struct UOptimusNode_ComputeKernelFunction {
    __padding_end: [u8; 280],
}
impl UOptimusNode_ComputeKernelFunction {}
#[repr(C, align(8))]
pub struct UOptimusNode_ConstantValueGeneratorClass {
    __padding_end: [u8; 688],
}
impl UOptimusNode_ConstantValueGeneratorClass {}
#[repr(C, align(8))]
pub struct UOptimusNode_ConstantValue {
    __padding_end: [u8; 296],
}
impl UOptimusNode_ConstantValue {}
#[repr(C, align(8))]
pub struct UOptimusNode_CustomComputeKernel {
    __padding_end: [u8; 528],
}
impl UOptimusNode_CustomComputeKernel {}
#[repr(C, align(8))]
pub struct UOptimusNode_FunctionReference {
    __padding_end: [u8; 392],
}
impl UOptimusNode_FunctionReference {}
#[repr(C, align(8))]
pub struct UOptimusNode_ResourceAccessorBase {
    __padding_end: [u8; 464],
}
impl UOptimusNode_ResourceAccessorBase {}
#[repr(C, align(8))]
pub struct UOptimusNode_GetResource {
    __padding_end: [u8; 464],
}
impl UOptimusNode_GetResource {}
#[repr(C, align(8))]
pub struct UOptimusNode_GetVariable {
    __padding_end: [u8; 400],
}
impl UOptimusNode_GetVariable {}
#[repr(C, align(8))]
pub struct UOptimusNode_GraphTerminal {
    __padding_end: [u8; 328],
}
impl UOptimusNode_GraphTerminal {}
#[repr(C, align(8))]
pub struct UOptimusNode_LoopTerminal {
    __padding_end: [u8; 376],
}
impl UOptimusNode_LoopTerminal {}
#[repr(C, align(8))]
pub struct UOptimusNode_Resource {
    __padding_end: [u8; 464],
}
impl UOptimusNode_Resource {}
#[repr(C, align(8))]
pub struct UOptimusNode_SetResource {
    __padding_end: [u8; 464],
}
impl UOptimusNode_SetResource {}
#[repr(C, align(8))]
pub struct UOptimusNode_SubGraphReference {
    __padding_end: [u8; 328],
}
impl UOptimusNode_SubGraphReference {}
#[repr(C, align(16))]
pub struct UOptimusActionStack {
    __padding_end: [u8; 192],
}
impl UOptimusActionStack {}
#[repr(C, align(8))]
pub struct UOptimusComponentSourceBinding {
    __padding_end: [u8; 104],
}
impl UOptimusComponentSourceBinding {}
#[repr(C, align(8))]
pub struct UOptimusComputeGraph {
    __padding_end: [u8; 248],
}
impl UOptimusComputeGraph {}
#[repr(C, align(8))]
pub struct UOptimusComponentSourceBindingContainer {
    __padding_end: [u8; 64],
}
impl UOptimusComponentSourceBindingContainer {}
#[repr(C, align(8))]
pub struct UOptimusVariableContainer {
    __padding_end: [u8; 64],
}
impl UOptimusVariableContainer {}
#[repr(C, align(8))]
pub struct UOptimusResourceContainer {
    __padding_end: [u8; 64],
}
impl UOptimusResourceContainer {}
#[repr(C, align(8))]
pub struct UOptimusDeformer {
    __padding_end: [u8; 520],
}
impl UOptimusDeformer {}
#[repr(C, align(8))]
pub struct UOptimusDeformerDynamicInstanceManager {
    __padding_end: [u8; 448],
}
impl UOptimusDeformerDynamicInstanceManager {}
#[repr(C, align(8))]
pub struct UOptimusDeformerInstanceSettings {
    __padding_end: [u8; 72],
}
impl UOptimusDeformerInstanceSettings {}
#[repr(C, align(8))]
pub struct UOptimusDeformerInstance {
    __padding_end: [u8; 448],
}
impl UOptimusDeformerInstance {}
#[repr(C, align(8))]
pub struct UOptimusNodeGraph {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub graph_type: EOptimusNodeGraphType,
    __padding_end: [u8; 124],
}
impl UOptimusNodeGraph {}
#[repr(C, align(8))]
pub struct UOptimusNodeSubGraph {
    __padding_end: [u8; 416],
}
impl UOptimusNodeSubGraph {}
#[repr(C, align(8))]
pub struct UOptimusFunctionNodeGraph {
    __padding_end: [u8; 456],
}
impl UOptimusFunctionNodeGraph {}
#[repr(C, align(8))]
pub struct UOptimusNodeLink {
    __padding_end: [u8; 64],
}
impl UOptimusNodeLink {}
#[repr(C, align(8))]
pub struct UOptimusNodePair {
    __padding_end: [u8; 64],
}
impl UOptimusNodePair {}
#[repr(C, align(8))]
pub struct UOptimusNodePin {
    __padding_end: [u8; 216],
}
impl UOptimusNodePin {}
#[repr(C, align(8))]
pub struct UOptimusNode_Comment {
    __padding_end: [u8; 352],
}
impl UOptimusNode_Comment {}
#[repr(C, align(8))]
pub struct UOptimusResourceDescription {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub resource_name: FName,
    __padding_end: [u8; 164],
}
impl UOptimusResourceDescription {}
#[repr(C, align(8))]
pub struct UOptimusSource {
    __padding_end: [u8; 88],
}
impl UOptimusSource {}
#[repr(C, align(8))]
pub struct UOptimusValueContainerGeneratorClass {
    __padding_end: [u8; 688],
}
impl UOptimusValueContainerGeneratorClass {}
#[repr(C, align(8))]
pub struct UOptimusValueContainer {
    __padding_end: [u8; 48],
}
impl UOptimusValueContainer {}
#[repr(C, align(8))]
pub struct UOptimusVariableDescription {
    __padding_end: [u8; 232],
}
impl UOptimusVariableDescription {}
#[repr(transparent)]
pub struct EOptimusDataDomainType(pub i32);
impl EOptimusDataDomainType {
    pub const DIMENSIONAL: EOptimusDataDomainType = EOptimusDataDomainType(0);
    pub const EXPRESSION: EOptimusDataDomainType = EOptimusDataDomainType(1);
}
#[repr(transparent)]
pub struct EOptimusValueType(pub u8);
impl EOptimusValueType {
    pub const INVALID: EOptimusValueType = EOptimusValueType(0);
    pub const CONSTANT: EOptimusValueType = EOptimusValueType(1);
    pub const VARIABLE: EOptimusValueType = EOptimusValueType(2);
}
#[repr(transparent)]
pub struct EOptimusValueUsage(pub u8);
impl EOptimusValueUsage {
    pub const NONE: EOptimusValueUsage = EOptimusValueUsage(0);
    pub const CPU: EOptimusValueUsage = EOptimusValueUsage(1);
    pub const GPU: EOptimusValueUsage = EOptimusValueUsage(2);
}
#[repr(transparent)]
pub struct EOptimusDeformerExecutionPhase(pub u8);
impl EOptimusDeformerExecutionPhase {
    pub const AFTER_DEFAULT_DEFORMER: EOptimusDeformerExecutionPhase = EOptimusDeformerExecutionPhase(
        0,
    );
    pub const OVERRIDE_DEFAULT_DEFORMER: EOptimusDeformerExecutionPhase = EOptimusDeformerExecutionPhase(
        1,
    );
    pub const BEFORE_DEFAULT_DEFORMER: EOptimusDeformerExecutionPhase = EOptimusDeformerExecutionPhase(
        2,
    );
}
#[repr(transparent)]
pub struct EOptimusConstantType(pub i32);
impl EOptimusConstantType {
    pub const INPUT: EOptimusConstantType = EOptimusConstantType(0);
    pub const OUTPUT: EOptimusConstantType = EOptimusConstantType(1);
}
#[repr(transparent)]
pub struct EOptimusDataTypeUsageFlags(pub u8);
impl EOptimusDataTypeUsageFlags {
    pub const NONE: EOptimusDataTypeUsageFlags = EOptimusDataTypeUsageFlags(0);
    pub const RESOURCE: EOptimusDataTypeUsageFlags = EOptimusDataTypeUsageFlags(1);
    pub const VARIABLE: EOptimusDataTypeUsageFlags = EOptimusDataTypeUsageFlags(2);
    pub const ANIM_ATTRIBUTES: EOptimusDataTypeUsageFlags = EOptimusDataTypeUsageFlags(
        4,
    );
    pub const DATA_INTERFACE_OUTPUT: EOptimusDataTypeUsageFlags = EOptimusDataTypeUsageFlags(
        8,
    );
    pub const PIN_TYPE: EOptimusDataTypeUsageFlags = EOptimusDataTypeUsageFlags(16);
    pub const PER_BONE_ANIM_ATTRIBUTE: EOptimusDataTypeUsageFlags = EOptimusDataTypeUsageFlags(
        32,
    );
    pub const PROPERTY: EOptimusDataTypeUsageFlags = EOptimusDataTypeUsageFlags(64);
}
#[repr(transparent)]
pub struct EOptimusDataTypeFlags(pub u8);
impl EOptimusDataTypeFlags {
    pub const NONE: EOptimusDataTypeFlags = EOptimusDataTypeFlags(0);
    pub const IS_STRUCT_TYPE: EOptimusDataTypeFlags = EOptimusDataTypeFlags(1);
    pub const SHOW_ELEMENTS: EOptimusDataTypeFlags = EOptimusDataTypeFlags(2);
}
#[repr(transparent)]
pub struct EOptimusNodeGraphType(pub i32);
impl EOptimusNodeGraphType {
    pub const SETUP: EOptimusNodeGraphType = EOptimusNodeGraphType(0);
    pub const UPDATE: EOptimusNodeGraphType = EOptimusNodeGraphType(1);
    pub const EXTERNAL_TRIGGER: EOptimusNodeGraphType = EOptimusNodeGraphType(2);
    pub const FUNCTION: EOptimusNodeGraphType = EOptimusNodeGraphType(3);
    pub const SUB_GRAPH: EOptimusNodeGraphType = EOptimusNodeGraphType(4);
    pub const TRANSIENT: EOptimusNodeGraphType = EOptimusNodeGraphType(5);
}
#[repr(transparent)]
pub struct EOptimusExecutionDomainType(pub i32);
impl EOptimusExecutionDomainType {
    pub const DOMAIN_NAME: EOptimusExecutionDomainType = EOptimusExecutionDomainType(0);
    pub const EXPRESSION: EOptimusExecutionDomainType = EOptimusExecutionDomainType(1);
}
#[repr(transparent)]
pub struct EOptimusSkinnedMeshExecDomain(pub u8);
impl EOptimusSkinnedMeshExecDomain {
    pub const NONE: EOptimusSkinnedMeshExecDomain = EOptimusSkinnedMeshExecDomain(0);
    pub const VERTEX: EOptimusSkinnedMeshExecDomain = EOptimusSkinnedMeshExecDomain(1);
    pub const TRIANGLE: EOptimusSkinnedMeshExecDomain = EOptimusSkinnedMeshExecDomain(2);
}
#[repr(transparent)]
pub struct EOptimusDiagnosticLevel(pub u8);
impl EOptimusDiagnosticLevel {
    pub const NONE: EOptimusDiagnosticLevel = EOptimusDiagnosticLevel(0);
    pub const INFO: EOptimusDiagnosticLevel = EOptimusDiagnosticLevel(1);
    pub const WARNING: EOptimusDiagnosticLevel = EOptimusDiagnosticLevel(2);
    pub const ERROR: EOptimusDiagnosticLevel = EOptimusDiagnosticLevel(3);
}
#[repr(transparent)]
pub struct EOptimusBufferWriteType(pub u8);
impl EOptimusBufferWriteType {
    pub const WRITE: EOptimusBufferWriteType = EOptimusBufferWriteType(0);
    pub const WRITE_ATOMIC_ADD: EOptimusBufferWriteType = EOptimusBufferWriteType(1);
    pub const WRITE_ATOMIC_MIN: EOptimusBufferWriteType = EOptimusBufferWriteType(2);
    pub const WRITE_ATOMIC_MAX: EOptimusBufferWriteType = EOptimusBufferWriteType(3);
    pub const COUNT: EOptimusBufferWriteType = EOptimusBufferWriteType(4);
}
#[repr(transparent)]
pub struct EOptimusTerminalType(pub i32);
impl EOptimusTerminalType {
    pub const UNKNOWN: EOptimusTerminalType = EOptimusTerminalType(0);
    pub const ENTRY: EOptimusTerminalType = EOptimusTerminalType(1);
    pub const RETURN: EOptimusTerminalType = EOptimusTerminalType(2);
}
#[repr(transparent)]
pub struct EOptimusDeformerStatus(pub i32);
impl EOptimusDeformerStatus {
    pub const COMPILED: EOptimusDeformerStatus = EOptimusDeformerStatus(0);
    pub const COMPILED_WITH_WARNINGS: EOptimusDeformerStatus = EOptimusDeformerStatus(1);
    pub const MODIFIED: EOptimusDeformerStatus = EOptimusDeformerStatus(2);
    pub const HAS_ERRORS: EOptimusDeformerStatus = EOptimusDeformerStatus(3);
}
#[repr(transparent)]
pub struct EOptimusNodePinDirection(pub u8);
impl EOptimusNodePinDirection {
    pub const UNKNOWN: EOptimusNodePinDirection = EOptimusNodePinDirection(0);
    pub const INPUT: EOptimusNodePinDirection = EOptimusNodePinDirection(1);
    pub const OUTPUT: EOptimusNodePinDirection = EOptimusNodePinDirection(2);
}
#[repr(transparent)]
pub struct EOptimusNodePinStorageType(pub u8);
impl EOptimusNodePinStorageType {
    pub const VALUE: EOptimusNodePinStorageType = EOptimusNodePinStorageType(0);
    pub const RESOURCE: EOptimusNodePinStorageType = EOptimusNodePinStorageType(1);
}
