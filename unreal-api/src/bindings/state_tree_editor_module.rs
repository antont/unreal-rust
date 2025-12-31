#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FClipboardEditorData {
    pub editor_nodes_buffer: TArray<FStateTreeEditorNode>,
    pub transitions_buffer: TArray<FStateTreeTransition>,
    pub bindings_buffer: TArray<
        crate::bindings::state_tree_module::FStateTreePropertyPathBinding,
    >,
}
#[repr(C, align(8))]
pub struct FStateTreeTransition {
    pub trigger: crate::bindings::state_tree_module::EStateTreeTransitionTrigger,
    pub required_event: FStateTreeEventDesc,
    pub state: crate::bindings::state_tree_module::FStateTreeStateLink,
    pub id: crate::bindings::core_u_object::FGuid,
    pub delegate_listener: FStateTreeTransitionDelegateListener,
    pub priority: crate::bindings::state_tree_module::EStateTreeTransitionPriority,
    pub b_delay_transition: bool,
    pub delay_duration: f32,
    pub delay_random_variance: f32,
    pub conditions: TArray<FStateTreeEditorNode>,
    pub b_transition_enabled: bool,
    pub event_tag_deprecated: crate::bindings::gameplay_tags::FGameplayTag,
}
#[repr(C, align(8))]
pub struct FStateTreeEditorNode {
    pub node: crate::bindings::core_u_object::FInstancedStruct,
    pub instance: crate::bindings::core_u_object::FInstancedStruct,
    pub instance_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub execution_runtime_data: crate::bindings::core_u_object::FInstancedStruct,
    pub execution_runtime_data_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub id: crate::bindings::core_u_object::FGuid,
    pub expression_indent: u8,
    pub expression_operand: crate::bindings::state_tree_module::EStateTreeExpressionOperand,
}
#[repr(C, align(1))]
pub struct FStateTreeTransitionDelegateListener {}
#[repr(C, align(8))]
pub struct FStateTreeEventDesc {
    pub tag: crate::bindings::gameplay_tags::FGameplayTag,
    pub payload_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub b_consume_event_on_select: bool,
    pub temporary_event: crate::bindings::state_tree_module::FStateTreeEvent,
}
#[repr(C, align(8))]
pub struct FStateItemLinkData {}
#[repr(C, align(8))]
pub struct FStateTreeCompilerLogMessage {
    pub severity: i32,
    pub state: UPtr<UStateTreeState>,
    pub item: crate::bindings::state_tree_module::FStateTreeBindableStructDesc,
    pub message: FString,
}
#[repr(C, align(8))]
pub struct FStateTreeCompilerLog {
    pub state_stack: TArray<UPtr<UStateTreeState>>,
    pub messages: TArray<FStateTreeCompilerLogMessage>,
}
#[repr(C, align(4))]
pub struct FStateTreeEditorBreakpoint {
    pub id: crate::bindings::core_u_object::FGuid,
    pub breakpoint_type: crate::bindings::state_tree_module::EStateTreeBreakpointType,
}
#[repr(C, align(16))]
pub struct FStateTreeEditorPropertyBindings {
    pub property_bindings: TArray<
        crate::bindings::state_tree_module::FStateTreePropertyPathBinding,
    >,
}
#[repr(C, align(4))]
pub struct FStateTreeEditorColorRef {
    pub id: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FStateTreeEditorColor {
    pub color_ref: FStateTreeEditorColorRef,
    pub display_name: FString,
    pub color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FStateTreeEditorDelegateDispatcherCompiledBinding {
    pub dispatcher_path: crate::bindings::property_binding_utils::FPropertyBindingPath,
    pub id: crate::bindings::state_tree_module::FStateTreeDelegateDispatcher,
}
#[repr(C, align(4))]
pub struct FStateTreeNodeClassData {}
#[repr(C, align(8))]
pub struct FStateTreePropertyBindingCompiler {
    pub source_structs: TArray<
        crate::bindings::state_tree_module::FStateTreeBindableStructDesc,
    >,
}
#[repr(C, align(8))]
pub struct FStateTreeStateParameters {
    pub parameters: crate::bindings::core_u_object::FInstancedPropertyBag,
    pub property_overrides: TArray<crate::bindings::core_u_object::FGuid>,
    pub b_fixed_layout: bool,
    pub id: crate::bindings::core_u_object::FGuid,
}
pub struct UStateTreeEditorContext {}
pub struct UAssetDefinition_StateTree {}
pub struct UStateTreeCompileAllCommandlet {}
pub struct UStateTreePropertyRefSchema {}
pub struct UK2Node_StateTreeBlueprintPropertyRef {}
pub struct UK2Node_StateTreeNodeGetPropertyDescription {
    pub variable: crate::bindings::engine::FMemberReference,
}
pub struct UK2Node_MakeStateTreeReference {
    pub show_pin_for_properties: TArray<
        crate::bindings::blueprint_graph::FOptionalPinFromProperty,
    >,
    pub state_tree: UPtr<crate::bindings::state_tree_module::UStateTree>,
}
pub struct UStateTreeEditingSubsystem {}
pub struct UStateTreeEditorData {
    pub schema: UPtr<crate::bindings::state_tree_module::UStateTreeSchema>,
    pub editor_schema: UPtr<UStateTreeEditorSchema>,
    pub extensions: TArray<UPtr<UStateTreeEditorDataExtension>>,
    pub root_parameters: FStateTreeStateParameters,
    pub root_parameters_guid: crate::bindings::core_u_object::FGuid,
    pub root_parameter_property_bag: crate::bindings::core_u_object::FInstancedPropertyBag,
    pub evaluators: TArray<FStateTreeEditorNode>,
    pub global_tasks: TArray<FStateTreeEditorNode>,
    pub global_tasks_completion: crate::bindings::state_tree_module::EStateTreeTaskCompletionType,
    pub editor_bindings: FStateTreeEditorPropertyBindings,
    pub colors: TSet<FStateTreeEditorColor>,
    pub sub_trees: TArray<UPtr<UStateTreeState>>,
    pub breakpoints: TArray<FStateTreeEditorBreakpoint>,
    pub compiled_dispatchers: TArray<FStateTreeEditorDelegateDispatcherCompiledBinding>,
}
pub struct UQAStateTreeEditorData {}
pub struct UStateTreeEditorDataExtension {}
pub struct UStateTreeEditorMode {}
pub struct UStateTreeEditorPropertyBindingsOwner {}
pub struct IStateTreeEditorPropertyBindingsOwner {}
pub struct UStateTreeEditorSchema {}
pub struct UStateTreeEditorSettings {
    pub save_on_compile: EStateTreeSaveOnCompile,
    pub b_enable_legacy_debugger_window: bool,
    pub b_should_debugger_auto_record_on_pie: bool,
    pub b_should_debugger_reset_data_on_new_pie_session: bool,
    pub b_retain_node_property_values: bool,
}
pub struct UStateTreeEditorUISubsystem {}
pub struct UStateTreeEditorUserSettings {
    pub states_view_display_node_type: EStateTreeEditorUserSettingsNodeType,
    pub states_view_state_row_height: f32,
    pub states_view_node_row_height: f32,
}
pub struct UStateTreeFactory {
    pub state_tree_schema_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
pub struct UStateTreeState {
    pub name: FName,
    pub description: FString,
    pub tag: crate::bindings::gameplay_tags::FGameplayTag,
    pub color_ref: FStateTreeEditorColorRef,
    pub ty: crate::bindings::state_tree_module::EStateTreeStateType,
    pub selection_behavior: crate::bindings::state_tree_module::EStateTreeStateSelectionBehavior,
    pub tasks_completion: crate::bindings::state_tree_module::EStateTreeTaskCompletionType,
    pub linked_subtree: crate::bindings::state_tree_module::FStateTreeStateLink,
    pub linked_asset: UPtr<crate::bindings::state_tree_module::UStateTree>,
    pub custom_tick_rate: f32,
    pub b_has_custom_tick_rate: bool,
    pub parameters: FStateTreeStateParameters,
    pub b_check_prerequisites_when_activating_child_directly: bool,
    pub b_has_required_event_to_enter: bool,
    pub required_event_to_enter: FStateTreeEventDesc,
    pub weight: f32,
    pub enter_conditions: TArray<FStateTreeEditorNode>,
    pub tasks: TArray<FStateTreeEditorNode>,
    pub considerations: TArray<FStateTreeEditorNode>,
    pub single_task: FStateTreeEditorNode,
    pub transitions: TArray<FStateTreeTransition>,
    pub children: TArray<UPtr<UStateTreeState>>,
    pub id: crate::bindings::core_u_object::FGuid,
    pub b_expanded: bool,
    pub b_enabled: bool,
    pub parent: UPtr<UStateTreeState>,
}
pub struct UStateTreeClipboardBindings {
    pub bindings: TArray<
        crate::bindings::state_tree_module::FStateTreePropertyPathBinding,
    >,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeSaveOnCompile(pub u8);
impl EStateTreeSaveOnCompile {
    pub const NEVER: EStateTreeSaveOnCompile = EStateTreeSaveOnCompile(0);
    pub const SUCCESS_ONLY: EStateTreeSaveOnCompile = EStateTreeSaveOnCompile(1);
    pub const ALWAYS: EStateTreeSaveOnCompile = EStateTreeSaveOnCompile(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStateTreeEditorUserSettingsNodeType(pub u8);
impl EStateTreeEditorUserSettingsNodeType {
    pub const CONDITION: EStateTreeEditorUserSettingsNodeType = EStateTreeEditorUserSettingsNodeType(
        1,
    );
    pub const TASK: EStateTreeEditorUserSettingsNodeType = EStateTreeEditorUserSettingsNodeType(
        2,
    );
    pub const TRANSITION: EStateTreeEditorUserSettingsNodeType = EStateTreeEditorUserSettingsNodeType(
        4,
    );
    pub const FLAG: EStateTreeEditorUserSettingsNodeType = EStateTreeEditorUserSettingsNodeType(
        8,
    );
    pub const ALL: EStateTreeEditorUserSettingsNodeType = EStateTreeEditorUserSettingsNodeType(
        15,
    );
}
