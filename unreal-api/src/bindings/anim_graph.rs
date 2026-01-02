#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAnimationGraph {
    __padding_end: [u8; 256],
}
impl UAnimationGraph {}
#[repr(C, align(8))]
pub struct UAnimationBlendSpaceSampleGraph {
    __padding_end: [u8; 264],
}
impl UAnimationBlendSpaceSampleGraph {}
#[repr(C, align(8))]
pub struct UAnimBlueprintExtension {
    __padding_end: [u8; 48],
}
impl UAnimBlueprintExtension {}
#[repr(C, align(8))]
pub struct UAnimBlueprintExtension_NodeRelevancy {
    __padding_end: [u8; 216],
}
impl UAnimBlueprintExtension_NodeRelevancy {}
#[repr(C, align(8))]
pub struct UAnimBlueprintExtension_SharedLinkedAnimLayers {
    __padding_end: [u8; 88],
}
impl UAnimBlueprintExtension_SharedLinkedAnimLayers {}
#[repr(C, align(8))]
pub struct UAnimGraphNodeBinding {
    __padding_end: [u8; 48],
}
impl UAnimGraphNodeBinding {}
pub struct UAnimGraphNodeCustomizationInterface {}
pub struct IAnimGraphNodeCustomizationInterface {}
pub struct UClassVariableCreator {}
pub struct IClassVariableCreator {}
#[repr(C, align(8))]
pub struct UAnimationConduitGraphSchema {
    __padding_end: [u8; 152],
}
impl UAnimationConduitGraphSchema {}
#[repr(C, align(8))]
pub struct UAnimationCustomTransitionGraph {
    __padding_end: [u8; 264],
}
impl UAnimationCustomTransitionGraph {}
#[repr(C, align(8))]
pub struct UAnimationGraphSchema {
    __padding_end: [u8; 256],
}
impl UAnimationGraphSchema {}
#[repr(C, align(8))]
pub struct UAnimationCustomTransitionSchema {
    __padding_end: [u8; 256],
}
impl UAnimationCustomTransitionSchema {}
#[repr(C, align(8))]
pub struct UAnimationStateGraph {
    __padding_end: [u8; 264],
}
impl UAnimationStateGraph {}
#[repr(C, align(8))]
pub struct UAnimationStateGraphSchema {
    __padding_end: [u8; 256],
}
impl UAnimationStateGraphSchema {}
#[repr(C, align(8))]
pub struct UAnimationStateMachineGraph {
    __padding_end: [u8; 208],
}
impl UAnimationStateMachineGraph {}
#[repr(C, align(8))]
pub struct UAnimationStateMachineSchema {
    __padding_end: [u8; 48],
}
impl UAnimationStateMachineSchema {}
#[repr(C, align(8))]
pub struct UAnimationTransitionGraph {
    __padding_end: [u8; 264],
}
impl UAnimationTransitionGraph {}
#[repr(C, align(8))]
pub struct UAnimationTransitionSchema {
    __padding_end: [u8; 152],
}
impl UAnimationTransitionSchema {}
#[repr(C, align(8))]
pub struct UAnimBlueprintExtension_Attributes {
    __padding_end: [u8; 48],
}
impl UAnimBlueprintExtension_Attributes {}
#[repr(C, align(8))]
pub struct UAnimBlueprintExtension_Base {
    __padding_end: [u8; 384],
}
impl UAnimBlueprintExtension_Base {}
#[repr(C, align(8))]
pub struct UAnimBlueprintExtension_BlendSpaceGraph {
    __padding_end: [u8; 80],
}
impl UAnimBlueprintExtension_BlendSpaceGraph {}
#[repr(C, align(8))]
pub struct UAnimBlueprintExtension_CachedPose {
    __padding_end: [u8; 128],
}
impl UAnimBlueprintExtension_CachedPose {}
#[repr(C, align(8))]
pub struct UAnimBlueprintExtension_CallFunction {
    __padding_end: [u8; 136],
}
impl UAnimBlueprintExtension_CallFunction {}
#[repr(C, align(8))]
pub struct UAnimBlueprintExtension_LinkedAnimGraph {
    __padding_end: [u8; 48],
}
impl UAnimBlueprintExtension_LinkedAnimGraph {}
#[repr(C, align(8))]
pub struct UAnimBlueprintExtension_LinkedInputPose {
    __padding_end: [u8; 48],
}
impl UAnimBlueprintExtension_LinkedInputPose {}
#[repr(C, align(8))]
pub struct UAnimBlueprintExtension_PropertyAccess {
    __padding_end: [u8; 312],
}
impl UAnimBlueprintExtension_PropertyAccess {}
#[repr(C, align(8))]
pub struct UAnimBlueprintExtension_StateMachine {
    __padding_end: [u8; 96],
}
impl UAnimBlueprintExtension_StateMachine {}
#[repr(C, align(8))]
pub struct UAnimBlueprintExtension_Tag {
    __padding_end: [u8; 240],
}
impl UAnimBlueprintExtension_Tag {}
#[repr(C, align(8))]
pub struct UAnimBlueprintPostCompileValidation {
    __padding_end: [u8; 48],
}
impl UAnimBlueprintPostCompileValidation {}
#[repr(C, align(8))]
pub struct UAnimGraphAttributes {
    __padding_end: [u8; 144],
}
impl UAnimGraphAttributes {}
#[repr(C, align(8))]
pub struct UAnimGraphNodeBinding_Base {
    __padding_end: [u8; 128],
}
impl UAnimGraphNodeBinding_Base {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_Base {
    __padding_end: [u8; 680],
}
impl UAnimGraphNode_Base {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_AssetPlayerBase {
    __padding_end: [u8; 720],
}
impl UAnimGraphNode_AssetPlayerBase {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_BlendSpaceBase {
    __padding_end: [u8; 720],
}
impl UAnimGraphNode_BlendSpaceBase {}
#[repr(C, align(16))]
pub struct UAnimGraphNode_AimOffsetLookAt {
    __padding_end: [u8; 1488],
}
impl UAnimGraphNode_AimOffsetLookAt {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_SkeletalControlBase {
    __padding_end: [u8; 680],
}
impl UAnimGraphNode_SkeletalControlBase {}
#[repr(C, align(16))]
pub struct UAnimGraphNode_AnimDynamics {
    __padding_end: [u8; 2720],
}
impl UAnimGraphNode_AnimDynamics {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_ApplyAdditive {
    __padding_end: [u8; 1024],
}
impl UAnimGraphNode_ApplyAdditive {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_ApplyLimits {
    __padding_end: [u8; 1136],
}
impl UAnimGraphNode_ApplyLimits {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_ApplyMeshSpaceAdditive {
    __padding_end: [u8; 1032],
}
impl UAnimGraphNode_ApplyMeshSpaceAdditive {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_BlendBoneByChannel {
    __padding_end: [u8; 920],
}
impl UAnimGraphNode_BlendBoneByChannel {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_BlendListBase {
    __padding_end: [u8; 688],
}
impl UAnimGraphNode_BlendListBase {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_BlendListByBool {
    __padding_end: [u8; 944],
}
impl UAnimGraphNode_BlendListByBool {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_BlendListByEnum {
    __padding_end: [u8; 1008],
}
impl UAnimGraphNode_BlendListByEnum {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_BlendListByInt {
    __padding_end: [u8; 936],
}
impl UAnimGraphNode_BlendListByInt {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_BlendSpaceEvaluator {
    __padding_end: [u8; 1104],
}
impl UAnimGraphNode_BlendSpaceEvaluator {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_BlendSpaceGraphBase {
    __padding_end: [u8; 736],
}
impl UAnimGraphNode_BlendSpaceGraphBase {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_BlendSpaceGraph {
    __padding_end: [u8; 992],
}
impl UAnimGraphNode_BlendSpaceGraph {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_BlendSpacePlayer {
    __padding_end: [u8; 1000],
}
impl UAnimGraphNode_BlendSpacePlayer {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_BlendSpaceSampleResult {
    __padding_end: [u8; 880],
}
impl UAnimGraphNode_BlendSpaceSampleResult {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_BoneDrivenController {
    __padding_end: [u8; 1216],
}
impl UAnimGraphNode_BoneDrivenController {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_CallFunction {
    __padding_end: [u8; 936],
}
impl UAnimGraphNode_CallFunction {}
#[repr(C, align(16))]
pub struct UAnimGraphNode_CCDIK {
    __padding_end: [u8; 1504],
}
impl UAnimGraphNode_CCDIK {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_ComponentToLocalSpace {
    __padding_end: [u8; 840],
}
impl UAnimGraphNode_ComponentToLocalSpace {}
#[repr(C, align(16))]
pub struct UAnimGraphNode_Constraint {
    __padding_end: [u8; 1488],
}
impl UAnimGraphNode_Constraint {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_CopyBone {
    __padding_end: [u8; 1248],
}
impl UAnimGraphNode_CopyBone {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_CopyBoneDelta {
    __padding_end: [u8; 1160],
}
impl UAnimGraphNode_CopyBoneDelta {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_CopyPoseFromMesh {
    __padding_end: [u8; 1144],
}
impl UAnimGraphNode_CopyPoseFromMesh {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_CurveSource {
    __padding_end: [u8; 872],
}
impl UAnimGraphNode_CurveSource {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_CustomProperty {
    __padding_end: [u8; 736],
}
impl UAnimGraphNode_CustomProperty {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_StateResult {
    __padding_end: [u8; 1272],
}
impl UAnimGraphNode_StateResult {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_CustomTransitionResult {
    __padding_end: [u8; 1272],
}
impl UAnimGraphNode_CustomTransitionResult {}
#[repr(C, align(16))]
pub struct UAnimGraphNode_DeadBlending {
    __padding_end: [u8; 2560],
}
impl UAnimGraphNode_DeadBlending {}
#[repr(C, align(16))]
pub struct UAnimGraphNode_Fabrik {
    __padding_end: [u8; 1552],
}
impl UAnimGraphNode_Fabrik {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_HandIKRetargeting {
    __padding_end: [u8; 1232],
}
impl UAnimGraphNode_HandIKRetargeting {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_RefPoseBase {
    __padding_end: [u8; 824],
}
impl UAnimGraphNode_RefPoseBase {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_IdentityPose {
    __padding_end: [u8; 824],
}
impl UAnimGraphNode_IdentityPose {}
#[repr(C, align(16))]
pub struct UAnimGraphNode_Inertialization {
    __padding_end: [u8; 2208],
}
impl UAnimGraphNode_Inertialization {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_LayeredBoneBlend {
    __padding_end: [u8; 1048],
}
impl UAnimGraphNode_LayeredBoneBlend {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_LegIK {
    __padding_end: [u8; 1160],
}
impl UAnimGraphNode_LegIK {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_LinkedAnimGraphBase {
    __padding_end: [u8; 824],
}
impl UAnimGraphNode_LinkedAnimGraphBase {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_LinkedAnimGraph {
    __padding_end: [u8; 1160],
}
impl UAnimGraphNode_LinkedAnimGraph {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_LinkedAnimLayer {
    __padding_end: [u8; 1232],
}
impl UAnimGraphNode_LinkedAnimLayer {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_LinkedInputPose {
    __padding_end: [u8; 1088],
}
impl UAnimGraphNode_LinkedInputPose {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_LocalRefPose {
    __padding_end: [u8; 824],
}
impl UAnimGraphNode_LocalRefPose {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_LocalToComponentSpace {
    __padding_end: [u8; 840],
}
impl UAnimGraphNode_LocalToComponentSpace {}
#[repr(C, align(16))]
pub struct UAnimGraphNode_LookAt {
    __padding_end: [u8; 2048],
}
impl UAnimGraphNode_LookAt {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_MakeDynamicAdditive {
    __padding_end: [u8; 872],
}
impl UAnimGraphNode_MakeDynamicAdditive {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_MeshRefPose {
    __padding_end: [u8; 816],
}
impl UAnimGraphNode_MeshRefPose {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_Mirror {
    __padding_end: [u8; 920],
}
impl UAnimGraphNode_Mirror {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_ModifyBone {
    __padding_end: [u8; 1304],
}
impl UAnimGraphNode_ModifyBone {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_ModifyCurve {
    __padding_end: [u8; 1096],
}
impl UAnimGraphNode_ModifyCurve {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_MultiWayBlend {
    __padding_end: [u8; 888],
}
impl UAnimGraphNode_MultiWayBlend {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_ObserveBone {
    __padding_end: [u8; 1200],
}
impl UAnimGraphNode_ObserveBone {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_PoseHandler {
    __padding_end: [u8; 720],
}
impl UAnimGraphNode_PoseHandler {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_PoseBlendNode {
    __padding_end: [u8; 1120],
}
impl UAnimGraphNode_PoseBlendNode {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_PoseByName {
    __padding_end: [u8; 1096],
}
impl UAnimGraphNode_PoseByName {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_PoseDriver {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub node: crate::bindings::anim_graph_runtime::FAnimNode_PoseDriver,
    __padding_end: [u8; 152],
}
impl UAnimGraphNode_PoseDriver {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_PoseSnapshot {
    __padding_end: [u8; 976],
}
impl UAnimGraphNode_PoseSnapshot {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_RandomPlayer {
    __padding_end: [u8; 928],
}
impl UAnimGraphNode_RandomPlayer {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_ResetRoot {
    __padding_end: [u8; 1120],
}
impl UAnimGraphNode_ResetRoot {}
#[repr(C, align(16))]
pub struct UAnimGraphNode_RigidBody {
    __padding_end: [u8; 3344],
}
impl UAnimGraphNode_RigidBody {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_Root {
    __padding_end: [u8; 880],
}
impl UAnimGraphNode_Root {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_RotateRootBone {
    __padding_end: [u8; 984],
}
impl UAnimGraphNode_RotateRootBone {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_RotationMultiplier {
    __padding_end: [u8; 1248],
}
impl UAnimGraphNode_RotationMultiplier {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_RotationOffsetBlendSpace {
    __padding_end: [u8; 1280],
}
impl UAnimGraphNode_RotationOffsetBlendSpace {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_RotationOffsetBlendSpaceGraph {
    __padding_end: [u8; 1176],
}
impl UAnimGraphNode_RotationOffsetBlendSpaceGraph {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_SaveCachedPose {
    __padding_end: [u8; 976],
}
impl UAnimGraphNode_SaveCachedPose {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_ScaleChainLength {
    __padding_end: [u8; 952],
}
impl UAnimGraphNode_ScaleChainLength {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_SequenceEvaluator {
    __padding_end: [u8; 960],
}
impl UAnimGraphNode_SequenceEvaluator {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_SequencePlayer {
    __padding_end: [u8; 1048],
}
impl UAnimGraphNode_SequencePlayer {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_Slot {
    __padding_end: [u8; 984],
}
impl UAnimGraphNode_Slot {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_SplineIK {
    __padding_end: [u8; 1632],
}
impl UAnimGraphNode_SplineIK {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_SpringBone {
    __padding_end: [u8; 1392],
}
impl UAnimGraphNode_SpringBone {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_StateMachineBase {
    __padding_end: [u8; 712],
}
impl UAnimGraphNode_StateMachineBase {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_StateMachine {
    __padding_end: [u8; 1072],
}
impl UAnimGraphNode_StateMachine {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_Sync {
    __padding_end: [u8; 856],
}
impl UAnimGraphNode_Sync {}
#[repr(C, align(16))]
pub struct UAnimGraphNode_Trail {
    __padding_end: [u8; 1744],
}
impl UAnimGraphNode_Trail {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_TransitionPoseEvaluator {
    #[doc(hidden)]
    __padding_680: [u8; 680],
    pub node: crate::bindings::engine::FAnimNode_TransitionPoseEvaluator,
}
impl UAnimGraphNode_TransitionPoseEvaluator {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_TransitionResult {
    __padding_end: [u8; 848],
}
impl UAnimGraphNode_TransitionResult {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_TwistCorrectiveNode {
    __padding_end: [u8; 1296],
}
impl UAnimGraphNode_TwistCorrectiveNode {}
#[repr(C, align(16))]
pub struct UAnimGraphNode_TwoBoneIK {
    __padding_end: [u8; 1840],
}
impl UAnimGraphNode_TwoBoneIK {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_TwoWayBlend {
    __padding_end: [u8; 1016],
}
impl UAnimGraphNode_TwoWayBlend {}
#[repr(C, align(8))]
pub struct UAnimGraphNode_UseCachedPose {
    __padding_end: [u8; 880],
}
impl UAnimGraphNode_UseCachedPose {}
#[repr(C, align(8))]
pub struct UAnimGraphSettings {
    __padding_end: [u8; 112],
}
impl UAnimGraphSettings {}
#[repr(C, align(16))]
pub struct UAnimPreviewAttacheInstance {
    __padding_end: [u8; 1136],
}
impl UAnimPreviewAttacheInstance {}
#[repr(C, align(16))]
pub struct UAnimPreviewInstance {
    __padding_end: [u8; 1184],
}
impl UAnimPreviewInstance {}
#[repr(C, align(8))]
pub struct UAnimStateNodeBase {
    __padding_end: [u8; 192],
}
impl UAnimStateNodeBase {}
#[repr(C, align(8))]
pub struct UAnimStateAliasNode {
    #[doc(hidden)]
    __padding_192: [u8; 192],
    pub b_global_alias: bool,
    __padding_end: [u8; 103],
}
impl UAnimStateAliasNode {}
#[repr(C, align(8))]
pub struct UAnimStateConduitNode {
    __padding_end: [u8; 200],
}
impl UAnimStateConduitNode {}
#[repr(C, align(8))]
pub struct UAnimStateEntryNode {
    __padding_end: [u8; 192],
}
impl UAnimStateEntryNode {}
#[repr(C, align(8))]
pub struct UAnimStateNode {
    __padding_end: [u8; 888],
}
impl UAnimStateNode {}
#[repr(C, align(8))]
pub struct UAnimStateTransitionNode {
    #[doc(hidden)]
    __padding_296: [u8; 296],
    pub logic_type: crate::bindings::engine::ETransitionLogicType,
    __padding_end: [u8; 791],
}
impl UAnimStateTransitionNode {}
#[repr(C, align(8))]
pub struct UBlendSpaceGraph {
    __padding_end: [u8; 200],
}
impl UBlendSpaceGraph {}
#[repr(C, align(8))]
pub struct UK2Node_AnimGetter {
    __padding_end: [u8; 408],
}
impl UK2Node_AnimGetter {}
#[repr(C, align(8))]
pub struct UK2Node_AnimNodeReference {
    __padding_end: [u8; 208],
}
impl UK2Node_AnimNodeReference {}
#[repr(C, align(8))]
pub struct UK2Node_PlayMontage {
    __padding_end: [u8; 240],
}
impl UK2Node_PlayMontage {}
#[repr(C, align(8))]
pub struct UK2Node_TransitionRuleGetter {
    __padding_end: [u8; 216],
}
impl UK2Node_TransitionRuleGetter {}
#[repr(transparent)]
pub struct EAnimGraphAttributesDisplayMode(pub i32);
impl EAnimGraphAttributesDisplayMode {
    pub const HIDE_ON_PINS: EAnimGraphAttributesDisplayMode = EAnimGraphAttributesDisplayMode(
        0,
    );
    pub const SHOW_ON_PINS: EAnimGraphAttributesDisplayMode = EAnimGraphAttributesDisplayMode(
        1,
    );
    pub const AUTOMATIC: EAnimGraphAttributesDisplayMode = EAnimGraphAttributesDisplayMode(
        2,
    );
}
#[repr(transparent)]
pub struct EAnimGraphAttributeBlend(pub i32);
impl EAnimGraphAttributeBlend {
    pub const BLENDABLE: EAnimGraphAttributeBlend = EAnimGraphAttributeBlend(0);
    pub const NON_BLENDABLE: EAnimGraphAttributeBlend = EAnimGraphAttributeBlend(1);
}
#[repr(transparent)]
pub struct EAnimGraphNodePropertyBindingType(pub i32);
impl EAnimGraphNodePropertyBindingType {
    pub const NONE: EAnimGraphNodePropertyBindingType = EAnimGraphNodePropertyBindingType(
        0,
    );
    pub const PROPERTY: EAnimGraphNodePropertyBindingType = EAnimGraphNodePropertyBindingType(
        1,
    );
    pub const FUNCTION: EAnimGraphNodePropertyBindingType = EAnimGraphNodePropertyBindingType(
        2,
    );
}
#[repr(transparent)]
pub struct EBlueprintUsage(pub u8);
impl EBlueprintUsage {
    pub const NO_PROPERTIES: EBlueprintUsage = EBlueprintUsage(0);
    pub const DOES_NOT_USE_BLUEPRINT: EBlueprintUsage = EBlueprintUsage(1);
    pub const USES_BLUEPRINT: EBlueprintUsage = EBlueprintUsage(2);
}
#[repr(transparent)]
pub struct EMontagePreviewType(pub u8);
impl EMontagePreviewType {
    pub const EMPT_NORMAL: EMontagePreviewType = EMontagePreviewType(0);
    pub const EMPT_ALL_SECTIONS: EMontagePreviewType = EMontagePreviewType(1);
}
#[repr(transparent)]
pub struct EAnimStateType(pub u8);
impl EAnimStateType {
    pub const AST_SINGLE_ANIMATION: EAnimStateType = EAnimStateType(0);
    pub const AST_BLEND_GRAPH: EAnimStateType = EAnimStateType(1);
}
#[repr(transparent)]
pub struct ETransitionGetter(pub u8);
impl ETransitionGetter {
    pub const ANIMATION_ASSET_GET_CURRENT_TIME: ETransitionGetter = ETransitionGetter(0);
    pub const ANIMATION_ASSET_GET_LENGTH: ETransitionGetter = ETransitionGetter(1);
    pub const ANIMATION_ASSET_GET_CURRENT_TIME_FRACTION: ETransitionGetter = ETransitionGetter(
        2,
    );
    pub const ANIMATION_ASSET_GET_TIME_FROM_END: ETransitionGetter = ETransitionGetter(
        3,
    );
    pub const ANIMATION_ASSET_GET_TIME_FROM_END_FRACTION: ETransitionGetter = ETransitionGetter(
        4,
    );
    pub const CURRENT_STATE_ELAPSED_TIME: ETransitionGetter = ETransitionGetter(5);
    pub const CURRENT_STATE_GET_BLEND_WEIGHT: ETransitionGetter = ETransitionGetter(6);
    pub const CURRENT_TRANSITION_DURATION: ETransitionGetter = ETransitionGetter(7);
    pub const ARBITRARY_STATE_GET_BLEND_WEIGHT: ETransitionGetter = ETransitionGetter(8);
}
