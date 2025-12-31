#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FRigVMUserWorkflow {
    pub title: FString,
    pub tooltip: FString,
    pub ty: ERigVMUserWorkflowType,
    pub perform_dynamic_delegate: FRigVMUserWorkflow_PerformDynamicDelegate,
    pub options_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(16))]
pub struct FRigVMExtendedExecuteContext {
    pub work_memory_storage_object_deprecated: UPtr<URigVMMemoryStorage>,
    pub debug_memory_storage_object_deprecated: UPtr<URigVMMemoryStorage>,
}
#[repr(C, align(8))]
pub struct FRigVMDrawContainer {
    pub instructions: TArray<FRigVMDrawInstruction>,
}
#[repr(C, align(16))]
pub struct FRigVMDrawInstruction {
    pub name: FName,
    pub primitive_type: ERigVMDrawSettings,
    pub positions: TArray<crate::bindings::core_u_object::FVector>,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub depth_priority: crate::bindings::engine::ESceneDepthPriorityGroup,
    pub lifetime: f32,
}
#[repr(C, align(8))]
pub struct FRigVMRuntimeSettings {
    pub maximum_array_size: i32,
    pub b_enable_profiling: bool,
}
#[repr(C, align(4))]
pub struct FRigVMDebugDrawSettings {
    pub depth_priority: crate::bindings::engine::ESceneDepthPriorityGroup,
    pub lifetime: f32,
}
#[repr(C, align(8))]
pub struct FRigVMStruct {}
#[repr(C, align(8))]
pub struct FRigVMFunction_AnimBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_DebugBase {
    pub debug_draw_settings: FRigVMDebugDrawSettings,
}
#[repr(C, align(8))]
pub struct FRigVMStructMutable {
    pub execute_pin: FRigVMExecutePin,
}
#[repr(C, align(8))]
pub struct FRigVMExecutePin {}
#[repr(C, align(8))]
pub struct FRigVMFunction_DebugBaseMutable {
    pub debug_draw_settings: FRigVMDebugDrawSettings,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathMutableBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_SimBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_SimBaseMutable {}
#[repr(C, align(8))]
pub struct FRigVMFunction_DampFloat {
    pub value: f32,
    pub target: f32,
    pub smoothing_time: f32,
    pub result: f32,
}
#[repr(C, align(16))]
pub struct FRigVMExecuteContext {}
#[repr(C, align(8))]
pub struct FRigVMDispatchFactory {}
#[repr(C, align(4))]
pub struct FRigVMUnknownType {
    pub hash: u32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_DampVector {
    pub value: crate::bindings::core_u_object::FVector,
    pub target: crate::bindings::core_u_object::FVector,
    pub smoothing_time: f32,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DampQuaternion {
    pub value: crate::bindings::core_u_object::FQuat,
    pub target: crate::bindings::core_u_object::FQuat,
    pub smoothing_time: f32,
    pub result: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_SpringDampFloat {
    pub value: f32,
    pub value_velocity: f32,
    pub target: f32,
    pub smoothing_time: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_SpringDampVector {
    pub value: crate::bindings::core_u_object::FVector,
    pub value_velocity: crate::bindings::core_u_object::FVector,
    pub target: crate::bindings::core_u_object::FVector,
    pub smoothing_time: f32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_SpringDampQuat {
    pub value: crate::bindings::core_u_object::FQuat,
    pub value_velocity: crate::bindings::core_u_object::FVector,
    pub target: crate::bindings::core_u_object::FQuat,
    pub smoothing_time: f32,
}
#[repr(C, align(4))]
pub struct FRigVMMemoryStatistics {
    pub register_count: i32,
    pub data_bytes: i32,
    pub total_bytes: i32,
}
#[repr(C, align(4))]
pub struct FRigVMByteCodeStatistics {
    pub instruction_count: i32,
    pub data_bytes: i32,
}
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
#[repr(C, align(8))]
pub struct FRigVMTrait {
    pub name: FString,
}
#[repr(C, align(8))]
pub struct FRigVMParameter {
    pub ty: ERigVMParameterType,
    pub name: FName,
    pub register_index: i32,
    pub cpp_type: FString,
    pub script_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub script_struct_path: FName,
}
#[repr(C, align(8))]
pub struct FRigVMPredicateBranch {}
#[repr(C, align(1))]
pub struct FRigVMBaseOp {}
#[repr(C, align(2))]
pub struct FRigVMExecuteOp {}
#[repr(C, align(2))]
pub struct FRigVMUnaryOp {}
#[repr(C, align(2))]
pub struct FRigVMBinaryOp {}
#[repr(C, align(2))]
pub struct FRigVMTernaryOp {}
#[repr(C, align(2))]
pub struct FRigVMQuaternaryOp {}
#[repr(C, align(2))]
pub struct FRigVMQuinaryOp {}
#[repr(C, align(2))]
pub struct FRigVMSenaryOp {}
#[repr(C, align(2))]
pub struct FRigVMCopyOp {}
#[repr(C, align(2))]
pub struct FRigVMComparisonOp {}
#[repr(C, align(4))]
pub struct FRigVMJumpOp {}
#[repr(C, align(4))]
pub struct FRigVMJumpIfOp {}
#[repr(C, align(2))]
pub struct FRigVMChangeTypeOp {}
#[repr(C, align(4))]
pub struct FRigVMInvokeEntryOp {}
#[repr(C, align(4))]
pub struct FRigVMJumpToBranchOp {}
#[repr(C, align(4))]
pub struct FRigVMRunInstructionsOp {}
#[repr(C, align(2))]
pub struct FRigVMSetupTraitsOp {}
#[repr(C, align(4))]
pub struct FRigVMInstruction {
    pub index: i32,
    pub byte_code_index: i32,
    pub op_code: ERigVMOpCode,
    pub operand_alignment: u8,
}
#[repr(C, align(8))]
pub struct FRigVMInstructionArray {
    pub instructions: TArray<FRigVMInstruction>,
}
#[repr(C, align(4))]
pub struct FRigVMByteCodeEntry {
    pub name: FName,
    pub instruction_index: i32,
}
#[repr(C, align(8))]
pub struct FRigVMByteCode {
    pub byte_code: TArray<u8>,
    pub num_instructions: i32,
    pub entries: TArray<FRigVMByteCodeEntry>,
    pub branch_infos: TArray<FRigVMBranchInfo>,
    pub predicate_branches: TArray<FRigVMPredicateBranch>,
    pub public_context_asset_path: crate::bindings::core_u_object::FTopLevelAssetPath,
}
#[repr(C, align(4))]
pub struct FRigVMBranchInfo {
    pub index: i32,
    pub label: FName,
    pub instruction_index: i32,
    pub argument_index: i32,
    pub first_instruction: i32,
    pub last_instruction: i32,
}
#[repr(C, align(8))]
pub struct FRigVMDebugInfo {}
#[repr(C, align(8))]
pub struct FRigVMDrawInterface {}
#[repr(C, align(4))]
pub struct FRigVMSlice {}
#[repr(C, align(8))]
pub struct FRigVMExternalVariableDef {}
#[repr(C, align(8))]
pub struct FRigVMExternalVariable {}
#[repr(C, align(8))]
pub struct FRigVMFunctionCompilationPropertyDescription {
    pub name: FName,
    pub cpp_type: FString,
    pub cpp_type_object: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    pub default_value: FString,
}
#[repr(C, align(8))]
pub struct FRigVMFunctionCompilationPropertyPath {
    pub property_index: i32,
    pub head_cpp_type: FString,
    pub segment_path: FString,
}
#[repr(C, align(8))]
pub struct FRigVMFunctionCompilationData {
    pub byte_code: FRigVMByteCode,
    pub function_names: TArray<FName>,
    pub work_property_descriptions: TArray<FRigVMFunctionCompilationPropertyDescription>,
    pub work_property_path_descriptions: TArray<FRigVMFunctionCompilationPropertyPath>,
    pub literal_property_descriptions: TArray<
        FRigVMFunctionCompilationPropertyDescription,
    >,
    pub literal_property_path_descriptions: TArray<
        FRigVMFunctionCompilationPropertyPath,
    >,
    pub debug_property_descriptions: TArray<
        FRigVMFunctionCompilationPropertyDescription,
    >,
    pub debug_property_path_descriptions: TArray<FRigVMFunctionCompilationPropertyPath>,
    pub external_property_descriptions: TArray<
        FRigVMFunctionCompilationPropertyDescription,
    >,
    pub external_property_path_descriptions: TArray<
        FRigVMFunctionCompilationPropertyPath,
    >,
    pub external_register_index_to_variable: TMap<i32, FName>,
    pub operands: TMap<FString, FRigVMOperand>,
    pub hash: u32,
    pub b_encountered_surpressed_errors: bool,
}
#[repr(C, align(2))]
pub struct FRigVMOperand {
    pub memory_type: ERigVMMemoryType,
    pub register_index: u16,
    pub register_offset: u16,
}
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
    pub path_to_tooltip: TMap<FString, FText>,
}
#[repr(C, align(8))]
pub struct FRigVMGraphFunctionIdentifier {
    pub library_node_deprecated: crate::bindings::core_u_object::FSoftObjectPath,
    pub library_node_path: FString,
    pub host_object: crate::bindings::core_u_object::FSoftObjectPath,
}
#[repr(C, align(8))]
pub struct FRigVMGraphFunctionHeader {
    pub library_pointer: FRigVMGraphFunctionIdentifier,
    pub variant: FRigVMVariant,
    pub name: FName,
    pub node_title: FString,
    pub node_color: crate::bindings::core_u_object::FLinearColor,
    pub tooltip_deprecated: FText,
    pub description: FString,
    pub category: FString,
    pub keywords: FString,
    pub arguments: TArray<FRigVMGraphFunctionArgument>,
    pub dependencies: TMap<FRigVMGraphFunctionIdentifier, u32>,
    pub external_variables: TArray<FRigVMExternalVariable>,
    pub layout: FRigVMNodeLayout,
}
#[repr(C, align(8))]
pub struct FRigVMNodeLayout {
    pub categories: TArray<FRigVMPinCategory>,
    pub pin_index_in_category: TMap<FString, i32>,
    pub display_names: TMap<FString, FString>,
}
#[repr(C, align(8))]
pub struct FRigVMPinCategory {
    pub path: FString,
    pub elements: TArray<FString>,
    pub b_expanded_by_default: bool,
}
#[repr(C, align(8))]
pub struct FRigVMVariant {
    pub guid: crate::bindings::core_u_object::FGuid,
    pub tags: TArray<FRigVMTag>,
}
#[repr(C, align(8))]
pub struct FRigVMTag {
    pub name: FName,
    pub label: FString,
    pub tool_tip: FText,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub b_show_in_user_interface: bool,
    pub b_marks_subject_as_invalid: bool,
}
#[repr(C, align(8))]
pub struct FRigVMGraphFunctionHeaderArray {
    pub headers: TArray<FRigVMGraphFunctionHeader>,
}
#[repr(C, align(8))]
pub struct FRigVMGraphFunctionData {
    pub header: FRigVMGraphFunctionHeader,
    pub compilation_data: FRigVMFunctionCompilationData,
    pub serialized_collapsed_node_deprecated: FString,
    pub collapse_node_archive: FRigVMObjectArchive,
}
#[repr(C, align(8))]
pub struct FRigVMObjectArchive {
    pub buffer: TArray<u8>,
    pub uncompressed_size: i32,
    pub compressed_size: i32,
    pub b_is_compressed: bool,
}
#[repr(C, align(8))]
pub struct FRigVMGraphFunctionStore {
    pub public_functions: TArray<FRigVMGraphFunctionData>,
    pub private_functions: TArray<FRigVMGraphFunctionData>,
}
#[repr(C, align(8))]
pub struct FRigVMInstructionSetExecuteState {
    pub slice_hash_to_num_instruction: TMap<u32, u32>,
}
#[repr(C, align(8))]
pub struct FRigVMRegister {
    pub ty: ERigVMRegisterType,
    pub byte_index: u32,
    pub element_size: u16,
    pub element_count: u16,
    pub slice_count: u16,
    pub alignment_bytes: u8,
    pub trailing_bytes: u16,
    pub name: FName,
    pub script_struct_index: i32,
    pub b_is_array: bool,
    pub b_is_dynamic: bool,
    pub base_cpp_type: FName,
    pub base_cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FRigVMRegisterOffset {
    pub segments: TArray<i32>,
    pub ty: ERigVMRegisterType,
    pub cpp_type: FName,
    pub script_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub parent_script_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub array_index: i32,
    pub element_size: u16,
    pub cached_segment_path: FString,
}
#[repr(C, align(8))]
pub struct FRigVMMemoryContainer {
    pub b_use_name_map: bool,
    pub memory_type: ERigVMMemoryType,
    pub registers: TArray<FRigVMRegister>,
    pub register_offsets: TArray<FRigVMRegisterOffset>,
    pub data: TArray<u8>,
    pub script_structs: TArray<UPtr<crate::bindings::core_u_object::UScriptStruct>>,
    pub name_map: TMap<FName, i32>,
    pub b_encountered_error_during_load: bool,
}
#[repr(C, align(8))]
pub struct FRigVMMemoryStorageStruct {}
#[repr(C, align(8))]
pub struct FRigVMInstructionVisitInfo {}
#[repr(C, align(8))]
pub struct FRigVMProfilingInfo {}
#[repr(C, align(8))]
pub struct FRigVMTemplateArgumentType {
    pub cpp_type: FName,
    pub cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FRigVMVariantRef {
    pub object_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub variant: FRigVMVariant,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_AnimEasingType {
    pub ty: ERigVMAnimEasingType,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_AnimEasing {
    pub value: f32,
    pub ty: ERigVMAnimEasingType,
    pub source_minimum: f32,
    pub source_maximum: f32,
    pub target_minimum: f32,
    pub target_maximum: f32,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_AnimEvalRichCurve {
    pub value: f32,
    pub curve: crate::bindings::engine::FRuntimeFloatCurve,
    pub source_minimum: f32,
    pub source_maximum: f32,
    pub target_minimum: f32,
    pub target_maximum: f32,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_AnimRichCurve {
    pub curve: crate::bindings::engine::FRuntimeFloatCurve,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_GetDeltaTime {
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_GetWorldTime {
    pub year: f32,
    pub month: f32,
    pub day: f32,
    pub week_day: f32,
    pub hours: f32,
    pub minutes: f32,
    pub seconds: f32,
    pub overall_seconds: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_FramesToSeconds {
    pub frames: f32,
    pub seconds: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_SecondsToFrames {
    pub seconds: f32,
    pub frames: f32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugLineNoSpace {
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugLineStripNoSpace {
    pub points: TArray<crate::bindings::core_u_object::FVector>,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugPoint {
    pub vector: crate::bindings::core_u_object::FVector,
    pub mode: ERigUnitDebugPointMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub scale: f32,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugPointMutable {
    pub vector: crate::bindings::core_u_object::FVector,
    pub mode: ERigUnitDebugPointMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub scale: f32,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugRectangle {
    pub transform: crate::bindings::core_u_object::FTransform,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub scale: f32,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugRectangleNoSpace {
    pub transform: crate::bindings::core_u_object::FTransform,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub scale: f32,
    pub thickness: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugArc {
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
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugArcNoSpace {
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
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugBoxNoSpace {
    pub box_: crate::bindings::core_u_object::FBox,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugTransformMutableNoSpace {
    pub transform: crate::bindings::core_u_object::FTransform,
    pub mode: ERigUnitDebugTransformMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_DebugTransformArrayMutable_WorkData {
    pub draw_transforms: TArray<crate::bindings::core_u_object::FTransform>,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugTransformArrayMutableNoSpace {
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub parent_indices: TArray<i32>,
    pub mode: ERigUnitDebugTransformMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualDebugVector {
    pub value: crate::bindings::core_u_object::FVector,
    pub b_enabled: bool,
    pub mode: ERigUnitVisualDebugPointMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualDebugVectorNoSpace {
    pub value: crate::bindings::core_u_object::FVector,
    pub b_enabled: bool,
    pub mode: ERigUnitVisualDebugPointMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_VisualDebugQuat {
    pub value: crate::bindings::core_u_object::FQuat,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_VisualDebugQuatNoSpace {
    pub value: crate::bindings::core_u_object::FQuat,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_VisualDebugTransform {
    pub value: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_VisualDebugTransformNoSpace {
    pub value: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogBase {
    pub text: FString,
    pub category: FName,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogText {}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogObject {
    pub object_color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogLocation {
    pub location: crate::bindings::core_u_object::FVector,
    pub radius: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogWireframeOptional {
    pub b_wireframe: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogSphere {
    pub center: crate::bindings::core_u_object::FVector,
    pub radius: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogCone {
    pub origin: crate::bindings::core_u_object::FVector,
    pub direction: crate::bindings::core_u_object::FVector,
    pub length: f32,
    pub angle: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogCylinder {
    pub start: crate::bindings::core_u_object::FVector,
    pub end: crate::bindings::core_u_object::FVector,
    pub radius: f32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_VisualLogCapsule {
    pub base: crate::bindings::core_u_object::FVector,
    pub half_height: f32,
    pub radius: f32,
    pub rotation: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogBox {
    pub box_: crate::bindings::core_u_object::FBox,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_VisualLogOrientedBox {
    pub box_: crate::bindings::core_u_object::FBox,
    pub transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogArrow {
    pub segment_start: crate::bindings::core_u_object::FVector,
    pub segment_end: crate::bindings::core_u_object::FVector,
    pub arrow_head_size: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogCircle {
    pub center: crate::bindings::core_u_object::FVector,
    pub up_axis: crate::bindings::core_u_object::FVector,
    pub radius: f32,
    pub thickness: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogSegment {
    pub segment_start: crate::bindings::core_u_object::FVector,
    pub segment_end: crate::bindings::core_u_object::FVector,
    pub thickness: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_IsHostBeingDebugged {
    pub result: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_ForLoopCount {
    pub block_to_run: FName,
    pub count: i32,
    pub index: i32,
    pub ratio: f32,
    pub completed: FRigVMExecuteContext,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_Sequence {
    pub execute_context: FRigVMExecuteContext,
    pub a: FRigVMExecuteContext,
    pub b: FRigVMExecuteContext,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_UserDefinedEvent {
    pub execute_pin: FRigVMExecutePin,
    pub event_name: FName,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolConstant {
    pub value: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolUnaryOp {
    pub value: bool,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolBinaryOp {
    pub a: bool,
    pub b: bool,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolBinaryAggregateOp {
    pub a: bool,
    pub b: bool,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolMake {
    pub value: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolConstTrue {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolConstFalse {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolNot {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolAnd {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolNand {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolNand2 {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolOr {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolEquals {
    pub a: bool,
    pub b: bool,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolNotEquals {
    pub a: bool,
    pub b: bool,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolToggled {
    pub value: bool,
    pub toggled: bool,
    pub initialized: bool,
    pub last_value: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolFlipFlop {
    pub start_value: bool,
    pub duration: f32,
    pub result: bool,
    pub last_value: bool,
    pub time_left: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolOnce {
    pub duration: f32,
    pub result: bool,
    pub last_value: bool,
    pub time_left: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolToFloat {
    pub value: bool,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoolToInteger {
    pub value: bool,
    pub result: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxFromArray {
    pub array: TArray<crate::bindings::core_u_object::FVector>,
    pub box_: crate::bindings::core_u_object::FBox,
    pub minimum: crate::bindings::core_u_object::FVector,
    pub maximum: crate::bindings::core_u_object::FVector,
    pub center: crate::bindings::core_u_object::FVector,
    pub size: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxIsValid {
    pub box_: crate::bindings::core_u_object::FBox,
    pub valid: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxGetCenter {
    pub box_: crate::bindings::core_u_object::FBox,
    pub center: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxGetSize {
    pub box_: crate::bindings::core_u_object::FBox,
    pub size: crate::bindings::core_u_object::FVector,
    pub extent: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxShift {
    pub box_: crate::bindings::core_u_object::FBox,
    pub amount: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FBox,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxMoveTo {
    pub box_: crate::bindings::core_u_object::FBox,
    pub center: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FBox,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxExpand {
    pub box_: crate::bindings::core_u_object::FBox,
    pub amount: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FBox,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathBoxTransform {
    pub box_: crate::bindings::core_u_object::FBox,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FBox,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxGetDistance {
    pub box_: crate::bindings::core_u_object::FBox,
    pub position: crate::bindings::core_u_object::FVector,
    pub square: bool,
    pub valid: bool,
    pub distance: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxIsInside {
    pub box_: crate::bindings::core_u_object::FBox,
    pub position: crate::bindings::core_u_object::FVector,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxGetVolume {
    pub box_: crate::bindings::core_u_object::FBox,
    pub volume: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorBinaryOp {
    pub a: crate::bindings::core_u_object::FLinearColor,
    pub b: crate::bindings::core_u_object::FLinearColor,
    pub result: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorBinaryAggregateOp {
    pub a: crate::bindings::core_u_object::FLinearColor,
    pub b: crate::bindings::core_u_object::FLinearColor,
    pub result: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorMake {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub result: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorFromFloat {
    pub value: f32,
    pub result: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorFromDouble {
    pub value: f64,
    pub result: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorAdd {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorSub {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorMul {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorLerp {
    pub a: crate::bindings::core_u_object::FLinearColor,
    pub b: crate::bindings::core_u_object::FLinearColor,
    pub t: f32,
    pub result: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleConstant {
    pub value: f64,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleUnaryOp {
    pub value: f64,
    pub result: f64,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleBinaryOp {
    pub a: f64,
    pub b: f64,
    pub result: f64,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleBinaryAggregateOp {
    pub a: f64,
    pub b: f64,
    pub result: f64,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleMake {
    pub value: f64,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleConstPi {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleConstHalfPi {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleConstTwoPi {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleConstE {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleAdd {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleSub {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleMul {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleDiv {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleMod {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleMin {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleMax {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoublePow {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleSqrt {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleNegate {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleAbs {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleFloor {
    pub value: f64,
    pub result: f64,
    pub int: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleCeil {
    pub value: f64,
    pub result: f64,
    pub int: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleRound {
    pub value: f64,
    pub result: f64,
    pub int: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleToInt {
    pub value: f64,
    pub result: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleToFloat {
    pub value: f64,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleSign {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleClamp {
    pub value: f64,
    pub minimum: f64,
    pub maximum: f64,
    pub result: f64,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleLerp {
    pub a: f64,
    pub b: f64,
    pub t: f64,
    pub result: f64,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleRemap {
    pub value: f64,
    pub source_minimum: f64,
    pub source_maximum: f64,
    pub target_minimum: f64,
    pub target_maximum: f64,
    pub b_clamp: bool,
    pub result: f64,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleEquals {
    pub a: f64,
    pub b: f64,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleNotEquals {
    pub a: f64,
    pub b: f64,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleGreater {
    pub a: f64,
    pub b: f64,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleLess {
    pub a: f64,
    pub b: f64,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleGreaterEqual {
    pub a: f64,
    pub b: f64,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleLessEqual {
    pub a: f64,
    pub b: f64,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleIsNearlyZero {
    pub value: f64,
    pub tolerance: f64,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleIsNearlyEqual {
    pub a: f64,
    pub b: f64,
    pub tolerance: f64,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleDeg {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleRad {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleSin {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleCos {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleTan {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleAsin {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleAcos {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleAtan {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleAtan2 {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleLawOfCosine {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub alpha_angle: f64,
    pub beta_angle: f64,
    pub gamma_angle: f64,
    pub b_valid: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleExponential {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleArraySum {
    pub array: TArray<f64>,
    pub sum: f64,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDoubleArrayAverage {
    pub array: TArray<f64>,
    pub average: f64,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatConstant {
    pub value: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatUnaryOp {
    pub value: f32,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatBinaryOp {
    pub a: f32,
    pub b: f32,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatBinaryAggregateOp {
    pub a: f32,
    pub b: f32,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatMake {
    pub value: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatConstPi {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatConstHalfPi {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatConstTwoPi {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatConstE {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatAdd {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatSub {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatMul {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatDiv {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatMod {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatMin {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatMax {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatPow {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatSqrt {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatNegate {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatAbs {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatFloor {
    pub value: f32,
    pub result: f32,
    pub int: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatCeil {
    pub value: f32,
    pub result: f32,
    pub int: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatRound {
    pub value: f32,
    pub result: f32,
    pub int: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatToInt {
    pub value: f32,
    pub result: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatToDouble {
    pub value: f32,
    pub result: f64,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatSign {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatClamp {
    pub value: f32,
    pub minimum: f32,
    pub maximum: f32,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatLerp {
    pub a: f32,
    pub b: f32,
    pub t: f32,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatRemap {
    pub value: f32,
    pub source_minimum: f32,
    pub source_maximum: f32,
    pub target_minimum: f32,
    pub target_maximum: f32,
    pub b_clamp: bool,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatEquals {
    pub a: f32,
    pub b: f32,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatNotEquals {
    pub a: f32,
    pub b: f32,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatGreater {
    pub a: f32,
    pub b: f32,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatLess {
    pub a: f32,
    pub b: f32,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatGreaterEqual {
    pub a: f32,
    pub b: f32,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatLessEqual {
    pub a: f32,
    pub b: f32,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatIsNearlyZero {
    pub value: f32,
    pub tolerance: f32,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatIsNearlyEqual {
    pub a: f32,
    pub b: f32,
    pub tolerance: f32,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatSelectBool {
    pub condition: bool,
    pub if_true: f32,
    pub if_false: f32,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatDeg {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatRad {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatSin {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatCos {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatTan {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatAsin {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatAcos {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatAtan {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatAtan2 {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatLawOfCosine {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub alpha_angle: f32,
    pub beta_angle: f32,
    pub gamma_angle: f32,
    pub b_valid: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatExponential {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatArraySum {
    pub array: TArray<f32>,
    pub sum: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathFloatArrayAverage {
    pub array: TArray<f32>,
    pub average: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntUnaryOp {
    pub value: i32,
    pub result: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntBinaryOp {
    pub a: i32,
    pub b: i32,
    pub result: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntBinaryAggregateOp {
    pub a: i32,
    pub b: i32,
    pub result: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntMake {
    pub value: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntAdd {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntSub {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntMul {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntDiv {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntMod {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntMin {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntMax {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntPow {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntNegate {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntAbs {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntToFloat {
    pub value: i32,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntToDouble {
    pub value: i32,
    pub result: f64,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntSign {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntClamp {
    pub value: i32,
    pub minimum: i32,
    pub maximum: i32,
    pub result: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntEquals {
    pub a: i32,
    pub b: i32,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntNotEquals {
    pub a: i32,
    pub b: i32,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntGreater {
    pub a: i32,
    pub b: i32,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntLess {
    pub a: i32,
    pub b: i32,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntGreaterEqual {
    pub a: i32,
    pub b: i32,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntLessEqual {
    pub a: i32,
    pub b: i32,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntArraySum {
    pub array: TArray<i32>,
    pub sum: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntArrayAverage {
    pub array: TArray<i32>,
    pub average: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntToString {
    pub number: i32,
    pub padded_size: i32,
    pub result: FString,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntToName {
    pub number: i32,
    pub padded_size: i32,
    pub result: FName,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathMatrixBase {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixUnaryOp {
    pub value: crate::bindings::core_u_object::FMatrix,
    pub result: crate::bindings::core_u_object::FMatrix,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixBinaryOp {
    pub a: crate::bindings::core_u_object::FMatrix,
    pub b: crate::bindings::core_u_object::FMatrix,
    pub result: crate::bindings::core_u_object::FMatrix,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixBinaryAggregateOp {
    pub a: crate::bindings::core_u_object::FMatrix,
    pub b: crate::bindings::core_u_object::FMatrix,
    pub result: crate::bindings::core_u_object::FMatrix,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixToTransform {
    pub value: crate::bindings::core_u_object::FMatrix,
    pub result: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixFromTransform {
    pub transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FMatrix,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixFromTransformV2 {
    pub value: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FMatrix,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixToVectors {
    pub value: crate::bindings::core_u_object::FMatrix,
    pub origin: crate::bindings::core_u_object::FVector,
    pub x: crate::bindings::core_u_object::FVector,
    pub y: crate::bindings::core_u_object::FVector,
    pub z: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixFromVectors {
    pub origin: crate::bindings::core_u_object::FVector,
    pub x: crate::bindings::core_u_object::FVector,
    pub y: crate::bindings::core_u_object::FVector,
    pub z: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FMatrix,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixMul {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixInverse {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathQuaternionBase {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionUnaryOp {
    pub value: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionBinaryOp {
    pub a: crate::bindings::core_u_object::FQuat,
    pub b: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionBinaryAggregateOp {
    pub a: crate::bindings::core_u_object::FQuat,
    pub b: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionMake {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub result: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionFromAxisAndAngle {
    pub axis: crate::bindings::core_u_object::FVector,
    pub angle: f32,
    pub result: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionFromEuler {
    pub euler: crate::bindings::core_u_object::FVector,
    pub rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    pub result: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionFromRotator {
    pub rotator: crate::bindings::core_u_object::FRotator,
    pub result: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionFromRotatorV2 {
    pub value: crate::bindings::core_u_object::FRotator,
    pub result: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionFromTwoVectors {
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionToAxisAndAngle {
    pub value: crate::bindings::core_u_object::FQuat,
    pub axis: crate::bindings::core_u_object::FVector,
    pub angle: f32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionToVectors {
    pub value: crate::bindings::core_u_object::FQuat,
    pub forward: crate::bindings::core_u_object::FVector,
    pub right: crate::bindings::core_u_object::FVector,
    pub up: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionScale {
    pub value: crate::bindings::core_u_object::FQuat,
    pub scale: f32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionScaleV2 {
    pub value: crate::bindings::core_u_object::FQuat,
    pub factor: f32,
    pub result: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionToEuler {
    pub value: crate::bindings::core_u_object::FQuat,
    pub rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionToRotator {
    pub value: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FRotator,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionMul {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionInverse {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionSlerp {
    pub a: crate::bindings::core_u_object::FQuat,
    pub b: crate::bindings::core_u_object::FQuat,
    pub t: f32,
    pub result: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionEquals {
    pub a: crate::bindings::core_u_object::FQuat,
    pub b: crate::bindings::core_u_object::FQuat,
    pub result: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionNotEquals {
    pub a: crate::bindings::core_u_object::FQuat,
    pub b: crate::bindings::core_u_object::FQuat,
    pub result: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionSelectBool {
    pub condition: bool,
    pub if_true: crate::bindings::core_u_object::FQuat,
    pub if_false: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionDot {
    pub a: crate::bindings::core_u_object::FQuat,
    pub b: crate::bindings::core_u_object::FQuat,
    pub result: f32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionUnit {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionRotateVector {
    pub transform: crate::bindings::core_u_object::FQuat,
    pub vector: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionGetAxis {
    pub quaternion: crate::bindings::core_u_object::FQuat,
    pub axis: crate::bindings::core_u_object::EAxis,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionSwingTwist {
    pub input: crate::bindings::core_u_object::FQuat,
    pub twist_axis: crate::bindings::core_u_object::FVector,
    pub swing: crate::bindings::core_u_object::FQuat,
    pub twist: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathQuaternionRotationOrder {
    pub rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionMakeRelative {
    pub global: crate::bindings::core_u_object::FQuat,
    pub parent: crate::bindings::core_u_object::FQuat,
    pub local: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionMakeAbsolute {
    pub local: crate::bindings::core_u_object::FQuat,
    pub parent: crate::bindings::core_u_object::FQuat,
    pub global: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionMirrorTransform {
    pub value: crate::bindings::core_u_object::FQuat,
    pub mirror_axis: crate::bindings::core_u_object::EAxis,
    pub axis_to_flip: crate::bindings::core_u_object::EAxis,
    pub central_transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathRayBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathRayIntersectRay {
    pub a: crate::bindings::core_u_object::FRay,
    pub b: crate::bindings::core_u_object::FRay,
    pub result: crate::bindings::core_u_object::FVector,
    pub distance: f32,
    pub ratio_a: f32,
    pub ratio_b: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathRayIntersectPlane {
    pub ray: crate::bindings::core_u_object::FRay,
    pub plane_point: crate::bindings::core_u_object::FVector,
    pub plane_normal: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
    pub distance: f32,
    pub ratio: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathRayGetAt {
    pub ray: crate::bindings::core_u_object::FRay,
    pub ratio: f32,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRayTransform {
    pub ray: crate::bindings::core_u_object::FRay,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FRay,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateQuatWorkData {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateVectorWorkData {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathRBFInterpolateBase {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateQuatBase {
    pub input: crate::bindings::core_u_object::FQuat,
    pub distance_function: ERBFQuatDistanceType,
    pub smoothing_function: ERBFKernelType,
    pub smoothing_angle: f32,
    pub b_normalize_output: bool,
    pub twist_axis: crate::bindings::core_u_object::FVector,
    pub work_data: FRigVMFunction_MathRBFInterpolateQuatWorkData,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateVectorBase {
    pub input: crate::bindings::core_u_object::FVector,
    pub distance_function: ERBFVectorDistanceType,
    pub smoothing_function: ERBFKernelType,
    pub smoothing_radius: f32,
    pub b_normalize_output: bool,
    pub work_data: FRigVMFunction_MathRBFInterpolateVectorWorkData,
}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateQuatFloat_Target {
    pub target: crate::bindings::core_u_object::FQuat,
    pub value: f32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateQuatFloat {
    pub targets: TArray<FMathRBFInterpolateQuatFloat_Target>,
    pub output: f32,
}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateQuatVector_Target {
    pub target: crate::bindings::core_u_object::FQuat,
    pub value: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateQuatVector {
    pub targets: TArray<FMathRBFInterpolateQuatVector_Target>,
    pub output: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateQuatColor_Target {
    pub target: crate::bindings::core_u_object::FQuat,
    pub value: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateQuatColor {
    pub targets: TArray<FMathRBFInterpolateQuatColor_Target>,
    pub output: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateQuatQuat_Target {
    pub target: crate::bindings::core_u_object::FQuat,
    pub value: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateQuatQuat {
    pub targets: TArray<FMathRBFInterpolateQuatQuat_Target>,
    pub output: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateQuatXform_Target {
    pub target: crate::bindings::core_u_object::FQuat,
    pub value: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateQuatXform {
    pub targets: TArray<FMathRBFInterpolateQuatXform_Target>,
    pub output: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FMathRBFInterpolateVectorFloat_Target {
    pub target: crate::bindings::core_u_object::FVector,
    pub value: f32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateVectorFloat {
    pub targets: TArray<FMathRBFInterpolateVectorFloat_Target>,
    pub output: f32,
}
#[repr(C, align(8))]
pub struct FMathRBFInterpolateVectorVector_Target {
    pub target: crate::bindings::core_u_object::FVector,
    pub value: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateVectorVector {
    pub targets: TArray<FMathRBFInterpolateVectorVector_Target>,
    pub output: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FMathRBFInterpolateVectorColor_Target {
    pub target: crate::bindings::core_u_object::FVector,
    pub value: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateVectorColor {
    pub targets: TArray<FMathRBFInterpolateVectorColor_Target>,
    pub output: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateVectorQuat_Target {
    pub target: crate::bindings::core_u_object::FVector,
    pub value: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateVectorQuat {
    pub targets: TArray<FMathRBFInterpolateVectorQuat_Target>,
    pub output: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateVectorXform_Target {
    pub target: crate::bindings::core_u_object::FVector,
    pub value: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateVectorXform {
    pub targets: TArray<FMathRBFInterpolateVectorXform_Target>,
    pub output: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathTransformBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathTransformMutableBase {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformUnaryOp {
    pub value: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformBinaryOp {
    pub a: crate::bindings::core_u_object::FTransform,
    pub b: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformBinaryAggregateOp {
    pub a: crate::bindings::core_u_object::FTransform,
    pub b: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformMake {
    pub translation: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FQuat,
    pub scale: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformFromEulerTransform {
    pub euler_transform: crate::bindings::animation_core::FEulerTransform,
    pub result: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformFromEulerTransformV2 {
    pub value: crate::bindings::animation_core::FEulerTransform,
    pub result: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformToEulerTransform {
    pub value: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::animation_core::FEulerTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformToVectors {
    pub value: crate::bindings::core_u_object::FTransform,
    pub forward: crate::bindings::core_u_object::FVector,
    pub right: crate::bindings::core_u_object::FVector,
    pub up: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformMul {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformMakeRelative {
    pub global: crate::bindings::core_u_object::FTransform,
    pub parent: crate::bindings::core_u_object::FTransform,
    pub local: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformMakeAbsolute {
    pub local: crate::bindings::core_u_object::FTransform,
    pub parent: crate::bindings::core_u_object::FTransform,
    pub global: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformAccumulateArray {
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub target_space: ERigVMTransformSpace,
    pub root: crate::bindings::core_u_object::FTransform,
    pub parent_indices: TArray<i32>,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformInverse {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformLerp {
    pub a: crate::bindings::core_u_object::FTransform,
    pub b: crate::bindings::core_u_object::FTransform,
    pub t: f32,
    pub result: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformSelectBool {
    pub condition: bool,
    pub if_true: crate::bindings::core_u_object::FTransform,
    pub if_false: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformRotateVector {
    pub transform: crate::bindings::core_u_object::FTransform,
    pub vector: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformTransformVector {
    pub transform: crate::bindings::core_u_object::FTransform,
    pub location: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformFromSRT {
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FVector,
    pub rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    pub scale: crate::bindings::core_u_object::FVector,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub euler_transform: crate::bindings::animation_core::FEulerTransform,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathTransformArrayToSRT {
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub translations: TArray<crate::bindings::core_u_object::FVector>,
    pub rotations: TArray<crate::bindings::core_u_object::FQuat>,
    pub scales: TArray<crate::bindings::core_u_object::FVector>,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformClampSpatially {
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
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformMirrorTransform {
    pub value: crate::bindings::core_u_object::FTransform,
    pub mirror_axis: crate::bindings::core_u_object::EAxis,
    pub axis_to_flip: crate::bindings::core_u_object::EAxis,
    pub central_transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorUnaryOp {
    pub value: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorBinaryOp {
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorBinaryAggregateOp {
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMake {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorFromFloat {
    pub value: f32,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorFromDouble {
    pub value: f64,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorAdd {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorSub {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMul {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorScale {
    pub value: crate::bindings::core_u_object::FVector,
    pub factor: f32,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorDiv {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMod {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMin {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMax {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorNegate {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorAbs {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorFloor {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorCeil {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorRound {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorSign {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorClamp {
    pub value: crate::bindings::core_u_object::FVector,
    pub minimum: crate::bindings::core_u_object::FVector,
    pub maximum: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorLerp {
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub t: f32,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorRemap {
    pub value: crate::bindings::core_u_object::FVector,
    pub source_minimum: crate::bindings::core_u_object::FVector,
    pub source_maximum: crate::bindings::core_u_object::FVector,
    pub target_minimum: crate::bindings::core_u_object::FVector,
    pub target_maximum: crate::bindings::core_u_object::FVector,
    pub b_clamp: bool,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorEquals {
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorNotEquals {
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorIsNearlyZero {
    pub value: crate::bindings::core_u_object::FVector,
    pub tolerance: f32,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorIsNearlyEqual {
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub tolerance: f32,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorSelectBool {
    pub condition: bool,
    pub if_true: crate::bindings::core_u_object::FVector,
    pub if_false: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorDeg {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorRad {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorLengthSquared {
    pub value: crate::bindings::core_u_object::FVector,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorLength {
    pub value: crate::bindings::core_u_object::FVector,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorDistance {
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorCross {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorDot {
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorUnit {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorSetLength {
    pub value: crate::bindings::core_u_object::FVector,
    pub length: f32,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorClampLength {
    pub value: crate::bindings::core_u_object::FVector,
    pub minimum_length: f32,
    pub maximum_length: f32,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMirror {
    pub value: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorAngle {
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorParallel {
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorOrthogonal {
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorBezierFourPoint {
    pub bezier: FRigVMFourPointBezier,
    pub t: f32,
    pub result: crate::bindings::core_u_object::FVector,
    pub tangent: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFourPointBezier {
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub c: crate::bindings::core_u_object::FVector,
    pub d: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMakeBezierFourPoint {
    pub bezier: FRigVMFourPointBezier,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathVectorClampSpatially {
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
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntersectPlane {
    pub start: crate::bindings::core_u_object::FVector,
    pub direction: crate::bindings::core_u_object::FVector,
    pub plane_point: crate::bindings::core_u_object::FVector,
    pub plane_normal: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
    pub distance: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDistanceToPlane {
    pub point: crate::bindings::core_u_object::FVector,
    pub plane_point: crate::bindings::core_u_object::FVector,
    pub plane_normal: crate::bindings::core_u_object::FVector,
    pub closest_point_on_plane: crate::bindings::core_u_object::FVector,
    pub signed_distance: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMakeRelative {
    pub global: crate::bindings::core_u_object::FVector,
    pub parent: crate::bindings::core_u_object::FVector,
    pub local: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMakeAbsolute {
    pub local: crate::bindings::core_u_object::FVector,
    pub parent: crate::bindings::core_u_object::FVector,
    pub global: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathVectorMirrorTransform {
    pub value: crate::bindings::core_u_object::FVector,
    pub mirror_axis: crate::bindings::core_u_object::EAxis,
    pub axis_to_flip: crate::bindings::core_u_object::EAxis,
    pub central_transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorArraySum {
    pub array: TArray<crate::bindings::core_u_object::FVector>,
    pub sum: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorArrayAverage {
    pub array: TArray<crate::bindings::core_u_object::FVector>,
    pub average: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_NoiseFloat {
    pub value: f32,
    pub speed: f32,
    pub frequency: f32,
    pub minimum: f32,
    pub maximum: f32,
    pub result: f32,
    pub time: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_NoiseDouble {
    pub value: f64,
    pub speed: f64,
    pub frequency: f64,
    pub minimum: f64,
    pub maximum: f64,
    pub result: f64,
    pub time: f64,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_NoiseVector {
    pub position: crate::bindings::core_u_object::FVector,
    pub speed: crate::bindings::core_u_object::FVector,
    pub frequency: crate::bindings::core_u_object::FVector,
    pub minimum: f32,
    pub maximum: f32,
    pub result: crate::bindings::core_u_object::FVector,
    pub time: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_NoiseVector2 {
    pub value: crate::bindings::core_u_object::FVector,
    pub speed: crate::bindings::core_u_object::FVector,
    pub frequency: crate::bindings::core_u_object::FVector,
    pub minimum: f64,
    pub maximum: f64,
    pub result: crate::bindings::core_u_object::FVector,
    pub time: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_RandomFloat {
    pub seed: i32,
    pub minimum: f32,
    pub maximum: f32,
    pub duration: f32,
    pub result: f32,
    pub last_result: f32,
    pub last_seed: i32,
    pub base_seed: i32,
    pub time_left: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_RandomVector {
    pub seed: i32,
    pub minimum: f32,
    pub maximum: f32,
    pub duration: f32,
    pub result: crate::bindings::core_u_object::FVector,
    pub last_result: crate::bindings::core_u_object::FVector,
    pub last_seed: i32,
    pub base_seed: i32,
    pub time_left: f32,
}
#[repr(C, align(8))]
pub struct FRigVMMirrorSettings {
    pub mirror_axis: crate::bindings::core_u_object::EAxis,
    pub axis_to_flip: crate::bindings::core_u_object::EAxis,
    pub search_string: FString,
    pub replace_string: FString,
}
#[repr(C, align(8))]
pub struct FRigVMSimPoint {
    pub mass: f32,
    pub size: f32,
    pub linear_damping: f32,
    pub inherit_motion: f32,
    pub position: crate::bindings::core_u_object::FVector,
    pub linear_velocity: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVMDispatch_CoreBase {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_ArrayBase {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_ArrayBaseMutable {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_ArrayMake {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_ArrayReset {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_ArrayGetNum {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_ArraySetNum {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_ArrayInit {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_ArrayGetAtIndex {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_ArraySetAtIndex {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_ArrayAdd {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_ArrayInsert {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_ArrayRemove {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_ArrayReverse {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_ArrayFind {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_ArrayAppend {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_ArrayClone {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_ArrayUnion {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_ArrayDifference {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_ArrayIntersection {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_ArrayIterator {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_CastEnumToInt {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_CastIntToEnum {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_CastObject {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_Constant {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_CoreEquals {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_CoreNotEquals {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_If {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_MakeStruct {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_BreakStruct {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_Print {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_SelectInt32 {}
#[repr(C, align(8))]
pub struct FRigVMDispatch_SwitchInt32 {}
#[repr(C, align(8))]
pub struct FRigVMFunction_ControlFlowBase {}
#[repr(C, align(16))]
pub struct FRigVMFunction_ControlFlowBranch {
    pub execute_context: FRigVMExecuteContext,
    pub condition: bool,
    pub true_: FRigVMExecuteContext,
    pub false_: FRigVMExecuteContext,
    pub completed: FRigVMExecuteContext,
    pub block_to_run: FName,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_NameBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_NameConcat {
    pub a: FName,
    pub b: FName,
    pub result: FName,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_NameTruncate {
    pub name: FName,
    pub count: i32,
    pub from_end: bool,
    pub remainder: FName,
    pub chopped: FName,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_NameReplace {
    pub name: FName,
    pub old: FName,
    pub new: FName,
    pub result: FName,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_EndsWith {
    pub name: FName,
    pub ending: FName,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_StartsWith {
    pub name: FName,
    pub start: FName,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_Contains {
    pub name: FName,
    pub search: FName,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_IsNone {
    pub name: FName,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_IsNameValid {
    pub value: FName,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_GetNameNumericSuffix {
    pub name: FName,
    pub suffix: i32,
    pub success: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringConcat {
    pub a: FString,
    pub b: FString,
    pub result: FString,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringTruncate {
    pub name: FString,
    pub count: i32,
    pub from_end: bool,
    pub remainder: FString,
    pub chopped: FString,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringReplace {
    pub name: FString,
    pub old: FString,
    pub new: FString,
    pub result: FString,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringEndsWith {
    pub name: FString,
    pub ending: FString,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringStartsWith {
    pub name: FString,
    pub start: FString,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringContains {
    pub name: FString,
    pub search: FString,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringLength {
    pub value: FString,
    pub length: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringTrimWhitespace {
    pub value: FString,
    pub result: FString,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringToUppercase {
    pub value: FString,
    pub result: FString,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringToLowercase {
    pub value: FString,
    pub result: FString,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringReverse {
    pub value: FString,
    pub reverse: FString,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringLeft {
    pub value: FString,
    pub count: i32,
    pub result: FString,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringRight {
    pub value: FString,
    pub count: i32,
    pub result: FString,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringMiddle {
    pub value: FString,
    pub start: i32,
    pub count: i32,
    pub result: FString,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringFind {
    pub value: FString,
    pub search: FString,
    pub found: bool,
    pub index: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringSplit {
    pub value: FString,
    pub separator: FString,
    pub result: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringJoin {
    pub values: TArray<FString>,
    pub separator: FString,
    pub result: FString,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringPadInteger {
    pub value: i32,
    pub digits: i32,
    pub result: FString,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_StringToInteger {
    pub value: FString,
    pub chop_left: bool,
    pub chop_right: bool,
    pub result: i32,
    pub success: bool,
}
#[repr(C, align(8))]
pub struct FRigDispatch_ToString {}
#[repr(C, align(8))]
pub struct FRigDispatch_FromString {}
#[repr(C, align(8))]
pub struct FRigVMFunction_AccumulateBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_AccumulateFloatAdd {
    pub increment: f32,
    pub initial_value: f32,
    pub b_integrate_delta_time: bool,
    pub result: f32,
    pub accumulated_value: f32,
    pub b_is_initialized: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_AccumulateVectorAdd {
    pub increment: crate::bindings::core_u_object::FVector,
    pub initial_value: crate::bindings::core_u_object::FVector,
    pub b_integrate_delta_time: bool,
    pub result: crate::bindings::core_u_object::FVector,
    pub accumulated_value: crate::bindings::core_u_object::FVector,
    pub b_is_initialized: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_AccumulateFloatMul {
    pub multiplier: f32,
    pub initial_value: f32,
    pub b_integrate_delta_time: bool,
    pub result: f32,
    pub accumulated_value: f32,
    pub b_is_initialized: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_AccumulateVectorMul {
    pub multiplier: crate::bindings::core_u_object::FVector,
    pub initial_value: crate::bindings::core_u_object::FVector,
    pub b_integrate_delta_time: bool,
    pub result: crate::bindings::core_u_object::FVector,
    pub accumulated_value: crate::bindings::core_u_object::FVector,
    pub b_is_initialized: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_AccumulateQuatMul {
    pub multiplier: crate::bindings::core_u_object::FQuat,
    pub initial_value: crate::bindings::core_u_object::FQuat,
    pub b_flip_order: bool,
    pub b_integrate_delta_time: bool,
    pub result: crate::bindings::core_u_object::FQuat,
    pub accumulated_value: crate::bindings::core_u_object::FQuat,
    pub b_is_initialized: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_AccumulateTransformMul {
    pub multiplier: crate::bindings::core_u_object::FTransform,
    pub initial_value: crate::bindings::core_u_object::FTransform,
    pub b_flip_order: bool,
    pub b_integrate_delta_time: bool,
    pub result: crate::bindings::core_u_object::FTransform,
    pub accumulated_value: crate::bindings::core_u_object::FTransform,
    pub b_is_initialized: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_AccumulateFloatLerp {
    pub target_value: f32,
    pub initial_value: f32,
    pub blend: f32,
    pub b_integrate_delta_time: bool,
    pub result: f32,
    pub accumulated_value: f32,
    pub b_is_initialized: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_AccumulateVectorLerp {
    pub target_value: crate::bindings::core_u_object::FVector,
    pub initial_value: crate::bindings::core_u_object::FVector,
    pub blend: f32,
    pub b_integrate_delta_time: bool,
    pub result: crate::bindings::core_u_object::FVector,
    pub accumulated_value: crate::bindings::core_u_object::FVector,
    pub b_is_initialized: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_AccumulateQuatLerp {
    pub target_value: crate::bindings::core_u_object::FQuat,
    pub initial_value: crate::bindings::core_u_object::FQuat,
    pub blend: f32,
    pub b_integrate_delta_time: bool,
    pub result: crate::bindings::core_u_object::FQuat,
    pub accumulated_value: crate::bindings::core_u_object::FQuat,
    pub b_is_initialized: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_AccumulateTransformLerp {
    pub target_value: crate::bindings::core_u_object::FTransform,
    pub initial_value: crate::bindings::core_u_object::FTransform,
    pub blend: f32,
    pub b_integrate_delta_time: bool,
    pub result: crate::bindings::core_u_object::FTransform,
    pub accumulated_value: crate::bindings::core_u_object::FTransform,
    pub b_is_initialized: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_AccumulateFloatRange {
    pub value: f32,
    pub minimum: f32,
    pub maximum: f32,
    pub accumulated_minimum: f32,
    pub accumulated_maximum: f32,
    pub b_is_initialized: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_AccumulateVectorRange {
    pub value: crate::bindings::core_u_object::FVector,
    pub minimum: crate::bindings::core_u_object::FVector,
    pub maximum: crate::bindings::core_u_object::FVector,
    pub accumulated_minimum: crate::bindings::core_u_object::FVector,
    pub accumulated_maximum: crate::bindings::core_u_object::FVector,
    pub b_is_initialized: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_AlphaInterp {
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
    pub scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_AlphaInterpVector {
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
    pub scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_AlphaInterpQuat {
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
    pub scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_DeltaFromPreviousFloat {
    pub value: f32,
    pub delta: f32,
    pub previous_value: f32,
    pub cache: f32,
    pub b_is_initialized: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_DeltaFromPreviousVector {
    pub value: crate::bindings::core_u_object::FVector,
    pub delta: crate::bindings::core_u_object::FVector,
    pub previous_value: crate::bindings::core_u_object::FVector,
    pub cache: crate::bindings::core_u_object::FVector,
    pub b_is_initialized: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DeltaFromPreviousQuat {
    pub value: crate::bindings::core_u_object::FQuat,
    pub delta: crate::bindings::core_u_object::FQuat,
    pub previous_value: crate::bindings::core_u_object::FQuat,
    pub cache: crate::bindings::core_u_object::FQuat,
    pub b_is_initialized: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DeltaFromPreviousTransform {
    pub value: crate::bindings::core_u_object::FTransform,
    pub delta: crate::bindings::core_u_object::FTransform,
    pub previous_value: crate::bindings::core_u_object::FTransform,
    pub cache: crate::bindings::core_u_object::FTransform,
    pub b_is_initialized: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_KalmanFloat {
    pub value: f32,
    pub buffer_size: i32,
    pub result: f32,
    pub buffer: TArray<f32>,
    pub last_insert_index: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_KalmanVector {
    pub value: crate::bindings::core_u_object::FVector,
    pub buffer_size: i32,
    pub result: crate::bindings::core_u_object::FVector,
    pub buffer: TArray<crate::bindings::core_u_object::FVector>,
    pub last_insert_index: i32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_KalmanTransform {
    pub value: crate::bindings::core_u_object::FTransform,
    pub buffer_size: i32,
    pub result: crate::bindings::core_u_object::FTransform,
    pub buffer: TArray<crate::bindings::core_u_object::FTransform>,
    pub last_insert_index: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_Timeline {
    pub speed: f32,
    pub time: f32,
    pub accumulated_value: f32,
    pub b_is_initialized: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_TimeLoop {
    pub speed: f32,
    pub duration: f32,
    pub normalize: bool,
    pub absolute: f32,
    pub relative: f32,
    pub flip_flop: f32,
    pub even: bool,
    pub accumulated_absolute: f32,
    pub accumulated_relative: f32,
    pub num_iterations: i32,
    pub b_is_initialized: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_TimeOffsetFloat {
    pub value: f32,
    pub seconds_ago: f32,
    pub buffer_size: i32,
    pub time_range: f32,
    pub result: f32,
    pub buffer: TArray<f32>,
    pub delta_times: TArray<f32>,
    pub last_insert_index: i32,
    pub upper_bound: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_TimeOffsetVector {
    pub value: crate::bindings::core_u_object::FVector,
    pub seconds_ago: f32,
    pub buffer_size: i32,
    pub time_range: f32,
    pub result: crate::bindings::core_u_object::FVector,
    pub buffer: TArray<crate::bindings::core_u_object::FVector>,
    pub delta_times: TArray<f32>,
    pub last_insert_index: i32,
    pub upper_bound: i32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_TimeOffsetTransform {
    pub value: crate::bindings::core_u_object::FTransform,
    pub seconds_ago: f32,
    pub buffer_size: i32,
    pub time_range: f32,
    pub result: crate::bindings::core_u_object::FTransform,
    pub buffer: TArray<crate::bindings::core_u_object::FTransform>,
    pub delta_times: TArray<f32>,
    pub last_insert_index: i32,
    pub upper_bound: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VerletIntegrateVector {
    pub target: crate::bindings::core_u_object::FVector,
    pub strength: f32,
    pub damp: f32,
    pub blend: f32,
    pub force: crate::bindings::core_u_object::FVector,
    pub position: crate::bindings::core_u_object::FVector,
    pub velocity: crate::bindings::core_u_object::FVector,
    pub acceleration: crate::bindings::core_u_object::FVector,
    pub point: FRigVMSimPoint,
    pub b_initialized: bool,
}
#[repr(C, align(8))]
pub struct FRigVMUserDefinedTypesInfo {
    pub struct_guid_to_path_name: TMap<
        FString,
        crate::bindings::core_u_object::FSoftObjectPath,
    >,
    pub enum_to_path_name: TMap<
        FString,
        crate::bindings::core_u_object::FSoftObjectPath,
    >,
    pub types_in_use: TSet<UPtr<crate::bindings::core_u_object::UObject>>,
}
pub struct URigVMBlueprintGeneratedClass {
    pub graph_function_store: FRigVMGraphFunctionStore,
    pub supported_event_names: TArray<FName>,
    pub asset_variant: FRigVMVariant,
}
pub struct URigVM {
    pub literal_memory_storage: FRigVMMemoryStorageStruct,
    pub default_work_memory_storage: FRigVMMemoryStorageStruct,
    pub default_debug_memory_storage: FRigVMMemoryStorageStruct,
    pub work_memory_storage_object_deprecated: UPtr<URigVMMemoryStorage>,
    pub literal_memory_storage_object_deprecated: UPtr<URigVMMemoryStorage>,
    pub debug_memory_storage_object_deprecated: UPtr<URigVMMemoryStorage>,
    pub byte_code_storage: FRigVMByteCode,
    pub instructions: FRigVMInstructionArray,
    pub num_executions_deprecated: u32,
    pub function_names_storage: TArray<FName>,
    pub parameters: TArray<FRigVMParameter>,
    pub cached_vm_hash: u32,
}
pub struct UNameSpacedUserData {
    pub name_space: FString,
}
pub struct UDataAssetLink {
    pub data_asset: TSoftObjectPtr<crate::bindings::engine::UDataAsset>,
    pub data_asset_cached: UPtr<crate::bindings::engine::UDataAsset>,
}
pub struct URigVMGraphFunctionHost {}
pub struct IRigVMGraphFunctionHost {}
pub struct URigVMMemoryStorageGeneratorClass {}
pub struct URigVMMemoryStorage {}
pub struct URigVMNativized {}
pub struct URigVMUserWorkflowOptions {
    pub subject: UPtr<crate::bindings::core_u_object::UObject>,
    pub workflow: FRigVMUserWorkflow,
}
pub struct URigVMHost {
    pub vm_runtime_settings: FRigVMRuntimeSettings,
    pub vm: UPtr<URigVM>,
    pub user_defined_struct_guid_to_path_name: TMap<
        FString,
        crate::bindings::core_u_object::FSoftObjectPath,
    >,
    pub user_defined_enum_to_path_name: TMap<
        FString,
        crate::bindings::core_u_object::FSoftObjectPath,
    >,
    pub user_defined_types_in_use: TSet<UPtr<crate::bindings::core_u_object::UObject>>,
    pub extended_execute_context_deprecated: FRigVMExtendedExecuteContext,
    pub draw_container: FRigVMDrawContainer,
    pub event_queue: TArray<FName>,
    pub asset_user_data: TArray<UPtr<crate::bindings::engine::UAssetUserData>>,
    pub asset_user_data_editor_only: TArray<
        UPtr<crate::bindings::engine::UAssetUserData>,
    >,
    pub b_is_in_debug_mode: bool,
}
pub struct URigVMEditorSettings {
    pub b_highlight_similar_nodes: bool,
    pub b_fade_out_unrelated_nodes: bool,
    pub b_use_flash_light: bool,
    pub b_auto_link_mutable_nodes: bool,
    pub b_enable_context_menu_time_slicing: bool,
}
pub struct URigVMProjectSettings {
    pub variant_tags: TArray<FRigVMTag>,
}
pub struct FRigVMUserWorkflow_PerformDynamicDelegate;
pub struct FGetSupportedWorkflows_PerformDynamicDelegate;
pub struct FGetWorkflows_PerformDynamicDelegate;
pub struct FMakeOptionsForWorkflow_PerformDynamicDelegate;
pub struct FPerformUserWorkflow_PerformDynamicDelegate;
pub struct FProvideWorkflows_PerformDynamicDelegate;
pub struct FRigVMUserWorkflowOptions_PerformDynamicDelegate;
pub struct FRigVMUserWorkflowProvider__DelegateSignature_PerformDynamicDelegate;
pub struct FCallPython_PerformDynamicDelegate;
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigVMDrawSettings(pub u8);
impl ERigVMDrawSettings {
    pub const POINTS: ERigVMDrawSettings = ERigVMDrawSettings(0);
    pub const LINES: ERigVMDrawSettings = ERigVMDrawSettings(1);
    pub const LINE_STRIP: ERigVMDrawSettings = ERigVMDrawSettings(2);
    pub const DYNAMIC_MESH: ERigVMDrawSettings = ERigVMDrawSettings(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigVMParameterType(pub u8);
impl ERigVMParameterType {
    pub const INPUT: ERigVMParameterType = ERigVMParameterType(0);
    pub const OUTPUT: ERigVMParameterType = ERigVMParameterType(1);
    pub const INVALID: ERigVMParameterType = ERigVMParameterType(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigVMMemoryType(pub u8);
impl ERigVMMemoryType {
    pub const WORK: ERigVMMemoryType = ERigVMMemoryType(0);
    pub const LITERAL: ERigVMMemoryType = ERigVMMemoryType(1);
    pub const EXTERNAL: ERigVMMemoryType = ERigVMMemoryType(2);
    pub const DEBUG: ERigVMMemoryType = ERigVMMemoryType(3);
    pub const INVALID: ERigVMMemoryType = ERigVMMemoryType(4);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigVMRegisterType(pub u8);
impl ERigVMRegisterType {
    pub const PLAIN: ERigVMRegisterType = ERigVMRegisterType(0);
    pub const STRING: ERigVMRegisterType = ERigVMRegisterType(1);
    pub const NAME: ERigVMRegisterType = ERigVMRegisterType(2);
    pub const STRUCT: ERigVMRegisterType = ERigVMRegisterType(3);
    pub const INVALID: ERigVMRegisterType = ERigVMRegisterType(4);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigUnitDebugPointMode(pub u8);
impl ERigUnitDebugPointMode {
    pub const POINT: ERigUnitDebugPointMode = ERigUnitDebugPointMode(0);
    pub const VECTOR: ERigUnitDebugPointMode = ERigUnitDebugPointMode(1);
    pub const MAX: ERigUnitDebugPointMode = ERigUnitDebugPointMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigUnitDebugTransformMode(pub u8);
impl ERigUnitDebugTransformMode {
    pub const POINT: ERigUnitDebugTransformMode = ERigUnitDebugTransformMode(0);
    pub const AXES: ERigUnitDebugTransformMode = ERigUnitDebugTransformMode(1);
    pub const BOX: ERigUnitDebugTransformMode = ERigUnitDebugTransformMode(2);
    pub const MAX: ERigUnitDebugTransformMode = ERigUnitDebugTransformMode(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigUnitVisualDebugPointMode(pub u8);
impl ERigUnitVisualDebugPointMode {
    pub const POINT: ERigUnitVisualDebugPointMode = ERigUnitVisualDebugPointMode(0);
    pub const VECTOR: ERigUnitVisualDebugPointMode = ERigUnitVisualDebugPointMode(1);
    pub const MAX: ERigUnitVisualDebugPointMode = ERigUnitVisualDebugPointMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERBFQuatDistanceType(pub u8);
impl ERBFQuatDistanceType {
    pub const EUCLIDEAN: ERBFQuatDistanceType = ERBFQuatDistanceType(0);
    pub const ARC_LENGTH: ERBFQuatDistanceType = ERBFQuatDistanceType(1);
    pub const SWING_ANGLE: ERBFQuatDistanceType = ERBFQuatDistanceType(2);
    pub const TWIST_ANGLE: ERBFQuatDistanceType = ERBFQuatDistanceType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERBFKernelType(pub u8);
impl ERBFKernelType {
    pub const GAUSSIAN: ERBFKernelType = ERBFKernelType(0);
    pub const EXPONENTIAL: ERBFKernelType = ERBFKernelType(1);
    pub const LINEAR: ERBFKernelType = ERBFKernelType(2);
    pub const CUBIC: ERBFKernelType = ERBFKernelType(3);
    pub const QUINTIC: ERBFKernelType = ERBFKernelType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERBFVectorDistanceType(pub u8);
impl ERBFVectorDistanceType {
    pub const EUCLIDEAN: ERBFVectorDistanceType = ERBFVectorDistanceType(0);
    pub const MANHATTAN: ERBFVectorDistanceType = ERBFVectorDistanceType(1);
    pub const ARC_LENGTH: ERBFVectorDistanceType = ERBFVectorDistanceType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigVMTransformSpace(pub u8);
impl ERigVMTransformSpace {
    pub const LOCAL_SPACE: ERigVMTransformSpace = ERigVMTransformSpace(0);
    pub const GLOBAL_SPACE: ERigVMTransformSpace = ERigVMTransformSpace(1);
    pub const MAX: ERigVMTransformSpace = ERigVMTransformSpace(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigVMClampSpatialMode(pub u8);
impl ERigVMClampSpatialMode {
    pub const PLANE: ERigVMClampSpatialMode = ERigVMClampSpatialMode(0);
    pub const CYLINDER: ERigVMClampSpatialMode = ERigVMClampSpatialMode(1);
    pub const SPHERE: ERigVMClampSpatialMode = ERigVMClampSpatialMode(2);
    pub const CAPSULE: ERigVMClampSpatialMode = ERigVMClampSpatialMode(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigVMSimPointIntegrateType(pub u8);
impl ERigVMSimPointIntegrateType {
    pub const VERLET: ERigVMSimPointIntegrateType = ERigVMSimPointIntegrateType(0);
    pub const SEMI_EXPLICIT_EULER: ERigVMSimPointIntegrateType = ERigVMSimPointIntegrateType(
        1,
    );
}
