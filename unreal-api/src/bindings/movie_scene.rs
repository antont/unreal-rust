#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FMovieSceneChannel {}
#[repr(C, align(8))]
pub struct FMovieSceneBoolChannel {
    pub pre_infinity_extrap: ERichCurveExtrapolation,
    pub post_infinity_extrap: ERichCurveExtrapolation,
    pub times: TArray<FFrameNumber>,
    pub default_value: bool,
    pub b_has_default_value: bool,
    pub values: TArray<bool>,
    pub key_handles: FMovieSceneKeyHandleMap,
}
#[repr(C, align(8))]
pub struct FMovieSceneKeyHandleMap {}
#[repr(C, align(8))]
pub struct FMovieSceneBindingProxy {
    pub binding_id: FGuid,
    pub sequence: UPtr<UMovieSceneSequence>,
}
#[repr(C, align(8))]
pub struct FMovieSceneDynamicBinding {
    pub function: UPtr<UFunction>,
    pub resolve_params_property: TFieldPath<FProperty>,
    pub payload_variables: TMap<FName, FMovieSceneDynamicBindingPayloadVariable>,
    pub compiled_function_name: FName,
    pub resolve_params_pin_name: FName,
    pub weak_endpoint: TWeakObjectPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FMovieSceneDynamicBindingPayloadVariable {
    pub object_value: FSoftObjectPath,
    pub value: FString,
}
#[repr(C, align(8))]
pub struct FMovieSceneEvalTemplateBase {}
#[repr(C, align(8))]
pub struct FMovieSceneEvalTemplate {
    pub completion_mode: EMovieSceneCompletionMode,
    pub source_section_ptr: TWeakObjectPtr<UMovieSceneSection>,
}
#[repr(C, align(8))]
pub struct FMovieSceneFloatChannel {
    pub pre_infinity_extrap: ERichCurveExtrapolation,
    pub post_infinity_extrap: ERichCurveExtrapolation,
    pub times: TArray<FFrameNumber>,
    pub values: TArray<FMovieSceneFloatValue>,
    pub default_value: f32,
    pub b_has_default_value: bool,
    pub key_handles: FMovieSceneKeyHandleMap,
    pub tick_resolution: FFrameRate,
    pub b_show_curve: bool,
}
#[repr(C, align(4))]
pub struct FMovieSceneFloatValue {
    pub value: f32,
    pub tangent: FMovieSceneTangentData,
    pub interp_mode: ERichCurveInterpMode,
    pub tangent_mode: ERichCurveTangentMode,
    pub padding_byte: u8,
}
#[repr(C, align(4))]
pub struct FMovieSceneTangentData {
    pub arrive_tangent: f32,
    pub leave_tangent: f32,
    pub arrive_tangent_weight: f32,
    pub leave_tangent_weight: f32,
    pub tangent_weight_mode: ERichCurveTangentWeightMode,
}
#[repr(C, align(4))]
pub struct FMovieSceneObjectBindingID {
    pub guid: FGuid,
    pub sequence_id: i32,
    pub resolve_parent_index: i32,
    pub space_deprecated: EMovieSceneObjectBindingSpace,
}
#[repr(C, align(8))]
pub struct FMovieScenePropertySectionData {
    pub property_name: FName,
    pub property_path: FString,
}
#[repr(C, align(8))]
pub struct FMovieSceneByteChannel {
    pub pre_infinity_extrap: ERichCurveExtrapolation,
    pub post_infinity_extrap: ERichCurveExtrapolation,
    pub times: TArray<FFrameNumber>,
    pub default_value: u8,
    pub b_has_default_value: bool,
    pub values: TArray<u8>,
    pub enum_: UPtr<UEnum>,
    pub key_handles: FMovieSceneKeyHandleMap,
}
#[repr(C, align(8))]
pub struct FMovieScenePropertySectionTemplate {
    pub property_data: FMovieScenePropertySectionData,
}
#[repr(C, align(8))]
pub struct FMovieSceneKeyStruct {}
#[repr(C, align(8))]
pub struct FMovieSceneTimeWarpVariant {
    pub variant: FMovieSceneNumericVariant,
}
#[repr(C, align(8))]
pub struct FMovieSceneNumericVariant {}
#[repr(C, align(4))]
pub struct FActorForWorldTransforms {
    pub actor: TWeakObjectPtr<AActor>,
    pub component: TWeakObjectPtr<USceneComponent>,
    pub socket_name: FName,
}
#[repr(C, align(8))]
pub struct FMovieSceneDeterminismData {
    pub fences: TArray<FMovieSceneDeterminismFence>,
    pub b_parent_sequence_requires_lower_fence: bool,
    pub b_parent_sequence_requires_upper_fence: bool,
}
#[repr(C, align(4))]
pub struct FMovieSceneDeterminismFence {
    pub frame_number: FFrameNumber,
    pub flags_4: u8,
}
#[repr(C, align(1))]
pub struct FMovieSceneEmptyStruct {}
#[repr(C, align(4))]
pub struct FMovieSceneEvaluationKey {
    pub sequence_id: FMovieSceneSequenceID,
    pub track_identifier: FMovieSceneTrackIdentifier,
    pub section_index: u32,
}
#[repr(C, align(4))]
pub struct FMovieSceneTrackIdentifier {
    pub value: u32,
}
#[repr(C, align(4))]
pub struct FMovieSceneSequenceID {
    pub value: u32,
}
#[repr(C, align(4))]
pub struct FMovieSceneEvaluationOperand {
    pub object_binding_id: FGuid,
    pub sequence_id: FMovieSceneSequenceID,
}
#[repr(C, align(16))]
pub struct FGeneratedMovieSceneKeyStruct {}
#[repr(C, align(8))]
pub struct FMovieSceneKeyTimeStruct {
    pub time: FFrameNumber,
}
#[repr(C, align(8))]
pub struct FMovieSceneSectionParameters {
    pub start_frame_offset: FFrameNumber,
    pub b_can_loop: bool,
    pub end_frame_offset: FFrameNumber,
    pub first_loop_start_frame_offset: FFrameNumber,
    pub time_scale: FMovieSceneTimeWarpVariant,
    pub hierarchical_bias: i32,
    pub flags: EMovieSceneSubSectionFlags,
    pub start_offset_deprecated: f32,
    pub preroll_time_deprecated: f32,
    pub postroll_time_deprecated: f32,
}
#[repr(C, align(4))]
pub struct FMovieSceneSegmentIdentifier {
    pub identifier_index: i32,
}
#[repr(C, align(4))]
pub struct FSectionEvaluationData {
    pub impl_index: i32,
    pub forced_time: FFrameNumber,
    pub flags: ESectionEvaluationFlags,
}
#[repr(C, align(8))]
pub struct FMovieSceneSegment {}
#[repr(C, align(4))]
pub struct FMovieSceneSequenceLoopCount {
    pub value: i32,
}
#[repr(C, align(4))]
pub struct FMovieSceneSequencePlaybackSettings {
    pub flags_0: u8,
    pub loop_count: FMovieSceneSequenceLoopCount,
    pub tick_interval: FMovieSceneSequenceTickInterval,
    pub play_rate: f32,
    pub start_time: f32,
    pub flags_28: u8,
    pub finish_completion_state_override: EMovieSceneCompletionModeOverride,
    pub flags_36: u8,
}
#[repr(C, align(4))]
pub struct FMovieSceneSequenceTickInterval {
    pub tick_interval_seconds: f32,
    pub evaluation_budget_microseconds: f32,
    pub b_tick_when_paused: bool,
    pub b_allow_rounding: bool,
}
#[repr(C, align(4))]
pub struct FMovieSceneTimeTransform {
    pub time_scale: f32,
    pub offset: FFrameTime,
}
#[repr(C, align(8))]
pub struct FMovieSceneBindingResolveResult {
    pub objects: TArray<UPtr<UObject>>,
    pub object: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FMovieSceneBindingResolveContext {
    pub world_context: UPtr<UObject>,
    pub binding: FMovieSceneBindingProxy,
}
#[repr(C, align(8))]
pub struct FMovieSceneAudioTriggerChannel {
    pub times: TArray<FFrameNumber>,
    pub values: TArray<bool>,
    pub key_handles: FMovieSceneKeyHandleMap,
}
#[repr(C, align(8))]
pub struct FMovieSceneDoubleValue {
    pub value: f64,
    pub tangent: FMovieSceneTangentData,
    pub interp_mode: ERichCurveInterpMode,
    pub tangent_mode: ERichCurveTangentMode,
    pub padding_byte: u8,
}
#[repr(C, align(8))]
pub struct FMovieSceneDoubleChannel {
    pub pre_infinity_extrap: ERichCurveExtrapolation,
    pub post_infinity_extrap: ERichCurveExtrapolation,
    pub times: TArray<FFrameNumber>,
    pub values: TArray<FMovieSceneDoubleValue>,
    pub default_value: f64,
    pub b_has_default_value: bool,
    pub key_handles: FMovieSceneKeyHandleMap,
    pub tick_resolution: FFrameRate,
    pub b_show_curve: bool,
}
#[repr(C, align(8))]
pub struct FMovieSceneIntegerChannel {
    pub pre_infinity_extrap: ERichCurveExtrapolation,
    pub post_infinity_extrap: ERichCurveExtrapolation,
    pub b_interpolate_linear_keys: bool,
    pub times: TArray<FFrameNumber>,
    pub default_value: i32,
    pub b_has_default_value: bool,
    pub values: TArray<i32>,
    pub key_handles: FMovieSceneKeyHandleMap,
}
#[repr(C, align(8))]
pub struct FMovieSceneObjectPathChannelKeyValue {
    pub soft_ptr: TSoftObjectPtr<UObject>,
    pub hard_ptr: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FMovieSceneObjectPathChannel {
    pub property_class: TSubclassOf<UObject>,
    pub times: TArray<FFrameNumber>,
    pub values: TArray<FMovieSceneObjectPathChannelKeyValue>,
    pub default_value: FMovieSceneObjectPathChannelKeyValue,
    pub key_handles: FMovieSceneKeyHandleMap,
}
#[repr(C, align(8))]
pub struct FMovieSceneTimeWarpChannel {
    pub owner: UPtr<UMovieScene>,
}
#[repr(C, align(1))]
pub struct FMovieSceneCompiledSequenceFlagStruct {
    pub flags_0: u8,
}
#[repr(C, align(8))]
pub struct FMovieSceneConditionContext {
    pub world_context: UPtr<UObject>,
    pub binding: FMovieSceneBindingProxy,
    pub bound_objects: TArray<UPtr<UObject>>,
}
#[repr(C, align(8))]
pub struct FMovieSceneConditionContainer {
    pub condition: UPtr<UMovieSceneCondition>,
}
#[repr(C, align(8))]
pub struct FMovieSceneDecorationContainer {
    pub decorations: TArray<UPtr<UObject>>,
}
#[repr(C, align(4))]
pub struct FMovieSceneScalingAnchor {
    pub position: FFrameNumber,
    pub duration: i32,
}
#[repr(C, align(8))]
pub struct FMovieSceneAnchorsScalingGroup {
    pub sections: TSet<UPtr<UMovieSceneSection>>,
}
#[repr(C, align(8))]
pub struct FNavigationToolSerializedItem {
    pub id: FString,
}
#[repr(C, align(8))]
pub struct FNavigationToolSerializedTreeNode {
    pub local_index: i32,
    pub global_index: i32,
    pub parent_index: i32,
    pub children_indices: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FNavigationToolSerializedTree {
    pub root_node: FNavigationToolSerializedTreeNode,
    pub scene_items: TArray<FNavigationToolSerializedItem>,
    pub item_tree_map: TMap<
        FNavigationToolSerializedItem,
        FNavigationToolSerializedTreeNode,
    >,
}
#[repr(C, align(4))]
pub struct FNavigationToolViewColumnSaveState {
    pub b_visible: bool,
    pub size: f32,
}
#[repr(C, align(8))]
pub struct FNavigationToolViewSaveState {
    pub view_item_flags: TMap<FString, ENavigationToolItemFlags>,
    pub columns_state: TMap<FName, FNavigationToolViewColumnSaveState>,
    pub active_item_filters: TSet<FName>,
}
#[repr(C, align(8))]
pub struct FNavigationToolSaveState {
    pub serialized_tree: FNavigationToolSerializedTree,
    pub item_color_map: TMap<FString, FColor>,
    pub tool_view_save_states: TArray<FNavigationToolViewSaveState>,
    pub context_path: FString,
}
#[repr(C, align(8))]
pub struct FEasingComponentData {
    pub section: UPtr<UMovieSceneSection>,
}
#[repr(C, align(8))]
pub struct FMovieSceneTrackInstanceComponent {
    pub owner: UPtr<UMovieSceneSection>,
    pub track_instance_class: TSubclassOf<UMovieSceneTrackInstance>,
}
#[repr(C, align(8))]
pub struct FMovieSceneEvaluationHookComponent {
    pub interface: TScriptInterface<IMovieSceneEvaluationHook>,
}
#[repr(C, align(8))]
pub struct FTrackInstanceInputComponent {
    pub section: UPtr<UMovieSceneSection>,
    pub output_index: i32,
}
#[repr(C, align(8))]
pub struct FMovieSceneEntitySystemGraphNode {
    pub system: UPtr<UMovieSceneEntitySystem>,
}
#[repr(C, align(8))]
pub struct FMovieSceneEntitySystemGraphNodes {}
#[repr(C, align(8))]
pub struct FMovieSceneEntitySystemGraph {
    pub nodes: FMovieSceneEntitySystemGraphNodes,
}
#[repr(C, align(8))]
pub struct FMovieSceneEvaluationHookEvent {
    pub hook: FMovieSceneEvaluationHookComponent,
}
#[repr(C, align(8))]
pub struct FMovieSceneEvaluationHookEventContainer {
    pub events: TArray<FMovieSceneEvaluationHookEvent>,
}
#[repr(C, align(2))]
pub struct FMovieSceneEvaluationInstanceKey {}
#[repr(C, align(8))]
pub struct FMovieSceneTrackInstanceInput {
    pub section: UPtr<UMovieSceneSection>,
}
#[repr(C, align(8))]
pub struct FMovieSceneTrackInstanceEntry {
    pub bound_object: UPtr<UObject>,
    pub track_instance: UPtr<UMovieSceneTrackInstance>,
}
#[repr(C, align(1))]
pub struct FOptionalMovieSceneBlendType {
    pub blend_type: EMovieSceneBlendType,
    pub b_is_valid: bool,
}
#[repr(C, align(8))]
pub struct FMovieSceneEvalTemplatePtr {}
#[repr(C, align(4))]
pub struct FMovieSceneEvaluationFieldEntityKey {
    pub entity_owner: TWeakObjectPtr<UObject>,
    pub entity_id: u32,
}
#[repr(C, align(4))]
pub struct FMovieSceneEvaluationFieldEntity {
    pub key: FMovieSceneEvaluationFieldEntityKey,
    pub shared_meta_data_index: i32,
}
#[repr(C, align(4))]
pub struct FMovieSceneEvaluationFieldSharedEntityMetaData {
    pub object_binding_id: FGuid,
}
#[repr(C, align(8))]
pub struct FMovieSceneEvaluationFieldEntityMetaData {
    pub condition: TSoftObjectPtr<UMovieSceneCondition>,
    pub override_bound_property_path: FString,
    pub notify_function_name: FName,
    pub forced_time: FFrameNumber,
    pub flags: ESectionEvaluationFlags,
    pub flags_81: u8,
}
#[repr(C, align(8))]
pub struct FMovieSceneEvaluationFieldEntityTree {}
#[repr(C, align(8))]
pub struct FMovieSceneEntityComponentField {
    pub persistent_entity_tree: FMovieSceneEvaluationFieldEntityTree,
    pub one_shot_entity_tree: FMovieSceneEvaluationFieldEntityTree,
    pub entities: TArray<FMovieSceneEvaluationFieldEntity>,
    pub entity_meta_data: TArray<FMovieSceneEvaluationFieldEntityMetaData>,
    pub shared_meta_data: TArray<FMovieSceneEvaluationFieldSharedEntityMetaData>,
}
#[repr(C, align(4))]
pub struct FMovieSceneEvaluationFieldTrackPtr {
    pub sequence_id: FMovieSceneSequenceID,
    pub track_identifier: FMovieSceneTrackIdentifier,
}
#[repr(C, align(4))]
pub struct FMovieSceneEvaluationFieldSegmentPtr {
    pub segment_id: FMovieSceneSegmentIdentifier,
}
#[repr(C, align(4))]
pub struct FMovieSceneFieldEntry_EvaluationTrack {
    pub track_ptr: FMovieSceneEvaluationFieldTrackPtr,
    pub num_children: u16,
}
#[repr(C, align(4))]
pub struct FMovieSceneFieldEntry_ChildTemplate {
    pub child_index: u16,
    pub flags: ESectionEvaluationFlags,
    pub forced_time: FFrameNumber,
}
#[repr(C, align(4))]
pub struct FMovieSceneEvaluationGroupLUTIndex {
    pub num_init_ptrs: i32,
    pub num_eval_ptrs: i32,
}
#[repr(C, align(8))]
pub struct FMovieSceneEvaluationGroup {
    pub lut_indices: TArray<FMovieSceneEvaluationGroupLUTIndex>,
    pub track_lut: TArray<FMovieSceneFieldEntry_EvaluationTrack>,
    pub section_lut: TArray<FMovieSceneFieldEntry_ChildTemplate>,
}
#[repr(C, align(4))]
pub struct FMovieSceneOrderedEvaluationKey {
    pub key: FMovieSceneEvaluationKey,
    pub setup_index: u16,
    pub tear_down_index: u16,
}
#[repr(C, align(8))]
pub struct FMovieSceneEvaluationMetaData {
    pub active_sequences: TArray<FMovieSceneSequenceID>,
    pub active_entities: TArray<FMovieSceneOrderedEvaluationKey>,
}
#[repr(C, align(8))]
pub struct FMovieSceneEvaluationField {
    pub signature: FGuid,
    pub ranges: TArray<FMovieSceneFrameRange>,
    pub groups: TArray<FMovieSceneEvaluationGroup>,
    pub meta_data: TArray<FMovieSceneEvaluationMetaData>,
}
#[repr(C, align(4))]
pub struct FMovieSceneFrameRange {}
#[repr(C, align(8))]
pub struct FMovieSceneTemplateGenerationLedger {
    pub last_track_identifier: FMovieSceneTrackIdentifier,
    pub track_signature_to_track_identifier: TMap<FGuid, FMovieSceneTrackIdentifier>,
    pub sub_section_ranges: TMap<FGuid, FMovieSceneFrameRange>,
}
#[repr(C, align(4))]
pub struct FMovieSceneSubSectionData {
    pub section: TWeakObjectPtr<UMovieSceneSubSection>,
    pub object_binding_id: FGuid,
    pub flags: ESectionEvaluationFlags,
}
#[repr(C, align(4))]
pub struct FMovieSceneEvaluationTemplateSerialNumber {
    pub value: u32,
}
#[repr(C, align(8))]
pub struct FMovieSceneEvaluationTemplate {
    pub tracks: TMap<FMovieSceneTrackIdentifier, FMovieSceneEvaluationTrack>,
    pub sequence_signature: FGuid,
    pub template_serial_number: FMovieSceneEvaluationTemplateSerialNumber,
    pub template_ledger: FMovieSceneTemplateGenerationLedger,
}
#[repr(C, align(8))]
pub struct FMovieSceneEvaluationTrack {
    pub object_binding_id: FGuid,
    pub evaluation_priority: u16,
    pub evaluation_method: EEvaluationMethod,
    pub source_track: TWeakObjectPtr<UMovieSceneTrack>,
    pub child_templates: TArray<FMovieSceneEvalTemplatePtr>,
    pub track_template: FMovieSceneTrackImplementationPtr,
    pub evaluation_group: FName,
    pub flags_116: u8,
}
#[repr(C, align(8))]
pub struct FMovieSceneTrackImplementationPtr {}
#[repr(C, align(8))]
pub struct FMovieSceneRootEvaluationTemplateInstance {
    pub entity_system_linker: UPtr<UMovieSceneEntitySystemLinker>,
}
#[repr(C, align(8))]
pub struct FMovieSceneSubSequenceData {
    pub sequence: FSoftObjectPath,
    pub outer_to_inner_transform: FMovieSceneSequenceTransform,
    pub root_to_sequence_transform: FMovieSceneSequenceTransform,
    pub local_to_warped_local_transform: FMovieSceneSequenceTransform,
    pub root_to_unwarped_local_transform: FMovieSceneSequenceTransform,
    pub start_time_breadcrumbs: FMovieSceneTransformBreadcrumbs,
    pub end_time_breadcrumbs: FMovieSceneTransformBreadcrumbs,
    pub tick_resolution: FFrameRate,
    pub deterministic_sequence_id: FMovieSceneSequenceID,
    pub play_range: FMovieSceneFrameRange,
    pub parent_play_range: FMovieSceneFrameRange,
    pub pre_roll_range: FMovieSceneFrameRange,
    pub post_roll_range: FMovieSceneFrameRange,
    pub hierarchical_bias: i16,
    pub accumulated_flags: EMovieSceneSubSectionFlags,
    pub b_can_loop: bool,
    pub instance_data: FMovieSceneSequenceInstanceDataPtr,
    pub section_path: FName,
    pub weak_condition: TSoftObjectPtr<UMovieSceneCondition>,
    pub sub_section_signature: FGuid,
}
#[repr(C, align(8))]
pub struct FMovieSceneSequenceInstanceDataPtr {}
#[repr(C, align(8))]
pub struct FMovieSceneTransformBreadcrumbs {
    pub breadcrumbs: TArray<FFrameTime>,
    pub mode: EMovieSceneBreadcrumbMode,
}
#[repr(C, align(8))]
pub struct FMovieSceneSequenceTransform {
    pub linear_transform: FMovieSceneTimeTransform,
    pub nested_transforms: TArray<FMovieSceneNestedSequenceTransform>,
}
#[repr(C, align(8))]
pub struct FMovieSceneNestedSequenceTransform {
    pub time_scale: FMovieSceneTimeWarpVariant,
    pub offset: FFrameTime,
}
#[repr(C, align(8))]
pub struct FMovieSceneSequenceHierarchyNode {
    pub parent_id: FMovieSceneSequenceID,
    pub children: TArray<FMovieSceneSequenceID>,
}
#[repr(C, align(8))]
pub struct FMovieSceneSubSequenceTreeEntry {}
#[repr(C, align(8))]
pub struct FMovieSceneSubSequenceTree {}
#[repr(C, align(8))]
pub struct FMovieSceneSequenceHierarchy {
    pub root_node: FMovieSceneSequenceHierarchyNode,
    pub tree: FMovieSceneSubSequenceTree,
    pub root_transform: FMovieSceneSequenceTransform,
    pub sub_sequences: TMap<FMovieSceneSequenceID, FMovieSceneSubSequenceData>,
    pub hierarchy: TMap<FMovieSceneSequenceID, FMovieSceneSequenceHierarchyNode>,
    pub accumulated_network_mask: EMovieSceneServerClientMask,
}
#[repr(C, align(8))]
pub struct FMovieSceneWarpCounter {}
#[repr(C, align(8))]
pub struct FMovieSceneInverseNestedSequenceTransform {
    pub time_scale: FMovieSceneTimeWarpVariant,
    pub offset: FFrameTime,
}
#[repr(C, align(8))]
pub struct FMovieSceneInverseSequenceTransform {
    pub linear_transform: FMovieSceneTimeTransform,
    pub nested_transforms: TArray<FMovieSceneInverseNestedSequenceTransform>,
}
#[repr(C, align(4))]
pub struct FMovieSceneTimeWarping {
    pub start: FFrameNumber,
    pub end: FFrameNumber,
}
#[repr(C, align(8))]
pub struct FMovieSceneTrackImplementation {}
#[repr(C, align(1))]
pub struct FMovieSceneExpansionState {
    pub b_expanded: bool,
}
#[repr(C, align(8))]
pub struct FMovieSceneEditorData {
    pub expansion_states: TMap<FString, FMovieSceneExpansionState>,
    pub pinned_nodes: TArray<FString>,
    pub view_start: f64,
    pub view_end: f64,
    pub work_start: f64,
    pub work_end: f64,
    pub marked_frames_deprecated: TSet<FFrameNumber>,
    pub working_range_deprecated: FFloatRange,
    pub view_range_deprecated: FFloatRange,
    pub navigation_tool_state: FNavigationToolSaveState,
    pub solo_nodes: TArray<FString>,
    pub mute_nodes: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FMovieSceneTrackLabels {
    pub strings: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FMovieSceneObjectBindingIDs {
    pub i_ds: TArray<FMovieSceneObjectBindingID>,
}
#[repr(C, align(8))]
pub struct FMovieSceneSectionGroup {
    pub sections: TArray<TWeakObjectPtr<UMovieSceneSection>>,
}
#[repr(C, align(8))]
pub struct FMovieSceneMarkedFrame {
    pub frame_number: FFrameNumber,
    pub label: FString,
    pub comment: FString,
    pub custom_color: FLinearColor,
    pub b_use_custom_color: bool,
    pub color_deprecated: FLinearColor,
    pub b_is_determinism_fence: bool,
    pub b_is_inclusive_time: bool,
}
#[repr(C, align(8))]
pub struct FMovieSceneBinding {
    pub object_guid: FGuid,
    pub binding_name_deprecated: FString,
    pub tracks: TArray<UPtr<UMovieSceneTrack>>,
    pub sorting_order: i32,
}
#[repr(C, align(8))]
pub struct FMovieSceneBindingOverrideData {
    pub object_binding_id: FMovieSceneObjectBindingID,
    pub object: TSoftObjectPtr<UObject>,
    pub b_overrides_default: bool,
}
#[repr(C, align(8))]
pub struct FMovieSceneBindingReference {
    pub id: FGuid,
    pub locator: FUniversalObjectLocator,
    pub resolve_flags: ELocatorResolveFlags,
    pub custom_binding: UPtr<UMovieSceneCustomBinding>,
}
#[repr(C, align(8))]
pub struct FMovieSceneBindingResolveParams {
    pub sequence: UPtr<UMovieSceneSequence>,
    pub object_binding_id: FGuid,
    pub sequence_id: FMovieSceneSequenceID,
    pub context: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FMovieSceneBindingReferences {
    pub sorted_references: TArray<FMovieSceneBindingReference>,
}
#[repr(C, align(8))]
pub struct FMovieSceneDynamicBindingResolveParams {
    pub sequence: UPtr<UMovieSceneSequence>,
    pub object_binding_id: FGuid,
    pub root_sequence: UPtr<UMovieSceneSequence>,
}
#[repr(C, align(8))]
pub struct FMovieSceneDynamicBindingResolveResult {
    pub objects: TArray<UPtr<UObject>>,
    pub object: UPtr<UObject>,
    pub b_is_possessed_object: bool,
}
#[repr(C, align(8))]
pub struct FMovieSceneDynamicBindingContainer {
    pub dynamic_binding: FMovieSceneDynamicBinding,
}
#[repr(C, align(8))]
pub struct FMovieScenePossessable {
    pub tags: TArray<FName>,
    pub dynamic_binding_deprecated: FMovieSceneDynamicBinding,
    pub guid: FGuid,
    pub name: FString,
    pub possessed_object_class: TSoftObjectPtr<UClass>,
    pub parent_guid: FGuid,
    pub spawnable_object_binding_id: FMovieSceneObjectBindingID,
}
#[repr(C, align(4))]
pub struct FMovieScenePropertyBinding {
    pub property_name: FName,
    pub property_path: FName,
    pub b_can_use_class_lookup: bool,
}
#[repr(C, align(1))]
pub struct FMovieSceneSectionEvalOptions {
    pub b_can_edit_completion_mode: bool,
    pub completion_mode: EMovieSceneCompletionMode,
}
#[repr(C, align(8))]
pub struct FMovieSceneEasingSettings {
    pub auto_ease_in_duration: i32,
    pub auto_ease_out_duration: i32,
    pub ease_in: TScriptInterface<IMovieSceneEasingFunction>,
    pub b_manual_ease_in: bool,
    pub manual_ease_in_duration: i32,
    pub ease_out: TScriptInterface<IMovieSceneEasingFunction>,
    pub b_manual_ease_out: bool,
    pub manual_ease_out_duration: i32,
    pub auto_ease_in_time_deprecated: f32,
    pub auto_ease_out_time_deprecated: f32,
    pub manual_ease_in_time_deprecated: f32,
    pub manual_ease_out_time_deprecated: f32,
}
#[repr(C, align(4))]
pub struct FMovieSceneTimecodeSource {
    pub timecode: FTimecode,
}
#[repr(C, align(8))]
pub struct FMovieSceneSequenceInstanceData {}
#[repr(C, align(4))]
pub struct FMovieSceneSequenceReplProperties {
    pub last_known_position: FFrameTime,
    pub last_known_status: EMovieScenePlayerStatus,
    pub last_known_num_loops: i32,
    pub last_known_serial_number: i32,
}
#[repr(C, align(8))]
pub struct FMovieSceneSequencePlaybackParams {
    pub frame: FFrameTime,
    pub time: f32,
    pub marked_frame: FString,
    pub timecode: FTimecode,
    pub position_type: EMovieScenePositionType,
    pub update_method: EUpdatePositionMethod,
    pub b_has_jumped: bool,
}
#[repr(C, align(1))]
pub struct FMovieSceneSequencePlayToParams {
    pub b_exclusive: bool,
}
#[repr(C, align(16))]
pub struct FMovieSceneSpawnable {
    pub spawn_transform: FTransform,
    pub tags: TArray<FName>,
    pub b_continuously_respawn: bool,
    pub b_net_addressable_name: bool,
    pub dynamic_binding_deprecated: FMovieSceneDynamicBinding,
    pub guid: FGuid,
    pub name: FString,
    pub object_template: UPtr<UObject>,
    pub child_possessables: TArray<FGuid>,
    pub ownership: ESpawnOwnership,
    pub generated_class_deprecated: TSubclassOf<UObject>,
    pub level_name: FName,
}
#[repr(C, align(4))]
pub struct FMovieSceneTrackEvalOptions {
    pub flags_0: u8,
}
#[repr(C, align(4))]
pub struct FMovieSceneTrackDisplayOptions {
    pub flags_0: u8,
}
#[repr(C, align(8))]
pub struct FMovieSceneTrackRowMetadata {
    pub condition_container: FMovieSceneConditionContainer,
}
#[repr(C, align(8))]
pub struct FMovieSceneTrackEvaluationFieldEntry {
    pub section: UPtr<UMovieSceneSection>,
    pub range: FFrameNumberRange,
    pub forced_time: FFrameNumber,
    pub flags: ESectionEvaluationFlags,
    pub legacy_sort_order: i16,
}
#[repr(C, align(8))]
pub struct FMovieSceneTrackEvaluationField {
    pub entries: TArray<FMovieSceneTrackEvaluationFieldEntry>,
}
#[repr(C, align(8))]
pub struct FMovieSceneSectionTimingParametersSeconds {
    pub play_rate: FMovieSceneTimeWarpVariant,
    pub inner_start_offset: f32,
    pub inner_end_offset: f32,
    pub first_loop_start_offset: f32,
    pub flags_28: u8,
}
#[repr(C, align(8))]
pub struct FMovieSceneSectionTimingParametersFrames {
    pub play_rate: FMovieSceneTimeWarpVariant,
    pub inner_start_offset: FFrameNumber,
    pub inner_end_offset: FFrameNumber,
    pub first_loop_start_offset: FFrameNumber,
    pub flags_28: u8,
}
#[repr(C, align(4))]
pub struct FMovieSceneSubSectionOriginOverrideMask {
    pub mask: u32,
}
#[repr(C, align(8))]
pub struct FTestMovieSceneEvalTemplate {}
#[repr(C, align(8))]
pub struct FTestMovieSceneStruct {
    pub first: f32,
    pub second: f32,
    pub enum_: ETestMovieSceneEnum,
    pub vector: FVector,
    pub multiple_integers: TArray<i32>,
    pub multiple_vectors: TArray<FVector>,
}
#[repr(C, align(4))]
pub struct FTestMovieSceneStruct2 {
    pub first: f32,
    pub second: f32,
}
#[repr(C, align(4))]
pub struct FMovieSceneTimeWarpFixedFrame {
    pub frame_number: FFrameNumber,
}
#[repr(C, align(8))]
pub struct FMovieSceneFixedPlayRateStruct {
    pub play_rate: f64,
}
#[repr(C, align(8))]
pub struct FMovieSceneCustomTimeWarpGetterStruct {
    pub object: UPtr<UMovieSceneTimeWarpGetter>,
}
#[repr(C, align(4))]
pub struct FMovieSceneTimeWarpLoop {
    pub duration: FFrameNumber,
}
#[repr(C, align(4))]
pub struct FMovieSceneTimeWarpClamp {
    pub max: FFrameNumber,
}
#[repr(C, align(4))]
pub struct FMovieSceneTimeWarpLoopFloat {
    pub duration: f32,
}
#[repr(C, align(4))]
pub struct FMovieSceneTimeWarpClampFloat {
    pub max: f32,
}
#[repr(C, align(1))]
pub struct FMovieSceneTimeWarpFrameRate {
    pub frame_rate_numerator: u8,
    pub frame_rate_denominator: u8,
}
pub struct UMovieSceneEntitySystem {
    pub linker: UPtr<UMovieSceneEntitySystemLinker>,
}
pub struct UMovieSceneSignedObject {
    pub signature: FGuid,
}
pub struct UMovieSceneDecorationContainerObject {
    pub decorations: FMovieSceneDecorationContainer,
}
pub struct UMovieSceneSection {
    pub eval_options: FMovieSceneSectionEvalOptions,
    pub easing: FMovieSceneEasingSettings,
    pub section_range: FMovieSceneFrameRange,
    pub timecode_source: FMovieSceneTimecodeSource,
    pub condition_container: FMovieSceneConditionContainer,
    pub pre_roll_frames: FFrameNumber,
    pub post_roll_frames: FFrameNumber,
    pub row_index: i32,
    pub overlap_priority: i32,
    pub flags_304: u8,
    pub color_tint: FColor,
    pub start_time_deprecated: f32,
    pub end_time_deprecated: f32,
    pub pre_roll_time_deprecated: f32,
    pub post_roll_time_deprecated: f32,
    pub flags_328: u8,
    pub b_supports_infinite_range: bool,
    pub blend_type: FOptionalMovieSceneBlendType,
}
pub struct UMovieSceneTrack {
    pub eval_options: FMovieSceneTrackEvalOptions,
    pub display_options: FMovieSceneTrackDisplayOptions,
    pub condition_container: FMovieSceneConditionContainer,
    pub b_is_eval_disabled: bool,
    pub rows_disabled: TArray<i32>,
    pub b_is_local_eval_disabled: bool,
    pub local_rows_disabled: TArray<i32>,
    pub evaluation_field_guid: FGuid,
    pub evaluation_field_version: i8,
    pub evaluation_field: FMovieSceneTrackEvaluationField,
    pub track_row_metadata: TMap<i32, FMovieSceneTrackRowMetadata>,
    pub object_binding_id: FGuid,
    pub track_tint: FColor,
    pub sorting_order: i32,
    pub b_supports_default_sections: bool,
    pub b_supports_conditions: bool,
}
pub struct UMovieSceneNameableTrack {
    pub display_name: FText,
    pub track_row_display_names: TArray<FText>,
}
pub struct UMovieSceneSequence {
    pub compiled_data: UPtr<UMovieSceneCompiledData>,
    pub default_completion_mode: EMovieSceneCompletionMode,
    pub b_parent_contexts_are_significant: bool,
    pub b_playable_directly: bool,
    pub sequence_flags: EMovieSceneSequenceFlags,
}
pub struct UMovieSceneCustomBinding {}
pub struct UMovieSceneReplaceableBindingBase {
    pub preview_spawnable: UPtr<UMovieSceneSpawnableBindingBase>,
}
pub struct UMovieSceneSpawnableBindingBase {
    pub spawn_ownership: ESpawnOwnership,
    pub b_continuously_respawn: bool,
}
pub struct UMovieSceneChannelOverrideContainer {}
pub struct UMovieSceneCondition {
    pub b_editor_force_true: bool,
    pub b_invert: bool,
}
pub struct UMovieSceneEntityInstantiatorSystem {}
pub struct UMovieSceneSubSection {
    pub parameters: FMovieSceneSectionParameters,
    pub start_offset_deprecated: f32,
    pub time_scale_deprecated: f32,
    pub preroll_time_deprecated: f32,
    pub network_mask: u8,
    pub origin_override_mask: FMovieSceneSubSectionOriginOverrideMask,
    pub translation: FMovieSceneDoubleChannel,
    pub rotation: FMovieSceneDoubleChannel,
    pub key_preview_position: TOptional<FVector>,
    pub key_preview_rotation: TOptional<FRotator>,
    pub sub_sequence: UPtr<UMovieSceneSequence>,
}
pub struct UMovieSceneBoolSection {
    pub default_value_deprecated: bool,
    pub bool_curve: FMovieSceneBoolChannel,
    pub b_is_externally_inverted: bool,
}
pub struct UMovieSceneBlenderSystem {}
pub struct UMovieSceneTrackInstance {
    pub weak_animated_object: TWeakObjectPtr<UObject>,
    pub b_is_root_track_instance: bool,
    pub private_linker: UPtr<UMovieSceneEntitySystemLinker>,
    pub inputs: TArray<FMovieSceneTrackInstanceInput>,
}
pub struct UMovieSceneSubTrack {
    pub sections: TArray<UPtr<UMovieSceneSection>>,
    pub row_height: i32,
    pub section_to_key: UPtr<UMovieSceneSection>,
}
pub struct UMovieSceneBlenderSystemSupport {}
pub struct IMovieSceneBlenderSystemSupport {}
pub struct UMovieSceneBoundObjectProxy {}
pub struct IMovieSceneBoundObjectProxy {}
pub struct UMovieSceneChannelDecoration {}
pub struct IMovieSceneChannelDecoration {}
pub struct UMovieSceneChannelOverrideProvider {}
pub struct IMovieSceneChannelOverrideProvider {}
pub struct UMovieSceneChannelOwner {}
pub struct IMovieSceneChannelOwner {}
pub struct UMovieSceneDecoration {}
pub struct IMovieSceneDecoration {}
pub struct UMovieSceneDeterminismSource {}
pub struct IMovieSceneDeterminismSource {}
pub struct UMovieSceneEntityDecorator {}
pub struct IMovieSceneEntityDecorator {}
pub struct UMovieSceneLifetimeDecoration {}
pub struct IMovieSceneLifetimeDecoration {}
pub struct UMovieSceneMetaDataInterface {}
pub struct IMovieSceneMetaDataInterface {}
pub struct UMovieScenePlaybackClient {}
pub struct IMovieScenePlaybackClient {}
pub struct UMovieSceneSectionDecoration {}
pub struct IMovieSceneSectionDecoration {}
pub struct UMovieSceneSequencePlayerObserver {}
pub struct IMovieSceneSequencePlayerObserver {}
pub struct UMovieSceneTrackDecoration {}
pub struct IMovieSceneTrackDecoration {}
pub struct UMovieSceneBindingEventReceiverInterface {}
pub struct IMovieSceneBindingEventReceiverInterface {}
pub struct UMovieSceneBindingOwnerInterface {}
pub struct IMovieSceneBindingOwnerInterface {}
pub struct UMovieSceneCachedTrack {}
pub struct IMovieSceneCachedTrack {}
pub struct UMovieSceneEasingFunction {}
pub struct IMovieSceneEasingFunction {}
pub struct UMovieSceneKeyProxy {}
pub struct IMovieSceneKeyProxy {}
pub struct UMovieSceneNumericVariantGetter {
    pub reference_to_self: UPtr<UMovieSceneNumericVariantGetter>,
}
pub struct UMovieSceneSequenceTickManagerClient {}
pub struct IMovieSceneSequenceTickManagerClient {}
pub struct UMovieSceneSectionChannelOverrideRegistry {
    pub overrides: TMap<FName, UPtr<UMovieSceneChannelOverrideContainer>>,
}
pub struct UMovieSceneTrackTemplateProducer {}
pub struct IMovieSceneTrackTemplateProducer {}
pub struct UMovieSceneCompiledData {
    pub evaluation_template: FMovieSceneEvaluationTemplate,
    pub hierarchy: FMovieSceneSequenceHierarchy,
    pub entity_component_field: FMovieSceneEntityComponentField,
    pub track_template_field: FMovieSceneEvaluationField,
    pub determinism_fences: TArray<FMovieSceneDeterminismFence>,
    pub compiled_signature: FGuid,
    pub compiler_version: FGuid,
    pub accumulated_mask: EMovieSceneSequenceCompilerMask,
    pub allocated_mask: EMovieSceneSequenceCompilerMask,
    pub accumulated_flags: EMovieSceneSequenceFlags,
}
pub struct UMovieSceneCompiledDataManager {
    pub hierarchies: TMap<i32, FMovieSceneSequenceHierarchy>,
    pub track_templates: TMap<i32, FMovieSceneEvaluationTemplate>,
    pub track_template_fields: TMap<i32, FMovieSceneEvaluationField>,
    pub entity_component_fields: TMap<i32, FMovieSceneEntityComponentField>,
}
pub struct UMovieSceneGroupCondition {
    pub operator: EMovieSceneGroupConditionOperator,
    pub sub_conditions: TArray<FMovieSceneConditionContainer>,
}
pub struct UMovieSceneLanguagePreviewDecoration {
    pub preview_language: FString,
}
pub struct UMovieSceneMuteSoloDecoration {}
pub struct UMovieSceneScalingDriver {}
pub struct IMovieSceneScalingDriver {}
pub struct UMovieSceneTimeWarpGetter {
    pub flags_128: u8,
}
pub struct UMovieScenePlayRateCurve {
    pub play_rate: FMovieSceneTimeWarpChannel,
    pub playback_start_frame: FFrameNumber,
    pub b_manual_playback_start: bool,
}
pub struct UMovieSceneScalingAnchors {
    pub scaling_drivers: TArray<TScriptInterface<IMovieSceneScalingDriver>>,
    pub initial_anchors: TMap<FGuid, FMovieSceneScalingAnchor>,
    pub scaling_groups: TMap<FGuid, FMovieSceneAnchorsScalingGroup>,
}
pub struct UMovieSceneSectionAnchorsDecoration {
    pub start_anchor: FGuid,
}
pub struct UMovieSceneTimeWarpSource {}
pub struct IMovieSceneTimeWarpSource {}
pub struct UMovieSceneTimeWarpDecoration {
    pub sources: TArray<TScriptInterface<IMovieSceneTimeWarpSource>>,
}
pub struct UMovieSceneTrackRowDecoration {}
pub struct UMovieSceneEntityProvider {}
pub struct IMovieSceneEntityProvider {}
pub struct UMovieSceneBindingLifetimeSystem {}
pub struct UMovieSceneGenericBoundObjectInstantiator {}
pub struct UMovieSceneBoundSceneComponentInstantiator {}
pub struct UMovieSceneValueDecomposer {}
pub struct IMovieSceneValueDecomposer {}
pub struct UMovieSceneEntityGroupingSystem {}
pub struct UMovieSceneEntitySystemLinker {
    pub system_graph: FMovieSceneEntitySystemGraph,
}
pub struct UMovieSceneEvalTimeSystem {}
pub struct UMovieSceneEvaluationHookSystem {
    pub pending_events_by_root_instance: TMap<
        FMovieSceneEvaluationInstanceKey,
        FMovieSceneEvaluationHookEventContainer,
    >,
}
pub struct UMovieSceneInitialValueSystem {}
pub struct UMovieScenePreAnimatedStateSystemInterface {}
pub struct IMovieScenePreAnimatedStateSystemInterface {}
pub struct UMovieSceneCachePreAnimatedStateSystem {}
pub struct UMovieSceneRestorePreAnimatedStateSystem {}
pub struct UMovieSceneRootInstantiatorSystem {}
pub struct UMovieSceneSpawnablesSystem {}
pub struct UMovieSceneTrackInstanceInstantiator {}
pub struct UMovieSceneTrackInstanceSystem {
    pub instantiator: UPtr<UMovieSceneTrackInstanceInstantiator>,
}
pub struct UMovieSceneCustomClockSource {}
pub struct IMovieSceneCustomClockSource {}
pub struct UMovieSceneEvaluationHook {}
pub struct IMovieSceneEvaluationHook {}
pub struct UMovieSceneBuiltInEasingFunction {
    pub ty: EMovieSceneBuiltInEasing,
}
pub struct UMovieSceneEasingExternalCurve {
    pub curve: UPtr<UCurveFloat>,
}
pub struct UNodeAndChannelMappings {}
pub struct INodeAndChannelMappings {}
pub struct UMovieSceneShotMetaData {
    pub b_is_no_good: TOptional<bool>,
    pub b_is_flagged: TOptional<bool>,
    pub b_is_recorded: TOptional<bool>,
    pub b_is_sub_sequence: TOptional<bool>,
    pub favorite_rating: TOptional<i32>,
}
pub struct UMovieSceneNodeGroup {
    pub name: FName,
    pub nodes: TArray<FString>,
}
pub struct UMovieSceneNodeGroupCollection {
    pub node_groups: TArray<UPtr<UMovieSceneNodeGroup>>,
}
pub struct UMovieScene {
    pub spawnables: TArray<FMovieSceneSpawnable>,
    pub possessables: TArray<FMovieScenePossessable>,
    pub object_bindings: TArray<FMovieSceneBinding>,
    pub binding_groups: TMap<FName, FMovieSceneObjectBindingIDs>,
    pub tracks: TArray<UPtr<UMovieSceneTrack>>,
    pub camera_cut_track: UPtr<UMovieSceneTrack>,
    pub custom_clock: UPtr<UMovieSceneClock>,
    pub selection_range: FMovieSceneFrameRange,
    pub playback_range: FMovieSceneFrameRange,
    pub tick_resolution: FFrameRate,
    pub display_rate: FFrameRate,
    pub evaluation_type: EMovieSceneEvaluationType,
    pub clock_source: EUpdateClockSource,
    pub custom_clock_source_path_deprecated: FSoftObjectPath,
    pub marked_frames: TArray<FMovieSceneMarkedFrame>,
    pub generated_conditions: TArray<UPtr<UMovieSceneGroupCondition>>,
    pub b_read_only: bool,
    pub b_playback_range_locked: bool,
    pub b_marked_frames_locked: bool,
    pub objects_to_display_names: TMap<FString, FText>,
    pub objects_to_labels: TMap<FString, FMovieSceneTrackLabels>,
    pub editor_data: FMovieSceneEditorData,
    pub root_folders: TArray<UPtr<UMovieSceneFolder>>,
    pub solo_nodes_deprecated: TArray<FString>,
    pub mute_nodes_deprecated: TArray<FString>,
    pub section_groups: TArray<FMovieSceneSectionGroup>,
    pub node_group_collection: UPtr<UMovieSceneNodeGroupCollection>,
    pub b_globally_show_marked_frames: bool,
    pub in_time_deprecated: f32,
    pub out_time_deprecated: f32,
    pub start_time_deprecated: f32,
    pub end_time_deprecated: f32,
    pub b_force_fixed_frame_interval_playback_deprecated: bool,
    pub fixed_frame_interval_deprecated: f32,
    pub master_tracks_deprecated: TArray<UPtr<UMovieSceneTrack>>,
}
pub struct UMovieSceneBindingOverrides {
    pub binding_data: TArray<FMovieSceneBindingOverrideData>,
}
pub struct UMovieSceneClock {}
pub struct UMovieSceneExternalClock {
    pub custom_clock_source_path: FSoftObjectPath,
}
pub struct UBuiltInDynamicBindingResolverLibrary {}
pub struct UMovieSceneFolder {
    pub folder_name: FName,
    pub child_folders: TArray<UPtr<UMovieSceneFolder>>,
    pub child_tracks: TArray<UPtr<UMovieSceneTrack>>,
    pub child_object_binding_strings: TArray<FString>,
    pub folder_color: FColor,
    pub sorting_order: i32,
    pub child_master_tracks_deprecated: TArray<UPtr<UMovieSceneTrack>>,
}
pub struct UMovieSceneMetaData {
    pub author: FString,
    pub created: FDateTime,
    pub notes: FString,
}
pub struct UMovieSceneSequencePlayer {
    pub observer: TScriptInterface<IMovieSceneSequencePlayerObserver>,
    pub on_play: FMovieSceneSequencePlayer_OnPlay,
    pub on_play_reverse: FMovieSceneSequencePlayer_OnPlayReverse,
    pub on_stop: FMovieSceneSequencePlayer_OnStop,
    pub on_pause: FMovieSceneSequencePlayer_OnPause,
    pub on_finished: FMovieSceneSequencePlayer_OnFinished,
    pub status: EMovieScenePlayerStatus,
    pub flags_724: u8,
    pub sequence: UPtr<UMovieSceneSequence>,
    pub start_time: FFrameNumber,
    pub duration_frames: i32,
    pub duration_sub_frames: f32,
    pub current_num_loops: i32,
    pub serial_number: i32,
    pub playback_settings: FMovieSceneSequencePlaybackSettings,
    pub root_template_instance: FMovieSceneRootEvaluationTemplateInstance,
    pub net_sync_props: FMovieSceneSequenceReplProperties,
    pub playback_client: TScriptInterface<IMovieScenePlaybackClient>,
    pub tick_manager: UPtr<UMovieSceneSequenceTickManager>,
}
pub struct UMovieSceneSequenceTickManager {}
pub struct UMovieSceneBindingLifetimeSection {}
pub struct UMovieSceneHookSection {
    pub flags_376: u8,
}
pub struct UMovieSceneSpawnSection {}
pub struct UMovieSceneTimeWarpSection {
    pub time_warp: FMovieSceneTimeWarpVariant,
}
pub struct UTestMovieSceneTrack {
    pub b_high_pass_filter: bool,
    pub section_array: TArray<UPtr<UMovieSceneSection>>,
}
pub struct UTestMovieSceneSection {}
pub struct UTestMovieSceneSequence {
    pub movie_scene: UPtr<UMovieScene>,
}
pub struct UTestMovieSceneSubTrack {
    pub section_array: TArray<UPtr<UMovieSceneSection>>,
}
pub struct UTestMovieSceneSubSection {}
pub struct UTestMovieSceneEvalHookTrack {
    pub section_array: TArray<UPtr<UMovieSceneSection>>,
}
pub struct UTestMovieSceneEvalHookSection {}
pub struct UTestMovieSceneObject {}
pub struct ATestMovieSceneArrayPropertiesActor {
    pub test_bool: bool,
    pub test_enum: ETestMovieSceneEnum,
    pub test_int32: i32,
    pub test_object: UPtr<UTestMovieSceneObject>,
    pub test_vector: FVector,
    pub multiple_floats: TArray<f32>,
    pub single_struct: FTestMovieSceneStruct,
    pub multiple_structs: TArray<FTestMovieSceneStruct>,
    pub single_instanced_struct: FInstancedStruct,
    pub multiple_instanced_structs: TArray<FInstancedStruct>,
    pub test_setter_float: f32,
}
pub struct UMovieSceneBindingLifetimeTrack {
    pub sections: TArray<UPtr<UMovieSceneSection>>,
}
pub struct UMovieSceneSpawnTrack {
    pub sections: TArray<UPtr<UMovieSceneSection>>,
    pub object_guid: FGuid,
}
pub struct UMovieSceneTimeWarpTrack {
    pub sections: TArray<UPtr<UMovieSceneSection>>,
    pub b_is_active_time_warp: bool,
}
pub struct UMovieSceneTimeWarpCurve {
    pub channel: FMovieSceneTimeWarpChannel,
}
