#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FBlueprintWatchedPin {
    pub owning_node: TSoftObjectPtr<crate::bindings::engine::UEdGraphNode>,
    pub pin_id: crate::bindings::core_u_object::FGuid,
    pub path_to_property: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FBlueprintBreakpoint {
    pub flags_0: u8,
    pub node: TSoftObjectPtr<crate::bindings::engine::UEdGraphNode>,
    pub flags_56: u8,
}
#[repr(C, align(8))]
pub struct FDFontParameters {
    pub font_value: UPtr<crate::bindings::engine::UFont>,
    pub font_page: i32,
}
#[repr(C, align(8))]
pub struct FScalarParameterAtlasData {
    pub b_is_used_as_atlas_position: bool,
    pub curve: TSoftObjectPtr<crate::bindings::engine::UCurveLinearColor>,
    pub atlas: TSoftObjectPtr<crate::bindings::engine::UCurveLinearColorAtlas>,
}
#[repr(C, align(4))]
pub struct FDComponentMaskParameter {
    pub flags_0: u8,
}
#[repr(C, align(8))]
pub struct FAutoReimportWildcard {
    pub wildcard: FString,
    pub b_include: bool,
}
#[repr(C, align(8))]
pub struct FAutoReimportDirectoryConfig {
    pub source_directory: FString,
    pub mount_point: FString,
    pub wildcards: TArray<FAutoReimportWildcard>,
}
#[repr(C, align(8))]
pub struct FPlayScreenResolution {
    pub description: FString,
    pub width: i32,
    pub height: i32,
    pub aspect_ratio: FString,
    pub b_can_swap_aspect_ratio: bool,
    pub profile_name: FString,
    pub scale_factor: f32,
    pub logical_height: i32,
    pub logical_width: i32,
}
#[repr(C, align(8))]
pub struct FLevelEditorViewportInstanceSettings {
    pub viewport_type: ELevelViewportType,
    pub persp_view_mode_index: crate::bindings::engine::EViewModeIndex,
    pub ortho_view_mode_index: crate::bindings::engine::EViewModeIndex,
    pub editor_show_flags_string: FString,
    pub game_show_flags_string: FString,
    pub buffer_visualization_mode: FName,
    pub nanite_visualization_mode: FName,
    pub lumen_visualization_mode: FName,
    pub substrate_visualization_mode: FName,
    pub groom_visualization_mode: FName,
    pub virtual_shadow_map_visualization_mode: FName,
    pub virtual_texture_visualization_mode: FName,
    pub ray_tracing_debug_visualization_mode: FName,
    pub gpu_skin_cache_visualization_mode: FName,
    pub exposure_settings: crate::bindings::engine::FExposureSettings,
    pub fov_angle: f32,
    pub far_view_plane: f32,
    pub b_is_realtime: bool,
    pub b_show_fps_deprecated: bool,
    pub b_show_on_screen_stats: bool,
    pub enabled_stats: TArray<FString>,
    pub b_show_full_toolbar: bool,
    pub b_allow_cinematic_control: bool,
}
#[repr(C, align(8))]
pub struct FLevelEditorViewportInstanceSettingsKeyValuePair {
    pub config_name: FString,
    pub config_settings: FLevelEditorViewportInstanceSettings,
}
#[repr(C, align(4))]
pub struct FSnapToSurfaceSettings {
    pub b_enabled: bool,
    pub snap_offset_extent: f32,
    pub b_snap_rotation: bool,
}
#[repr(C, align(8))]
pub struct FLevelEditorViewporEditorViews {
    pub level_viewports_info: TArray<crate::bindings::engine::FLevelViewportInfo>,
}
#[repr(C, align(8))]
pub struct FEditorParameterGroup {
    pub group_name: FName,
    pub group_association: crate::bindings::engine::EMaterialParameterAssociation,
    pub parameters: TArray<UPtr<UDEditorParameterValue>>,
    pub group_sort_priority: i32,
}
#[repr(C, align(4))]
pub struct FEditorParameterValue {
    pub flags_0: u8,
    pub parameter_info: crate::bindings::engine::FMaterialParameterInfo,
    pub expression_id: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(4))]
pub struct FEditorVectorParameterValue {
    pub parameter_value: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(4))]
pub struct FEditorScalarParameterValue {
    pub parameter_value: f32,
}
#[repr(C, align(8))]
pub struct FEditorTextureParameterValue {
    pub parameter_value: UPtr<crate::bindings::engine::UTexture>,
}
#[repr(C, align(8))]
pub struct FEditorFontParameterValue {
    pub font_value: UPtr<crate::bindings::engine::UFont>,
    pub font_page: i32,
}
#[repr(C, align(8))]
pub struct FEditorMaterialLayersParameterValue {
    pub function_value: UPtr<crate::bindings::engine::UMaterialFunctionInterface>,
}
#[repr(C, align(4))]
pub struct FEditorStaticSwitchParameterValue {
    pub flags_40: u8,
}
#[repr(C, align(4))]
pub struct FComponentMaskParameter {
    pub flags_0: u8,
}
#[repr(C, align(4))]
pub struct FEditorStaticComponentMaskParameterValue {
    pub parameter_value: FComponentMaskParameter,
}
#[repr(C, align(4))]
pub struct FEditorUserSceneTextureOverride {
    pub key: FName,
    pub value: FName,
}
#[repr(C, align(8))]
pub struct FMaterialEditorPostProcessOverrides {
    pub b_is_overrideable: bool,
    pub b_override_blendable_location: bool,
    pub b_override_blendable_priority: bool,
    pub blendable_location_override: crate::bindings::engine::EBlendableLocation,
    pub blendable_priority_override: i32,
    pub user_scene_texture_inputs: TArray<FEditorUserSceneTextureOverride>,
    pub user_scene_texture_output: FName,
}
#[repr(C, align(4))]
pub struct FViewportConfigOptions {
    pub view_mode_index: crate::bindings::engine::EViewModeIndex,
    pub view_fov: f32,
    pub camera_speed_settings: FEditorViewportCameraSpeedSettings,
    pub camera_speed_setting_deprecated: i32,
    pub camera_speed_scalar_deprecated: f32,
    pub camera_follow_mode: EAnimationViewportCameraFollowMode,
    pub camera_follow_bone_name: FName,
}
#[repr(C, align(4))]
pub struct FEditorViewportCameraSpeedSettings {
    pub current_speed: f32,
    pub absolute_min_speed: f32,
    pub absolute_max_speed: f32,
    pub min_ui_speed: f32,
    pub max_ui_speed: f32,
}
#[repr(C, align(4))]
pub struct FAssetEditorOptions {
    pub context: FName,
    pub viewport_configs: FViewportConfigOptions,
}
#[repr(C, align(8))]
pub struct FSlatePlayInEditorInfo {}
#[repr(C, align(8))]
pub struct FPIELoginInfo {
    pub ty: FString,
    pub id: FString,
    pub token: FString,
}
#[repr(C, align(8))]
pub struct FPropertyColorCustomProperty {
    pub name: FName,
    pub text: FText,
    pub text_tool_tip: FText,
    pub property_chain: FString,
    pub property_value: FString,
    pub property_color: crate::bindings::core_u_object::FColor,
    pub default_color: crate::bindings::core_u_object::FColor,
}
#[repr(C, align(8))]
pub struct FTemplateMapInfo {
    pub thumbnail_texture: TSoftObjectPtr<crate::bindings::engine::UTexture2D>,
    pub thumbnail: crate::bindings::core_u_object::FSoftObjectPath,
    pub map: crate::bindings::core_u_object::FSoftObjectPath,
    pub display_name: FText,
    pub category: FString,
}
#[repr(C, align(8))]
pub struct FEditorKeyBinding {
    pub flags_0: u8,
    pub key: crate::bindings::input_core::FKey,
    pub command_name: FName,
}
#[repr(C, align(4))]
pub struct FEditorCommandCategory {
    pub parent: FName,
    pub name: FName,
}
#[repr(C, align(8))]
pub struct FEditorCommand {
    pub parent: FName,
    pub command_name: FName,
    pub exec_command: FString,
    pub description: FString,
}
#[repr(C, align(8))]
pub struct FClassPickerDefaults {
    pub class_name: FString,
    pub asset_class: FString,
}
#[repr(C, align(8))]
pub struct FSelectedSocketInfo {}
#[repr(C, align(8))]
pub struct FIniStringValue {
    pub section: FString,
    pub key: FString,
    pub value: FString,
    pub filename: FString,
    pub branch: FString,
}
#[repr(C, align(4))]
pub struct FChunkDependency {
    pub chunk_id: i32,
    pub parent_chunk_id: i32,
}
#[repr(C, align(8))]
pub struct FDialogueScriptEntry {
    pub speaking_voice: FString,
    pub target_voices: TArray<FString>,
    pub spoken_dialogue: FString,
    pub voice_actor_direction: FString,
    pub audio_file_name: FString,
    pub dialogue_asset: FString,
    pub is_recorded: bool,
    pub localization_keys: TArray<FString>,
    pub speaking_voice_guid: FString,
    pub target_voice_gui_ds: TArray<FString>,
    pub dialogue_asset_guid: FString,
}
#[repr(C, align(8))]
pub struct FGatherTextCommandletTask {
    pub commandlet_params: FString,
    pub commandlet: UPtr<UGatherTextCommandletBase>,
}
#[repr(C, align(8))]
pub struct FGatherTextCommandletPhase {
    pub sequential_tasks: TArray<FGatherTextCommandletTask>,
    pub parallel_tasks: TArray<FGatherTextCommandletTask>,
}
#[repr(C, align(4))]
pub struct FGatherTextFromAssetsWorkerMessage_Ping {
    pub protocol_version: i32,
}
#[repr(C, align(8))]
pub struct FGatherTextFromAssetsWorkerMessage_Pong {
    pub worker_id: crate::bindings::core_u_object::FGuid,
    pub idle_start_time_utc: TOptional<crate::bindings::core_u_object::FDateTime>,
}
#[repr(C, align(8))]
pub struct FGatherTextFromAssetsWorkerMessage_PackageRequest {
    pub package_name: FName,
    pub dependencies: TSet<FName>,
    pub external_actors: TSet<crate::bindings::core_u_object::FGuid>,
}
#[repr(C, align(8))]
pub struct FGatherTextFromAssetsWorkerMessage_PackageResult {
    pub worker_id: crate::bindings::core_u_object::FGuid,
    pub package_name: FName,
    pub gatherable_text_data: TArray<u8>,
    pub load_log_capture: FString,
    pub b_load_error: bool,
}
#[repr(C, align(8))]
pub struct FHLODLayerActorMapping {
    pub actor_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub hlod_layer: crate::bindings::core_u_object::FTopLevelAssetPath,
}
#[repr(C, align(4))]
pub struct FPropertyNameAndIndex {
    pub name: FName,
    pub index: i32,
}
#[repr(C, align(8))]
pub struct FComponentPropertyPath {
    pub parent_owning_actor: TWeakObjectPtr<crate::bindings::engine::AActor>,
    pub last_resort_component_ptr: TWeakObjectPtr<
        crate::bindings::engine::UActorComponent,
    >,
    pub property_chain: TArray<FPropertyNameAndIndex>,
}
#[repr(C, align(8))]
pub struct FZenCookOnTheFlyRegisterServiceMessage {
    pub service_id: FString,
    pub port: i32,
}
#[repr(C, align(8))]
pub struct FCopySelectedInfo {}
#[repr(C, align(8))]
pub struct FSelectionStateOfLevel {
    pub selected_actors: TArray<FString>,
    pub selected_components: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FEditorStateCollection {
    pub editor_states_deprecated: TMap<TSubclassOf<UEditorState>, UPtr<UEditorState>>,
    pub states: TArray<UPtr<UEditorState>>,
}
#[repr(C, align(8))]
pub struct FEditorWorldExtensionActorData {
    pub actor: UPtr<crate::bindings::engine::AActor>,
    pub b_valid_for_pie: bool,
}
#[repr(C, align(8))]
pub struct FCSVImportSettings {
    pub import_row_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub import_type: ECSVImportType,
    pub import_curve_interp_mode: crate::bindings::engine::ERichCurveInterpMode,
}
#[repr(C, align(4))]
pub struct FFbxMaterialBakeSize {
    pub size: crate::bindings::core_u_object::FIntPoint,
    pub b_auto_detect: bool,
}
#[repr(C, align(8))]
pub struct FImportMeshLodSectionsData {
    pub section_original_material_name: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FMaterialGraphSchemaAction_NewNode {
    pub material_expression_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FMaterialGraphSchemaAction_NewFunctionCall {
    pub function_path: FString,
}
#[repr(C, align(8))]
pub struct FMaterialGraphSchemaAction_NewComposite {}
#[repr(C, align(8))]
pub struct FMaterialGraphSchemaAction_NewComment {}
#[repr(C, align(8))]
pub struct FMaterialGraphSchemaAction_NewNamedRerouteUsage {
    pub declaration: UPtr<
        crate::bindings::engine::UMaterialExpressionNamedRerouteDeclaration,
    >,
}
#[repr(C, align(8))]
pub struct FMaterialGraphSchemaAction_Paste {}
#[repr(C, align(8))]
pub struct FPropertyEditTestTextStruct {
    pub normal_property: FText,
}
#[repr(C, align(8))]
pub struct FPropertyEditorTestSubStruct {
    pub first_property: i32,
    pub second_property: i32,
    pub customized_struct_inside_uncustomized_struct: crate::bindings::core_u_object::FLinearColor,
    pub customized_struct_inside_uncustomized_struct2: crate::bindings::core_u_object::FSoftObjectPath,
}
#[repr(C, align(8))]
pub struct FPropertyEditorTestBasicStruct {
    pub int_property_inside_a_struct: i32,
    pub float_property_inside_a_struct: f32,
    pub object_property_inside_a_struct: UPtr<crate::bindings::core_u_object::UObject>,
    pub inner_struct: FPropertyEditorTestSubStruct,
}
#[repr(C, align(4))]
pub struct FPropertyEditorTestEditCondition {
    pub inline_edit_condition: bool,
    pub has_inline_edit_condition: i32,
    pub flags: ETestEnumFlags,
    pub enabled_and_visible_when_one: i32,
}
#[repr(C, align(8))]
pub struct FPropertyEditorTestInstancedStruct {
    pub object: UPtr<UPropertyEditorTestInstancedObject>,
}
#[repr(C, align(8))]
pub struct FTestSparseClassDataStorage {
    pub map: TMap<i32, i32>,
}
#[repr(C, align(8))]
pub struct FMode2DLayer {
    pub name: FString,
    pub depth: f32,
}
#[repr(C, align(8))]
pub struct FRecentProjectFile {
    pub project_name: FString,
    pub last_open_time: crate::bindings::core_u_object::FDateTime,
}
#[repr(C, align(4))]
pub struct FNetworkEmulationPacketSettings {
    pub min_latency: i32,
    pub max_latency: i32,
    pub packet_loss_percentage: i32,
}
#[repr(C, align(8))]
pub struct FLevelEditorPlayNetworkEmulationSettings {
    pub b_is_network_emulation_enabled: bool,
    pub emulation_target: NetworkEmulationTarget,
    pub current_profile: FString,
    pub out_packets: FNetworkEmulationPacketSettings,
    pub in_packets: FNetworkEmulationPacketSettings,
}
#[repr(C, align(8))]
pub struct FCollectionScriptingContainerSource {
    pub name: FName,
    pub title: FText,
}
#[repr(C, align(4))]
pub struct FCollectionScriptingRef {
    pub container: FName,
    pub name: FName,
    pub share_type: crate::bindings::engine::ECollectionScriptingShareType,
}
#[repr(C, align(8))]
pub struct FFbxTestPlanExpectedResult {
    pub expected_presets_type: EFBXExpectedResultPreset,
    pub expected_presets_data_integer: TArray<i32>,
    pub expected_presets_data_float: TArray<f32>,
    pub expected_presets_data_double: TArray<f64>,
    pub expected_presets_data_string: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FThumbnailRenderingInfo {
    pub class_needing_thumbnail_name: FString,
    pub class_needing_thumbnail: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub renderer_class_name: FString,
    pub renderer: UPtr<UThumbnailRenderer>,
    pub b_use_class_default_object: bool,
}
#[repr(C, align(8))]
pub struct FClassMoveInfo {
    pub class_name: FString,
    pub package_name: FString,
    pub group_name: FString,
    pub flags_48: u8,
}
#[repr(C, align(4))]
pub struct FLightmassParameterValue {
    pub flags_0: u8,
}
#[repr(C, align(4))]
pub struct FLightmassBooleanParameterValue {
    pub flags_4: u8,
}
#[repr(C, align(4))]
pub struct FLightmassScalarParameterValue {
    pub parameter_value: f32,
}
#[repr(C, align(4))]
pub struct FLightmassParameterizedMaterialSettings {
    pub cast_shadow_as_masked: FLightmassBooleanParameterValue,
    pub emissive_boost: FLightmassScalarParameterValue,
    pub diffuse_boost: FLightmassScalarParameterValue,
    pub export_resolution_scale: FLightmassScalarParameterValue,
}
#[repr(C, align(8))]
pub struct FStructVariableDescription {
    pub var_name: FName,
    pub var_guid: crate::bindings::core_u_object::FGuid,
    pub friendly_name: FString,
    pub default_value: FString,
    pub category: FName,
    pub sub_category: FName,
    pub sub_category_object: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    pub pin_value_type: crate::bindings::engine::FEdGraphTerminalType,
    pub container_type: crate::bindings::engine::EPinContainerType,
    pub flags_173: u8,
    pub current_default_value: FString,
    pub tool_tip: FString,
    pub meta_data: TMap<FName, FString>,
}
#[repr(C, align(1))]
pub struct FActorFolderProps {}
#[repr(C, align(4))]
pub struct FActorPlacementFolder {
    pub path: FName,
    pub root_object_ptr: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
    pub actor_folder_guid: crate::bindings::core_u_object::FGuid,
}
pub struct UAssetDefinitionDefault {}
pub struct UFactory {
    pub flags_48: u8,
    pub supported_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub context_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub formats: TArray<FString>,
    pub flags_88: u8,
    pub import_priority: i32,
    pub automated_import_data: UPtr<UAutomatedAssetImportData>,
    pub asset_import_task: UPtr<UAssetImportTask>,
    pub supported_workflows: u8,
    pub overwrite_yes_or_no_to_all_state: i32,
}
pub struct UEditorWorldExtension {
    pub extension_actors: TArray<FEditorWorldExtensionActorData>,
}
pub struct UActorFactory {
    pub display_name: FText,
    pub menu_priority: i32,
    pub new_actor_class_name: FString,
    pub b_should_auto_register: bool,
    pub new_actor_class: TSubclassOf<crate::bindings::engine::AActor>,
    pub flags_112: u8,
    pub spawn_position_offset: crate::bindings::core_u_object::FVector,
}
pub struct UActorFactoryVolume {}
pub struct UActorFactoryBoxVolume {}
pub struct UThumbnailRenderer {}
pub struct UDefaultSizedThumbnailRenderer {
    pub default_size_x: i32,
    pub default_size_y: i32,
}
pub struct UBlueprintThumbnailRenderer {}
pub struct UEditorState {}
pub struct UWorldDependantEditorState {}
pub struct UActorEditorContextStateCollection {
    pub client_states: TMap<
        TSubclassOf<UActorEditorContextClientState>,
        UPtr<UActorEditorContextClientState>,
    >,
}
pub struct UActorEditorContextClientState {}
pub struct UBaseWidgetBlueprint {
    pub widget_tree: UPtr<crate::bindings::umg::UWidgetTree>,
}
pub struct UEditorInteractiveToolsContext {
    pub standard_vertex_color_material: UPtr<
        crate::bindings::engine::UMaterialInterface,
    >,
}
pub struct UModeManagerInteractiveToolsContext {
    pub ed_mode_tools_contexts: TArray<UPtr<UEdModeInteractiveToolsContext>>,
    pub drag_tools_behavior_source: UPtr<
        crate::bindings::editor_interactive_tools_framework::UDragToolsBehaviorSource,
    >,
    pub viewport_interactions_behavior_source: UPtr<
        crate::bindings::editor_interactive_tools_framework::UViewportInteractionsBehaviorSource,
    >,
}
pub struct UActorExporterT3D {}
pub struct UGroupActorExporterT3D {}
pub struct UPhysicsVolumeExporterT3D {}
pub struct UActorFactoryAmbientSound {}
pub struct UActorFactorySkeletalMesh {}
pub struct UActorFactoryAnimationAsset {}
pub struct UActorFactoryStaticMesh {}
pub struct UActorFactoryBasicShape {}
pub struct UActorFactoryBlueprint {}
pub struct UActorFactoryBoxReflectionCapture {}
pub struct UActorFactoryCameraActor {}
pub struct UActorFactoryCharacter {}
pub struct UActorFactoryClass {}
pub struct UActorFactoryCylinderVolume {}
pub struct UActorFactoryDeferredDecal {}
pub struct UActorFactoryDirectionalLight {}
pub struct UActorFactoryEmitter {}
pub struct UActorFactoryEmptyActor {
    pub b_visualize_actor: bool,
}
pub struct UActorFactoryExponentialHeightFog {}
pub struct UActorFactoryInteractiveFoliage {}
pub struct UActorFactoryLevelSequence {}
pub struct UActorFactoryLocalFogVolume {}
pub struct UActorFactoryNote {}
pub struct UActorFactoryPawn {}
pub struct UActorFactoryPhysicsAsset {}
pub struct UActorFactoryPlanarReflection {}
pub struct UActorFactoryPlaneReflectionCapture {}
pub struct UActorFactoryPlayerStart {}
pub struct UActorFactoryRuntimeVirtualTextureVolume {}
pub struct UActorFactorySkyAtmosphere {}
pub struct UActorFactorySkyLight {}
pub struct UActorFactorySphereReflectionCapture {}
pub struct UActorFactorySphereVolume {}
pub struct UActorFactoryTargetPoint {}
pub struct UActorFactoryTextRender {}
pub struct UActorFactoryTriggerBox {}
pub struct UActorFactoryTriggerCapsule {}
pub struct UActorFactoryTriggerSphere {}
pub struct UActorFactoryVectorFieldVolume {}
pub struct UActorFactoryVolumetricCloud {}
pub struct UBlendSpaceFactory1D {
    pub target_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub preview_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
}
pub struct UAimOffsetBlendSpaceFactory1D {}
pub struct UBlendSpaceFactoryNew {
    pub target_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub preview_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
}
pub struct UAimOffsetBlendSpaceFactoryNew {}
pub struct UAnimationBlueprintEditorOptions {
    pub flags_48: u8,
}
pub struct UAnimBlueprintSettings {
    pub b_allow_anim_blueprints: bool,
    pub b_allow_event_graphs: bool,
    pub b_allow_macros: bool,
    pub b_allow_delegates: bool,
    pub b_allow_pose_watches: bool,
    pub b_restrict_base_function_overrides: bool,
    pub b_support_input_events_for_backwards_compatibility: bool,
    pub b_perform_validation: bool,
    pub base_function_override_allow_list: TArray<FName>,
}
pub struct UExporterFBX {}
pub struct UAnimSequenceExporterFBX {}
pub struct UAssetEditorToolkitMenuContext {}
pub struct UBlueprintFactory {
    pub parent_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub blueprint_type: crate::bindings::engine::EBlueprintType,
    pub b_skip_class_picker: bool,
}
pub struct UBlueprintFunctionLibraryFactory {}
pub struct UBlueprintInterfaceFactory {}
pub struct UBlueprintMacroFactory {}
pub struct UCanvasRenderTarget2DFactoryNew {
    pub width: i32,
    pub height: i32,
    pub format: u8,
}
pub struct UCascadeOptions {
    pub flags_48: u8,
    pub background_color: crate::bindings::core_u_object::FColor,
    pub flags_56: u8,
    pub empty_background: crate::bindings::core_u_object::FColor,
    pub emitter_background: crate::bindings::core_u_object::FColor,
    pub emitter_unselected: crate::bindings::core_u_object::FColor,
    pub emitter_selected: crate::bindings::core_u_object::FColor,
    pub module_color_general_unselected: crate::bindings::core_u_object::FColor,
    pub module_color_general_selected: crate::bindings::core_u_object::FColor,
    pub module_color_type_data_unselected: crate::bindings::core_u_object::FColor,
    pub module_color_type_data_selected: crate::bindings::core_u_object::FColor,
    pub module_color_beam_unselected: crate::bindings::core_u_object::FColor,
    pub module_color_beam_selected: crate::bindings::core_u_object::FColor,
    pub module_color_trail_unselected: crate::bindings::core_u_object::FColor,
    pub module_color_trail_selected: crate::bindings::core_u_object::FColor,
    pub module_color_spawn_unselected: crate::bindings::core_u_object::FColor,
    pub module_color_spawn_selected: crate::bindings::core_u_object::FColor,
    pub module_color_light_unselected: crate::bindings::core_u_object::FColor,
    pub module_color_light_selected: crate::bindings::core_u_object::FColor,
    pub module_color_sub_uv_unselected: crate::bindings::core_u_object::FColor,
    pub module_color_sub_uv_selected: crate::bindings::core_u_object::FColor,
    pub module_color_required_unselected: crate::bindings::core_u_object::FColor,
    pub module_color_required_selected: crate::bindings::core_u_object::FColor,
    pub module_color_event_unselected: crate::bindings::core_u_object::FColor,
    pub module_color_event_selected: crate::bindings::core_u_object::FColor,
    pub flags_148: u8,
    pub grid_color_hi: crate::bindings::core_u_object::FColor,
    pub grid_color_low: crate::bindings::core_u_object::FColor,
    pub grid_perspective_size: f32,
    pub flags_164: u8,
    pub particle_memory_update_time: f32,
    pub flags_172: u8,
    pub floor_mesh: FString,
    pub floor_position: crate::bindings::core_u_object::FVector,
    pub floor_rotation: crate::bindings::core_u_object::FRotator,
    pub floor_scale: f32,
    pub floor_scale3_d: crate::bindings::core_u_object::FVector,
    pub show_pp_flags: i32,
    pub flags_276: u8,
    pub slim_cascade_draw_height: i32,
    pub flags_284: u8,
    pub cascade_mouse_move_threshold: i32,
    pub motion_mode_radius: f32,
}
pub struct UClassViewerSettings {
    pub allowed_classes: TArray<FString>,
    pub display_internal_classes: bool,
    pub developer_folder_type: EClassViewerDeveloperType,
}
pub struct UCompressAnimationsCommandlet {}
pub struct UEditorBrushBuilder {}
pub struct UConeBuilder {
    pub z: f32,
    pub cap_z: f32,
    pub outer_radius: f32,
    pub inner_radius: f32,
    pub sides: i32,
    pub group_name: FName,
    pub flags_168: u8,
}
pub struct UContentBrowserSettings {
    pub num_objects_to_load_before_warning: i32,
    pub real_time_thumbnails: bool,
    pub display_folders: bool,
    pub display_empty_folders: bool,
    pub filter_recursively: bool,
    pub b_show_all_folder: bool,
    pub b_organize_folders: bool,
    pub b_display_content_folder_suffix: bool,
    pub b_display_friendly_name_for_plugin_folders: bool,
    pub num_objects_in_recent_list_deprecated: i32,
    pub b_enable_realtime_material_instance_thumbnails: bool,
    pub b_display_excluded_collections: bool,
    pub display_engine_folder: bool,
    pub display_developers_folder: bool,
    pub display_l10n_folder: bool,
    pub display_plugin_folders: bool,
    pub display_favorites: bool,
    pub display_cpp_folders: bool,
    pub include_class_names: bool,
    pub include_asset_paths: bool,
    pub include_collection_names: bool,
    pub always_expand_tooltips: bool,
}
pub struct UCookGlobalShadersCommandlet {}
pub struct UCookGlobalShadersDeviceHelperBase {}
pub struct UCookGlobalShadersDeviceHelperStaged {}
pub struct UCrashReporterSettings {
    pub upload_symbols_path: FString,
    pub downstream_storage: FString,
    pub remote_storage: TArray<FString>,
}
pub struct UCubeBuilder {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub wall_thickness: f32,
    pub group_name: FName,
    pub flags_164: u8,
}
pub struct UCurvedStairBuilder {
    pub inner_radius: i32,
    pub step_height: i32,
    pub step_width: i32,
    pub angle_of_curve: i32,
    pub num_steps: i32,
    pub add_to_first_step: i32,
    pub group_name: FName,
    pub flags_172: u8,
}
pub struct UCurveEdOptions {
    pub min_view_range: f32,
    pub max_view_range: f32,
    pub background_color: crate::bindings::core_u_object::FLinearColor,
    pub label_color: crate::bindings::core_u_object::FLinearColor,
    pub selected_label_color: crate::bindings::core_u_object::FLinearColor,
    pub grid_color: crate::bindings::core_u_object::FLinearColor,
    pub grid_text_color: crate::bindings::core_u_object::FLinearColor,
    pub label_block_bkg_color: crate::bindings::core_u_object::FLinearColor,
    pub selected_key_color: crate::bindings::core_u_object::FLinearColor,
}
pub struct UCurveFactory {
    pub curve_class: TSubclassOf<crate::bindings::engine::UCurveBase>,
}
pub struct UCurveFloatFactory {}
pub struct UCurveLinearColorFactory {}
pub struct UCurveVectorFactory {}
pub struct UCurveImportFactory {}
pub struct UCurveLinearColorAtlasFactory {
    pub width: i32,
}
pub struct UCylinderBuilder {
    pub z: f32,
    pub outer_radius: f32,
    pub inner_radius: f32,
    pub sides: i32,
    pub group_name: FName,
    pub flags_164: u8,
}
pub struct UDataAssetFactory {
    pub data_asset_class: TSubclassOf<crate::bindings::engine::UDataAsset>,
}
pub struct UDEditorParameterValue {
    pub flags_48: u8,
    pub parameter_info: crate::bindings::engine::FMaterialParameterInfo,
    pub expression_id: crate::bindings::core_u_object::FGuid,
    pub description: FString,
    pub asset_path: FString,
    pub sort_priority: i32,
}
pub struct UDEditorDoubleVectorParameterValue {
    pub parameter_value: crate::bindings::core_u_object::FVector4d,
}
pub struct UDEditorFontParameterValue {
    pub parameter_value: FDFontParameters,
}
pub struct UDEditorMaterialLayersParameterValue {
    pub parameter_value: crate::bindings::engine::FMaterialLayersFunctions,
}
pub struct UDEditorParameterCollectionParameterValue {
    pub parameter_value: UPtr<crate::bindings::engine::UMaterialParameterCollection>,
}
pub struct UDEditorRuntimeVirtualTextureParameterValue {
    pub parameter_value: UPtr<crate::bindings::engine::URuntimeVirtualTexture>,
}
pub struct UDEditorScalarParameterValue {
    pub parameter_value: f32,
    pub atlas_data: FScalarParameterAtlasData,
    pub enumeration: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
}
pub struct UDEditorSparseVolumeTextureParameterValue {
    pub parameter_value: UPtr<crate::bindings::engine::USparseVolumeTexture>,
}
pub struct UDEditorStaticComponentMaskParameterValue {
    pub parameter_value: FDComponentMaskParameter,
}
pub struct UDEditorStaticSwitchParameterValue {
    pub flags_128: u8,
}
pub struct UDEditorTextureCollectionParameterValue {
    pub parameter_value: UPtr<crate::bindings::engine::UTextureCollection>,
}
pub struct UDEditorTextureParameterValue {
    pub parameter_value: UPtr<crate::bindings::engine::UTexture>,
    pub channel_names: crate::bindings::engine::FParameterChannelNames,
}
pub struct UDEditorVectorParameterValue {
    pub parameter_value: crate::bindings::core_u_object::FLinearColor,
    pub b_is_used_as_channel_mask: bool,
    pub b_use_custom_primitive_data: bool,
    pub channel_names: crate::bindings::engine::FParameterChannelNames,
}
pub struct UEditorExperimentalSettings {
    pub b_enable_async_texture_compilation: bool,
    pub b_enable_async_static_mesh_compilation: bool,
    pub b_enable_async_skeletal_mesh_compilation: bool,
    pub b_enable_async_skinned_asset_compilation: bool,
    pub b_enable_async_sound_wave_compilation: bool,
    pub b_enable_async_groom_binding_compilation: bool,
    pub b_hdr_editor: bool,
    pub hdr_editor_nit_level: f32,
    pub b_procedural_foliage: bool,
    pub b_enable_translation_picker: bool,
    pub console_for_gamepad_labels: crate::bindings::input_core::EConsoleForGamepadLabels,
    pub b_toolbar_customization: bool,
    pub b_break_on_exceptions: bool,
    pub b_draw_midpoint_arrows_in_blueprints: bool,
    pub b_context_menu_chunk_assignments: bool,
    pub b_disable_cook_in_editor: bool,
    pub b_shared_cooked_builds: bool,
    pub b_allow_late_join_in_pie: bool,
    pub b_enable_multithreaded_lightmap_encoding: bool,
    pub b_enable_multithreaded_shadowmap_encoding: bool,
    pub b_use_open_cl_for_convex_hull_decomp: bool,
    pub b_allow_potentially_unsafe_property_editing: bool,
    pub b_facial_animation_importer: bool,
    pub b_project_custom_build_tools: bool,
    pub b_enable_project_launcher_extensions: bool,
    pub b_mobile_pie_preview_device_launch: bool,
    pub b_text_asset_format_support: bool,
    pub b_virtualized_asset_rehydration: bool,
    pub b_example_layers_and_blends: bool,
    pub b_enable_long_paths_support: bool,
    pub b_enable_world_partition_actor_filters: bool,
    pub b_enable_world_partition_external_data_layers: bool,
    pub b_enable_standalone_hlod: bool,
    pub b_enable_level_instance_property_overrides: bool,
    pub b_enable_level_instance_landscape_edit: bool,
}
pub struct UEditorLoadingSavingSettings {
    pub load_level_at_startup: ELoadLevelAtStartup,
    pub flags_52: u8,
    pub restore_open_asset_tabs_on_restart: ERestoreOpenAssetTabsMethod,
    pub flags_60: u8,
    pub b_enable_source_control_compatability_check_deprecated: bool,
    pub b_monitor_content_directories: bool,
    pub auto_reimport_directories_deprecated: TArray<FString>,
    pub auto_reimport_directory_settings: TArray<FAutoReimportDirectoryConfig>,
    pub auto_reimport_threshold: f32,
    pub b_auto_create_assets: bool,
    pub b_auto_delete_assets: bool,
    pub b_detect_changes_on_startup: bool,
    pub b_prompt_before_auto_importing: bool,
    pub b_delete_source_files_with_assets: bool,
    pub b_auto_reimport_textures_deprecated: bool,
    pub b_auto_reimport_csv_deprecated: bool,
    pub b_dirty_migrated_blueprints: bool,
    pub flags_116: u8,
    pub auto_save_method: EAutoSaveMethod,
    pub auto_save_time_minutes: i32,
    pub auto_save_interaction_delay_in_seconds: i32,
    pub auto_save_warning_in_seconds: i32,
    pub auto_save_max_backups: i32,
    pub flags_140: u8,
    pub text_diff_tool_path: crate::bindings::core_u_object::FFilePath,
}
pub struct UEditorMiscSettings {}
pub struct UEditorStyleSettings {
    pub b_enable_high_dpi_awareness: bool,
    pub application_scale: f32,
    pub b_enable_user_editor_layout_management: bool,
    pub color_vision_deficiency_preview_type: crate::bindings::slate_core::EColorVisionDeficiency,
    pub color_vision_deficiency_severity: i32,
    pub b_color_vision_deficiency_correction: bool,
    pub b_color_vision_deficiency_correction_preview_with_deficiency: bool,
    pub x_axis_color: crate::bindings::core_u_object::FLinearColor,
    pub y_axis_color: crate::bindings::core_u_object::FLinearColor,
    pub z_axis_color: crate::bindings::core_u_object::FLinearColor,
    pub selection_color: crate::bindings::core_u_object::FLinearColor,
    pub additional_selection_colors: crate::bindings::core_u_object::FLinearColor,
    pub viewport_tool_overlay_color: crate::bindings::core_u_object::FLinearColor,
    pub b_enable_editor_window_background_color: bool,
    pub editor_window_background_color: crate::bindings::core_u_object::FLinearColor,
    pub flags_264: u8,
    pub menu_search_field_visibility_threshold: u32,
    pub flags_272: u8,
    pub regular_color: crate::bindings::core_u_object::FLinearColor,
    pub rule_color: crate::bindings::core_u_object::FLinearColor,
    pub center_color: crate::bindings::core_u_object::FLinearColor,
    pub grid_snap_size: u32,
    pub graph_background_brush: crate::bindings::slate_core::FSlateBrush,
    pub flags_544: u8,
    pub asset_editor_open_location: EAssetEditorOpenLocation,
    pub flags_552: u8,
    pub current_applied_theme: crate::bindings::core_u_object::FGuid,
    pub b_enable_middle_ellipsis: bool,
    pub asset_picker_size_scale: f32,
}
pub struct UEnumFactory {}
pub struct UExportTextContainer {
    pub export_text: FString,
}
pub struct UFbxImportUI {
    pub b_is_obj_import: bool,
    pub original_import_type: EFBXImportType,
    pub mesh_type_to_import: EFBXImportType,
    pub flags_60: u8,
    pub b_import_as_skeletal: bool,
    pub b_import_mesh: bool,
    pub skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub flags_80: u8,
    pub physics_asset: UPtr<crate::bindings::engine::UPhysicsAsset>,
    pub flags_96: u8,
    pub lod_distance0: f32,
    pub lod_distance1: f32,
    pub lod_distance2: f32,
    pub lod_distance3: f32,
    pub lod_distance4: f32,
    pub lod_distance5: f32,
    pub lod_distance6: f32,
    pub lod_distance7: f32,
    pub minimum_lod_number: i32,
    pub lod_number: i32,
    pub flags_140: u8,
    pub override_animation_name: FString,
    pub flags_160: u8,
    pub static_mesh_import_data: UPtr<UFbxStaticMeshImportData>,
    pub skeletal_mesh_import_data: UPtr<UFbxSkeletalMeshImportData>,
    pub anim_sequence_import_data: UPtr<UFbxAnimSequenceImportData>,
    pub texture_import_data: UPtr<UFbxTextureImportData>,
    pub b_automated_import_should_detect_type: bool,
    pub file_version: FString,
    pub file_creator: FString,
    pub file_creator_application: FString,
    pub file_units: FString,
    pub file_axis_direction: FString,
    pub file_sample_rate: FString,
    pub anim_start_frame: FString,
    pub anim_end_frame: FString,
}
pub struct UFontFactory {}
pub struct UFontFileImportFactory {}
pub struct UForceFeedbackAttenuationFactory {}
pub struct UForceFeedbackEffectFactory {}
pub struct UHapticFeedbackEffectBufferFactory {}
pub struct UHapticFeedbackEffectCurveFactory {}
pub struct UHapticFeedbackEffectSoundWaveFactory {}
pub struct ULandscapeGrassTypeCommandlet {}
pub struct ULegacyEdModeSelectInterface {}
pub struct ILegacyEdModeSelectInterface {}
pub struct ULegacyEdModeWidgetInterface {}
pub struct ILegacyEdModeWidgetInterface {}
pub struct ULegacyEdModeToolInterface {}
pub struct ILegacyEdModeToolInterface {}
pub struct ULegacyEdModeDrawHelperInterface {}
pub struct ILegacyEdModeDrawHelperInterface {}
pub struct ULegacyEdModeViewportInterface {}
pub struct ILegacyEdModeViewportInterface {}
pub struct ULevelEditorMiscSettings {
    pub flags_104: u8,
    pub flags_105: u8,
    pub editor_volume_level: f32,
    pub flags_112: u8,
    pub default_level_streaming_class: TSubclassOf<
        crate::bindings::engine::ULevelStreaming,
    >,
    pub b_prompt_when_adding_to_level_before_checkout: bool,
    pub b_prompt_when_adding_to_level_outside_bounds: bool,
    pub percentage_threshold_for_prompt: f32,
    pub minimum_bounds_for_checking_size: crate::bindings::core_u_object::FVector,
    pub editor_screenshot_save_directory: crate::bindings::core_u_object::FDirectoryPath,
}
pub struct UCommonResolutionMenuContext {}
pub struct ULevelEditorPlaySettings {
    pub play_from_here_player_start_class_name: FString,
    pub game_gets_mouse_control: bool,
    pub use_mouse_for_touch: bool,
    pub b_simulate_device_mapping_policy: bool,
    pub simulated_device_mapping_policy: crate::bindings::core_u_object::EInputDeviceMappingPolicy,
    pub show_mouse_control_label: bool,
    pub mouse_control_label_position: ELabelAnchorMode,
    pub viewport_gets_hmd_control: bool,
    pub should_minimize_editor_on_vrpie: bool,
    pub b_should_minimize_editor_on_non_vrpie: bool,
    pub b_emulate_stereo: bool,
    pub auto_recompile_blueprints: bool,
    pub enable_game_sound: bool,
    pub solo_audio_in_first_pie_client: bool,
    pub enable_pie_enter_and_exit_sounds: bool,
    pub play_in_editor_sound_quality_level: i32,
    pub b_use_non_realtime_audio_device: bool,
    pub flags_124: u8,
    pub b_promote_output_log_warnings_during_pie: bool,
    pub new_window_width: i32,
    pub new_window_height: i32,
    pub new_window_position: crate::bindings::core_u_object::FIntPoint,
    pub flags_148: u8,
    pub additional_launch_parameters: FString,
    pub build_game_before_launch: EPlayOnBuildMode,
    pub launch_configuration: EPlayOnLaunchConfiguration,
    pub pack_files_for_launch: EPlayOnPakFileMode,
    pub b_auto_compile_blueprints_on_launch: bool,
    pub b_launch_separate_server: bool,
    pub play_net_mode: EPlayNetMode,
    pub run_under_one_process: bool,
    pub play_number_of_clients: i32,
    pub primary_pie_client_index: i32,
    pub server_port: u16,
    pub client_window_width: i32,
    pub route_gamepad_to_second_window: bool,
    pub create_audio_device_for_every_player: bool,
    pub client_window_height: i32,
    pub server_map_name_override: FString,
    pub additional_server_game_options: FString,
    pub b_show_server_debug_drawing_by_default: bool,
    pub server_debug_drawing_color_tint_strength: f32,
    pub server_debug_drawing_color_tint: crate::bindings::core_u_object::FLinearColor,
    pub b_one_headset_each_process_deprecated: bool,
    pub b_hmd_for_primary_process_only: bool,
    pub additional_server_launch_parameters: FString,
    pub server_fixed_fps: i32,
    pub client_fixed_fps: TArray<i32>,
    pub network_emulation_settings: FLevelEditorPlayNetworkEmulationSettings,
    pub last_size: crate::bindings::core_u_object::FIntPoint,
    pub multiple_instance_positions: TArray<crate::bindings::core_u_object::FIntPoint>,
    pub last_executed_launch_device: FString,
    pub last_executed_launch_name: FString,
    pub last_executed_launch_mode_type: ELaunchModeType,
    pub last_executed_play_mode_location: EPlayModeLocations,
    pub last_executed_play_mode_type: EPlayModeType,
    pub last_executed_pie_preview_device: FString,
    pub laptop_screen_resolutions: TArray<FPlayScreenResolution>,
    pub monitor_screen_resolutions: TArray<FPlayScreenResolution>,
    pub phone_screen_resolutions: TArray<FPlayScreenResolution>,
    pub tablet_screen_resolutions: TArray<FPlayScreenResolution>,
    pub television_screen_resolutions: TArray<FPlayScreenResolution>,
    pub device_to_emulate: FString,
    pub pie_safe_zone_override: crate::bindings::slate_core::FMargin,
    pub custom_unsafe_zone_starts: TArray<crate::bindings::core_u_object::FVector2D>,
    pub custom_unsafe_zone_dimensions: TArray<crate::bindings::core_u_object::FVector2D>,
}
pub struct ULevelEditorViewportSettings {
    pub flight_camera_control_type: EWASDType,
    pub flight_camera_control_experimental_navigation: bool,
    pub landscape_editor_control_type: ELandscapeFoliageEditorControlType,
    pub foliage_editor_control_type: ELandscapeFoliageEditorControlType,
    pub flags_52: u8,
    pub minimum_orthographic_zoom: f32,
    pub flags_60: u8,
    pub camera_speed_settings: FEditorViewportCameraSpeedSettings,
    pub camera_speed_deprecated: i32,
    pub camera_speed_scalar_deprecated: f32,
    pub mouse_scroll_camera_speed: i32,
    pub mouse_sensitivty: f32,
    pub flags_100: u8,
    pub b_invert_mouse_look_y_axis: bool,
    pub b_invert_orbit_y_axis: bool,
    pub b_invert_middle_mouse_pan: bool,
    pub b_world_space_vertical_pan: bool,
    pub b_invert_right_mouse_dolly_y_axis: bool,
    pub flags_112: u8,
    pub b_level_streaming_volume_previs: bool,
    pub b_use_ue3_orbit_controls: bool,
    pub scroll_gesture_direction_for3_d_viewports: EScrollGestureDirection,
    pub scroll_gesture_direction_for_ortho_viewports: EScrollGestureDirection,
    pub b_level_editor_joystick_controls: bool,
    pub b_use_distance_scaled_camera_speed: bool,
    pub b_orbit_camera_around_selection: bool,
    pub b_use_power_of2_snap_size: bool,
    pub decimal_grid_sizes: TArray<f32>,
    pub decimal_grid_intervals: TArray<f32>,
    pub pow2_grid_sizes: TArray<f32>,
    pub pow2_grid_intervals: TArray<f32>,
    pub common_rot_grid_sizes: TArray<f32>,
    pub divisions_of360_rot_grid_sizes: TArray<f32>,
    pub scaling_grid_sizes: TArray<f32>,
    pub flags_240: u8,
    pub snap_to_surface: FSnapToSurfaceSettings,
    pub flags_256: u8,
    pub active_snap_layer_index: i32,
    pub flags_264: u8,
    pub actor_snap_scale: f32,
    pub actor_snap_distance: f32,
    pub b_snap_vertices: bool,
    pub snap_distance: f32,
    pub current_pos_grid_size: i32,
    pub current_rot_grid_size: i32,
    pub current_scaling_grid_size: i32,
    pub preserve_non_uniform_scale: bool,
    pub current_rot_grid_mode: ERotationGridMode,
    pub aspect_ratio_axis_constraint: crate::bindings::engine::EAspectRatioAxisConstraint,
    pub flags_300: u8,
    pub marquee_selection_mode: EMarqueeSelectionMode,
    pub flags_308: u8,
    pub selection_highlight_intensity: f32,
    pub bsp_selection_highlight_intensity: f32,
    pub flags_320: u8,
    pub camera_preview_size: f32,
    pub background_drop_distance: f32,
    pub preview_meshes: TArray<crate::bindings::core_u_object::FSoftObjectPath>,
    pub billboard_scale: f32,
    pub transform_widget_size_adjustment_deprecated: i32,
    pub flags_360: u8,
    pub measuring_tool_units: EMeasuringToolUnits,
    pub selected_spline_point_size_adjustment: f32,
    pub spline_line_thickness_adjustment: f32,
    pub spline_tangent_handle_size_adjustment: f32,
    pub spline_tangent_scale: f32,
    pub last_in_viewport_menu_location: crate::bindings::core_u_object::FVector2D,
    pub material_for_dropped_textures: TSoftObjectPtr<
        crate::bindings::engine::UMaterialInterface,
    >,
    pub material_params_for_dropped_textures: TMap<EMaterialKind, FName>,
    pub editor_views: TMap<
        TSoftObjectPtr<crate::bindings::engine::UWorld>,
        FLevelEditorViewporEditorViews,
    >,
    pub property_coloration_color_for_matching_objects: crate::bindings::core_u_object::FColor,
    pub per_instance_settings: TArray<FLevelEditorViewportInstanceSettingsKeyValuePair>,
}
pub struct ULevelExporterFBX {}
pub struct ULevelExporterLOD {}
pub struct ULevelExporterOBJ {}
pub struct ULevelExporterSTL {}
pub struct ULevelExporterT3D {}
pub struct ULevelFactory {}
pub struct ULinearStairBuilder {
    pub step_length: i32,
    pub step_height: i32,
    pub step_width: i32,
    pub num_steps: i32,
    pub add_to_first_step: i32,
    pub group_name: FName,
}
pub struct UListMaterialsUsedWithMeshEmittersCommandlet {}
pub struct UListStaticMeshesImportedFromSpeedTreesCommandlet {}
pub struct ULoadPackageCommandlet {}
pub struct UMaterialEditorParameters {
    pub stored_layer_previews: TArray<
        UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
    >,
    pub stored_blend_previews: TArray<
        UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
    >,
}
pub struct UMaterialEditorInstanceConstant {
    pub phys_material: UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    pub parent: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub parameter_groups: TArray<FEditorParameterGroup>,
    pub refraction_depth_bias: f32,
    pub subsurface_profile: UPtr<crate::bindings::engine::USubsurfaceProfile>,
    pub flags_128: u8,
    pub specular_profile: UPtr<crate::bindings::engine::USpecularProfile>,
    pub flags_144: u8,
    pub base_property_overrides: crate::bindings::engine::FMaterialInstanceBasePropertyOverrides,
    pub source_instance: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
    pub source_function: UPtr<crate::bindings::engine::UMaterialFunctionInstance>,
    pub visible_expressions: TArray<crate::bindings::engine::FMaterialParameterInfo>,
    pub lightmass_settings: FLightmassParameterizedMaterialSettings,
    pub flags_248: u8,
    pub nanite_override_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub enumeration_objects: TArray<
        TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
    >,
    pub post_process_overrides: FMaterialEditorPostProcessOverrides,
}
pub struct UMaterialEditorOptions {
    pub flags_48: u8,
    pub favorite_expressions: TArray<FString>,
}
pub struct UMaterialEditorPreviewParameters {
    pub parameter_groups: TArray<FEditorParameterGroup>,
    pub preview_material: UPtr<crate::bindings::engine::UMaterial>,
    pub original_function: UPtr<crate::bindings::engine::UMaterialFunction>,
    pub original_material: UPtr<crate::bindings::engine::UMaterial>,
}
pub struct UMaterialFactoryNew {
    pub initial_texture: UPtr<crate::bindings::engine::UTexture>,
}
pub struct UMaterialFunctionFactoryNew {}
pub struct UMaterialFunctionInstanceFactory {
    pub initial_parent: UPtr<crate::bindings::engine::UMaterialFunctionInterface>,
}
pub struct UMaterialFunctionMaterialLayerInstanceFactory {}
pub struct UMaterialFunctionMaterialLayerBlendInstanceFactory {}
pub struct UMaterialFunctionMaterialLayerBlendFactory {}
pub struct UMaterialFunctionMaterialLayerFactory {}
pub struct UMaterialInstanceConstantFactoryNew {
    pub initial_parent: UPtr<crate::bindings::engine::UMaterialInterface>,
}
pub struct UMaterialParameterCollectionFactoryNew {}
pub struct UMaterialStatsOptions {
    pub b_platform_used: i32,
    pub b_material_quality_used: i32,
    pub material_stats_derived_mi_option: EMaterialStatsDerivedMIOption,
}
pub struct UModelExporterT3D {}
pub struct UModelFactory {}
pub struct UNeuralProfileFactory {}
pub struct UObjectExporterT3D {}
pub struct UObjectLibraryFactory {}
pub struct UPackageFactory {}
pub struct UParticleSystemFactoryNew {}
pub struct UPersonaOptions {
    pub flags_48: u8,
    pub show_mesh_stats: i32,
    pub default_local_axes_selection: u32,
    pub default_bone_draw_selection: u32,
    pub b_show_bone_colors: bool,
    pub default_bone_color: crate::bindings::core_u_object::FLinearColor,
    pub selected_bone_color: crate::bindings::core_u_object::FLinearColor,
    pub affected_bone_color: crate::bindings::core_u_object::FLinearColor,
    pub disabled_bone_color: crate::bindings::core_u_object::FLinearColor,
    pub parent_of_selected_bone_color: crate::bindings::core_u_object::FLinearColor,
    pub virtual_bone_color: crate::bindings::core_u_object::FLinearColor,
    pub section_timing_node_color: crate::bindings::core_u_object::FLinearColor,
    pub notify_timing_node_color: crate::bindings::core_u_object::FLinearColor,
    pub branching_point_timing_node_color: crate::bindings::core_u_object::FLinearColor,
    pub b_pause_animation_on_camera_move: bool,
    pub b_use_inline_socket_editor: bool,
    pub b_flatten_skeleton_hierarchy_when_filtering: bool,
    pub b_hide_parents_when_filtering: bool,
    pub b_show_bone_indexes: bool,
    pub b_expand_tree_on_selection: bool,
    pub b_allow_preview_mesh_collections_to_select_from_different_skeletons: bool,
    pub b_allow_preview_mesh_collections_to_use_custom_anim_bp: bool,
    pub b_allow_mesh_section_selection: bool,
    pub num_folder_filters_in_asset_browser: u32,
    pub asset_editor_options: TArray<FAssetEditorOptions>,
    pub curve_editor_snap_interval: f32,
    pub timeline_scrub_snap_value: i32,
    pub timeline_display_format: crate::bindings::time_management::EFrameNumberDisplayFormats,
    pub b_timeline_display_percentage: bool,
    pub b_timeline_display_format_secondary: bool,
    pub b_timeline_display_curve_keys: bool,
    pub timeline_enabled_snaps: TArray<FName>,
    pub b_allow_incompatible_skeleton_selection: bool,
    pub b_use_tree_view_for_animation_curves: bool,
    pub animation_curve_grouping_delimiters: FString,
}
pub struct UPhysicalMaterialFactoryNew {
    pub physical_material_class: TSubclassOf<
        crate::bindings::physics_core::UPhysicalMaterial,
    >,
}
pub struct UPhysicalMaterialMaskFactory {
    pub physical_material_mask_class: TSubclassOf<
        crate::bindings::engine::UPhysicalMaterialMask,
    >,
}
pub struct UPhysicsAssetEditorOptions {
    pub physics_blend: f32,
    pub b_update_joints_from_animation: bool,
    pub physics_update_mode: crate::bindings::engine::EPhysicsTransformUpdateMode,
    pub poke_pause_time: f32,
    pub poke_blend_time: f32,
    pub grav_scale: f32,
    pub gravity_override_z: f32,
    pub b_use_gravity_override: bool,
    pub max_fps: i32,
    pub handle_linear_damping: f32,
    pub handle_linear_stiffness: f32,
    pub handle_angular_damping: f32,
    pub handle_angular_stiffness: f32,
    pub interpolation_speed: f32,
    pub poke_strength: f32,
    pub interaction_distance: f32,
    pub flags_108: u8,
    pub com_render_size: f32,
    pub constraint_draw_size: f32,
    pub mesh_view_mode: EPhysicsAssetEditorMeshViewMode,
    pub collision_view_mode: EPhysicsAssetEditorCollisionViewMode,
    pub center_of_mass_view_mode: EPhysicsAssetEditorCenterOfMassViewMode,
    pub constraint_view_mode: EPhysicsAssetEditorConstraintViewMode,
    pub simulation_mesh_view_mode: EPhysicsAssetEditorMeshViewMode,
    pub simulation_collision_view_mode: EPhysicsAssetEditorCollisionViewMode,
    pub simulation_center_of_mass_view_mode: EPhysicsAssetEditorCenterOfMassViewMode,
    pub simulation_constraint_view_mode: EPhysicsAssetEditorConstraintViewMode,
    pub collision_opacity: f32,
    pub b_solid_rendering_for_selected_only: bool,
    pub b_hide_simulated_bodies: bool,
    pub b_hide_kinematic_bodies: bool,
    pub b_hide_body_mass: bool,
    pub b_create_constraints_when_creating_bodies_from_skeleton_tree: bool,
    pub b_reset_cloth_when_simulating: bool,
    pub b_expose_legacy_menu_simulation_controls: bool,
    pub b_expose_legacy_menu_constraint_controls: bool,
}
pub struct UPhysicsAssetGenerationSettings {
    pub create_params: crate::bindings::physics_utilities::FPhysAssetCreateParams,
}
pub struct UPkgInfoCommandlet {}
pub struct UPolysExporterOBJ {}
pub struct UPolysExporterT3D {}
pub struct UPolysFactory {}
pub struct UPreviewMeshCollectionFactory {
    pub current_skeleton: TWeakObjectPtr<crate::bindings::engine::USkeleton>,
}
pub struct UPropertyColorSettings {
    pub custom_properties: TArray<FPropertyColorCustomProperty>,
}
pub struct UCSVImportFactory {
    pub automated_import_settings: FCSVImportSettings,
    pub data_table_import_options: UPtr<crate::bindings::engine::UDataTable>,
}
pub struct UReimportCurveFactory {}
pub struct UReimportCurveTableFactory {}
pub struct UReimportDataTableFactory {}
pub struct UFbxFactory {
    pub import_ui: UPtr<UFbxImportUI>,
    pub original_import_ui: UPtr<UFbxImportUI>,
}
pub struct UReimportFbxAnimSequenceFactory {}
pub struct UReimportFbxSkeletalMeshFactory {}
pub struct UReimportFbxStaticMeshFactory {}
pub struct UTextureFactory {
    pub flags_144: u8,
    pub compression_settings: crate::bindings::engine::TextureCompressionSettings,
    pub flags_152: u8,
    pub blending: crate::bindings::engine::EBlendMode,
    pub shading_model: crate::bindings::engine::EMaterialShadingModel,
    pub mip_gen_settings: crate::bindings::engine::TextureMipGenSettings,
    pub lod_group: crate::bindings::engine::TextureGroup,
    pub b_do_scale_mips_for_alpha_coverage: bool,
    pub b_use_new_mip_filter: bool,
    pub alpha_coverage_thresholds: crate::bindings::core_u_object::FVector4,
    pub flags_208: u8,
    pub udim_regex_pattern: FString,
    pub color_space_mode: ETextureSourceColorSpace,
    pub hdr_import_should_be_long_lat_cube_map: crate::bindings::core_u_object::EAppReturnType,
}
pub struct UReimportTextureFactory {
    pub p_original_tex: UPtr<crate::bindings::engine::UTexture>,
}
pub struct UVectorFieldStaticFactory {}
pub struct UReimportVectorFieldStaticFactory {}
pub struct URenderTargetExporterPNG {}
pub struct URenderTargetExporterEXR {}
pub struct UReplaceActorCommandlet {}
pub struct UResavePackagesCommandlet {}
pub struct USceneImportFactory {}
pub struct UEditorViewportViewMenuContext {}
pub struct USequenceExporterT3D {}
pub struct USheetBuilder {
    pub x: i32,
    pub y: i32,
    pub x_segments: i32,
    pub y_segments: i32,
    pub axis: ESheetAxis,
    pub group_name: FName,
}
pub struct USkeletalMeshEditorSettings {
    pub anim_preview_floor_color: crate::bindings::core_u_object::FColor,
    pub anim_preview_sky_color: crate::bindings::core_u_object::FColor,
    pub anim_preview_sky_brightness: f32,
    pub anim_preview_light_brightness: f32,
    pub anim_preview_lighting_direction: crate::bindings::core_u_object::FRotator,
    pub anim_preview_directional_color: crate::bindings::core_u_object::FColor,
}
pub struct USkeletalMeshExporterFBX {}
pub struct USkeletalMeshToolMenuContext {}
pub struct USoundExporterOGG {}
pub struct USoundExporterWAV {}
pub struct USoundSurroundExporterWAV {}
pub struct USpecularProfileFactory {}
pub struct USpiralStairBuilder {
    pub inner_radius: i32,
    pub step_width: i32,
    pub step_height: i32,
    pub step_thickness: i32,
    pub num_steps_per360: i32,
    pub num_steps: i32,
    pub group_name: FName,
    pub flags_172: u8,
}
pub struct UStaticMeshExporterFBX {}
pub struct UStaticMeshExporterOBJ {}
pub struct UStaticMeshMinLodCommandlet {}
pub struct UStringTableFactory {}
pub struct UStructureFactory {}
pub struct UStructViewerSettings {
    pub display_internal_structs: bool,
    pub developer_folder_type: EStructViewerDeveloperType,
}
pub struct USubsurfaceProfileFactory {}
pub struct USubUVAnimationFactory {
    pub initial_texture: UPtr<crate::bindings::engine::UTexture2D>,
}
pub struct UTetrahedronBuilder {
    pub radius: f32,
    pub sphere_extrapolation: i32,
    pub group_name: FName,
}
pub struct UTexAligner {
    pub def_tex_align: ETexAlign,
    pub t_axis: u8,
    pub u_tile: f32,
    pub v_tile: f32,
    pub desc: FString,
}
pub struct UTexAlignerBox {}
pub struct UTexAlignerDefault {}
pub struct UTexAlignerFit {}
pub struct UTexAlignerPlanar {}
pub struct UTextBufferExporterTXT {}
pub struct UTextureThumbnailRenderer {}
pub struct UTexture2DArrayThumbnailRenderer {}
pub struct UTexture2DFactoryNew {
    pub width: i32,
    pub height: i32,
}
pub struct UTextureCubeExporterHDR {}
pub struct URenderTargetCubeExporterHDR {}
pub struct UTextureExporterGeneric {}
pub struct UTextureExporterBMP {}
pub struct UVirtualTextureBuilderExporterBMP {}
pub struct UTextureExporterDDS {}
pub struct UVirtualTextureBuilderExporterDDS {}
pub struct UTextureExporterEXR {}
pub struct UVirtualTextureBuilderExporterEXR {}
pub struct UTextureExporterHDR {}
pub struct UVirtualTextureBuilderExporterHDR {}
pub struct UTextureExporterPNG {}
pub struct UVirtualTextureBuilderExporterPNG {}
pub struct UTextureExporterJPEG {}
pub struct UTextureExporterUEJPEG {}
pub struct UTextureExporterTGA {}
pub struct UUDIMTextureFunctionLibrary {}
pub struct UTextureRenderTarget2DArrayFactoryNew {
    pub width: i32,
    pub height: i32,
    pub slices: i32,
    pub format: u8,
}
pub struct UTextureRenderTargetCubeFactoryNew {
    pub width: i32,
    pub format: u8,
}
pub struct UTextureRenderTargetFactoryNew {
    pub width: i32,
    pub height: i32,
    pub format: u8,
}
pub struct UTextureRenderTargetVolumeFactoryNew {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub format: u8,
}
pub struct UTouchInterfaceFactory {}
pub struct UTransactor {}
pub struct UTransBuffer {}
pub struct UTrueTypeFontFactory {
    pub import_options: UPtr<crate::bindings::engine::UFontImportOptions>,
    pub b_properties_configured: bool,
    pub b_font_selected: bool,
}
pub struct UUnrealEdKeyBindings {
    pub key_bindings: TArray<FEditorKeyBinding>,
}
pub struct UUnrealEdOptions {
    pub editor_categories: TArray<FEditorCommandCategory>,
    pub editor_commands: TArray<FEditorCommand>,
    pub editor_key_bindings: UPtr<UUnrealEdKeyBindings>,
    pub b_expand_class_picker_class_list: bool,
    pub new_asset_default_classes: TArray<FClassPickerDefaults>,
}
pub struct UVectorFieldExporter {}
pub struct UViewportToolBarContext {}
pub struct UVolumetricBuilder {
    pub z: f32,
    pub radius: f32,
    pub num_sheets: i32,
    pub group_name: FName,
}
pub struct UWorldPartitionBuildNavigationOptions {
    pub b_verbose: bool,
    pub b_clean_packages: bool,
}
pub struct UDataLayerConversionInfo {
    pub data_layer_to_convert: UPtr<
        crate::bindings::engine::UDeprecatedDataLayerInstance,
    >,
    pub data_layer_asset: UPtr<crate::bindings::engine::UDataLayerAsset>,
    pub data_layer_instance: UPtr<crate::bindings::engine::UDataLayerInstanceWithAsset>,
    pub previous_conversions_info: TArray<TWeakObjectPtr<UDataLayerConversionInfo>>,
    pub current_converting_info: TWeakObjectPtr<UDataLayerConversionInfo>,
}
pub struct UDataLayerToAssetCommandletContext {
    pub data_layer_conversion_info: TArray<UPtr<UDataLayerConversionInfo>>,
    pub converting_data_layer_info: TArray<TWeakObjectPtr<UDataLayerConversionInfo>>,
}
pub struct UDataLayerToAssetCommandlet {
    pub destination_folder: FString,
    pub conversion_folder: FString,
    pub b_perform_save_packages: bool,
    pub b_ignore_actor_loading_errors: bool,
    pub data_layer_factory: UPtr<crate::bindings::data_layer_editor::UDataLayerFactory>,
    pub main_world: UPtr<crate::bindings::engine::UWorld>,
}
pub struct UWorldPartitionBuilder {}
pub struct UWorldPartitionLandscapeBuilder {}
pub struct UWrangleContentCommandlet {}
pub struct UActorGroupingUtils {}
pub struct UAnalyticsPrivacySettings {
    pub b_send_usage_data: bool,
    pub b_suppress_industry_popup: bool,
}
pub struct UCrashReportsPrivacySettings {
    pub b_send_unattended_bug_reports: bool,
}
pub struct UAnimSeqExportOption {
    pub b_export_transforms: bool,
    pub b_export_morph_targets: bool,
    pub b_export_attribute_curves: bool,
    pub b_export_material_curves: bool,
    pub b_record_in_world_space: bool,
    pub b_evaluate_all_skeletal_mesh_components: bool,
    pub interpolation: crate::bindings::engine::EAnimInterpolationType,
    pub curve_interpolation: crate::bindings::engine::ERichCurveInterpMode,
    pub include_animation_names: TArray<FString>,
    pub exclude_animation_names: TArray<FString>,
    pub warm_up_frames: crate::bindings::core_u_object::FFrameNumber,
    pub delay_before_start: crate::bindings::core_u_object::FFrameNumber,
    pub b_transact_recording: bool,
    pub b_bake_timecode: bool,
    pub b_timecode_rate_override: bool,
    pub override_timecode_rate: crate::bindings::core_u_object::FFrameRate,
    pub b_use_custom_time_range: bool,
    pub custom_start_frame: crate::bindings::core_u_object::FFrameNumber,
    pub custom_end_frame: crate::bindings::core_u_object::FFrameNumber,
    pub custom_display_rate: crate::bindings::core_u_object::FFrameRate,
    pub b_use_custom_frame_rate: bool,
    pub custom_frame_rate: crate::bindings::core_u_object::FFrameRate,
}
pub struct UDebugSkelMeshComponent {
    pub skeleton_draw_mode: ESkeletonDrawMode,
    pub flags_4676: u8,
    pub flags_4677: u8,
    pub flags_4678: u8,
    pub b_display_bound: bool,
    pub b_display_vertex_colors: bool,
    pub wireframe_mesh_overlay_color: crate::bindings::core_u_object::FLinearColor,
    pub requested_process_root_motion_mode: EProcessRootMotionMode,
    pub process_root_motion_mode: EProcessRootMotionMode,
    pub visualize_root_motion_mode: EVisualizeRootMotionMode,
    pub consume_root_motion_previous_playback_time: f32,
    pub consume_root_motion_previous_additive_anim_type: crate::bindings::engine::EAdditiveAnimationType,
    pub flags_4712: u8,
    pub min_cloth_property_view: f32,
    pub max_cloth_property_view: f32,
    pub cloth_mesh_opacity: f32,
    pub b_cloth_flip_normal: bool,
    pub b_cloth_cull_backface: bool,
    pub flags_4732: u8,
    pub bones_of_interest: TArray<i32>,
    pub morph_target_of_interests: TArray<UPtr<crate::bindings::engine::UMorphTarget>>,
    pub skel_materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub preview_instance: UPtr<crate::bindings::anim_graph::UAnimPreviewInstance>,
    pub saved_anim_script_instance: UPtr<crate::bindings::engine::UAnimInstance>,
    pub b_is_using_in_game_bounds: bool,
    pub b_is_using_pre_skinned_bounds: bool,
    pub b_perform_single_clothing_tick: bool,
    pub b_pause_clothing_simulation_with_anim: bool,
    pub b_track_attached_instance_lod: bool,
}
pub struct UEditorAnimBaseObj {}
pub struct UEditorAnimCompositeSegment {
    pub anim_segment: crate::bindings::engine::FAnimSegment,
}
pub struct UEditorAnimCurveBoneLinks {
    pub curve_name: FName,
    pub connected_bones: TArray<crate::bindings::engine::FBoneReference>,
    pub max_lod: u8,
}
pub struct UEditorAnimSegment {
    pub anim_segment: crate::bindings::engine::FAnimSegment,
}
pub struct UEditorCompositeSection {
    pub composite_section: crate::bindings::engine::FCompositeSection,
}
pub struct UEditorNotifyObject {
    pub event: crate::bindings::engine::FAnimNotifyEvent,
}
pub struct UEditorParentPlayerListObj {
    pub overrides: TArray<crate::bindings::engine::FAnimParentNodeAssetOverride>,
}
pub struct UEditorSkeletonNotifyObj {
    pub name: FName,
}
pub struct UAssetGuideline {
    pub plugins: TArray<FString>,
    pub project_settings: TArray<FIniStringValue>,
    pub guideline_name: FName,
}
pub struct UAssetImportTask {
    pub filename: FString,
    pub destination_path: FString,
    pub destination_name: FString,
    pub b_replace_existing: bool,
    pub b_replace_existing_settings: bool,
    pub b_automated: bool,
    pub b_save: bool,
    pub b_async: bool,
    pub factory: UPtr<UFactory>,
    pub options: UPtr<crate::bindings::core_u_object::UObject>,
    pub imported_object_paths: TArray<FString>,
    pub result: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
pub struct UAutomatedAssetImportData {
    pub group_name: FString,
    pub filenames: TArray<FString>,
    pub destination_path: FString,
    pub factory_name: FString,
    pub b_replace_existing: bool,
    pub b_skip_read_only: bool,
    pub factory: UPtr<UFactory>,
    pub level_to_load: FString,
}
pub struct UAutoReimportManager {}
pub struct UAssetRegUtilCommandlet {}
pub struct UAssetRegistryDumpCommandlet {}
pub struct UAssetSizeQueryCommandlet {}
pub struct UAudioMixerCommandlet {}
pub struct UBaseIteratePackagesCommandlet {}
pub struct UChunkDependencyInfo {
    pub dependency_array: TArray<FChunkDependency>,
}
pub struct UCompileAllBlueprintsCommandlet {}
pub struct UCompileShadersTestBedCommandlet {}
pub struct UConvertLevelsToExternalActorsCommandlet {}
pub struct UCookCommandlet {}
pub struct UCookShadersCommandlet {}
pub struct UCopyNaniteFallbackSettingsToRayTracingProxyCommandlet {}
pub struct UDDCCleanupCommandlet {}
pub struct UDebugShaderCompileJobCommandlet {}
pub struct UDerivedDataCacheCommandlet {}
pub struct UDetectOrphanedLocalizedAssetsCommandlet {}
pub struct UDiffAssetRegistriesCommandlet {
    pub asset_registry_search_path: TArray<FString>,
    pub p4_repository: FString,
    pub p4_engine_base_path: FString,
    pub p4_engine_asset_path: FString,
    pub p4_game_base_path: FString,
    pub p4_game_asset_path: FString,
    pub regex_branch_cl: FString,
}
pub struct UDiffAssetsCommandlet {}
pub struct UDiffCookCommandlet {}
pub struct UDiffFilesCommandlet {}
pub struct UDumpAssetRegistryCommandlet {}
pub struct UDumpBlueprintsInfoCommandlet {}
pub struct UDumpHiddenCategoriesCommandlet {}
pub struct UDumpLightFunctionMaterialInfoCommandlet {}
pub struct UDumpMaterialExpressionInfoCommandlet {}
pub struct UDumpMaterialExpressionsCommandlet {}
pub struct UDumpMaterialInfoCommandlet {}
pub struct UDumpMaterialShaderTypesCommandlet {}
pub struct UGatherTextCommandletBase {}
pub struct UExportDialogueScriptCommandlet {}
pub struct UExportPakDependenciesCommandlet {}
pub struct UExternalActorsCommandlet {
    pub b_repair: bool,
    pub b_disable: bool,
    pub b_force: bool,
    pub b_report: bool,
    pub b_list_maps: bool,
    pub map_list_file: FString,
    pub dump_csv_file: FString,
}
pub struct UExtractLocResCommandlet {}
pub struct UFileServerCommandlet {}
pub struct UFixConflictingLocalizationKeysCommandlet {}
pub struct UFixupNeedsLoadForEditorGameCommandlet {}
pub struct UGatherTextCommandlet {
    pub gather_text_phases: FGatherTextCommandletPhase,
}
pub struct UGatherTextFromAssetsCommandlet {}
pub struct UGatherTextFromAssetsWorkerCommandlet {}
pub struct UGatherTextFromMetaDataCommandlet {}
pub struct UGatherTextFromSourceCommandlet {}
pub struct UGenerateAssetManifestCommandlet {}
pub struct UGenerateBlueprintAPICommandlet {}
pub struct UGenerateDistillFileSetsCommandlet {}
pub struct UGenerateGatherArchiveCommandlet {}
pub struct UGenerateGatherManifestCommandlet {}
pub struct UGenerateTextLocalizationReportCommandlet {}
pub struct UGenerateTextLocalizationResourceCommandlet {}
pub struct UImportAssetsCommandlet {
    pub global_import_data: UPtr<UAutomatedAssetImportData>,
    pub import_data_list: TArray<UPtr<UAutomatedAssetImportData>>,
}
pub struct UImportDialogueScriptCommandlet {}
pub struct UImportLocalizedDialogueCommandlet {}
pub struct UInternationalizationConditioningCommandlet {}
pub struct UInternationalizationExportCommandlet {}
pub struct UIoStoreCommandlet {}
pub struct UMakeBinaryConfigCommandlet {}
pub struct UMaterialLayerUsageCommandlet {}
pub struct UMergeShaderPipelineCachesCommandlet {}
pub struct UParticleSystemAuditCommandlet {
    pub high_spawn_rate_or_burst_threshold: f32,
    pub far_lod_distance_theshold: f32,
}
pub struct UPopulateDialogueWaveFromCharacterSheetCommandlet {}
pub struct UReplaceAssetsCommandlet {}
pub struct USavePackageUtilitiesCommandlet {}
pub struct UShaderCodeLibraryToolsCommandlet {}
pub struct UShaderPipelineCacheToolsCommandlet {}
pub struct UStabilizeLocalizationKeysCommandlet {}
pub struct USubstrateCommandlet {}
pub struct USummarizeTraceCommandlet {}
pub struct USummarizeTraceEditorWorkflowsCommandlet {}
pub struct USwapSoundForDialogueInCuesCommandlet {}
pub struct UTextAssetCommandlet {}
pub struct UUnrealPakCommandlet {}
pub struct UUpdateGameProjectCommandlet {}
pub struct UWorldPartitionBuilderCommandlet {}
pub struct UWorldPartitionConvertCommandlet {
    pub editor_hash_class: TSubclassOf<
        crate::bindings::engine::UWorldPartitionEditorHash,
    >,
    pub runtime_hash_class: TSubclassOf<
        crate::bindings::engine::UWorldPartitionRuntimeHash,
    >,
    pub excluded_levels: TArray<FString>,
    pub b_convert_actors_not_referenced_by_level_script: bool,
    pub world_origin: crate::bindings::core_u_object::FVector,
    pub world_extent: crate::bindings::core_u_object::FVector,
    pub default_hlod_layer_asset: crate::bindings::core_u_object::FTopLevelAssetPath,
    pub foliage_type_path: FString,
    pub hlod_layers_for_actor_classes: TArray<FHLODLayerActorMapping>,
    pub landscape_grid_size: u32,
    pub data_layer_asset_folder: FString,
    pub data_layer_factory: UPtr<crate::bindings::data_layer_editor::UDataLayerFactory>,
}
pub struct UCookFunctionLibrary {}
pub struct UEdGraphNode_Comment {
    pub comment_color: crate::bindings::core_u_object::FLinearColor,
    pub font_size: i32,
    pub flags_212: u8,
    pub move_mode: ECommentBoxMode,
    pub node_details: FText,
    pub comment_depth: i32,
}
pub struct UActorEditorContextActorFolderState {
    pub folder_path: FName,
}
pub struct UEditorDomainSaveCommandlet {}
pub struct UEditorEngine {
    pub temp_model: UPtr<crate::bindings::engine::UModel>,
    pub conversion_temp_model: UPtr<crate::bindings::engine::UModel>,
    pub trans: UPtr<UTransactor>,
    pub bad: UPtr<crate::bindings::engine::UTexture2D>,
    pub editor_font: UPtr<crate::bindings::engine::UFont>,
    pub preview_sound_cue: UPtr<crate::bindings::engine::USoundCue>,
    pub preview_audio_component: UPtr<crate::bindings::engine::UAudioComponent>,
    pub editor_cube: UPtr<crate::bindings::engine::UStaticMesh>,
    pub editor_sphere: UPtr<crate::bindings::engine::UStaticMesh>,
    pub editor_plane: UPtr<crate::bindings::engine::UStaticMesh>,
    pub editor_cylinder: UPtr<crate::bindings::engine::UStaticMesh>,
    pub flags_6920: u8,
    pub click_flags: u32,
    pub parent_context: UPtr<crate::bindings::core_u_object::UPackage>,
    pub unsnapped_click_location: crate::bindings::core_u_object::FVector,
    pub click_location: crate::bindings::core_u_object::FVector,
    pub click_plane: crate::bindings::core_u_object::FPlane,
    pub mouse_movement: crate::bindings::core_u_object::FVector,
    pub detail_mode: crate::bindings::engine::EDetailMode,
    pub flags_7052: u8,
    pub game_command_line: FString,
    pub flags_7072: u8,
    pub height_map_export_class_name: FString,
    pub actor_factories: TArray<UPtr<UActorFactory>>,
    pub user_opened_file: FString,
    pub in_editor_game_url_options: FString,
    pub play_world: UPtr<crate::bindings::engine::UWorld>,
    pub flags_7152: u8,
    pub play_from_here_player_start_class: TSubclassOf<
        crate::bindings::engine::ANavigationObjectBase,
    >,
    pub editor_world: UPtr<crate::bindings::engine::UWorld>,
    pub actors_that_were_selected: TArray<
        TWeakObjectPtr<crate::bindings::engine::AActor>,
    >,
    pub play_world_destination: i32,
    pub current_play_world_destination: i32,
    pub flags_7200: u8,
    pub build_play_device: i32,
    pub user_edited_play_world_url: FString,
    pub scratch_render_target2048: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
    pub scratch_render_target1024: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
    pub scratch_render_target512: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
    pub scratch_render_target256: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
    pub preview_mesh_comp: UPtr<crate::bindings::engine::UStaticMeshComponent>,
    pub preview_mesh_index: i32,
    pub flags_7348: u8,
    pub custom_camera_align_emitter_distance: f32,
    pub flags_7356: u8,
    pub brush_builders: TArray<UPtr<crate::bindings::engine::UBrushBuilder>>,
    pub editor_world_extensions_manager: UPtr<UEditorWorldExtensionManager>,
    pub drag_drop_handler: UPtr<ULevelEditorDragDropHandler>,
    pub actor_grouping_utils_class_name: crate::bindings::core_u_object::FSoftClassPath,
    pub actor_grouping_utils: UPtr<UActorGroupingUtils>,
    pub flags_10016: u8,
}
pub struct UEditorLevelUtils {}
pub struct UEditorPerformanceSettings {
    pub b_enable_editor_peformance_tool: bool,
    pub b_enable_notifications: bool,
    pub notify_list: TArray<FName>,
    pub notification_list_deprecated: TArray<FName>,
    pub b_enable_telemetry: bool,
    pub b_enable_snapshots: bool,
    pub b_show_warnings_only: bool,
    pub b_enable_experimental_features: bool,
    pub flags_148: u8,
    pub flags_149: u8,
    pub realtime_screen_percentage_mode: EEditorUserScreenPercentageModeOverride,
    pub mobile_screen_percentage_mode: EEditorUserScreenPercentageModeOverride,
    pub vr_screen_percentage_mode: EEditorUserScreenPercentageModeOverride,
    pub path_tracer_screen_percentage_mode: EEditorUserScreenPercentageModeOverride,
    pub non_realtime_screen_percentage_mode: EEditorUserScreenPercentageModeOverride,
    pub flags_172: u8,
    pub manual_screen_percentage: f32,
    pub flags_180: u8,
    pub min_viewport_rendering_resolution: i32,
    pub flags_188: u8,
    pub max_viewport_rendering_resolution: i32,
}
pub struct UActorEditorContextEditorState {
    pub actor_editor_context_state_collection: UPtr<UActorEditorContextStateCollection>,
    pub b_apply_actor_editor_context_on_load: bool,
}
pub struct UEditorStateSubsystem {}
pub struct UWorldEditorState {
    pub world: TSoftObjectPtr<crate::bindings::engine::UWorld>,
}
pub struct UEditorWorldExtensionCollection {}
pub struct UEditorWorldExtensionManager {
    pub editor_world_extension_collection: TArray<UPtr<UEditorWorldExtensionCollection>>,
}
pub struct UActorElementDetailsInterface {}
pub struct UActorElementEditorAssetDataInterface {}
pub struct UActorElementsCopy {
    pub actors_to_copy: TArray<UPtr<crate::bindings::engine::AActor>>,
}
pub struct UActorElementsExporterT3D {}
pub struct UActorElementEditorSelectionInterface {}
pub struct UActorElementEditorWorldInterface {}
pub struct UComponentElementDetailsInterface {}
pub struct UComponentElementsCopy {
    pub components_to_copy: TArray<UPtr<crate::bindings::engine::UActorComponent>>,
}
pub struct UComponentElementsExporterT3D {}
pub struct UComponentElementEditorSelectionInterface {}
pub struct UComponentElementEditorWorldInterface {}
pub struct UObjectElementDetailsInterface {}
pub struct UObjectElementEditorSelectionInterface {}
pub struct USMInstanceElementDetailsInterface {}
pub struct USMInstanceElementDetailsProxyObject {
    pub transform: crate::bindings::core_u_object::FTransform,
}
pub struct USMInstanceElementEditorSelectionInterface {}
pub struct USMInstanceElementEditorWorldInterface {}
pub struct UActorFactoryPointLight {}
pub struct UActorFactoryRectLight {}
pub struct UActorFactorySpotLight {}
pub struct UAnimBankFactory {
    pub target_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub preview_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
}
pub struct UTransformProviderDataFactory {
    pub provider_data_class: TSubclassOf<
        crate::bindings::engine::UTransformProviderData,
    >,
}
pub struct UAnimBankDataFactory {}
pub struct UAnimBlueprintFactory {
    pub blueprint_type: crate::bindings::engine::EBlueprintType,
    pub parent_class: TSubclassOf<crate::bindings::engine::UAnimInstance>,
    pub target_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub preview_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub b_template: bool,
}
pub struct UAnimLayerInterfaceFactory {}
pub struct UAnimBoneCompressionSettingsFactory {}
pub struct UAnimCompositeFactory {
    pub target_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub source_animation: UPtr<crate::bindings::engine::UAnimSequence>,
    pub preview_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
}
pub struct UAnimCurveCompressionSettingsFactory {}
pub struct UAnimMontageFactory {
    pub target_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub source_animation: UPtr<crate::bindings::engine::UAnimSequence>,
    pub preview_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
}
pub struct UAnimSequenceFactory {
    pub target_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub preview_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
}
pub struct UAnimStreamableFactory {
    pub target_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub source_animation: UPtr<crate::bindings::engine::UAnimSequence>,
}
pub struct UCompositeCurveTableFactory {}
pub struct UDataTableFactory {
    pub _struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
}
pub struct UCompositeDataTableFactory {}
pub struct UCurveTableFactory {}
pub struct UEditorStaticMeshFactory {}
pub struct UInterchangeReimportHandler {}
pub struct ULightWeightInstanceFactory {
    pub parent_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
pub struct UMaterialImportHelpers {}
pub struct UMeshDeformerCollectionFactory {}
pub struct UMirrorTableFindReplaceExpressions {
    pub find_replace_expressions: TArray<
        crate::bindings::engine::FMirrorFindReplaceExpression,
    >,
}
pub struct UMirrorDataTableFactory {
    pub _struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub mirror_find_replace_expressions: UPtr<UMirrorTableFindReplaceExpressions>,
}
pub struct UPackFactory {}
pub struct UPhysicsAssetFactory {
    pub target_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
}
pub struct UPoseAssetFactory {
    pub source_animation: UPtr<crate::bindings::engine::UAnimSequence>,
    pub pose_names: TArray<FString>,
    pub target_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub preview_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
}
pub struct USkeletonFactory {
    pub target_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
}
pub struct USlateBrushAssetFactory {
    pub initial_texture: UPtr<crate::bindings::engine::UTexture2D>,
}
pub struct USlateWidgetStyleAssetFactory {
    pub style_type: TSubclassOf<
        crate::bindings::slate_core::USlateWidgetStyleContainerBase,
    >,
}
pub struct USparseVolumeTextureMaterialFactoryNew {
    pub initial_texture: UPtr<crate::bindings::engine::USparseVolumeTexture>,
}
pub struct USparseVolumeTextureMaterialInstanceFactoryNew {
    pub initial_texture: UPtr<crate::bindings::engine::USparseVolumeTexture>,
    pub default_svt_material: TSoftObjectPtr<
        crate::bindings::engine::UMaterialInterface,
    >,
}
pub struct UTexture2DArrayFactory {
    pub initial_textures: TArray<UPtr<crate::bindings::engine::UTexture2D>>,
}
pub struct UTextureCubeArrayFactory {
    pub initial_textures: TArray<UPtr<crate::bindings::engine::UTextureCube>>,
}
pub struct UVariableFrameStrippingSettingsFactory {}
pub struct UVolumeTextureFactory {
    pub initial_texture: UPtr<crate::bindings::engine::UTexture2D>,
}
pub struct UWorldFactory {}
pub struct UFbxAssetImportData {
    pub import_translation: crate::bindings::core_u_object::FVector,
    pub import_rotation: crate::bindings::core_u_object::FRotator,
    pub import_uniform_scale: f32,
    pub coordinate_system_policy: ECoordinateSystemPolicy,
    pub b_convert_scene: bool,
    pub b_force_front_x_axis: bool,
    pub b_convert_scene_unit: bool,
    pub b_import_as_scene: bool,
    pub fbx_scene_import_data_reference: UPtr<UFbxSceneImportData>,
    pub b_using_luf_coordinate_sysem: bool,
}
pub struct UFbxAnimSequenceImportData {
    pub b_import_meshes_in_bone_hierarchy: bool,
    pub animation_length: EFBXAnimationLengthImportType,
    pub start_frame_deprecated: i32,
    pub end_frame_deprecated: i32,
    pub frame_import_range: crate::bindings::core_u_object::FInt32Interval,
    pub b_use_default_sample_rate: bool,
    pub custom_sample_rate: i32,
    pub b_snap_to_closest_frame_boundary: bool,
    pub source_animation_name: FString,
    pub b_import_custom_attribute: bool,
    pub b_delete_existing_custom_attribute_curves: bool,
    pub b_delete_existing_non_curve_custom_attributes: bool,
    pub b_import_bone_tracks: bool,
    pub b_set_material_drive_parameter_on_custom_attribute: bool,
    pub b_add_curve_metadata_to_skeleton: bool,
    pub material_curve_suffixes: TArray<FString>,
    pub b_remove_redundant_keys: bool,
    pub b_delete_existing_morph_target_curves: bool,
    pub b_do_not_import_curve_with_zero: bool,
    pub b_preserve_local_transform: bool,
}
pub struct UFbxExportOption {
    pub fbx_export_compatibility: EFbxExportCompatibility,
    pub flags_52: u8,
    pub flags_53: u8,
    pub bake_camera_and_light_animation: EMovieSceneBakeType,
    pub bake_actor_animation: EMovieSceneBakeType,
    pub bake_material_inputs: EFbxMaterialBakeMode,
    pub default_material_bake_size: FFbxMaterialBakeSize,
}
pub struct UFbxMeshImportData {
    pub b_transform_vertex_to_absolute: bool,
    pub b_bake_pivot_in_vertex: bool,
    pub flags_180: u8,
    pub normal_import_method: EFBXNormalImportMethod,
    pub normal_generation_method: EFBXNormalGenerationMethod,
    pub flags_188: u8,
    pub b_reorder_material_to_fbx_order: bool,
    pub import_material_original_name_data: TArray<FName>,
    pub import_mesh_lod_data: TArray<FImportMeshLodSectionsData>,
}
pub struct UFbxSceneImportData {
    pub source_fbx_file: FString,
}
pub struct UFbxSceneImportFactory {
    pub scene_import_options: UPtr<UFbxSceneImportOptions>,
    pub scene_import_options_static_mesh: UPtr<UFbxSceneImportOptionsStaticMesh>,
    pub scene_import_options_skeletal_mesh: UPtr<UFbxSceneImportOptionsSkeletalMesh>,
    pub static_mesh_import_data: UPtr<UFbxStaticMeshImportData>,
    pub skeletal_mesh_import_data: UPtr<UFbxSkeletalMeshImportData>,
    pub anim_sequence_import_data: UPtr<UFbxAnimSequenceImportData>,
    pub texture_import_data: UPtr<UFbxTextureImportData>,
    pub reimport_data: UPtr<UFbxSceneImportData>,
}
pub struct UFbxSceneImportOptions {
    pub flags_48: u8,
    pub hierarchy_type: EFBXSceneOptionsCreateHierarchyType,
    pub flags_56: u8,
    pub import_translation: crate::bindings::core_u_object::FVector,
    pub import_rotation: crate::bindings::core_u_object::FRotator,
    pub import_uniform_scale: f32,
    pub b_transform_vertex_to_absolute: bool,
    pub b_bake_pivot_in_vertex: bool,
    pub flags_120: u8,
}
pub struct UFbxSceneImportOptionsSkeletalMesh {
    pub flags_48: u8,
    pub threshold_position: f32,
    pub threshold_tangent_normal: f32,
    pub threshold_uv: f32,
    pub morph_threshold_position: f32,
    pub flags_68: u8,
    pub animation_length: EFBXAnimationLengthImportType,
    pub frame_import_range: crate::bindings::core_u_object::FInt32Interval,
    pub b_use_default_sample_rate: bool,
    pub custom_sample_rate: i32,
    pub b_snap_to_closest_frame_boundary: bool,
    pub b_import_custom_attribute: bool,
    pub b_delete_existing_custom_attribute_curves: bool,
    pub b_delete_existing_non_curve_custom_attributes: bool,
    pub b_preserve_local_transform: bool,
    pub b_delete_existing_morph_target_curves: bool,
}
pub struct UFbxSceneImportOptionsStaticMesh {
    pub static_mesh_lod_group: FName,
    pub flags_60: u8,
    pub vertex_color_import_option: EFbxSceneVertexColorImportOption,
    pub vertex_override_color: crate::bindings::core_u_object::FColor,
    pub flags_72: u8,
    pub normal_import_method: EFBXSceneNormalImportMethod,
    pub normal_generation_method: EFBXSceneNormalGenerationMethod,
}
pub struct UFbxSkeletalMeshImportData {
    pub import_content_type: EFBXImportContentType,
    pub last_import_content_type: EFBXImportContentType,
    pub vertex_color_import_option: EVertexColorImportOption,
    pub vertex_override_color: crate::bindings::core_u_object::FColor,
    pub flags_240: u8,
    pub threshold_position: f32,
    pub threshold_tangent_normal: f32,
    pub threshold_uv: f32,
    pub morph_threshold_position: f32,
}
pub struct UFbxStaticMeshImportData {
    pub static_mesh_lod_group: FName,
    pub vertex_color_import_option: EVertexColorImportOption,
    pub vertex_override_color: crate::bindings::core_u_object::FColor,
    pub flags_252: u8,
    pub distance_field_resolution_scale: f32,
}
pub struct UFbxTextureImportData {
    pub flags_176: u8,
    pub material_search_location: EMaterialSearchLocation,
    pub base_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    pub base_color_name: FString,
    pub base_diffuse_texture_name: FString,
    pub base_normal_texture_name: FString,
    pub base_emissive_color_name: FString,
    pub base_emmisive_texture_name: FString,
    pub base_specular_texture_name: FString,
    pub base_opacity_texture_name: FString,
}
pub struct UReimportFbxSceneFactory {}
pub struct UEditorLoadingAndSavingUtils {}
pub struct AGroupActor {
    pub flags_1136: u8,
    pub group_actors: TArray<UPtr<crate::bindings::engine::AActor>>,
    pub sub_groups: TArray<UPtr<AGroupActor>>,
}
pub struct UHierarchicalLODSettings {
    pub b_force_settings_in_all_maps: bool,
    pub b_save_lod_actors_to_hlod_packages: bool,
    pub default_setup: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub directories_for_hlod_commandlet: TArray<
        crate::bindings::core_u_object::FDirectoryPath,
    >,
    pub maps_to_build: TArray<crate::bindings::core_u_object::FFilePath>,
    pub base_material: TSoftObjectPtr<crate::bindings::engine::UMaterialInterface>,
}
pub struct AHierarchicalLODVolume {
    pub b_include_overlapping_actors: bool,
    pub apply_only_to_specific_hlod_levels: TArray<i32>,
}
pub struct UEditorInstancedPlacementSettings {}
pub struct ULayersSubsystem {}
pub struct ULevelEditorDragDropHandler {}
pub struct ULightmassOptionsObject {
    pub debug_settings: crate::bindings::engine::FLightmassDebugOptions,
    pub swarm_settings: crate::bindings::engine::FSwarmDebugOptions,
}
pub struct UMaterialEditorMeshComponent {}
pub struct UMaterialGraph {
    pub material: UPtr<crate::bindings::engine::UMaterial>,
    pub material_function: UPtr<crate::bindings::engine::UMaterialFunction>,
    pub root_node: UPtr<UMaterialGraphNode_Root>,
    pub subgraph_expression: UPtr<crate::bindings::engine::UMaterialExpression>,
    pub original_material_full_name: FString,
}
pub struct UMaterialGraphNode_Base {}
pub struct UMaterialGraphNode {
    pub material_expression: UPtr<crate::bindings::engine::UMaterialExpression>,
}
pub struct UMaterialGraphNode_Comment {
    pub material_expression_comment: UPtr<
        crate::bindings::engine::UMaterialExpressionComment,
    >,
}
pub struct UMaterialGraphNode_Composite {
    pub bound_graph: UPtr<UMaterialGraph>,
}
pub struct UMaterialGraphNode_Custom {}
pub struct UMaterialGraphNode_Knot {}
pub struct UMaterialGraphNode_Operator {}
pub struct UMaterialGraphNode_PinBase {}
pub struct UMaterialGraphNode_Root {
    pub material: UPtr<crate::bindings::engine::UMaterial>,
}
pub struct UMaterialGraphSchema {}
pub struct UUndoableResolveHandler {
    pub b_should_be_resolved: bool,
}
pub struct UPackageTools {}
pub struct UPreviewMaterial {}
pub struct UPropertyEditorTestInstancedObject {
    pub number: i32,
}
pub struct UFirstDerivedPropertyEditorTestObject {
    pub string: FString,
}
pub struct USecondDerivedPropertyEditorTestObject {
    pub bool: bool,
}
pub struct UPropertyEditorTestObject {
    pub int8_property: i8,
    pub int16_property: i16,
    pub int32_property: i32,
    pub int64_property: i64,
    pub byte_property: u8,
    pub unsigned_int16_property: u16,
    pub unsigned_int32_property: u32,
    pub unsigned_int64_property: u64,
    pub float_property: f32,
    pub double_property: f64,
    pub name_property: FName,
    pub bool_property: bool,
    pub string_property: FString,
    pub text_property: FText,
    pub int_point_property: crate::bindings::core_u_object::FIntPoint,
    pub vector3_property: crate::bindings::core_u_object::FVector,
    pub vector2_property: crate::bindings::core_u_object::FVector2D,
    pub vector4_property: crate::bindings::core_u_object::FVector4,
    pub rotator_property: crate::bindings::core_u_object::FRotator,
    pub object_property: UPtr<crate::bindings::core_u_object::UObject>,
    pub linear_color_property: crate::bindings::core_u_object::FLinearColor,
    pub color_property: crate::bindings::core_u_object::FColor,
    pub enum_byte_property: EPropertyEditorTestEnum,
    pub enum_property: EPropertyEditorTestEditColor,
    pub enum_underscores: EPropertyEditorTestUnderscores,
    pub matrix_property: crate::bindings::core_u_object::FMatrix,
    pub transform_property: crate::bindings::core_u_object::FTransform,
    pub gigabyte_property: f64,
    pub class_property: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub class_property_with_allowed: TSubclassOf<
        crate::bindings::core_u_object::UObject,
    >,
    pub class_property_with_disallowed: TSubclassOf<
        crate::bindings::core_u_object::UObject,
    >,
    pub subclass_of_texture: TSubclassOf<crate::bindings::engine::UTexture>,
    pub subclass_of_with_allowed: TSubclassOf<crate::bindings::engine::UTexture>,
    pub subclass_of_with_disallowed: TSubclassOf<crate::bindings::engine::UTexture>,
    pub asset_pointer_with_allowed_and_whitespace: TSoftObjectPtr<
        crate::bindings::core_u_object::UObject,
    >,
    pub int_property32_array: TArray<i32>,
    pub byte_property_array: TArray<u8>,
    pub float_property_array: TArray<f32>,
    pub name_property_array: TArray<FName>,
    pub bool_property_array: TArray<bool>,
    pub string_property_array: TArray<FString>,
    pub text_property_array: TArray<FText>,
    pub vector3_property_array: TArray<crate::bindings::core_u_object::FVector>,
    pub vector2_property_array: TArray<crate::bindings::core_u_object::FVector2D>,
    pub vector4_property_array: TArray<crate::bindings::core_u_object::FVector4>,
    pub rotator_property_array: TArray<crate::bindings::core_u_object::FRotator>,
    pub object_property_array: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub actor_property_array: TArray<UPtr<crate::bindings::engine::AActor>>,
    pub linear_color_property_array: TArray<
        crate::bindings::core_u_object::FLinearColor,
    >,
    pub color_property_array: TArray<crate::bindings::core_u_object::FColor>,
    pub timecode_property_array: TArray<crate::bindings::core_u_object::FTimecode>,
    pub enum_property_array: TArray<EPropertyEditorTestEnum>,
    pub struct_property_array: TArray<FPropertyEditorTestBasicStruct>,
    pub struct_property_array_with_title: TArray<FPropertyEditorTestBasicStruct>,
    pub struct_property_array_with_formatted_title: TArray<
        FPropertyEditorTestBasicStruct,
    >,
    pub struct_property_array_with_title_error: TArray<FPropertyEditorTestBasicStruct>,
    pub struct_property_array_with_formatted_title_error: TArray<
        FPropertyEditorTestBasicStruct,
    >,
    pub instanced_struct_array: TArray<FPropertyEditorTestInstancedStruct>,
    pub object_property_array_with_title: TArray<
        UPtr<UPropertyEditorTestInstancedObject>,
    >,
    pub instanced_u_object_array: TArray<UPtr<UPropertyEditorTestInstancedObject>>,
    pub fixed_array_of_ints: TArray<i32>,
    pub static_array_of_ints: i32,
    pub static_array_of_ints_with_enum_labels: i32,
    pub float_range: crate::bindings::core_u_object::FFloatRange,
    pub float_property_with_clamped_range: f32,
    pub int_property_with_clamped_range: i32,
    pub int_that_cannot_be_changed: i32,
    pub string_that_cannot_be_changed: FString,
    pub object_that_cannot_be_changed: UPtr<
        crate::bindings::engine::UPrimitiveComponent,
    >,
    pub enum_bitflags: i32,
    pub string_password_property: FString,
    pub text_password_property: FText,
    pub this_is_broken_if_its_visible_in_a_details_view: FPropertyEditorTestBasicStruct,
    pub struct_with_multiple_instances1: FPropertyEditorTestBasicStruct,
    pub b_edit_condition_struct_with_multiple_instances2: bool,
    pub struct_with_multiple_instances2: FPropertyEditorTestBasicStruct,
    pub rich_curve: crate::bindings::engine::FRichCurve,
    pub soft_object_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub primary_asset_id: crate::bindings::core_u_object::FPrimaryAssetId,
    pub primary_asset_id_without_thumbnail: crate::bindings::core_u_object::FPrimaryAssetId,
    pub asset_reference_custom_struct_with_thumbnail: crate::bindings::core_u_object::FSoftObjectPath,
    pub exactly_point_light_actor_reference: crate::bindings::core_u_object::FSoftObjectPath,
    pub light_actor_reference: crate::bindings::core_u_object::FSoftObjectPath,
    pub exact_point_or_spot_light_actor_reference: crate::bindings::core_u_object::FSoftObjectPath,
    pub light_or_static_mesh_actor_reference: crate::bindings::core_u_object::FSoftObjectPath,
    pub not_light_actor_reference: crate::bindings::core_u_object::FSoftObjectPath,
    pub material_or_texture_asset_reference: crate::bindings::core_u_object::FSoftObjectPath,
    pub actor_with_meta_class: crate::bindings::core_u_object::FSoftObjectPath,
    pub disabled_by_can_edit_change: crate::bindings::core_u_object::FSoftObjectPath,
    pub component_reference: crate::bindings::engine::FComponentReference,
    pub b_edit_condition: bool,
    pub simple_property_with_edit_condition: i32,
    pub b_edit_condition_asset_reference_custom_struct_with_edit_condition: bool,
    pub asset_reference_custom_struct_with_edit_condition: crate::bindings::core_u_object::FSoftObjectPath,
    pub array_of_structs: TArray<FPropertyEditorTestBasicStruct>,
    pub _struct: FPropertyEditTestTextStruct,
    pub edit_inline_new_static_mesh_component: UPtr<
        crate::bindings::engine::UStaticMeshComponent,
    >,
    pub array_of_edit_inline_new_sm_cs: TArray<
        UPtr<crate::bindings::engine::UStaticMeshComponent>,
    >,
    pub texture_prop: UPtr<crate::bindings::engine::UTexture>,
    pub static_mesh_prop: UPtr<crate::bindings::engine::UStaticMesh>,
    pub any_material_interface: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub material_no_thumbnail: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub only_actors_allowed: UPtr<crate::bindings::engine::AActor>,
    pub int32_set: TSet<i32>,
    pub float_set: TSet<f32>,
    pub string_set: TSet<FString>,
    pub object_set: TSet<UPtr<crate::bindings::core_u_object::UObject>>,
    pub actor_set: TSet<UPtr<crate::bindings::engine::AActor>>,
    pub edit_color_set: TSet<EPropertyEditorTestEditColor>,
    pub name_set: TSet<FName>,
    pub int32_to_string_map: TMap<i32, FString>,
    pub string_to_multiline_text_map: TMap<FString, FText>,
    pub string_to_color_map: TMap<FString, crate::bindings::core_u_object::FLinearColor>,
    pub int32_to_struct_map: TMap<i32, FPropertyEditorTestBasicStruct>,
    pub string_to_float_map: TMap<FString, f32>,
    pub string_to_object_map: TMap<
        FString,
        UPtr<crate::bindings::core_u_object::UObject>,
    >,
    pub string_to_actor_map: TMap<FString, UPtr<crate::bindings::engine::AActor>>,
    pub object_to_int32_map: TMap<UPtr<crate::bindings::core_u_object::UObject>, i32>,
    pub object_to_color_map: TMap<
        UPtr<crate::bindings::core_u_object::UObject>,
        crate::bindings::core_u_object::FLinearColor,
    >,
    pub int_to_enum_map: TMap<i32, EPropertyEditorTestEnum>,
    pub name_to_name_map: TMap<FName, FName>,
    pub name_to_object_map: TMap<FName, UPtr<crate::bindings::core_u_object::UObject>>,
    pub name_to_custom_map: TMap<FName, FPropertyEditorTestBasicStruct>,
    pub name_to_color_map: TMap<FName, crate::bindings::core_u_object::FLinearColor>,
    pub int_to_custom_map: TMap<i32, FPropertyEditorTestBasicStruct>,
    pub int_to_sub_struct_map: TMap<i32, FPropertyEditorTestSubStruct>,
    pub linear_color_set: TSet<crate::bindings::core_u_object::FLinearColor>,
    pub vector_set: TSet<crate::bindings::core_u_object::FVector>,
    pub linear_color_to_string_map: TMap<
        crate::bindings::core_u_object::FLinearColor,
        FString,
    >,
    pub vector_to_float_map: TMap<crate::bindings::core_u_object::FVector, f32>,
    pub linear_color_to_vector_map: TMap<
        crate::bindings::core_u_object::FLinearColor,
        crate::bindings::core_u_object::FVector,
    >,
    pub blendable_interface: TScriptInterface<
        crate::bindings::engine::IBlendableInterface,
    >,
    pub anim_class_interface: TScriptInterface<
        crate::bindings::engine::IAnimClassInterface,
    >,
    pub light_propagation_volume_blendable: TScriptInterface<
        crate::bindings::engine::IBlendableInterface,
    >,
    pub texture_or_blendable_interface: UPtr<crate::bindings::core_u_object::UObject>,
    pub b_subcategory: bool,
    pub b_subcategory_advanced: bool,
    pub b_subcategory_foo_simple: bool,
    pub b_subcategory_foo_advanced: bool,
    pub b_subcategory_bar_simple: bool,
    pub b_subcategory_bar_advanced: bool,
    pub b_subcategory_last: bool,
    pub b_enables_next: bool,
    pub b_enabled_by_previous: bool,
    pub enum_edit_condition: EPropertyEditorTestEditColor,
    pub b_enabled_when_blue: bool,
    pub b_enabled_when_pink: bool,
    pub enum_as_byte_edit_condition: EPropertyEditorTestEnum,
    pub b_enabled_when_enum_is2: bool,
    pub b_enabled_when_enum_is4: bool,
    pub integer_edit_condition: i32,
    pub b_enabled_when_int_greater_or_equal5: bool,
    pub b_enabled_when_int_less_or_equal10: bool,
    pub float_edit_condition: f32,
    pub b_enabled_when_float_greater_than5: bool,
    pub b_enabled_when_float_less_than10: bool,
    pub b_edit_condition_for_arrays: bool,
    pub array_with_edit_condition: TArray<UPtr<crate::bindings::engine::UTexture2D>>,
    pub array_of_structs_with_edit_condition: TArray<FPropertyEditorTestBasicStruct>,
    pub b_edit_condition_for_fixed_array: bool,
    pub fixed_array_with_edit_condition: FString,
    pub b_edit_condition_for_directory_path: bool,
    pub directory_path: crate::bindings::core_u_object::FDirectoryPath,
    pub edit_condition_flags: i64,
    pub b_enabled_when_flags_has_two_or_four: bool,
    pub b_disabled_when_flags_is_odd: bool,
    pub always_disabled: i32,
    pub b_category_inline_edit_condition: bool,
    pub enabled_when_category_checked: f32,
    pub inline_property: crate::bindings::engine::EComponentMobility,
    pub property_that_hides: crate::bindings::engine::EComponentMobility,
    pub b_visible_when_static: bool,
    pub visible_when_stationary: i32,
    pub date_time: crate::bindings::core_u_object::FDateTime,
    pub timespan: crate::bindings::core_u_object::FTimespan,
    pub guid: crate::bindings::core_u_object::FGuid,
    pub per_platform_float: crate::bindings::core_u_object::FPerPlatformFloat,
    pub per_platform_int: crate::bindings::core_u_object::FPerPlatformInt,
    pub b_inline_edit_condition_without_meta_toggle: bool,
    pub inline_edit_condition_without_meta: f32,
    pub b_inline_edit_condition_with_meta_toggle: bool,
    pub inline_edit_condition_with_meta: f32,
    pub b_inline_edit_condition_not_editable: bool,
    pub has_non_editable_inline_condition: f32,
    pub b_shared_edit_condition: bool,
    pub uses_shared_edit_condition1: f32,
    pub uses_shared_edit_condition2: f32,
    pub struct_with_inline_condition: FPropertyEditorTestEditCondition,
    pub array_of_structs_with_inline_condition: TArray<FPropertyEditorTestEditCondition>,
    pub nested_array_of_ints: i32,
}
pub struct UHideCategoriesBase {
    pub hidden_in_base: i32,
}
pub struct UShowCategoriesTest {
    pub in_derived: i32,
}
pub struct UBlueprintPropertyTestObject {
    pub should_be_hidden: i32,
    pub should_be_visible: i32,
    pub object_a: UPtr<crate::bindings::engine::USoundBase>,
    pub object_b: UPtr<crate::bindings::engine::USoundBase>,
}
pub struct UBlueprintPropertyContainerTestObject {
    pub array: TArray<UPtr<UBlueprintPropertyTestObject>>,
}
pub struct UTestSparseClassDataBase {}
pub struct UTestSparseClassData {}
pub struct APropertyEditorTestActor {
    pub instanced_u_object_array: TArray<UPtr<UPropertyEditorTestInstancedObject>>,
    pub get_options_value: FName,
    pub defaults_only: f32,
    pub defaults_only_subcategory: f32,
    pub instance_only: f32,
    pub instance_only_subcategory: f32,
    pub multi_line_map: TMap<i32, FText>,
}
pub struct UPropertyEditorRowGeneratorTest {}
pub struct UUnrealEdViewportToolbarContext {}
pub struct UCommonViewportToolbarBaseMenuContext {}
pub struct USelection {
    pub element_selection_set: UPtr<
        crate::bindings::typed_element_runtime::UTypedElementSelectionSet,
    >,
}
pub struct UBlueprintEditorProjectSettings {
    pub flags_104: u8,
    pub default_child_actor_tree_view_mode: crate::bindings::engine::EChildActorComponentTreeViewVisualizationMode,
    pub namespaces_to_always_include: TArray<FString>,
    pub disabled_compiler_messages_except_editor: TArray<FName>,
    pub disabled_compiler_messages: TArray<FName>,
    pub suppressed_deprecation_messages: TArray<FString>,
    pub base_classes_to_allow_recompiling_during_play_in_editor: TArray<
        TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    >,
    pub base_classes_to_disallow_recompiling_during_play_in_editor: TArray<
        TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    >,
    pub b_disallow_editor_utility_blueprint_functions_in_details_view: bool,
}
pub struct UEditorPerProjectUserSettings {
    pub flags_48: u8,
    pub b_display_engine_version_in_badge: bool,
    pub b_use_simplygon_swarm: bool,
    pub simplygon_server_ip: FString,
    pub b_enable_swarm_debugging: bool,
    pub simplygon_swarm_delay: u32,
    pub swarm_num_of_concurrent_jobs: u32,
    pub swarm_max_upload_chunk_size_in_mb: u32,
    pub swarm_intermediate_folder: FString,
    pub flags_104: u8,
    pub data_source_folder: crate::bindings::core_u_object::FDirectoryPath,
    pub flags_128: u8,
    pub property_matrix_number_of_paste_operations_before_warning: i32,
    pub b_scs_editor_show_grid: bool,
    pub b_scs_editor_show_floor: bool,
    pub b_get_attention_on_uat_completion: bool,
    pub b_always_build_uat: bool,
    pub scs_viewport_camera_speed_settings: FEditorViewportCameraSpeedSettings,
    pub scs_viewport_camera_speed_deprecated: i32,
    pub flags_164: u8,
    pub blueprint_favorites: UPtr<crate::bindings::kismet::UBlueprintPaletteFavorites>,
    pub asset_viewer_profile_index: i32,
    pub asset_viewer_profile_name: FString,
    pub preview_feature_level: i32,
    pub preview_platform_name: FName,
    pub preview_shader_format_name: FName,
    pub preview_shader_platform_name: FName,
    pub preview_constrained_aspect_ratio: f32,
    pub preview_safe_zones: crate::bindings::core_u_object::FVector4f,
    pub b_preview_feature_level_active: bool,
    pub b_preview_feature_level_was_default: bool,
    pub preview_device_profile_name: FName,
}
pub struct UEditorProjectAppearanceSettings {
    pub b_display_units: bool,
    pub b_display_units_on_component_transforms: bool,
    pub distance_units: TArray<crate::bindings::core_u_object::EUnit>,
    pub mass_units: TArray<crate::bindings::core_u_object::EUnit>,
    pub time_units: TArray<crate::bindings::core_u_object::EUnit>,
    pub angle_units: crate::bindings::core_u_object::EUnit,
    pub speed_units: crate::bindings::core_u_object::EUnit,
    pub angular_speed_units: crate::bindings::core_u_object::EUnit,
    pub acceleration_units: crate::bindings::core_u_object::EUnit,
    pub temperature_units: crate::bindings::core_u_object::EUnit,
    pub force_units: crate::bindings::core_u_object::EUnit,
    pub torque_units: crate::bindings::core_u_object::EUnit,
    pub impulse_units: crate::bindings::core_u_object::EUnit,
    pub positional_impulse_units: crate::bindings::core_u_object::EUnit,
    pub show_searchable_names: EReferenceViewerSettingMode,
    pub reference_viewer_default_max_search_breadth: i32,
    pub unit_display_deprecated: EUnitDisplay,
    pub default_input_units_deprecated: EDefaultLocationUnit,
}
pub struct ULevelEditor2DSettings {
    pub b_enable2_d_widget: bool,
    pub b_enable_snap_layers: bool,
    pub snap_axis: ELevelEditor2DAxis,
    pub snap_layers: TArray<FMode2DLayer>,
}
pub struct ULevelEditorProjectSettings {
    pub b_enable_viewport_sm_instance_selection: bool,
}
pub struct UEditorPerformanceProjectSettings {
    pub realtime_screen_percentage_mode: crate::bindings::engine::EScreenPercentageMode,
    pub mobile_screen_percentage_mode: crate::bindings::engine::EScreenPercentageMode,
    pub vr_screen_percentage_mode: crate::bindings::engine::EScreenPercentageMode,
    pub path_tracer_screen_percentage_mode: crate::bindings::engine::EScreenPercentageMode,
    pub non_realtime_screen_percentage_mode: crate::bindings::engine::EScreenPercentageMode,
    pub manual_screen_percentage: f32,
    pub min_viewport_rendering_resolution: i32,
    pub max_viewport_rendering_resolution: i32,
}
pub struct UEditorProjectAssetSettings {
    pub b_prompt_to_delete_unreferenced_redirectors: bool,
    pub b_rename_localized_variants_alongside_source_asset: bool,
}
pub struct UDDCProjectSettings {
    pub enable_warnings: bool,
    pub recommend_everyone_setup_a_global_local_ddc_path: bool,
    pub recommend_everyone_setup_a_global_shared_ddc_path: bool,
    pub recommend_everyone_setup_a_global_s3ddc_path: bool,
    pub recommend_everyone_enable_s3ddc: bool,
    pub recommend_everyone_use_unreal_cloud_ddc: bool,
}
pub struct UEditorSettings {
    pub global_local_ddc_path: crate::bindings::core_u_object::FDirectoryPath,
    pub global_shared_ddc_path: crate::bindings::core_u_object::FDirectoryPath,
    pub local_derived_data_cache: crate::bindings::core_u_object::FDirectoryPath,
    pub shared_derived_data_cache: crate::bindings::core_u_object::FDirectoryPath,
    pub b_enable_ddc_notifications: bool,
    pub b_notify_use_unreal_cloud_ddc: bool,
    pub b_notify_setup_ddc_path: bool,
    pub b_notify_enable_s3dd: bool,
    pub b_enable_s3ddc: bool,
    pub global_s3ddc_path: crate::bindings::core_u_object::FDirectoryPath,
    pub horde_url: FString,
    pub b_load_the_most_recently_loaded_project_at_startup: bool,
    pub recently_opened_project_files: TArray<FRecentProjectFile>,
    pub created_project_paths: TArray<FString>,
    pub b_copy_starter_content_preference_deprecated: bool,
    pub completed_surveys: TArray<crate::bindings::core_u_object::FGuid>,
    pub in_progress_surveys: TArray<crate::bindings::core_u_object::FGuid>,
    pub auto_scalability_work_scale_amount: f32,
}
pub struct UActorEditorContextSubsystem {}
pub struct UAssetEditorSubsystem {
    pub owned_asset_editors: TArray<UPtr<UAssetEditor>>,
}
pub struct UBrowseToAssetOverrideSubsystem {}
pub struct UBrushEditingSubsystem {}
pub struct UCollectionManagerScriptingSubsystem {}
pub struct UEditorActorSubsystem {
    pub on_new_actors_dropped: FEditorActorSubsystem_OnNewActorsDropped,
    pub on_new_actors_placed: FEditorActorSubsystem_OnNewActorsPlaced,
    pub on_edit_cut_actors_begin: FEditorActorSubsystem_OnEditCutActorsBegin,
    pub on_edit_cut_actors_end: FEditorActorSubsystem_OnEditCutActorsEnd,
    pub on_edit_copy_actors_begin: FEditorActorSubsystem_OnEditCopyActorsBegin,
    pub on_edit_copy_actors_end: FEditorActorSubsystem_OnEditCopyActorsEnd,
    pub on_edit_paste_actors_begin: FEditorActorSubsystem_OnEditPasteActorsBegin,
    pub on_edit_paste_actors_end: FEditorActorSubsystem_OnEditPasteActorsEnd,
    pub on_duplicate_actors_begin: FEditorActorSubsystem_OnDuplicateActorsBegin,
    pub on_duplicate_actors_end: FEditorActorSubsystem_OnDuplicateActorsEnd,
    pub on_delete_actors_begin: FEditorActorSubsystem_OnDeleteActorsBegin,
    pub on_delete_actors_end: FEditorActorSubsystem_OnDeleteActorsEnd,
    pub on_actor_label_changed: FEditorActorSubsystem_OnActorLabelChanged,
}
pub struct UEditorAssetSubsystem {}
pub struct UEditorSubsystemBlueprintLibrary {}
pub struct UImportSubsystem {
    pub on_asset_pre_import_bp: FImportSubsystem_OnAssetPreImport_BP,
    pub on_asset_post_import_bp: FImportSubsystem_OnAssetPostImport_BP,
    pub on_asset_reimport_bp: FImportSubsystem_OnAssetReimport_BP,
    pub on_asset_post_lod_import_bp: FImportSubsystem_OnAssetPostLODImport_BP,
}
pub struct UPanelExtensionSubsystem {}
pub struct UPropertyVisibilityOverrideSubsystem {}
pub struct UUnrealEditorSubsystem {}
pub struct UDEPRECATED_TemplateMapMetadata {}
pub struct UFbxTestPlan {
    pub test_plan_name: FString,
    pub action: EFBXTestPlanActionType,
    pub lod_index: i32,
    pub b_delete_folder_assets: bool,
    pub expected_result: TArray<FFbxTestPlanExpectedResult>,
    pub import_ui: UPtr<UFbxImportUI>,
}
pub struct AAnimationThumbnailSkeletalMeshActor {}
pub struct UThumbnailManager {
    pub not_supported: FThumbnailRenderingInfo,
    pub editor_cube: UPtr<crate::bindings::engine::UStaticMesh>,
    pub editor_sphere: UPtr<crate::bindings::engine::UStaticMesh>,
    pub editor_cylinder: UPtr<crate::bindings::engine::UStaticMesh>,
    pub editor_plane: UPtr<crate::bindings::engine::UStaticMesh>,
    pub editor_sky_sphere: UPtr<crate::bindings::engine::UStaticMesh>,
    pub floor_plane_material: UPtr<crate::bindings::engine::UMaterial>,
    pub ambient_cubemap: UPtr<crate::bindings::engine::UTextureCube>,
    pub checkerboard_texture: UPtr<crate::bindings::engine::UTexture2D>,
    pub renderable_thumbnail_types: TArray<FThumbnailRenderingInfo>,
    pub thumbnail_manager_class_name: FString,
}
pub struct UAnimBlueprintThumbnailRenderer {}
pub struct UAnimSequenceThumbnailRenderer {}
pub struct UBlendSpaceThumbnailRenderer {}
pub struct UClassThumbnailRenderer {}
pub struct UCurveFloatThumbnailRenderer {}
pub struct UCurveVector3ThumbnailRenderer {}
pub struct UCurveLinearColorThumbnailRenderer {}
pub struct UFontThumbnailRenderer {}
pub struct ULevelThumbnailRenderer {}
pub struct UMaterialFunctionThumbnailRenderer {}
pub struct UMaterialInstanceThumbnailRenderer {}
pub struct UNeuralProfileRenderer {}
pub struct UParticleSystemThumbnailRenderer {
    pub no_image: UPtr<crate::bindings::engine::UTexture2D>,
    pub out_of_date: UPtr<crate::bindings::engine::UTexture2D>,
}
pub struct UPhysicalMaterialMaskThumbnailRenderer {}
pub struct UPhysicsAssetThumbnailRenderer {}
pub struct USceneThumbnailInfo {
    pub orbit_pitch: f32,
    pub orbit_yaw: f32,
    pub orbit_zoom: f32,
}
pub struct USceneThumbnailInfoWithPrimitive {
    pub primitive_type: EThumbnailPrimType,
    pub preview_mesh: crate::bindings::core_u_object::FSoftObjectPath,
    pub b_user_modified_shape: bool,
}
pub struct USkeletalMeshThumbnailRenderer {}
pub struct USkeletonThumbnailRenderer {}
pub struct USlateBrushThumbnailRenderer {}
pub struct USoundWaveThumbnailRenderer {}
pub struct USpecularProfileRenderer {}
pub struct UStaticMeshThumbnailRenderer {}
pub struct USubsurfaceProfileRenderer {}
pub struct UTextureCubeArrayThumbnailRenderer {}
pub struct UTextureCubeThumbnailRenderer {}
pub struct UVolumeTextureThumbnailRenderer {
    pub material_instance: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
}
pub struct UWorldThumbnailInfo {
    pub camera_mode: crate::bindings::engine::ECameraProjectionMode,
    pub ortho_direction: EOrthoThumbnailDirection,
}
pub struct UWorldThumbnailRenderer {
    pub global_orbit_pitch_offset: f32,
    pub global_orbit_yaw_offset: f32,
    pub b_use_unlit_scene: bool,
    pub b_allow_world_thumbnails: bool,
}
pub struct UAssetEditorContextObject {}
pub struct UEdMode {
    pub mode_tools_context: UPtr<UEdModeInteractiveToolsContext>,
    pub settings_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub settings_object: UPtr<crate::bindings::core_u_object::UObject>,
}
pub struct UBaseLegacyWidgetEdMode {}
pub struct UEdModeDefault {}
pub struct UAssetEdModeDefault {}
pub struct UEdModeInteractiveToolsContext {
    pub parent_mode_manager_tools_context: UPtr<UModeManagerInteractiveToolsContext>,
}
pub struct ULegacyEdModeWrapper {}
pub struct UAssetEditor {}
pub struct UUnrealEdEngine {
    pub editor_options_inst: UPtr<UUnrealEdOptions>,
    pub auto_reimport_manager: UPtr<UAutoReimportManager>,
    pub material_copy_paste_buffer: UPtr<crate::bindings::engine::UMaterial>,
    pub sound_cue_copy_paste_buffer: UPtr<crate::bindings::engine::USoundCue>,
    pub animation_compression_algorithms: TArray<
        UPtr<crate::bindings::engine::UAnimCompress>,
    >,
    pub packages_to_be_fully_loaded_at_startup: TArray<FString>,
    pub current_lod_parent_actor: UPtr<crate::bindings::engine::AActor>,
    pub flags_10104: u8,
    pub sorted_sprite_categories_deprecated: TArray<FString>,
    pub template_map_infos: TArray<FTemplateMapInfo>,
    pub cook_server: UPtr<UCookOnTheFlyServer>,
    pub classes_to_ignore_delete_reference_warning: TArray<
        TSubclassOf<crate::bindings::core_u_object::UObject>,
    >,
}
pub struct UUnrealEdTypes {}
pub struct UUserDefinedStructEditorData {
    pub unique_name_id: u32,
    pub variables_descriptions: TArray<FStructVariableDescription>,
    pub tool_tip: FString,
}
pub struct UWorldFolders {
    pub current_folder: FActorPlacementFolder,
}
pub struct UWorldPartitionFoliageBuilder {}
pub struct UWorldPartitionHLODsBuilder {}
pub struct UWorldPartitionLandscapeSplineMeshesBuilder {}
pub struct UWorldPartitionMiniMapBuilder {}
pub struct UWorldPartitionNavigationDataBuilder {}
pub struct UWorldPartitionRenameDuplicateBuilder {
    pub duplicated_objects: TMap<
        UPtr<crate::bindings::core_u_object::UObject>,
        UPtr<crate::bindings::core_u_object::UObject>,
    >,
}
pub struct UWorldPartitionResaveActorsBuilder {
    pub actor_class_name: FString,
    pub actor_classes_from_file: FString,
    pub b_report_only: bool,
    pub b_resave_dirty_actor_descs_only: bool,
    pub b_diff_dirty_actor_descs: bool,
    pub b_switch_actor_packaging_scheme_to_reduced: bool,
    pub b_enable_actor_folders: bool,
    pub b_resave_blueprints: bool,
    pub actor_tags: TSet<FName>,
    pub actor_properties: TMap<FName, FName>,
}
pub struct UWorldPartitionRuntimeVirtualTextureBuilder {}
pub struct UWorldPartitionStaticLightingBuilder {}
pub struct UCookOnTheFlyServer {}
pub struct FAddOnExtractAssetFromFile_Delegate;
pub struct FRemoveOnExtractAssetFromFile_Delegate;
pub struct FEditorActorSubsystem_OnNewActorsDropped;
pub struct FEditorActorSubsystem_OnNewActorsPlaced;
pub struct FEditorActorSubsystem_OnEditCutActorsBegin;
pub struct FEditorActorSubsystem_OnEditCutActorsEnd;
pub struct FEditorActorSubsystem_OnEditCopyActorsBegin;
pub struct FEditorActorSubsystem_OnEditCopyActorsEnd;
pub struct FEditorActorSubsystem_OnEditPasteActorsBegin;
pub struct FEditorActorSubsystem_OnEditPasteActorsEnd;
pub struct FEditorActorSubsystem_OnDuplicateActorsBegin;
pub struct FEditorActorSubsystem_OnDuplicateActorsEnd;
pub struct FEditorActorSubsystem_OnDeleteActorsBegin;
pub struct FEditorActorSubsystem_OnDeleteActorsEnd;
pub struct FEditorActorSubsystem_OnActorLabelChanged;
pub struct FImportSubsystem_OnAssetPreImport_BP;
pub struct FImportSubsystem_OnAssetPostImport_BP;
pub struct FImportSubsystem_OnAssetReimport_BP;
pub struct FImportSubsystem_OnAssetPostLODImport_BP;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELevelViewportType(pub u8);
impl ELevelViewportType {
    pub const LVT_ORTHO_XY: ELevelViewportType = ELevelViewportType(0);
    pub const LVT_ORTHO_XZ: ELevelViewportType = ELevelViewportType(1);
    pub const LVT_ORTHO_YZ: ELevelViewportType = ELevelViewportType(2);
    pub const LVT_PERSPECTIVE: ELevelViewportType = ELevelViewportType(3);
    pub const LVT_ORTHO_FREELOOK: ELevelViewportType = ELevelViewportType(4);
    pub const LVT_ORTHO_NEGATIVE_XY: ELevelViewportType = ELevelViewportType(5);
    pub const LVT_ORTHO_NEGATIVE_XZ: ELevelViewportType = ELevelViewportType(6);
    pub const LVT_ORTHO_NEGATIVE_YZ: ELevelViewportType = ELevelViewportType(7);
    pub const LVT_ORTHO_TOP: ELevelViewportType = ELevelViewportType(0);
    pub const LVT_ORTHO_LEFT: ELevelViewportType = ELevelViewportType(1);
    pub const LVT_ORTHO_FRONT: ELevelViewportType = ELevelViewportType(7);
    pub const LVT_ORTHO_BACK: ELevelViewportType = ELevelViewportType(2);
    pub const LVT_ORTHO_BOTTOM: ELevelViewportType = ELevelViewportType(5);
    pub const LVT_ORTHO_RIGHT: ELevelViewportType = ELevelViewportType(6);
    pub const LVT_NONE: ELevelViewportType = ELevelViewportType(255);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAnimationViewportCameraFollowMode(pub u8);
impl EAnimationViewportCameraFollowMode {
    pub const NONE: EAnimationViewportCameraFollowMode = EAnimationViewportCameraFollowMode(
        0,
    );
    pub const BOUNDS: EAnimationViewportCameraFollowMode = EAnimationViewportCameraFollowMode(
        1,
    );
    pub const BONE: EAnimationViewportCameraFollowMode = EAnimationViewportCameraFollowMode(
        2,
    );
    pub const ROOT: EAnimationViewportCameraFollowMode = EAnimationViewportCameraFollowMode(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECSVImportType(pub u8);
impl ECSVImportType {
    pub const ECSV_DATA_TABLE: ECSVImportType = ECSVImportType(0);
    pub const ECSV_CURVE_TABLE: ECSVImportType = ECSVImportType(1);
    pub const ECSV_CURVE_FLOAT: ECSVImportType = ECSVImportType(2);
    pub const ECSV_CURVE_VECTOR: ECSVImportType = ECSVImportType(3);
    pub const ECSV_CURVE_LINEAR_COLOR: ECSVImportType = ECSVImportType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETestEnumFlags(pub u8);
impl ETestEnumFlags {
    pub const NONE: ETestEnumFlags = ETestEnumFlags(0);
    pub const ONE: ETestEnumFlags = ETestEnumFlags(1);
    pub const TWO: ETestEnumFlags = ETestEnumFlags(2);
    pub const FOUR: ETestEnumFlags = ETestEnumFlags(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct NetworkEmulationTarget(pub i32);
impl NetworkEmulationTarget {
    pub const SERVER: NetworkEmulationTarget = NetworkEmulationTarget(0);
    pub const CLIENT: NetworkEmulationTarget = NetworkEmulationTarget(1);
    pub const ANY: NetworkEmulationTarget = NetworkEmulationTarget(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFBXExpectedResultPreset(pub u8);
impl EFBXExpectedResultPreset {
    pub const ERROR_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(0);
    pub const WARNING_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(1);
    pub const CREATED_STATICMESH_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        2,
    );
    pub const CREATED_SKELETALMESH_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        3,
    );
    pub const MATERIALS_CREATED_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        4,
    );
    pub const MATERIAL_SLOT_IMPORTED_NAME: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        5,
    );
    pub const VERTEX_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(6);
    pub const LOD_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(7);
    pub const VERTEX_NUMBER_LOD: EFBXExpectedResultPreset = EFBXExpectedResultPreset(8);
    pub const MESH_MATERIALS_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        9,
    );
    pub const MESH_LOD_SECTION_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        10,
    );
    pub const MESH_LOD_SECTION_VERTEX_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        11,
    );
    pub const MESH_LOD_SECTION_TRIANGLE_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        12,
    );
    pub const MESH_LOD_SECTION_MATERIAL_NAME: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        13,
    );
    pub const MESH_LOD_SECTION_MATERIAL_INDEX: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        14,
    );
    pub const MESH_LOD_SECTION_MATERIAL_IMPORTED_NAME: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        15,
    );
    pub const MESH_LOD_VERTEX_POSITION: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        16,
    );
    pub const MESH_LOD_VERTEX_NORMAL: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        17,
    );
    pub const LOD_UV_CHANNEL_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        18,
    );
    pub const BONE_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(19);
    pub const BONE_POSITION: EFBXExpectedResultPreset = EFBXExpectedResultPreset(20);
    pub const ANIMATION_FRAME_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        21,
    );
    pub const ANIMATION_LENGTH: EFBXExpectedResultPreset = EFBXExpectedResultPreset(22);
    pub const ANIMATION_CUSTOM_CURVE_KEY_VALUE: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        23,
    );
    pub const ANIMATION_CUSTOM_CURVE_KEY_ARRIVE_TANGENT: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        24,
    );
    pub const ANIMATION_CUSTOM_CURVE_KEY_LEAVE_TANGENT: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        25,
    );
    pub const SKIN_BY_BONE_VERTEX_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        26,
    );
    pub const ANIMATION_CUSTOM_CURVE_KEY_ARRIVE_TANGENT_WEIGHT: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        27,
    );
    pub const ANIMATION_CUSTOM_CURVE_KEY_LEAVE_TANGENT_WEIGHT: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        28,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPhysicsAssetEditorCenterOfMassViewMode(pub u8);
impl EPhysicsAssetEditorCenterOfMassViewMode {
    pub const ALL: EPhysicsAssetEditorCenterOfMassViewMode = EPhysicsAssetEditorCenterOfMassViewMode(
        0,
    );
    pub const SELECTED: EPhysicsAssetEditorCenterOfMassViewMode = EPhysicsAssetEditorCenterOfMassViewMode(
        1,
    );
    pub const NONE: EPhysicsAssetEditorCenterOfMassViewMode = EPhysicsAssetEditorCenterOfMassViewMode(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPhysicsAssetEditorCollisionViewMode(pub u8);
impl EPhysicsAssetEditorCollisionViewMode {
    pub const SOLID: EPhysicsAssetEditorCollisionViewMode = EPhysicsAssetEditorCollisionViewMode(
        0,
    );
    pub const WIREFRAME: EPhysicsAssetEditorCollisionViewMode = EPhysicsAssetEditorCollisionViewMode(
        1,
    );
    pub const SOLID_WIREFRAME: EPhysicsAssetEditorCollisionViewMode = EPhysicsAssetEditorCollisionViewMode(
        2,
    );
    pub const NONE: EPhysicsAssetEditorCollisionViewMode = EPhysicsAssetEditorCollisionViewMode(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPhysicsAssetEditorConstraintViewMode(pub u8);
impl EPhysicsAssetEditorConstraintViewMode {
    pub const NONE: EPhysicsAssetEditorConstraintViewMode = EPhysicsAssetEditorConstraintViewMode(
        0,
    );
    pub const ALL_POSITIONS: EPhysicsAssetEditorConstraintViewMode = EPhysicsAssetEditorConstraintViewMode(
        1,
    );
    pub const ALL_LIMITS: EPhysicsAssetEditorConstraintViewMode = EPhysicsAssetEditorConstraintViewMode(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EEditorAssetMetaDataSortType(pub u8);
impl EEditorAssetMetaDataSortType {
    pub const STRING: EEditorAssetMetaDataSortType = EEditorAssetMetaDataSortType(0);
    pub const NUMERIC: EEditorAssetMetaDataSortType = EEditorAssetMetaDataSortType(1);
    pub const DATE_TIME: EEditorAssetMetaDataSortType = EEditorAssetMetaDataSortType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EEditorAssetSortOrder(pub u8);
impl EEditorAssetSortOrder {
    pub const ASCENDING: EEditorAssetSortOrder = EEditorAssetSortOrder(0);
    pub const DESCENDING: EEditorAssetSortOrder = EEditorAssetSortOrder(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELevelVisibilityDirtyMode(pub u8);
impl ELevelVisibilityDirtyMode {
    pub const MODIFY_ON_CHANGE: ELevelVisibilityDirtyMode = ELevelVisibilityDirtyMode(0);
    pub const DONT_MODIFY: ELevelVisibilityDirtyMode = ELevelVisibilityDirtyMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMaterialSearchLocation(pub u8);
impl EMaterialSearchLocation {
    pub const LOCAL: EMaterialSearchLocation = EMaterialSearchLocation(0);
    pub const UNDER_PARENT: EMaterialSearchLocation = EMaterialSearchLocation(1);
    pub const UNDER_ROOT: EMaterialSearchLocation = EMaterialSearchLocation(2);
    pub const ALL_ASSETS: EMaterialSearchLocation = EMaterialSearchLocation(3);
    pub const DO_NOT_SEARCH: EMaterialSearchLocation = EMaterialSearchLocation(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EReloadPackagesInteractionMode(pub u8);
impl EReloadPackagesInteractionMode {
    pub const INTERACTIVE: EReloadPackagesInteractionMode = EReloadPackagesInteractionMode(
        0,
    );
    pub const ASSUME_POSITIVE: EReloadPackagesInteractionMode = EReloadPackagesInteractionMode(
        1,
    );
    pub const ASSUME_NEGATIVE: EReloadPackagesInteractionMode = EReloadPackagesInteractionMode(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFbxExportCompatibility(pub u8);
impl EFbxExportCompatibility {
    pub const FBX_2011: EFbxExportCompatibility = EFbxExportCompatibility(0);
    pub const FBX_2012: EFbxExportCompatibility = EFbxExportCompatibility(1);
    pub const FBX_2013: EFbxExportCompatibility = EFbxExportCompatibility(2);
    pub const FBX_2014: EFbxExportCompatibility = EFbxExportCompatibility(3);
    pub const FBX_2016: EFbxExportCompatibility = EFbxExportCompatibility(4);
    pub const FBX_2018: EFbxExportCompatibility = EFbxExportCompatibility(5);
    pub const FBX_2019: EFbxExportCompatibility = EFbxExportCompatibility(6);
    pub const FBX_2020: EFbxExportCompatibility = EFbxExportCompatibility(7);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBlueprintBreakpointReloadMethod(pub i32);
impl EBlueprintBreakpointReloadMethod {
    pub const RESTORE_ALL: EBlueprintBreakpointReloadMethod = EBlueprintBreakpointReloadMethod(
        0,
    );
    pub const RESTORE_ALL_AND_DISABLE: EBlueprintBreakpointReloadMethod = EBlueprintBreakpointReloadMethod(
        1,
    );
    pub const DISCARD_ALL: EBlueprintBreakpointReloadMethod = EBlueprintBreakpointReloadMethod(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECommentBoxMode(pub u8);
impl ECommentBoxMode {
    pub const GROUP_MOVEMENT: ECommentBoxMode = ECommentBoxMode(0);
    pub const NO_GROUP_MOVEMENT: ECommentBoxMode = ECommentBoxMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EClassViewerDeveloperType(pub u8);
impl EClassViewerDeveloperType {
    pub const CVDT_NONE: EClassViewerDeveloperType = EClassViewerDeveloperType(0);
    pub const CVDT_CURRENT_USER: EClassViewerDeveloperType = EClassViewerDeveloperType(
        1,
    );
    pub const CVDT_ALL: EClassViewerDeveloperType = EClassViewerDeveloperType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELoadLevelAtStartup(pub u8);
impl ELoadLevelAtStartup {
    pub const NONE: ELoadLevelAtStartup = ELoadLevelAtStartup(0);
    pub const PROJECT_DEFAULT: ELoadLevelAtStartup = ELoadLevelAtStartup(1);
    pub const LAST_OPENED: ELoadLevelAtStartup = ELoadLevelAtStartup(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERestoreOpenAssetTabsMethod(pub u8);
impl ERestoreOpenAssetTabsMethod {
    pub const ALWAYS_PROMPT: ERestoreOpenAssetTabsMethod = ERestoreOpenAssetTabsMethod(
        0,
    );
    pub const ALWAYS_RESTORE: ERestoreOpenAssetTabsMethod = ERestoreOpenAssetTabsMethod(
        1,
    );
    pub const NEVER_RESTORE: ERestoreOpenAssetTabsMethod = ERestoreOpenAssetTabsMethod(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAutoSaveMethod(pub u8);
impl EAutoSaveMethod {
    pub const BACKUP_AND_RESTORE: EAutoSaveMethod = EAutoSaveMethod(0);
    pub const BACKUP_AND_OVERWRITE: EAutoSaveMethod = EAutoSaveMethod(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAssetEditorOpenLocation(pub u8);
impl EAssetEditorOpenLocation {
    pub const DEFAULT: EAssetEditorOpenLocation = EAssetEditorOpenLocation(0);
    pub const NEW_WINDOW: EAssetEditorOpenLocation = EAssetEditorOpenLocation(1);
    pub const MAIN_WINDOW: EAssetEditorOpenLocation = EAssetEditorOpenLocation(2);
    pub const CONTENT_BROWSER: EAssetEditorOpenLocation = EAssetEditorOpenLocation(3);
    pub const LAST_DOCKED_WINDOW_OR_NEW_WINDOW: EAssetEditorOpenLocation = EAssetEditorOpenLocation(
        4,
    );
    pub const LAST_DOCKED_WINDOW_OR_MAIN_WINDOW: EAssetEditorOpenLocation = EAssetEditorOpenLocation(
        5,
    );
    pub const LAST_DOCKED_WINDOW_OR_CONTENT_BROWSER: EAssetEditorOpenLocation = EAssetEditorOpenLocation(
        6,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFBXImportType(pub u8);
impl EFBXImportType {
    pub const FBXIT_STATIC_MESH: EFBXImportType = EFBXImportType(0);
    pub const FBXIT_SKELETAL_MESH: EFBXImportType = EFBXImportType(1);
    pub const FBXIT_ANIMATION: EFBXImportType = EFBXImportType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELabelAnchorMode(pub u8);
impl ELabelAnchorMode {
    pub const LABEL_ANCHOR_MODE_TOP_LEFT: ELabelAnchorMode = ELabelAnchorMode(0);
    pub const LABEL_ANCHOR_MODE_TOP_CENTER: ELabelAnchorMode = ELabelAnchorMode(1);
    pub const LABEL_ANCHOR_MODE_TOP_RIGHT: ELabelAnchorMode = ELabelAnchorMode(2);
    pub const LABEL_ANCHOR_MODE_CENTER_LEFT: ELabelAnchorMode = ELabelAnchorMode(3);
    pub const LABEL_ANCHOR_MODE_CENTERED: ELabelAnchorMode = ELabelAnchorMode(4);
    pub const LABEL_ANCHOR_MODE_CENTER_RIGHT: ELabelAnchorMode = ELabelAnchorMode(5);
    pub const LABEL_ANCHOR_MODE_BOTTOM_LEFT: ELabelAnchorMode = ELabelAnchorMode(6);
    pub const LABEL_ANCHOR_MODE_BOTTOM_CENTER: ELabelAnchorMode = ELabelAnchorMode(7);
    pub const LABEL_ANCHOR_MODE_BOTTOM_RIGHT: ELabelAnchorMode = ELabelAnchorMode(8);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPlayOnBuildMode(pub u8);
impl EPlayOnBuildMode {
    pub const PLAY_ON_BUILD_ALWAYS: EPlayOnBuildMode = EPlayOnBuildMode(0);
    pub const PLAY_ON_BUILD_NEVER: EPlayOnBuildMode = EPlayOnBuildMode(1);
    pub const PLAY_ON_BUILD_DEFAULT: EPlayOnBuildMode = EPlayOnBuildMode(2);
    pub const PLAY_ON_BUILD_IF_EDITOR_BUILT_LOCALLY: EPlayOnBuildMode = EPlayOnBuildMode(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPlayOnLaunchConfiguration(pub u8);
impl EPlayOnLaunchConfiguration {
    pub const LAUNCH_CONFIG_DEFAULT: EPlayOnLaunchConfiguration = EPlayOnLaunchConfiguration(
        0,
    );
    pub const LAUNCH_CONFIG_DEBUG: EPlayOnLaunchConfiguration = EPlayOnLaunchConfiguration(
        1,
    );
    pub const LAUNCH_CONFIG_DEVELOPMENT: EPlayOnLaunchConfiguration = EPlayOnLaunchConfiguration(
        2,
    );
    pub const LAUNCH_CONFIG_TEST: EPlayOnLaunchConfiguration = EPlayOnLaunchConfiguration(
        3,
    );
    pub const LAUNCH_CONFIG_SHIPPING: EPlayOnLaunchConfiguration = EPlayOnLaunchConfiguration(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPlayOnPakFileMode(pub u8);
impl EPlayOnPakFileMode {
    pub const NO_PAK: EPlayOnPakFileMode = EPlayOnPakFileMode(0);
    pub const PAK_NO_COMPRESS: EPlayOnPakFileMode = EPlayOnPakFileMode(1);
    pub const PAK_COMPRESS: EPlayOnPakFileMode = EPlayOnPakFileMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPlayNetMode(pub u8);
impl EPlayNetMode {
    pub const PIE_STANDALONE: EPlayNetMode = EPlayNetMode(0);
    pub const PIE_LISTEN_SERVER: EPlayNetMode = EPlayNetMode(1);
    pub const PIE_CLIENT: EPlayNetMode = EPlayNetMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELaunchModeType(pub u8);
impl ELaunchModeType {
    pub const LAUNCH_MODE_ON_DEVICE: ELaunchModeType = ELaunchModeType(0);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPlayModeLocations(pub u8);
impl EPlayModeLocations {
    pub const PLAY_LOCATION_CURRENT_CAMERA_LOCATION: EPlayModeLocations = EPlayModeLocations(
        0,
    );
    pub const PLAY_LOCATION_DEFAULT_PLAYER_START: EPlayModeLocations = EPlayModeLocations(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPlayModeType(pub u8);
impl EPlayModeType {
    pub const PLAY_MODE_IN_VIEW_PORT: EPlayModeType = EPlayModeType(0);
    pub const PLAY_MODE_IN_EDITOR_FLOATING: EPlayModeType = EPlayModeType(1);
    pub const PLAY_MODE_IN_MOBILE_PREVIEW: EPlayModeType = EPlayModeType(2);
    pub const PLAY_MODE_IN_TARGETED_MOBILE_PREVIEW: EPlayModeType = EPlayModeType(3);
    pub const PLAY_MODE_IN_VULKAN_PREVIEW: EPlayModeType = EPlayModeType(4);
    pub const PLAY_MODE_IN_NEW_PROCESS: EPlayModeType = EPlayModeType(5);
    pub const PLAY_MODE_IN_VR: EPlayModeType = EPlayModeType(6);
    pub const PLAY_MODE_SIMULATE: EPlayModeType = EPlayModeType(7);
    pub const PLAY_MODE_QUICK_LAUNCH: EPlayModeType = EPlayModeType(8);
    pub const PLAY_MODE_COUNT: EPlayModeType = EPlayModeType(9);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWASDType(pub u8);
impl EWASDType {
    pub const WASD_ALWAYS: EWASDType = EWASDType(0);
    pub const WASD_RMB_ONLY: EWASDType = EWASDType(1);
    pub const WASD_NEVER: EWASDType = EWASDType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeFoliageEditorControlType(pub u8);
impl ELandscapeFoliageEditorControlType {
    pub const IGNORE_CTRL: ELandscapeFoliageEditorControlType = ELandscapeFoliageEditorControlType(
        0,
    );
    pub const REQUIRE_CTRL: ELandscapeFoliageEditorControlType = ELandscapeFoliageEditorControlType(
        1,
    );
    pub const REQUIRE_NO_CTRL: ELandscapeFoliageEditorControlType = ELandscapeFoliageEditorControlType(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EScrollGestureDirection(pub u8);
impl EScrollGestureDirection {
    pub const USE_SYSTEM_SETTING: EScrollGestureDirection = EScrollGestureDirection(0);
    pub const STANDARD: EScrollGestureDirection = EScrollGestureDirection(1);
    pub const NATURAL: EScrollGestureDirection = EScrollGestureDirection(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERotationGridMode(pub u8);
impl ERotationGridMode {
    pub const GRID_MODE_DIVISIONS_OF360: ERotationGridMode = ERotationGridMode(0);
    pub const GRID_MODE_COMMON: ERotationGridMode = ERotationGridMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMarqueeSelectionMode(pub u8);
impl EMarqueeSelectionMode {
    pub const CROSSING: EMarqueeSelectionMode = EMarqueeSelectionMode(0);
    pub const WINDOW: EMarqueeSelectionMode = EMarqueeSelectionMode(1);
    pub const CROSS_LEFT: EMarqueeSelectionMode = EMarqueeSelectionMode(2);
    pub const CROSS_RIGHT: EMarqueeSelectionMode = EMarqueeSelectionMode(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeasuringToolUnits(pub u8);
impl EMeasuringToolUnits {
    pub const MEASURE_UNITS_CENTIMETERS: EMeasuringToolUnits = EMeasuringToolUnits(0);
    pub const MEASURE_UNITS_METERS: EMeasuringToolUnits = EMeasuringToolUnits(1);
    pub const MEASURE_UNITS_KILOMETERS: EMeasuringToolUnits = EMeasuringToolUnits(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMaterialKind(pub u8);
impl EMaterialKind {
    pub const UNKNOWN: EMaterialKind = EMaterialKind(0);
    pub const BASE: EMaterialKind = EMaterialKind(1);
    pub const NORMAL: EMaterialKind = EMaterialKind(2);
    pub const SPECULAR: EMaterialKind = EMaterialKind(3);
    pub const EMISSIVE: EMaterialKind = EMaterialKind(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMaterialStatsDerivedMIOption(pub u8);
impl EMaterialStatsDerivedMIOption {
    pub const IGNORE: EMaterialStatsDerivedMIOption = EMaterialStatsDerivedMIOption(0);
    pub const COMPILE_ONLY: EMaterialStatsDerivedMIOption = EMaterialStatsDerivedMIOption(
        1,
    );
    pub const SHOW_STATS: EMaterialStatsDerivedMIOption = EMaterialStatsDerivedMIOption(
        2,
    );
    pub const INVALID_OR_MAX: EMaterialStatsDerivedMIOption = EMaterialStatsDerivedMIOption(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPhysicsAssetEditorMeshViewMode(pub u8);
impl EPhysicsAssetEditorMeshViewMode {
    pub const SOLID: EPhysicsAssetEditorMeshViewMode = EPhysicsAssetEditorMeshViewMode(
        0,
    );
    pub const WIREFRAME: EPhysicsAssetEditorMeshViewMode = EPhysicsAssetEditorMeshViewMode(
        1,
    );
    pub const NONE: EPhysicsAssetEditorMeshViewMode = EPhysicsAssetEditorMeshViewMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETextureSourceColorSpace(pub i32);
impl ETextureSourceColorSpace {
    pub const AUTO: ETextureSourceColorSpace = ETextureSourceColorSpace(0);
    pub const LINEAR: ETextureSourceColorSpace = ETextureSourceColorSpace(1);
    pub const SRGB: ETextureSourceColorSpace = ETextureSourceColorSpace(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESheetAxis(pub u8);
impl ESheetAxis {
    pub const AX_HORIZONTAL: ESheetAxis = ESheetAxis(0);
    pub const AX_X_AXIS: ESheetAxis = ESheetAxis(1);
    pub const AX_Y_AXIS: ESheetAxis = ESheetAxis(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStructViewerDeveloperType(pub u8);
impl EStructViewerDeveloperType {
    pub const SVDT_NONE: EStructViewerDeveloperType = EStructViewerDeveloperType(0);
    pub const SVDT_CURRENT_USER: EStructViewerDeveloperType = EStructViewerDeveloperType(
        1,
    );
    pub const SVDT_ALL: EStructViewerDeveloperType = EStructViewerDeveloperType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETexAlign(pub u8);
impl ETexAlign {
    pub const TEXALIGN_NONE: ETexAlign = ETexAlign(0);
    pub const TEXALIGN_DEFAULT: ETexAlign = ETexAlign(1);
    pub const TEXALIGN_BOX: ETexAlign = ETexAlign(2);
    pub const TEXALIGN_PLANAR: ETexAlign = ETexAlign(3);
    pub const TEXALIGN_FIT: ETexAlign = ETexAlign(4);
    pub const TEXALIGN_PLANAR_AUTO: ETexAlign = ETexAlign(5);
    pub const TEXALIGN_PLANAR_WALL: ETexAlign = ETexAlign(6);
    pub const TEXALIGN_PLANAR_FLOOR: ETexAlign = ETexAlign(7);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESkeletonDrawMode(pub u8);
impl ESkeletonDrawMode {
    pub const DEFAULT: ESkeletonDrawMode = ESkeletonDrawMode(0);
    pub const HIDDEN: ESkeletonDrawMode = ESkeletonDrawMode(1);
    pub const GREYED_OUT: ESkeletonDrawMode = ESkeletonDrawMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EProcessRootMotionMode(pub u8);
impl EProcessRootMotionMode {
    pub const IGNORE: EProcessRootMotionMode = EProcessRootMotionMode(0);
    pub const LOOP: EProcessRootMotionMode = EProcessRootMotionMode(1);
    pub const LOOP_AND_RESET: EProcessRootMotionMode = EProcessRootMotionMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EVisualizeRootMotionMode(pub u8);
impl EVisualizeRootMotionMode {
    pub const NONE: EVisualizeRootMotionMode = EVisualizeRootMotionMode(0);
    pub const TRAJECTORY: EVisualizeRootMotionMode = EVisualizeRootMotionMode(1);
    pub const TRAJECTORY_AND_ORIENTATION: EVisualizeRootMotionMode = EVisualizeRootMotionMode(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EEditorUserScreenPercentageModeOverride(pub i32);
impl EEditorUserScreenPercentageModeOverride {
    pub const PROJECT_DEFAULT: EEditorUserScreenPercentageModeOverride = EEditorUserScreenPercentageModeOverride(
        0,
    );
    pub const MANUAL: EEditorUserScreenPercentageModeOverride = EEditorUserScreenPercentageModeOverride(
        1,
    );
    pub const BASED_ON_DISPLAY_RESOLUTION: EEditorUserScreenPercentageModeOverride = EEditorUserScreenPercentageModeOverride(
        2,
    );
    pub const BASED_ON_DPI_SCALE: EEditorUserScreenPercentageModeOverride = EEditorUserScreenPercentageModeOverride(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECoordinateSystemPolicy(pub u8);
impl ECoordinateSystemPolicy {
    pub const MATCH_UP_FORWARD_AXES: ECoordinateSystemPolicy = ECoordinateSystemPolicy(
        0,
    );
    pub const MATCH_UP_AXIS: ECoordinateSystemPolicy = ECoordinateSystemPolicy(1);
    pub const KEEP_XYZ_AXES: ECoordinateSystemPolicy = ECoordinateSystemPolicy(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFBXAnimationLengthImportType(pub u8);
impl EFBXAnimationLengthImportType {
    pub const FBXALIT_EXPORTED_TIME: EFBXAnimationLengthImportType = EFBXAnimationLengthImportType(
        0,
    );
    pub const FBXALIT_ANIMATED_KEY: EFBXAnimationLengthImportType = EFBXAnimationLengthImportType(
        1,
    );
    pub const FBXALIT_SET_RANGE: EFBXAnimationLengthImportType = EFBXAnimationLengthImportType(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMovieSceneBakeType(pub u8);
impl EMovieSceneBakeType {
    pub const NONE: EMovieSceneBakeType = EMovieSceneBakeType(0);
    pub const BAKE_CHANNELS: EMovieSceneBakeType = EMovieSceneBakeType(1);
    pub const BAKE_TRANSFORMS: EMovieSceneBakeType = EMovieSceneBakeType(2);
    pub const BAKE_ALL: EMovieSceneBakeType = EMovieSceneBakeType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFbxMaterialBakeMode(pub u8);
impl EFbxMaterialBakeMode {
    pub const DISABLED: EFbxMaterialBakeMode = EFbxMaterialBakeMode(0);
    pub const SIMPLE: EFbxMaterialBakeMode = EFbxMaterialBakeMode(1);
    pub const USE_MESH_DATA: EFbxMaterialBakeMode = EFbxMaterialBakeMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFBXNormalImportMethod(pub u8);
impl EFBXNormalImportMethod {
    pub const FBXNIM_COMPUTE_NORMALS: EFBXNormalImportMethod = EFBXNormalImportMethod(0);
    pub const FBXNIM_IMPORT_NORMALS: EFBXNormalImportMethod = EFBXNormalImportMethod(1);
    pub const FBXNIM_IMPORT_NORMALS_AND_TANGENTS: EFBXNormalImportMethod = EFBXNormalImportMethod(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFBXNormalGenerationMethod(pub u8);
impl EFBXNormalGenerationMethod {
    pub const BUILT_IN: EFBXNormalGenerationMethod = EFBXNormalGenerationMethod(0);
    pub const MIKK_T_SPACE: EFBXNormalGenerationMethod = EFBXNormalGenerationMethod(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFBXSceneOptionsCreateHierarchyType(pub u8);
impl EFBXSceneOptionsCreateHierarchyType {
    pub const FBXSOCHT_CREATE_LEVEL_ACTORS: EFBXSceneOptionsCreateHierarchyType = EFBXSceneOptionsCreateHierarchyType(
        0,
    );
    pub const FBXSOCHT_CREATE_ACTOR_COMPONENTS: EFBXSceneOptionsCreateHierarchyType = EFBXSceneOptionsCreateHierarchyType(
        1,
    );
    pub const FBXSOCHT_CREATE_BLUEPRINT: EFBXSceneOptionsCreateHierarchyType = EFBXSceneOptionsCreateHierarchyType(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFbxSceneVertexColorImportOption(pub u8);
impl EFbxSceneVertexColorImportOption {
    pub const REPLACE: EFbxSceneVertexColorImportOption = EFbxSceneVertexColorImportOption(
        0,
    );
    pub const IGNORE: EFbxSceneVertexColorImportOption = EFbxSceneVertexColorImportOption(
        1,
    );
    pub const OVERRIDE: EFbxSceneVertexColorImportOption = EFbxSceneVertexColorImportOption(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFBXSceneNormalImportMethod(pub u8);
impl EFBXSceneNormalImportMethod {
    pub const FBX_SCENE_NIM_COMPUTE_NORMALS: EFBXSceneNormalImportMethod = EFBXSceneNormalImportMethod(
        0,
    );
    pub const FBX_SCENE_NIM_IMPORT_NORMALS: EFBXSceneNormalImportMethod = EFBXSceneNormalImportMethod(
        1,
    );
    pub const FBX_SCENE_NIM_IMPORT_NORMALS_AND_TANGENTS: EFBXSceneNormalImportMethod = EFBXSceneNormalImportMethod(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFBXSceneNormalGenerationMethod(pub u8);
impl EFBXSceneNormalGenerationMethod {
    pub const BUILT_IN: EFBXSceneNormalGenerationMethod = EFBXSceneNormalGenerationMethod(
        0,
    );
    pub const MIKK_T_SPACE: EFBXSceneNormalGenerationMethod = EFBXSceneNormalGenerationMethod(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFBXImportContentType(pub u8);
impl EFBXImportContentType {
    pub const FBXICT_ALL: EFBXImportContentType = EFBXImportContentType(0);
    pub const FBXICT_GEOMETRY: EFBXImportContentType = EFBXImportContentType(1);
    pub const FBXICT_SKINNING_WEIGHTS: EFBXImportContentType = EFBXImportContentType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EVertexColorImportOption(pub u8);
impl EVertexColorImportOption {
    pub const REPLACE: EVertexColorImportOption = EVertexColorImportOption(0);
    pub const IGNORE: EVertexColorImportOption = EVertexColorImportOption(1);
    pub const OVERRIDE: EVertexColorImportOption = EVertexColorImportOption(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPropertyEditorTestEnum(pub u8);
impl EPropertyEditorTestEnum {
    pub const PROPERTY_EDITOR_TEST_ENUM1: EPropertyEditorTestEnum = EPropertyEditorTestEnum(
        0,
    );
    pub const PROPERTY_EDITOR_TEST_ENUM2: EPropertyEditorTestEnum = EPropertyEditorTestEnum(
        1,
    );
    pub const PROPERTY_EDITOR_TEST_ENUM3: EPropertyEditorTestEnum = EPropertyEditorTestEnum(
        2,
    );
    pub const PROPERTY_EDITOR_TEST_ENUM4: EPropertyEditorTestEnum = EPropertyEditorTestEnum(
        3,
    );
    pub const PROPERTY_EDITOR_TEST_ENUM5: EPropertyEditorTestEnum = EPropertyEditorTestEnum(
        4,
    );
    pub const PROPERTY_EDITOR_TEST_ENUM6: EPropertyEditorTestEnum = EPropertyEditorTestEnum(
        5,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPropertyEditorTestEditColor(pub u8);
impl EPropertyEditorTestEditColor {
    pub const RED: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(0);
    pub const ORANGE: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(1);
    pub const YELLOW: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(2);
    pub const GREEN: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(3);
    pub const BLUE: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(4);
    pub const INDIGO: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(5);
    pub const VIOLET: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(6);
    pub const PINK: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(7);
    pub const MAGENTA: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(8);
    pub const CYAN: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(9);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPropertyEditorTestUnderscores(pub u8);
impl EPropertyEditorTestUnderscores {
    pub const ONE: EPropertyEditorTestUnderscores = EPropertyEditorTestUnderscores(0);
    pub const TWO: EPropertyEditorTestUnderscores = EPropertyEditorTestUnderscores(1);
    pub const THREE: EPropertyEditorTestUnderscores = EPropertyEditorTestUnderscores(2);
    pub const NOT_UNDERSCORE: EPropertyEditorTestUnderscores = EPropertyEditorTestUnderscores(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EReferenceViewerSettingMode(pub u8);
impl EReferenceViewerSettingMode {
    pub const NO_PREFERENCE: EReferenceViewerSettingMode = EReferenceViewerSettingMode(
        0,
    );
    pub const SHOW_BY_DEFAULT: EReferenceViewerSettingMode = EReferenceViewerSettingMode(
        1,
    );
    pub const HIDE_BY_DEFAULT: EReferenceViewerSettingMode = EReferenceViewerSettingMode(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EUnitDisplay(pub u8);
impl EUnitDisplay {
    pub const NONE: EUnitDisplay = EUnitDisplay(0);
    pub const METRIC: EUnitDisplay = EUnitDisplay(1);
    pub const IMPERIAL: EUnitDisplay = EUnitDisplay(2);
    pub const INVALID: EUnitDisplay = EUnitDisplay(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDefaultLocationUnit(pub u8);
impl EDefaultLocationUnit {
    pub const MICROMETERS: EDefaultLocationUnit = EDefaultLocationUnit(0);
    pub const MILLIMETERS: EDefaultLocationUnit = EDefaultLocationUnit(1);
    pub const CENTIMETERS: EDefaultLocationUnit = EDefaultLocationUnit(2);
    pub const METERS: EDefaultLocationUnit = EDefaultLocationUnit(3);
    pub const KILOMETERS: EDefaultLocationUnit = EDefaultLocationUnit(4);
    pub const INCHES: EDefaultLocationUnit = EDefaultLocationUnit(5);
    pub const FEET: EDefaultLocationUnit = EDefaultLocationUnit(6);
    pub const YARDS: EDefaultLocationUnit = EDefaultLocationUnit(7);
    pub const MILES: EDefaultLocationUnit = EDefaultLocationUnit(8);
    pub const INVALID: EDefaultLocationUnit = EDefaultLocationUnit(9);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELevelEditor2DAxis(pub u8);
impl ELevelEditor2DAxis {
    pub const X: ELevelEditor2DAxis = ELevelEditor2DAxis(0);
    pub const Y: ELevelEditor2DAxis = ELevelEditor2DAxis(1);
    pub const Z: ELevelEditor2DAxis = ELevelEditor2DAxis(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFBXTestPlanActionType(pub u8);
impl EFBXTestPlanActionType {
    pub const IMPORT: EFBXTestPlanActionType = EFBXTestPlanActionType(0);
    pub const REIMPORT: EFBXTestPlanActionType = EFBXTestPlanActionType(1);
    pub const ADD_LOD: EFBXTestPlanActionType = EFBXTestPlanActionType(2);
    pub const REIMPORT_LOD: EFBXTestPlanActionType = EFBXTestPlanActionType(3);
    pub const IMPORT_RELOAD: EFBXTestPlanActionType = EFBXTestPlanActionType(4);
    pub const ADD_ALTERNATE_SKINNIG: EFBXTestPlanActionType = EFBXTestPlanActionType(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EThumbnailPrimType(pub u8);
impl EThumbnailPrimType {
    pub const TPT_NONE: EThumbnailPrimType = EThumbnailPrimType(0);
    pub const TPT_SPHERE: EThumbnailPrimType = EThumbnailPrimType(1);
    pub const TPT_CUBE: EThumbnailPrimType = EThumbnailPrimType(2);
    pub const TPT_PLANE: EThumbnailPrimType = EThumbnailPrimType(3);
    pub const TPT_CYLINDER: EThumbnailPrimType = EThumbnailPrimType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EOrthoThumbnailDirection(pub u8);
impl EOrthoThumbnailDirection {
    pub const TOP: EOrthoThumbnailDirection = EOrthoThumbnailDirection(0);
    pub const BOTTOM: EOrthoThumbnailDirection = EOrthoThumbnailDirection(1);
    pub const LEFT: EOrthoThumbnailDirection = EOrthoThumbnailDirection(2);
    pub const RIGHT: EOrthoThumbnailDirection = EOrthoThumbnailDirection(3);
    pub const FRONT: EOrthoThumbnailDirection = EOrthoThumbnailDirection(4);
    pub const BACK: EOrthoThumbnailDirection = EOrthoThumbnailDirection(5);
}
