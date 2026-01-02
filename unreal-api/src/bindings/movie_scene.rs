#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FMovieSceneBindingProxy {
    pub binding_id: crate::bindings::core_u_object::FGuid,
    pub sequence: UPtr<UMovieSceneSequence>,
}
impl FMovieSceneBindingProxy {}
#[repr(C, align(8))]
pub struct FMovieSceneDynamicBindingPayloadVariable {
    __padding_end: [u8; 56],
}
impl FMovieSceneDynamicBindingPayloadVariable {}
#[repr(C, align(4))]
pub struct FMovieSceneObjectBindingID {
    __padding_end: [u8; 28],
}
impl FMovieSceneObjectBindingID {}
#[repr(C, align(8))]
pub struct FMovieSceneTimeWarpVariant {
    __padding_end: [u8; 16],
}
impl FMovieSceneTimeWarpVariant {}
#[repr(C, align(8))]
pub struct FMovieSceneNumericVariant {
    __padding_end: [u8; 16],
}
impl FMovieSceneNumericVariant {}
#[repr(C, align(4))]
pub struct FActorForWorldTransforms {
    pub actor: TWeakObjectPtr<crate::bindings::engine::AActor>,
    pub component: TWeakObjectPtr<crate::bindings::engine::USceneComponent>,
    pub socket_name: FName,
}
impl FActorForWorldTransforms {}
#[repr(C, align(8))]
pub struct FMovieSceneSectionParameters {
    pub start_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub b_can_loop: bool,
    pub end_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub first_loop_start_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub time_scale: FMovieSceneTimeWarpVariant,
    pub hierarchical_bias: i32,
    pub flags: EMovieSceneSubSectionFlags,
    __padding_end: [u8; 19],
}
impl FMovieSceneSectionParameters {}
#[repr(C, align(4))]
pub struct FMovieSceneSequenceLoopCount {
    pub value: i32,
}
impl FMovieSceneSequenceLoopCount {}
#[repr(C, align(4))]
pub struct FMovieSceneSequencePlaybackSettings {
    pub flags_0: u8,
    pub loop_count: FMovieSceneSequenceLoopCount,
    pub tick_interval: FMovieSceneSequenceTickInterval,
    pub play_rate: f32,
    pub start_time: f32,
    pub flags_28: u8,
    #[doc(hidden)]
    __padding_32: [u8; 3],
    pub finish_completion_state_override: EMovieSceneCompletionModeOverride,
    #[doc(hidden)]
    __padding_36: [u8; 3],
    pub flags_36: u8,
    __padding_end: [u8; 3],
}
impl FMovieSceneSequencePlaybackSettings {}
#[repr(C, align(4))]
pub struct FMovieSceneSequenceTickInterval {
    pub tick_interval_seconds: f32,
    pub evaluation_budget_microseconds: f32,
    pub b_tick_when_paused: bool,
    pub b_allow_rounding: bool,
    __padding_end: [u8; 2],
}
impl FMovieSceneSequenceTickInterval {}
#[repr(C, align(8))]
pub struct FMovieSceneBindingResolveResult {
    pub objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub object: UPtr<crate::bindings::core_u_object::UObject>,
}
impl FMovieSceneBindingResolveResult {}
#[repr(C, align(8))]
pub struct FMovieSceneBindingResolveContext {
    pub world_context: UPtr<crate::bindings::core_u_object::UObject>,
    pub binding: FMovieSceneBindingProxy,
}
impl FMovieSceneBindingResolveContext {}
#[repr(C, align(8))]
pub struct FMovieSceneConditionContext {
    pub world_context: UPtr<crate::bindings::core_u_object::UObject>,
    pub binding: FMovieSceneBindingProxy,
    pub bound_objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
impl FMovieSceneConditionContext {}
#[repr(C, align(8))]
pub struct FMovieSceneConditionContainer {
    pub condition: UPtr<UMovieSceneCondition>,
}
impl FMovieSceneConditionContainer {}
#[repr(C, align(1))]
pub struct FOptionalMovieSceneBlendType {
    pub blend_type: EMovieSceneBlendType,
    pub b_is_valid: bool,
}
impl FOptionalMovieSceneBlendType {}
#[repr(C, align(8))]
pub struct FMovieSceneMarkedFrame {
    pub frame_number: crate::bindings::core_u_object::FFrameNumber,
    pub label: FString,
    #[doc(hidden)]
    __padding_76: [u8; 52],
    pub b_is_determinism_fence: bool,
    pub b_is_inclusive_time: bool,
    __padding_end: [u8; 2],
}
impl FMovieSceneMarkedFrame {}
#[repr(C, align(8))]
pub struct FMovieSceneDynamicBindingResolveParams {
    pub sequence: UPtr<UMovieSceneSequence>,
    pub object_binding_id: crate::bindings::core_u_object::FGuid,
    pub root_sequence: UPtr<UMovieSceneSequence>,
}
impl FMovieSceneDynamicBindingResolveParams {}
#[repr(C, align(8))]
pub struct FMovieSceneDynamicBindingResolveResult {
    pub objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub object: UPtr<crate::bindings::core_u_object::UObject>,
    pub b_is_possessed_object: bool,
    __padding_end: [u8; 7],
}
impl FMovieSceneDynamicBindingResolveResult {}
#[repr(C, align(4))]
pub struct FMovieSceneTimecodeSource {
    pub timecode: crate::bindings::core_u_object::FTimecode,
}
impl FMovieSceneTimecodeSource {}
#[repr(C, align(8))]
pub struct FMovieSceneSequencePlaybackParams {
    pub frame: crate::bindings::core_u_object::FFrameTime,
    pub time: f32,
    pub marked_frame: FString,
    pub timecode: crate::bindings::core_u_object::FTimecode,
    pub position_type: EMovieScenePositionType,
    pub update_method: EUpdatePositionMethod,
    pub b_has_jumped: bool,
    __padding_end: [u8; 5],
}
impl FMovieSceneSequencePlaybackParams {}
#[repr(C, align(1))]
pub struct FMovieSceneSequencePlayToParams {
    pub b_exclusive: bool,
}
impl FMovieSceneSequencePlayToParams {}
#[repr(C, align(8))]
pub struct FMovieSceneSectionTimingParametersSeconds {
    pub play_rate: FMovieSceneTimeWarpVariant,
    pub inner_start_offset: f32,
    pub inner_end_offset: f32,
    pub first_loop_start_offset: f32,
    pub flags_28: u8,
    __padding_end: [u8; 3],
}
impl FMovieSceneSectionTimingParametersSeconds {}
#[repr(C, align(8))]
pub struct FMovieSceneSectionTimingParametersFrames {
    pub play_rate: FMovieSceneTimeWarpVariant,
    pub inner_start_offset: crate::bindings::core_u_object::FFrameNumber,
    pub inner_end_offset: crate::bindings::core_u_object::FFrameNumber,
    pub first_loop_start_offset: crate::bindings::core_u_object::FFrameNumber,
    pub flags_28: u8,
    __padding_end: [u8; 3],
}
impl FMovieSceneSectionTimingParametersFrames {}
#[repr(C, align(8))]
pub struct FTestMovieSceneStruct {
    pub first: f32,
    pub second: f32,
    pub enum_: ETestMovieSceneEnum,
    pub vector: crate::bindings::core_u_object::FVector,
    pub multiple_integers: TArray<i32>,
    pub multiple_vectors: TArray<crate::bindings::core_u_object::FVector>,
}
impl FTestMovieSceneStruct {}
#[repr(C, align(4))]
pub struct FTestMovieSceneStruct2 {
    pub first: f32,
    pub second: f32,
}
impl FTestMovieSceneStruct2 {}
#[repr(C, align(8))]
pub struct UMovieSceneEntitySystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneEntitySystem {}
#[repr(C, align(8))]
pub struct UMovieSceneSignedObject {
    __padding_end: [u8; 112],
}
impl UMovieSceneSignedObject {}
#[repr(C, align(8))]
pub struct UMovieSceneDecorationContainerObject {
    __padding_end: [u8; 128],
}
impl UMovieSceneDecorationContainerObject {}
#[repr(C, align(8))]
pub struct UMovieSceneSection {
    __padding_end: [u8; 360],
}
impl UMovieSceneSection {}
#[repr(C, align(8))]
pub struct UMovieSceneTrack {
    __padding_end: [u8; 352],
}
impl UMovieSceneTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneNameableTrack {
    __padding_end: [u8; 384],
}
impl UMovieSceneNameableTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneSequence {
    __padding_end: [u8; 128],
}
impl UMovieSceneSequence {}
#[repr(C, align(8))]
pub struct UMovieSceneCustomBinding {
    __padding_end: [u8; 48],
}
impl UMovieSceneCustomBinding {}
#[repr(C, align(8))]
pub struct UMovieSceneReplaceableBindingBase {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub preview_spawnable: UPtr<UMovieSceneSpawnableBindingBase>,
}
impl UMovieSceneReplaceableBindingBase {}
#[repr(C, align(8))]
pub struct UMovieSceneSpawnableBindingBase {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub spawn_ownership: ESpawnOwnership,
    pub b_continuously_respawn: bool,
    __padding_end: [u8; 6],
}
impl UMovieSceneSpawnableBindingBase {}
#[repr(C, align(8))]
pub struct UMovieSceneChannelOverrideContainer {
    __padding_end: [u8; 112],
}
impl UMovieSceneChannelOverrideContainer {}
#[repr(C, align(8))]
pub struct UMovieSceneCondition {
    #[doc(hidden)]
    __padding_112: [u8; 112],
    pub b_editor_force_true: bool,
    pub b_invert: bool,
    __padding_end: [u8; 6],
}
impl UMovieSceneCondition {}
#[repr(C, align(8))]
pub struct UMovieSceneEntityInstantiatorSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneEntityInstantiatorSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneSubSection {
    #[doc(hidden)]
    __padding_368: [u8; 368],
    pub parameters: FMovieSceneSectionParameters,
    __padding_end: [u8; 2000],
}
impl UMovieSceneSubSection {}
#[repr(C, align(8))]
pub struct UMovieSceneBoolSection {
    __padding_end: [u8; 664],
}
impl UMovieSceneBoolSection {}
#[repr(C, align(8))]
pub struct UMovieSceneBlenderSystem {
    __padding_end: [u8; 120],
}
impl UMovieSceneBlenderSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneTrackInstance {
    __padding_end: [u8; 88],
}
impl UMovieSceneTrackInstance {}
#[repr(C, align(8))]
pub struct UMovieSceneSubTrack {
    __padding_end: [u8; 416],
}
impl UMovieSceneSubTrack {}
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
#[repr(C, align(8))]
pub struct UMovieSceneNumericVariantGetter {
    __padding_end: [u8; 120],
}
impl UMovieSceneNumericVariantGetter {}
pub struct UMovieSceneSequenceTickManagerClient {}
pub struct IMovieSceneSequenceTickManagerClient {}
#[repr(C, align(8))]
pub struct UMovieSceneSectionChannelOverrideRegistry {
    __padding_end: [u8; 128],
}
impl UMovieSceneSectionChannelOverrideRegistry {}
pub struct UMovieSceneTrackTemplateProducer {}
pub struct IMovieSceneTrackTemplateProducer {}
#[repr(C, align(8))]
pub struct UMovieSceneCompiledData {
    __padding_end: [u8; 1080],
}
impl UMovieSceneCompiledData {}
#[repr(C, align(8))]
pub struct UMovieSceneCompiledDataManager {
    __padding_end: [u8; 568],
}
impl UMovieSceneCompiledDataManager {}
#[repr(C, align(8))]
pub struct UMovieSceneGroupCondition {
    #[doc(hidden)]
    __padding_120: [u8; 120],
    pub operator: EMovieSceneGroupConditionOperator,
    pub sub_conditions: TArray<FMovieSceneConditionContainer>,
}
impl UMovieSceneGroupCondition {}
#[repr(C, align(8))]
pub struct UMovieSceneLanguagePreviewDecoration {
    __padding_end: [u8; 64],
}
impl UMovieSceneLanguagePreviewDecoration {}
#[repr(C, align(8))]
pub struct UMovieSceneMuteSoloDecoration {
    __padding_end: [u8; 64],
}
impl UMovieSceneMuteSoloDecoration {}
pub struct UMovieSceneScalingDriver {}
pub struct IMovieSceneScalingDriver {}
#[repr(C, align(8))]
pub struct UMovieSceneTimeWarpGetter {
    __padding_end: [u8; 136],
}
impl UMovieSceneTimeWarpGetter {}
#[repr(C, align(8))]
pub struct UMovieScenePlayRateCurve {
    __padding_end: [u8; 496],
}
impl UMovieScenePlayRateCurve {}
#[repr(C, align(8))]
pub struct UMovieSceneScalingAnchors {
    __padding_end: [u8; 784],
}
impl UMovieSceneScalingAnchors {}
#[repr(C, align(8))]
pub struct UMovieSceneSectionAnchorsDecoration {
    __padding_end: [u8; 88],
}
impl UMovieSceneSectionAnchorsDecoration {}
pub struct UMovieSceneTimeWarpSource {}
pub struct IMovieSceneTimeWarpSource {}
#[repr(C, align(8))]
pub struct UMovieSceneTimeWarpDecoration {
    __padding_end: [u8; 72],
}
impl UMovieSceneTimeWarpDecoration {}
#[repr(C, align(8))]
pub struct UMovieSceneTrackRowDecoration {
    __padding_end: [u8; 136],
}
impl UMovieSceneTrackRowDecoration {}
pub struct UMovieSceneEntityProvider {}
pub struct IMovieSceneEntityProvider {}
#[repr(C, align(8))]
pub struct UMovieSceneBindingLifetimeSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneBindingLifetimeSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneGenericBoundObjectInstantiator {
    __padding_end: [u8; 80],
}
impl UMovieSceneGenericBoundObjectInstantiator {}
#[repr(C, align(8))]
pub struct UMovieSceneBoundSceneComponentInstantiator {
    __padding_end: [u8; 80],
}
impl UMovieSceneBoundSceneComponentInstantiator {}
pub struct UMovieSceneValueDecomposer {}
pub struct IMovieSceneValueDecomposer {}
#[repr(C, align(8))]
pub struct UMovieSceneEntityGroupingSystem {
    __padding_end: [u8; 360],
}
impl UMovieSceneEntityGroupingSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneEntitySystemLinker {
    __padding_end: [u8; 1952],
}
impl UMovieSceneEntitySystemLinker {}
#[repr(C, align(8))]
pub struct UMovieSceneEvalTimeSystem {
    __padding_end: [u8; 480],
}
impl UMovieSceneEvalTimeSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneEvaluationHookSystem {
    __padding_end: [u8; 160],
}
impl UMovieSceneEvaluationHookSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneInitialValueSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneInitialValueSystem {}
pub struct UMovieScenePreAnimatedStateSystemInterface {}
pub struct IMovieScenePreAnimatedStateSystemInterface {}
#[repr(C, align(8))]
pub struct UMovieSceneCachePreAnimatedStateSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneCachePreAnimatedStateSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneRestorePreAnimatedStateSystem {
    __padding_end: [u8; 96],
}
impl UMovieSceneRestorePreAnimatedStateSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneRootInstantiatorSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneRootInstantiatorSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneSpawnablesSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneSpawnablesSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneTrackInstanceInstantiator {
    __padding_end: [u8; 256],
}
impl UMovieSceneTrackInstanceInstantiator {}
#[repr(C, align(8))]
pub struct UMovieSceneTrackInstanceSystem {
    __padding_end: [u8; 88],
}
impl UMovieSceneTrackInstanceSystem {}
pub struct UMovieSceneCustomClockSource {}
pub struct IMovieSceneCustomClockSource {}
pub struct UMovieSceneEvaluationHook {}
pub struct IMovieSceneEvaluationHook {}
#[repr(C, align(8))]
pub struct UMovieSceneBuiltInEasingFunction {
    __padding_end: [u8; 64],
}
impl UMovieSceneBuiltInEasingFunction {}
#[repr(C, align(8))]
pub struct UMovieSceneEasingExternalCurve {
    __padding_end: [u8; 64],
}
impl UMovieSceneEasingExternalCurve {}
pub struct UNodeAndChannelMappings {}
pub struct INodeAndChannelMappings {}
#[repr(C, align(8))]
pub struct UMovieSceneShotMetaData {
    __padding_end: [u8; 72],
}
impl UMovieSceneShotMetaData {}
#[repr(C, align(8))]
pub struct UMovieSceneNodeGroup {
    __padding_end: [u8; 120],
}
impl UMovieSceneNodeGroup {}
#[repr(C, align(8))]
pub struct UMovieSceneNodeGroupCollection {
    __padding_end: [u8; 104],
}
impl UMovieSceneNodeGroupCollection {}
#[repr(C, align(8))]
pub struct UMovieScene {
    __padding_end: [u8; 1304],
}
impl UMovieScene {}
#[repr(C, align(8))]
pub struct UMovieSceneBindingOverrides {
    __padding_end: [u8; 152],
}
impl UMovieSceneBindingOverrides {}
#[repr(C, align(8))]
pub struct UMovieSceneClock {
    __padding_end: [u8; 112],
}
impl UMovieSceneClock {}
#[repr(C, align(8))]
pub struct UMovieSceneExternalClock {
    __padding_end: [u8; 152],
}
impl UMovieSceneExternalClock {}
#[repr(C, align(8))]
pub struct UBuiltInDynamicBindingResolverLibrary {
    __padding_end: [u8; 48],
}
impl UBuiltInDynamicBindingResolverLibrary {}
#[repr(C, align(8))]
pub struct UMovieSceneFolder {
    __padding_end: [u8; 248],
}
impl UMovieSceneFolder {}
#[repr(C, align(8))]
pub struct UMovieSceneMetaData {
    __padding_end: [u8; 96],
}
impl UMovieSceneMetaData {}
#[repr(C, align(8))]
pub struct UMovieSceneSequencePlayer {
    __padding_end: [u8; 1216],
}
impl UMovieSceneSequencePlayer {}
#[repr(C, align(8))]
pub struct UMovieSceneSequenceTickManager {
    __padding_end: [u8; 160],
}
impl UMovieSceneSequenceTickManager {}
#[repr(C, align(8))]
pub struct UMovieSceneBindingLifetimeSection {
    __padding_end: [u8; 368],
}
impl UMovieSceneBindingLifetimeSection {}
#[repr(C, align(8))]
pub struct UMovieSceneHookSection {
    __padding_end: [u8; 384],
}
impl UMovieSceneHookSection {}
#[repr(C, align(8))]
pub struct UMovieSceneSpawnSection {
    __padding_end: [u8; 672],
}
impl UMovieSceneSpawnSection {}
#[repr(C, align(8))]
pub struct UMovieSceneTimeWarpSection {
    __padding_end: [u8; 376],
}
impl UMovieSceneTimeWarpSection {}
#[repr(C, align(8))]
pub struct UTestMovieSceneTrack {
    __padding_end: [u8; 384],
}
impl UTestMovieSceneTrack {}
#[repr(C, align(8))]
pub struct UTestMovieSceneSection {
    __padding_end: [u8; 360],
}
impl UTestMovieSceneSection {}
#[repr(C, align(8))]
pub struct UTestMovieSceneSequence {
    __padding_end: [u8; 136],
}
impl UTestMovieSceneSequence {}
#[repr(C, align(8))]
pub struct UTestMovieSceneSubTrack {
    __padding_end: [u8; 432],
}
impl UTestMovieSceneSubTrack {}
#[repr(C, align(8))]
pub struct UTestMovieSceneSubSection {
    __padding_end: [u8; 2424],
}
impl UTestMovieSceneSubSection {}
#[repr(C, align(8))]
pub struct UTestMovieSceneEvalHookTrack {
    __padding_end: [u8; 368],
}
impl UTestMovieSceneEvalHookTrack {}
#[repr(C, align(8))]
pub struct UTestMovieSceneEvalHookSection {
    __padding_end: [u8; 408],
}
impl UTestMovieSceneEvalHookSection {}
#[repr(C, align(8))]
pub struct UTestMovieSceneObject {
    __padding_end: [u8; 48],
}
impl UTestMovieSceneObject {}
#[repr(C, align(8))]
pub struct ATestMovieSceneArrayPropertiesActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub test_bool: bool,
    pub test_enum: ETestMovieSceneEnum,
    pub test_int32: i32,
    pub test_object: UPtr<UTestMovieSceneObject>,
    pub test_vector: crate::bindings::core_u_object::FVector,
    pub multiple_floats: TArray<f32>,
    pub single_struct: FTestMovieSceneStruct,
    pub multiple_structs: TArray<FTestMovieSceneStruct>,
    pub single_instanced_struct: crate::bindings::core_u_object::FInstancedStruct,
    pub multiple_instanced_structs: TArray<
        crate::bindings::core_u_object::FInstancedStruct,
    >,
    pub test_setter_float: f32,
    __padding_end: [u8; 4],
}
impl ATestMovieSceneArrayPropertiesActor {}
#[repr(C, align(8))]
pub struct UMovieSceneBindingLifetimeTrack {
    __padding_end: [u8; 376],
}
impl UMovieSceneBindingLifetimeTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneSpawnTrack {
    __padding_end: [u8; 384],
}
impl UMovieSceneSpawnTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneTimeWarpTrack {
    __padding_end: [u8; 384],
}
impl UMovieSceneTimeWarpTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneTimeWarpCurve {
    __padding_end: [u8; 464],
}
impl UMovieSceneTimeWarpCurve {}
#[repr(transparent)]
pub struct FMovieSceneSequencePlayer_OnPlay {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMovieSceneSequencePlayer_OnPlayReverse {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMovieSceneSequencePlayer_OnStop {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMovieSceneSequencePlayer_OnPause {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMovieSceneSequencePlayer_OnFinished {
    _opague: u8,
}
#[repr(transparent)]
pub struct EMovieSceneCompletionMode(pub u8);
impl EMovieSceneCompletionMode {
    pub const KEEP_STATE: EMovieSceneCompletionMode = EMovieSceneCompletionMode(0);
    pub const RESTORE_STATE: EMovieSceneCompletionMode = EMovieSceneCompletionMode(1);
    pub const PROJECT_DEFAULT: EMovieSceneCompletionMode = EMovieSceneCompletionMode(2);
}
#[repr(transparent)]
pub struct EMovieSceneObjectBindingSpace(pub u8);
impl EMovieSceneObjectBindingSpace {
    pub const LOCAL: EMovieSceneObjectBindingSpace = EMovieSceneObjectBindingSpace(0);
    pub const ROOT: EMovieSceneObjectBindingSpace = EMovieSceneObjectBindingSpace(1);
    pub const UNUSED: EMovieSceneObjectBindingSpace = EMovieSceneObjectBindingSpace(2);
}
#[repr(transparent)]
pub struct EMovieSceneSubSectionFlags(pub u8);
impl EMovieSceneSubSectionFlags {
    pub const NONE: EMovieSceneSubSectionFlags = EMovieSceneSubSectionFlags(0);
    pub const OVERRIDE_KEEP_STATE: EMovieSceneSubSectionFlags = EMovieSceneSubSectionFlags(
        1,
    );
    pub const OVERRIDE_RESTORE_STATE: EMovieSceneSubSectionFlags = EMovieSceneSubSectionFlags(
        2,
    );
    pub const IGNORE_HIERARCHICAL_BIAS: EMovieSceneSubSectionFlags = EMovieSceneSubSectionFlags(
        4,
    );
    pub const BLEND_HIERARCHICAL_BIAS: EMovieSceneSubSectionFlags = EMovieSceneSubSectionFlags(
        8,
    );
    pub const ANY_RESTORE_STATE_OVERRIDE: EMovieSceneSubSectionFlags = EMovieSceneSubSectionFlags(
        3,
    );
}
#[repr(transparent)]
pub struct ESectionEvaluationFlags(pub u8);
impl ESectionEvaluationFlags {
    pub const NONE: ESectionEvaluationFlags = ESectionEvaluationFlags(0);
    pub const PRE_ROLL: ESectionEvaluationFlags = ESectionEvaluationFlags(1);
    pub const POST_ROLL: ESectionEvaluationFlags = ESectionEvaluationFlags(2);
    pub const FORCE_KEEP_STATE: ESectionEvaluationFlags = ESectionEvaluationFlags(4);
    pub const FORCE_RESTORE_STATE: ESectionEvaluationFlags = ESectionEvaluationFlags(8);
}
#[repr(transparent)]
pub struct EMovieSceneCompletionModeOverride(pub u8);
impl EMovieSceneCompletionModeOverride {
    pub const NONE: EMovieSceneCompletionModeOverride = EMovieSceneCompletionModeOverride(
        0,
    );
    pub const FORCE_KEEP_STATE: EMovieSceneCompletionModeOverride = EMovieSceneCompletionModeOverride(
        1,
    );
    pub const FORCE_RESTORE_STATE: EMovieSceneCompletionModeOverride = EMovieSceneCompletionModeOverride(
        2,
    );
}
#[repr(transparent)]
pub struct ENavigationToolItemFlags(pub u8);
impl ENavigationToolItemFlags {
    pub const NONE: ENavigationToolItemFlags = ENavigationToolItemFlags(0);
    pub const IGNORE_PENDING_KILL: ENavigationToolItemFlags = ENavigationToolItemFlags(
        1,
    );
    pub const PENDING_REMOVAL: ENavigationToolItemFlags = ENavigationToolItemFlags(2);
    pub const EXPANDED: ENavigationToolItemFlags = ENavigationToolItemFlags(4);
}
#[repr(transparent)]
pub struct EMovieSceneBlendType(pub u8);
impl EMovieSceneBlendType {
    pub const INVALID: EMovieSceneBlendType = EMovieSceneBlendType(0);
    pub const ABSOLUTE: EMovieSceneBlendType = EMovieSceneBlendType(1);
    pub const ADDITIVE: EMovieSceneBlendType = EMovieSceneBlendType(2);
    pub const RELATIVE: EMovieSceneBlendType = EMovieSceneBlendType(4);
    pub const ADDITIVE_FROM_BASE: EMovieSceneBlendType = EMovieSceneBlendType(8);
    pub const OVERRIDE: EMovieSceneBlendType = EMovieSceneBlendType(16);
}
#[repr(transparent)]
pub struct EEvaluationMethod(pub u8);
impl EEvaluationMethod {
    pub const STATIC: EEvaluationMethod = EEvaluationMethod(0);
    pub const SWEPT: EEvaluationMethod = EEvaluationMethod(1);
}
#[repr(transparent)]
pub struct EMovieSceneBreadcrumbMode(pub u8);
impl EMovieSceneBreadcrumbMode {
    pub const SPARSE: EMovieSceneBreadcrumbMode = EMovieSceneBreadcrumbMode(0);
    pub const DENSE: EMovieSceneBreadcrumbMode = EMovieSceneBreadcrumbMode(1);
}
#[repr(transparent)]
pub struct EMovieSceneServerClientMask(pub u8);
impl EMovieSceneServerClientMask {
    pub const NONE: EMovieSceneServerClientMask = EMovieSceneServerClientMask(0);
    pub const SERVER: EMovieSceneServerClientMask = EMovieSceneServerClientMask(1);
    pub const CLIENT: EMovieSceneServerClientMask = EMovieSceneServerClientMask(2);
    pub const ALL: EMovieSceneServerClientMask = EMovieSceneServerClientMask(3);
}
#[repr(transparent)]
pub struct EMovieScenePlayerStatus(pub u8);
impl EMovieScenePlayerStatus {
    pub const STOPPED: EMovieScenePlayerStatus = EMovieScenePlayerStatus(0);
    pub const PLAYING: EMovieScenePlayerStatus = EMovieScenePlayerStatus(1);
    pub const SCRUBBING: EMovieScenePlayerStatus = EMovieScenePlayerStatus(2);
    pub const JUMPING: EMovieScenePlayerStatus = EMovieScenePlayerStatus(3);
    pub const STEPPING: EMovieScenePlayerStatus = EMovieScenePlayerStatus(4);
    pub const PAUSED: EMovieScenePlayerStatus = EMovieScenePlayerStatus(5);
    pub const MAX: EMovieScenePlayerStatus = EMovieScenePlayerStatus(6);
}
#[repr(transparent)]
pub struct EMovieScenePositionType(pub u8);
impl EMovieScenePositionType {
    pub const FRAME: EMovieScenePositionType = EMovieScenePositionType(0);
    pub const TIME: EMovieScenePositionType = EMovieScenePositionType(1);
    pub const MARKED_FRAME: EMovieScenePositionType = EMovieScenePositionType(2);
    pub const TIMECODE: EMovieScenePositionType = EMovieScenePositionType(3);
}
#[repr(transparent)]
pub struct EUpdatePositionMethod(pub u8);
impl EUpdatePositionMethod {
    pub const PLAY: EUpdatePositionMethod = EUpdatePositionMethod(0);
    pub const JUMP: EUpdatePositionMethod = EUpdatePositionMethod(1);
    pub const SCRUB: EUpdatePositionMethod = EUpdatePositionMethod(2);
}
#[repr(transparent)]
pub struct ESpawnOwnership(pub u8);
impl ESpawnOwnership {
    pub const INNER_SEQUENCE: ESpawnOwnership = ESpawnOwnership(0);
    pub const ROOT_SEQUENCE: ESpawnOwnership = ESpawnOwnership(1);
    pub const EXTERNAL: ESpawnOwnership = ESpawnOwnership(2);
}
#[repr(transparent)]
pub struct ETestMovieSceneEnum(pub u8);
impl ETestMovieSceneEnum {
    pub const ONE: ETestMovieSceneEnum = ETestMovieSceneEnum(0);
    pub const TWO: ETestMovieSceneEnum = ETestMovieSceneEnum(1);
    pub const THREE: ETestMovieSceneEnum = ETestMovieSceneEnum(2);
}
#[repr(transparent)]
pub struct EUpdateClockSource(pub u8);
impl EUpdateClockSource {
    pub const TICK: EUpdateClockSource = EUpdateClockSource(0);
    pub const PLATFORM: EUpdateClockSource = EUpdateClockSource(1);
    pub const AUDIO: EUpdateClockSource = EUpdateClockSource(2);
    pub const RELATIVE_TIMECODE: EUpdateClockSource = EUpdateClockSource(3);
    pub const TIMECODE: EUpdateClockSource = EUpdateClockSource(4);
    pub const PLAY_EVERY_FRAME: EUpdateClockSource = EUpdateClockSource(5);
    pub const CUSTOM: EUpdateClockSource = EUpdateClockSource(6);
}
#[repr(transparent)]
pub struct EMovieSceneTimeUnit(pub u8);
impl EMovieSceneTimeUnit {
    pub const DISPLAY_RATE: EMovieSceneTimeUnit = EMovieSceneTimeUnit(0);
    pub const TICK_RESOLUTION: EMovieSceneTimeUnit = EMovieSceneTimeUnit(1);
}
#[repr(transparent)]
pub struct EMovieSceneConditionCheckFrequency(pub u8);
impl EMovieSceneConditionCheckFrequency {
    pub const ONCE: EMovieSceneConditionCheckFrequency = EMovieSceneConditionCheckFrequency(
        0,
    );
    pub const ON_TICK: EMovieSceneConditionCheckFrequency = EMovieSceneConditionCheckFrequency(
        1,
    );
}
#[repr(transparent)]
pub struct EMovieSceneConditionScope(pub u8);
impl EMovieSceneConditionScope {
    pub const GLOBAL: EMovieSceneConditionScope = EMovieSceneConditionScope(0);
    pub const BINDING: EMovieSceneConditionScope = EMovieSceneConditionScope(1);
    pub const OWNER_OBJECT: EMovieSceneConditionScope = EMovieSceneConditionScope(2);
}
#[repr(transparent)]
pub struct EMovieSceneKeyInterpolation(pub u8);
impl EMovieSceneKeyInterpolation {
    pub const AUTO: EMovieSceneKeyInterpolation = EMovieSceneKeyInterpolation(0);
    pub const USER: EMovieSceneKeyInterpolation = EMovieSceneKeyInterpolation(1);
    pub const BREAK: EMovieSceneKeyInterpolation = EMovieSceneKeyInterpolation(2);
    pub const LINEAR: EMovieSceneKeyInterpolation = EMovieSceneKeyInterpolation(3);
    pub const CONSTANT: EMovieSceneKeyInterpolation = EMovieSceneKeyInterpolation(4);
    pub const SMART_AUTO: EMovieSceneKeyInterpolation = EMovieSceneKeyInterpolation(5);
}
#[repr(transparent)]
pub struct EMovieSceneEvaluationType(pub u8);
impl EMovieSceneEvaluationType {
    pub const FRAME_LOCKED: EMovieSceneEvaluationType = EMovieSceneEvaluationType(0);
    pub const WITH_SUB_FRAMES: EMovieSceneEvaluationType = EMovieSceneEvaluationType(1);
}
#[repr(transparent)]
pub struct EMovieSceneSequenceFlags(pub u8);
impl EMovieSceneSequenceFlags {
    pub const NONE: EMovieSceneSequenceFlags = EMovieSceneSequenceFlags(0);
    pub const VOLATILE: EMovieSceneSequenceFlags = EMovieSceneSequenceFlags(1);
    pub const BLOCKING_EVALUATION: EMovieSceneSequenceFlags = EMovieSceneSequenceFlags(
        2,
    );
    pub const DYNAMIC_WEIGHTING: EMovieSceneSequenceFlags = EMovieSceneSequenceFlags(4);
    pub const LOOP_CUTS: EMovieSceneSequenceFlags = EMovieSceneSequenceFlags(8);
    pub const INHERITED_FLAGS: EMovieSceneSequenceFlags = EMovieSceneSequenceFlags(1);
}
#[repr(transparent)]
pub struct EMovieSceneSequenceCompilerMask(pub u8);
impl EMovieSceneSequenceCompilerMask {
    pub const HIERARCHY: EMovieSceneSequenceCompilerMask = EMovieSceneSequenceCompilerMask(
        1,
    );
    pub const EVALUATION_TEMPLATE: EMovieSceneSequenceCompilerMask = EMovieSceneSequenceCompilerMask(
        2,
    );
    pub const EVALUATION_TEMPLATE_FIELD: EMovieSceneSequenceCompilerMask = EMovieSceneSequenceCompilerMask(
        4,
    );
    pub const ENTITY_COMPONENT_FIELD: EMovieSceneSequenceCompilerMask = EMovieSceneSequenceCompilerMask(
        8,
    );
    pub const NONE: EMovieSceneSequenceCompilerMask = EMovieSceneSequenceCompilerMask(0);
}
#[repr(transparent)]
pub struct EMovieSceneGroupConditionOperator(pub u8);
impl EMovieSceneGroupConditionOperator {
    pub const AND: EMovieSceneGroupConditionOperator = EMovieSceneGroupConditionOperator(
        0,
    );
    pub const OR: EMovieSceneGroupConditionOperator = EMovieSceneGroupConditionOperator(
        1,
    );
    pub const XOR: EMovieSceneGroupConditionOperator = EMovieSceneGroupConditionOperator(
        2,
    );
}
#[repr(transparent)]
pub struct EMovieSceneBuiltInEasing(pub u8);
impl EMovieSceneBuiltInEasing {
    pub const LINEAR: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(0);
    pub const SIN_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(1);
    pub const SIN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(2);
    pub const SIN_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(3);
    pub const QUAD_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(4);
    pub const QUAD_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(5);
    pub const QUAD_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(6);
    pub const CUBIC: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(7);
    pub const CUBIC_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(8);
    pub const CUBIC_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(9);
    pub const CUBIC_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(10);
    pub const HERMITE_CUBIC_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(
        11,
    );
    pub const QUART_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(12);
    pub const QUART_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(13);
    pub const QUART_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(14);
    pub const QUINT_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(15);
    pub const QUINT_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(16);
    pub const QUINT_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(17);
    pub const EXPO_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(18);
    pub const EXPO_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(19);
    pub const EXPO_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(20);
    pub const CIRC_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(21);
    pub const CIRC_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(22);
    pub const CIRC_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(23);
    pub const CUSTOM: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(24);
}
