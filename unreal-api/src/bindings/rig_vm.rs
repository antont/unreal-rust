#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FRigVMUserWorkflow {
    pub title: FString,
    pub tooltip: FString,
    pub ty: ERigVMUserWorkflowType,
    pub perform_dynamic_delegate: FRigVMUserWorkflow_PerformDynamicDelegate,
    pub options_class: TSubclassOf<UObject>,
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
    pub positions: TArray<FVector>,
    pub color: FLinearColor,
    pub thickness: f32,
    pub transform: FTransform,
    pub depth_priority: ESceneDepthPriorityGroup,
    pub lifetime: f32,
}
#[repr(C, align(8))]
pub struct FRigVMRuntimeSettings {
    pub maximum_array_size: i32,
    pub b_enable_profiling: bool,
}
#[repr(C, align(4))]
pub struct FRigVMDebugDrawSettings {
    pub depth_priority: ESceneDepthPriorityGroup,
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
    pub value: FVector,
    pub target: FVector,
    pub smoothing_time: f32,
    pub result: FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DampQuaternion {
    pub value: FQuat,
    pub target: FQuat,
    pub smoothing_time: f32,
    pub result: FQuat,
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
    pub value: FVector,
    pub value_velocity: FVector,
    pub target: FVector,
    pub smoothing_time: f32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_SpringDampQuat {
    pub value: FQuat,
    pub value_velocity: FVector,
    pub target: FQuat,
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
    pub script_struct: UPtr<UScriptStruct>,
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
    pub public_context_asset_path: FTopLevelAssetPath,
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
    pub cpp_type_object: TSoftObjectPtr<UObject>,
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
    pub cpp_type_object: TSoftObjectPtr<UObject>,
    pub b_is_array: bool,
    pub direction: ERigVMPinDirection,
    pub default_value: FString,
    pub b_is_const: bool,
    pub path_to_tooltip: TMap<FString, FText>,
}
#[repr(C, align(8))]
pub struct FRigVMGraphFunctionIdentifier {
    pub library_node_deprecated: FSoftObjectPath,
    pub library_node_path: FString,
    pub host_object: FSoftObjectPath,
}
#[repr(C, align(8))]
pub struct FRigVMGraphFunctionHeader {
    pub library_pointer: FRigVMGraphFunctionIdentifier,
    pub variant: FRigVMVariant,
    pub name: FName,
    pub node_title: FString,
    pub node_color: FLinearColor,
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
    pub guid: FGuid,
    pub tags: TArray<FRigVMTag>,
}
#[repr(C, align(8))]
pub struct FRigVMTag {
    pub name: FName,
    pub label: FString,
    pub tool_tip: FText,
    pub color: FLinearColor,
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
    pub base_cpp_type_object: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FRigVMRegisterOffset {
    pub segments: TArray<i32>,
    pub ty: ERigVMRegisterType,
    pub cpp_type: FName,
    pub script_struct: UPtr<UScriptStruct>,
    pub parent_script_struct: UPtr<UScriptStruct>,
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
    pub script_structs: TArray<UPtr<UScriptStruct>>,
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
    pub cpp_type_object: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FRigVMVariantRef {
    pub object_path: FSoftObjectPath,
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
    pub curve: FRuntimeFloatCurve,
    pub source_minimum: f32,
    pub source_maximum: f32,
    pub target_minimum: f32,
    pub target_maximum: f32,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_AnimRichCurve {
    pub curve: FRuntimeFloatCurve,
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
    pub a: FVector,
    pub b: FVector,
    pub color: FLinearColor,
    pub thickness: f32,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugLineStripNoSpace {
    pub points: TArray<FVector>,
    pub color: FLinearColor,
    pub thickness: f32,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugPoint {
    pub vector: FVector,
    pub mode: ERigUnitDebugPointMode,
    pub color: FLinearColor,
    pub scale: f32,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugPointMutable {
    pub vector: FVector,
    pub mode: ERigUnitDebugPointMode,
    pub color: FLinearColor,
    pub scale: f32,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugRectangle {
    pub transform: FTransform,
    pub color: FLinearColor,
    pub scale: f32,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugRectangleNoSpace {
    pub transform: FTransform,
    pub color: FLinearColor,
    pub scale: f32,
    pub thickness: f32,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugArc {
    pub transform: FTransform,
    pub color: FLinearColor,
    pub radius: f32,
    pub minimum_degrees: f32,
    pub maximum_degrees: f32,
    pub thickness: f32,
    pub detail: i32,
    pub space: FName,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugArcNoSpace {
    pub transform: FTransform,
    pub color: FLinearColor,
    pub radius: f32,
    pub minimum_degrees: f32,
    pub maximum_degrees: f32,
    pub thickness: f32,
    pub detail: i32,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugBoxNoSpace {
    pub box_: FBox,
    pub color: FLinearColor,
    pub thickness: f32,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugTransformMutableNoSpace {
    pub transform: FTransform,
    pub mode: ERigUnitDebugTransformMode,
    pub color: FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_DebugTransformArrayMutable_WorkData {
    pub draw_transforms: TArray<FTransform>,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DebugTransformArrayMutableNoSpace {
    pub transforms: TArray<FTransform>,
    pub parent_indices: TArray<i32>,
    pub mode: ERigUnitDebugTransformMode,
    pub color: FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualDebugVector {
    pub value: FVector,
    pub b_enabled: bool,
    pub mode: ERigUnitVisualDebugPointMode,
    pub color: FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualDebugVectorNoSpace {
    pub value: FVector,
    pub b_enabled: bool,
    pub mode: ERigUnitVisualDebugPointMode,
    pub color: FLinearColor,
    pub thickness: f32,
    pub scale: f32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_VisualDebugQuat {
    pub value: FQuat,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_VisualDebugQuatNoSpace {
    pub value: FQuat,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_VisualDebugTransform {
    pub value: FTransform,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_VisualDebugTransformNoSpace {
    pub value: FTransform,
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
    pub object_color: FLinearColor,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogLocation {
    pub location: FVector,
    pub radius: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogWireframeOptional {
    pub b_wireframe: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogSphere {
    pub center: FVector,
    pub radius: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogCone {
    pub origin: FVector,
    pub direction: FVector,
    pub length: f32,
    pub angle: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogCylinder {
    pub start: FVector,
    pub end: FVector,
    pub radius: f32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_VisualLogCapsule {
    pub base: FVector,
    pub half_height: f32,
    pub radius: f32,
    pub rotation: FQuat,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogBox {
    pub box_: FBox,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_VisualLogOrientedBox {
    pub box_: FBox,
    pub transform: FTransform,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogArrow {
    pub segment_start: FVector,
    pub segment_end: FVector,
    pub arrow_head_size: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogCircle {
    pub center: FVector,
    pub up_axis: FVector,
    pub radius: f32,
    pub thickness: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VisualLogSegment {
    pub segment_start: FVector,
    pub segment_end: FVector,
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
    pub array: TArray<FVector>,
    pub box_: FBox,
    pub minimum: FVector,
    pub maximum: FVector,
    pub center: FVector,
    pub size: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxIsValid {
    pub box_: FBox,
    pub valid: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxGetCenter {
    pub box_: FBox,
    pub center: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxGetSize {
    pub box_: FBox,
    pub size: FVector,
    pub extent: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxShift {
    pub box_: FBox,
    pub amount: FVector,
    pub result: FBox,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxMoveTo {
    pub box_: FBox,
    pub center: FVector,
    pub result: FBox,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxExpand {
    pub box_: FBox,
    pub amount: FVector,
    pub result: FBox,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathBoxTransform {
    pub box_: FBox,
    pub transform: FTransform,
    pub result: FBox,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxGetDistance {
    pub box_: FBox,
    pub position: FVector,
    pub square: bool,
    pub valid: bool,
    pub distance: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxIsInside {
    pub box_: FBox,
    pub position: FVector,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathBoxGetVolume {
    pub box_: FBox,
    pub volume: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorBinaryOp {
    pub a: FLinearColor,
    pub b: FLinearColor,
    pub result: FLinearColor,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorBinaryAggregateOp {
    pub a: FLinearColor,
    pub b: FLinearColor,
    pub result: FLinearColor,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorMake {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub result: FLinearColor,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorFromFloat {
    pub value: f32,
    pub result: FLinearColor,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorFromDouble {
    pub value: f64,
    pub result: FLinearColor,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorAdd {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorSub {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorMul {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathColorLerp {
    pub a: FLinearColor,
    pub b: FLinearColor,
    pub t: f32,
    pub result: FLinearColor,
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
    pub value: FMatrix,
    pub result: FMatrix,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixBinaryOp {
    pub a: FMatrix,
    pub b: FMatrix,
    pub result: FMatrix,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixBinaryAggregateOp {
    pub a: FMatrix,
    pub b: FMatrix,
    pub result: FMatrix,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixToTransform {
    pub value: FMatrix,
    pub result: FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixFromTransform {
    pub transform: FTransform,
    pub result: FMatrix,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixFromTransformV2 {
    pub value: FTransform,
    pub result: FMatrix,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixToVectors {
    pub value: FMatrix,
    pub origin: FVector,
    pub x: FVector,
    pub y: FVector,
    pub z: FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixFromVectors {
    pub origin: FVector,
    pub x: FVector,
    pub y: FVector,
    pub z: FVector,
    pub result: FMatrix,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixMul {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathMatrixInverse {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathQuaternionBase {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionUnaryOp {
    pub value: FQuat,
    pub result: FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionBinaryOp {
    pub a: FQuat,
    pub b: FQuat,
    pub result: FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionBinaryAggregateOp {
    pub a: FQuat,
    pub b: FQuat,
    pub result: FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionMake {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub result: FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionFromAxisAndAngle {
    pub axis: FVector,
    pub angle: f32,
    pub result: FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionFromEuler {
    pub euler: FVector,
    pub rotation_order: EEulerRotationOrder,
    pub result: FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionFromRotator {
    pub rotator: FRotator,
    pub result: FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionFromRotatorV2 {
    pub value: FRotator,
    pub result: FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionFromTwoVectors {
    pub a: FVector,
    pub b: FVector,
    pub result: FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionToAxisAndAngle {
    pub value: FQuat,
    pub axis: FVector,
    pub angle: f32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionToVectors {
    pub value: FQuat,
    pub forward: FVector,
    pub right: FVector,
    pub up: FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionScale {
    pub value: FQuat,
    pub scale: f32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionScaleV2 {
    pub value: FQuat,
    pub factor: f32,
    pub result: FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionToEuler {
    pub value: FQuat,
    pub rotation_order: EEulerRotationOrder,
    pub result: FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionToRotator {
    pub value: FQuat,
    pub result: FRotator,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionMul {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionInverse {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionSlerp {
    pub a: FQuat,
    pub b: FQuat,
    pub t: f32,
    pub result: FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionEquals {
    pub a: FQuat,
    pub b: FQuat,
    pub result: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionNotEquals {
    pub a: FQuat,
    pub b: FQuat,
    pub result: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionSelectBool {
    pub condition: bool,
    pub if_true: FQuat,
    pub if_false: FQuat,
    pub result: FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionDot {
    pub a: FQuat,
    pub b: FQuat,
    pub result: f32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionUnit {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionRotateVector {
    pub transform: FQuat,
    pub vector: FVector,
    pub result: FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionGetAxis {
    pub quaternion: FQuat,
    pub axis: EAxis,
    pub result: FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionSwingTwist {
    pub input: FQuat,
    pub twist_axis: FVector,
    pub swing: FQuat,
    pub twist: FQuat,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathQuaternionRotationOrder {
    pub rotation_order: EEulerRotationOrder,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionMakeRelative {
    pub global: FQuat,
    pub parent: FQuat,
    pub local: FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionMakeAbsolute {
    pub local: FQuat,
    pub parent: FQuat,
    pub global: FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathQuaternionMirrorTransform {
    pub value: FQuat,
    pub mirror_axis: EAxis,
    pub axis_to_flip: EAxis,
    pub central_transform: FTransform,
    pub result: FQuat,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathRayBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathRayIntersectRay {
    pub a: FRay,
    pub b: FRay,
    pub result: FVector,
    pub distance: f32,
    pub ratio_a: f32,
    pub ratio_b: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathRayIntersectPlane {
    pub ray: FRay,
    pub plane_point: FVector,
    pub plane_normal: FVector,
    pub result: FVector,
    pub distance: f32,
    pub ratio: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathRayGetAt {
    pub ray: FRay,
    pub ratio: f32,
    pub result: FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRayTransform {
    pub ray: FRay,
    pub transform: FTransform,
    pub result: FRay,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateQuatWorkData {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateVectorWorkData {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathRBFInterpolateBase {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateQuatBase {
    pub input: FQuat,
    pub distance_function: ERBFQuatDistanceType,
    pub smoothing_function: ERBFKernelType,
    pub smoothing_angle: f32,
    pub b_normalize_output: bool,
    pub twist_axis: FVector,
    pub work_data: FRigVMFunction_MathRBFInterpolateQuatWorkData,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateVectorBase {
    pub input: FVector,
    pub distance_function: ERBFVectorDistanceType,
    pub smoothing_function: ERBFKernelType,
    pub smoothing_radius: f32,
    pub b_normalize_output: bool,
    pub work_data: FRigVMFunction_MathRBFInterpolateVectorWorkData,
}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateQuatFloat_Target {
    pub target: FQuat,
    pub value: f32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateQuatFloat {
    pub targets: TArray<FMathRBFInterpolateQuatFloat_Target>,
    pub output: f32,
}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateQuatVector_Target {
    pub target: FQuat,
    pub value: FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateQuatVector {
    pub targets: TArray<FMathRBFInterpolateQuatVector_Target>,
    pub output: FVector,
}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateQuatColor_Target {
    pub target: FQuat,
    pub value: FLinearColor,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateQuatColor {
    pub targets: TArray<FMathRBFInterpolateQuatColor_Target>,
    pub output: FLinearColor,
}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateQuatQuat_Target {
    pub target: FQuat,
    pub value: FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateQuatQuat {
    pub targets: TArray<FMathRBFInterpolateQuatQuat_Target>,
    pub output: FQuat,
}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateQuatXform_Target {
    pub target: FQuat,
    pub value: FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateQuatXform {
    pub targets: TArray<FMathRBFInterpolateQuatXform_Target>,
    pub output: FTransform,
}
#[repr(C, align(8))]
pub struct FMathRBFInterpolateVectorFloat_Target {
    pub target: FVector,
    pub value: f32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateVectorFloat {
    pub targets: TArray<FMathRBFInterpolateVectorFloat_Target>,
    pub output: f32,
}
#[repr(C, align(8))]
pub struct FMathRBFInterpolateVectorVector_Target {
    pub target: FVector,
    pub value: FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateVectorVector {
    pub targets: TArray<FMathRBFInterpolateVectorVector_Target>,
    pub output: FVector,
}
#[repr(C, align(8))]
pub struct FMathRBFInterpolateVectorColor_Target {
    pub target: FVector,
    pub value: FLinearColor,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateVectorColor {
    pub targets: TArray<FMathRBFInterpolateVectorColor_Target>,
    pub output: FLinearColor,
}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateVectorQuat_Target {
    pub target: FVector,
    pub value: FQuat,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateVectorQuat {
    pub targets: TArray<FMathRBFInterpolateVectorQuat_Target>,
    pub output: FQuat,
}
#[repr(C, align(16))]
pub struct FMathRBFInterpolateVectorXform_Target {
    pub target: FVector,
    pub value: FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathRBFInterpolateVectorXform {
    pub targets: TArray<FMathRBFInterpolateVectorXform_Target>,
    pub output: FTransform,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathTransformBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathTransformMutableBase {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformUnaryOp {
    pub value: FTransform,
    pub result: FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformBinaryOp {
    pub a: FTransform,
    pub b: FTransform,
    pub result: FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformBinaryAggregateOp {
    pub a: FTransform,
    pub b: FTransform,
    pub result: FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformMake {
    pub translation: FVector,
    pub rotation: FQuat,
    pub scale: FVector,
    pub result: FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformFromEulerTransform {
    pub euler_transform: FEulerTransform,
    pub result: FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformFromEulerTransformV2 {
    pub value: FEulerTransform,
    pub result: FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformToEulerTransform {
    pub value: FTransform,
    pub result: FEulerTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformToVectors {
    pub value: FTransform,
    pub forward: FVector,
    pub right: FVector,
    pub up: FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformMul {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformMakeRelative {
    pub global: FTransform,
    pub parent: FTransform,
    pub local: FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformMakeAbsolute {
    pub local: FTransform,
    pub parent: FTransform,
    pub global: FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformAccumulateArray {
    pub transforms: TArray<FTransform>,
    pub target_space: ERigVMTransformSpace,
    pub root: FTransform,
    pub parent_indices: TArray<i32>,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformInverse {}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformLerp {
    pub a: FTransform,
    pub b: FTransform,
    pub t: f32,
    pub result: FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformSelectBool {
    pub condition: bool,
    pub if_true: FTransform,
    pub if_false: FTransform,
    pub result: FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformRotateVector {
    pub transform: FTransform,
    pub vector: FVector,
    pub result: FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformTransformVector {
    pub transform: FTransform,
    pub location: FVector,
    pub result: FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformFromSRT {
    pub location: FVector,
    pub rotation: FVector,
    pub rotation_order: EEulerRotationOrder,
    pub scale: FVector,
    pub transform: FTransform,
    pub euler_transform: FEulerTransform,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathTransformArrayToSRT {
    pub transforms: TArray<FTransform>,
    pub translations: TArray<FVector>,
    pub rotations: TArray<FQuat>,
    pub scales: TArray<FVector>,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformClampSpatially {
    pub value: FTransform,
    pub axis: EAxis,
    pub ty: ERigVMClampSpatialMode,
    pub minimum: f32,
    pub maximum: f32,
    pub space: FTransform,
    pub b_draw_debug: bool,
    pub debug_color: FLinearColor,
    pub debug_thickness: f32,
    pub result: FTransform,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathTransformMirrorTransform {
    pub value: FTransform,
    pub mirror_axis: EAxis,
    pub axis_to_flip: EAxis,
    pub central_transform: FTransform,
    pub result: FTransform,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorBase {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorUnaryOp {
    pub value: FVector,
    pub result: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorBinaryOp {
    pub a: FVector,
    pub b: FVector,
    pub result: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorBinaryAggregateOp {
    pub a: FVector,
    pub b: FVector,
    pub result: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMake {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub result: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorFromFloat {
    pub value: f32,
    pub result: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorFromDouble {
    pub value: f64,
    pub result: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorAdd {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorSub {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMul {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorScale {
    pub value: FVector,
    pub factor: f32,
    pub result: FVector,
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
    pub value: FVector,
    pub minimum: FVector,
    pub maximum: FVector,
    pub result: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorLerp {
    pub a: FVector,
    pub b: FVector,
    pub t: f32,
    pub result: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorRemap {
    pub value: FVector,
    pub source_minimum: FVector,
    pub source_maximum: FVector,
    pub target_minimum: FVector,
    pub target_maximum: FVector,
    pub b_clamp: bool,
    pub result: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorEquals {
    pub a: FVector,
    pub b: FVector,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorNotEquals {
    pub a: FVector,
    pub b: FVector,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorIsNearlyZero {
    pub value: FVector,
    pub tolerance: f32,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorIsNearlyEqual {
    pub a: FVector,
    pub b: FVector,
    pub tolerance: f32,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorSelectBool {
    pub condition: bool,
    pub if_true: FVector,
    pub if_false: FVector,
    pub result: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorDeg {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorRad {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorLengthSquared {
    pub value: FVector,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorLength {
    pub value: FVector,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorDistance {
    pub a: FVector,
    pub b: FVector,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorCross {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorDot {
    pub a: FVector,
    pub b: FVector,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorUnit {}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorSetLength {
    pub value: FVector,
    pub length: f32,
    pub result: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorClampLength {
    pub value: FVector,
    pub minimum_length: f32,
    pub maximum_length: f32,
    pub result: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMirror {
    pub value: FVector,
    pub normal: FVector,
    pub result: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorAngle {
    pub a: FVector,
    pub b: FVector,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorParallel {
    pub a: FVector,
    pub b: FVector,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorOrthogonal {
    pub a: FVector,
    pub b: FVector,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorBezierFourPoint {
    pub bezier: FRigVMFourPointBezier,
    pub t: f32,
    pub result: FVector,
    pub tangent: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFourPointBezier {
    pub a: FVector,
    pub b: FVector,
    pub c: FVector,
    pub d: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMakeBezierFourPoint {
    pub bezier: FRigVMFourPointBezier,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathVectorClampSpatially {
    pub value: FVector,
    pub axis: EAxis,
    pub ty: ERigVMClampSpatialMode,
    pub minimum: f32,
    pub maximum: f32,
    pub space: FTransform,
    pub b_draw_debug: bool,
    pub debug_color: FLinearColor,
    pub debug_thickness: f32,
    pub result: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathIntersectPlane {
    pub start: FVector,
    pub direction: FVector,
    pub plane_point: FVector,
    pub plane_normal: FVector,
    pub result: FVector,
    pub distance: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathDistanceToPlane {
    pub point: FVector,
    pub plane_point: FVector,
    pub plane_normal: FVector,
    pub closest_point_on_plane: FVector,
    pub signed_distance: f32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMakeRelative {
    pub global: FVector,
    pub parent: FVector,
    pub local: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorMakeAbsolute {
    pub local: FVector,
    pub parent: FVector,
    pub global: FVector,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_MathVectorMirrorTransform {
    pub value: FVector,
    pub mirror_axis: EAxis,
    pub axis_to_flip: EAxis,
    pub central_transform: FTransform,
    pub result: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorArraySum {
    pub array: TArray<FVector>,
    pub sum: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_MathVectorArrayAverage {
    pub array: TArray<FVector>,
    pub average: FVector,
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
    pub position: FVector,
    pub speed: FVector,
    pub frequency: FVector,
    pub minimum: f32,
    pub maximum: f32,
    pub result: FVector,
    pub time: FVector,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_NoiseVector2 {
    pub value: FVector,
    pub speed: FVector,
    pub frequency: FVector,
    pub minimum: f64,
    pub maximum: f64,
    pub result: FVector,
    pub time: FVector,
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
    pub result: FVector,
    pub last_result: FVector,
    pub last_seed: i32,
    pub base_seed: i32,
    pub time_left: f32,
}
#[repr(C, align(8))]
pub struct FRigVMMirrorSettings {
    pub mirror_axis: EAxis,
    pub axis_to_flip: EAxis,
    pub search_string: FString,
    pub replace_string: FString,
}
#[repr(C, align(8))]
pub struct FRigVMSimPoint {
    pub mass: f32,
    pub size: f32,
    pub linear_damping: f32,
    pub inherit_motion: f32,
    pub position: FVector,
    pub linear_velocity: FVector,
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
    pub increment: FVector,
    pub initial_value: FVector,
    pub b_integrate_delta_time: bool,
    pub result: FVector,
    pub accumulated_value: FVector,
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
    pub multiplier: FVector,
    pub initial_value: FVector,
    pub b_integrate_delta_time: bool,
    pub result: FVector,
    pub accumulated_value: FVector,
    pub b_is_initialized: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_AccumulateQuatMul {
    pub multiplier: FQuat,
    pub initial_value: FQuat,
    pub b_flip_order: bool,
    pub b_integrate_delta_time: bool,
    pub result: FQuat,
    pub accumulated_value: FQuat,
    pub b_is_initialized: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_AccumulateTransformMul {
    pub multiplier: FTransform,
    pub initial_value: FTransform,
    pub b_flip_order: bool,
    pub b_integrate_delta_time: bool,
    pub result: FTransform,
    pub accumulated_value: FTransform,
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
    pub target_value: FVector,
    pub initial_value: FVector,
    pub blend: f32,
    pub b_integrate_delta_time: bool,
    pub result: FVector,
    pub accumulated_value: FVector,
    pub b_is_initialized: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_AccumulateQuatLerp {
    pub target_value: FQuat,
    pub initial_value: FQuat,
    pub blend: f32,
    pub b_integrate_delta_time: bool,
    pub result: FQuat,
    pub accumulated_value: FQuat,
    pub b_is_initialized: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_AccumulateTransformLerp {
    pub target_value: FTransform,
    pub initial_value: FTransform,
    pub blend: f32,
    pub b_integrate_delta_time: bool,
    pub result: FTransform,
    pub accumulated_value: FTransform,
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
    pub value: FVector,
    pub minimum: FVector,
    pub maximum: FVector,
    pub accumulated_minimum: FVector,
    pub accumulated_maximum: FVector,
    pub b_is_initialized: bool,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_AlphaInterp {
    pub value: f32,
    pub scale: f32,
    pub bias: f32,
    pub b_map_range: bool,
    pub in_range: FInputRange,
    pub out_range: FInputRange,
    pub b_clamp_result: bool,
    pub clamp_min: f32,
    pub clamp_max: f32,
    pub b_interp_result: bool,
    pub interp_speed_increasing: f32,
    pub interp_speed_decreasing: f32,
    pub result: f32,
    pub scale_bias_clamp: FInputScaleBiasClamp,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_AlphaInterpVector {
    pub value: FVector,
    pub scale: f32,
    pub bias: f32,
    pub b_map_range: bool,
    pub in_range: FInputRange,
    pub out_range: FInputRange,
    pub b_clamp_result: bool,
    pub clamp_min: f32,
    pub clamp_max: f32,
    pub b_interp_result: bool,
    pub interp_speed_increasing: f32,
    pub interp_speed_decreasing: f32,
    pub result: FVector,
    pub scale_bias_clamp: FInputScaleBiasClamp,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_AlphaInterpQuat {
    pub value: FQuat,
    pub scale: f32,
    pub bias: f32,
    pub b_map_range: bool,
    pub in_range: FInputRange,
    pub out_range: FInputRange,
    pub b_clamp_result: bool,
    pub clamp_min: f32,
    pub clamp_max: f32,
    pub b_interp_result: bool,
    pub interp_speed_increasing: f32,
    pub interp_speed_decreasing: f32,
    pub result: FQuat,
    pub scale_bias_clamp: FInputScaleBiasClamp,
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
    pub value: FVector,
    pub delta: FVector,
    pub previous_value: FVector,
    pub cache: FVector,
    pub b_is_initialized: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DeltaFromPreviousQuat {
    pub value: FQuat,
    pub delta: FQuat,
    pub previous_value: FQuat,
    pub cache: FQuat,
    pub b_is_initialized: bool,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_DeltaFromPreviousTransform {
    pub value: FTransform,
    pub delta: FTransform,
    pub previous_value: FTransform,
    pub cache: FTransform,
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
    pub value: FVector,
    pub buffer_size: i32,
    pub result: FVector,
    pub buffer: TArray<FVector>,
    pub last_insert_index: i32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_KalmanTransform {
    pub value: FTransform,
    pub buffer_size: i32,
    pub result: FTransform,
    pub buffer: TArray<FTransform>,
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
    pub value: FVector,
    pub seconds_ago: f32,
    pub buffer_size: i32,
    pub time_range: f32,
    pub result: FVector,
    pub buffer: TArray<FVector>,
    pub delta_times: TArray<f32>,
    pub last_insert_index: i32,
    pub upper_bound: i32,
}
#[repr(C, align(16))]
pub struct FRigVMFunction_TimeOffsetTransform {
    pub value: FTransform,
    pub seconds_ago: f32,
    pub buffer_size: i32,
    pub time_range: f32,
    pub result: FTransform,
    pub buffer: TArray<FTransform>,
    pub delta_times: TArray<f32>,
    pub last_insert_index: i32,
    pub upper_bound: i32,
}
#[repr(C, align(8))]
pub struct FRigVMFunction_VerletIntegrateVector {
    pub target: FVector,
    pub strength: f32,
    pub damp: f32,
    pub blend: f32,
    pub force: FVector,
    pub position: FVector,
    pub velocity: FVector,
    pub acceleration: FVector,
    pub point: FRigVMSimPoint,
    pub b_initialized: bool,
}
#[repr(C, align(8))]
pub struct FRigVMUserDefinedTypesInfo {
    pub struct_guid_to_path_name: TMap<FString, FSoftObjectPath>,
    pub enum_to_path_name: TMap<FString, FSoftObjectPath>,
    pub types_in_use: TSet<UPtr<UObject>>,
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
    pub data_asset: TSoftObjectPtr<UDataAsset>,
    pub data_asset_cached: UPtr<UDataAsset>,
}
pub struct URigVMGraphFunctionHost {}
pub struct IRigVMGraphFunctionHost {}
pub struct URigVMMemoryStorageGeneratorClass {}
pub struct URigVMMemoryStorage {}
pub struct URigVMNativized {}
pub struct URigVMUserWorkflowOptions {
    pub subject: UPtr<UObject>,
    pub workflow: FRigVMUserWorkflow,
}
pub struct URigVMHost {
    pub vm_runtime_settings: FRigVMRuntimeSettings,
    pub vm: UPtr<URigVM>,
    pub user_defined_struct_guid_to_path_name: TMap<FString, FSoftObjectPath>,
    pub user_defined_enum_to_path_name: TMap<FString, FSoftObjectPath>,
    pub user_defined_types_in_use: TSet<UPtr<UObject>>,
    pub extended_execute_context_deprecated: FRigVMExtendedExecuteContext,
    pub draw_container: FRigVMDrawContainer,
    pub event_queue: TArray<FName>,
    pub asset_user_data: TArray<UPtr<UAssetUserData>>,
    pub asset_user_data_editor_only: TArray<UPtr<UAssetUserData>>,
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
