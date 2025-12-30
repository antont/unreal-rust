#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FNiagaraDataChannelUpdateContext {
    pub reader: UPtr<UNiagaraDataChannelReader>,
    pub first_new_data_index: i32,
    pub last_new_data_index: i32,
    pub new_element_count: i32,
}
#[repr(C, align(4))]
pub struct FNiagaraSimCacheCaptureParameters {
    pub flags_0: u8,
    pub num_frames: i32,
    pub capture_rate: i32,
    pub flags_12: u8,
    pub timeout_frame_count: i32,
    pub flags_20: u8,
    pub immediate_capture_delta_time: f32,
}
#[repr(C, align(8))]
pub struct FNiagaraSimCacheCreateParameters {
    pub attribute_capture_mode: ENiagaraSimCacheAttributeCaptureMode,
    pub flags_4: u8,
    pub rebase_include_attributes: TArray<FName>,
    pub rebase_exclude_attributes: TArray<FName>,
    pub interpolation_include_attributes: TArray<FName>,
    pub interpolation_exclude_attributes: TArray<FName>,
    pub explicit_capture_attributes: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FNDIDynamicMeshSimCacheSection {
    pub triangle_offset: u32,
    pub max_triangles: u32,
    pub allocated_triangles: u32,
    pub material: UPtr<UMaterialInterface>,
}
#[repr(C, align(8))]
pub struct FNDIDynamicMeshSimCacheFrame {
    pub num_triangles: u32,
    pub num_vertices: u32,
    pub num_tex_coords: u32,
    pub local_bounds: FBox,
    pub vertex_buffer_size: u32,
    pub position_offset: u32,
    pub tangent_basis_offset: u32,
    pub tex_coord_offset: u32,
    pub color_offset: u32,
    pub index_data: TArray<u8>,
    pub vertex_data: TArray<u8>,
    pub sections: TArray<FNDIDynamicMeshSimCacheSection>,
}
#[repr(C, align(8))]
pub struct FNiagaraVariableLayoutInfo {
    pub float_component_start: u16,
    pub int32_component_start: u16,
    pub half_component_start: u16,
    pub layout_info: FNiagaraTypeLayoutInfo,
}
#[repr(C, align(8))]
pub struct FNiagaraTypeLayoutInfo {
    pub num_float_components: u16,
    pub num_int32_components: u16,
    pub num_half_components: u16,
    pub components_offsets: TArray<u32>,
}
#[repr(C, align(8))]
pub struct FNiagaraDataSetCompiledData {
    pub variables: TArray<FNiagaraVariableBase>,
    pub variable_layouts: TArray<FNiagaraVariableLayoutInfo>,
    pub id: FNiagaraDataSetID,
    pub total_float_components: u32,
    pub total_int32_components: u32,
    pub total_half_components: u32,
    pub flags_60: u8,
    pub sim_target: ENiagaraSimTarget,
}
#[repr(C, align(4))]
pub struct FNiagaraDataSetID {
    pub name: FName,
    pub ty: ENiagaraDataSetType,
}
#[repr(C, align(8))]
pub struct FNiagaraVariableBase {
    pub name: FName,
    pub type_def_handle: FNiagaraTypeDefinitionHandle,
    pub type_def_deprecated: FNiagaraTypeDefinition,
}
#[repr(C, align(8))]
pub struct FNiagaraTypeDefinition {
    pub class_struct_or_enum: UPtr<UObject>,
    pub underlying_type: u16,
    pub flags: u8,
    pub _struct_deprecated: UPtr<UStruct>,
    pub enum__deprecated: UPtr<UEnum>,
}
#[repr(C, align(4))]
pub struct FNiagaraTypeDefinitionHandle {
    pub registered_type_index: i32,
}
#[repr(C, align(8))]
pub struct FNiagaraMeshRendererMeshPropertiesBase {
    pub mesh: UPtr<UStaticMesh>,
    pub scale: FVector,
    pub rotation: FRotator,
    pub pivot_offset: FVector,
    pub pivot_offset_space: ENiagaraMeshPivotOffsetSpace,
}
#[repr(C, align(8))]
pub struct FNiagaraMeshRendererMeshProperties {
    pub user_param_binding_deprecated: FNiagaraUserParameterBinding,
    pub mesh_parameter_binding: FNiagaraParameterBinding,
    pub lod_mode: ENiagaraMeshLODMode,
    pub lod_level_binding: FNiagaraParameterBindingWithValue,
    pub lod_bias_binding: FNiagaraParameterBindingWithValue,
    pub lod_level: i32,
    pub lod_bias: i32,
    pub lod_distance_factor: f32,
    pub b_use_lod_range: bool,
    pub lod_range: FIntVector2,
}
#[repr(C, align(8))]
pub struct FNiagaraParameterBinding {
    pub resolved_parameter: FNiagaraVariableBase,
    pub aliased_parameter: FNiagaraVariableBase,
    pub allowed_data_interfaces: TArray<TSubclassOf<UObject>>,
    pub allowed_objects: TArray<TSubclassOf<UObject>>,
    pub allowed_interfaces: TArray<TSubclassOf<UObject>>,
}
#[repr(C, align(8))]
pub struct FNiagaraParameterBindingWithValue {
    pub default_value: TArray<u8>,
}
#[repr(C, align(8))]
pub struct FNiagaraUserParameterBinding {
    pub parameter: FNiagaraVariable,
}
#[repr(C, align(8))]
pub struct FNiagaraVariable {
    pub var_data: TArray<u8>,
}
#[repr(C, align(4))]
pub struct FNiagaraScalabilityState {
    pub significance: f32,
    pub last_visible_time: f32,
    pub flags_10: u8,
}
#[repr(C, align(8))]
pub struct FNiagaraStackSection {
    pub section_identifier: FName,
    pub section_display_name: FText,
    pub categories: TArray<FText>,
    pub tooltip: FText,
    pub b_enabled: bool,
}
#[repr(C, align(8))]
pub struct FNDIRenderTargetSimCacheFrame {
    pub size: FIntVector,
    pub format: EPixelFormat,
    pub uncompressed_size: i32,
    pub compressed_size: i32,
}
#[repr(C, align(4))]
pub struct FNDIDistributionIntArrayEntry {
    pub value: i32,
    pub weight: f32,
}
#[repr(C, align(8))]
pub struct FNDIDataChannelFunctionInfo {
    pub function_name: FName,
    pub inputs: TArray<FNiagaraVariableBase>,
    pub outputs: TArray<FNiagaraVariableBase>,
}
#[repr(C, align(8))]
pub struct FNDIDataChannel_GPUScriptParameterAccessInfo {
    pub sorted_parameters: TArray<FNiagaraVariableBase>,
}
#[repr(C, align(8))]
pub struct FNDIDataChannelCompiledData {
    pub function_info: TArray<FNDIDataChannelFunctionInfo>,
    pub gpu_script_parameter_infos: TMap<
        FNiagaraCompileHash,
        FNDIDataChannel_GPUScriptParameterAccessInfo,
    >,
    pub total_params: u32,
    pub b_used_by_cpu: bool,
    pub b_used_by_gpu: bool,
    pub b_needs_spawn_data_table: bool,
    pub b_spawns_particles: bool,
    pub b_calls_write: bool,
}
#[repr(C, align(8))]
pub struct FNDIDataChannelWriteCompiledData {
    pub data_layout: FNiagaraDataSetCompiledData,
}
#[repr(C, align(8))]
pub struct FNDIDataChannelWriteSimCacheFrameBuffer {
    pub data: TArray<u8>,
    pub size: i32,
    pub source_var: FNiagaraVariableBase,
}
#[repr(C, align(8))]
pub struct FNDIDataChannelWriteSimCacheFrame {
    pub num_elements: i32,
    pub variable_data: TArray<FNDIDataChannelWriteSimCacheFrameBuffer>,
    pub b_visible_to_game: bool,
    pub b_visible_to_cpu_sims: bool,
    pub b_visible_to_gpu_sims: bool,
}
#[repr(C, align(4))]
pub struct FNiagaraDynamicMeshSection {
    pub num_triangles: i32,
    pub material_index: i32,
}
#[repr(C, align(8))]
pub struct FNiagaraDynamicMeshMaterial {
    pub material: UPtr<UMaterialInterface>,
    pub material_user_param_binding: FNiagaraUserParameterBinding,
}
#[repr(C, align(4))]
pub struct FNDIMemoryBufferSimCacheDataFrame {
    pub cpu_buffer_size: i32,
    pub cpu_data_offset: i32,
    pub gpu_buffer_size: i32,
    pub gpu_data_offset: i32,
}
#[repr(C, align(8))]
pub struct FNDIStaticMeshSectionFilter {
    pub allowed_material_slots: TArray<i32>,
}
#[repr(C, align(4))]
pub struct FNiagaraUObjectPropertyReaderRemap {
    pub graph_name: FName,
    pub remap_name: FName,
}
#[repr(C, align(8))]
pub struct FMovieSceneNiagaraSystemTrackTemplate {}
#[repr(C, align(8))]
pub struct FMovieSceneNiagaraSystemTrackImplementation {
    pub spawn_section_start_frame: FFrameNumber,
    pub spawn_section_end_frame: FFrameNumber,
    pub spawn_section_start_behavior: ENiagaraSystemSpawnSectionStartBehavior,
    pub spawn_section_evaluate_behavior: ENiagaraSystemSpawnSectionEvaluateBehavior,
    pub spawn_section_end_behavior: ENiagaraSystemSpawnSectionEndBehavior,
    pub age_update_mode: ENiagaraAgeUpdateMode,
    pub b_allow_scalability: bool,
}
#[repr(C, align(8))]
pub struct FMovieSceneNiagaraParameterSectionTemplate {
    pub parameter: FNiagaraVariable,
}
#[repr(C, align(8))]
pub struct FMovieSceneNiagaraBoolParameterSectionTemplate {
    pub bool_channel: FMovieSceneBoolChannel,
}
#[repr(C, align(8))]
pub struct FMovieSceneNiagaraColorParameterSectionTemplate {
    pub red_channel: FMovieSceneFloatChannel,
    pub green_channel: FMovieSceneFloatChannel,
    pub blue_channel: FMovieSceneFloatChannel,
    pub alpha_channel: FMovieSceneFloatChannel,
}
#[repr(C, align(8))]
pub struct FMovieSceneNiagaraFloatParameterSectionTemplate {
    pub float_channel: FMovieSceneFloatChannel,
}
#[repr(C, align(8))]
pub struct FMovieSceneNiagaraIntegerParameterSectionTemplate {
    pub integer_channel: FMovieSceneIntegerChannel,
}
#[repr(C, align(8))]
pub struct FMovieSceneNiagaraVectorParameterSectionTemplate {
    pub vector_channels: FMovieSceneFloatChannel,
    pub channels_used: i32,
}
#[repr(C, align(8))]
pub struct FNiagaraAssetTagDefinition {
    pub asset_tag: FText,
    pub asset_flags: i32,
    pub description: FText,
    pub display_type: ENiagaraAssetTagDefinitionImportance,
    pub color: FLinearColor,
    pub tag_guid: FGuid,
}
#[repr(C, align(4))]
pub struct FNiagaraAssetTagDefinitionReference {
    pub asset_tag_definition_guid: FGuid,
}
#[repr(C, align(8))]
pub struct FEmitterCompiledScriptPair {}
#[repr(C, align(8))]
pub struct FNiagaraBakerTextureSource {
    pub display_string: FString,
    pub source_name: FName,
}
#[repr(C, align(8))]
pub struct FNiagaraBakerCameraSettings {
    pub view_mode: ENiagaraBakerViewMode,
    pub viewport_location: FVector,
    pub viewport_rotation: FRotator,
    pub orbit_distance: f32,
    pub fov: f32,
    pub ortho_width: f32,
    pub b_use_aspect_ratio: bool,
    pub aspect_ratio: f32,
}
#[repr(C, align(8))]
pub struct FNiagaraBakerTextureSettings {
    pub output_name: FName,
    pub source_binding: FNiagaraBakerTextureSource,
    pub flags_48: u8,
    pub frame_size: FIntPoint,
    pub texture_size: FIntPoint,
    pub generated_texture: UPtr<UTexture2D>,
}
#[repr(C, align(8))]
pub struct FNiagaraDataSetProperties {
    pub id: FNiagaraDataSetID,
    pub variables: TArray<FNiagaraVariableBase>,
}
#[repr(C, align(1))]
pub struct FNiagaraScriptDataUsageInfo {
    pub b_reads_attribute_data: bool,
}
#[repr(C, align(8))]
pub struct FNiagaraFunctionSignature {
    pub name: FName,
    pub inputs: TArray<FNiagaraVariable>,
    pub outputs: TArray<FNiagaraVariableBase>,
    pub owner_name: FName,
    pub flags_60: u8,
    pub experimental_message: FText,
    pub function_version: u32,
    pub no_default_value_inputs: TArray<FNiagaraVariableBase>,
    pub flags_104: u8,
    pub module_usage_bitmask: i32,
    pub misc_usage_bit_mask: u16,
    pub context_stage_index: i32,
    pub required_inputs: i16,
    pub required_outputs: i16,
    pub function_specifiers: TMap<FName, FName>,
    pub description: FText,
    pub input_descriptions: TMap<FNiagaraVariableBase, FText>,
    pub output_descriptions: TMap<FNiagaraVariableBase, FText>,
    pub source_file: FString,
    pub source_line: i32,
}
#[repr(C, align(8))]
pub struct FNiagaraScriptUObjectCompileInfo {
    pub variable: FNiagaraVariableBase,
    pub object: UPtr<UObject>,
    pub object_path: FSoftObjectPath,
    pub registered_parameter_map_read: FName,
    pub registered_parameter_map_writes: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FNiagaraResolvedUObjectInfo {
    pub read_variable_name: FName,
    pub resolved_variable: FNiagaraVariableBase,
    pub object: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FNiagaraExternalUObjectInfo {
    pub variable: FNiagaraVariableBase,
    pub external_name: FName,
}
#[repr(C, align(8))]
pub struct FNiagaraScriptDataInterfaceInfo {
    pub data_interface: UPtr<UNiagaraDataInterface>,
    pub name: FName,
    pub compile_name: FName,
    pub user_ptr_idx: i32,
    pub ty: FNiagaraTypeDefinition,
    pub registered_parameter_map_read: FName,
    pub registered_parameter_map_write: FName,
    pub source_emitter_name: FString,
}
#[repr(C, align(4))]
pub struct FNiagaraResolvedUserDataInterfaceBinding {
    pub user_parameter_store_data_interface_index: i32,
    pub script_parameter_store_data_interface_index: i32,
}
#[repr(C, align(8))]
pub struct FNiagaraScriptResolvedDataInterfaceInfo {
    pub name: FName,
    pub compile_name: FName,
    pub resolved_source_emitter_name: FString,
    pub resolved_variable: FNiagaraVariableBase,
    pub parameter_store_variable: FNiagaraVariableBase,
    pub b_is_internal: bool,
    pub resolved_data_interface: UPtr<UNiagaraDataInterface>,
    pub user_ptr_idx: i32,
}
#[repr(C, align(8))]
pub struct FNiagaraScriptDataInterfaceCompileInfo {
    pub name: FName,
    pub user_ptr_idx: i32,
    pub ty: FNiagaraTypeDefinition,
    pub registered_functions: TArray<FNiagaraFunctionSignature>,
    pub registered_parameter_map_read: FName,
    pub registered_parameter_map_write: FName,
    pub b_is_placeholder: bool,
    pub source_emitter_name: FString,
}
#[repr(C, align(4))]
pub struct FNiagaraStatScope {
    pub full_name: FName,
    pub friendly_name: FName,
}
#[repr(C, align(4))]
pub struct FVMFunctionSpecifier {
    pub key: FName,
    pub value: FName,
}
#[repr(C, align(8))]
pub struct FVMExternalFunctionBindingInfo {
    pub name: FName,
    pub owner_name: FName,
    pub input_param_locations: TArray<bool>,
    pub num_outputs: i32,
    pub function_specifiers: TArray<FVMFunctionSpecifier>,
    pub variadic_inputs: TArray<FNiagaraVariableBase>,
    pub variadic_outputs: TArray<FNiagaraVariableBase>,
    pub specifiers_deprecated: TMap<FName, FName>,
}
#[repr(C, align(8))]
pub struct FNiagaraSystemUpdateContext {
    pub components_to_reset: TArray<UPtr<UNiagaraComponent>>,
    pub components_to_re_init: TArray<UPtr<UNiagaraComponent>>,
    pub components_to_notify_sim_destroy: TArray<UPtr<UNiagaraComponent>>,
    pub components_to_destroy_instance: TArray<UPtr<UNiagaraComponent>>,
    pub system_sims_to_destroy: TArray<UPtr<UNiagaraSystem>>,
    pub system_sims_to_recache: TArray<UPtr<UNiagaraSystem>>,
}
#[repr(C, align(8))]
pub struct FNiagaraVariableInfo {
    pub variable: FNiagaraVariable,
    pub definition: FText,
    pub data_interface: UPtr<UNiagaraDataInterface>,
}
#[repr(C, align(8))]
pub struct FNiagaraVariableAttributeBinding {
    pub root_name: FName,
    pub param_map_variable: FNiagaraVariableBase,
    pub data_set_name: FName,
    pub bound_variable: FNiagaraVariable,
    pub default_value: FNiagaraVariable,
    pub cached_display_name: FName,
    pub binding_source_mode: ENiagaraBindingSource,
    pub flags_245: u8,
    pub data_set_variable_deprecated: FNiagaraVariable,
    pub root_variable_deprecated: FNiagaraVariable,
}
#[repr(C, align(8))]
pub struct FNiagaraMaterialAttributeBinding {
    pub material_parameter_name: FName,
    pub niagara_variable: FNiagaraVariableBase,
    pub resolved_niagara_variable: FNiagaraVariableBase,
    pub niagara_child_variable: FNiagaraVariableBase,
}
#[repr(C, align(8))]
pub struct FNiagaraVariableDataInterfaceBinding {
    pub bound_variable: FNiagaraVariable,
}
#[repr(C, align(4))]
pub struct FNiagaraScriptVariableBinding {
    pub name: FName,
}
#[repr(C, align(8))]
pub struct FNiagaraCompileDependency {
    pub linker_error_message: FString,
    pub node_guid: FGuid,
    pub pin_guid: FGuid,
    pub stack_guids: TArray<FGuid>,
    pub dependent_variable: FNiagaraVariableBase,
    pub b_dependent_variable_from_custom_iteration_namespace: bool,
}
#[repr(C, align(8))]
pub struct FNiagaraScriptAsyncCompileData {
    pub rapid_iteration_parameters: TArray<FNiagaraVariable>,
    pub named_data_interfaces: TMap<FName, UPtr<UNiagaraDataInterface>>,
}
#[repr(C, align(8))]
pub struct FNiagaraSystemAsyncCompileResults {
    pub root_objects: TArray<UPtr<UObject>>,
    pub exposed_variables: TArray<FNiagaraVariable>,
}
#[repr(C, align(8))]
pub struct FNiagaraCompileHashVisitorDebugInfo {
    pub object: FString,
    pub property_keys: TArray<FString>,
    pub property_values: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FNCPoolElement {
    pub component: UPtr<UNiagaraComponent>,
}
#[repr(C, align(8))]
pub struct FNCPool {
    pub free_elements: TArray<FNCPoolElement>,
}
#[repr(C, align(8))]
pub struct FNiagaraComponentPropertyBinding {
    pub attribute_binding: FNiagaraVariableAttributeBinding,
    pub property_name: FName,
    pub property_type: FNiagaraTypeDefinition,
    pub metadata_setter_name: FName,
    pub property_setter_parameter_defaults: TMap<FString, FString>,
}
#[repr(C, align(4))]
pub struct FNiagaraCulledComponentInfo {}
#[repr(C, align(8))]
pub struct FNiagaraDataChannelSearchParameters {
    pub owning_component: UPtr<USceneComponent>,
    pub location: FVector,
    pub flags_32: u8,
}
#[repr(C, align(8))]
pub struct FNDCAccessContextBase {
    pub owning_component: UPtr<USceneComponent>,
    pub flags_8: u8,
}
#[repr(C, align(4))]
pub struct FNDCSpawnedSystemRef {
    pub spawned_system: TWeakObjectPtr<UNiagaraComponent>,
}
#[repr(C, align(8))]
pub struct FNDCAccessContext {
    pub location: FVector,
    pub flags_40: u8,
    pub system_to_spawn: UPtr<UObject>,
    pub spawned_systems: TArray<FNDCSpawnedSystemRef>,
}
#[repr(C, align(8))]
pub struct FNDCAccessContextLegacy {
    pub location: FVector,
    pub flags_40: u8,
}
#[repr(C, align(8))]
pub struct FNDCAccessContextInst {
    pub access_context: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraDataChannelGameDataLayout {
    pub variable_indices: TMap<FNiagaraVariableBase, i32>,
    pub lwc_converters: TArray<FNiagaraLwcStructConverter>,
}
#[repr(C, align(8))]
pub struct FNiagaraLwcStructConverter {
    pub lwc_size: i32,
    pub swc_size: i32,
    pub conversion_steps: TArray<FNiagaraStructConversionStep>,
}
#[repr(C, align(4))]
pub struct FNiagaraStructConversionStep {
    pub lwc_bytes: i32,
    pub lwc_offset: i32,
    pub simulation_bytes: i32,
    pub simulation_offset: i32,
    pub conversion_type: ENiagaraStructConversionType,
}
#[repr(C, align(8))]
pub struct FNiagaraDataChannelReference {
    pub data_channel: UPtr<UNiagaraDataChannelAsset>,
    pub access_context: FNDCAccessContextInst,
    pub b_custom_access_context: bool,
}
#[repr(C, align(8))]
pub struct FNiagaraDataChannelVariable {
    pub version: FGuid,
}
#[repr(C, align(8))]
pub struct FNDCAccessContext_MapBase {}
#[repr(C, align(8))]
pub struct FNDCAccessContext_GameplayBurst {
    pub flags_72: u8,
    pub cell_size_override: FVector,
    pub system_bounds_padding: FVector,
    pub gameplay_tag: FGameplayTag,
}
#[repr(C, align(8))]
pub struct FNDCGameplayBurstAttachmentSettings {
    pub speed_threshold: f32,
    pub gameplay_tags: FGameplayTagContainer,
    pub component_types: TArray<TSubclassOf<USceneComponent>>,
}
#[repr(C, align(8))]
pub struct FNDCMapEntryBase {
    pub owner: UPtr<UNiagaraDataChannelHandler_MapBase>,
    pub spawned_components: TArray<UPtr<UNiagaraComponent>>,
    pub last_used_time: f32,
}
#[repr(C, align(8))]
pub struct FNDCMapEntry_GameplayBurst {}
#[repr(C, align(8))]
pub struct FNDCIsland {
    pub owner: UPtr<UNiagaraDataChannelHandler_Islands>,
    pub bounds: FBoxSphereBounds,
    pub spawned_components: TArray<UPtr<UNiagaraComponent>>,
}
#[repr(C, align(4))]
pub struct FNDCIslandDebugDrawSettings {
    pub flags_0: u8,
}
#[repr(C, align(8))]
pub struct FNDCMapEntry {
    pub entry_data: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNDCMapKey {}
#[repr(C, align(4))]
pub struct FNDIArraySimCacheDataFrame {
    pub num_elements: i32,
    pub data_offset: i32,
}
#[repr(C, align(4))]
pub struct FNiagaraDataInterfaceEmitterBinding {
    pub binding_mode: ENiagaraDataInterfaceEmitterBindingMode,
    pub emitter_name: FName,
}
#[repr(C, align(8))]
pub struct FBasicParticleData {
    pub position: FVector,
    pub size: f32,
    pub velocity: FVector,
}
#[repr(C, align(4))]
pub struct FMeshTriCoordinate {
    pub tri: i32,
    pub bary_coord: FVector3f,
}
#[repr(C, align(8))]
pub struct FNiagaraDataInterfaceSplineLUT {
    pub positions: TArray<FVector>,
    pub scales: TArray<FVector>,
    pub rotations: TArray<FQuat>,
    pub spline_length: f32,
    pub spline_distance_step: f32,
    pub inv_spline_distance_step: f32,
    pub max_index: i32,
}
#[repr(C, align(4))]
pub struct FNiagaraOutlinerTimingData {
    pub game_thread: f32,
    pub render_thread: f32,
}
#[repr(C, align(8))]
pub struct FNiagaraOutlinerEmitterInstanceData {
    pub emitter_name: FString,
    pub sim_target: ENiagaraSimTarget,
    pub exec_state: ENiagaraExecutionState,
    pub num_particles: i32,
    pub flags_28: u8,
}
#[repr(C, align(8))]
pub struct FNiagaraOutlinerSystemInstanceData {
    pub component_name: FString,
    pub lwc_tile: FVector3f,
    pub emitters: TArray<FNiagaraOutlinerEmitterInstanceData>,
    pub actual_execution_state: ENiagaraExecutionState,
    pub requested_execution_state: ENiagaraExecutionState,
    pub scalability_state: FNiagaraScalabilityState,
    pub flags_68: u8,
    pub pool_method: ENCPoolMethod,
    pub average_time: FNiagaraOutlinerTimingData,
    pub max_time: FNiagaraOutlinerTimingData,
    pub tick_group: ETickingGroup,
    pub gpu_tick_stage: ENiagaraGpuComputeTickStage,
    pub flags_96: u8,
}
#[repr(C, align(8))]
pub struct FNiagaraOutlinerSystemData {
    pub system_instances: TArray<FNiagaraOutlinerSystemInstanceData>,
    pub average_per_frame_time: FNiagaraOutlinerTimingData,
    pub max_per_frame_time: FNiagaraOutlinerTimingData,
    pub average_per_instance_time: FNiagaraOutlinerTimingData,
    pub max_per_instance_time: FNiagaraOutlinerTimingData,
}
#[repr(C, align(8))]
pub struct FNiagaraOutlinerWorldData {
    pub systems: TMap<FString, FNiagaraOutlinerSystemData>,
    pub b_has_begun_play: bool,
    pub world_type: u8,
    pub net_mode: u8,
    pub average_per_frame_time: FNiagaraOutlinerTimingData,
    pub max_per_frame_time: FNiagaraOutlinerTimingData,
}
#[repr(C, align(8))]
pub struct FNiagaraOutlinerData {
    pub world_data: TMap<FString, FNiagaraOutlinerWorldData>,
}
#[repr(C, align(4))]
pub struct FNiagaraDebuggerRequestConnection {
    pub session_id: FGuid,
    pub instance_id: FGuid,
}
#[repr(C, align(4))]
pub struct FNiagaraDebuggerAcceptConnection {
    pub session_id: FGuid,
    pub instance_id: FGuid,
}
#[repr(C, align(4))]
pub struct FNiagaraDebuggerConnectionClosed {
    pub session_id: FGuid,
    pub instance_id: FGuid,
}
#[repr(C, align(8))]
pub struct FNiagaraDebuggerExecuteConsoleCommand {
    pub command: FString,
    pub b_requires_world: bool,
}
#[repr(C, align(8))]
pub struct FNiagaraDebuggerOutlinerUpdate {
    pub outliner_data: FNiagaraOutlinerData,
}
#[repr(C, align(8))]
pub struct FNiagaraDebugHudTextOptions {
    pub font: ENiagaraDebugHudFont,
    pub horizontal_alignment: ENiagaraDebugHudHAlign,
    pub vertical_alignment: ENiagaraDebugHudVAlign,
    pub screen_offset: FVector2D,
}
#[repr(C, align(8))]
pub struct FNiagaraDebugHUDVariable {
    pub b_enabled: bool,
    pub name: FString,
}
#[repr(C, align(8))]
pub struct FNiagaraDebugHUDSettingsData {
    pub b_widget_enabled: bool,
    pub b_hud_enabled: bool,
    pub b_hud_rendering_enabled: bool,
    pub b_validation_enabled: bool,
    pub b_overview_enabled: bool,
    pub overview_mode: ENiagaraDebugHUDOverviewMode,
    pub overview_sort_mode: ENiagaraDebugHUDDOverviewSort,
    pub b_include_cascade: bool,
    pub b_show_registered_components: bool,
    pub b_overview_show_filtered_system_only: bool,
    pub b_show_global_budget_info: bool,
    pub b_system_filter_enabled: bool,
    pub system_filter: FString,
    pub b_emitter_filter_enabled: bool,
    pub emitter_filter: FString,
    pub b_actor_filter_enabled: bool,
    pub actor_filter: FString,
    pub b_component_filter_enabled: bool,
    pub component_filter: FString,
    pub b_validate_system_simulation_data_buffers: bool,
    pub b_validate_particle_data_buffers: bool,
    pub b_validation_log_errors: bool,
    pub validation_attribute_display_truncate: i32,
    pub system_debug_verbosity: ENiagaraDebugHudVerbosity,
    pub system_emitter_verbosity: ENiagaraDebugHudVerbosity,
    pub data_interface_verbosity: ENiagaraDebugHudVerbosity,
    pub system_variables: TArray<FNiagaraDebugHUDVariable>,
    pub b_system_show_active_only_in_world: bool,
    pub b_show_particle_variables: bool,
    pub particles_variables: TArray<FNiagaraDebugHUDVariable>,
    pub b_enable_gpu_particle_readback: bool,
    pub b_show_particle_index: bool,
    pub b_show_particles_variables_with_system: bool,
    pub b_show_particle_variables_vertical: bool,
    pub b_use_max_particles_to_display: bool,
    pub max_particles_to_display: i32,
    pub b_use_particle_display_clip: bool,
    pub particle_display_clip: FVector2D,
    pub b_use_particle_display_center_radius: bool,
    pub particle_display_center_radius: f32,
    pub perf_report_frames: i32,
    pub perf_sample_mode: ENiagaraDebugHUDPerfSampleMode,
    pub perf_units: ENiagaraDebugHUDPerfUnits,
    pub b_show_perf_colum_game_thread_only: bool,
    pub perf_graph_mode: ENiagaraDebugHUDPerfGraphMode,
    pub perf_history_frames: i32,
    pub b_use_perf_graph_time_range: bool,
    pub perf_graph_time_range: f32,
    pub perf_graph_size: FVector2D,
    pub perf_graph_axis_color: FLinearColor,
    pub smoothing_width: i32,
    pub overview_font: ENiagaraDebugHudFont,
    pub overview_location: FVector2D,
    pub system_text_options: FNiagaraDebugHudTextOptions,
    pub particle_text_options: FNiagaraDebugHudTextOptions,
    pub b_draw_bounds_enabled: bool,
    pub b_draw_bounds_wireframe: bool,
    pub draw_bounds_alpha: f32,
    pub default_background_color: FLinearColor,
    pub overview_heading_color: FLinearColor,
    pub overview_detail_color: FLinearColor,
    pub overview_detail_highlight_color: FLinearColor,
    pub in_world_error_text_color: FLinearColor,
    pub in_world_text_color: FLinearColor,
    pub message_info_text_color: FLinearColor,
    pub message_warning_text_color: FLinearColor,
    pub message_error_text_color: FLinearColor,
    pub system_color_table_opacity: f32,
    pub system_color_seed: u32,
    pub system_color_hsv_min: FVector,
    pub system_color_hsv_max: FVector,
    pub playback_mode: ENiagaraDebugPlaybackMode,
    pub b_playback_rate_enabled: bool,
    pub playback_rate: f32,
    pub b_loop_time_enabled: bool,
    pub loop_time: f32,
}
#[repr(C, align(1))]
pub struct FNiagaraRequestSimpleClientInfoMessage {}
#[repr(C, align(4))]
pub struct FNiagaraOutlinerCaptureSettings {
    pub b_trigger_capture: bool,
    pub capture_delay_frames: u32,
    pub b_gather_perf_data: bool,
    pub sim_cache_capture_frames: u32,
}
#[repr(C, align(8))]
pub struct FNiagaraSimpleClientInfo {
    pub systems: TArray<FString>,
    pub actors: TArray<FString>,
    pub components: TArray<FString>,
    pub emitters: TArray<FString>,
}
#[repr(C, align(4))]
pub struct FNiagaraSystemSimCacheCaptureRequest {
    pub component_name: FName,
    pub capture_delay_frames: u32,
    pub capture_frames: u32,
}
#[repr(C, align(8))]
pub struct FNiagaraSystemSimCacheCaptureReply {
    pub component_name: FName,
    pub sim_cache_data: TArray<u8>,
}
#[repr(C, align(4))]
pub struct FNiagaraGraphViewSettings {
    pub location: FDeprecateSlateVector2D,
    pub zoom: f32,
    pub b_is_valid: bool,
}
#[repr(C, align(4))]
pub struct FNiagaraLinearRamp {
    pub start_x: f32,
    pub start_y: f32,
    pub end_x: f32,
    pub end_y: f32,
}
#[repr(C, align(4))]
pub struct FNiagaraGlobalBudgetScaling {
    pub flags_0: u8,
    pub max_global_budget_usage: f32,
    pub max_distance_scale_by_global_budget_use: FNiagaraLinearRamp,
    pub max_instance_count_scale_by_global_budget_use: FNiagaraLinearRamp,
    pub max_system_instance_count_scale_by_global_budget_use: FNiagaraLinearRamp,
}
#[repr(C, align(4))]
pub struct FNiagaraSystemVisibilityCullingSettings {
    pub flags_0: u8,
    pub max_time_outside_view_frustum: f32,
    pub max_time_without_render: f32,
}
#[repr(C, align(8))]
pub struct FNiagaraSystemScalabilitySettings {
    pub platforms: FNiagaraPlatformSet,
    pub flags_96: u8,
    pub max_distance: f32,
    pub flags_104: u8,
    pub max_instances: i32,
    pub max_system_instances: i32,
    pub max_time_without_render_deprecated: f32,
    pub cull_proxy_mode: ENiagaraCullProxyMode,
    pub max_system_proxies: i32,
    pub visibility_culling: FNiagaraSystemVisibilityCullingSettings,
    pub budget_scaling: FNiagaraGlobalBudgetScaling,
}
#[repr(C, align(8))]
pub struct FNiagaraPlatformSet {
    pub device_profile_states: TArray<FNiagaraDeviceProfileStateEntry>,
    pub c_var_conditions: TArray<FNiagaraPlatformSetCVarCondition>,
    pub quality_level_mask: i32,
}
#[repr(C, align(8))]
pub struct FNiagaraPlatformSetCVarCondition {
    pub c_var_name: FName,
    pub pass_response: ENiagaraCVarConditionResponse,
    pub fail_response: ENiagaraCVarConditionResponse,
    pub value: bool,
    pub min_int: i32,
    pub max_int: i32,
    pub min_float: f32,
    pub max_float: f32,
    pub flags_32: u8,
}
#[repr(C, align(4))]
pub struct FNiagaraDeviceProfileStateEntry {
    pub profile_name: FName,
    pub quality_level_mask: u32,
    pub set_quality_level_mask: u32,
}
#[repr(C, align(8))]
pub struct FNiagaraSystemScalabilitySettingsArray {
    pub settings: TArray<FNiagaraSystemScalabilitySettings>,
}
#[repr(C, align(8))]
pub struct FNiagaraSystemScalabilityOverride {
    pub flags_200: u8,
}
#[repr(C, align(8))]
pub struct FNiagaraEmitterScalabilitySettings {
    pub platforms: FNiagaraPlatformSet,
    pub flags_96: u8,
    pub spawn_count_scale: f32,
}
#[repr(C, align(8))]
pub struct FNiagaraEmitterScalabilitySettingsArray {
    pub settings: TArray<FNiagaraEmitterScalabilitySettings>,
}
#[repr(C, align(8))]
pub struct FNiagaraEmitterScalabilityOverride {
    pub flags_104: u8,
}
#[repr(C, align(8))]
pub struct FNiagaraEmitterScalabilityOverrides {
    pub overrides: TArray<FNiagaraEmitterScalabilityOverride>,
}
#[repr(C, align(4))]
pub struct FNiagaraEventReceiverProperties {
    pub name: FName,
    pub source_event_generator: FName,
    pub source_emitter: FName,
}
#[repr(C, align(8))]
pub struct FNiagaraEventGeneratorProperties {
    pub max_events_per_frame: i32,
    pub id: FName,
    pub data_set_compiled_data: FNiagaraDataSetCompiledData,
}
#[repr(C, align(8))]
pub struct FNiagaraEmitterScriptProperties {
    pub script: UPtr<UNiagaraScript>,
    pub event_receivers: TArray<FNiagaraEventReceiverProperties>,
    pub event_generators: TArray<FNiagaraEventGeneratorProperties>,
}
#[repr(C, align(8))]
pub struct FNiagaraEventScriptProperties {
    pub execution_mode: EScriptExecutionMode,
    pub spawn_number: u32,
    pub max_events_per_frame: u32,
    pub source_emitter_id: FGuid,
    pub source_event_name: FName,
    pub b_random_spawn_number: bool,
    pub min_spawn_number: u32,
    pub update_attribute_initial_values: bool,
}
#[repr(C, align(4))]
pub struct FNiagaraDetailsLevelScaleOverrides {
    pub low: f32,
    pub medium: f32,
    pub high: f32,
    pub epic: f32,
    pub cine: f32,
}
#[repr(C, align(8))]
pub struct FVersionedNiagaraEmitterData {
    pub version: FNiagaraAssetVersion,
    pub version_change_description: FText,
    pub update_script_execution: ENiagaraPythonUpdateScriptReference,
    pub python_update_script: FString,
    pub script_asset: FFilePath,
    pub b_deprecated: bool,
    pub deprecation_message: FText,
    pub b_local_space: bool,
    pub b_determinism: bool,
    pub random_seed: i32,
    pub interpolated_spawn_mode: ENiagaraInterpolatedSpawnMode,
    pub flags_124: u8,
    pub sim_target: ENiagaraSimTarget,
    pub calculate_bounds_mode: ENiagaraEmitterCalculateBoundMode,
    pub fixed_bounds: FBox,
    pub flags_192: u8,
    pub event_handler_script_props: TArray<FNiagaraEventScriptProperties>,
    pub platforms: FNiagaraPlatformSet,
    pub scalability_overrides: FNiagaraEmitterScalabilityOverrides,
    pub max_gpu_particles_spawn_per_frame: i32,
    pub allocation_mode: EParticleAllocationMode,
    pub pre_allocation_count: i32,
    pub emitter_dependencies: TArray<FNiagaraDataInterfaceEmitterBinding>,
    pub update_script_props: FNiagaraEmitterScriptProperties,
    pub spawn_script_props: FNiagaraEmitterScriptProperties,
    pub renderer_bindings: FNiagaraParameterStore,
    pub renderer_bindings_external_objects: TArray<FNiagaraExternalUObjectInfo>,
    pub resolved_di_bindings: TMap<FNiagaraVariableBase, FNiagaraVariableBase>,
    pub attributes_to_preserve: TArray<FString>,
    pub add_emitter_default_view_state: ENiagaraEmitterDefaultSummaryState,
    pub emitter_spawn_script_props: FNiagaraEmitterScriptProperties,
    pub emitter_update_script_props: FNiagaraEmitterScriptProperties,
    pub graph_source: UPtr<UNiagaraScriptSourceBase>,
    pub scratch_pads: UPtr<UNiagaraScratchPadContainer>,
    pub parent_scratch_pads: UPtr<UNiagaraScratchPadContainer>,
    pub versioned_parent: FVersionedNiagaraEmitter,
    pub versioned_parent_at_last_merge: FVersionedNiagaraEmitter,
    pub renderer_properties: TArray<UPtr<UNiagaraRendererProperties>>,
    pub simulation_stages: TArray<UPtr<UNiagaraSimulationStageBase>>,
    pub sim_stage_execution_loops: TArray<FNiagaraSimStageExecutionLoopData>,
    pub sim_stage_execution_loop_editor_data: TArray<
        FNiagaraSimStageExecutionLoopEditorData,
    >,
    pub gpu_compute_script: UPtr<UNiagaraScript>,
    pub shared_event_generator_ids: TArray<FName>,
    pub current_scalability_settings: FNiagaraEmitterScalabilitySettings,
    pub editor_data: UPtr<UNiagaraEditorDataBase>,
    pub editor_parameters: UPtr<UNiagaraEditorParametersAdapterBase>,
}
#[repr(C, align(4))]
pub struct FNiagaraSimStageExecutionLoopEditorData {
    pub b_enabled: bool,
    pub num_loops_binding_name: FName,
    pub num_loops: i32,
    pub stage_name_start: FName,
    pub stage_name_end: FName,
}
#[repr(C, align(4))]
pub struct FNiagaraSimStageExecutionLoopData {
    pub num_loops_binding: FName,
    pub num_loops: i32,
    pub start_stage_index: i32,
    pub end_stage_index: i32,
}
#[repr(C, align(8))]
pub struct FVersionedNiagaraEmitter {
    pub emitter: UPtr<UNiagaraEmitter>,
    pub version: FGuid,
}
#[repr(C, align(8))]
pub struct FNiagaraParameterStore {
    pub owner: TWeakObjectPtr<UObject>,
    pub parameter_offsets: TMap<FNiagaraVariable, i32>,
    pub sorted_parameter_offsets: TArray<FNiagaraVariableWithOffset>,
    pub parameter_data: TArray<u8>,
    pub data_interfaces: TArray<UPtr<UNiagaraDataInterface>>,
    pub u_objects: TArray<UPtr<UObject>>,
    pub original_position_data: TArray<FNiagaraPositionSource>,
    pub debug_name: FString,
    pub parameter_guid_mapping: TMap<FNiagaraVariable, FGuid>,
}
#[repr(C, align(8))]
pub struct FNiagaraPositionSource {
    pub name: FName,
    pub value: FVector,
}
#[repr(C, align(8))]
pub struct FNiagaraVariableWithOffset {
    pub offset: i32,
    pub struct_converter: FNiagaraLwcStructConverter,
}
#[repr(C, align(4))]
pub struct FNiagaraAssetVersion {
    pub major_version: i32,
    pub minor_version: i32,
    pub version_guid: FGuid,
    pub b_is_visible_in_version_selector: bool,
}
#[repr(C, align(8))]
pub struct FNiagaraEmitterHandle {
    pub name: FName,
    pub id: FGuid,
    pub id_name: FName,
    pub b_is_enabled: bool,
    pub emitter_mode: ENiagaraEmitterMode,
    pub source_deprecated: UPtr<UNiagaraEmitter>,
    pub last_merged_source_deprecated: UPtr<UNiagaraEmitter>,
    pub b_isolated: bool,
    pub instance_deprecated: UPtr<UNiagaraEmitter>,
    pub versioned_instance: FVersionedNiagaraEmitter,
    pub stateless_emitter: UPtr<UNiagaraStatelessEmitter>,
}
#[repr(C, align(8))]
pub struct FNiagaraCollisionEventPayload {
    pub collision_pos: FVector,
    pub collision_normal: FVector,
    pub collision_velocity: FVector,
    pub particle_index: i32,
    pub physical_material_index: i32,
}
#[repr(C, align(8))]
pub struct FNiagaraMeshMICOverride {
    pub original_material: UPtr<UMaterialInterface>,
    pub replacement_material: UPtr<UMaterialInstanceConstant>,
}
#[repr(C, align(8))]
pub struct FNiagaraMeshMaterialOverride {
    pub explicit_mat: UPtr<UMaterialInterface>,
    pub user_param_binding: FNiagaraUserParameterBinding,
}
#[repr(C, align(8))]
pub struct FNiagaraMessageStore {
    pub message_key_to_message_map: TMap<FGuid, UPtr<UNiagaraMessageDataBase>>,
    pub dismissed_message_keys: TArray<FGuid>,
}
#[repr(C, align(8))]
pub struct FParameterDefinitionsSubscription {
    pub definitions: UPtr<UNiagaraParameterDefinitionsBase>,
    pub definitions_id_deprecated: FGuid,
    pub cached_change_id_hash: i32,
}
#[repr(C, align(8))]
pub struct FNiagaraParameters {
    pub parameters: TArray<FNiagaraVariable>,
}
#[repr(C, align(8))]
pub struct FNiagaraBoundParameter {
    pub parameter: FNiagaraVariableBase,
    pub src_offset: i32,
    pub dest_offset: i32,
}
#[repr(C, align(4))]
pub struct FNiagaraPerfBaselineStats {
    pub per_instance_avg_gt: f32,
    pub per_instance_avg_rt: f32,
    pub per_instance_max_gt: f32,
    pub per_instance_max_rt: f32,
}
#[repr(C, align(4))]
pub struct FNiagaraPlatformSetConflictEntry {
    pub profile_name: FName,
    pub quality_level_mask: i32,
}
#[repr(C, align(8))]
pub struct FNiagaraPlatformSetConflictInfo {
    pub set_a_index: i32,
    pub set_b_index: i32,
    pub conflicts: TArray<FNiagaraPlatformSetConflictEntry>,
}
#[repr(C, align(8))]
pub struct FNiagaraPlatformSetRedirect {
    pub profile_names: TArray<FName>,
    pub mode: ENiagaraDeviceProfileRedirectMode,
    pub redirect_profile_name: FName,
    pub c_var_condition_enabled: FNiagaraPlatformSetCVarCondition,
    pub c_var_condition_disabled: FNiagaraPlatformSetCVarCondition,
}
#[repr(C, align(4))]
pub struct FNiagaraRendererMaterialScalarParameter {
    pub material_parameter_name: FName,
    pub value: f32,
}
#[repr(C, align(4))]
pub struct FNiagaraRendererMaterialVectorParameter {
    pub material_parameter_name: FName,
    pub value: FLinearColor,
}
#[repr(C, align(8))]
pub struct FNiagaraRendererMaterialTextureParameter {
    pub material_parameter_name: FName,
    pub texture: UPtr<UTexture>,
}
#[repr(C, align(4))]
pub struct FNiagaraRendererMaterialStaticBoolParameter {
    pub material_parameter_name: FName,
    pub static_variable_name: FName,
    pub static_value: TOptional<bool>,
}
#[repr(C, align(8))]
pub struct FNiagaraRendererMaterialParameters {
    pub attribute_bindings: TArray<FNiagaraMaterialAttributeBinding>,
    pub scalar_parameters: TArray<FNiagaraRendererMaterialScalarParameter>,
    pub vector_parameters: TArray<FNiagaraRendererMaterialVectorParameter>,
    pub texture_parameters: TArray<FNiagaraRendererMaterialTextureParameter>,
    pub static_bool_parameters: TArray<FNiagaraRendererMaterialStaticBoolParameter>,
}
#[repr(C, align(4))]
pub struct FNiagaraRendererReadbackParameters {
    pub b_export_position: bool,
    pub b_export_tangent_basis: bool,
    pub b_export_color: bool,
    pub export_num_tex_coords: i32,
    pub b_export_materials: bool,
    pub b_apply_wpo: bool,
    pub view_index_to_capture: TOptional<i32>,
}
#[repr(C, align(4))]
pub struct FNiagaraRibbonShapeCustomVertex {
    pub position: FVector2f,
    pub normal: FVector2f,
    pub texture_v: f32,
}
#[repr(C, align(8))]
pub struct FNiagaraRibbonUVSettings {
    pub distribution_mode: ENiagaraRibbonUVDistributionMode,
    pub leading_edge_mode: ENiagaraRibbonUVEdgeMode,
    pub trailing_edge_mode: ENiagaraRibbonUVEdgeMode,
    pub flags_3: u8,
    pub tiling_length: f32,
    pub offset: FVector2D,
    pub scale: FVector2D,
}
#[repr(C, align(8))]
pub struct FNiagaraScalabilityManager {
    pub effect_type: UPtr<UNiagaraEffectType>,
    pub managed_components: TArray<UPtr<UNiagaraComponent>>,
}
#[repr(C, align(8))]
pub struct FNiagaraModuleDependency {
    pub id: FName,
    pub ty: ENiagaraModuleDependencyType,
    pub script_constraint: ENiagaraModuleDependencyScriptConstraint,
    pub required_version: FString,
    pub only_evaluate_in_script_usage: i32,
    pub description: FText,
}
#[repr(C, align(8))]
pub struct FNiagaraCompilerTag {
    pub variable: FNiagaraVariable,
    pub string_value: FString,
}
#[repr(C, align(8))]
pub struct FNiagaraVMExecutableDataId {
    pub compiler_version_id: FGuid,
    pub interpolated_spawn_mode: ENiagaraInterpolatedSpawnMode,
    pub script_usage_type_id: FGuid,
    pub script_usage_type: ENiagaraScriptUsage,
    pub additional_defines: TArray<FString>,
    pub additional_variables: TArray<FNiagaraVariableBase>,
    pub flags_72: u8,
    pub base_script_compile_hash: FNiagaraCompileHash,
    pub referenced_compile_hashes: TArray<FNiagaraCompileHash>,
    pub script_version_id: FGuid,
}
#[repr(C, align(8))]
pub struct FNiagaraVMExecutableByteCode {
    pub data: TArray<u8>,
    pub uncompressed_size: i32,
}
#[repr(C, align(8))]
pub struct FNiagaraVMExecutableData {
    pub byte_code: FNiagaraVMExecutableByteCode,
    pub num_temp_registers: i32,
    pub num_user_ptrs: i32,
    pub parameters: FNiagaraParameters,
    pub internal_parameters: FNiagaraParameters,
    pub external_dependencies: TArray<FNiagaraCompileDependency>,
    pub baked_rapid_iteration_parameters: TArray<FNiagaraVariable>,
    pub compile_tags_editor_only: TArray<FNiagaraCompilerTag>,
    pub compile_tags: TArray<FNiagaraCompilerTag>,
    pub script_literals: TArray<u8>,
    pub attributes: TArray<FNiagaraVariableBase>,
    pub data_usage: FNiagaraScriptDataUsageInfo,
    pub data_set_to_parameters: TMap<FName, FNiagaraParameters>,
    pub additional_external_functions: TArray<FNiagaraFunctionSignature>,
    pub u_object_infos: TArray<FNiagaraScriptUObjectCompileInfo>,
    pub data_interface_info: TArray<FNiagaraScriptDataInterfaceCompileInfo>,
    pub called_vm_external_functions: TArray<FVMExternalFunctionBindingInfo>,
    pub read_data_sets: TArray<FNiagaraDataSetID>,
    pub write_data_sets: TArray<FNiagaraDataSetProperties>,
    pub stat_scopes: TArray<FNiagaraStatScope>,
    pub last_hlsl_translation: FString,
    pub last_hlsl_translation_gpu: FString,
    pub last_assembly_translation: FString,
    pub last_op_count: u32,
    pub shader_script_parameters_metadata: FNiagaraShaderScriptParametersMetadata,
    pub parameter_collection_paths: TArray<FString>,
    pub last_compile_status: ENiagaraScriptCompileStatus,
    pub simulation_stage_meta_data: TArray<FSimulationStageMetaData>,
    pub b_reads_attribute_data: bool,
    pub attributes_written: TArray<FNiagaraVariableBase>,
    pub static_variables_written: TArray<FNiagaraVariable>,
    pub error_msg: FString,
    pub last_compile_events: TArray<FNiagaraCompileEvent>,
    pub experimental_context_data: TArray<u8>,
    pub last_experimental_assembly_script: FString,
    pub flags_664: u8,
}
#[repr(C, align(8))]
pub struct FNiagaraInlineDynamicInputFormatToken {
    pub usage: ENiagaraInlineDynamicInputFormatTokenUsage,
    pub value: FString,
}
#[repr(C, align(8))]
pub struct FVersionedNiagaraScriptData {
    pub version: FNiagaraAssetVersion,
    pub version_change_description: FText,
    pub module_usage_bitmask: i32,
    pub category: FText,
    pub inline_overview_display_name: FText,
    pub b_suggested: bool,
    pub provided_dependencies: TArray<FName>,
    pub required_dependencies: TArray<FNiagaraModuleDependency>,
    pub b_deprecated: bool,
    pub deprecation_message: FText,
    pub deprecation_recommendation: UPtr<UNiagaraScript>,
    pub b_use_python_script_conversion: bool,
    pub conversion_script_execution: ENiagaraPythonUpdateScriptReference,
    pub python_conversion_script: FString,
    pub conversion_script_asset: FFilePath,
    pub conversion_utility: TSubclassOf<UNiagaraConvertInPlaceUtilityBase>,
    pub flags_208: u8,
    pub experimental_message: FText,
    pub note_message: FText,
    pub debug_draw_message: FText,
    pub library_visibility: ENiagaraScriptLibraryVisibility,
    pub numeric_output_type_selection_mode: ENiagaraNumericOutputTypeSelectionMode,
    pub description: FText,
    pub keywords: FText,
    pub collapsed_view_format: FText,
    pub inline_expression_format: TArray<FNiagaraInlineDynamicInputFormatToken>,
    pub inline_graph_format: TArray<FNiagaraInlineDynamicInputFormatToken>,
    pub flags_352: u8,
    pub script_meta_data: TMap<FName, FString>,
    pub last_generated_vm_id: FNiagaraVMExecutableDataId,
    pub update_script_execution: ENiagaraPythonUpdateScriptReference,
    pub python_update_script: FString,
    pub script_asset: FFilePath,
    pub parameter_definitions_subscriptions: TArray<FParameterDefinitionsSubscription>,
    pub input_sections_deprecated: TArray<FNiagaraStackSection>,
    pub source: UPtr<UNiagaraScriptSourceBase>,
}
#[repr(C, align(2))]
pub struct FNiagaraScriptExecutionPaddingInfo {
    pub src_offset: u16,
    pub dest_offset: u16,
    pub src_size: u16,
    pub dest_size: u16,
}
#[repr(C, align(8))]
pub struct FNiagaraScriptExecutionParameterStore {
    pub parameter_size: i32,
    pub flags_420: u8,
}
#[repr(C, align(8))]
pub struct FNiagaraScriptInstanceParameterStore {}
#[repr(C, align(8))]
pub struct FNiagaraScriptHighlight {
    pub color: FLinearColor,
    pub display_name: FText,
}
#[repr(C, align(8))]
pub struct FNiagaraSimCacheDataBuffers {
    pub num_instances: u32,
    pub id_acquire_tag: u32,
    pub id_to_index_table_elements: u32,
}
#[repr(C, align(8))]
pub struct FNiagaraSimCacheEmitterFrame {
    pub local_bounds: FBox,
    pub total_spawned_particles: i32,
    pub particle_data_buffers: FNiagaraSimCacheDataBuffers,
}
#[repr(C, align(8))]
pub struct FNiagaraSimCacheSystemFrame {
    pub local_bounds: FBox,
    pub system_data_buffers: FNiagaraSimCacheDataBuffers,
}
#[repr(C, align(16))]
pub struct FNiagaraSimCacheFrame {
    pub local_to_world: FTransform,
    pub lwc_tile: FVector3f,
    pub simulation_age: f32,
    pub simulation_tick_count: i32,
    pub system_data: FNiagaraSimCacheSystemFrame,
    pub emitter_data: TArray<FNiagaraSimCacheEmitterFrame>,
}
#[repr(C, align(8))]
pub struct FNiagaraSimCacheVariable {
    pub variable: FNiagaraVariableBase,
    pub float_offset: u16,
    pub float_count: u16,
    pub half_offset: u16,
    pub half_count: u16,
    pub int32_offset: u16,
    pub int32_count: u16,
}
#[repr(C, align(8))]
pub struct FNiagaraSimCacheDataBuffersLayout {
    pub layout_name: FName,
    pub sim_target: ENiagaraSimTarget,
    pub variables: TArray<FNiagaraSimCacheVariable>,
    pub float_count: u16,
    pub half_count: u16,
    pub int32_count: u16,
    pub b_local_space: bool,
    pub b_allow_interpolation: bool,
    pub b_allow_velocity_extrapolation: bool,
    pub rebase_variable_names: TArray<FName>,
    pub interp_variable_names: TArray<FName>,
    pub component_velocity: u16,
}
#[repr(C, align(8))]
pub struct FNiagaraSimCacheLayout {
    pub system_layout: FNiagaraSimCacheDataBuffersLayout,
    pub emitter_layouts: TArray<FNiagaraSimCacheDataBuffersLayout>,
}
#[repr(C, align(8))]
pub struct FNiagaraSimCacheDebugDataFrame {
    pub debug_parameter_stores: TMap<FString, FNiagaraParameterStore>,
}
#[repr(C, align(8))]
pub struct FNiagaraEmitterCompiledData {
    pub spawn_attributes: TArray<FName>,
    pub emitter_spawn_interval_var: FNiagaraVariable,
    pub emitter_interp_spawn_start_dt_var: FNiagaraVariable,
    pub emitter_spawn_group_var: FNiagaraVariable,
    pub emitter_age_var: FNiagaraVariable,
    pub emitter_random_seed_var: FNiagaraVariable,
    pub emitter_instance_seed_var: FNiagaraVariable,
    pub emitter_total_spawned_particles_var: FNiagaraVariable,
    pub data_set_compiled_data: FNiagaraDataSetCompiledData,
}
#[repr(C, align(4))]
pub struct FNiagaraParameterDataSetBinding {
    pub parameter_offset: i32,
    pub data_set_component_offset: i32,
}
#[repr(C, align(8))]
pub struct FNiagaraParameterDataSetBindingCollection {
    pub float_offsets: TArray<FNiagaraParameterDataSetBinding>,
    pub int32_offsets: TArray<FNiagaraParameterDataSetBinding>,
}
#[repr(C, align(8))]
pub struct FNiagaraSystemCompiledData {
    pub instance_param_store: FNiagaraParameterStore,
    pub data_set_compiled_data: FNiagaraDataSetCompiledData,
    pub spawn_instance_params_data_set_compiled_data: FNiagaraDataSetCompiledData,
    pub update_instance_params_data_set_compiled_data: FNiagaraDataSetCompiledData,
    pub spawn_instance_global_binding: FNiagaraParameterDataSetBindingCollection,
    pub spawn_instance_system_binding: FNiagaraParameterDataSetBindingCollection,
    pub spawn_instance_owner_binding: FNiagaraParameterDataSetBindingCollection,
    pub spawn_instance_emitter_bindings: TArray<
        FNiagaraParameterDataSetBindingCollection,
    >,
    pub update_instance_global_binding: FNiagaraParameterDataSetBindingCollection,
    pub update_instance_system_binding: FNiagaraParameterDataSetBindingCollection,
    pub update_instance_owner_binding: FNiagaraParameterDataSetBindingCollection,
    pub update_instance_emitter_bindings: TArray<
        FNiagaraParameterDataSetBindingCollection,
    >,
}
#[repr(C, align(8))]
pub struct FNiagaraSystemScalabilityOverrides {
    pub overrides: TArray<FNiagaraSystemScalabilityOverride>,
}
#[repr(C, align(8))]
pub struct FNiagaraSystemCollectionData {
    pub systems: TArray<TSoftObjectPtr<UNiagaraSystem>>,
    pub systems_internal: TArray<UPtr<UNiagaraSystem>>,
}
#[repr(C, align(8))]
pub struct FNiagaraSystemStateData {
    pub flags_0: u8,
    pub inactive_response: ENiagaraSystemInactiveResponse,
    pub loop_behavior: ENiagaraLoopBehavior,
    pub loop_duration: FNiagaraDistributionRangeFloat,
    pub loop_count: i32,
    pub loop_delay: FNiagaraDistributionRangeFloat,
}
#[repr(C, align(8))]
pub struct FNiagaraDistributionBase {
    pub mode: ENiagaraDistributionMode,
    pub flags_9: u8,
    pub lookup_value_mode: u8,
    pub parameter_binding: FNiagaraVariableBase,
    pub parameter_expression: FInstancedStruct,
    pub channel_constants_and_ranges: TArray<f32>,
    pub channel_curves: TArray<FRichCurve>,
    pub max_lut_sample_count: i32,
}
#[repr(C, align(8))]
pub struct FNiagaraDistributionRangeFloat {
    pub min: f32,
    pub max: f32,
}
#[repr(C, align(8))]
pub struct FNiagaraEmitterStateData {
    pub inactive_response: ENiagaraEmitterInactiveResponse,
    pub loop_behavior: ENiagaraLoopBehavior,
    pub loop_count: i32,
    pub loop_duration_mode: ENiagaraLoopDurationMode,
    pub loop_duration: FNiagaraDistributionRangeFloat,
    pub loop_delay: FNiagaraDistributionRangeFloat,
    pub flags_288: u8,
    pub flags_289: u8,
    pub min_distance: f32,
    pub min_distance_reaction: ENiagaraExecutionStateManagement,
    pub max_distance: f32,
    pub max_distance_reaction: ENiagaraExecutionStateManagement,
    pub visibility_cull_reaction: ENiagaraExecutionStateManagement,
    pub visibility_cull_delay: f32,
}
#[repr(C, align(1))]
pub struct FNiagaraWildcard {}
#[repr(C, align(4))]
pub struct FNiagaraFloat {
    pub value: f32,
}
#[repr(C, align(4))]
pub struct FNiagaraInt32 {
    pub value: i32,
}
#[repr(C, align(4))]
pub struct FNiagaraBool {
    pub value: i32,
}
#[repr(C, align(4))]
pub struct FNiagaraPosition {}
#[repr(C, align(2))]
pub struct FNiagaraHalf {
    pub value: u16,
}
#[repr(C, align(2))]
pub struct FNiagaraHalfVector2 {
    pub x: u16,
    pub y: u16,
}
#[repr(C, align(2))]
pub struct FNiagaraHalfVector3 {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}
#[repr(C, align(2))]
pub struct FNiagaraHalfVector4 {
    pub x: u16,
    pub y: u16,
    pub z: u16,
    pub w: u16,
}
#[repr(C, align(1))]
pub struct FNiagaraNumeric {}
#[repr(C, align(1))]
pub struct FNiagaraParameterMap {}
#[repr(C, align(8))]
pub struct FNiagaraDouble {
    pub value: f64,
}
#[repr(C, align(16))]
pub struct FNiagaraMatrix {
    pub row0: FVector4f,
    pub row1: FVector4f,
    pub row2: FVector4f,
    pub row3: FVector4f,
}
#[repr(C, align(4))]
pub struct FNiagaraEmitterID {
    pub id: i32,
}
#[repr(C, align(4))]
pub struct FNiagaraSpawnInfo {
    pub count: i32,
    pub interp_start_dt: f32,
    pub interval_dt: f32,
    pub spawn_group: i32,
}
#[repr(C, align(4))]
pub struct FNiagaraID {
    pub index: i32,
    pub acquire_tag: i32,
}
#[repr(C, align(4))]
pub struct FNiagaraRandInfo {
    pub seed1: i32,
    pub seed2: i32,
    pub seed3: i32,
}
#[repr(C, align(8))]
pub struct FNiagaraUserRedirectionParameterStore {
    pub user_parameter_redirects: TMap<FNiagaraVariable, FNiagaraVariable>,
}
#[repr(C, align(8))]
pub struct FNiagaraInputConditionMetadata {
    pub input_name: FName,
    pub target_values: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FNiagaraEnumParameterMetaData {
    pub override_name: FName,
    pub icon_override: UPtr<UTexture2D>,
    pub b_use_color_override: bool,
    pub color_override: FLinearColor,
}
#[repr(C, align(8))]
pub struct FWidgetNamedInputValue {
    pub value: f32,
    pub display_name: FText,
    pub tooltip: FText,
}
#[repr(C, align(8))]
pub struct FWidgetSegmentValueOverride {
    pub enum_index_to_override: i32,
    pub b_override_display_name: bool,
    pub display_name_override: FText,
    pub display_icon: UPtr<UTexture2D>,
}
#[repr(C, align(8))]
pub struct FNiagaraWidgetNamedIntegerInputValue {
    pub display_name: FText,
    pub tooltip: FText,
}
#[repr(C, align(8))]
pub struct FNiagaraInputParameterCustomization {
    pub widget_type: ENiagaraInputWidgetType,
    pub b_has_min_value: bool,
    pub min_value: f32,
    pub b_has_max_value: bool,
    pub max_value: f32,
    pub b_has_step_width: bool,
    pub step_width: f32,
    pub input_dropdown_values: TArray<FWidgetNamedInputValue>,
    pub enum_style_dropdown_values: TArray<FNiagaraWidgetNamedIntegerInputValue>,
    pub max_segments_per_row: i32,
    pub segment_value_overrides: TArray<FWidgetSegmentValueOverride>,
    pub b_broadcast_value_changes_on_commit_only: bool,
}
#[repr(C, align(8))]
pub struct FNiagaraBoolParameterMetaData {
    pub display_mode: ENiagaraBoolDisplayMode,
    pub override_name_true: FName,
    pub override_name_false: FName,
    pub icon_override_true: UPtr<UTexture2D>,
    pub icon_override_false: UPtr<UTexture2D>,
}
#[repr(C, align(8))]
pub struct FNiagaraVariableMetaData {
    pub description: FText,
    pub display_unit: EUnit,
    pub b_advanced_display: bool,
    pub b_display_in_overview_stack: bool,
    pub inline_parameter_sort_priority: i32,
    pub b_override_color: bool,
    pub inline_parameter_color_override: FLinearColor,
    pub inline_parameter_enum_overrides: TArray<FNiagaraEnumParameterMetaData>,
    pub b_enable_bool_override: bool,
    pub inline_parameter_bool_override: FNiagaraBoolParameterMetaData,
    pub b_inline_edit_condition_toggle: bool,
    pub edit_condition: FNiagaraInputConditionMetadata,
    pub visible_condition: FNiagaraInputConditionMetadata,
    pub property_meta_data: TMap<FName, FString>,
    pub alternate_aliases: TArray<FName>,
    pub widget_customization: FNiagaraInputParameterCustomization,
    pub variable_guid: FGuid,
    pub b_is_static_switch_deprecated: bool,
    pub static_switch_default_value_deprecated: i32,
    pub category_name_deprecated: FText,
    pub parent_attribute_deprecated: FName,
    pub editor_sort_priority_deprecated: i32,
}
#[repr(C, align(8))]
pub struct FNiagaraVariant {
    pub object: UPtr<UObject>,
    pub data_interface: UPtr<UNiagaraDataInterface>,
    pub bytes: TArray<u8>,
    pub current_mode: ENiagaraVariantMode,
}
#[repr(C, align(8))]
pub struct FNiagaraWorldManagerTickFunction {}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpression {}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionColor {}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionColorConstant {
    pub a: FLinearColor,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionColorBinding {
    pub a: FName,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionColorAdd {
    pub a: FInstancedStruct,
    pub b: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionColorSubtract {
    pub a: FInstancedStruct,
    pub b: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionColorMultiply {
    pub a: FInstancedStruct,
    pub b: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionColorDivide {
    pub a: FInstancedStruct,
    pub b: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionFloat {}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionFloatConstant {
    pub a: f32,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionFloatBinding {
    pub a: FName,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionFloatAdd {
    pub a: FInstancedStruct,
    pub b: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionFloatSubtract {
    pub a: FInstancedStruct,
    pub b: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionFloatMultiply {
    pub a: FInstancedStruct,
    pub b: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionFloatDivide {
    pub a: FInstancedStruct,
    pub b: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionVec2 {}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionVec2Constant {
    pub a: FVector2f,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionVec2Binding {
    pub a: FName,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionVec2Add {
    pub a: FInstancedStruct,
    pub b: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionVec2Subtract {
    pub a: FInstancedStruct,
    pub b: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionVec2Multiply {
    pub a: FInstancedStruct,
    pub b: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionVec2Divide {
    pub a: FInstancedStruct,
    pub b: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionVec3 {}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionVec3Constant {
    pub a: FVector3f,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionVec3Binding {
    pub a: FName,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionVec3Add {
    pub a: FInstancedStruct,
    pub b: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionVec3Subtract {
    pub a: FInstancedStruct,
    pub b: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionVec3Multiply {
    pub a: FInstancedStruct,
    pub b: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionVec3Divide {
    pub a: FInstancedStruct,
    pub b: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionVec4 {}
#[repr(C, align(16))]
pub struct FNiagaraStatelessExpressionVec4Constant {
    pub a: FVector4f,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionVec4Binding {
    pub a: FName,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionVec4Add {
    pub a: FInstancedStruct,
    pub b: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionVec4Subtract {
    pub a: FInstancedStruct,
    pub b: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionVec4Multiply {
    pub a: FInstancedStruct,
    pub b: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessExpressionVec4Divide {
    pub a: FInstancedStruct,
    pub b: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessDynamicParameterSet {
    pub flags_0: u8,
    pub x_channel_distribution: FNiagaraDistributionFloat,
    pub y_channel_distribution: FNiagaraDistributionFloat,
    pub z_channel_distribution: FNiagaraDistributionFloat,
    pub w_channel_distribution: FNiagaraDistributionFloat,
}
#[repr(C, align(8))]
pub struct FNiagaraDistributionFloat {
    pub values: TArray<f32>,
    pub values_time_range: FVector2f,
}
#[repr(C, align(8))]
pub struct FNiagaraDistributionRangeInt {
    pub mode: ENiagaraDistributionMode,
    pub parameter_binding: FNiagaraVariableBase,
    pub parameter_expression: FInstancedStruct,
    pub min: i32,
    pub max: i32,
}
#[repr(C, align(8))]
pub struct FNiagaraDistributionRangeVector2 {
    pub min: FVector2f,
    pub max: FVector2f,
}
#[repr(C, align(8))]
pub struct FNiagaraDistributionRangeVector3 {
    pub min: FVector3f,
    pub max: FVector3f,
}
#[repr(C, align(8))]
pub struct FNiagaraDistributionRangeColor {
    pub min: FLinearColor,
    pub max: FLinearColor,
}
#[repr(C, align(8))]
pub struct FNiagaraDistributionRangeRotator {
    pub min: FRotator3f,
    pub max: FRotator3f,
}
#[repr(C, align(8))]
pub struct FNiagaraDistributionVector2 {
    pub values: TArray<FVector2f>,
    pub values_time_range: FVector2f,
}
#[repr(C, align(8))]
pub struct FNiagaraDistributionVector3 {
    pub values: TArray<FVector3f>,
    pub values_time_range: FVector2f,
}
#[repr(C, align(8))]
pub struct FNiagaraDistributionPosition {}
#[repr(C, align(8))]
pub struct FNiagaraDistributionColor {
    pub values: TArray<FLinearColor>,
    pub values_time_range: FVector2f,
}
#[repr(C, align(8))]
pub struct FNiagaraDistributionCurveFloat {
    pub values: TArray<f32>,
    pub values_time_range: FVector2f,
}
#[repr(C, align(8))]
pub struct FNiagaraDistributionCurveVector3 {
    pub values: TArray<FVector3f>,
    pub values_time_range: FVector2f,
}
#[repr(C, align(8))]
pub struct FNiagaraStatelessSpawnInfo {
    pub source_id: FGuid,
    pub ty: ENiagaraStatelessSpawnInfoType,
    pub spawn_time: f32,
    pub amount: FNiagaraDistributionRangeInt,
    pub rate: FNiagaraDistributionRangeFloat,
    pub flags_248: u8,
    pub spawn_probability: FNiagaraDistributionRangeFloat,
    pub loop_count_limit: FNiagaraDistributionRangeInt,
}
pub struct UNiagaraDataInterface {}
pub struct UNiagaraDataInterfacePlatformSet {
    pub platforms: FNiagaraPlatformSet,
}
pub struct UNiagaraSystem {
    pub thumbnail_image: UPtr<UTexture2D>,
    pub b_expose_to_library_deprecated: bool,
    pub library_visibility: ENiagaraScriptLibraryVisibility,
    pub b_is_template_asset_deprecated: bool,
    pub template_specification_deprecated: ENiagaraScriptTemplateSpecification,
    pub asset_tags_deprecated: TArray<FNiagaraAssetTagDefinitionReference>,
    pub template_asset_description: FText,
    pub category: FText,
    pub preview_movie_path: FSoftObjectPath,
    pub scratch_pad_scripts: TArray<UPtr<UNiagaraScript>>,
    pub editor_only_added_parameters: FNiagaraParameterStore,
    pub update_context: FNiagaraSystemUpdateContext,
    pub flags_832: u8,
    pub parameter_definitions_subscriptions: TArray<FParameterDefinitionsSubscription>,
    pub flags_856: u8,
    pub flags_857: u8,
    pub large_world_coordinate_tile_update_mode: TOptional<ENiagaraLwcTileUpdateMode>,
    pub custom_depth_stencil_write_mask: ERendererStencilMask,
    pub custom_depth_stencil_value: i32,
    pub translucency_sort_priority: i32,
    pub translucency_sort_distance_offset: f32,
    pub flags_876: u8,
    pub flags_880: u8,
    pub effect_type: UPtr<UNiagaraEffectType>,
    pub flags_896: u8,
    pub system_scalability_overrides: FNiagaraSystemScalabilityOverrides,
    pub platforms: FNiagaraPlatformSet,
    pub emitter_handles: TArray<FNiagaraEmitterHandle>,
    pub parameter_collection_overrides: TArray<
        UPtr<UNiagaraParameterCollectionInstance>,
    >,
    pub scalability_overrides_deprecated: TArray<FNiagaraSystemScalabilityOverride>,
    pub system_spawn_script: UPtr<UNiagaraScript>,
    pub system_update_script: UPtr<UNiagaraScript>,
    pub system_compiled_data: FNiagaraSystemCompiledData,
    pub exposed_parameters: FNiagaraUserRedirectionParameterStore,
    pub editor_data: UPtr<UNiagaraEditorDataBase>,
    pub editor_parameters: UPtr<UNiagaraEditorParametersAdapterBase>,
    pub fixed_bounds: FBox,
    pub b_use_initial_streaming_bounds: bool,
    pub initial_streaming_bounds: FBox,
    pub b_needs_gpu_context_init_for_data_interfaces: bool,
    pub b_determinism: bool,
    pub b_fixed_tick_delta: bool,
    pub random_seed: i32,
    pub warmup_time: f32,
    pub warmup_tick_count: i32,
    pub warmup_tick_delta: f32,
    pub fixed_tick_delta_time: f32,
    pub baker_settings: UPtr<UNiagaraBakerSettings>,
    pub baker_generated_settings: UPtr<UNiagaraBakerSettings>,
    pub flags_3064: u8,
    pub message_key_to_message_map_deprecated: TMap<
        FGuid,
        UPtr<UNiagaraMessageDataBase>,
    >,
    pub message_store: FNiagaraMessageStore,
    pub system_state_data_struct: FInstancedStruct,
}
pub struct UNiagaraConvertInPlaceUtilityBase {}
pub struct UNiagaraDataInterfaceDynamicMeshSimCacheData {
    pub unique_frames: TArray<FNDIDynamicMeshSimCacheFrame>,
    pub cpu_frame_indices: TArray<i32>,
    pub gpu_frame_indices: TArray<i32>,
}
pub struct UNiagaraParameterDefinitionsBase {
    pub unique_id: FGuid,
}
pub struct UNiagaraRenderableMeshArrayInterface {}
pub struct INiagaraRenderableMeshArrayInterface {}
pub struct UNiagaraRenderableMeshInterface {}
pub struct INiagaraRenderableMeshInterface {}
pub struct UNiagaraScriptSourceBase {}
pub struct UNiagaraSimCacheCustomStorageInterface {}
pub struct INiagaraSimCacheCustomStorageInterface {}
pub struct UNiagaraValidationRuleSet {
    pub validation_rules: TArray<UPtr<UNiagaraValidationRule>>,
}
pub struct UNDIRenderTargetSimCacheData {
    pub compression_type: FName,
    pub frames: TArray<FNDIRenderTargetSimCacheFrame>,
}
pub struct UNiagaraDataInterfaceActorComponent {
    pub source_mode: ENDIActorComponentSourceMode,
    pub local_player_index: i32,
    pub source_actor: TLazyObjectPtr<AActor>,
    pub actor_or_component_parameter: FNiagaraUserParameterBinding,
    pub b_require_current_frame_data: bool,
}
pub struct UNiagaraDataInterfaceArrayDistributionInt {
    pub array_data: TArray<FNDIDistributionIntArrayEntry>,
    pub built_table_data: TArray<u8>,
}
pub struct UNiagaraDataInterfaceRWBase {}
pub struct UNiagaraDataInterfaceArray {
    pub gpu_sync_mode: ENiagaraGpuSyncMode,
    pub max_elements: i32,
}
pub struct UNiagaraDataInterfaceArrayMesh {
    pub mesh_data: TArray<FNiagaraMeshRendererMeshPropertiesBase>,
}
pub struct UNiagaraDataInterfaceAsyncGpuTrace {
    pub max_traces_per_particle: i32,
    pub max_retraces: i32,
    pub trace_provider: ENDICollisionQuery_AsyncGpuTraceProvider,
}
pub struct UNiagaraDataInterfaceConsoleVariable {}
pub struct UNiagaraDataInterfaceDataChannelRead {
    pub channel: UPtr<UNiagaraDataChannelAsset>,
    pub access_context: FNDCAccessContextInst,
    pub b_auto_link_to_spawning_ndc: bool,
    pub b_read_current_frame: bool,
    pub b_update_source_data_every_tick: bool,
    pub b_override_spawn_group_to_data_channel_index: bool,
    pub b_only_spawn_once_on_subticks: bool,
    pub compiled_data: FNDIDataChannelCompiledData,
}
pub struct UNDIDataChannelWriteSimCacheData {
    pub frame_data: TArray<FNDIDataChannelWriteSimCacheFrame>,
    pub data_channel_reference: FSoftObjectPath,
    pub data_interface: UPtr<UNiagaraDataInterfaceDataChannelWrite>,
}
pub struct UNiagaraDataInterfaceDataChannelWrite {
    pub allocation_mode: ENiagaraDataChannelAllocationMode,
    pub allocation_count: u32,
    pub b_publish_to_game: bool,
    pub b_publish_to_cpu: bool,
    pub b_publish_to_gpu: bool,
    pub b_update_destination_data_every_tick: bool,
    pub b_only_write_once_on_subticks: bool,
    pub channel: UPtr<UNiagaraDataChannelAsset>,
    pub access_context: FNDCAccessContextInst,
    pub compiled_data: FNDIDataChannelWriteCompiledData,
}
pub struct UNiagaraDataInterfaceDataTable {
    pub data_table: UPtr<UDataTable>,
    pub filtered_row_names: TArray<FName>,
    pub object_parameter_binding: FNiagaraUserParameterBinding,
    pub b_create_filtered_table: bool,
}
pub struct UNiagaraDataInterfaceDebugDraw {
    pub override_max_line_instances: u32,
}
pub struct UNiagaraDataInterfaceDynamicMesh {
    pub sections: TArray<FNiagaraDynamicMeshSection>,
    pub materials: TArray<FNiagaraDynamicMeshMaterial>,
    pub num_vertices: i32,
    pub num_tex_coords: i32,
    pub b_has_colors: bool,
    pub b_has_tangent_basis: bool,
    pub b_clear_triangles_per_frame: bool,
    pub default_bounds: FBox,
}
pub struct UNiagaraDataInterfaceEmitterProperties {
    pub emitter_binding: FNiagaraDataInterfaceEmitterBinding,
}
pub struct UNiagaraDataInterfaceGBuffer {}
pub struct UNiagaraDataInterfaceMemoryBuffer {
    pub default_num_elements: i32,
    pub gpu_sync_mode: ENiagaraGpuSyncMode,
}
pub struct UNDIMemoryBufferSimCacheData {
    pub frame_data: TArray<FNDIMemoryBufferSimCacheDataFrame>,
    pub buffer_data: TArray<u32>,
}
pub struct UNiagaraDataInterfacePhysicsAsset {
    pub default_source: UPtr<UPhysicsAsset>,
    pub soft_source_actor: TSoftObjectPtr<AActor>,
    pub source_actor_deprecated: UPtr<AActor>,
    pub mesh_user_parameter: FNiagaraUserParameterBinding,
}
pub struct UNiagaraPhysicsAssetDICollectorInterface {}
pub struct INiagaraPhysicsAssetDICollectorInterface {}
pub struct UNiagaraDataInterfaceSceneCapture2D {
    pub source_mode: ENDISceneCapture2DSourceMode,
    pub scene_capture_user_parameter: FNiagaraUserParameterBinding,
    pub b_auto_move_with_component: bool,
    pub auto_move_offset_location_mode: ENDISceneCapture2DOffsetMode,
    pub auto_move_offset_location: FVector,
    pub auto_move_offset_rotation_mode: ENDISceneCapture2DOffsetMode,
    pub auto_move_offset_rotation: FRotator,
    pub managed_capture_source: ESceneCaptureSource,
    pub managed_texture_size: FIntPoint,
    pub managed_texture_format: ETextureRenderTargetFormat,
    pub managed_projection_type: ECameraProjectionMode,
    pub managed_fov_angle: f32,
    pub managed_ortho_width: f32,
    pub b_managed_capture_every_frame: bool,
    pub b_managed_capture_on_movement: bool,
    pub b_managed_hide_attach_component: bool,
    pub managed_hiden_actors: TArray<UPtr<AActor>>,
    pub managed_show_only_actors: TArray<UPtr<AActor>>,
    pub managed_capture_components: TMap<u64, UPtr<USceneCaptureComponent2D>>,
}
pub struct UNiagaraDataInterfaceSimCacheReader {
    pub sim_cache_binding: FNiagaraUserParameterBinding,
    pub sim_cache: UPtr<UNiagaraSimCache>,
    pub emitter_binding: FName,
}
pub struct UNiagaraDataInterfaceSimpleCounter {
    pub gpu_sync_mode: ENiagaraGpuSyncMode,
    pub initial_value: i32,
}
pub struct UNDISimpleCounterSimCacheData {
    pub values: TArray<i32>,
}
pub struct UNiagaraDataInterfaceSocketReader {
    pub source_mode: ENDISocketReaderSourceMode,
    pub filtered_sockets: TArray<FName>,
    pub editor_preview_asset: TSoftObjectPtr<UObject>,
    pub source_actor: TLazyObjectPtr<AActor>,
    pub source_asset: UPtr<UObject>,
    pub attach_component_class: TSubclassOf<UObject>,
    pub attach_component_tag: FName,
    pub object_parameter_binding: FNiagaraUserParameterBinding,
    pub b_update_sockets_per_frame: bool,
    pub b_require_current_frame_data: bool,
}
pub struct UNiagaraDataInterfaceStaticMesh {
    pub source_mode: ENDIStaticMesh_SourceMode,
    pub preview_mesh: TSoftObjectPtr<UStaticMesh>,
    pub default_mesh: UPtr<UStaticMesh>,
    pub soft_source_actor: TSoftObjectPtr<AActor>,
    pub source_deprecated: UPtr<AActor>,
    pub source_component: TWeakObjectPtr<UStaticMeshComponent>,
    pub section_filter: FNDIStaticMeshSectionFilter,
    pub b_capture_transforms_per_frame: bool,
    pub b_use_physics_body_velocity: bool,
    pub b_allow_sampling_from_streaming_lo_ds: bool,
    pub lod_index: i32,
    pub lod_index_user_parameter: FNiagaraUserParameterBinding,
    pub mesh_parameter_binding: FNiagaraUserParameterBinding,
    pub instance_index: i32,
    pub filtered_sockets: TArray<FName>,
}
pub struct UNiagaraDataInterfaceUObjectPropertyReader {
    pub source_mode: ENDIObjectPropertyReaderSourceMode,
    pub u_object_parameter_binding: FNiagaraUserParameterBinding,
    pub property_remap: TArray<FNiagaraUObjectPropertyReaderRemap>,
    pub source_actor: TSoftObjectPtr<AActor>,
    pub source_actor_component_class: TSubclassOf<UObject>,
}
pub struct UNiagaraDataInterfaceVirtualTexture {
    pub texture: UPtr<URuntimeVirtualTexture>,
    pub texture_user_parameter: FNiagaraUserParameterBinding,
}
pub struct UMovieSceneNiagaraSystemSpawnSection {
    pub section_start_behavior: ENiagaraSystemSpawnSectionStartBehavior,
    pub section_evaluate_behavior: ENiagaraSystemSpawnSectionEvaluateBehavior,
    pub section_end_behavior: ENiagaraSystemSpawnSectionEndBehavior,
    pub age_update_mode: ENiagaraAgeUpdateMode,
    pub b_allow_scalability: bool,
}
pub struct UMovieSceneNiagaraTrack {
    pub sections: TArray<UPtr<UMovieSceneSection>>,
}
pub struct UMovieSceneNiagaraSystemTrack {}
pub struct UMovieSceneNiagaraParameterTrack {
    pub parameter: FNiagaraVariable,
}
pub struct UMovieSceneNiagaraBoolParameterTrack {}
pub struct UMovieSceneNiagaraColorParameterTrack {}
pub struct UMovieSceneNiagaraFloatParameterTrack {}
pub struct UMovieSceneNiagaraIntegerParameterTrack {}
pub struct UMovieSceneNiagaraVectorParameterTrack {
    pub channels_used: i32,
}
pub struct ANiagaraActor {
    pub niagara_component: UPtr<UNiagaraComponent>,
    pub sprite_component: UPtr<UBillboardComponent>,
    pub arrow_component: UPtr<UArrowComponent>,
    pub flags_1160: u8,
}
pub struct UDEPRECATED_NiagaraAssetTagDefinitions {
    pub display_name: FText,
    pub description: FText,
    pub tag_definitions: TArray<FNiagaraAssetTagDefinition>,
    pub b_display_tags_as_flat_list: bool,
    pub sort_order: i32,
}
pub struct UNiagaraBakerOutput {
    pub output_name: FString,
}
pub struct UNiagaraBakerOutputSimCache {
    pub sim_cache_asset_path_format: FString,
    pub create_parameters: FNiagaraSimCacheCreateParameters,
}
pub struct UNiagaraBakerOutputSparseVolumeTexture {
    pub source_binding: FNiagaraBakerTextureSource,
    pub volume_world_space_size_binding: FNiagaraParameterBinding,
    pub sparse_volume_texture_asset_path_format: FString,
    pub b_enable_looped_output: bool,
    pub looped_sparse_volume_texture_asset_path_format: FString,
    pub start_time: f32,
    pub blend_duration: f32,
}
pub struct UNiagaraBakerOutputStaticMesh {
    pub frames_asset_path_format: FString,
    pub export_parameters: FNiagaraRendererReadbackParameters,
}
pub struct UNiagaraBakerOutputTexture2D {
    pub source_binding: FNiagaraBakerTextureSource,
    pub flags_96: u8,
    pub frame_size: FIntPoint,
    pub atlas_texture_size: FIntPoint,
    pub texture_address_x: TextureAddress,
    pub texture_address_y: TextureAddress,
    pub atlas_asset_path_format: FString,
    pub frames_asset_path_format: FString,
    pub frames_export_path_format: FString,
}
pub struct UNiagaraBakerOutputVolumeTexture {
    pub source_binding: FNiagaraBakerTextureSource,
    pub flags_96: u8,
    pub atlas_asset_path_format: FString,
    pub frames_asset_path_format: FString,
    pub frames_export_path_format: FString,
}
pub struct UNiagaraBakerSettings {
    pub start_seconds: f32,
    pub duration_seconds: f32,
    pub frames_per_second: i32,
    pub flags_60: u8,
    pub frames_per_dimension: FIntPoint,
    pub outputs: TArray<UPtr<UNiagaraBakerOutput>>,
    pub camera_settings: TArray<FNiagaraBakerCameraSettings>,
    pub current_camera_index: i32,
    pub bake_quality_level: FName,
    pub flags_120: u8,
    pub output_textures_deprecated: TArray<FNiagaraBakerTextureSettings>,
    pub camera_viewport_mode_deprecated: ENiagaraBakerViewMode,
    pub camera_viewport_location_deprecated: FVector,
    pub camera_viewport_rotation_deprecated: FRotator,
    pub camera_orbit_distance_deprecated: f32,
    pub camera_fov_deprecated: f32,
    pub camera_ortho_width_deprecated: f32,
    pub flags_500: u8,
    pub camera_aspect_ratio_deprecated: f32,
}
pub struct UNiagaraComponent {
    pub asset: UPtr<UNiagaraSystem>,
    pub tick_behavior: ENiagaraTickBehavior,
    pub random_seed_offset: i32,
    pub override_parameters: FNiagaraUserRedirectionParameterStore,
    pub editor_overrides_value_deprecated: TMap<FName, bool>,
    pub template_parameter_overrides: TMap<FNiagaraVariableBase, FNiagaraVariant>,
    pub instance_parameter_overrides: TMap<FNiagaraVariableBase, FNiagaraVariant>,
    pub template_parameter_overrides_cache: TMap<FNiagaraVariableBase, FNiagaraVariant>,
    pub instance_parameter_overrides_cache: TMap<FNiagaraVariableBase, FNiagaraVariant>,
    pub flags_2488: u8,
    pub warmup_tick_count: i32,
    pub warmup_tick_delta: f32,
    pub flags_2552: u8,
    pub max_time_before_force_update_transform: f32,
    pub occlusion_query_mode: ENiagaraOcclusionQueryMode,
    pub on_system_finished: FNiagaraComponent_OnSystemFinished,
    pub auto_attach_parent: TWeakObjectPtr<USceneComponent>,
    pub auto_attach_socket_name: FName,
    pub auto_attach_location_rule: EAttachmentRule,
    pub auto_attach_rotation_rule: EAttachmentRule,
    pub auto_attach_scale_rule: EAttachmentRule,
    pub flags_2624: u8,
    pub sim_cache_debug_num_frames_to_capture: TOptional<i32>,
    pub flags_2648: u8,
    pub sim_cache: UPtr<UNiagaraSimCache>,
    pub cull_proxy: UPtr<UNiagaraCullProxyComponent>,
}
pub struct UNiagaraComponentPool {
    pub world_particle_system_pools: TMap<UPtr<UNiagaraSystem>, FNCPool>,
}
pub struct UNiagaraRendererProperties {
    pub platforms: FNiagaraPlatformSet,
    pub sort_order_hint: i32,
    pub motion_vector_setting: ENiagaraRendererMotionVectorSetting,
    pub b_is_enabled: bool,
    pub b_allow_in_cull_proxies: bool,
    pub renderer_enabled_binding: FNiagaraVariableAttributeBinding,
    pub outer_emitter_version: FGuid,
    pub b_motion_blur_enabled_deprecated: bool,
    pub b_debug_draw_enabled: bool,
}
pub struct UNiagaraComponentRendererProperties {
    pub component_type: TSubclassOf<USceneComponent>,
    pub component_count_limit: u32,
    pub enabled_binding: FNiagaraVariableAttributeBinding,
    pub renderer_visibility_tag_binding: FNiagaraVariableAttributeBinding,
    pub b_assign_components_on_particle_id: bool,
    pub b_create_component_first_particle_frame: bool,
    pub b_only_activate_newly_aquired_components: bool,
    pub b_visualize_components: bool,
    pub b_only_create_components_on_particle_spawn_deprecated: bool,
    pub renderer_visibility: i32,
    pub template_component: UPtr<USceneComponent>,
    pub property_bindings: TArray<FNiagaraComponentPropertyBinding>,
}
pub struct UNiagaraCullProxyComponent {
    pub instances: TArray<FNiagaraCulledComponentInfo>,
}
pub struct UNiagaraDataChannel {
    pub channel_variables: TArray<FNiagaraDataChannelVariable>,
    pub b_keep_previous_frame_data: bool,
    pub b_enforce_tick_group_read_write_order: bool,
    pub final_write_tick_group: ETickingGroup,
    pub version_guid: FGuid,
    pub variables_deprecated: TArray<FNiagaraVariable>,
}
pub struct UNiagaraDataChannelReader {
    pub owner: UPtr<UNiagaraDataChannelHandler>,
}
pub struct UNiagaraDataChannelWriter {
    pub owner: UPtr<UNiagaraDataChannelHandler>,
}
pub struct UNiagaraDataChannelAsset {
    pub data_channel: UPtr<UNiagaraDataChannel>,
    pub cached_pre_change_data_channel: UPtr<UNiagaraDataChannel>,
}
pub struct UNiagaraDataChannelLibrary {}
pub struct UNiagaraDataChannelHandler {
    pub data_channel: TWeakObjectPtr<UNiagaraDataChannel>,
    pub writer: UPtr<UNiagaraDataChannelWriter>,
    pub reader: UPtr<UNiagaraDataChannelReader>,
}
pub struct UNiagaraDataChannel_MapBase {
    pub default_system_to_spawn: UPtr<UObject>,
}
pub struct UNiagaraDataChannel_GameplayBurst {
    pub cell_size: FVector,
    pub system_bounds_padding: FVector,
    pub attachment_settings: FNDCGameplayBurstAttachmentSettings,
}
pub struct UNiagaraDataChannelHandler_MapBase {
    pub entry_pool: TArray<FNDCMapEntry>,
    pub spawned_components_to_active_entry: TMap<UPtr<UNiagaraComponent>, i32>,
    pub default_system_to_spawn: UPtr<UObject>,
}
pub struct UNiagaraDataChannelHandler_GameplayBurst {
    pub attachment_settings: FNDCGameplayBurstAttachmentSettings,
}
pub struct UNiagaraDataChannel_Global {}
pub struct UNiagaraDataChannelHandler_Global {}
pub struct UNiagaraDataChannel_Islands {
    pub mode: ENiagraDataChannel_IslandMode,
    pub initial_extents: FVector,
    pub max_extents: FVector,
    pub per_element_extents: FVector,
    pub systems: TArray<TSoftObjectPtr<UNiagaraSystem>>,
    pub island_pool_size: i32,
    pub debug_draw_settings: FNDCIslandDebugDrawSettings,
    pub systems_internal: TArray<UPtr<UNiagaraSystem>>,
}
pub struct UNiagaraDataChannelHandler_Islands {
    pub active_islands: TArray<i32>,
    pub free_islands: TArray<i32>,
    pub island_pool: TArray<FNDCIsland>,
}
pub struct UNiagaraDataInterface2DArrayTexture {
    pub texture: UPtr<UTexture>,
    pub texture_user_parameter: FNiagaraUserParameterBinding,
}
pub struct UNDIArraySimCacheData {
    pub cpu_frame_data: TArray<FNDIArraySimCacheDataFrame>,
    pub gpu_frame_data: TArray<FNDIArraySimCacheDataFrame>,
    pub buffer_data: TArray<u8>,
}
pub struct UNiagaraDataInterfaceArrayFloat {
    pub float_data: TArray<f32>,
}
pub struct UNiagaraDataInterfaceArrayFloat2 {
    pub float_data: TArray<FVector2D>,
    pub internal_float_data: TArray<FVector2f>,
}
pub struct UNiagaraDataInterfaceArrayFloat3 {
    pub float_data: TArray<FVector>,
    pub internal_float_data: TArray<FVector3f>,
}
pub struct UNiagaraDataInterfaceArrayPosition {
    pub position_data: TArray<FNiagaraPosition>,
}
pub struct UNiagaraDataInterfaceArrayFloat4 {
    pub float_data: TArray<FVector4>,
    pub internal_float_data: TArray<FVector4f>,
}
pub struct UNiagaraDataInterfaceArrayColor {
    pub color_data: TArray<FLinearColor>,
}
pub struct UNiagaraDataInterfaceArrayQuat {
    pub quat_data: TArray<FQuat>,
    pub internal_quat_data: TArray<FQuat4f>,
}
pub struct UNiagaraDataInterfaceArrayMatrix {
    pub matrix_data: TArray<FMatrix>,
    pub internal_matrix_data: TArray<FMatrix44f>,
}
pub struct UNiagaraDataInterfaceArrayFunctionLibrary {}
pub struct UNiagaraDataInterfaceArrayInt32 {
    pub int_data: TArray<i32>,
}
pub struct UNiagaraDataInterfaceArrayUInt8 {
    pub int_data: TArray<i32>,
    pub internal_int_data: TArray<u8>,
}
pub struct UNiagaraDataInterfaceArrayBool {
    pub bool_data: TArray<bool>,
}
pub struct UNiagaraDataInterfaceArrayNiagaraID {
    pub int_data: TArray<FNiagaraID>,
}
pub struct UNiagaraDataInterfaceAudioSubmix {
    pub submix: UPtr<USoundSubmix>,
}
pub struct UNiagaraDataInterfaceAudioOscilloscope {
    pub submix: UPtr<USoundSubmix>,
    pub resolution: i32,
    pub scope_in_milliseconds: f32,
}
pub struct UNiagaraDataInterfaceAudioPlayerSettings {
    pub b_override_concurrency: bool,
    pub concurrency: UPtr<USoundConcurrency>,
    pub b_override_attenuation_settings: bool,
    pub attenuation_settings: FSoundAttenuationSettings,
}
pub struct UNiagaraDataInterfaceAudioPlayer {
    pub sound_to_play: UPtr<USoundBase>,
    pub attenuation: UPtr<USoundAttenuation>,
    pub concurrency: UPtr<USoundConcurrency>,
    pub parameter_names: TArray<FName>,
    pub configuration_user_parameter: FNiagaraUserParameterBinding,
    pub b_limit_plays_per_tick: bool,
    pub max_plays_per_tick: i32,
    pub b_stop_when_component_is_destroyed: bool,
    pub b_allow_looping_one_shot_sounds: bool,
    pub b_only_active_during_gameplay: bool,
}
pub struct UNiagaraDataInterfaceAudioSpectrum {
    pub resolution: i32,
    pub minimum_frequency: f32,
    pub maximum_frequency: f32,
    pub noise_floor_db: f32,
}
pub struct UNiagaraDataInterfaceCamera {
    pub player_controller_index: i32,
    pub b_require_current_frame_data: bool,
}
pub struct UNiagaraDataInterfaceCollisionQuery {}
pub struct UNiagaraDataInterfaceCurveBase {
    pub shader_lut: TArray<f32>,
    pub lut_min_time: f32,
    pub lut_max_time: f32,
    pub lut_inv_time_range: f32,
    pub lut_num_samples_minus_one: f32,
    pub curve_asset: UPtr<UCurveBase>,
    pub flags_200: u8,
    pub optimize_threshold: f32,
    pub exposed_name: FName,
    pub exposed_texture: UPtr<UTexture2D>,
}
pub struct UNiagaraDataInterfaceColorCurve {
    pub red_curve: FRichCurve,
    pub green_curve: FRichCurve,
    pub blue_curve: FRichCurve,
    pub alpha_curve: FRichCurve,
    pub red_curve_cooked_editor_cache: FRichCurve,
    pub green_curve_cooked_editor_cache: FRichCurve,
    pub blue_curve_cooked_editor_cache: FRichCurve,
    pub alpha_curve_cooked_editor_cache: FRichCurve,
}
pub struct UNiagaraDataInterfaceCubeTexture {
    pub texture: UPtr<UTexture>,
    pub texture_user_parameter: FNiagaraUserParameterBinding,
}
pub struct UNiagaraDataInterfaceCurlNoise {
    pub seed: u32,
}
pub struct UNiagaraDataInterfaceCurve {
    pub curve: FRichCurve,
    pub curve_cooked_editor_cache: FRichCurve,
}
pub struct UNiagaraParticleCallbackHandler {}
pub struct INiagaraParticleCallbackHandler {}
pub struct UNiagaraDataInterfaceExport {
    pub callback_handler_parameter: FNiagaraUserParameterBinding,
    pub gpu_allocation_mode: ENDIExport_GPUAllocationMode,
    pub gpu_allocation_fixed_size: i32,
    pub gpu_allocation_per_particle_size: f32,
}
pub struct UNiagaraDataInterfaceGrid2D {
    pub clear_before_non_iteration_stage: bool,
    pub num_cells_x: i32,
    pub num_cells_y: i32,
    pub num_cells_max_axis: i32,
    pub num_attributes: i32,
    pub set_grid_from_max_axis: bool,
    pub world_b_box_size: FVector2D,
}
pub struct UNiagaraDataInterfaceGrid2DCollection {
    pub render_target_user_parameter: FNiagaraUserParameterBinding,
    pub override_buffer_format: ENiagaraGpuBufferFormat,
    pub flags_265: u8,
    pub preview_attribute: FName,
    pub managed_render_targets: TMap<u64, UPtr<UTextureRenderTarget2DArray>>,
}
pub struct UNiagaraDataInterfaceGrid2DCollectionReader {
    pub emitter_name: FString,
    pub di_name: FString,
}
pub struct UNiagaraDataInterfaceGrid3D {
    pub clear_before_non_iteration_stage: bool,
    pub num_cells: FIntVector,
    pub cell_size: f32,
    pub num_cells_max_axis: i32,
    pub set_resolution_method: ESetResolutionMethod,
    pub world_b_box_size: FVector,
}
pub struct UNiagaraDataInterfaceGrid3DCollection {
    pub num_attributes: i32,
    pub render_target_user_parameter: FNiagaraUserParameterBinding,
    pub override_buffer_format: ENiagaraGpuBufferFormat,
    pub flags_289: u8,
    pub preview_attribute: FName,
}
pub struct UNiagaraDataInterfaceGrid3DCollectionReader {
    pub emitter_name: FString,
    pub di_name: FString,
}
pub struct UNiagaraDataInterfaceIntRenderTarget2D {
    pub size: FIntPoint,
    pub flags_160: u8,
    pub preview_display_range: FVector2D,
    pub render_target_user_parameter: FNiagaraUserParameterBinding,
}
pub struct UNiagaraDataInterfaceLandscape {
    pub source_landscape: UPtr<AActor>,
    pub source_mode: ENDILandscape_SourceMode,
    pub physical_materials: TArray<UPtr<UPhysicalMaterial>>,
    pub b_virtual_textures_supported: bool,
}
pub struct UNDILandscapeSimCacheData {
    pub height_field_textures: TArray<UPtr<UTexture2D>>,
}
pub struct UNiagaraDataInterfaceMaterialInstanceDynamic {
    pub default_material_inst: UPtr<UMaterialInstanceDynamic>,
    pub instanced_material_param_binding: FNiagaraParameterBinding,
}
pub struct UNiagaraDataInterfaceMaterialParameterCollection {
    pub default_collection: UPtr<UMaterialParameterCollection>,
    pub collection_binding: FNiagaraParameterBinding,
}
pub struct UNiagaraDataInterfaceMeshRendererInfo {
    pub mesh_renderer: UPtr<UNiagaraMeshRendererProperties>,
}
pub struct UNiagaraDataInterfaceNeighborGrid3D {
    pub max_neighbors_per_cell: u32,
}
pub struct UNiagaraDataInterfaceOcclusion {}
pub struct UNiagaraDataInterfaceParticleRead {
    pub emitter_binding: FNiagaraDataInterfaceEmitterBinding,
    pub emitter_name_deprecated: FString,
}
pub struct UNiagaraDataInterfaceRasterizationGrid3D {
    pub num_attributes: i32,
    pub precision: f32,
    pub reset_value: i32,
}
pub struct UNiagaraDataInterfaceRenderTarget2D {
    pub size: FIntPoint,
    pub mip_map_generation: ENiagaraMipMapGeneration,
    pub mip_map_generation_type: ENiagaraMipMapGenerationType,
    pub override_render_target_format: ETextureRenderTargetFormat,
    pub override_render_target_filter: TextureFilter,
    pub flags_172: u8,
    pub render_target_user_parameter: FNiagaraUserParameterBinding,
}
pub struct UNiagaraDataInterfaceRenderTarget2DArray {
    pub size: FIntVector,
    pub override_render_target_format: ETextureRenderTargetFormat,
    pub override_render_target_filter: TextureFilter,
    pub flags_166: u8,
    pub render_target_user_parameter: FNiagaraUserParameterBinding,
}
pub struct UNiagaraDataInterfaceRenderTargetCube {
    pub size: i32,
    pub override_render_target_format: ETextureRenderTargetFormat,
    pub override_render_target_filter: TextureFilter,
    pub flags_158: u8,
    pub render_target_user_parameter: FNiagaraUserParameterBinding,
}
pub struct UNiagaraDataInterfaceRenderTargetVolume {
    pub size: FIntVector,
    pub override_render_target_format: ETextureRenderTargetFormat,
    pub override_render_target_filter: TextureFilter,
    pub flags_174: u8,
    pub render_target_user_parameter: FNiagaraUserParameterBinding,
}
pub struct UNiagaraDataInterfaceRigidMeshCollisionQuery {
    pub tag_deprecated: FString,
    pub actor_tags: TArray<FName>,
    pub component_tags: TArray<FName>,
    pub source_actors: TArray<TSoftObjectPtr<AActor>>,
    pub only_use_moveable: bool,
    pub use_complex_collisions: bool,
    pub b_filter_by_object_type: bool,
    pub global_search_allowed: bool,
    pub global_search_forced: bool,
    pub global_search_fallback_unscripted: bool,
    pub max_num_primitives: i32,
}
pub struct UNiagaraDIRigidMeshCollisionFunctionLibrary {}
pub struct UNiagaraDataInterfaceSkeletalMesh {
    pub source_mode: ENDISkeletalMesh_SourceMode,
    pub preview_mesh: TSoftObjectPtr<USkeletalMesh>,
    pub default_mesh: UPtr<USkeletalMesh>,
    pub soft_source_actor: TSoftObjectPtr<AActor>,
    pub component_tags: TArray<FName>,
    pub source_deprecated: UPtr<AActor>,
    pub source_component: UPtr<USkeletalMeshComponent>,
    pub mesh_user_parameter: FNiagaraUserParameterBinding,
    pub skinning_mode: ENDISkeletalMesh_SkinningMode,
    pub sampling_regions: TArray<FName>,
    pub whole_mesh_lod: i32,
    pub filtered_bones: TArray<FName>,
    pub filtered_sockets: TArray<FName>,
    pub exclude_bone_name: FName,
    pub flags_444: u8,
    pub uv_set_index: i32,
    pub b_require_current_frame_data: bool,
    pub b_read_deformed_geometry: bool,
}
pub struct UNiagaraDataInterfaceSparseVolumeTexture {
    pub sparse_volume_texture: UPtr<USparseVolumeTexture>,
    pub sparse_volume_texture_user_parameter: FNiagaraUserParameterBinding,
    pub blocking_streaming_requests: bool,
}
pub struct UNiagaraDataInterfaceSpline {
    pub soft_source_actor: TSoftObjectPtr<AActor>,
    pub source_deprecated: UPtr<AActor>,
    pub spline_user_parameter: FNiagaraUserParameterBinding,
    pub b_use_lut: bool,
    pub num_lut_steps: i32,
}
pub struct UNiagaraDataInterfaceSpriteRendererInfo {
    pub sprite_renderer: UPtr<UNiagaraSpriteRendererProperties>,
}
pub struct UNiagaraDataInterfaceTexture {
    pub texture: UPtr<UTexture>,
    pub texture_user_parameter: FNiagaraUserParameterBinding,
}
pub struct UNiagaraDataInterfaceVector2DCurve {
    pub x_curve: FRichCurve,
    pub y_curve: FRichCurve,
    pub x_curve_cooked_editor_cache: FRichCurve,
    pub y_curve_cooked_editor_cache: FRichCurve,
}
pub struct UNiagaraDataInterfaceVector4Curve {
    pub x_curve: FRichCurve,
    pub y_curve: FRichCurve,
    pub z_curve: FRichCurve,
    pub w_curve: FRichCurve,
    pub x_curve_cooked_editor_cache: FRichCurve,
    pub y_curve_cooked_editor_cache: FRichCurve,
    pub z_curve_cooked_editor_cache: FRichCurve,
    pub w_curve_cooked_editor_cache: FRichCurve,
}
pub struct UNiagaraDataInterfaceVectorCurve {
    pub x_curve: FRichCurve,
    pub y_curve: FRichCurve,
    pub z_curve: FRichCurve,
    pub x_curve_cooked_editor_cache: FRichCurve,
    pub y_curve_cooked_editor_cache: FRichCurve,
    pub z_curve_cooked_editor_cache: FRichCurve,
}
pub struct UNiagaraDataInterfaceVectorField {
    pub field: UPtr<UVectorField>,
    pub b_tile_x: bool,
    pub b_tile_y: bool,
    pub b_tile_z: bool,
}
pub struct UNiagaraDataInterfaceVirtualTextureSample {
    pub texture: UPtr<UTexture>,
    pub texture_user_parameter: FNiagaraUserParameterBinding,
}
pub struct UNiagaraDataInterfaceVolumeCache {
    pub volume_cache: UPtr<UVolumeCache>,
}
pub struct UNiagaraDataInterfaceVolumeTexture {
    pub texture: UPtr<UTexture>,
    pub texture_user_parameter: FNiagaraUserParameterBinding,
}
pub struct UNiagaraDebugHUDSettings {
    pub data: FNiagaraDebugHUDSettingsData,
}
pub struct UNiagaraDecalRendererProperties {
    pub material: UPtr<UMaterialInterface>,
    pub mic_material: UPtr<UMaterialInstanceConstant>,
    pub material_parameter_binding: FNiagaraParameterBinding,
    pub source_mode: ENiagaraRendererSourceDataMode,
    pub renderer_visibility: i32,
    pub decal_screen_size_fade: f32,
    pub position_binding: FNiagaraVariableAttributeBinding,
    pub decal_orientation_binding: FNiagaraVariableAttributeBinding,
    pub decal_size_binding: FNiagaraVariableAttributeBinding,
    pub decal_fade_binding: FNiagaraVariableAttributeBinding,
    pub decal_sort_order_binding: FNiagaraVariableAttributeBinding,
    pub decal_color_binding: FNiagaraVariableAttributeBinding,
    pub decal_visible_binding: FNiagaraVariableAttributeBinding,
    pub renderer_visibility_tag_binding: FNiagaraVariableAttributeBinding,
    pub material_parameters: FNiagaraRendererMaterialParameters,
}
pub struct UNiagaraEditorDataBase {}
pub struct UNiagaraEditorParametersAdapterBase {}
pub struct ANiagaraEditorPreviewActor {
    pub motion_duration: f64,
    pub playback_type: ENiagaraEditorPreviewActorPlaybackType,
    pub motion_type: ENiagaraEditorPreviewActorShapeType,
    pub shape_tension: f64,
    pub shape_scale: f64,
    pub shape_rotation: FRotator,
    pub custom_shape_points: TArray<FVector>,
    pub circle_radius: f64,
    pub circle_end_radius: TOptional<f64>,
    pub circle_rotation_rate: TOptional<f64>,
    pub square_size: FVector2D,
    pub triangle_size: FVector2D,
    pub rotation_mode: ENiagaraEditorPreviewActorRotationMode,
    pub niagara_component: UPtr<UNiagaraComponent>,
    pub arrow_component: UPtr<UArrowComponent>,
}
pub struct UNiagaraSignificanceHandler {}
pub struct UNiagaraSignificanceHandlerDistance {}
pub struct UNiagaraSignificanceHandlerAge {}
pub struct UNiagaraEffectType {
    pub b_allow_culling_for_local_players: bool,
    pub update_frequency: ENiagaraScalabilityUpdateFrequency,
    pub cull_reaction: ENiagaraCullReaction,
    pub significance_handler: UPtr<UNiagaraSignificanceHandler>,
    pub detail_level_scalability_settings_deprecated: TArray<
        FNiagaraSystemScalabilitySettings,
    >,
    pub system_scalability_settings: FNiagaraSystemScalabilitySettingsArray,
    pub emitter_scalability_settings: FNiagaraEmitterScalabilitySettingsArray,
    pub validation_rules: TArray<UPtr<UNiagaraValidationRule>>,
    pub validation_rule_sets: TArray<UPtr<UNiagaraValidationRuleSet>>,
    pub performance_baseline_controller: UPtr<UNiagaraBaselineController>,
    pub perf_baseline_stats: FNiagaraPerfBaselineStats,
    pub perf_baseline_version: FGuid,
}
pub struct UNiagaraEmitterBase {
    pub unique_emitter_name: FString,
}
pub struct UNiagaraEmitter {
    pub exposed_version: FGuid,
    pub b_versioning_enabled: bool,
    pub version_data: TArray<FVersionedNiagaraEmitterData>,
    pub b_is_inheritable: bool,
    pub template_asset_description: FText,
    pub preview_movie_path: FSoftObjectPath,
    pub category: FText,
    pub thumbnail_image: UPtr<UTexture2D>,
    pub library_visibility: ENiagaraScriptLibraryVisibility,
    pub version_to_open_in_editor: FGuid,
    pub b_local_space_deprecated: bool,
    pub b_determinism_deprecated: bool,
    pub random_seed_deprecated: i32,
    pub allocation_mode_deprecated: EParticleAllocationMode,
    pub pre_allocation_count_deprecated: i32,
    pub update_script_props_deprecated: FNiagaraEmitterScriptProperties,
    pub spawn_script_props_deprecated: FNiagaraEmitterScriptProperties,
    pub template_specification_deprecated: ENiagaraScriptTemplateSpecification,
    pub emitter_spawn_script_props_deprecated: FNiagaraEmitterScriptProperties,
    pub emitter_update_script_props_deprecated: FNiagaraEmitterScriptProperties,
    pub attributes_to_preserve_deprecated: TArray<FString>,
    pub parent_scratch_pad_scripts_deprecated: TArray<UPtr<UNiagaraScript>>,
    pub sim_target_deprecated: ENiagaraSimTarget,
    pub fixed_bounds_deprecated: FBox,
    pub min_detail_level_deprecated: i32,
    pub max_detail_level_deprecated: i32,
    pub global_spawn_count_scale_overrides_deprecated: FNiagaraDetailsLevelScaleOverrides,
    pub platforms_deprecated: FNiagaraPlatformSet,
    pub scalability_overrides_deprecated: FNiagaraEmitterScalabilityOverrides,
    pub flags_728: u8,
    pub renderer_bindings_deprecated: FNiagaraParameterStore,
    pub flags_1152: u8,
    pub max_gpu_particles_spawn_per_frame_deprecated: u32,
    pub asset_tags_deprecated: TArray<FNiagaraAssetTagDefinitionReference>,
    pub graph_source_deprecated: UPtr<UNiagaraScriptSourceBase>,
    pub b_expose_to_library_deprecated: bool,
    pub b_is_template_asset_deprecated: bool,
    pub scratch_pad_scripts_deprecated: TArray<UPtr<UNiagaraScript>>,
    pub change_id: FGuid,
    pub editor_data_deprecated: UPtr<UNiagaraEditorDataBase>,
    pub editor_parameters_deprecated: UPtr<UNiagaraEditorParametersAdapterBase>,
    pub renderer_properties_deprecated: TArray<UPtr<UNiagaraRendererProperties>>,
    pub event_handler_script_props_deprecated: TArray<FNiagaraEventScriptProperties>,
    pub simulation_stages_deprecated: TArray<UPtr<UNiagaraSimulationStageBase>>,
    pub gpu_compute_script_deprecated: UPtr<UNiagaraScript>,
    pub shared_event_generator_ids_deprecated: TArray<FName>,
    pub parent_deprecated: UPtr<UNiagaraEmitter>,
    pub parent_at_last_merge_deprecated: UPtr<UNiagaraEmitter>,
    pub parameter_definitions_subscriptions: TArray<FParameterDefinitionsSubscription>,
    pub message_key_to_message_map_deprecated: TMap<
        FGuid,
        UPtr<UNiagaraMessageDataBase>,
    >,
    pub message_store: FNiagaraMessageStore,
}
pub struct UNiagaraEventReceiverEmitterAction {}
pub struct UNiagaraEventReceiverEmitterAction_SpawnParticles {
    pub num_particles: u32,
}
pub struct UNiagaraFunctionLibrary {}
pub struct ANiagaraLensEffectBase {
    pub desired_relative_transform: FTransform,
    pub base_authored_fov: f32,
    pub flags_1284: u8,
    pub emitters_to_treat_as_same: TArray<TSubclassOf<AActor>>,
    pub owning_camera_manager: UPtr<APlayerCameraManager>,
}
pub struct UNiagaraLightRendererProperties {
    pub source_mode: ENiagaraRendererSourceDataMode,
    pub flags_676: u8,
    pub radius_scale: f32,
    pub default_exponent: f32,
    pub specular_scale: f32,
    pub diffuse_scale: f32,
    pub color_add: FVector3f,
    pub inverse_exposure_blend: f32,
    pub renderer_visibility: i32,
    pub light_rendering_enabled_binding: FNiagaraVariableAttributeBinding,
    pub light_exponent_binding: FNiagaraVariableAttributeBinding,
    pub position_binding: FNiagaraVariableAttributeBinding,
    pub color_binding: FNiagaraVariableAttributeBinding,
    pub radius_binding: FNiagaraVariableAttributeBinding,
    pub volumetric_scattering_binding: FNiagaraVariableAttributeBinding,
    pub renderer_visibility_tag_binding: FNiagaraVariableAttributeBinding,
    pub specular_scale_binding: FNiagaraVariableAttributeBinding,
    pub diffuse_scale_binding: FNiagaraVariableAttributeBinding,
}
pub struct UNiagaraMeshRendererProperties {
    pub meshes: TArray<FNiagaraMeshRendererMeshProperties>,
    pub meshes_binding: FNiagaraParameterBinding,
    pub source_mode: ENiagaraRendererSourceDataMode,
    pub sort_mode: ENiagaraSortMode,
    pub sort_precision: ENiagaraRendererSortPrecision,
    pub gpu_translucent_latency: ENiagaraRendererGpuTranslucentLatency,
    pub flags_1036: u8,
    pub flags_1040: u8,
    pub override_materials: TArray<FNiagaraMeshMaterialOverride>,
    pub mic_override_materials: TArray<FNiagaraMeshMICOverride>,
    pub sub_image_size: FVector2D,
    pub locked_axis: FVector,
    pub mesh_bounds_scale: FVector,
    pub facing_mode: ENiagaraMeshFacingMode,
    pub locked_axis_space: ENiagaraMeshLockedAxisSpace,
    pub min_camera_distance: f32,
    pub max_camera_distance: f32,
    pub renderer_visibility: u32,
    pub position_binding: FNiagaraVariableAttributeBinding,
    pub color_binding: FNiagaraVariableAttributeBinding,
    pub velocity_binding: FNiagaraVariableAttributeBinding,
    pub mesh_orientation_binding: FNiagaraVariableAttributeBinding,
    pub scale_binding: FNiagaraVariableAttributeBinding,
    pub sub_image_index_binding: FNiagaraVariableAttributeBinding,
    pub dynamic_material_binding: FNiagaraVariableAttributeBinding,
    pub dynamic_material1_binding: FNiagaraVariableAttributeBinding,
    pub dynamic_material2_binding: FNiagaraVariableAttributeBinding,
    pub dynamic_material3_binding: FNiagaraVariableAttributeBinding,
    pub material_random_binding: FNiagaraVariableAttributeBinding,
    pub custom_sorting_binding: FNiagaraVariableAttributeBinding,
    pub normalized_age_binding: FNiagaraVariableAttributeBinding,
    pub camera_offset_binding: FNiagaraVariableAttributeBinding,
    pub renderer_visibility_tag_binding: FNiagaraVariableAttributeBinding,
    pub mesh_index_binding: FNiagaraVariableAttributeBinding,
    pub material_parameters: FNiagaraRendererMaterialParameters,
    pub material_parameter_bindings_deprecated: TArray<FNiagaraMaterialAttributeBinding>,
    pub prev_position_binding: FNiagaraVariableAttributeBinding,
    pub prev_scale_binding: FNiagaraVariableAttributeBinding,
    pub prev_mesh_orientation_binding: FNiagaraVariableAttributeBinding,
    pub prev_camera_offset_binding: FNiagaraVariableAttributeBinding,
    pub prev_velocity_binding: FNiagaraVariableAttributeBinding,
    pub first_flipbook_frame: UPtr<UStaticMesh>,
    pub flipbook_suffix_format: FString,
    pub flipbook_suffix_num_digits: u32,
    pub num_flipbook_frames: u32,
    pub material_param_valid_mask: u32,
    pub particle_mesh_deprecated: UPtr<UStaticMesh>,
    pub pivot_offset_deprecated: FVector,
    pub pivot_offset_space_deprecated: ENiagaraMeshPivotOffsetSpace,
}
pub struct UNiagaraMessageDataBase {}
pub struct UNiagaraParameterCollectionInstance {
    pub collection: UPtr<UNiagaraParameterCollection>,
    pub overriden_parameters: TArray<FNiagaraVariable>,
    pub parameter_storage: FNiagaraParameterStore,
    pub source_material_collection_instance: UPtr<UMaterialParameterCollectionInstance>,
}
pub struct UNiagaraParameterCollection {
    pub namespace: FName,
    pub parameters: TArray<FNiagaraVariable>,
    pub source_material_collection: UPtr<UMaterialParameterCollection>,
    pub default_instance: UPtr<UNiagaraParameterCollectionInstance>,
    pub compile_id: FGuid,
}
pub struct UNiagaraBaselineController {
    pub test_duration: f32,
    pub effect_type: UPtr<UNiagaraEffectType>,
    pub owner: UPtr<ANiagaraPerfBaselineActor>,
    pub system: TSoftObjectPtr<UNiagaraSystem>,
}
pub struct UNiagaraBaselineController_Basic {
    pub num_instances: i32,
    pub spawned_components: TArray<UPtr<UNiagaraComponent>>,
}
pub struct ANiagaraPerfBaselineActor {
    pub controller: UPtr<UNiagaraBaselineController>,
    pub label: UPtr<UTextRenderComponent>,
}
pub struct UNiagaraPrecompileContainer {
    pub scripts: TArray<UPtr<UNiagaraScript>>,
    pub system: UPtr<UNiagaraSystem>,
}
pub struct ANiagaraPreviewBase {}
pub struct UNiagaraPreviewAxis {}
pub struct UNiagaraPreviewAxis_InterpParamBase {
    pub param: FName,
    pub count: i32,
}
pub struct UNiagaraPreviewAxis_InterpParamInt32 {
    pub min: i32,
    pub max: i32,
}
pub struct UNiagaraPreviewAxis_InterpParamFloat {
    pub min: f32,
    pub max: f32,
}
pub struct UNiagaraPreviewAxis_InterpParamVector2D {
    pub min: FVector2D,
    pub max: FVector2D,
}
pub struct UNiagaraPreviewAxis_InterpParamVector {
    pub min: FVector,
    pub max: FVector,
}
pub struct UNiagaraPreviewAxis_InterpParamVector4 {
    pub min: FVector4,
    pub max: FVector4,
}
pub struct UNiagaraPreviewAxis_InterpParamLinearColor {
    pub min: FLinearColor,
    pub max: FLinearColor,
}
pub struct ANiagaraPreviewGrid {
    pub system: UPtr<UNiagaraSystem>,
    pub reset_mode: ENiagaraPreviewGridResetMode,
    pub preview_axis_x: UPtr<UNiagaraPreviewAxis>,
    pub preview_axis_y: UPtr<UNiagaraPreviewAxis>,
    pub preview_class: TSubclassOf<ANiagaraPreviewBase>,
    pub spacing_x: f32,
    pub spacing_y: f32,
    pub num_x: i32,
    pub num_y: i32,
    pub preview_components: TArray<UPtr<UChildActorComponent>>,
    pub sprite_component: UPtr<UBillboardComponent>,
    pub arrow_component: UPtr<UArrowComponent>,
}
pub struct UNiagaraRibbonRendererProperties {
    pub material: UPtr<UMaterialInterface>,
    pub mic_material: UPtr<UMaterialInstanceConstant>,
    pub material_user_param_binding: FNiagaraUserParameterBinding,
    pub uv0_settings: FNiagaraRibbonUVSettings,
    pub uv1_settings: FNiagaraRibbonUVSettings,
    pub facing_mode: ENiagaraRibbonFacingMode,
    pub uv0_tiling_distance_deprecated: f32,
    pub uv0_scale_deprecated: FVector2D,
    pub uv0_offset_deprecated: FVector2D,
    pub uv0_age_offset_mode_deprecated: ENiagaraRibbonAgeOffsetMode,
    pub uv1_tiling_distance_deprecated: f32,
    pub uv1_scale_deprecated: FVector2D,
    pub uv1_offset_deprecated: FVector2D,
    pub uv1_age_offset_mode_deprecated: ENiagaraRibbonAgeOffsetMode,
    pub max_num_ribbons: i32,
    pub draw_direction: ENiagaraRibbonDrawDirection,
    pub shape: ENiagaraRibbonShapeMode,
    pub flags_930: u8,
    pub flags_931: u8,
    pub width_segmentation_count: i32,
    pub multi_plane_count: i32,
    pub tube_subdivisions: i32,
    pub custom_vertices: TArray<FNiagaraRibbonShapeCustomVertex>,
    pub tessellation_mode: ENiagaraRibbonTessellationMode,
    pub curve_tension: f32,
    pub tessellation_factor: i32,
    pub tessellation_angle: f32,
    pub position_binding: FNiagaraVariableAttributeBinding,
    pub color_binding: FNiagaraVariableAttributeBinding,
    pub velocity_binding: FNiagaraVariableAttributeBinding,
    pub normalized_age_binding: FNiagaraVariableAttributeBinding,
    pub ribbon_twist_binding: FNiagaraVariableAttributeBinding,
    pub ribbon_width_binding: FNiagaraVariableAttributeBinding,
    pub ribbon_facing_binding: FNiagaraVariableAttributeBinding,
    pub ribbon_id_binding: FNiagaraVariableAttributeBinding,
    pub ribbon_link_order_binding: FNiagaraVariableAttributeBinding,
    pub material_random_binding: FNiagaraVariableAttributeBinding,
    pub dynamic_material_binding: FNiagaraVariableAttributeBinding,
    pub dynamic_material1_binding: FNiagaraVariableAttributeBinding,
    pub dynamic_material2_binding: FNiagaraVariableAttributeBinding,
    pub dynamic_material3_binding: FNiagaraVariableAttributeBinding,
    pub ribbon_uv_distance: FNiagaraVariableAttributeBinding,
    pub u0_override_binding: FNiagaraVariableAttributeBinding,
    pub v0_range_override_binding: FNiagaraVariableAttributeBinding,
    pub u1_override_binding: FNiagaraVariableAttributeBinding,
    pub v1_range_override_binding: FNiagaraVariableAttributeBinding,
    pub material_parameters: FNiagaraRendererMaterialParameters,
    pub material_parameter_bindings_deprecated: TArray<FNiagaraMaterialAttributeBinding>,
    pub prev_position_binding: FNiagaraVariableAttributeBinding,
    pub prev_ribbon_width_binding: FNiagaraVariableAttributeBinding,
    pub prev_ribbon_facing_binding: FNiagaraVariableAttributeBinding,
    pub prev_ribbon_twist_binding: FNiagaraVariableAttributeBinding,
    pub material_param_valid_mask: u32,
}
pub struct UNiagaraScratchPadContainer {
    pub scripts: TArray<UPtr<UNiagaraScript>>,
}
pub struct UNiagaraScript {
    pub validation_rules: TArray<UPtr<UNiagaraValidationRule>>,
    pub usage: ENiagaraScriptUsage,
    pub usage_id: FGuid,
    pub preview_movie_path: FSoftObjectPath,
    pub exposed_version: FGuid,
    pub b_versioning_enabled: bool,
    pub version_data: TArray<FVersionedNiagaraScriptData>,
    pub rapid_iteration_parameters: FNiagaraParameterStore,
    pub rapid_iteration_parameters_cooked_editor_cache: FNiagaraParameterStore,
    pub version_to_open_in_editor: FGuid,
    pub usage_index_deprecated: i32,
    pub module_usage_bitmask_deprecated: i32,
    pub category_deprecated: FText,
    pub provided_dependencies_deprecated: TArray<FName>,
    pub required_dependencies_deprecated: TArray<FNiagaraModuleDependency>,
    pub flags_1096: u8,
    pub deprecation_message_deprecated: FText,
    pub deprecation_recommendation_deprecated: UPtr<UNiagaraScript>,
    pub conversion_utility_deprecated: TSubclassOf<UNiagaraConvertInPlaceUtilityBase>,
    pub flags_1136: u8,
    pub experimental_message_deprecated: FText,
    pub note_message_deprecated: FText,
    pub flags_1176: u8,
    pub library_visibility_deprecated: ENiagaraScriptLibraryVisibility,
    pub numeric_output_type_selection_mode_deprecated: ENiagaraNumericOutputTypeSelectionMode,
    pub description_deprecated: FText,
    pub keywords_deprecated: FText,
    pub collapsed_view_format_deprecated: FText,
    pub script_meta_data_deprecated: TMap<FName, FString>,
    pub source_deprecated: UPtr<UNiagaraScriptSourceBase>,
    pub script_execution_param_store_cpu: FNiagaraScriptExecutionParameterStore,
    pub script_execution_param_store_gpu: FNiagaraScriptExecutionParameterStore,
    pub script_execution_param_store: FNiagaraScriptExecutionParameterStore,
    pub script_execution_bound_parameters: TArray<FNiagaraBoundParameter>,
    pub cached_script_vm_id: FNiagaraVMExecutableDataId,
    pub active_compile_roots: TArray<UPtr<UObject>>,
    pub cached_script_vm: FNiagaraVMExecutableData,
    pub cached_parameter_collection_references: TArray<
        UPtr<UNiagaraParameterCollection>,
    >,
    pub cached_default_data_interfaces: TArray<FNiagaraScriptDataInterfaceInfo>,
    pub resolved_data_interfaces: TArray<FNiagaraScriptResolvedDataInterfaceInfo>,
    pub resolved_user_data_interface_bindings: TArray<
        FNiagaraResolvedUserDataInterfaceBinding,
    >,
    pub cached_default_u_objects: TArray<FNiagaraScriptUObjectCompileInfo>,
    pub resolved_u_object_infos: TArray<FNiagaraResolvedUObjectInfo>,
    pub data_interfaces_pending_invalidation: TArray<UPtr<UNiagaraDataInterface>>,
}
pub struct UNiagaraSettings {
    pub additional_parameter_types: TArray<FSoftObjectPath>,
    pub additional_payload_types: TArray<FSoftObjectPath>,
    pub additional_parameter_enums: TArray<FSoftObjectPath>,
    pub b_system_viewport_in_orbit_mode: bool,
    pub b_show_convertible_inputs_in_stack: bool,
    pub quick_sim_cache_capture_frame_count: i32,
    pub b_systems_support_large_world_coordinates: bool,
    pub large_world_coordinate_tile_update_mode: ENiagaraLwcTileUpdateMode,
    pub large_world_coordinate_max_tiles_before_reset: u32,
    pub b_enforce_strict_stack_types: bool,
    pub b_accurate_quat_interpolation: bool,
    pub invalid_namespace_write_severity: ENiagaraCompileErrorSeverity,
    pub b_limit_delta_time: bool,
    pub max_delta_time_per_tick: f32,
    pub default_effect_type: FSoftObjectPath,
    pub b_allow_create_actor_from_system_with_no_effect_type: bool,
    pub position_pin_type_color: FLinearColor,
    pub byte_code_strip_option: ENiagaraStripScriptByteCodeOption,
    pub compilation_mode: ENiagaraCompilationMode,
    pub b_enable_hlsl2021: bool,
    pub quality_levels: TArray<FText>,
    pub component_renderer_warnings_per_class: TMap<FString, FText>,
    pub default_render_target_format: ETextureRenderTargetFormat,
    pub default_grid_format: ENiagaraGpuBufferFormat,
    pub default_renderer_motion_vector_setting: ENiagaraDefaultRendererMotionVectorSetting,
    pub default_pixel_coverage_mode: ENiagaraDefaultRendererPixelCoverageMode,
    pub default_sort_precision: ENiagaraDefaultSortPrecision,
    pub default_gpu_translucent_latency: ENiagaraDefaultGpuTranslucentLatency,
    pub default_light_inverse_exposure_blend: f32,
    pub ndi_skel_mesh_support_reading_deformed_geometry: bool,
    pub ndi_skel_mesh_support16_bit_index_weight: bool,
    pub ndi_skel_mesh_gpu_max_influences: ENDISkelMesh_GpuMaxInfluences,
    pub ndi_skel_mesh_gpu_uniform_sampling_format: ENDISkelMesh_GpuUniformSamplingFormat,
    pub ndi_skel_mesh_adjacency_triangle_index_format: ENDISkelMesh_AdjacencyTriangleIndexFormat,
    pub ndi_static_mesh_allow_distance_fields: bool,
    pub ndi_collision_query_async_gpu_trace_provider_order: TArray<
        ENDICollisionQuery_AsyncGpuTraceProvider,
    >,
    pub sim_cache_auxiliary_file_base_path: FString,
    pub sim_cache_max_cpu_memory_volumetrics: i64,
    pub platform_set_redirects: TArray<FNiagaraPlatformSetRedirect>,
    pub b_generate_meta_data_on_compile: bool,
}
pub struct UNiagaraSimCache {
    pub cache_guid: FGuid,
    pub soft_niagara_system: TSoftObjectPtr<UNiagaraSystem>,
    pub start_seconds: f32,
    pub duration_seconds: f32,
    pub create_parameters: FNiagaraSimCacheCreateParameters,
    pub b_needs_read_component_mapping_recache: bool,
    pub b_can_run_async: bool,
    pub cached_script_vm_ids: TArray<FNiagaraVMExecutableDataId>,
    pub cache_layout: FNiagaraSimCacheLayout,
    pub cache_frames: TArray<FNiagaraSimCacheFrame>,
    pub data_interface_storage: TMap<FNiagaraVariableBase, UPtr<UObject>>,
    pub debug_data: UPtr<UNiagaraSimCacheDebugData>,
}
pub struct UNiagaraSimCacheDebugData {
    pub frames: TArray<FNiagaraSimCacheDebugDataFrame>,
}
pub struct UAsyncNiagaraCaptureSimCache {
    pub capture_sim_cache: UPtr<UNiagaraSimCache>,
    pub capture_component: UPtr<UNiagaraComponent>,
    pub capture_complete: FAsyncNiagaraCaptureSimCache_CaptureComplete,
}
pub struct UNiagaraSimCacheFunctionLibrary {}
pub struct UNiagaraSimulationStageBase {
    pub script: UPtr<UNiagaraScript>,
    pub simulation_stage_name: FName,
    pub flags_116: u8,
    pub outer_emitter_version: FGuid,
}
pub struct UNiagaraSimulationStageGeneric {
    pub enabled_binding: FNiagaraVariableAttributeBinding,
    pub iteration_source: ENiagaraIterationSource,
    pub num_iterations: FNiagaraParameterBindingWithValue,
    pub execute_behavior: ENiagaraSimStageExecuteBehavior,
    pub flags_892: u8,
    pub data_interface: FNiagaraVariableDataInterfaceBinding,
    pub flags_968: u8,
    pub particle_iteration_state_binding: FNiagaraVariableAttributeBinding,
    pub particle_iteration_state_range: FIntPoint,
    pub flags_1376: u8,
    pub override_gpu_dispatch_num_threads_x: FNiagaraParameterBindingWithValue,
    pub override_gpu_dispatch_num_threads_y: FNiagaraParameterBindingWithValue,
    pub override_gpu_dispatch_num_threads_z: FNiagaraParameterBindingWithValue,
    pub direct_dispatch_type: ENiagaraGpuDispatchType,
    pub direct_dispatch_element_type: ENiagaraDirectDispatchElementType,
    pub element_count_x: FNiagaraParameterBindingWithValue,
    pub element_count_y: FNiagaraParameterBindingWithValue,
    pub element_count_z: FNiagaraParameterBindingWithValue,
    pub element_count_binding_deprecated: FNiagaraVariableAttributeBinding,
    pub element_count_x_binding_deprecated: FNiagaraVariableAttributeBinding,
    pub element_count_y_binding_deprecated: FNiagaraVariableAttributeBinding,
    pub element_count_z_binding_deprecated: FNiagaraVariableAttributeBinding,
    pub flags_5072: u8,
    pub override_gpu_dispatch_type_deprecated: ENiagaraGpuDispatchType,
    pub iterations_deprecated: i32,
    pub num_iterations_binding_deprecated: FNiagaraVariableAttributeBinding,
    pub override_gpu_dispatch_num_threads_deprecated: FIntVector,
}
pub struct UNiagaraSpriteRendererProperties {
    pub material: UPtr<UMaterialInterface>,
    pub mic_material: UPtr<UMaterialInstanceConstant>,
    pub material_user_param_binding: FNiagaraUserParameterBinding,
    pub source_mode: ENiagaraRendererSourceDataMode,
    pub alignment: ENiagaraSpriteAlignment,
    pub facing_mode: ENiagaraSpriteFacingMode,
    pub sort_mode: ENiagaraSortMode,
    pub macro_uv_radius: f32,
    pub pivot_in_uv_space: FVector2D,
    pub sub_image_size: FVector2D,
    pub flags_800: u8,
    pub sort_precision: ENiagaraRendererSortPrecision,
    pub gpu_translucent_latency: ENiagaraRendererGpuTranslucentLatency,
    pub pixel_coverage_mode: ENiagaraRendererPixelCoverageMode,
    pub pixel_coverage_blend: f32,
    pub min_facing_camera_blend_distance: f32,
    pub max_facing_camera_blend_distance: f32,
    pub min_camera_distance: f32,
    pub max_camera_distance: f32,
    pub renderer_visibility: u32,
    pub position_binding: FNiagaraVariableAttributeBinding,
    pub color_binding: FNiagaraVariableAttributeBinding,
    pub velocity_binding: FNiagaraVariableAttributeBinding,
    pub sprite_rotation_binding: FNiagaraVariableAttributeBinding,
    pub sprite_size_binding: FNiagaraVariableAttributeBinding,
    pub sprite_facing_binding: FNiagaraVariableAttributeBinding,
    pub sprite_alignment_binding: FNiagaraVariableAttributeBinding,
    pub sub_image_index_binding: FNiagaraVariableAttributeBinding,
    pub dynamic_material_binding: FNiagaraVariableAttributeBinding,
    pub dynamic_material1_binding: FNiagaraVariableAttributeBinding,
    pub dynamic_material2_binding: FNiagaraVariableAttributeBinding,
    pub dynamic_material3_binding: FNiagaraVariableAttributeBinding,
    pub camera_offset_binding: FNiagaraVariableAttributeBinding,
    pub uv_scale_binding: FNiagaraVariableAttributeBinding,
    pub pivot_offset_binding: FNiagaraVariableAttributeBinding,
    pub material_random_binding: FNiagaraVariableAttributeBinding,
    pub custom_sorting_binding: FNiagaraVariableAttributeBinding,
    pub normalized_age_binding: FNiagaraVariableAttributeBinding,
    pub renderer_visibility_tag_binding: FNiagaraVariableAttributeBinding,
    pub material_parameters: FNiagaraRendererMaterialParameters,
    pub material_parameter_bindings_deprecated: TArray<FNiagaraMaterialAttributeBinding>,
    pub prev_position_binding: FNiagaraVariableAttributeBinding,
    pub prev_velocity_binding: FNiagaraVariableAttributeBinding,
    pub prev_sprite_rotation_binding: FNiagaraVariableAttributeBinding,
    pub prev_sprite_size_binding: FNiagaraVariableAttributeBinding,
    pub prev_sprite_facing_binding: FNiagaraVariableAttributeBinding,
    pub prev_sprite_alignment_binding: FNiagaraVariableAttributeBinding,
    pub prev_camera_offset_binding: FNiagaraVariableAttributeBinding,
    pub prev_pivot_offset_binding: FNiagaraVariableAttributeBinding,
    pub b_use_material_cutout_texture: bool,
    pub cutout_texture: UPtr<UTexture2D>,
    pub bounding_mode: ESubUVBoundingVertexCount,
    pub opacity_source_mode: EOpacitySourceMode,
    pub alpha_threshold: f32,
    pub material_param_valid_mask: u32,
}
pub struct UNiagaraSystemCollection {
    pub system_collection: FNiagaraSystemCollectionData,
    pub b_load_immediately: bool,
}
pub struct UNiagaraValidationRule {
    pub b_is_config_disabled: bool,
}
pub struct UNiagaraVolumeRendererProperties {
    pub material: UPtr<UMaterialInterface>,
    pub mic_material: UPtr<UMaterialInstanceConstant>,
    pub material_parameter_binding: FNiagaraParameterBinding,
    pub renderer_visibility: i32,
    pub step_factor: f32,
    pub lighting_downsample_factor: f32,
    pub shadow_step_factor: f32,
    pub shadow_bias_factor: f32,
    pub renderer_visibility_tag_binding: FNiagaraVariableAttributeBinding,
    pub volume_resolution_max_axis_binding: FNiagaraVariableAttributeBinding,
    pub volume_world_space_size_binding: FNiagaraVariableAttributeBinding,
    pub material_parameters: FNiagaraRendererMaterialParameters,
}
pub struct UNiagaraStatelessModule {
    pub flags_96: u8,
}
pub struct UNiagaraStatelessModule_AccelerationForce {
    pub acceleration_distribution: FNiagaraDistributionRangeVector3,
    pub coordinate_space: ENiagaraCoordinateSpace,
}
pub struct UNiagaraStatelessModule_AddVelocity {
    pub velocity_type: ENSM_VelocityType,
    pub linear_velocity_distribution: FNiagaraDistributionRangeVector3,
    pub linear_velocity_scale: FNiagaraDistributionRangeFloat,
    pub cone_velocity_distribution: FNiagaraDistributionRangeFloat,
    pub cone_rotation: FRotator,
    pub cone_angle: f32,
    pub inner_cone: f32,
    pub point_velocity_distribution: FNiagaraDistributionRangeFloat,
    pub point_origin: FVector3f,
    pub b_speed_falloff_from_cone_axis_enabled: bool,
    pub speed_falloff_from_cone_axis: f32,
    pub coordinate_space: ENiagaraCoordinateSpace,
}
pub struct UNiagaraStatelessModule_CalculateAccurateVelocity {}
pub struct UNiagaraStatelessModule_CameraOffset {
    pub camera_offset_distribution: FNiagaraDistributionFloat,
}
pub struct UNiagaraStatelessModule_CurlNoiseForce {
    pub noise_strength: f32,
    pub noise_frequency: f32,
}
pub struct UNiagaraStatelessModule_DecalAttributes {
    pub b_apply_orientation: bool,
    pub b_apply_size: bool,
    pub b_apply_fade: bool,
    pub orientation: FNiagaraDistributionVector3,
    pub orientation_coordinate_space: ENiagaraCoordinateSpace,
    pub size: FNiagaraDistributionVector3,
    pub fade: FNiagaraDistributionFloat,
}
pub struct UNiagaraStatelessModule_Drag {
    pub drag_distribution: FNiagaraDistributionRangeFloat,
}
pub struct UNiagaraStatelessModule_DynamicMaterialParameters {
    pub flags_104: u8,
    pub parameter0: FNiagaraStatelessDynamicParameterSet,
    pub parameter1: FNiagaraStatelessDynamicParameterSet,
    pub parameter2: FNiagaraStatelessDynamicParameterSet,
    pub parameter3: FNiagaraStatelessDynamicParameterSet,
}
pub struct UNiagaraStatelessModule_GravityForce {
    pub gravity_distribution: FNiagaraDistributionRangeVector3,
}
pub struct UNiagaraStatelessModule_InitializeParticle {
    pub lifetime_distribution: FNiagaraDistributionRangeFloat,
    pub color_distribution: FNiagaraDistributionColor,
    pub mass_distribution: FNiagaraDistributionRangeFloat,
    pub sprite_size_distribution: FNiagaraDistributionRangeVector2,
    pub sprite_rotation_distribution: FNiagaraDistributionRangeFloat,
    pub mesh_scale_distribution: FNiagaraDistributionRangeVector3,
    pub b_write_ribbon_width: bool,
    pub ribbon_width_distribution: FNiagaraDistributionRangeFloat,
    pub initial_position_distribution: FNiagaraDistributionPosition,
}
pub struct UNiagaraStatelessModule_InitialMeshOrientation {
    pub mesh_orientation_mode: ENSMInitialMeshOrientationMode,
    pub orientation_vector: FNiagaraDistributionRangeVector3,
    pub mesh_axis_to_orient: FNiagaraDistributionRangeVector3,
    pub rotation: FNiagaraDistributionRangeVector3,
    pub random_rotation_range_deprecated: FVector3f,
}
pub struct UNiagaraStatelessModule_LightAttributes {
    pub flags_104: u8,
    pub radius: FNiagaraDistributionFloat,
    pub falloff_exponent: FNiagaraDistributionFloat,
    pub diffuse_scale: FNiagaraDistributionFloat,
    pub specular_scale: FNiagaraDistributionFloat,
    pub volumetric_scattering: FNiagaraDistributionFloat,
}
pub struct UNiagaraStatelessModule_MeshIndex {
    pub mesh_index: FNiagaraDistributionRangeInt,
    pub mesh_index_weight: TArray<f32>,
}
pub struct UNiagaraStatelessModule_MeshRotationRate {
    pub b_use_rate_scale: bool,
    pub rotation_rate_distribution: FNiagaraDistributionRangeVector3,
    pub rate_scale_distribution: FNiagaraDistributionCurveVector3,
}
pub struct UNiagaraStatelessModule_RotateAroundPoint {
    pub rate: FNiagaraDistributionRangeFloat,
    pub radius: FNiagaraDistributionRangeFloat,
    pub initial_phase: FNiagaraDistributionRangeFloat,
    pub center_coordinate_space: ENiagaraCoordinateSpace,
    pub center: FNiagaraDistributionRangeVector3,
    pub rotation_coordinate_space: ENiagaraCoordinateSpace,
    pub rotation_axis: FNiagaraDistributionRangeVector3,
}
pub struct UNiagaraStatelessModule_ScaleColor {
    pub scale_distribution: FNiagaraDistributionColor,
}
pub struct UNiagaraStatelessModule_ScaleMeshSize {
    pub scale_distribution: FNiagaraDistributionVector3,
    pub scale_curve_range: FNiagaraParameterBindingWithValue,
}
pub struct UNiagaraStatelessModule_ScaleMeshSizeBySpeed {
    pub velocity_threshold: FNiagaraDistributionRangeFloat,
    pub min_scale_factor: FNiagaraDistributionRangeVector3,
    pub max_scale_factor: FNiagaraDistributionRangeVector3,
    pub b_sample_scale_factor_by_curve: bool,
    pub sample_factor_curve: FNiagaraDistributionFloat,
}
pub struct UNiagaraStatelessModule_ScaleRibbonWidth {
    pub scale_distribution: FNiagaraDistributionFloat,
    pub scale_curve_range: FNiagaraParameterBindingWithValue,
}
pub struct UNiagaraStatelessModule_ScaleSpriteSize {
    pub scale_distribution: FNiagaraDistributionVector2,
    pub scale_curve_range: FNiagaraParameterBindingWithValue,
}
pub struct UNiagaraStatelessModule_ScaleSpriteSizeBySpeed {
    pub velocity_threshold: FNiagaraDistributionRangeFloat,
    pub min_scale_factor: FNiagaraDistributionRangeVector2,
    pub max_scale_factor: FNiagaraDistributionRangeVector2,
    pub b_sample_scale_factor_by_curve: bool,
    pub sample_factor_curve: FNiagaraDistributionFloat,
}
pub struct UNiagaraStatelessModule_ShapeLocation {
    pub shape_primitive: ENSM_ShapePrimitive,
    pub box_size: FNiagaraDistributionRangeVector3,
    pub b_box_surface_only: bool,
    pub box_surface_expansion: ENSM_SurfaceExpansionMode,
    pub box_surface_thickness: FNiagaraDistributionRangeFloat,
    pub plane_size: FNiagaraDistributionRangeVector2,
    pub b_plane_edges_only: bool,
    pub plane_edge_expansion: ENSM_SurfaceExpansionMode,
    pub plane_edge_thickness: FNiagaraDistributionRangeFloat,
    pub cylinder_height: FNiagaraDistributionRangeFloat,
    pub cylinder_radius: FNiagaraDistributionRangeFloat,
    pub cylinder_height_midpoint: FNiagaraDistributionRangeFloat,
    pub b_cylinder_surface_only: bool,
    pub b_cylinder_surface_only_include_end_caps: bool,
    pub cylinder_surface_expansion: ENSM_SurfaceExpansionMode,
    pub cylinder_surface_thickness: FNiagaraDistributionRangeFloat,
    pub ring_radius: FNiagaraDistributionRangeFloat,
    pub disc_coverage: FNiagaraDistributionRangeFloat,
    pub ring_u_distribution: FNiagaraDistributionRangeFloat,
    pub sphere_radius: FNiagaraDistributionRangeFloat,
    pub coordinate_space: ENiagaraCoordinateSpace,
    pub shape_rotation: FNiagaraDistributionRangeRotator,
    pub shape_scale: FNiagaraDistributionRangeVector3,
    pub sphere_min_deprecated: f32,
    pub sphere_max_deprecated: f32,
}
pub struct UNiagaraStatelessModule_SolveVelocitiesAndForces {}
pub struct UNiagaraStatelessModule_SpriteFacingAndAlignment {
    pub b_sprite_facing_enabled: bool,
    pub b_sprite_alignment_enabled: bool,
    pub sprite_facing: FNiagaraDistributionRangeVector3,
    pub sprite_alignment: FNiagaraDistributionRangeVector3,
}
pub struct UNiagaraStatelessModule_SpriteRotationRate {
    pub b_use_rate_scale: bool,
    pub rotation_rate_distribution: FNiagaraDistributionRangeFloat,
    pub rate_scale_distribution: FNiagaraDistributionCurveFloat,
}
pub struct UNiagaraStatelessModule_SubUVAnimation {
    pub num_frames: i32,
    pub frame_index: FNiagaraDistributionRangeInt,
    pub b_start_frame_range_override_enabled: bool,
    pub b_end_frame_range_override_enabled: bool,
    pub start_frame_range_override: i32,
    pub end_frame_range_override: i32,
    pub animation_mode: ENSMSubUVAnimation_Mode,
    pub loops_per_second: f32,
    pub random_change_interval: f32,
}
pub struct UNiagaraStatelessEmitter {
    pub emitter_template_class_deprecated: TSubclassOf<UObject>,
    pub emitter_template: UPtr<UNiagaraStatelessEmitterTemplate>,
    pub flags_128: u8,
    pub allowed_feature_mask: u32,
    pub random_seed: i32,
    pub fixed_bounds: FBox,
    pub emitter_state: FNiagaraEmitterStateData,
    pub spawn_infos: TArray<FNiagaraStatelessSpawnInfo>,
    pub modules: TArray<UPtr<UNiagaraStatelessModule>>,
    pub renderer_properties: TArray<UPtr<UNiagaraRendererProperties>>,
    pub platforms: FNiagaraPlatformSet,
    pub scalability_overrides: FNiagaraEmitterScalabilityOverrides,
    pub particle_data_set_compiled_data: FNiagaraDataSetCompiledData,
    pub shader_output_variable_offsets: TArray<i32>,
    pub cached_parameter_collection_references: TArray<
        UPtr<UNiagaraParameterCollection>,
    >,
}
pub struct UNiagaraStatelessEmitterTemplate {
    pub modules: TArray<TSubclassOf<UObject>>,
    pub b_exposed_to_library: bool,
}
pub struct UNiagaraStatelessEmitterDefault {}
pub struct UVolumeCache {
    pub file_path: FString,
    pub cache_type: EVolumeCacheType,
    pub resolution: FIntVector,
    pub frame_range_start: i32,
    pub frame_range_end: i32,
}
