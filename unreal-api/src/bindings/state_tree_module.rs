#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FStateTreeAnyEnum {
    pub value: u32,
    pub enum_: UPtr<crate::bindings::core_u_object::UEnum>,
}
#[repr(C, align(4))]
pub struct FStateTreeDelegateDispatcher {
    pub id: crate::bindings::core_u_object::FGuid,
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
    pub instance_structs: crate::bindings::core_u_object::FInstancedStructContainer,
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
    pub type_object: UPtr<crate::bindings::core_u_object::UObject>,
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
    pub operator: crate::bindings::ai_module::EGenericAICheck,
}
#[repr(C, align(8))]
pub struct FStateTreeCompareFloatConditionInstanceData {
    pub left: f64,
    pub right: f64,
}
#[repr(C, align(8))]
pub struct FStateTreeCompareFloatCondition {
    pub b_invert: bool,
    pub operator: crate::bindings::ai_module::EGenericAICheck,
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
    pub source: crate::bindings::core_u_object::FVector,
    pub target: crate::bindings::core_u_object::FVector,
    pub distance: f64,
}
#[repr(C, align(8))]
pub struct FStateTreeCompareDistanceCondition {
    pub b_invert: bool,
    pub operator: crate::bindings::ai_module::EGenericAICheck,
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
    pub tag_container: crate::bindings::gameplay_tags::FGameplayTagContainer,
    pub tag: crate::bindings::gameplay_tags::FGameplayTag,
}
#[repr(C, align(8))]
pub struct FGameplayTagMatchCondition {
    pub b_exact_match: bool,
    pub b_invert: bool,
}
#[repr(C, align(8))]
pub struct FGameplayTagContainerMatchConditionInstanceData {
    pub tag_container: crate::bindings::gameplay_tags::FGameplayTagContainer,
    pub other_container: crate::bindings::gameplay_tags::FGameplayTagContainer,
}
#[repr(C, align(8))]
pub struct FGameplayTagContainerMatchCondition {
    pub match_type: crate::bindings::gameplay_tags::EGameplayContainerMatchType,
    pub b_exact_match: bool,
    pub b_invert: bool,
}
#[repr(C, align(8))]
pub struct FGameplayTagQueryConditionInstanceData {
    pub tag_container: crate::bindings::gameplay_tags::FGameplayTagContainer,
}
#[repr(C, align(8))]
pub struct FGameplayTagQueryCondition {
    pub tag_query: crate::bindings::gameplay_tags::FGameplayTagQuery,
    pub b_invert: bool,
}
#[repr(C, align(8))]
pub struct FStateTreeObjectIsValidConditionInstanceData {
    pub object: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FStateTreeObjectIsValidCondition {
    pub b_invert: bool,
}
#[repr(C, align(8))]
pub struct FStateTreeObjectEqualsConditionInstanceData {
    pub left: UPtr<crate::bindings::core_u_object::UObject>,
    pub right: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FStateTreeObjectEqualsCondition {
    pub b_invert: bool,
}
#[repr(C, align(8))]
pub struct FStateTreeObjectIsChildOfClassConditionInstanceData {
    pub object: UPtr<crate::bindings::core_u_object::UObject>,
    pub class: TSubclassOf<crate::bindings::core_u_object::UObject>,
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
    pub curve_info: crate::bindings::engine::FRuntimeFloatCurve,
}
#[repr(C, align(4))]
pub struct FStateTreeFloatInputConsiderationInstanceData {
    pub input: f32,
    pub interval: crate::bindings::core_u_object::FFloatInterval,
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
    pub enum_: UPtr<crate::bindings::core_u_object::UEnum>,
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
    pub result: crate::bindings::core_u_object::FFloatInterval,
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
    pub tag: crate::bindings::gameplay_tags::FGameplayTag,
    pub payload: crate::bindings::core_u_object::FInstancedStruct,
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
    pub _struct: UPtr<crate::bindings::core_u_object::UStruct>,
    pub name: FName,
    pub handle: FStateTreeExternalDataHandle,
    pub requirement: EStateTreeExternalDataRequirement,
    pub id: crate::bindings::core_u_object::FGuid,
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
    pub random_stream: crate::bindings::core_u_object::FRandomStream,
    pub execution_extension: crate::bindings::core_u_object::FInstancedStruct,
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
    pub instance_object: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FStateTreeTemporaryInstanceData {
    pub data_handle: FStateTreeDataHandle,
    pub owner_node_index: FStateTreeIndex16,
    pub instance: crate::bindings::core_u_object::FInstancedStruct,
    pub state_tree: UPtr<UStateTree>,
    pub root_state: FStateTreeStateHandle,
}
#[repr(C, align(8))]
pub struct FStateTreeInstanceStorage {
    pub instance_structs: crate::bindings::core_u_object::FInstancedStructContainer,
    pub execution_state: FStateTreeExecutionState,
    pub execution_runtime_data: FInstanceContainer,
    pub temporary_instances: TArray<FStateTreeTemporaryInstanceData>,
    pub transition_requests: TArray<FStateTreeTransitionRequest>,
    pub global_parameters: crate::bindings::core_u_object::FInstancedPropertyBag,
    pub unique_id_generator: u32,
}
#[repr(C, align(8))]
pub struct FStateTreeInstanceData {
    pub instance_storage_deprecated: crate::bindings::core_u_object::FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FStateTreeBindableStructDesc {
    pub data_handle: FStateTreeDataHandle,
    pub data_source: EStateTreeBindableStructSource,
    pub state_path: FString,
}
#[repr(C, align(8))]
pub struct FStateTreeEditorPropertyPath {
    pub struct_id: crate::bindings::core_u_object::FGuid,
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
    pub source_property_path: crate::bindings::property_binding_utils::FPropertyBindingPath,
    pub source_data_handle: FStateTreeDataHandle,
}
#[repr(C, align(8))]
pub struct FStateTreePropertyAccess {
    pub source_indirection: crate::bindings::property_binding_utils::FPropertyBindingPropertyIndirection,
    pub source_struct_type: UPtr<crate::bindings::core_u_object::UStruct>,
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
    pub parameters: crate::bindings::core_u_object::FInstancedPropertyBag,
    pub property_overrides: TArray<crate::bindings::core_u_object::FGuid>,
}
#[repr(C, align(8))]
pub struct FStateTreeReferenceOverrideItem {
    pub state_tag: crate::bindings::gameplay_tags::FGameplayTag,
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
    pub payload_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub tag: crate::bindings::gameplay_tags::FGameplayTag,
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
    pub tag: crate::bindings::gameplay_tags::FGameplayTag,
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
    pub parameters: crate::bindings::core_u_object::FInstancedPropertyBag,
}
#[repr(C, align(4))]
pub struct FStateTreeStateIdToHandle {
    pub id: crate::bindings::core_u_object::FGuid,
    pub handle: FStateTreeStateHandle,
}
#[repr(C, align(4))]
pub struct FStateTreeNodeIdToIndex {
    pub id: crate::bindings::core_u_object::FGuid,
    pub index: FStateTreeIndex16,
}
#[repr(C, align(4))]
pub struct FStateTreeTransitionIdToIndex {
    pub id: crate::bindings::core_u_object::FGuid,
    pub index: FStateTreeIndex16,
}
#[repr(C, align(8))]
pub struct FStateTreeStructRef {}
#[repr(C, align(4))]
pub struct FStateTreeStateLink {
    pub name: FName,
    pub id: crate::bindings::core_u_object::FGuid,
    pub link_type: EStateTreeTransitionType,
    pub ty_deprecated: EStateTreeTransitionType,
    pub state_handle: FStateTreeStateHandle,
    pub fallback: EStateTreeSelectionFallback,
}
#[repr(C, align(8))]
pub struct FStateTreeDebugTextTaskInstanceData {
    pub reference_actor: UPtr<crate::bindings::engine::AActor>,
    pub bindable_text: FString,
}
#[repr(C, align(8))]
pub struct FStateTreeDebugTextTask {
    pub text: FString,
    pub text_color: crate::bindings::core_u_object::FColor,
    pub font_scale: f32,
    pub offset: crate::bindings::core_u_object::FVector,
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
    pub state_tree_override_tag: crate::bindings::gameplay_tags::FGameplayTag,
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
    pub icon_color: crate::bindings::core_u_object::FColor,
}
pub struct UStateTreeConditionBlueprintBase {}
pub struct UStateTreeConsiderationBlueprintBase {}
pub struct UStateTreeEvaluatorBlueprintBase {}
pub struct UStateTreeTaskBlueprintBase {
    pub flags_205: u8,
}
pub struct UStateTree {
    pub editor_data: UPtr<crate::bindings::core_u_object::UObject>,
    pub last_compiled_editor_data_hash: u32,
    pub schema: UPtr<UStateTreeSchema>,
    pub frames: TArray<FCompactStateTreeFrame>,
    pub states: TArray<FCompactStateTreeState>,
    pub transitions: TArray<FCompactStateTransition>,
    pub nodes: crate::bindings::core_u_object::FInstancedStructContainer,
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
    pub parameters: crate::bindings::core_u_object::FInstancedPropertyBag,
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
pub struct FBindDelegate_Delegate;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeDataSourceType(pub u8);
impl EStateTreeDataSourceType {
    pub const NONE: EStateTreeDataSourceType = EStateTreeDataSourceType(0);
    pub const GLOBAL_INSTANCE_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(
        1,
    );
    pub const GLOBAL_INSTANCE_DATA_OBJECT: EStateTreeDataSourceType = EStateTreeDataSourceType(
        2,
    );
    pub const ACTIVE_INSTANCE_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(
        3,
    );
    pub const ACTIVE_INSTANCE_DATA_OBJECT: EStateTreeDataSourceType = EStateTreeDataSourceType(
        4,
    );
    pub const SHARED_INSTANCE_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(
        5,
    );
    pub const SHARED_INSTANCE_DATA_OBJECT: EStateTreeDataSourceType = EStateTreeDataSourceType(
        6,
    );
    pub const EVALUATION_SCOPE_INSTANCE_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(
        7,
    );
    pub const EVALUATION_SCOPE_INSTANCE_DATA_OBJECT: EStateTreeDataSourceType = EStateTreeDataSourceType(
        8,
    );
    pub const EXECUTION_RUNTIME_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(
        9,
    );
    pub const EXECUTION_RUNTIME_DATA_OBJECT: EStateTreeDataSourceType = EStateTreeDataSourceType(
        10,
    );
    pub const CONTEXT_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(11);
    pub const EXTERNAL_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(12);
    pub const GLOBAL_PARAMETER_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(
        13,
    );
    pub const SUBTREE_PARAMETER_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(
        14,
    );
    pub const STATE_PARAMETER_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(
        15,
    );
    pub const TRANSITION_EVENT: EStateTreeDataSourceType = EStateTreeDataSourceType(16);
    pub const STATE_EVENT: EStateTreeDataSourceType = EStateTreeDataSourceType(17);
    pub const EXTERNAL_GLOBAL_PARAMETER_DATA: EStateTreeDataSourceType = EStateTreeDataSourceType(
        18,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreePropertyRefType(pub u8);
impl EStateTreePropertyRefType {
    pub const NONE: EStateTreePropertyRefType = EStateTreePropertyRefType(0);
    pub const BOOL: EStateTreePropertyRefType = EStateTreePropertyRefType(1);
    pub const BYTE: EStateTreePropertyRefType = EStateTreePropertyRefType(2);
    pub const INT32: EStateTreePropertyRefType = EStateTreePropertyRefType(3);
    pub const INT64: EStateTreePropertyRefType = EStateTreePropertyRefType(4);
    pub const FLOAT: EStateTreePropertyRefType = EStateTreePropertyRefType(5);
    pub const DOUBLE: EStateTreePropertyRefType = EStateTreePropertyRefType(6);
    pub const NAME: EStateTreePropertyRefType = EStateTreePropertyRefType(7);
    pub const STRING: EStateTreePropertyRefType = EStateTreePropertyRefType(8);
    pub const TEXT: EStateTreePropertyRefType = EStateTreePropertyRefType(9);
    pub const ENUM: EStateTreePropertyRefType = EStateTreePropertyRefType(10);
    pub const STRUCT: EStateTreePropertyRefType = EStateTreePropertyRefType(11);
    pub const OBJECT: EStateTreePropertyRefType = EStateTreePropertyRefType(12);
    pub const SOFT_OBJECT: EStateTreePropertyRefType = EStateTreePropertyRefType(13);
    pub const CLASS: EStateTreePropertyRefType = EStateTreePropertyRefType(14);
    pub const SOFT_CLASS: EStateTreePropertyRefType = EStateTreePropertyRefType(15);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeExpressionOperand(pub u8);
impl EStateTreeExpressionOperand {
    pub const COPY: EStateTreeExpressionOperand = EStateTreeExpressionOperand(0);
    pub const AND: EStateTreeExpressionOperand = EStateTreeExpressionOperand(1);
    pub const OR: EStateTreeExpressionOperand = EStateTreeExpressionOperand(2);
    pub const MULTIPLY: EStateTreeExpressionOperand = EStateTreeExpressionOperand(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeConditionEvaluationMode(pub u8);
impl EStateTreeConditionEvaluationMode {
    pub const EVALUATED: EStateTreeConditionEvaluationMode = EStateTreeConditionEvaluationMode(
        0,
    );
    pub const FORCED_TRUE: EStateTreeConditionEvaluationMode = EStateTreeConditionEvaluationMode(
        1,
    );
    pub const FORCED_FALSE: EStateTreeConditionEvaluationMode = EStateTreeConditionEvaluationMode(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeTransitionPriority(pub u8);
impl EStateTreeTransitionPriority {
    pub const NONE: EStateTreeTransitionPriority = EStateTreeTransitionPriority(0);
    pub const LOW: EStateTreeTransitionPriority = EStateTreeTransitionPriority(1);
    pub const NORMAL: EStateTreeTransitionPriority = EStateTreeTransitionPriority(2);
    pub const MEDIUM: EStateTreeTransitionPriority = EStateTreeTransitionPriority(3);
    pub const HIGH: EStateTreeTransitionPriority = EStateTreeTransitionPriority(4);
    pub const CRITICAL: EStateTreeTransitionPriority = EStateTreeTransitionPriority(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeExternalDataRequirement(pub u8);
impl EStateTreeExternalDataRequirement {
    pub const REQUIRED: EStateTreeExternalDataRequirement = EStateTreeExternalDataRequirement(
        0,
    );
    pub const OPTIONAL: EStateTreeExternalDataRequirement = EStateTreeExternalDataRequirement(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeSelectionFallback(pub u8);
impl EStateTreeSelectionFallback {
    pub const NONE: EStateTreeSelectionFallback = EStateTreeSelectionFallback(0);
    pub const NEXT_SELECTABLE_SIBLING: EStateTreeSelectionFallback = EStateTreeSelectionFallback(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeRunStatus(pub u8);
impl EStateTreeRunStatus {
    pub const RUNNING: EStateTreeRunStatus = EStateTreeRunStatus(0);
    pub const STOPPED: EStateTreeRunStatus = EStateTreeRunStatus(1);
    pub const SUCCEEDED: EStateTreeRunStatus = EStateTreeRunStatus(2);
    pub const FAILED: EStateTreeRunStatus = EStateTreeRunStatus(3);
    pub const UNSET: EStateTreeRunStatus = EStateTreeRunStatus(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeUpdatePhase(pub u8);
impl EStateTreeUpdatePhase {
    pub const UNSET: EStateTreeUpdatePhase = EStateTreeUpdatePhase(0);
    pub const START_TREE: EStateTreeUpdatePhase = EStateTreeUpdatePhase(1);
    pub const STOP_TREE: EStateTreeUpdatePhase = EStateTreeUpdatePhase(2);
    pub const START_GLOBAL_TASKS: EStateTreeUpdatePhase = EStateTreeUpdatePhase(3);
    pub const START_GLOBAL_TASKS_FOR_SELECTION: EStateTreeUpdatePhase = EStateTreeUpdatePhase(
        4,
    );
    pub const STOP_GLOBAL_TASKS: EStateTreeUpdatePhase = EStateTreeUpdatePhase(5);
    pub const STOP_GLOBAL_TASKS_FOR_SELECTION: EStateTreeUpdatePhase = EStateTreeUpdatePhase(
        6,
    );
    pub const TICK_STATE_TREE: EStateTreeUpdatePhase = EStateTreeUpdatePhase(7);
    pub const APPLY_TRANSITIONS: EStateTreeUpdatePhase = EStateTreeUpdatePhase(8);
    pub const TICK_TRANSITIONS: EStateTreeUpdatePhase = EStateTreeUpdatePhase(9);
    pub const TRIGGER_TRANSITIONS: EStateTreeUpdatePhase = EStateTreeUpdatePhase(10);
    pub const TICKING_GLOBAL_TASKS: EStateTreeUpdatePhase = EStateTreeUpdatePhase(11);
    pub const TICKING_TASKS: EStateTreeUpdatePhase = EStateTreeUpdatePhase(12);
    pub const TRANSITION_CONDITIONS: EStateTreeUpdatePhase = EStateTreeUpdatePhase(13);
    pub const STATE_SELECTION: EStateTreeUpdatePhase = EStateTreeUpdatePhase(14);
    pub const TRY_SELECT_BEHAVIOR: EStateTreeUpdatePhase = EStateTreeUpdatePhase(15);
    pub const ENTER_CONDITIONS: EStateTreeUpdatePhase = EStateTreeUpdatePhase(16);
    pub const ENTER_STATES: EStateTreeUpdatePhase = EStateTreeUpdatePhase(17);
    pub const EXIT_STATES: EStateTreeUpdatePhase = EStateTreeUpdatePhase(18);
    pub const STATE_COMPLETED: EStateTreeUpdatePhase = EStateTreeUpdatePhase(19);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeStateChangeType(pub u8);
impl EStateTreeStateChangeType {
    pub const NONE: EStateTreeStateChangeType = EStateTreeStateChangeType(0);
    pub const CHANGED: EStateTreeStateChangeType = EStateTreeStateChangeType(1);
    pub const SUSTAINED: EStateTreeStateChangeType = EStateTreeStateChangeType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeBindableStructSource(pub u8);
impl EStateTreeBindableStructSource {
    pub const CONTEXT: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        0,
    );
    pub const PARAMETER: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        1,
    );
    pub const EVALUATOR: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        2,
    );
    pub const GLOBAL_TASK: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        3,
    );
    pub const STATE_PARAMETER: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        4,
    );
    pub const TASK: EStateTreeBindableStructSource = EStateTreeBindableStructSource(5);
    pub const CONDITION: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        6,
    );
    pub const CONSIDERATION: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        7,
    );
    pub const TRANSITION_EVENT: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        8,
    );
    pub const STATE_EVENT: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        9,
    );
    pub const PROPERTY_FUNCTION: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        10,
    );
    pub const TRANSITION: EStateTreeBindableStructSource = EStateTreeBindableStructSource(
        11,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeTransitionTrigger(pub u8);
impl EStateTreeTransitionTrigger {
    pub const NONE: EStateTreeTransitionTrigger = EStateTreeTransitionTrigger(0);
    pub const ON_STATE_COMPLETED: EStateTreeTransitionTrigger = EStateTreeTransitionTrigger(
        3,
    );
    pub const ON_STATE_SUCCEEDED: EStateTreeTransitionTrigger = EStateTreeTransitionTrigger(
        1,
    );
    pub const ON_STATE_FAILED: EStateTreeTransitionTrigger = EStateTreeTransitionTrigger(
        2,
    );
    pub const ON_TICK: EStateTreeTransitionTrigger = EStateTreeTransitionTrigger(4);
    pub const ON_EVENT: EStateTreeTransitionTrigger = EStateTreeTransitionTrigger(8);
    pub const ON_DELEGATE: EStateTreeTransitionTrigger = EStateTreeTransitionTrigger(16);
    pub const MAX: EStateTreeTransitionTrigger = EStateTreeTransitionTrigger(17);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeTaskCompletionType(pub u8);
impl EStateTreeTaskCompletionType {
    pub const ALL: EStateTreeTaskCompletionType = EStateTreeTaskCompletionType(0);
    pub const ANY: EStateTreeTaskCompletionType = EStateTreeTaskCompletionType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeStateType(pub u8);
impl EStateTreeStateType {
    pub const STATE: EStateTreeStateType = EStateTreeStateType(0);
    pub const GROUP: EStateTreeStateType = EStateTreeStateType(1);
    pub const LINKED: EStateTreeStateType = EStateTreeStateType(2);
    pub const LINKED_ASSET: EStateTreeStateType = EStateTreeStateType(3);
    pub const SUBTREE: EStateTreeStateType = EStateTreeStateType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeStateSelectionBehavior(pub u8);
impl EStateTreeStateSelectionBehavior {
    pub const NONE: EStateTreeStateSelectionBehavior = EStateTreeStateSelectionBehavior(
        0,
    );
    pub const TRY_ENTER_STATE: EStateTreeStateSelectionBehavior = EStateTreeStateSelectionBehavior(
        1,
    );
    pub const TRY_SELECT_CHILDREN_IN_ORDER: EStateTreeStateSelectionBehavior = EStateTreeStateSelectionBehavior(
        2,
    );
    pub const TRY_SELECT_CHILDREN_AT_RANDOM: EStateTreeStateSelectionBehavior = EStateTreeStateSelectionBehavior(
        3,
    );
    pub const TRY_SELECT_CHILDREN_WITH_HIGHEST_UTILITY: EStateTreeStateSelectionBehavior = EStateTreeStateSelectionBehavior(
        4,
    );
    pub const TRY_SELECT_CHILDREN_AT_RANDOM_WEIGHTED_BY_UTILITY: EStateTreeStateSelectionBehavior = EStateTreeStateSelectionBehavior(
        5,
    );
    pub const TRY_FOLLOW_TRANSITIONS: EStateTreeStateSelectionBehavior = EStateTreeStateSelectionBehavior(
        6,
    );
    pub const TRY_SELECT_CHILDREN_AT_UNIFORM_RANDOM: EStateTreeStateSelectionBehavior = EStateTreeStateSelectionBehavior(
        3,
    );
    pub const TRY_SELECT_CHILDREN_BASED_ON_RELATIVE_UTILITY: EStateTreeStateSelectionBehavior = EStateTreeStateSelectionBehavior(
        5,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeTransitionType(pub u8);
impl EStateTreeTransitionType {
    pub const NONE: EStateTreeTransitionType = EStateTreeTransitionType(0);
    pub const SUCCEEDED: EStateTreeTransitionType = EStateTreeTransitionType(1);
    pub const FAILED: EStateTreeTransitionType = EStateTreeTransitionType(2);
    pub const GOTO_STATE: EStateTreeTransitionType = EStateTreeTransitionType(3);
    pub const NEXT_STATE: EStateTreeTransitionType = EStateTreeTransitionType(4);
    pub const NEXT_SELECTABLE_STATE: EStateTreeTransitionType = EStateTreeTransitionType(
        5,
    );
    pub const NOT_SET: EStateTreeTransitionType = EStateTreeTransitionType(6);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeBreakpointType(pub u8);
impl EStateTreeBreakpointType {
    pub const UNSET: EStateTreeBreakpointType = EStateTreeBreakpointType(0);
    pub const ON_ENTER: EStateTreeBreakpointType = EStateTreeBreakpointType(1);
    pub const ON_EXIT: EStateTreeBreakpointType = EStateTreeBreakpointType(2);
    pub const ON_TRANSITION: EStateTreeBreakpointType = EStateTreeBreakpointType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeNodeFormatting(pub u8);
impl EStateTreeNodeFormatting {
    pub const RICH_TEXT: EStateTreeNodeFormatting = EStateTreeNodeFormatting(0);
    pub const TEXT: EStateTreeNodeFormatting = EStateTreeNodeFormatting(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeParameterDataType(pub u8);
impl EStateTreeParameterDataType {
    pub const GLOBAL_PARAMETER_DATA: EStateTreeParameterDataType = EStateTreeParameterDataType(
        0,
    );
    pub const EXTERNAL_GLOBAL_PARAMETER_DATA: EStateTreeParameterDataType = EStateTreeParameterDataType(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeStateSelectionRules(pub u32);
impl EStateTreeStateSelectionRules {
    pub const NONE: EStateTreeStateSelectionRules = EStateTreeStateSelectionRules(0);
    pub const COMPLETED_TRANSITION_STATES_CREATE_NEW_STATES: EStateTreeStateSelectionRules = EStateTreeStateSelectionRules(
        1,
    );
    pub const COMPLETED_STATE_BEFORE_TRANSITION_SOURCE_FAILS_TRANSITION: EStateTreeStateSelectionRules = EStateTreeStateSelectionRules(
        2,
    );
    pub const RESELECTED_STATE_CREATES_NEW_STATES: EStateTreeStateSelectionRules = EStateTreeStateSelectionRules(
        4,
    );
    pub const DEFAULT: EStateTreeStateSelectionRules = EStateTreeStateSelectionRules(3);
}
