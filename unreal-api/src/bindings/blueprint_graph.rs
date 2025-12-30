#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FPerBlueprintSettings {
    pub breakpoints: TArray<FBlueprintBreakpoint>,
    pub watched_pins: TArray<FBlueprintWatchedPin>,
}
#[repr(C, align(8))]
pub struct FAdditionalBlueprintCategory {
    pub name: FText,
    pub class_filter: FSoftClassPath,
}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_K2Struct {}
#[repr(C, align(8))]
pub struct FBlueprintCallableFunctionRedirect {
    pub class_name: FString,
    pub old_function_name: FString,
    pub new_function_name: FString,
    pub blueprint_param_name: FString,
    pub class_param_name: FString,
}
#[repr(C, align(8))]
pub struct FAllowedMutableContainerFunction {
    pub function: UPtr<UFunction>,
    pub container_parameter_name: FString,
}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_K2NewNode {
    pub node_template: UPtr<UK2Node>,
    pub b_goto_node: bool,
}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_K2ViewNode {
    pub node_ptr: UPtr<UK2Node>,
}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_K2AssignDelegate {}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_EventFromFunction {
    pub signature_function: UPtr<UFunction>,
}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_K2AddComponent {
    pub component_class: TSubclassOf<UActorComponent>,
    pub component_asset: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_K2AddEvent {}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_K2AddCustomEvent {}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_K2AddCallOnActor {
    pub level_actors: TArray<UPtr<AActor>>,
}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_K2AddComment {}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_K2TargetNode {}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_K2PasteHere {}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_K2Enum {}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_BlueprintVariableBase {}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_K2Var {}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_K2LocalVar {}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_K2Graph {}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_K2Event {}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_K2InputAction {}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_K2Delegate {}
#[repr(C, align(8))]
pub struct FOptionalPinFromProperty {
    pub property_name: FName,
    pub property_friendly_name: FString,
    pub property_tooltip: FText,
    pub category_name: FName,
    pub flags_60: u8,
}
#[repr(C, align(4))]
pub struct FCustomPin {
    pub pin_name: FName,
    pub call_func_pin_name: FName,
    pub b_value_pin: bool,
}
#[repr(C, align(8))]
pub struct FUserPinInfo {
    pub pin_name: FName,
    pub pin_type: FEdGraphPinType,
    pub desired_pin_direction: EEdGraphPinDirection,
    pub pin_default_value: FString,
}
#[repr(C, align(8))]
pub struct FKismetUserDeclaredFunctionMetadata {
    pub tool_tip: FText,
    pub category: FText,
    pub keywords: FText,
    pub compact_node_title: FText,
    pub instance_title_color: FLinearColor,
    pub deprecation_message: FString,
    pub b_is_deprecated: bool,
    pub b_call_in_editor: bool,
    pub b_thread_safe: bool,
    pub b_is_unsafe_during_actor_construction: bool,
    pub has_latent_functions: i8,
    pub meta_data_map: TMap<FName, FString>,
}
pub struct UK2Node {}
pub struct UEdGraphSchema_K2 {
    pub editoronly_bp_function_redirects: TArray<FBlueprintCallableFunctionRedirect>,
    pub b_enable_mutable_container_warnings: bool,
    pub allowed_mutable_container_functions: TSet<FAllowedMutableContainerFunction>,
}
pub struct UK2Node_EditablePinBase {
    pub flags_192: u8,
}
pub struct UK2Node_Event {
    pub event_signature_name_deprecated: FName,
    pub event_signature_class_deprecated: TSubclassOf<UObject>,
    pub event_reference: FMemberReference,
    pub flags_304: u8,
    pub custom_function_name: FName,
    pub function_flags: u32,
}
pub struct UK2Node_ConstructObjectFromClass {}
pub struct UK2Node_BaseAsyncTask {
    pub proxy_factory_function_name: FName,
    pub proxy_factory_class: TSubclassOf<UObject>,
    pub proxy_class: TSubclassOf<UObject>,
    pub proxy_activate_function_name: FName,
}
pub struct UK2Node_BaseMCDelegate {
    pub delegate_reference: FMemberReference,
}
pub struct UK2Node_AddDelegate {}
pub struct UK2Node_CallDelegate {}
pub struct UK2Node_ClearDelegate {}
pub struct UK2Node_EventNodeInterface {}
pub struct IK2Node_EventNodeInterface {}
pub struct UK2Node_ExternalGraphInterface {}
pub struct IK2Node_ExternalGraphInterface {}
pub struct UK2Node_RemoveDelegate {}
pub struct UBlueprintNodeSpawner {
    pub node_class: TSubclassOf<UEdGraphNode>,
}
pub struct UBlueprintEventNodeSpawner {
    pub event_func: UPtr<UFunction>,
    pub custom_event_name: FName,
}
pub struct UAnimNotifyEventNodeSpawner {
    pub skeleton_object_path: FSoftObjectPath,
}
pub struct UBlueprintAssetNodeSpawner {
    pub asset_data: FAssetData,
}
pub struct UBlueprintBoundEventNodeSpawner {
    pub event_delegate: TFieldPath<FMulticastDelegateProperty>,
}
pub struct UBlueprintBoundNodeSpawner {}
pub struct UBlueprintComponentNodeSpawner {
    pub component_class: TSubclassOf<UActorComponent>,
    pub component_name: FString,
    pub component_asset_name: FString,
}
pub struct UBlueprintFieldNodeSpawner {
    pub owner_class: TSubclassOf<UObject>,
    pub field: UPtr<UField>,
    pub property: TFieldPath<FProperty>,
}
pub struct UBlueprintDelegateNodeSpawner {}
pub struct UBlueprintEditorSettings {
    pub b_draw_midpoint_arrows_in_blueprints: bool,
    pub b_show_graph_instruction_text: bool,
    pub b_hide_unrelated_nodes: bool,
    pub b_show_short_tooltips: bool,
    pub b_show_function_parameter_icon: bool,
    pub b_show_function_local_variable_icon: bool,
    pub b_enable_input_trigger_support_warnings: bool,
    pub b_split_context_target_settings: bool,
    pub b_expose_all_member_component_functions: bool,
    pub b_show_contextual_favorites: bool,
    pub b_expose_deprecated_functions: bool,
    pub b_compact_call_on_member_nodes: bool,
    pub b_flatten_favorites_menus: bool,
    pub b_auto_cast_object_connections: bool,
    pub b_show_viewport_on_simulate: bool,
    pub b_spawn_default_blueprint_nodes: bool,
    pub b_hide_construction_script_components_in_details_view: bool,
    pub b_host_find_in_blueprints_in_global_tab: bool,
    pub b_navigate_to_native_functions_from_call_nodes: bool,
    pub b_double_click_navigates_to_parent: bool,
    pub b_enable_type_promotion: bool,
    pub b_show_panel_context_menu_for_incompatible_connections: bool,
    pub type_promotion_pin_deny_list: TSet<FName>,
    pub additional_blueprint_categories: TArray<FAdditionalBlueprintCategory>,
    pub breakpoint_reload_method: EBlueprintBreakpointReloadMethod,
    pub b_enable_pin_value_inspection_tooltips: bool,
    pub b_enable_namespace_editor_features: bool,
    pub namespaces_to_always_include: TArray<FString>,
    pub b_enable_context_menu_time_slicing: bool,
    pub context_menu_time_slicing_threshold_ms: i32,
    pub b_include_actions_for_selected_assets_in_context_menu: bool,
    pub b_limit_asset_action_binding_to_single_selection_only: bool,
    pub b_load_selected_assets_for_context_menu_action_binding: bool,
    pub b_do_not_mark_all_instances_dirty_on_default_value_change: bool,
    pub b_favor_pure_cast_nodes: bool,
    pub save_on_compile: ESaveOnCompile,
    pub b_jump_to_node_errors: bool,
    pub b_allow_explicit_impure_node_disabling: bool,
    pub b_show_action_menu_item_signatures: bool,
    pub b_blueprint_node_unique_names: bool,
    pub node_template_cache_cap_mb: f32,
    pub allow_index_all_blueprints: EFiBIndexAllPermission,
    pub b_search_for_references_when_variable_type_changed: bool,
    pub b_show_inherited_variables: bool,
    pub b_always_show_interfaces_in_overrides: bool,
    pub b_show_parent_class_in_overrides: bool,
    pub b_show_empty_sections: bool,
    pub b_show_access_specifier: bool,
    pub bookmarks: TMap<FGuid, FEditedDocumentInfo>,
    pub bookmark_nodes: TArray<FBPEditorBookmarkNode>,
    pub per_blueprint_settings: TMap<FString, FPerBlueprintSettings>,
    pub b_include_comment_nodes_in_bookmarks_tab: bool,
    pub b_show_bookmarks_for_current_document_only_in_tab: bool,
    pub graph_editor_quick_jumps: TMap<i32, FEditedDocumentInfo>,
    pub b_enable_namespace_filtering_features: bool,
    pub b_enable_namespace_importing_features: bool,
    pub b_inherit_imported_namespaces_from_parent_bp: bool,
    pub base_classes_to_allow_recompiling_during_play_in_editor: TArray<
        TSoftObjectPtr<UClass>,
    >,
    pub base_classes_to_disallow_recompiling_during_play_in_editor: TArray<
        TSoftObjectPtr<UClass>,
    >,
}
pub struct UBlueprintFunctionNodeSpawner {}
pub struct UBlueprintVariableNodeSpawner {
    pub local_var_outer: UPtr<UEdGraph>,
    pub local_var_desc: FBPVariableDescription,
}
pub struct UK2Node_ActorBoundEvent {
    pub delegate_property_name: FName,
    pub delegate_owner_class: TSubclassOf<UObject>,
    pub event_owner: UPtr<AActor>,
}
pub struct UK2Node_CallFunction {
    pub flags_192: u8,
    pub function_reference: FMemberReference,
    pub call_function_name_deprecated: FName,
    pub call_function_class_deprecated: TSubclassOf<UObject>,
    pub node_purity_override: ENodePurityOverride,
}
pub struct UK2Node_AddComponent {
    pub flags_336: u8,
    pub template_blueprint: FString,
    pub template_type: TSubclassOf<UObject>,
}
pub struct UK2Node_AddComponentByClass {}
pub struct UK2Node_AddPinInterface {}
pub struct IK2Node_AddPinInterface {}
pub struct UK2Node_AssignDelegate {}
pub struct UK2Node_AssignmentStatement {}
pub struct UK2Node_AsyncAction {}
pub struct UK2Node_BitmaskLiteral {
    pub bitflags_enum: UPtr<UEnum>,
}
pub struct UK2Node_Variable {
    pub variable_reference: FMemberReference,
    pub self_context_info: ESelfContextInfo,
    pub variable_source_class_deprecated: TSubclassOf<UObject>,
    pub variable_name_deprecated: FName,
    pub flags_276: u8,
}
pub struct UK2Node_StructOperation {
    pub struct_type: UPtr<UScriptStruct>,
}
pub struct UK2Node_StructMemberGet {
    pub show_pin_for_properties: TArray<FOptionalPinFromProperty>,
}
pub struct UK2Node_BreakStruct {
    pub b_made_after_override_pin_removal: bool,
}
pub struct UK2Node_CallArrayFunction {}
pub struct UK2Node_CallDataTableFunction {}
pub struct UK2Node_CallFunctionOnMember {
    pub member_variable_to_call_on: FMemberReference,
}
pub struct UK2Node_CallMaterialParameterCollectionFunction {}
pub struct UK2Node_CallParentFunction {}
pub struct UK2Node_CastByteToEnum {
    pub enum_: UPtr<UEnum>,
    pub b_safe: bool,
}
pub struct UK2Node_DynamicCast {
    pub target_type: TSubclassOf<UObject>,
    pub b_is_pure_cast_deprecated: bool,
}
pub struct UK2Node_ClassDynamicCast {}
pub struct UK2Node_CommutativeAssociativeBinaryOperator {
    pub num_additional_inputs: i32,
}
pub struct UK2Node_ComponentBoundEvent {
    pub delegate_property_name: FName,
    pub delegate_owner_class: TSubclassOf<UObject>,
    pub component_property_name: FName,
}
pub struct UK2Node_Tunnel {
    pub output_source_node: UPtr<UK2Node_Tunnel>,
    pub input_sink_node: UPtr<UK2Node_Tunnel>,
    pub flags_232: u8,
    pub meta_data: FKismetUserDeclaredFunctionMetadata,
}
pub struct UK2Node_Composite {
    pub bound_graph: UPtr<UEdGraph>,
}
pub struct UK2Node_ConvertAsset {}
pub struct UK2Node_Copy {}
pub struct UK2Node_CreateDelegate {
    pub selected_function_name: FName,
    pub selected_function_guid: FGuid,
}
pub struct UK2Node_CustomEvent {
    pub deprecation_message: FString,
    pub b_is_deprecated: bool,
    pub b_call_in_editor: bool,
    pub meta_data: FKismetUserDeclaredFunctionMetadata,
}
pub struct UK2Node_DeadClass {}
pub struct UK2Node_DelegateSet {
    pub delegate_property_name: FName,
    pub delegate_property_class: TSubclassOf<UObject>,
}
pub struct UK2Node_DoOnceMultiInput {
    pub num_additional_inputs: i32,
    pub data_node: UPtr<UK2Node_TemporaryVariable>,
}
pub struct UK2Node_EaseFunction {
    pub ease_function_name: FName,
}
pub struct UK2Node_EditorPropertyAccessBase {}
pub struct UK2Node_GetEditorProperty {}
pub struct UK2Node_SetEditorProperty {}
pub struct UK2Node_EnumEquality {}
pub struct UK2Node_EnumInequality {}
pub struct UK2Node_EnumLiteral {
    pub enum_: UPtr<UEnum>,
}
pub struct UK2Node_ExecutionSequence {}
pub struct UK2Node_ForEachElementInEnum {
    pub enum_: UPtr<UEnum>,
}
pub struct UK2Node_FormatText {
    pub pin_names: TArray<FName>,
}
pub struct UK2Node_FunctionTerminator {
    pub function_reference: FMemberReference,
    pub signature_class_deprecated: TSubclassOf<UObject>,
    pub signature_name_deprecated: FName,
}
pub struct UK2Node_FunctionEntry {
    pub custom_generated_function_name: FName,
    pub meta_data: FKismetUserDeclaredFunctionMetadata,
    pub local_variables: TArray<FBPVariableDescription>,
    pub b_enforce_const_correctness: bool,
    pub extra_flags: i32,
}
pub struct UK2Node_FunctionResult {}
pub struct UK2Node_GeneratedBoundEvent {
    pub delegate_property_name: FName,
    pub delegate_owner_class: TSubclassOf<UObject>,
}
pub struct UK2Node_GenericCreateObject {}
pub struct UK2Node_GenericToText {}
pub struct UK2Node_GetArrayItem {
    pub b_return_by_ref_desired: bool,
}
pub struct UK2Node_GetClassDefaults {
    pub blueprint_subscribed_to: UPtr<UBlueprint>,
    pub show_pin_for_properties: TArray<FOptionalPinFromProperty>,
    pub b_exclude_object_containers: bool,
    pub b_exclude_object_arrays_deprecated: bool,
}
pub struct UK2Node_GetDataTableRow {}
pub struct UK2Node_GetEnumeratorName {}
pub struct UK2Node_GetEnumeratorNameAsString {}
pub struct UK2Node_GetInputAxisKeyValue {
    pub input_axis_key: FKey,
    pub flags_368: u8,
}
pub struct UK2Node_GetInputAxisValue {
    pub input_axis_name: FName,
    pub flags_348: u8,
}
pub struct UK2Node_GetInputVectorAxisValue {}
pub struct UK2Node_GetNumEnumEntries {
    pub enum_: UPtr<UEnum>,
}
pub struct UK2Node_GetSubsystem {
    pub custom_class: TSubclassOf<USubsystem>,
}
pub struct UK2Node_GetSubsystemFromPC {}
pub struct UK2Node_GetEngineSubsystem {}
pub struct UK2Node_GetEditorSubsystem {}
pub struct UK2Node_IfThenElse {}
pub struct UK2Node_InputAction {
    pub input_action_name: FName,
    pub flags_212: u8,
}
pub struct UK2Node_InputActionEvent {
    pub input_action_name: FName,
    pub input_key_event: EInputEvent,
    pub flags_368: u8,
}
pub struct UK2Node_InputAxisEvent {
    pub input_axis_name: FName,
    pub flags_364: u8,
}
pub struct UK2Node_InputAxisKeyEvent {
    pub axis_key: FKey,
    pub flags_384: u8,
}
pub struct UK2Node_InputKey {
    pub input_key: FKey,
    pub flags_232: u8,
}
pub struct UK2Node_InputKeyEvent {
    pub input_chord: FInputChord,
    pub input_key_event: EInputEvent,
    pub flags_396: u8,
}
pub struct UK2Node_InputTouch {
    pub flags_200: u8,
}
pub struct UK2Node_InputTouchEvent {
    pub input_key_event: EInputEvent,
    pub flags_353: u8,
}
pub struct UK2Node_InputVectorAxisEvent {}
pub struct UK2Node_InstancedStruct {}
pub struct UK2Node_Knot {}
pub struct UK2Node_Literal {
    pub object_ref: UPtr<UObject>,
}
pub struct UK2Node_LoadAsset {}
pub struct UK2Node_LoadAssetClass {}
pub struct UK2Node_LoadAssets {}
pub struct UK2Node_TemporaryVariable {
    pub variable_type: FEdGraphPinType,
    pub b_is_persistent: bool,
}
pub struct UDEPRECATED_K2Node_LocalVariable {
    pub custom_variable_name: FName,
    pub variable_tooltip: FText,
}
pub struct UK2Node_MacroInstance {
    pub macro_graph_deprecated: UPtr<UEdGraph>,
    pub macro_graph_reference: FGraphReference,
    pub resolved_wildcard_type: FEdGraphPinType,
}
pub struct UK2Node_MakeContainer {
    pub num_inputs: i32,
}
pub struct UK2Node_MakeArray {}
pub struct UK2Node_MakeMap {}
pub struct UK2Node_MakeSet {}
pub struct UK2Node_StructMemberSet {
    pub show_pin_for_properties: TArray<FOptionalPinFromProperty>,
}
pub struct UK2Node_MakeStruct {
    pub b_made_after_override_pin_removal: bool,
}
pub struct UK2Node_MakeVariable {
    pub variable_type: FBPVariableDescription,
}
pub struct UK2Node_MapForEach {
    pub key_name: FString,
    pub value_name: FString,
}
pub struct UK2Node_MathExpression {
    pub expression: FString,
    pub b_made_after_rot_change: bool,
}
pub struct UK2Node_Message {}
pub struct UK2Node_MultiGate {
    pub data_node: UPtr<UK2Node_TemporaryVariable>,
}
pub struct UK2Node_PromotableOperator {
    pub operation_name: FName,
    pub num_additional_inputs: i32,
}
pub struct UK2Node_PureAssignmentStatement {}
pub struct UK2Node_Select {
    pub num_option_pins: i32,
    pub index_pin_type: FEdGraphPinType,
    pub enum_: UPtr<UEnum>,
    pub enum_entries: TArray<FName>,
    pub enum_entry_friendly_names: TArray<FText>,
    pub flags_368: u8,
}
pub struct UK2Node_Self {}
pub struct UK2Node_SetFieldsInStruct {}
pub struct UK2Node_SetForEach {
    pub value_name: FString,
}
pub struct UK2Node_SetVariableOnPersistentFrame {}
pub struct UK2Node_SpawnActor {}
pub struct UK2Node_SpawnActorFromClass {}
pub struct UK2Node_Switch {
    pub flags_192: u8,
    pub function_name: FName,
    pub function_class: TSubclassOf<UObject>,
}
pub struct UK2Node_SwitchEnum {
    pub enum_: UPtr<UEnum>,
    pub enum_entries: TArray<FName>,
    pub enum_friendly_names: TArray<FText>,
}
pub struct UK2Node_SwitchInteger {
    pub start_index: i32,
}
pub struct UK2Node_SwitchName {
    pub pin_names: TArray<FName>,
}
pub struct UK2Node_SwitchString {
    pub pin_names: TArray<FName>,
    pub flags_240: u8,
}
pub struct UK2Node_Timeline {
    pub timeline_name: FName,
    pub flags_204: u8,
    pub timeline_guid: FGuid,
    pub flags_224: u8,
}
pub struct UK2Node_TunnelBoundary {
    pub base_name: FName,
    pub tunnel_boundary_type: ETunnelBoundaryType,
}
pub struct UK2Node_VariableGet {
    pub b_is_pure_get_deprecated: bool,
    pub current_variation: EGetNodeVariation,
}
pub struct UK2Node_VariableSet {}
pub struct UK2Node_VariableSetRef {}
pub struct UNodeDependingOnEnumInterface {}
pub struct INodeDependingOnEnumInterface {}
