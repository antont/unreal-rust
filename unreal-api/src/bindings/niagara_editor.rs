#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FNiagaraStackNoteData {
    pub message_header: FText,
    pub color: FLinearColor,
    pub message: FText,
    pub b_inline_note: bool,
}
#[repr(C, align(8))]
pub struct FNiagaraSpawnShortcut {
    pub name: FString,
    pub input: FInputChord,
}
#[repr(C, align(8))]
pub struct FNiagaraDistributionArray {
    pub float_data: TArray<f32>,
    pub vector2_data: TArray<FVector2f>,
    pub vector3_data: TArray<FVector3f>,
    pub vector4_data: TArray<FVector4f>,
}
#[repr(C, align(8))]
pub struct FNiagaraActionIdentifier {
    pub names: TArray<FName>,
    pub guids: TArray<FGuid>,
}
#[repr(C, align(8))]
pub struct FNiagaraFavoriteActionsProfile {
    pub favorite_actions: TSet<FNiagaraActionIdentifier>,
    pub unfavorite_actions: TSet<FNiagaraActionIdentifier>,
}
#[repr(C, align(8))]
pub struct FNiagaraBakerTextureSourceAction {}
#[repr(C, align(1))]
pub struct FNiagaraEnumToByteHelper {
    pub value: u8,
}
#[repr(C, align(8))]
pub struct FNiagaraStackAssetAction_EventSource {}
#[repr(C, align(8))]
pub struct FNiagaraOutlinerWorldDataCustomizationWrapper {
    pub data: FNiagaraOutlinerWorldData,
}
#[repr(C, align(8))]
pub struct FNiagaraOutlinerSystemDataCustomizationWrapper {
    pub data: FNiagaraOutlinerSystemData,
}
#[repr(C, align(8))]
pub struct FNiagaraOutlinerSystemInstanceDataCustomizationWrapper {
    pub data: FNiagaraOutlinerSystemInstanceData,
}
#[repr(C, align(8))]
pub struct FNiagaraOutlinerEmitterInstanceDataCustomizationWrapper {
    pub data: FNiagaraOutlinerEmitterInstanceData,
}
#[repr(C, align(8))]
pub struct FNiagaraStackAssetAction_VarBind {}
#[repr(C, align(8))]
pub struct FNiagaraSchemaAction_NewNode {
    pub node_template: UPtr<UEdGraphNode>,
    pub internal_name: FName,
}
#[repr(C, align(8))]
pub struct FNiagaraSchemaAction_NewComment {}
#[repr(C, align(8))]
pub struct FNiagaraActionSourceData {}
#[repr(C, align(8))]
pub struct FNiagaraMenuAction {}
#[repr(C, align(8))]
pub struct FNiagaraMenuAction_Base {}
#[repr(C, align(8))]
pub struct FNiagaraMenuAction_Generic {}
#[repr(C, align(8))]
pub struct FNiagaraAction_NewNode {
    pub weak_node_template: TWeakObjectPtr<UEdGraphNode>,
}
#[repr(C, align(8))]
pub struct FNiagaraClipboardScriptVariable {
    pub script_variable: UPtr<UNiagaraScriptVariable>,
    pub original_change_id: FGuid,
}
#[repr(C, align(8))]
pub struct FNiagaraClipboardCurveCollection {
    pub curves: TArray<FRichCurve>,
}
#[repr(C, align(8))]
pub struct FNiagaraClipboardPortableValue {
    pub value_string: FString,
}
#[repr(C, align(4))]
pub struct FFunctionInputSummaryViewKey {
    pub function_guid: FGuid,
    pub input_guid: FGuid,
    pub input_name: FName,
}
#[repr(C, align(4))]
pub struct FFunctionInputSummaryViewMetadata {
    pub b_visible: bool,
    pub display_name: FName,
    pub category: FName,
    pub sort_index: i32,
}
#[repr(C, align(8))]
pub struct FReservedParameter {
    pub parameter: FNiagaraVariableBase,
    pub reserving_definitions_asset: UPtr<UNiagaraParameterDefinitions>,
}
#[repr(C, align(8))]
pub struct FNiagaraRendererCreationInfo {
    pub display_name: FText,
    pub description: FText,
    pub renderer_class_path: FTopLevelAssetPath,
}
#[repr(C, align(8))]
pub struct FNiagaraNamespaceMetadata {
    pub namespaces: TArray<FName>,
    pub required_namespace_modifier: FName,
    pub display_name: FText,
    pub display_name_long: FText,
    pub description: FText,
    pub background_color: FLinearColor,
    pub foreground_style: FName,
    pub sort_id: i32,
    pub optional_namespace_modifiers: TArray<FName>,
    pub options: TArray<ENiagaraNamespaceMetadataOptions>,
    pub guid: FGuid,
}
#[repr(C, align(8))]
pub struct FNiagaraCurveTemplate {
    pub display_name_override: FString,
    pub curve_asset: FSoftObjectPath,
}
#[repr(C, align(4))]
pub struct FNiagaraActionColors {
    pub niagara_color: FLinearColor,
    pub game_color: FLinearColor,
    pub plugin_color: FLinearColor,
    pub developer_color: FLinearColor,
}
#[repr(C, align(8))]
pub struct FNiagaraParameterPanelSectionStorage {
    pub param_storage_id: FGuid,
    pub expanded_categories: TArray<FGuid>,
}
#[repr(C, align(8))]
pub struct FNiagaraViewportSharedSettings {
    pub viewport_type: ELevelViewportType,
    pub persp_view_mode_index: EViewModeIndex,
    pub ortho_view_mode_index: EViewModeIndex,
    pub editor_show_flags_string: FString,
    pub game_show_flags_string: FString,
    pub exposure_settings: FExposureSettings,
    pub fov_angle: f32,
    pub b_is_realtime: bool,
    pub b_show_on_screen_stats: bool,
    pub b_show_grid_in_viewport: bool,
    pub b_show_instructions_count: bool,
    pub b_show_particle_counts_in_viewport: bool,
    pub b_show_emitter_execution_order: bool,
    pub b_show_gpu_tick_information: bool,
    pub b_show_memory_info: bool,
    pub b_show_stateless_info: bool,
}
#[repr(C, align(1))]
pub struct FNiagaraCurveEditorSharedSettings {
    pub b_curve_input_snap_enabled: bool,
    pub b_curve_output_snap_enabled: bool,
}
#[repr(C, align(4))]
pub struct FNiagaraGraphParameterReference {
    pub key: FGuid,
    pub value: TWeakObjectPtr<UObject>,
    pub b_is_user_facing: bool,
}
#[repr(C, align(8))]
pub struct FNiagaraGraphParameterReferenceCollection {
    pub parameter_references: TArray<FNiagaraGraphParameterReference>,
    pub b_created_by_user: bool,
}
#[repr(C, align(8))]
pub struct FNiagaraGraphScriptUsageInfo {
    pub base_id: FGuid,
    pub usage_type: ENiagaraScriptUsage,
    pub usage_id: FGuid,
    pub compile_hash: FNiagaraCompileHash,
    pub compile_hash_from_graph: FNiagaraCompileHash,
    pub reference_hash_from_graph: FNiagaraCompileHash,
    pub compile_last_objects: TArray<FNiagaraCompileHashVisitorDebugInfo>,
    pub traversal: TArray<UPtr<UNiagaraNode>>,
    pub data_hash_deprecated: TArray<u8>,
    pub generated_compile_id_deprecated: FGuid,
}
#[repr(C, align(1))]
pub struct FNiagaraParametersChangedData {}
#[repr(C, align(4))]
pub struct FNiagaraParameterCreatedData {}
#[repr(C, align(4))]
pub struct FNiagaraParameterRenamedData {}
#[repr(C, align(8))]
pub struct FNiagaraStackMessage {
    pub message_text: FText,
    pub short_description: FText,
    pub message_severity: ENiagaraMessageSeverity,
    pub b_allow_dismissal: bool,
    pub guid: FGuid,
}
#[repr(C, align(8))]
pub struct FNiagaraConvertPinRecord {
    pub pin_id: FGuid,
    pub path: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FNiagaraConvertConnection {
    pub source_pin_id: FGuid,
    pub source_path: TArray<FName>,
    pub destination_pin_id: FGuid,
    pub destination_path: TArray<FName>,
    pub source_property_id: FGuid,
    pub destination_property_id: FGuid,
}
#[repr(C, align(8))]
pub struct FNiagaraPropagatedVariable {
    pub switch_parameter: FNiagaraVariable,
    pub propagated_name: FString,
}
#[repr(C, align(4))]
pub struct FPinGuidsForPath {
    pub output_pin_guid: FGuid,
    pub input_true_pin_guid: FGuid,
    pub input_false_pin_guid: FGuid,
}
#[repr(C, align(4))]
pub struct FNiagaraInputExposureOptions {
    pub flags_0: u8,
}
#[repr(C, align(8))]
pub struct FAddedPinData {
    pub pin_type: FEdGraphPinType,
    pub pin_name: FName,
}
#[repr(C, align(8))]
pub struct FStaticSwitchTypeData {
    pub switch_type: ENiagaraStaticSwitchType,
    pub enum_: UPtr<UEnum>,
    pub switch_constant: FName,
    pub b_auto_refresh_enabled: bool,
    pub b_expose_as_pin: bool,
}
#[repr(C, align(4))]
pub struct FNiagaraOutlinerFilterSettings {
    pub flags_0: u8,
    pub system_execution_state: ENiagaraExecutionState,
    pub emitter_execution_state: ENiagaraExecutionState,
    pub emitter_sim_target: ENiagaraSimTarget,
    pub b_system_cull_state: bool,
}
#[repr(C, align(4))]
pub struct FNiagaraOutlinerViewSettings {
    pub view_mode: ENiagaraOutlinerViewModes,
    pub filter_settings: FNiagaraOutlinerFilterSettings,
    pub b_sort_descending: bool,
    pub sort_mode: ENiagaraOutlinerSortMode,
    pub time_units: ENiagaraOutlinerTimeUnits,
}
#[repr(C, align(8))]
pub struct FScriptVarBindingNameSubscription {
    pub external_script_var_id: FGuid,
    pub internal_script_var_ids: TArray<FGuid>,
}
#[repr(C, align(8))]
pub struct FParameterDefinitionsBindingNameSubscription {
    pub subscribed_parameter_definitions: UPtr<UNiagaraParameterDefinitions>,
    pub binding_name_subscriptions: TArray<FScriptVarBindingNameSubscription>,
}
#[repr(C, align(8))]
pub struct FNiagaraScriptVariableData {
    pub default_mode: ENiagaraDefaultMode,
    pub default_binding: FNiagaraScriptVariableBinding,
    pub variable: FNiagaraVariable,
    pub metadata: FNiagaraVariableMetaData,
    pub default_value_variant: FNiagaraVariant,
    pub static_switch_default_value: i32,
    pub b_is_static_switch: bool,
    pub b_subscribed_to_parameter_definitions: bool,
    pub change_id: FGuid,
    pub b_override_parameter_definitions_default_value: bool,
}
#[repr(C, align(1))]
pub struct FNiagaraStatelessModuleEditorData {
    pub b_show_when_disabled: bool,
}
#[repr(C, align(8))]
pub struct FNiagaraPerAssetViewportSettings {
    pub view_location: FVector,
    pub view_rotation: FRotator,
    pub b_use_orbit_mode: bool,
}
#[repr(C, align(8))]
pub struct FNiagaraValidationRule_EmitterCountAndPlatformSet {
    pub rule_name: FString,
    pub platforms: FNiagaraPlatformSet,
    pub b_include_stateful: bool,
    pub b_include_stateless: bool,
    pub emitter_count_limit: i32,
}
#[repr(C, align(8))]
pub struct FNiagaraValidationRule_RendererCountAndPlatformSet {
    pub rule_name: FString,
    pub platforms: FNiagaraPlatformSet,
    pub renderer_count_limit: i32,
}
#[repr(C, align(8))]
pub struct FNiagaraEmitterSectionKey {
    pub module_id: FGuid,
    pub value: FNiagaraVariable,
}
#[repr(C, align(8))]
pub struct FMovieSceneNiagaraEmitterChannel {}
#[repr(C, align(8))]
pub struct FNiagaraDebuggerSpawnData {
    pub systems_to_spawn: TArray<TSoftObjectPtr<UNiagaraSystem>>,
    pub b_spawn_all_at_once: bool,
    pub time_between_spawns: f32,
    pub b_kill_before_spawn: bool,
    pub b_world_location: bool,
    pub location: FVector,
    pub b_attach_to_player: bool,
    pub b_auto_activate: bool,
    pub b_auto_destroy: bool,
    pub b_do_pre_cull_check: bool,
}
pub struct UActorFactoryNiagara {}
pub struct UNiagaraScriptFactoryNew {}
pub struct UNiagaraModuleScriptFactory {}
pub struct UNiagaraFunctionScriptFactory {}
pub struct UNiagaraDynamicInputScriptFactory {}
pub struct UAssetDefinition_NiagaraDataChannel {}
pub struct UAssetDefinition_NiagaraEffectType {}
pub struct UAssetDefinition_NiagaraEmitter {}
pub struct UAssetDefinition_NiagaraParameterCollection {}
pub struct UAssetDefinition_NiagaraParameterCollectionInstance {}
pub struct UAssetDefinition_NiagaraParameterDefinitions {}
pub struct UAssetDefinition_NiagaraScript {}
pub struct UAssetDefinition_NiagaraSimCache {}
pub struct UAssetDefinition_NiagaraStatelessEmitterTemplate {}
pub struct UAssetDefinition_NiagaraSystem {}
pub struct UAssetDefinition_NiagaraValidationRuleSet {}
pub struct UNiagaraDumpByteCodeCommandlet {}
pub struct UNiagaraDumpModuleInfoCommandlet {}
pub struct UNiagaraScriptValidationCommandlet {}
pub struct UNiagaraStatelessAuditCommandlet {}
pub struct UNiagaraSystemAuditCommandlet {
    pub packages_to_save: TArray<UPtr<UPackage>>,
}
pub struct UNiagaraTraversalCacheAuditCommandlet {}
pub struct UNiagaraFavoriteActionsConfig {
    pub profiles: TMap<FName, FNiagaraFavoriteActionsProfile>,
}
pub struct UNiagaraObjectAssetHelper {
    pub path: FSoftObjectPath,
}
pub struct UFNiagaraMemoryBufferSimCacheVisualizerSettings {
    pub display_sim_target: ENiagaraSimTarget,
    pub display_as_type: ENDIMemoryBufferViewType,
    pub display_columns: i32,
}
pub struct UEdGraphSchema_Niagara {}
pub struct UEdGraphSchema_NiagaraSystemOverview {}
pub struct UNiagaraBakerFunctionLibrary {}
pub struct UNiagaraBakerStaticMeshFactoryNew {}
pub struct UNiagaraClipboardFunctionInput {
    pub input_name: FName,
    pub input_type: FNiagaraTypeDefinition,
    pub b_has_edit_condition: bool,
    pub b_edit_condition_value: bool,
    pub value_mode: ENiagaraClipboardFunctionInputValueMode,
    pub local: TArray<u8>,
    pub linked: FNiagaraVariableBase,
    pub data: UPtr<UNiagaraDataInterface>,
    pub object_asset: UPtr<UObject>,
    pub expression: FString,
    pub dynamic: UPtr<UNiagaraClipboardFunction>,
    pub children_inputs: TArray<UPtr<UNiagaraClipboardFunctionInput>>,
}
pub struct UNiagaraClipboardRenderer {
    pub renderer_properties: UPtr<UNiagaraRendererProperties>,
    pub stack_note_data: FNiagaraStackNoteData,
}
pub struct UNiagaraClipboardFunction {
    pub function_name: FString,
    pub display_name: FText,
    pub script_mode: ENiagaraClipboardFunctionScriptMode,
    pub script: TSoftObjectPtr<UNiagaraScript>,
    pub assignment_targets: TArray<FNiagaraVariable>,
    pub assignment_defaults: TArray<FString>,
    pub inputs: TArray<UPtr<UNiagaraClipboardFunctionInput>>,
    pub on_pasted_function_call_node_delegate: FNiagaraClipboardFunction_OnPastedFunctionCallNodeDelegate,
    pub script_version: FGuid,
    pub stack_note_data: FNiagaraStackNoteData,
}
pub struct UNiagaraClipboardContent {
    pub functions: TArray<UPtr<UNiagaraClipboardFunction>>,
    pub function_inputs: TArray<UPtr<UNiagaraClipboardFunctionInput>>,
    pub renderers: TArray<UPtr<UNiagaraClipboardRenderer>>,
    pub scripts: TArray<UPtr<UNiagaraScript>>,
    pub script_variables: TArray<FNiagaraClipboardScriptVariable>,
    pub stateless_modules: TArray<UPtr<UObject>>,
    pub exported_nodes: FString,
    pub b_fixup_paste_index_for_script_dependencies_in_stack: bool,
    pub stack_note: FNiagaraStackNoteData,
    pub portable_values: TArray<FNiagaraClipboardPortableValue>,
}
pub struct UNiagaraClipboardEditorScriptingUtilities {}
pub struct UNiagaraConvertInPlaceEmitterAndSystemState {}
pub struct UNiagaraDataChannelAssetFactoryNew {}
pub struct UNiagaraEditorParametersAdapter {
    pub editor_only_script_vars: TArray<UPtr<UNiagaraScriptVariable>>,
}
pub struct UNiagaraEditorSettings {
    pub default_script: FSoftObjectPath,
    pub default_dynamic_input_script: FSoftObjectPath,
    pub default_function_script: FSoftObjectPath,
    pub default_module_script: FSoftObjectPath,
    pub default_empty_emitter: FSoftObjectPath,
    pub emitter_asset_wizard_configuration: FSoftObjectPath,
    pub system_asset_wizard_configuration: FSoftObjectPath,
    pub default_decal_renderer_material: FSoftObjectPath,
    pub default_mesh_renderer_mesh: FSoftObjectPath,
    pub default_ribbon_renderer_material: FSoftObjectPath,
    pub default_sprite_renderer_material: FSoftObjectPath,
    pub b_display_preview_movies_in_tooltips: bool,
    pub tooltip_preview_movie_size: FVector2f,
    pub required_system_update_script: FSoftObjectPath,
    pub default_validation_rule_sets: TArray<TSoftObjectPtr<UNiagaraValidationRuleSet>>,
    pub graph_creation_shortcuts: TArray<FNiagaraSpawnShortcut>,
    pub b_simplify_stack_nodes_at_low_resolution: bool,
    pub low_resolution_node_max_name_chars: i32,
    pub b_always_zoom_to_fit_system_graph: bool,
    pub renderer_category_expand_state: ENiagaraCategoryExpandState,
    pub defaults_sequencer_subtracks: ENiagaraAddDefaultsTrackMode,
    pub script_graph_zoom_limit_handling: EGraphZoomLimitHandling,
    pub b_expand_module_notes_by_default: bool,
    pub b_auto_compile: bool,
    pub b_auto_play: bool,
    pub b_reset_simulation_on_change: bool,
    pub b_resimulate_on_change_while_paused: bool,
    pub b_reset_dependent_systems_when_editing_emitters: bool,
    pub b_display_advanced_parameter_panel_categories: bool,
    pub b_display_affected_asset_stats: bool,
    pub affected_asset_search_limit: i32,
    pub b_update_stack_values_on_commit_only: bool,
    pub playback_speeds: TArray<f32>,
    pub action_colors: FNiagaraActionColors,
    pub default_note_color: FLinearColor,
    pub hlsl_keyword_replacements: TMap<FString, FString>,
    pub namespace_metadata: TArray<FNiagaraNamespaceMetadata>,
    pub namespace_modifier_metadata: TArray<FNiagaraNamespaceMetadata>,
    pub default_namespace_metadata: FNiagaraNamespaceMetadata,
    pub default_namespace_modifier_metadata: FNiagaraNamespaceMetadata,
    pub curve_templates: TArray<FNiagaraCurveTemplate>,
    pub curve_editor_shared_settings: FNiagaraCurveEditorSharedSettings,
    pub viewport_settings: FNiagaraViewportSharedSettings,
    pub system_parameter_panel_section_data: TArray<
        FNiagaraParameterPanelSectionStorage,
    >,
    pub b_force_silent_loading_of_cached_assets: bool,
}
pub struct UNiagaraEffectTypeFactoryNew {}
pub struct UNiagaraEmitterEditorData {
    pub stack_editor_data: UPtr<UNiagaraStackEditorData>,
    pub playback_range_min: f32,
    pub playback_range_max: f32,
    pub flags_96: u8,
    pub summary_view_function_input_metadata_deprecated: TMap<
        FFunctionInputSummaryViewKey,
        FFunctionInputSummaryViewMetadata,
    >,
    pub summary_sections_deprecated: TArray<FNiagaraStackSection>,
    pub summary_view_root: UPtr<UHierarchyRoot>,
    pub emitter_thumbnail: UPtr<UTexture2D>,
}
pub struct UNiagaraEmitterFactoryNew {}
pub struct UNiagaraGraph {
    pub change_id: FGuid,
    pub force_rebuild_id: FGuid,
    pub last_built_traversal_data_change_id: FGuid,
    pub last_built_script_version_id: FGuid,
    pub cached_usage_info: TArray<FNiagaraGraphScriptUsageInfo>,
    pub variable_to_meta_data_deprecated: TMap<
        FNiagaraVariable,
        FNiagaraVariableMetaData,
    >,
    pub variable_to_script_variable: TMap<
        FNiagaraVariable,
        UPtr<UNiagaraScriptVariable>,
    >,
    pub parameter_hierarchy_root: UPtr<UHierarchyRoot>,
    pub parameter_to_references_map: TMap<
        FNiagaraVariable,
        FNiagaraGraphParameterReferenceCollection,
    >,
    pub compilation_script_variables: TArray<FNiagaraScriptVariableData>,
    pub b_has_valid_last_built_script_version_id: bool,
}
pub struct UNiagaraMessageData {}
pub struct UNiagaraMessageDataText {
    pub message_text: FText,
    pub short_description: FText,
    pub message_severity: ENiagaraMessageSeverity,
    pub b_allow_dismissal: bool,
    pub topic_name: FName,
}
pub struct UNiagaraNode {
    pub change_id: FGuid,
}
pub struct UNiagaraNodeWithDynamicPins {}
pub struct UNiagaraNodeFunctionCall {
    pub function_script: UPtr<UNiagaraScript>,
    pub selected_script_version: FGuid,
    pub function_script_asset_object_path: FName,
    pub signature: FNiagaraFunctionSignature,
    pub function_specifiers: TMap<FName, FName>,
    pub propagated_static_switch_parameters: TArray<FNiagaraPropagatedVariable>,
    pub previous_script_version: FGuid,
    pub python_upgrade_script_warnings: FString,
    pub debug_state: ENiagaraFunctionDebugState,
    pub b_inherit_debug_status: bool,
    pub cached_change_id: FGuid,
    pub invalid_script_version_reference: FGuid,
    pub function_display_name: FString,
    pub message_key_to_message_map_deprecated: TMap<FGuid, UPtr<UNiagaraMessageData>>,
    pub message_store: FNiagaraMessageStore,
    pub stack_messages: TArray<FNiagaraStackMessage>,
    pub bound_pin_names: TMap<FGuid, FName>,
}
pub struct UNiagaraNodeAssignment {
    pub assignment_target_deprecated: FNiagaraVariable,
    pub assignment_default_value_deprecated: FString,
    pub assignment_targets: TArray<FNiagaraVariable>,
    pub assignment_default_values: TArray<FString>,
    pub old_function_call_name: FString,
}
pub struct UNiagaraNodeConvert {
    pub autowire_swizzle: FString,
    pub autowire_make_type: FNiagaraTypeDefinition,
    pub autowire_break_type: FNiagaraTypeDefinition,
    pub connections: TArray<FNiagaraConvertConnection>,
    pub b_is_wiring_shown: bool,
    pub expanded_items: TArray<FNiagaraConvertPinRecord>,
}
pub struct UNiagaraNodeCustomHlsl {
    pub script_usage: ENiagaraScriptUsage,
    pub custom_hlsl: FString,
    pub absolute_include_file_paths: TArray<FFilePath>,
    pub virtual_include_file_paths: TArray<FString>,
    pub b_is_shader_code_shown: bool,
}
pub struct UNiagaraNodeDataSetBase {
    pub data_set: FNiagaraDataSetID,
    pub variables: TArray<FNiagaraVariable>,
    pub variable_friendly_names: TArray<FString>,
    pub external_struct_asset: UPtr<UStruct>,
}
pub struct UNiagaraNodeEmitter {
    pub owner_system: UPtr<UNiagaraSystem>,
    pub emitter_handle_id: FGuid,
    pub display_name: FText,
    pub script_type: ENiagaraScriptUsage,
}
pub struct UNiagaraNodeIf {
    pub output_vars: TArray<FNiagaraVariable>,
    pub path_associated_pin_guids: TArray<FPinGuidsForPath>,
    pub condition_pin_guid: FGuid,
}
pub struct UNiagaraNodeInput {
    pub input: FNiagaraVariable,
    pub usage: ENiagaraInputNodeUsage,
    pub call_sort_priority: i32,
    pub exposure_options: FNiagaraInputExposureOptions,
    pub data_interface: UPtr<UNiagaraDataInterface>,
    pub object_asset: UPtr<UObject>,
}
pub struct UNiagaraNodeOp {
    pub op_name: FName,
    pub added_pins: TArray<FAddedPinData>,
    pub b_all_static: bool,
}
pub struct UNiagaraNodeOutput {
    pub outputs: TArray<FNiagaraVariable>,
    pub script_type: ENiagaraScriptUsage,
    pub script_type_id: FGuid,
    pub script_type_index_deprecated: i32,
}
pub struct UNiagaraNodeOutputTag {
    pub b_editor_only: bool,
    pub b_emit_message_on_failure: bool,
    pub failure_severity: FNiagaraCompileEventSeverity,
}
pub struct UNiagaraNodeParameterMapBase {}
pub struct UNiagaraNodeParameterMapSet {}
pub struct UNiagaraNodeParameterMapFor {}
pub struct UNiagaraNodeParameterMapForWithContinue {}
pub struct UNiagaraNodeParameterMapForIndex {}
pub struct UNiagaraNodeParameterMapGet {
    pub pin_output_to_pin_default_persistent_id: TMap<FGuid, FGuid>,
}
pub struct UNiagaraNodeReadDataSet {}
pub struct UNiagaraNodeReroute {}
pub struct UNiagaraNodeUsageSelector {
    pub output_vars: TArray<FNiagaraVariable>,
    pub output_var_guids: TArray<FGuid>,
    pub selector_guid: FGuid,
    pub num_options_per_variable: i32,
}
pub struct UNiagaraNodeSelect {
    pub selector_pin_type: FNiagaraTypeDefinition,
    pub selector_pin_guid: FGuid,
}
pub struct UNiagaraNodeSimTargetSelector {}
pub struct UNiagaraNodeStaticSwitch {
    pub input_parameter_name: FName,
    pub switch_type_data: FStaticSwitchTypeData,
}
pub struct UNiagaraNodeWriteDataSet {
    pub event_name: FName,
}
pub struct UNiagaraOutliner {
    pub capture_settings: FNiagaraOutlinerCaptureSettings,
    pub view_settings: FNiagaraOutlinerViewSettings,
    pub data: FNiagaraOutlinerData,
    pub system_sim_caches: TMap<FName, UPtr<UNiagaraSimCache>>,
}
pub struct UNiagaraOverviewNode {
    pub owning_system: UPtr<UNiagaraSystem>,
    pub emitter_handle_guid: FGuid,
}
pub struct UNiagaraParameterCollectionFactoryNew {}
pub struct UNiagaraParameterCollectionInstanceFactoryNew {}
pub struct UNiagaraParameterDefinitions {
    pub b_promote_to_top_in_add_menus: bool,
    pub menu_sort_order: i32,
    pub script_variables: TArray<UPtr<UNiagaraScriptVariable>>,
    pub external_parameter_definitions_subscriptions: TArray<
        FParameterDefinitionsBindingNameSubscription,
    >,
}
pub struct UNiagaraParameterDefinitionsFactory {}
pub struct UNiagaraScriptSource {
    pub node_graph: UPtr<UNiagaraGraph>,
}
pub struct UNiagaraScriptVariable {
    pub default_mode: ENiagaraDefaultMode,
    pub default_binding: FNiagaraScriptVariableBinding,
    pub variable: FNiagaraVariable,
    pub metadata: FNiagaraVariableMetaData,
    pub default_value_variant: FNiagaraVariant,
    pub static_switch_default_value: i32,
    pub b_is_static_switch: bool,
    pub b_subscribed_to_parameter_definitions: bool,
    pub change_id: FGuid,
    pub b_override_parameter_definitions_default_value: bool,
}
pub struct UNiagaraSimCacheFactoryNew {}
pub struct UNiagaraStackEditorData {
    pub b_hide_disabled_modules: bool,
    pub stack_entry_key_to_expanded_map: TMap<FString, bool>,
    pub stack_entry_key_to_inline_display_mode_map: TMap<
        FString,
        ENiagaraStackEntryInlineDisplayMode,
    >,
    pub stack_entry_key_to_expanded_overview_map: TMap<FString, bool>,
    pub stack_notes: TMap<FString, FNiagaraStackNoteData>,
    pub stack_entry_key_to_stateless_module_editor_data: TMap<
        FString,
        FNiagaraStatelessModuleEditorData,
    >,
    pub stack_entry_key_to_display_name: TMap<FString, FText>,
    pub dismissed_stack_issue_ids: TArray<FString>,
}
pub struct UNiagaraStatelessEmitterTemplateFactoryNew {}
pub struct UNiagaraSystemEditorFolder {
    pub folder_name: FName,
    pub child_folders: TArray<UPtr<UNiagaraSystemEditorFolder>>,
    pub child_emitter_handle_ids: TArray<FGuid>,
}
pub struct UNiagaraSystemEditorData {
    pub user_parameter_hierarchy: UPtr<UHierarchyRoot>,
    pub root_folder: UPtr<UNiagaraSystemEditorFolder>,
    pub stack_editor_data: UPtr<UNiagaraStackEditorData>,
    pub owner_transform: FTransform,
    pub playback_range_min: f32,
    pub playback_range_max: f32,
    pub playback_frame_rate: FFrameRate,
    pub b_lock_playback_frame_rate: bool,
    pub system_overview_graph: UPtr<UEdGraph>,
    pub overview_graph_view_settings: FNiagaraGraphViewSettings,
    pub asset_viewport_settings: FNiagaraPerAssetViewportSettings,
    pub b_system_is_placeholder: bool,
    pub user_parameter_meta_data: TArray<UPtr<UNiagaraScriptVariable>>,
}
pub struct UNiagaraSystemFactoryNew {}
pub struct UNiagaraThumbnailRendererBase {}
pub struct UNiagaraEmitterThumbnailRenderer {}
pub struct UNiagaraSystemThumbnailRenderer {}
pub struct UNiagaraValidationRule_NoWarmupTime {}
pub struct UNiagaraValidationRule_NoEvents {
    pub severity: ENiagaraValidationSeverity,
    pub platforms: FNiagaraPlatformSet,
}
pub struct UNiagaraValidationRule_FixedGPUBoundsSet {}
pub struct UNiagaraValidationRule_EmitterCount {
    pub severity: ENiagaraValidationSeverity,
    pub emitter_count_limits: TArray<FNiagaraValidationRule_EmitterCountAndPlatformSet>,
}
pub struct UNiagaraValidationRule_RendererCount {
    pub severity: ENiagaraValidationSeverity,
    pub renderer_count_limits: TArray<
        FNiagaraValidationRule_RendererCountAndPlatformSet,
    >,
}
pub struct UNiagaraValidationRule_BannedRenderers {
    pub platforms: FNiagaraPlatformSet,
    pub severity: ENiagaraValidationSeverity,
    pub banned_renderers: TArray<TSubclassOf<UNiagaraRendererProperties>>,
}
pub struct UNiagaraValidationRule_Lightweight {
    pub platforms: FNiagaraPlatformSet,
    pub used_with_emitter: TOptional<ENiagaraValidationSeverity>,
    pub using_experimental_module: TOptional<ENiagaraValidationSeverity>,
}
pub struct UNiagaraValidationRule_BannedModules {
    pub platforms: FNiagaraPlatformSet,
    pub b_ban_on_gpu: bool,
    pub b_ban_on_cpu: bool,
    pub severity: ENiagaraValidationSeverity,
    pub banned_modules: TArray<UPtr<UNiagaraScript>>,
}
pub struct UNiagaraValidationRule_BannedDataInterfaces {
    pub severity: ENiagaraValidationSeverity,
    pub b_ban_on_gpu: bool,
    pub b_ban_on_cpu: bool,
    pub platforms: FNiagaraPlatformSet,
    pub banned_data_interfaces: TArray<TSubclassOf<UNiagaraDataInterface>>,
}
pub struct UNiagaraValidationRule_RendererSortingEnabled {
    pub severity: ENiagaraValidationSeverity,
    pub platforms: FNiagaraPlatformSet,
}
pub struct UNiagaraValidationRule_GpuUsage {
    pub severity: ENiagaraValidationSeverity,
    pub platforms: FNiagaraPlatformSet,
}
pub struct UNiagaraValidationRule_RibbonRenderer {
    pub severity: ENiagaraValidationSeverity,
    pub b_fail_if_used_by_gpu_simulation: bool,
    pub b_fail_if_used_by_gpu_init: bool,
    pub platforms: FNiagaraPlatformSet,
}
pub struct UNiagaraValidationRule_InvalidEffectType {}
pub struct UNiagaraValidationRule_HasEffectType {
    pub severity: ENiagaraValidationSeverity,
}
pub struct UDEPRECATED_NiagaraValidationRule_CheckDeprecatedEmitters {
    pub severity: ENiagaraValidationSeverity,
}
pub struct UNiagaraValidationRule_LWC {}
pub struct UNiagaraValidationRule_NoOpaqueRenderMaterial {}
pub struct UNiagaraValidationRule_NoFixedDeltaTime {}
pub struct UNiagaraValidationRule_SimulationStageBudget {
    pub b_max_simulation_stages_enabled: bool,
    pub b_max_iterations_per_stage_enabled: bool,
    pub b_max_total_iterations_enabled: bool,
    pub severity: ENiagaraValidationSeverity,
    pub max_simulation_stages: i32,
    pub max_iterations_per_stage: i32,
    pub max_total_iterations: i32,
}
pub struct UNiagaraValidationRule_TickDependencyCheck {
    pub severity: ENiagaraValidationSeverity,
    pub b_check_actor_component_interface: bool,
    pub b_check_camera_data_interface: bool,
    pub b_check_skeletal_mesh_interface: bool,
    pub effect_types_to_exclude: TArray<TSoftObjectPtr<UNiagaraEffectType>>,
}
pub struct UNiagaraValidationRule_UserDataInterfaces {
    pub b_only_include_exposed_u_objects: bool,
    pub banned_data_interfaces: TArray<TSubclassOf<UNiagaraDataInterface>>,
    pub allow_data_interfaces: TArray<TSubclassOf<UNiagaraDataInterface>>,
    pub severity: ENiagaraValidationSeverity,
}
pub struct UNiagaraValidationRule_SingletonModule {
    pub severity: ENiagaraValidationSeverity,
    pub b_check_detailed_usage_context: bool,
}
pub struct UNiagaraValidationRule_NoMapForOnCpu {
    pub severity: ENiagaraValidationSeverity,
}
pub struct UNiagaraValidationRule_ModuleSimTargetRestriction {
    pub severity: ENiagaraValidationSeverity,
    pub supported_sim_target: ENiagaraSimTarget,
}
pub struct UNiagaraValidationRule_MaterialUsage {
    pub failed_usage_severity: ENiagaraValidationSeverity,
}
pub struct UNiagaraValidationRuleSetFactoryNew {}
pub struct UNiagaraVersionMetaData {
    pub b_is_exposed_version: bool,
    pub change_description: FText,
    pub b_is_visible_in_version_selector: bool,
    pub b_deprecated: bool,
    pub deprecation_message: FText,
    pub version_guid: FGuid,
    pub update_script_execution: ENiagaraPythonUpdateScriptReference,
    pub python_update_script: FString,
    pub script_asset: FFilePath,
}
pub struct UMovieSceneNiagaraEmitterSectionBase {}
pub struct UMovieSceneNiagaraEmitterTrack {
    pub sections: TArray<UPtr<UMovieSceneSection>>,
    pub b_sections_were_modified: bool,
    pub emitter_handle_id: FGuid,
    pub system_path: FString,
}
pub struct UNiagaraSequence {
    pub movie_scene: UPtr<UMovieScene>,
}
pub struct UNiagaraSequencerTrackFilter {}
pub struct UMovieSceneNiagaraEmitterSection {
    pub num_loops: i32,
    pub b_start_time_included_in_first_loop_only: bool,
}
pub struct UNiagaraPythonScriptModuleInput {
    pub input: UPtr<UNiagaraClipboardFunctionInput>,
}
pub struct UUpgradeNiagaraScriptResults {
    pub b_cancelled_by_python_error: bool,
    pub old_inputs: TArray<UPtr<UNiagaraPythonScriptModuleInput>>,
    pub new_inputs: TArray<UPtr<UNiagaraPythonScriptModuleInput>>,
    pub dummy_input: UPtr<UNiagaraPythonScriptModuleInput>,
}
pub struct UNiagaraPythonModule {
    pub module_item: UPtr<UNiagaraStackModuleItem>,
}
pub struct UNiagaraPythonEmitter {}
pub struct UUpgradeNiagaraEmitterContext {
    pub b_cancelled_by_python_error: bool,
    pub old_emitter: UPtr<UNiagaraPythonEmitter>,
    pub new_emitter: UPtr<UNiagaraPythonEmitter>,
}
pub struct UNiagaraHierarchyScriptParameterRefreshContext {
    pub niagara_graph: UPtr<UNiagaraGraph>,
}
pub struct UNiagaraHierarchyScriptParameter {}
pub struct UNiagaraHierarchyScriptCategory {}
pub struct UNiagaraScriptParametersHierarchyViewModel {}
pub struct UNiagaraHierarchySummaryDataRefreshContext {
    pub renderers: TArray<UPtr<UNiagaraRendererProperties>>,
}
pub struct UNiagaraHierarchyModule {}
pub struct UNiagaraHierarchyModuleInput {
    pub display_name_override: FText,
    pub tooltip_override: FText,
}
pub struct UNiagaraHierarchyAssignmentInput {
    pub tooltip_override: FText,
}
pub struct UNiagaraHierarchyEmitterProperties {}
pub struct UNiagaraHierarchyRenderer {}
pub struct UNiagaraHierarchyEventHandler {}
pub struct UNiagaraHierarchyEventHandlerProperties {}
pub struct UNiagaraHierarchySimStage {}
pub struct UNiagaraHierarchySimStageProperties {}
pub struct UNiagaraHierarchyObjectProperty {}
pub struct UNiagaraSummaryViewViewModel {}
pub struct UNiagaraHierarchyUserParameterRefreshContext {
    pub system: UPtr<UNiagaraSystem>,
}
pub struct UNiagaraHierarchyUserParameter {
    pub user_parameter_script_variable: UPtr<UNiagaraScriptVariable>,
}
pub struct UNiagaraUserParametersHierarchyViewModel {}
pub struct UNiagaraCurveSelectionViewModel {}
pub struct UNiagaraScratchPadViewModel {}
pub struct UNiagaraScripStatsViewModelSettings {
    pub enabled_platforms: TArray<i32>,
}
pub struct UNiagaraSystemEditorDocumentsViewModel {}
pub struct UNiagaraSystemScalabilityViewModel {}
pub struct UNiagaraSystemSelectionViewModel {
    pub stack_selection: UPtr<UNiagaraStackSelection>,
    pub selection_stack_view_model: UPtr<UNiagaraStackViewModel>,
}
pub struct UNiagaraStackEntry {
    pub stack_editor_data: UPtr<UNiagaraStackEditorData>,
    pub children: TArray<UPtr<UNiagaraStackEntry>>,
    pub stack_note: UPtr<UNiagaraStackNote>,
    pub error_children: TArray<UPtr<UNiagaraStackErrorItem>>,
}
pub struct UNiagaraStackCommentCollection {}
pub struct UNiagaraStackItemGroup {
    pub group_footer: UPtr<UNiagaraStackItemGroupFooter>,
}
pub struct UNiagaraStackEmitterPropertiesGroup {
    pub properties_item: UPtr<UNiagaraStackEmitterPropertiesItem>,
}
pub struct UNiagaraStackItem {
    pub item_footer: UPtr<UNiagaraStackItemFooter>,
}
pub struct UNiagaraStackEmitterPropertiesItem {
    pub emitter_object: UPtr<UNiagaraStackObject>,
}
pub struct UNiagaraStackEmitterSummaryItem {
    pub summary_view_collection: UPtr<UNiagaraStackSummaryViewCollection>,
}
pub struct UNiagaraStackEmitterSummaryGroup {
    pub summary_item: UPtr<UNiagaraStackEmitterSummaryItem>,
}
pub struct UNiagaraStackSpacer {}
pub struct UNiagaraStackErrorItem {}
pub struct UNiagaraStackErrorItemLongDescription {}
pub struct UNiagaraStackErrorItemFix {}
pub struct UNiagaraStackErrorItemDismiss {}
pub struct UNiagaraStackEventWrapper {
    pub event_handler_script_props: TArray<FNiagaraEventScriptProperties>,
}
pub struct UNiagaraStackEventHandlerPropertiesItem {
    pub emitter_object: UPtr<UNiagaraStackObject>,
    pub event_wrapper: UPtr<UNiagaraStackEventWrapper>,
}
pub struct UNiagaraStackScriptItemGroup {}
pub struct UNiagaraStackEventScriptItemGroup {
    pub event_handler_properties: UPtr<UNiagaraStackEventHandlerPropertiesItem>,
}
pub struct UNiagaraStackItemContent {}
pub struct UNiagaraStackFunctionInput {}
pub struct UNiagaraStackCategory {
    pub category_spacer: UPtr<UNiagaraStackSpacer>,
}
pub struct UNiagaraStackScriptHierarchyCategory {}
pub struct UNiagaraStackSummaryCategory {}
pub struct UNiagaraStackItemTextContent {}
pub struct UNiagaraStackItemFooter {}
pub struct UNiagaraStackItemGroupFooter {}
pub struct UNiagaraStackModuleItem {
    pub linked_input_collection: UPtr<UNiagaraStackModuleItemLinkedInputCollection>,
    pub input_root: UPtr<UNiagaraStackScriptHierarchyRoot>,
    pub output_collection: UPtr<UNiagaraStackModuleItemOutputCollection>,
}
pub struct UNiagaraStackModuleItemLinkedInputCollection {}
pub struct UNiagaraStackModuleItemOutput {}
pub struct UNiagaraStackModuleItemOutputCollection {}
pub struct UNiagaraStackNote {}
pub struct UNiagaraStackObject {}
pub struct UNiagaraStackPropertyRow {
    pub category_spacer: UPtr<UNiagaraStackSpacer>,
}
pub struct UNiagaraStackRendererItem {
    pub renderer_object: UPtr<UNiagaraStackObject>,
}
pub struct UNiagaraStackRenderItemGroup {}
pub struct UNiagaraStackRoot {}
pub struct UNiagaraStackScriptHierarchyRoot {}
pub struct UNiagaraStackSelection {}
pub struct UNiagaraStackSimulationStagePropertiesItem {
    pub simulation_stage_object: UPtr<UNiagaraStackObject>,
}
pub struct UNiagaraStackSimulationStageGroup {
    pub simulation_stage_properties: UPtr<UNiagaraStackSimulationStagePropertiesItem>,
}
pub struct UNiagaraStackStatelessEmitterGroup {}
pub struct UNiagaraStackStatelessEmitterObjectItem {}
pub struct UNiagaraStackStatelessEmitterSimulateGroup {}
pub struct UNiagaraStackStatelessModuleItem {}
pub struct UNiagaraStackStatelessEmitterSpawnGroup {}
pub struct UNiagaraStackStatelessEmitterSpawnItem {}
pub struct UNiagaraStackValueCollection {}
pub struct UNiagaraStackSummaryViewCollection {}
pub struct UNiagaraStackSystemPropertiesItem {
    pub system_object: UPtr<UNiagaraStackObject>,
}
pub struct UNiagaraStackSystemPropertiesGroup {}
pub struct UNiagaraStackSystemUserParametersGroup {}
pub struct UNiagaraStackViewModel {
    pub root_entry: UPtr<UNiagaraStackEntry>,
}
pub struct UVolumeCacheFactory {}
pub struct UNiagaraDataChannelReadModuleData {
    pub data_channel: UPtr<UNiagaraDataChannelAsset>,
    pub b_read_current_frame: bool,
    pub b_update_source_data_every_tick: bool,
    pub b_auto_transform_position_data: bool,
}
pub struct UNiagaraDataChannelSpawnModuleData {
    pub data_channel: UPtr<UNiagaraDataChannelAsset>,
    pub spawn_mode: ENiagaraDataChanneSpawnModuleMode,
    pub b_read_current_frame: bool,
    pub b_update_source_data_every_tick: bool,
    pub b_auto_transform_position_data: bool,
    pub b_modify_spawn_count_by_scalability: bool,
}
pub struct UNiagaraDataChannelWriteModuleData {
    pub data_channel: UPtr<UNiagaraDataChannelAsset>,
    pub write_mode: ENiagaraDataChanneWriteModuleMode,
    pub b_publish_to_game: bool,
    pub b_publish_to_cpu: bool,
    pub b_publish_to_gpu: bool,
    pub allocation_mode: ENiagaraDataChannelAllocationMode,
    pub allocation_count: u32,
    pub b_update_destination_data_every_tick: bool,
    pub b_auto_transform_position_data: bool,
}
