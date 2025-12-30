#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FStateTreeAnyEnum {
    pub value: u32,
    pub enum_: UPtr<UEnum>,
}
#[repr(C, align(4))]
pub struct FStateTreeDelegateDispatcher {
    pub id: FGuid,
}
#[repr(C, align(4))]
pub struct FStateTreeDelegateListener {
    pub dispatcher: FStateTreeDelegateDispatcher,
    pub id: i32,
}
#[repr(C, align(8))]
pub struct FExecutionRuntimeData {
    pub instances: FInstanceContainer,
}
#[repr(C, align(8))]
pub struct FInstanceContainer {
    pub instance_structs: FInstancedStructContainer,
}
#[repr(C, align(8))]
pub struct FStateTreeNodeBase {
    pub name: FName,
    pub bindings_batch: FStateTreeIndex16,
    pub output_bindings_batch: FStateTreeIndex16,
    pub instance_template_index: FStateTreeIndex16,
    pub execution_runtime_template_index: FStateTreeIndex16,
    pub instance_data_handle: FStateTreeDataHandle,
}
#[repr(C, align(2))]
pub struct FStateTreeDataHandle {
    pub source: EStateTreeDataSourceType,
    pub index: u16,
    pub state_handle: FStateTreeStateHandle,
}
#[repr(C, align(2))]
pub struct FStateTreeStateHandle {
    pub index: u16,
}
#[repr(C, align(2))]
pub struct FStateTreeIndex16 {
    pub value: u16,
}
#[repr(C, align(2))]
pub struct FStateTreePropertyRef {
    pub ref_access_index: FStateTreeIndex16,
}
#[repr(C, align(8))]
pub struct FStateTreeBlueprintPropertyRef {
    pub ref_type: EStateTreePropertyRefType,
    pub flags_3: u8,
    pub type_object: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FStateTreeConditionBase {
    pub operand: EStateTreeExpressionOperand,
    pub delta_indent: i8,
    pub evaluation_mode: EStateTreeConditionEvaluationMode,
}
#[repr(C, align(8))]
pub struct FStateTreeBlueprintConditionWrapper {
    pub condition_class: TSubclassOf<UStateTreeConditionBlueprintBase>,
}
#[repr(C, align(8))]
pub struct FStateTreeConsiderationBase {
    pub operand: EStateTreeExpressionOperand,
    pub delta_indent: i8,
}
#[repr(C, align(8))]
pub struct FStateTreeBlueprintConsiderationWrapper {
    pub consideration_class: TSubclassOf<UStateTreeConsiderationBlueprintBase>,
}
#[repr(C, align(8))]
pub struct FStateTreeEvaluatorBase {}
#[repr(C, align(8))]
pub struct FStateTreeBlueprintEvaluatorWrapper {
    pub evaluator_class: TSubclassOf<UStateTreeEvaluatorBlueprintBase>,
}
#[repr(C, align(8))]
pub struct FStateTreeTaskBase {
    pub flags_40: u8,
    pub transition_handling_priority: EStateTreeTransitionPriority,
    pub flags_42: u8,
}
#[repr(C, align(8))]
pub struct FStateTreeBlueprintTaskWrapper {
    pub task_class: TSubclassOf<UStateTreeTaskBlueprintBase>,
    pub task_flags: u8,
}
#[repr(C, align(4))]
pub struct FStateTreeCompareIntConditionInstanceData {
    pub left: i32,
    pub right: i32,
}
#[repr(C, align(8))]
pub struct FStateTreeConditionCommonBase {}
#[repr(C, align(8))]
pub struct FStateTreeCompareIntCondition {
    pub b_invert: bool,
    pub operator: EGenericAICheck,
}
#[repr(C, align(8))]
pub struct FStateTreeCompareFloatConditionInstanceData {
    pub left: f64,
    pub right: f64,
}
#[repr(C, align(8))]
pub struct FStateTreeCompareFloatCondition {
    pub b_invert: bool,
    pub operator: EGenericAICheck,
}
#[repr(C, align(1))]
pub struct FStateTreeCompareBoolConditionInstanceData {
    pub b_left: bool,
    pub b_right: bool,
}
#[repr(C, align(8))]
pub struct FStateTreeCompareBoolCondition {
    pub b_invert: bool,
}
#[repr(C, align(8))]
pub struct FStateTreeCompareEnumConditionInstanceData {
    pub left: FStateTreeAnyEnum,
    pub right: FStateTreeAnyEnum,
}
#[repr(C, align(8))]
pub struct FStateTreeCompareEnumCondition {
    pub b_invert: bool,
}
#[repr(C, align(8))]
pub struct FStateTreeCompareDistanceConditionInstanceData {
    pub source: FVector,
    pub target: FVector,
    pub distance: f64,
}
#[repr(C, align(8))]
pub struct FStateTreeCompareDistanceCondition {
    pub b_invert: bool,
    pub operator: EGenericAICheck,
}
#[repr(C, align(4))]
pub struct FStateTreeCompareNameConditionInstanceData {
    pub left: FName,
    pub right: FName,
}
#[repr(C, align(8))]
pub struct FStateTreeCompareNameCondition {
    pub b_invert: bool,
}
#[repr(C, align(4))]
pub struct FStateTreeRandomConditionInstanceData {
    pub threshold: f32,
}
#[repr(C, align(8))]
pub struct FStateTreeRandomCondition {}
#[repr(C, align(8))]
pub struct FGameplayTagMatchConditionInstanceData {
    pub tag_container: FGameplayTagContainer,
    pub tag: FGameplayTag,
}
#[repr(C, align(8))]
pub struct FGameplayTagMatchCondition {
    pub b_exact_match: bool,
    pub b_invert: bool,
}
#[repr(C, align(8))]
pub struct FGameplayTagContainerMatchConditionInstanceData {
    pub tag_container: FGameplayTagContainer,
    pub other_container: FGameplayTagContainer,
}
#[repr(C, align(8))]
pub struct FGameplayTagContainerMatchCondition {
    pub match_type: EGameplayContainerMatchType,
    pub b_exact_match: bool,
    pub b_invert: bool,
}
#[repr(C, align(8))]
pub struct FGameplayTagQueryConditionInstanceData {
    pub tag_container: FGameplayTagContainer,
}
#[repr(C, align(8))]
pub struct FGameplayTagQueryCondition {
    pub tag_query: FGameplayTagQuery,
    pub b_invert: bool,
}
#[repr(C, align(8))]
pub struct FStateTreeObjectIsValidConditionInstanceData {
    pub object: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FStateTreeObjectIsValidCondition {
    pub b_invert: bool,
}
#[repr(C, align(8))]
pub struct FStateTreeObjectEqualsConditionInstanceData {
    pub left: UPtr<UObject>,
    pub right: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FStateTreeObjectEqualsCondition {
    pub b_invert: bool,
}
#[repr(C, align(8))]
pub struct FStateTreeObjectIsChildOfClassConditionInstanceData {
    pub object: UPtr<UObject>,
    pub class: TSubclassOf<UObject>,
}
#[repr(C, align(8))]
pub struct FStateTreeObjectIsChildOfClassCondition {
    pub b_invert: bool,
}
#[repr(C, align(4))]
pub struct FStateTreeConstantConsiderationInstanceData {
    pub constant: f32,
}
#[repr(C, align(8))]
pub struct FStateTreeConsiderationCommonBase {}
#[repr(C, align(8))]
pub struct FStateTreeConstantConsideration {}
#[repr(C, align(8))]
pub struct FStateTreeConsiderationResponseCurve {
    pub curve_info: FRuntimeFloatCurve,
}
#[repr(C, align(4))]
pub struct FStateTreeFloatInputConsiderationInstanceData {
    pub input: f32,
    pub interval: FFloatInterval,
}
#[repr(C, align(8))]
pub struct FStateTreeFloatInputConsideration {
    pub response_curve: FStateTreeConsiderationResponseCurve,
}
#[repr(C, align(8))]
pub struct FStateTreeEnumValueScorePair {
    pub enum_name: FName,
    pub enum_value: i64,
    pub score: f32,
}
#[repr(C, align(8))]
pub struct FStateTreeEnumValueScorePairs {
    pub enum_: UPtr<UEnum>,
    pub data: TArray<FStateTreeEnumValueScorePair>,
}
#[repr(C, align(8))]
pub struct FStateTreeEnumInputConsiderationInstanceData {
    pub input: FStateTreeAnyEnum,
}
#[repr(C, align(8))]
pub struct FStateTreeEnumInputConsideration {
    pub enum_value_score_pairs: FStateTreeEnumValueScorePairs,
}
#[repr(C, align(1))]
pub struct FStateTreeBooleanOperationPropertyFunctionInstanceData {
    pub b_left: bool,
    pub b_right: bool,
    pub b_result: bool,
}
#[repr(C, align(8))]
pub struct FStateTreePropertyFunctionBase {}
#[repr(C, align(8))]
pub struct FStateTreePropertyFunctionCommonBase {}
#[repr(C, align(8))]
pub struct FStateTreeBooleanAndPropertyFunction {}
#[repr(C, align(8))]
pub struct FStateTreeBooleanOrPropertyFunction {}
#[repr(C, align(8))]
pub struct FStateTreeBooleanXOrPropertyFunction {}
#[repr(C, align(1))]
pub struct FStateTreeBooleanNotOperationPropertyFunctionInstanceData {
    pub b_input: bool,
    pub b_result: bool,
}
#[repr(C, align(8))]
pub struct FStateTreeBooleanNotPropertyFunction {}
#[repr(C, align(4))]
pub struct FStateTreeFloatCombinaisonPropertyFunctionInstanceData {
    pub left: f32,
    pub right: f32,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FStateTreeAddFloatPropertyFunction {}
#[repr(C, align(8))]
pub struct FStateTreeSubtractFloatPropertyFunction {}
#[repr(C, align(8))]
pub struct FStateTreeMultiplyFloatPropertyFunction {}
#[repr(C, align(8))]
pub struct FStateTreeDivideFloatPropertyFunction {}
#[repr(C, align(4))]
pub struct FStateTreeSingleFloatPropertyFunctionInstanceData {
    pub input: f32,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FStateTreeInvertFloatPropertyFunction {}
#[repr(C, align(8))]
pub struct FStateTreeAbsoluteFloatPropertyFunction {}
#[repr(C, align(4))]
pub struct FStateTreeMakeIntervalPropertyFunctionInstanceData {
    pub min: f32,
    pub max: f32,
    pub result: FFloatInterval,
}
#[repr(C, align(8))]
pub struct FStateTreeMakeIntervalPropertyFunction {}
#[repr(C, align(4))]
pub struct FStateTreeIntCombinaisonPropertyFunctionInstanceData {
    pub left: i32,
    pub right: i32,
    pub result: i32,
}
#[repr(C, align(8))]
pub struct FStateTreeAddIntPropertyFunction {}
#[repr(C, align(8))]
pub struct FStateTreeSubtractIntPropertyFunction {}
#[repr(C, align(8))]
pub struct FStateTreeMultiplyIntPropertyFunction {}
#[repr(C, align(8))]
pub struct FStateTreeDivideIntPropertyFunction {}
#[repr(C, align(4))]
pub struct FStateTreeSingleIntPropertyFunctionInstanceData {
    pub input: i32,
    pub result: i32,
}
#[repr(C, align(8))]
pub struct FStateTreeInvertIntPropertyFunction {}
#[repr(C, align(8))]
pub struct FStateTreeAbsoluteIntPropertyFunction {}
#[repr(C, align(8))]
pub struct FStateTreeEvaluatorCommonBase {}
#[repr(C, align(8))]
pub struct FStateTreeEvent {
    pub tag: FGameplayTag,
    pub payload: FInstancedStruct,
    pub origin: FName,
}
#[repr(C, align(8))]
pub struct FStateTreeSharedEvent {}
#[repr(C, align(8))]
pub struct FStateTreeEventQueue {
    pub shared_events: TArray<FStateTreeSharedEvent>,
}
#[repr(C, align(8))]
pub struct FStateTreeExecutionExtension {}
#[repr(C, align(2))]
pub struct FStateTreeExternalDataHandle {
    pub data_handle: FStateTreeDataHandle,
}
#[repr(C, align(8))]
pub struct FStateTreeExternalDataDesc {
    pub _struct: UPtr<UStruct>,
    pub name: FName,
    pub handle: FStateTreeExternalDataHandle,
    pub requirement: EStateTreeExternalDataRequirement,
    pub id: FGuid,
}
#[repr(C, align(8))]
pub struct FStateTreeTransitionRequest {
    pub target_state: FStateTreeStateHandle,
    pub priority: EStateTreeTransitionPriority,
    pub fallback: EStateTreeSelectionFallback,
    pub source_state_tree: UPtr<UStateTree>,
    pub source_root_state: FStateTreeStateHandle,
    pub source_state: FStateTreeStateHandle,
}
#[repr(C, align(4))]
pub struct FStateTreeActiveStates {
    pub states: FStateTreeStateHandle,
    pub num_states: u8,
}
#[repr(C, align(4))]
pub struct FStateTreeTransitionSource {}
#[repr(C, align(8))]
pub struct FStateTreeTransitionDelayedState {
    pub state_tree: UPtr<UStateTree>,
    pub captured_event: FStateTreeSharedEvent,
    pub time_left: f32,
    pub captured_event_hash: u32,
    pub state_handle: FStateTreeStateHandle,
    pub transition_index: FStateTreeIndex16,
}
#[repr(C, align(4))]
pub struct FStateTreeScheduledTick {
    pub next_delta_time: f32,
}
#[repr(C, align(8))]
pub struct FExecutionFrameHandle {
    pub state_tree: UPtr<UStateTree>,
    pub root_state: FStateTreeStateHandle,
}
#[repr(C, align(8))]
pub struct FStateTreeExecutionFrame {
    pub state_tree: UPtr<UStateTree>,
    pub root_state: FStateTreeStateHandle,
    pub active_states: FStateTreeActiveStates,
    pub active_tasks_status: FStateTreeTasksCompletionStatus,
    pub external_data_base_index: FStateTreeIndex16,
    pub global_instance_index_base: FStateTreeIndex16,
    pub active_instance_index_base: FStateTreeIndex16,
    pub execution_runtime_index_base: FStateTreeIndex16,
    pub state_parameter_data_handle: FStateTreeDataHandle,
    pub global_parameter_data_handle: FStateTreeDataHandle,
    pub flags_106: u8,
}
#[repr(C, align(8))]
pub struct FStateTreeTasksCompletionStatus {}
#[repr(C, align(8))]
pub struct FStateTreeExecutionState {
    pub active_frames: TArray<FStateTreeExecutionFrame>,
    pub delayed_transitions: TArray<FStateTreeTransitionDelayedState>,
    pub random_stream: FRandomStream,
    pub execution_extension: FInstancedStruct,
    pub last_tick_status: EStateTreeRunStatus,
    pub tree_run_status: EStateTreeRunStatus,
    pub requested_stop: EStateTreeRunStatus,
    pub current_phase: EStateTreeUpdatePhase,
    pub state_change_count: u16,
    pub b_has_pending_completed_state: bool,
    pub completed_frame_index: FStateTreeIndex16,
    pub completed_state_handle: FStateTreeStateHandle,
    pub enter_state_failed_frame_index: FStateTreeIndex16,
    pub enter_state_failed_task_index: FStateTreeIndex16,
    pub last_exited_node_index: FStateTreeIndex16,
}
#[repr(C, align(8))]
pub struct FStateTreeTransitionResult {
    pub target_state: FStateTreeStateHandle,
    pub current_state: FStateTreeStateHandle,
    pub current_run_status: EStateTreeRunStatus,
    pub change_type: EStateTreeStateChangeType,
    pub priority: EStateTreeTransitionPriority,
    pub next_active_frames: TArray<FStateTreeExecutionFrame>,
    pub source_state_tree: UPtr<UStateTree>,
    pub source_root_state: FStateTreeStateHandle,
    pub source_state: FStateTreeStateHandle,
}
#[repr(C, align(8))]
pub struct FRecordedStateTreeExecutionFrame {
    pub state_tree: UPtr<UStateTree>,
    pub root_state: FStateTreeStateHandle,
    pub active_states: FStateTreeActiveStates,
    pub flags_64: u8,
}
#[repr(C, align(8))]
pub struct FRecordedActiveState {
    pub state_tree: UPtr<UStateTree>,
    pub state: FStateTreeStateHandle,
    pub event_index: i32,
}
#[repr(C, align(8))]
pub struct FRecordedStateTreeTransitionResult {
    pub states: TArray<FRecordedActiveState>,
    pub events: TArray<FStateTreeEvent>,
    pub priority: EStateTreeTransitionPriority,
    pub next_active_frames: TArray<FRecordedStateTreeExecutionFrame>,
    pub next_active_frame_events: TArray<FStateTreeEvent>,
    pub source_state: FStateTreeStateHandle,
    pub target_state: FStateTreeStateHandle,
    pub source_state_tree: UPtr<UStateTree>,
    pub source_root_state: FStateTreeStateHandle,
}
#[repr(C, align(8))]
pub struct FStateTreeInstanceObjectWrapper {
    pub instance_object: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FStateTreeTemporaryInstanceData {
    pub data_handle: FStateTreeDataHandle,
    pub owner_node_index: FStateTreeIndex16,
    pub instance: FInstancedStruct,
    pub state_tree: UPtr<UStateTree>,
    pub root_state: FStateTreeStateHandle,
}
#[repr(C, align(8))]
pub struct FStateTreeInstanceStorage {
    pub instance_structs: FInstancedStructContainer,
    pub execution_state: FStateTreeExecutionState,
    pub execution_runtime_data: FInstanceContainer,
    pub temporary_instances: TArray<FStateTreeTemporaryInstanceData>,
    pub transition_requests: TArray<FStateTreeTransitionRequest>,
    pub global_parameters: FInstancedPropertyBag,
    pub unique_id_generator: u32,
}
#[repr(C, align(8))]
pub struct FStateTreeInstanceData {
    pub instance_storage_deprecated: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FStateTreeBindableStructDesc {
    pub data_handle: FStateTreeDataHandle,
    pub data_source: EStateTreeBindableStructSource,
    pub state_path: FString,
}
#[repr(C, align(8))]
pub struct FStateTreeEditorPropertyPath {
    pub struct_id: FGuid,
    pub path: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FStateTreePropertyPathBinding {
    pub source_data_handle: FStateTreeDataHandle,
    pub b_is_output_binding: bool,
    pub source_path_deprecated: FStateTreeEditorPropertyPath,
    pub target_path_deprecated: FStateTreeEditorPropertyPath,
}
#[repr(C, align(8))]
pub struct FStateTreePropertyRefPath {
    pub source_property_path: FPropertyBindingPath,
    pub source_data_handle: FStateTreeDataHandle,
}
#[repr(C, align(8))]
pub struct FStateTreePropertyAccess {
    pub source_indirection: FPropertyBindingPropertyIndirection,
    pub source_struct_type: UPtr<UStruct>,
    pub source_data_handle: FStateTreeDataHandle,
}
#[repr(C, align(16))]
pub struct FStateTreePropertyBindings {
    pub source_structs: TArray<FStateTreeBindableStructDesc>,
    pub property_path_bindings: TArray<FStateTreePropertyPathBinding>,
    pub property_reference_paths: TArray<FStateTreePropertyRefPath>,
    pub property_accesses: TArray<FStateTreePropertyAccess>,
}
#[repr(C, align(8))]
pub struct FStateTreeReference {
    pub state_tree: UPtr<UStateTree>,
    pub parameters: FInstancedPropertyBag,
    pub property_overrides: TArray<FGuid>,
}
#[repr(C, align(8))]
pub struct FStateTreeReferenceOverrideItem {
    pub state_tag: FGameplayTag,
    pub state_tree_reference: FStateTreeReference,
}
#[repr(C, align(8))]
pub struct FStateTreeReferenceOverrides {
    pub override_items: TArray<FStateTreeReferenceOverrideItem>,
}
#[repr(C, align(8))]
pub struct FStateTreeTaskCommonBase {}
#[repr(C, align(2))]
pub struct FStateTreeRandomTimeDuration {
    pub duration: u16,
    pub random_variance: u16,
}
#[repr(C, align(8))]
pub struct FCompactEventDesc {
    pub payload_struct: UPtr<UScriptStruct>,
    pub tag: FGameplayTag,
}
#[repr(C, align(8))]
pub struct FCompactStateTransition {
    pub required_event: FCompactEventDesc,
    pub required_delegate_dispatcher: FStateTreeDelegateDispatcher,
    pub conditions_begin: u16,
    pub state: FStateTreeStateHandle,
    pub delay: FStateTreeRandomTimeDuration,
    pub trigger: EStateTreeTransitionTrigger,
    pub priority: EStateTreeTransitionPriority,
    pub fallback: EStateTreeSelectionFallback,
    pub conditions_num: u8,
    pub flags_64: u8,
}
#[repr(C, align(2))]
pub struct FCompactStateTreeFrame {
    pub root_state: FStateTreeStateHandle,
    pub number_of_tasks_status_masks: u8,
}
#[repr(C, align(8))]
pub struct FCompactStateTreeState {
    pub required_event_to_enter: FCompactEventDesc,
    pub name: FName,
    pub tag: FGameplayTag,
    pub linked_asset: UPtr<UStateTree>,
    pub linked_state: FStateTreeStateHandle,
    pub parent: FStateTreeStateHandle,
    pub children_begin: u16,
    pub children_end: u16,
    pub enter_conditions_begin: u16,
    pub utility_considerations_begin: u16,
    pub transitions_begin: u16,
    pub tasks_begin: u16,
    pub parameter_template_index: FStateTreeIndex16,
    pub parameter_data_handle: FStateTreeDataHandle,
    pub parameter_bindings_batch: FStateTreeIndex16,
    pub event_data_index: FStateTreeIndex16,
    pub weight: f32,
    pub custom_tick_rate: f32,
    pub completion_tasks_mask: u32,
    pub completion_tasks_mask_buffer_index: u8,
    pub completion_tasks_mask_bits_offset: u8,
    pub completion_tasks_control: EStateTreeTaskCompletionType,
    pub enter_conditions_num: u8,
    pub utility_considerations_num: u8,
    pub transitions_num: u8,
    pub tasks_num: u8,
    pub enabled_tasks_num: u8,
    pub instance_data_num: u8,
    pub depth: u8,
    pub ty: EStateTreeStateType,
    pub selection_behavior: EStateTreeStateSelectionBehavior,
    pub flags_132: u8,
    pub flags_133: u8,
}
#[repr(C, align(8))]
pub struct FCompactStateTreeParameters {
    pub parameters: FInstancedPropertyBag,
}
#[repr(C, align(4))]
pub struct FStateTreeStateIdToHandle {
    pub id: FGuid,
    pub handle: FStateTreeStateHandle,
}
#[repr(C, align(4))]
pub struct FStateTreeNodeIdToIndex {
    pub id: FGuid,
    pub index: FStateTreeIndex16,
}
#[repr(C, align(4))]
pub struct FStateTreeTransitionIdToIndex {
    pub id: FGuid,
    pub index: FStateTreeIndex16,
}
#[repr(C, align(8))]
pub struct FStateTreeStructRef {}
#[repr(C, align(4))]
pub struct FStateTreeStateLink {
    pub name: FName,
    pub id: FGuid,
    pub link_type: EStateTreeTransitionType,
    pub ty_deprecated: EStateTreeTransitionType,
    pub state_handle: FStateTreeStateHandle,
    pub fallback: EStateTreeSelectionFallback,
}
#[repr(C, align(8))]
pub struct FStateTreeDebugTextTaskInstanceData {
    pub reference_actor: UPtr<AActor>,
    pub bindable_text: FString,
}
#[repr(C, align(8))]
pub struct FStateTreeDebugTextTask {
    pub text: FString,
    pub text_color: FColor,
    pub font_scale: f32,
    pub offset: FVector,
    pub b_enabled: bool,
}
#[repr(C, align(4))]
pub struct FStateTreeDelayTaskInstanceData {
    pub duration: f32,
    pub random_deviation: f32,
    pub b_run_forever: bool,
}
#[repr(C, align(8))]
pub struct FStateTreeDelayTask {}
#[repr(C, align(8))]
pub struct FStateTreeRunParallelStateTreeTaskInstanceData {
    pub state_tree: FStateTreeReference,
    pub tree_instance_data: FStateTreeInstanceData,
    pub running_state_tree: UPtr<UStateTree>,
}
#[repr(C, align(8))]
pub struct FStateTreeRunParallelStateTreeExecutionExtension {}
#[repr(C, align(8))]
pub struct FStateTreeRunParallelStateTreeTask {
    pub state_tree_override_tag: FGameplayTag,
    pub event_handling_priority: EStateTreeTransitionPriority,
}
#[repr(C, align(1))]
pub struct FStateTreeIndex8 {
    pub value: u8,
}
pub struct UStateTreeSchemaProvider {}
pub struct IStateTreeSchemaProvider {}
pub struct UStateTreeSettings {
    pub b_auto_start_debugger_traces_on_non_editor_targets: bool,
}
pub struct UStateTreeNodeBlueprintBase {
    pub cached_frame_state_tree: UPtr<UStateTree>,
    pub description: FText,
    pub icon_name: FName,
    pub icon_color: FColor,
}
pub struct UStateTreeConditionBlueprintBase {}
pub struct UStateTreeConsiderationBlueprintBase {}
pub struct UStateTreeEvaluatorBlueprintBase {}
pub struct UStateTreeTaskBlueprintBase {
    pub flags_205: u8,
}
pub struct UStateTree {
    pub editor_data: UPtr<UObject>,
    pub last_compiled_editor_data_hash: u32,
    pub schema: UPtr<UStateTreeSchema>,
    pub frames: TArray<FCompactStateTreeFrame>,
    pub states: TArray<FCompactStateTreeState>,
    pub transitions: TArray<FCompactStateTransition>,
    pub nodes: FInstancedStructContainer,
    pub default_instance_data: FStateTreeInstanceData,
    pub default_evaluation_scope_instance_data: FInstanceContainer,
    pub default_execution_runtime_data: FInstanceContainer,
    pub shared_instance_data: FStateTreeInstanceData,
    pub context_data_descs: TArray<FStateTreeExternalDataDesc>,
    pub property_bindings: FStateTreePropertyBindings,
    pub extensions: TArray<UPtr<UStateTreeExtension>>,
    pub id_to_state_mappings: TArray<FStateTreeStateIdToHandle>,
    pub id_to_node_mappings: TArray<FStateTreeNodeIdToIndex>,
    pub id_to_transition_mappings: TArray<FStateTreeTransitionIdToIndex>,
    pub parameters: FInstancedPropertyBag,
    pub external_data_descs: TArray<FStateTreeExternalDataDesc>,
    pub completion_global_tasks_mask: u32,
    pub num_context_data: u16,
    pub num_global_instance_data: u16,
    pub evaluators_begin: u16,
    pub evaluators_num: u16,
    pub global_tasks_begin: u16,
    pub global_tasks_num: u16,
    pub completion_global_tasks_control: EStateTreeTaskCompletionType,
    pub parameter_data_type: EStateTreeParameterDataType,
    pub flags_694: u8,
}
pub struct UStateTreeExtension {}
pub struct UStateTreeFunctionLibrary {}
pub struct UStateTreeSchema {}
