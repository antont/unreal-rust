#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_NewStateNode {}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_NewStateComment {}
#[repr(C, align(16))]
pub struct FAnimGraphAttributeDesc {
    pub name: FName,
    pub icon: FSlateBrush,
    pub display_name: FText,
    pub tool_tip_text: FText,
    pub color: FSlateColor,
    pub display_mode: EAnimGraphAttributesDisplayMode,
    pub blend: EAnimGraphAttributeBlend,
    pub sort_order: i32,
}
#[repr(C, align(8))]
pub struct FAnimGraphNodePropertyBinding {
    pub pin_type: FEdGraphPinType,
    pub promoted_pin_type: FEdGraphPinType,
    pub property_name: FName,
    pub array_index: i32,
    pub path_as_text: FText,
    pub property_path: TArray<FString>,
    pub context_id: FName,
    pub compiled_context: FText,
    pub compiled_context_desc: FText,
    pub ty: EAnimGraphNodePropertyBindingType,
    pub b_is_bound: bool,
    pub b_is_promotion: bool,
    pub b_only_update_when_active: bool,
}
#[repr(C, align(8))]
pub struct FAnimBlueprintFunctionPinInfo {
    pub name: FName,
    pub ty: FEdGraphPinType,
}
#[repr(C, align(16))]
pub struct FAnimPreviewAttacheInstanceProxy {}
#[repr(C, align(16))]
pub struct FAnimPreviewInstanceProxy {}
#[repr(C, align(8))]
pub struct FNodeSpawnData {
    pub cached_title: FText,
    pub source_node: UPtr<UAnimGraphNode_Base>,
    pub source_state_node: UPtr<UAnimStateNodeBase>,
    pub anim_instance_class: TSubclassOf<UObject>,
    pub source_blueprint: UPtr<UAnimBlueprint>,
    pub getter: UPtr<UField>,
    pub getter_context_string: FString,
}
pub struct UAnimationGraph {
    pub blend_options: FAnimGraphBlendOptions,
}
pub struct UAnimationBlendSpaceSampleGraph {
    pub result_node: UPtr<UAnimGraphNode_BlendSpaceSampleResult>,
}
pub struct UAnimBlueprintExtension {}
pub struct UAnimBlueprintExtension_NodeRelevancy {
    pub subsystem: FAnimSubsystemInstance_NodeRelevancy,
}
pub struct UAnimBlueprintExtension_SharedLinkedAnimLayers {
    pub subsystem: FAnimSubsystem_SharedLinkedAnimLayers,
}
pub struct UAnimGraphNodeBinding {}
pub struct UAnimGraphNodeCustomizationInterface {}
pub struct IAnimGraphNodeCustomizationInterface {}
pub struct UClassVariableCreator {}
pub struct IClassVariableCreator {}
pub struct UAnimationConduitGraphSchema {}
pub struct UAnimationCustomTransitionGraph {
    pub my_result_node: UPtr<UAnimGraphNode_CustomTransitionResult>,
}
pub struct UAnimationGraphSchema {
    pub pn_sequence_name: FString,
    pub name_never_as_pin: FName,
    pub name_pin_hidden_by_default: FName,
    pub name_pin_shown_by_default: FName,
    pub name_always_as_pin: FName,
    pub name_customize_property: FName,
    pub name_on_evaluate: FName,
    pub default_evaluation_handler_name: FName,
}
pub struct UAnimationCustomTransitionSchema {}
pub struct UAnimationStateGraph {
    pub my_result_node: UPtr<UAnimGraphNode_StateResult>,
}
pub struct UAnimationStateGraphSchema {}
pub struct UAnimationStateMachineGraph {
    pub entry_node: UPtr<UAnimStateEntryNode>,
    pub owner_anim_graph_node: UPtr<UAnimGraphNode_StateMachineBase>,
}
pub struct UAnimationStateMachineSchema {}
pub struct UAnimationTransitionGraph {
    pub my_result_node: UPtr<UAnimGraphNode_TransitionResult>,
}
pub struct UAnimationTransitionSchema {}
pub struct UAnimBlueprintExtension_Attributes {}
pub struct UAnimBlueprintExtension_Base {
    pub subsystem: FAnimSubsystem_Base,
}
pub struct UAnimBlueprintExtension_BlendSpaceGraph {
    pub class: TSubclassOf<UObject>,
    pub subsystem: FAnimSubsystem_BlendSpaceGraph,
}
pub struct UAnimBlueprintExtension_CachedPose {}
pub struct UAnimBlueprintExtension_CallFunction {}
pub struct UAnimBlueprintExtension_LinkedAnimGraph {}
pub struct UAnimBlueprintExtension_LinkedInputPose {}
pub struct UAnimBlueprintExtension_PropertyAccess {
    pub subsystem: FAnimSubsystem_PropertyAccess,
}
pub struct UAnimBlueprintExtension_StateMachine {}
pub struct UAnimBlueprintExtension_Tag {
    pub subsystem: FAnimSubsystem_Tag,
}
pub struct UAnimBlueprintPostCompileValidation {}
pub struct UAnimGraphAttributes {
    pub attributes: TArray<FAnimGraphAttributeDesc>,
}
pub struct UAnimGraphNodeBinding_Base {
    pub property_bindings: TMap<FName, FAnimGraphNodePropertyBinding>,
}
pub struct UAnimGraphNode_Base {
    pub show_pin_for_properties: TArray<FOptionalPinFromProperty>,
    pub property_bindings_deprecated: TMap<FName, FAnimGraphNodePropertyBinding>,
    pub always_dynamic_properties: TSet<FName>,
    pub blueprint_usage: EBlueprintUsage,
    pub initial_update_function: FMemberReference,
    pub become_relevant_function: FMemberReference,
    pub update_function: FMemberReference,
    pub binding: UPtr<UAnimGraphNodeBinding>,
    pub tag: FName,
}
pub struct UAnimGraphNode_AssetPlayerBase {
    pub sync_group_deprecated: FAnimationGroupReference,
}
pub struct UAnimGraphNode_BlendSpaceBase {}
pub struct UAnimGraphNode_AimOffsetLookAt {
    pub node: FAnimNode_AimOffsetLookAt,
}
pub struct UAnimGraphNode_SkeletalControlBase {}
pub struct UAnimGraphNode_AnimDynamics {
    pub node: FAnimNode_AnimDynamics,
    pub b_preview_live: bool,
    pub b_show_linear_limits: bool,
    pub b_show_angular_limits: bool,
    pub b_show_planar_limit: bool,
    pub b_show_spherical_limit: bool,
    pub b_show_collision_spheres: bool,
}
pub struct UAnimGraphNode_ApplyAdditive {
    pub node: FAnimNode_ApplyAdditive,
}
pub struct UAnimGraphNode_ApplyLimits {
    pub node: FAnimNode_ApplyLimits,
}
pub struct UAnimGraphNode_ApplyMeshSpaceAdditive {
    pub node: FAnimNode_ApplyMeshSpaceAdditive,
}
pub struct UAnimGraphNode_BlendBoneByChannel {
    pub blend_node: FAnimNode_BlendBoneByChannel,
}
pub struct UAnimGraphNode_BlendListBase {}
pub struct UAnimGraphNode_BlendListByBool {
    pub node: FAnimNode_BlendListByBool,
}
pub struct UAnimGraphNode_BlendListByEnum {
    pub node: FAnimNode_BlendListByEnum,
    pub bound_enum: UPtr<UEnum>,
    pub visible_enum_entries: TArray<FName>,
}
pub struct UAnimGraphNode_BlendListByInt {
    pub node: FAnimNode_BlendListByInt,
}
pub struct UAnimGraphNode_BlendSpaceEvaluator {
    pub node: FAnimNode_BlendSpaceEvaluator,
}
pub struct UAnimGraphNode_BlendSpaceGraphBase {
    pub blend_space: UPtr<UBlendSpace>,
    pub blend_space_class: TSubclassOf<UBlendSpace>,
    pub blend_space_graph: UPtr<UBlendSpaceGraph>,
    pub graphs: TArray<UPtr<UEdGraph>>,
}
pub struct UAnimGraphNode_BlendSpaceGraph {
    pub node: FAnimNode_BlendSpaceGraph,
}
pub struct UAnimGraphNode_BlendSpacePlayer {
    pub node: FAnimNode_BlendSpacePlayer,
}
pub struct UAnimGraphNode_BlendSpaceSampleResult {
    pub node: FAnimNode_BlendSpaceSampleResult,
}
pub struct UAnimGraphNode_BoneDrivenController {
    pub node: FAnimNode_BoneDrivenController,
}
pub struct UAnimGraphNode_CallFunction {
    pub inner_graph: UPtr<UEdGraph>,
    pub call_function_prototype: UPtr<UK2Node_CallFunction>,
    pub node: FAnimNode_CallFunction,
}
pub struct UAnimGraphNode_CCDIK {
    pub node: FAnimNode_CCDIK,
}
pub struct UAnimGraphNode_ComponentToLocalSpace {
    pub node: FAnimNode_ConvertComponentToLocalSpace,
}
pub struct UAnimGraphNode_Constraint {
    pub node: FAnimNode_Constraint,
}
pub struct UAnimGraphNode_CopyBone {
    pub node: FAnimNode_CopyBone,
}
pub struct UAnimGraphNode_CopyBoneDelta {
    pub node: FAnimNode_CopyBoneDelta,
}
pub struct UAnimGraphNode_CopyPoseFromMesh {
    pub node: FAnimNode_CopyPoseFromMesh,
}
pub struct UAnimGraphNode_CurveSource {
    pub node: FAnimNode_CurveSource,
}
pub struct UAnimGraphNode_CustomProperty {
    pub known_exposable_properties_deprecated: TArray<FName>,
    pub exposed_property_names_deprecated: TArray<FName>,
    pub custom_pin_properties: TArray<FOptionalPinFromProperty>,
}
pub struct UAnimGraphNode_StateResult {
    pub node: FAnimNode_StateResult,
    pub state_entry_function: FMemberReference,
    pub state_fully_blended_in_function: FMemberReference,
    pub state_exit_function: FMemberReference,
    pub state_fully_blended_out_function: FMemberReference,
}
pub struct UAnimGraphNode_CustomTransitionResult {}
pub struct UAnimGraphNode_DeadBlending {
    pub node: FAnimNode_DeadBlending,
}
pub struct UAnimGraphNode_Fabrik {
    pub node: FAnimNode_Fabrik,
}
pub struct UAnimGraphNode_HandIKRetargeting {
    pub node: FAnimNode_HandIKRetargeting,
}
pub struct UAnimGraphNode_RefPoseBase {
    pub node: FAnimNode_RefPose,
}
pub struct UAnimGraphNode_IdentityPose {}
pub struct UAnimGraphNode_Inertialization {
    pub node: FAnimNode_Inertialization,
}
pub struct UAnimGraphNode_LayeredBoneBlend {
    pub node: FAnimNode_LayeredBoneBlend,
}
pub struct UAnimGraphNode_LegIK {
    pub node: FAnimNode_LegIK,
}
pub struct UAnimGraphNode_LinkedAnimGraphBase {
    pub function_reference: FMemberReference,
}
pub struct UAnimGraphNode_LinkedAnimGraph {
    pub node: FAnimNode_LinkedAnimGraph,
}
pub struct UAnimGraphNode_LinkedAnimLayer {
    pub node: FAnimNode_LinkedAnimLayer,
    pub interface_guid: FGuid,
}
pub struct UAnimGraphNode_LinkedInputPose {
    pub node: FAnimNode_LinkedInputPose,
    pub inputs: TArray<FAnimBlueprintFunctionPinInfo>,
    pub function_reference: FMemberReference,
    pub input_pose_index: i32,
}
pub struct UAnimGraphNode_LocalRefPose {}
pub struct UAnimGraphNode_LocalToComponentSpace {
    pub node: FAnimNode_ConvertLocalToComponentSpace,
}
pub struct UAnimGraphNode_LookAt {
    pub node: FAnimNode_LookAt,
}
pub struct UAnimGraphNode_MakeDynamicAdditive {
    pub node: FAnimNode_MakeDynamicAdditive,
}
pub struct UAnimGraphNode_MeshRefPose {
    pub node: FAnimNode_MeshSpaceRefPose,
}
pub struct UAnimGraphNode_Mirror {
    pub node: FAnimNode_Mirror,
}
pub struct UAnimGraphNode_ModifyBone {
    pub node: FAnimNode_ModifyBone,
}
pub struct UAnimGraphNode_ModifyCurve {
    pub node: FAnimNode_ModifyCurve,
}
pub struct UAnimGraphNode_MultiWayBlend {
    pub node: FAnimNode_MultiWayBlend,
}
pub struct UAnimGraphNode_ObserveBone {
    pub node: FAnimNode_ObserveBone,
}
pub struct UAnimGraphNode_PoseHandler {}
pub struct UAnimGraphNode_PoseBlendNode {
    pub node: FAnimNode_PoseBlendNode,
}
pub struct UAnimGraphNode_PoseByName {
    pub node: FAnimNode_PoseByName,
}
pub struct UAnimGraphNode_PoseDriver {
    pub node: FAnimNode_PoseDriver,
    pub axis_length: f32,
    pub cone_subdivision: i32,
    pub b_draw_debug_cones: bool,
    pub last_preview_component: UPtr<USkeletalMeshComponent>,
}
pub struct UAnimGraphNode_PoseSnapshot {
    pub node: FAnimNode_PoseSnapshot,
}
pub struct UAnimGraphNode_RandomPlayer {
    pub node: FAnimNode_RandomPlayer,
}
pub struct UAnimGraphNode_ResetRoot {
    pub node: FAnimNode_ResetRoot,
}
pub struct UAnimGraphNode_RigidBody {
    pub node: FAnimNode_RigidBody,
}
pub struct UAnimGraphNode_Root {
    pub node: FAnimNode_Root,
}
pub struct UAnimGraphNode_RotateRootBone {
    pub node: FAnimNode_RotateRootBone,
}
pub struct UAnimGraphNode_RotationMultiplier {
    pub node: FAnimNode_RotationMultiplier,
}
pub struct UAnimGraphNode_RotationOffsetBlendSpace {
    pub node: FAnimNode_RotationOffsetBlendSpace,
}
pub struct UAnimGraphNode_RotationOffsetBlendSpaceGraph {
    pub node: FAnimNode_RotationOffsetBlendSpaceGraph,
}
pub struct UAnimGraphNode_SaveCachedPose {
    pub node: FAnimNode_SaveCachedPose,
    pub cache_name: FString,
}
pub struct UAnimGraphNode_ScaleChainLength {
    pub node: FAnimNode_ScaleChainLength,
}
pub struct UAnimGraphNode_SequenceEvaluator {
    pub node: FAnimNode_SequenceEvaluator,
}
pub struct UAnimGraphNode_SequencePlayer {
    pub node: FAnimNode_SequencePlayer,
}
pub struct UAnimGraphNode_Slot {
    pub node: FAnimNode_Slot,
}
pub struct UAnimGraphNode_SplineIK {
    pub node: FAnimNode_SplineIK,
}
pub struct UAnimGraphNode_SpringBone {
    pub node: FAnimNode_SpringBone,
}
pub struct UAnimGraphNode_StateMachineBase {
    pub editor_state_machine_graph: UPtr<UAnimationStateMachineGraph>,
}
pub struct UAnimGraphNode_StateMachine {
    pub node: FAnimNode_StateMachine,
}
pub struct UAnimGraphNode_Sync {
    pub node: FAnimNode_Sync,
}
pub struct UAnimGraphNode_Trail {
    pub node: FAnimNode_Trail,
}
pub struct UAnimGraphNode_TransitionPoseEvaluator {
    pub node: FAnimNode_TransitionPoseEvaluator,
}
pub struct UAnimGraphNode_TransitionResult {
    pub node: FAnimNode_TransitionResult,
}
pub struct UAnimGraphNode_TwistCorrectiveNode {
    pub node: FAnimNode_TwistCorrectiveNode,
}
pub struct UAnimGraphNode_TwoBoneIK {
    pub node: FAnimNode_TwoBoneIK,
    pub b_enable_debug_draw: bool,
}
pub struct UAnimGraphNode_TwoWayBlend {
    pub blend_node: FAnimNode_TwoWayBlend,
}
pub struct UAnimGraphNode_UseCachedPose {
    pub node: FAnimNode_UseCachedPose,
    pub save_cached_pose_node: TWeakObjectPtr<UAnimGraphNode_SaveCachedPose>,
    pub name_of_cache: FString,
}
pub struct UAnimGraphSettings {
    pub b_show_instanced_enum_blend_anim_node_blueprint_actions: bool,
}
pub struct UAnimPreviewAttacheInstance {}
pub struct UAnimPreviewInstance {
    pub montage_preview_type: EMontagePreviewType,
    pub montage_preview_start_section_idx: i32,
}
pub struct UAnimStateNodeBase {}
pub struct UAnimStateAliasNode {
    pub b_global_alias: bool,
    pub state_alias_name: FString,
    pub aliased_state_nodes: TSet<TWeakObjectPtr<UAnimStateNodeBase>>,
}
pub struct UAnimStateConduitNode {
    pub bound_graph: UPtr<UEdGraph>,
}
pub struct UAnimStateEntryNode {}
pub struct UAnimStateNode {
    pub bound_graph: UPtr<UEdGraph>,
    pub state_type: EAnimStateType,
    pub state_entered: FAnimNotifyEvent,
    pub state_left: FAnimNotifyEvent,
    pub state_fully_blended: FAnimNotifyEvent,
    pub b_always_reset_on_entry: bool,
}
pub struct UAnimStateTransitionNode {
    pub bound_graph: UPtr<UEdGraph>,
    pub custom_transition_graph: UPtr<UEdGraph>,
    pub priority_order: i32,
    pub crossfade_duration: f32,
    pub crossfade_mode_deprecated: ETransitionBlendMode,
    pub blend_mode: EAlphaBlendOption,
    pub custom_blend_curve: UPtr<UCurveFloat>,
    pub blend_profile_deprecated: UPtr<UBlendProfile>,
    pub blend_profile_wrapper: FBlendProfileInterfaceWrapper,
    pub b_automatic_rule_based_on_sequence_player_in_state: bool,
    pub automatic_rule_trigger_time: f32,
    pub min_time_before_reentry: f32,
    pub sync_group_name_to_require_valid_markers_rule: FName,
    pub logic_type: ETransitionLogicType,
    pub transition_start: FAnimNotifyEvent,
    pub transition_end: FAnimNotifyEvent,
    pub transition_interrupt: FAnimNotifyEvent,
    pub b_allow_inertialization_for_self_transitions: bool,
    pub bidirectional: bool,
    pub b_disabled: bool,
    pub b_shared_rules: bool,
    pub b_shared_crossfade: bool,
    pub shared_rules_name: FString,
    pub shared_rules_guid: FGuid,
    pub shared_color: FLinearColor,
    pub shared_crossfade_name: FString,
    pub shared_crossfade_guid: FGuid,
    pub shared_crossfade_idx: i32,
    pub cached_rotation: FVector2D,
}
pub struct UBlendSpaceGraph {
    pub blend_space: UPtr<UBlendSpace>,
}
pub struct UK2Node_AnimGetter {
    pub source_node: UPtr<UAnimGraphNode_Base>,
    pub source_state_node: UPtr<UAnimStateNodeBase>,
    pub getter_class: TSubclassOf<UObject>,
    pub source_anim_blueprint: UPtr<UAnimBlueprint>,
    pub cached_title: FText,
    pub contexts: TArray<FString>,
}
pub struct UK2Node_AnimNodeReference {
    pub tag: FName,
}
pub struct UK2Node_PlayMontage {}
pub struct UK2Node_TransitionRuleGetter {
    pub getter_type: ETransitionGetter,
    pub associated_anim_asset_player_node: UPtr<UAnimGraphNode_Base>,
    pub associated_state_node: UPtr<UAnimStateNode>,
}
