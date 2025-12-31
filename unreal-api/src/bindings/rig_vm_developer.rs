#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FRigVMGraphVariableDescription {
    pub name: FName,
    pub cpp_type: FString,
    pub cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub cpp_type_object_path: FName,
    pub default_value: FString,
    pub category: FText,
    pub tooltip: FText,
    pub b_exposed_on_spawn: bool,
    pub b_expose_to_cinematics: bool,
    pub b_public: bool,
    pub b_private: bool,
}
#[repr(C, align(8))]
pub struct FRigVMEdGraphSchemaAction_LocalVar {}
#[repr(C, align(8))]
pub struct FRigVMEdGraphSchemaAction_PromoteToVariable {}
#[repr(C, align(8))]
pub struct FRigVMEdGraphSchemaAction_PromoteToExposedPin {}
#[repr(C, align(8))]
pub struct FRigVMEdGraphSchemaAction_Event {}
#[repr(C, align(1))]
pub struct FRigVMPythonSettings {}
#[repr(C, align(8))]
pub struct FRigVMEdGraphDisplaySettings {
    pub b_show_node_instruction_index: bool,
    pub b_show_node_run_counts: bool,
    pub node_run_lower_bound: i32,
    pub node_run_limit: i32,
    pub min_micro_seconds: f64,
    pub max_micro_seconds: f64,
    pub total_micro_seconds: f64,
    pub average_frames: i32,
    pub b_auto_determine_range: bool,
    pub last_min_micro_seconds: f64,
    pub last_max_micro_seconds: f64,
    pub min_duration_color: crate::bindings::core_u_object::FLinearColor,
    pub max_duration_color: crate::bindings::core_u_object::FLinearColor,
    pub tag_display_mode: ERigVMTagDisplayMode,
}
#[repr(C, align(4))]
pub struct FRigVMOldPublicFunctionArg {
    pub name: FName,
    pub cpp_type: FName,
    pub cpp_type_object_path: FName,
    pub b_is_array: bool,
    pub direction: crate::bindings::rig_vm::ERigVMPinDirection,
}
#[repr(C, align(8))]
pub struct FRigVMOldPublicFunctionData {
    pub name: FName,
    pub display_name: FString,
    pub category: FString,
    pub keywords: FString,
    pub return_value: FRigVMOldPublicFunctionArg,
    pub arguments: TArray<FRigVMOldPublicFunctionArg>,
}
#[repr(C, align(8))]
pub struct FRigVMParserASTSettings {
    pub b_fold_assignments: bool,
    pub b_fold_sub_pin_copies: bool,
    pub b_fold_literals: bool,
    pub b_setup_traits: bool,
    pub links_to_skip: TArray<UPtr<URigVMLink>>,
    pub execute_context_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
}
#[repr(C, align(8))]
pub struct FRigVMCompileSettings {
    pub surpress_info_messages: bool,
    pub surpress_warnings: bool,
    pub surpress_errors: bool,
    pub enable_pin_watches: bool,
    pub is_preprocessor_phase: bool,
    pub ast_settings: FRigVMParserASTSettings,
    pub setup_node_instruction_index: bool,
    pub ast_errors_as_notifications: bool,
    pub b_warn_about_deprecated_nodes: bool,
    pub b_warn_about_duplicate_events: bool,
}
#[repr(C, align(8))]
pub struct FRigVMGraphParameterDescription {
    pub name: FName,
    pub b_is_input: bool,
    pub cpp_type: FString,
    pub cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub default_value: FString,
}
#[repr(C, align(8))]
pub struct FRigVMTemplatePreferredType {
    pub argument: FName,
    pub type_index: i32,
    pub type_string: FString,
}
#[repr(C, align(8))]
pub struct FRigVMFunctionReferenceArray {
    pub function_references: TArray<TSoftObjectPtr<URigVMFunctionReferenceNode>>,
}
#[repr(C, align(8))]
pub struct FRigVMReferenceNodeData {
    pub reference_node_path: FString,
    pub referenced_function_path_deprecated: FString,
    pub referenced_header_deprecated: crate::bindings::rig_vm::FRigVMGraphFunctionHeader,
    pub referenced_function_identifier: crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
}
#[repr(C, align(8))]
pub struct FRigVMClient {
    pub default_schema_class: TSubclassOf<URigVMSchema>,
    pub controller_class: TSubclassOf<URigVMController>,
    pub models: TArray<UPtr<URigVMGraph>>,
    pub function_library: UPtr<URigVMFunctionLibrary>,
    pub controllers: TMap<
        crate::bindings::core_u_object::FSoftObjectPath,
        UPtr<URigVMController>,
    >,
    pub action_stack: UPtr<URigVMActionStack>,
    pub undo_redo_index: i32,
}
#[repr(C, align(8))]
pub struct FRigVMActionKey {
    pub script_struct_path: FString,
    pub exported_text: FString,
}
#[repr(C, align(8))]
pub struct FRigVMActionNodeContent {
    pub old: FString,
    pub new: FString,
}
#[repr(C, align(8))]
pub struct FRigVMBaseAction {
    pub controller_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub client_host_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub controller_model_path: FString,
    pub title: FString,
    pub sub_actions: TArray<FRigVMActionKey>,
    pub exported_nodes: TMap<FName, FRigVMActionNodeContent>,
}
#[repr(C, align(8))]
pub struct FRigVMInjectNodeIntoPinAction {
    pub pin_path: FString,
    pub b_as_input: bool,
    pub input_pin_name: FName,
    pub output_pin_name: FName,
    pub node_path: FString,
}
#[repr(C, align(8))]
pub struct FRigVMEjectNodeFromPinAction {}
#[repr(C, align(8))]
pub struct FRigVMRemoveNodesAction {
    pub node_names: TArray<FName>,
    pub exported_content: FString,
}
#[repr(C, align(8))]
pub struct FRigVMSetNodeSelectionAction {
    pub new_selection: TArray<FName>,
    pub old_selection: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FRigVMSetNodePositionAction {
    pub node_path: FString,
    pub old_position: crate::bindings::core_u_object::FVector2D,
    pub new_position: crate::bindings::core_u_object::FVector2D,
}
#[repr(C, align(8))]
pub struct FRigVMSetNodeSizeAction {
    pub node_path: FString,
    pub old_size: crate::bindings::core_u_object::FVector2D,
    pub new_size: crate::bindings::core_u_object::FVector2D,
}
#[repr(C, align(8))]
pub struct FRigVMSetNodeTitleAction {
    pub node_path: FString,
    pub old_title: FString,
    pub new_title: FString,
}
#[repr(C, align(8))]
pub struct FRigVMSetNodeColorAction {
    pub node_path: FString,
    pub old_color: crate::bindings::core_u_object::FLinearColor,
    pub new_color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FRigVMSetNodeCategoryAction {
    pub node_path: FString,
    pub old_category: FString,
    pub new_category: FString,
}
#[repr(C, align(8))]
pub struct FRigVMSetNodeKeywordsAction {
    pub node_path: FString,
    pub old_keywords: FString,
    pub new_keywords: FString,
}
#[repr(C, align(8))]
pub struct FRigVMSetNodeDescriptionAction {
    pub node_path: FString,
    pub old_description: FString,
    pub new_description: FString,
}
#[repr(C, align(8))]
pub struct FRigVMSetCommentTextAction {
    pub node_path: FString,
    pub old_text: FString,
    pub new_text: FString,
    pub old_font_size: i32,
    pub new_font_size: i32,
    pub b_old_bubble_visible: bool,
    pub b_new_bubble_visible: bool,
    pub b_old_color_bubble: bool,
    pub b_new_color_bubble: bool,
}
#[repr(C, align(8))]
pub struct FRigVMRenameVariableAction {
    pub old_variable_name: FString,
    pub new_variable_name: FString,
}
#[repr(C, align(8))]
pub struct FRigVMSetPinExpansionAction {
    pub pin_path: FString,
    pub old_is_expanded: bool,
    pub new_is_expanded: bool,
}
#[repr(C, align(8))]
pub struct FRigVMSetPinWatchAction {
    pub pin_path: FString,
    pub old_is_watched: bool,
    pub new_is_watched: bool,
}
#[repr(C, align(8))]
pub struct FRigVMSetPinDisplayNameAction {
    pub pin_path: FString,
    pub old_display_name: FString,
    pub new_display_name: FString,
}
#[repr(C, align(8))]
pub struct FRigVMSetPinCategoryAction {
    pub pin_path: FString,
    pub old_category: FString,
    pub old_pin_index: i32,
    pub new_category: FString,
}
#[repr(C, align(8))]
pub struct FRigVMChangeNodePinCategoriesAction {
    pub node_name: FString,
    pub old_categories: TArray<FString>,
    pub new_categories: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FRigVMSetPinCategoryExpansionAction {
    pub node_name: FString,
    pub pin_category: FString,
    pub b_old_expansion_state: bool,
}
#[repr(C, align(8))]
pub struct FRigVMSetPinIndexInCategoryAction {
    pub pin_path: FString,
    pub old_index_in_category: i32,
    pub new_index_in_category: i32,
}
#[repr(C, align(8))]
pub struct FRigVMSetPinDefaultValueAction {
    pub pin_path: FString,
    pub old_default_value: FString,
    pub new_default_value: FString,
    pub old_default_value_type: ERigVMPinDefaultValueType,
    pub new_default_value_type: ERigVMPinDefaultValueType,
}
#[repr(C, align(8))]
pub struct FRigVMInsertArrayPinAction {
    pub array_pin_path: FString,
    pub index: i32,
    pub new_default_value: FString,
    pub new_default_value_type: ERigVMPinDefaultValueType,
}
#[repr(C, align(8))]
pub struct FRigVMRemoveArrayPinAction {
    pub array_pin_path: FString,
    pub index: i32,
    pub default_value: FString,
    pub default_value_type: ERigVMPinDefaultValueType,
}
#[repr(C, align(8))]
pub struct FRigVMAddLinkAction {
    pub output_pin_path: FString,
    pub input_pin_path: FString,
}
#[repr(C, align(8))]
pub struct FRigVMBreakLinkAction {
    pub graph_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub output_pin_path: FString,
    pub input_pin_path: FString,
}
#[repr(C, align(8))]
pub struct FRigVMChangePinTypeAction {
    pub pin_path: FString,
    pub old_type_index: i32,
    pub new_type_index: i32,
    pub b_setup_orphan_pins: bool,
    pub b_break_links: bool,
    pub b_remove_sub_pins: bool,
}
#[repr(C, align(8))]
pub struct FRigVMCollapseNodesAction {
    pub library_node_path: FString,
    pub collapsed_nodes_content: FString,
    pub collapsed_nodes_paths: TArray<FString>,
    pub b_is_aggregate: bool,
}
#[repr(C, align(8))]
pub struct FRigVMExpandNodeAction {
    pub library_node_path: FString,
    pub library_node_content: FString,
    pub expanded_node_paths: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FRigVMRenameNodeAction {
    pub old_node_name: FString,
    pub new_node_name: FString,
}
#[repr(C, align(8))]
pub struct FRigVMAddExposedPinAction {
    pub pin_name: FString,
    pub direction: crate::bindings::rig_vm::ERigVMPinDirection,
    pub cpp_type: FString,
    pub cpp_type_object_path: FString,
    pub default_value: FString,
}
#[repr(C, align(8))]
pub struct FRigVMRemoveExposedPinAction {
    pub pin_name: FString,
    pub direction: crate::bindings::rig_vm::ERigVMPinDirection,
    pub cpp_type: FString,
    pub cpp_type_object_path: FString,
    pub default_value: FString,
    pub pin_index: i32,
}
#[repr(C, align(8))]
pub struct FRigVMRenameExposedPinAction {
    pub old_pin_name: FString,
    pub new_pin_name: FString,
}
#[repr(C, align(8))]
pub struct FRigVMSetPinIndexAction {
    pub pin_path: FString,
    pub old_index: i32,
    pub new_index: i32,
}
#[repr(C, align(8))]
pub struct FRigVMSetRemappedVariableAction {
    pub node_path: FString,
    pub inner_variable_name: FName,
    pub old_outer_variable_name: FName,
    pub new_outer_variable_name: FName,
}
#[repr(C, align(8))]
pub struct FRigVMAddLocalVariableAction {
    pub local_variable: FRigVMGraphVariableDescription,
}
#[repr(C, align(8))]
pub struct FRigVMRemoveLocalVariableAction {
    pub local_variable: FRigVMGraphVariableDescription,
}
#[repr(C, align(8))]
pub struct FRigVMRenameLocalVariableAction {
    pub old_variable_name: FName,
    pub new_variable_name: FName,
}
#[repr(C, align(8))]
pub struct FRigVMChangeLocalVariableTypeAction {
    pub local_variable: FRigVMGraphVariableDescription,
    pub cpp_type: FString,
    pub cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FRigVMChangeLocalVariableDefaultValueAction {
    pub local_variable: FRigVMGraphVariableDescription,
    pub default_value: FString,
}
#[repr(C, align(8))]
pub struct FRigVMPromoteNodeAction {
    pub library_node_path: FString,
    pub function_definition_path: FString,
    pub b_from_function_to_collapse_node: bool,
}
#[repr(C, align(8))]
pub struct FRigVMMarkFunctionPublicAction {
    pub function_name: FName,
    pub b_is_public: bool,
}
#[repr(C, align(8))]
pub struct FRigVMCreateFunctionVariantAction {
    pub function_name: FName,
    pub new_function_name: FName,
}
#[repr(C, align(8))]
pub struct FRigVMAddFunctionVariantTagAction {
    pub function_name: FName,
    pub function_tag: crate::bindings::rig_vm::FRigVMTag,
}
#[repr(C, align(8))]
pub struct FRigVMRemoveFunctionVariantTagAction {
    pub function_name: FName,
    pub function_tag: crate::bindings::rig_vm::FRigVMTag,
}
#[repr(C, align(8))]
pub struct FRigVMImportFromTextAction {
    pub content: FString,
    pub top_level_node_names: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FRigVMReplaceNodesAction {}
#[repr(C, align(8))]
pub struct FRigVMAddTraitAction {
    pub node_name: FName,
    pub trait_name: FName,
    pub script_struct_path: FString,
    pub trait_default: FString,
    pub pin_index: i32,
}
#[repr(C, align(8))]
pub struct FRigVMRemoveTraitAction {}
#[repr(C, align(8))]
pub struct FRigVMExternalDependency {
    pub external_path: FString,
    pub internal_path: FString,
    pub category: FName,
}
#[repr(C, align(8))]
pub struct FRigVMGraphSectionLink {
    pub source_node_index: i32,
    pub target_node_index: i32,
    pub source_node_hash: u32,
    pub target_node_hash: u32,
    pub source_pin_path: FString,
    pub target_pin_path: FString,
}
#[repr(C, align(8))]
pub struct FRigVMGraphSection {
    pub hash: u32,
    pub nodes: TArray<FName>,
    pub node_hashes: TArray<u32>,
    pub leaf_nodes: TArray<i32>,
    pub links: TArray<FRigVMGraphSectionLink>,
}
#[repr(C, align(8))]
pub struct FRigVMTraitDefaultValueStruct {
    pub property_bag: crate::bindings::core_u_object::FInstancedPropertyBag,
}
#[repr(C, align(8))]
pub struct FRigStructScope {}
#[repr(C, align(8))]
pub struct FRigVMClientPatchResult {}
#[repr(C, align(8))]
pub struct FRigVMController_CommonTypePerTemplate {
    pub counts: TMap<FString, i32>,
}
pub struct URigVMEdGraph {
    pub model_node_path: FString,
    pub b_is_function_definition: bool,
}
pub struct URigVMEdGraphNode {
    pub model_node_path: FString,
    pub cached_model_node: TWeakObjectPtr<URigVMNode>,
    pub pin_path_to_model_pin: TMap<FString, TWeakObjectPtr<URigVMPin>>,
    pub property_name_deprecated: FName,
    pub struct_path_deprecated: FString,
    pub pin_type_deprecated: crate::bindings::engine::FEdGraphPinType,
    pub parameter_type_deprecated: i32,
    pub expanded_pins_deprecated: TArray<FString>,
}
pub struct URigVMEdGraphSchema {}
pub struct URigVMAssetInterface {}
pub struct IRigVMAssetInterface {}
pub struct URigVMBlueprint {
    pub function_library_ed_graph: UPtr<URigVMEdGraph>,
    pub rig_graph_display_settings: FRigVMEdGraphDisplaySettings,
    pub vm_runtime_settings: crate::bindings::rig_vm::FRigVMRuntimeSettings,
    pub vm_compile_settings: FRigVMCompileSettings,
    pub python_log_settings: FRigVMPythonSettings,
    pub user_defined_struct_guid_to_path_name: TMap<
        FString,
        crate::bindings::core_u_object::FSoftObjectPath,
    >,
    pub user_defined_enum_to_path_name: TMap<
        FString,
        crate::bindings::core_u_object::FSoftObjectPath,
    >,
    pub user_defined_types_in_use: TSet<UPtr<crate::bindings::core_u_object::UObject>>,
    pub model_deprecated: UPtr<URigVMGraph>,
    pub function_library_deprecated: UPtr<URigVMFunctionLibrary>,
    pub rig_vm_client: FRigVMClient,
    pub referenced_object_paths_stored: bool,
    pub referenced_object_paths: TArray<crate::bindings::core_u_object::FSoftObjectPath>,
    pub public_graph_functions: TArray<
        crate::bindings::rig_vm::FRigVMGraphFunctionHeader,
    >,
    pub function_reference_node_data: TArray<FRigVMReferenceNodeData>,
    pub asset_variant: crate::bindings::rig_vm::FRigVMVariant,
    pub supported_event_names: TArray<FName>,
    pub editor_host: UPtr<crate::bindings::rig_vm::URigVMHost>,
}
pub struct URigVMCompiler {
    pub settings_deprecated: FRigVMCompileSettings,
}
pub struct URigVMNode {
    pub node_title: FString,
    pub position: crate::bindings::core_u_object::FVector2D,
    pub size: crate::bindings::core_u_object::FVector2D,
    pub node_color: crate::bindings::core_u_object::FLinearColor,
    pub node_color_type: ERigVMNodeColorType,
    pub previous_name: FName,
    pub b_has_early_exit_marker: bool,
    pub b_is_excluded_by_early_exit: bool,
    pub trait_root_pin_names: TArray<FString>,
    pub trait_default_values: TMap<FString, FRigVMTraitDefaultValueStruct>,
    pub pins: TArray<UPtr<URigVMPin>>,
    pub orphaned_pins: TArray<UPtr<URigVMPin>>,
    pub pin_categories: TArray<FString>,
    pub pin_category_expansion: TMap<FString, bool>,
}
pub struct URigVMTemplateNode {
    pub template_notation: FName,
    pub resolved_function_name: FString,
    pub preferred_permutation_types_deprecated: TArray<FString>,
    pub preferred_permutation_pairs_deprecated: TArray<FRigVMTemplatePreferredType>,
}
pub struct URigVMLibraryNode {}
pub struct URigVMCollapseNode {
    pub contained_graph: UPtr<URigVMGraph>,
    pub node_category: FString,
    pub node_keywords: FString,
    pub node_description: FString,
}
pub struct URigVMAggregateNode {}
pub struct UDEPRECATED_RigVMArrayNode {
    pub op_code: crate::bindings::rig_vm::ERigVMOpCode,
}
pub struct UDEPRECATED_RigVMBranchNode {}
pub struct URigVMCommentNode {
    pub comment_text: FString,
    pub font_size: i32,
    pub b_bubble_visible: bool,
    pub b_color_bubble: bool,
}
pub struct URigVMDispatchNode {}
pub struct URigVMEnumNode {}
pub struct URigVMFunctionInterfaceNode {}
pub struct URigVMFunctionEntryNode {}
pub struct URigVMFunctionReferenceNode {
    pub referenced_function_header: crate::bindings::rig_vm::FRigVMGraphFunctionHeader,
    pub referenced_node_ptr_deprecated: TSoftObjectPtr<URigVMLibraryNode>,
    pub variable_map: TMap<FName, FName>,
}
pub struct URigVMFunctionReturnNode {}
pub struct UDEPRECATED_RigVMIfNode {}
pub struct URigVMInvokeEntryNode {}
pub struct URigVMParameterNode {}
pub struct URigVMRerouteNode {}
pub struct UDEPRECATED_RigVMSelectNode {}
pub struct URigVMUnitNode {
    pub script_struct_deprecated: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub method_name_deprecated: FName,
}
pub struct URigVMVariableNode {}
pub struct URigVMBuildData {
    pub function_references_deprecated: TMap<
        TSoftObjectPtr<URigVMLibraryNode>,
        FRigVMFunctionReferenceArray,
    >,
    pub graph_function_references: TMap<
        crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
        FRigVMFunctionReferenceArray,
    >,
}
pub struct URigVMClientHost {}
pub struct IRigVMClientHost {}
pub struct URigVMEditorSideObject {}
pub struct IRigVMEditorSideObject {}
pub struct URigVMClientExternalModelHost {}
pub struct IRigVMClientExternalModelHost {}
pub struct URigVMActionStack {
    pub action_index: i32,
    pub undo_actions: TArray<FRigVMActionKey>,
    pub redo_actions: TArray<FRigVMActionKey>,
}
pub struct URigVMExternalDependencyManager {}
pub struct IRigVMExternalDependencyManager {}
pub struct URigVMGraph {
    pub nodes: TArray<UPtr<URigVMNode>>,
    pub links: TArray<UPtr<URigVMLink>>,
    pub detached_links: TArray<UPtr<URigVMLink>>,
    pub selected_nodes: TArray<FName>,
    pub default_function_library_ptr: TWeakObjectPtr<URigVMGraph>,
    pub execute_context_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub last_structure_hash: u32,
    pub b_editable: bool,
    pub local_variables: TArray<FRigVMGraphVariableDescription>,
    pub schema_class: TSubclassOf<URigVMSchema>,
    pub b_sections_enabled: bool,
    pub selected_sections: TArray<FRigVMGraphSection>,
    pub sections_matching_the_selection: TArray<FRigVMGraphSection>,
}
pub struct URigVMFunctionLibrary {
    pub public_function_names: TArray<FName>,
    pub function_to_variant: TMap<FName, crate::bindings::rig_vm::FRigVMVariant>,
    pub function_references_deprecated: TMap<
        UPtr<URigVMLibraryNode>,
        FRigVMFunctionReferenceArray,
    >,
    pub localized_functions: TMap<FString, UPtr<URigVMLibraryNode>>,
}
pub struct URigVMLink {
    pub source_pin_path: FString,
    pub target_pin_path: FString,
}
pub struct URigVMInjectionInfo {
    pub unit_node_deprecated: UPtr<URigVMUnitNode>,
    pub node: UPtr<URigVMNode>,
    pub b_injected_as_input: bool,
    pub input_pin: UPtr<URigVMPin>,
    pub output_pin: UPtr<URigVMPin>,
}
pub struct URigVMPin {
    pub display_name: FName,
    pub direction: crate::bindings::rig_vm::ERigVMPinDirection,
    pub b_is_expanded: bool,
    pub b_is_constant: bool,
    pub b_requires_watch: bool,
    pub b_is_dynamic_array: bool,
    pub b_is_lazy: bool,
    pub cpp_type: FString,
    pub cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub cpp_type_object_path: FName,
    pub default_value: FString,
    pub default_value_type: ERigVMPinDefaultValueType,
    pub custom_widget_name: FName,
    pub sub_pins: TArray<UPtr<URigVMPin>>,
    pub links: TArray<UPtr<URigVMLink>>,
    pub injection_infos: TArray<UPtr<URigVMInjectionInfo>>,
    pub user_defined_category: FString,
    pub index_in_category: i32,
    pub bound_variable_path_deprecated: FString,
}
pub struct URigVMSchema {
    pub execute_context_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
}
pub struct URigVMUserWorkflowRegistry {}
pub struct URigVMController {
    pub modified_event_dynamic: FRigVMController_ModifiedEventDynamic,
    pub graphs: TArray<UPtr<URigVMGraph>>,
    pub schema_class: TSubclassOf<URigVMSchema>,
}
pub struct URigVMControllerSettings {
    pub b_auto_resolve_template_nodes_when_linking_execute: bool,
    pub template_default_types: TMap<FName, FRigVMController_CommonTypePerTemplate>,
}
pub struct FRegisterProvider_InProvider;
pub struct FRigVMController_ModifiedEventDynamic;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigVMTagDisplayMode(pub u8);
impl ERigVMTagDisplayMode {
    pub const NONE: ERigVMTagDisplayMode = ERigVMTagDisplayMode(0);
    pub const ALL: ERigVMTagDisplayMode = ERigVMTagDisplayMode(1);
    pub const DEPRECATION_ONLY: ERigVMTagDisplayMode = ERigVMTagDisplayMode(2);
    pub const LAST: ERigVMTagDisplayMode = ERigVMTagDisplayMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigVMPinDefaultValueType(pub u8);
impl ERigVMPinDefaultValueType {
    pub const AUTO_DETECT: ERigVMPinDefaultValueType = ERigVMPinDefaultValueType(0);
    pub const UNSET: ERigVMPinDefaultValueType = ERigVMPinDefaultValueType(1);
    pub const OVERRIDE: ERigVMPinDefaultValueType = ERigVMPinDefaultValueType(2);
    pub const KEEP_VALUE_TYPE: ERigVMPinDefaultValueType = ERigVMPinDefaultValueType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigVMNodeColorType(pub u8);
impl ERigVMNodeColorType {
    pub const FROM_METADATA: ERigVMNodeColorType = ERigVMNodeColorType(0);
    pub const USER_DEFINED: ERigVMNodeColorType = ERigVMNodeColorType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigVMGraphNotifType(pub u8);
impl ERigVMGraphNotifType {
    pub const GRAPH_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(0);
    pub const NODE_ADDED: ERigVMGraphNotifType = ERigVMGraphNotifType(1);
    pub const NODE_REMOVED: ERigVMGraphNotifType = ERigVMGraphNotifType(2);
    pub const NODE_SELECTED: ERigVMGraphNotifType = ERigVMGraphNotifType(3);
    pub const NODE_DESELECTED: ERigVMGraphNotifType = ERigVMGraphNotifType(4);
    pub const NODE_SELECTION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(5);
    pub const NODE_POSITION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(6);
    pub const NODE_SIZE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(7);
    pub const NODE_TITLE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(8);
    pub const NODE_COLOR_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(9);
    pub const PIN_ADDED: ERigVMGraphNotifType = ERigVMGraphNotifType(10);
    pub const PIN_REMOVED: ERigVMGraphNotifType = ERigVMGraphNotifType(11);
    pub const PIN_RENAMED: ERigVMGraphNotifType = ERigVMGraphNotifType(12);
    pub const PIN_EXPANSION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(13);
    pub const PIN_WATCHED_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(14);
    pub const PIN_ARRAY_SIZE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(15);
    pub const PIN_DEFAULT_VALUE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(16);
    pub const PIN_DIRECTION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(17);
    pub const PIN_TYPE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(18);
    pub const PIN_INDEX_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(19);
    pub const LINK_ADDED: ERigVMGraphNotifType = ERigVMGraphNotifType(20);
    pub const LINK_REMOVED: ERigVMGraphNotifType = ERigVMGraphNotifType(21);
    pub const COMMENT_TEXT_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(22);
    pub const VARIABLE_ADDED: ERigVMGraphNotifType = ERigVMGraphNotifType(23);
    pub const VARIABLE_REMOVED: ERigVMGraphNotifType = ERigVMGraphNotifType(24);
    pub const VARIABLE_RENAMED: ERigVMGraphNotifType = ERigVMGraphNotifType(25);
    pub const INTERACTION_BRACKET_OPENED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        26,
    );
    pub const INTERACTION_BRACKET_CLOSED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        27,
    );
    pub const INTERACTION_BRACKET_CANCELED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        28,
    );
    pub const PIN_BOUND_VARIABLE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        29,
    );
    pub const NODE_RENAMED: ERigVMGraphNotifType = ERigVMGraphNotifType(30);
    pub const FUNCTION_RENAMED: ERigVMGraphNotifType = ERigVMGraphNotifType(31);
    pub const NODE_REFERENCE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(32);
    pub const NODE_CATEGORY_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(33);
    pub const NODE_KEYWORDS_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(34);
    pub const NODE_DESCRIPTION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(35);
    pub const VARIABLE_REMAPPING_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        36,
    );
    pub const LIBRARY_TEMPLATE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(37);
    pub const FUNCTION_ACCESS_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(38);
    pub const VARIANT_TAGS_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(39);
    pub const PIN_DISPLAY_NAME_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(40);
    pub const PIN_CATEGORY_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(41);
    pub const PIN_CATEGORIES_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(42);
    pub const PIN_CATEGORY_EXPANSION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        43,
    );
    pub const FUNCTION_VARIANT_GUID_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        44,
    );
    pub const NODE_EARLY_EXIT_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(45);
    pub const LOCAL_VARIABLE_DEFAULT_VALUE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        46,
    );
    pub const LOCAL_VARIABLE_ADDED: ERigVMGraphNotifType = ERigVMGraphNotifType(47);
    pub const LOCAL_VARIABLE_REMOVED: ERigVMGraphNotifType = ERigVMGraphNotifType(48);
    pub const LOCAL_VARIABLE_RENAMED: ERigVMGraphNotifType = ERigVMGraphNotifType(49);
    pub const LOCAL_VARIABLE_TYPE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        50,
    );
    pub const INVALID: ERigVMGraphNotifType = ERigVMGraphNotifType(51);
}
