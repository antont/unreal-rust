#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FMovieSceneCameraShakeSourceTrigger {
    pub shake_class: TSubclassOf<crate::bindings::engine::UCameraShakeBase>,
    pub play_scale: f32,
    pub play_space: crate::bindings::engine::ECameraShakePlaySpace,
    pub user_defined_play_space: crate::bindings::core_u_object::FRotator,
}
impl FMovieSceneCameraShakeSourceTrigger {}
#[repr(C, align(8))]
pub struct FMovieSceneEventPayloadVariable {
    __padding_end: [u8; 56],
}
impl FMovieSceneEventPayloadVariable {}
#[repr(C, align(8))]
pub struct FMovieSceneEventPtrs {
    __padding_end: [u8; 56],
}
impl FMovieSceneEventPtrs {}
#[repr(C, align(8))]
pub struct FMovieSceneEvent {
    __padding_end: [u8; 208],
}
impl FMovieSceneEvent {}
#[repr(C, align(8))]
pub struct FMovieSceneDirectorBlueprintConditionPayloadVariable {
    __padding_end: [u8; 56],
}
impl FMovieSceneDirectorBlueprintConditionPayloadVariable {}
#[repr(C, align(8))]
pub struct FEventPayload {
    pub event_name: FName,
    pub parameters: FMovieSceneEventParameters,
}
impl FEventPayload {}
#[repr(C, align(8))]
pub struct FMovieSceneEventParameters {
    __padding_end: [u8; 72],
}
impl FMovieSceneEventParameters {}
#[repr(C, align(8))]
pub struct FMovieSceneSkeletalAnimationParams {
    pub animation: UPtr<crate::bindings::engine::UAnimSequenceBase>,
    pub first_loop_start_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub start_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub end_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub play_rate: crate::bindings::movie_scene::FMovieSceneTimeWarpVariant,
    pub flags_40: u8,
    pub slot_name: FName,
    pub mirror_data_table: UPtr<crate::bindings::engine::UMirrorDataTable>,
    #[doc(hidden)]
    __padding_368: [u8; 304],
    pub b_skip_anim_notifiers: bool,
    pub b_force_custom_mode: bool,
    pub swap_root_bone: crate::bindings::anim_graph_runtime::ESwapRootBone,
    pub b_linear_playback_when_scaled: bool,
    __padding_end: [u8; 12],
}
impl FMovieSceneSkeletalAnimationParams {}
#[repr(C, align(4))]
pub struct FComponentMaterialInfo {
    pub material_slot_name: FName,
    pub material_slot_index: i32,
    pub material_type: EComponentMaterialType,
    __padding_end: [u8; 3],
}
impl FComponentMaterialInfo {}
#[repr(C, align(8))]
pub struct UMovieScenePropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieScenePropertySystem {}
#[repr(C, align(8))]
pub struct UMovieScenePropertyTrack {
    __padding_end: [u8; 480],
}
impl UMovieScenePropertyTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneMaterialTrack {
    __padding_end: [u8; 408],
}
impl UMovieSceneMaterialTrack {}
pub struct UMovieSceneTransformOrigin {}
pub struct IMovieSceneTransformOrigin {}
pub struct UMovieSceneConsoleVariableTrackInterface {}
pub struct IMovieSceneConsoleVariableTrackInterface {}
#[repr(C, align(8))]
pub struct UMovieSceneReplaceableActorBinding {
    __padding_end: [u8; 56],
}
impl UMovieSceneReplaceableActorBinding {}
#[repr(C, align(8))]
pub struct UMovieSceneReplaceableActorBinding_BPBase {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub binding_type_pretty_name: FText,
    pub binding_type_tooltip: FText,
    pub custom_binding_priority: i32,
    pub preview_spawnable_type: TSubclassOf<
        crate::bindings::movie_scene::UMovieSceneSpawnableBindingBase,
    >,
}
impl UMovieSceneReplaceableActorBinding_BPBase {}
#[repr(C, align(8))]
pub struct UMovieSceneReplaceableDirectorBlueprintBinding {
    #[doc(hidden)]
    __padding_224: [u8; 224],
    pub preview_spawnable_type: TSubclassOf<
        crate::bindings::movie_scene::UMovieSceneSpawnableBindingBase,
    >,
}
impl UMovieSceneReplaceableDirectorBlueprintBinding {}
#[repr(C, align(8))]
pub struct UMovieSceneSpawnableActorBindingBase {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub b_net_addressable_name: bool,
    pub level_name: FName,
}
impl UMovieSceneSpawnableActorBindingBase {}
#[repr(C, align(8))]
pub struct UMovieSceneSpawnableActorBinding {
    __padding_end: [u8; 80],
}
impl UMovieSceneSpawnableActorBinding {}
#[repr(C, align(8))]
pub struct UMovieSceneSpawnableDirectorBlueprintBinding {
    __padding_end: [u8; 224],
}
impl UMovieSceneSpawnableDirectorBlueprintBinding {}
#[repr(C, align(8))]
pub struct UMovieSceneDoublePerlinNoiseChannelContainer {
    __padding_end: [u8; 240],
}
impl UMovieSceneDoublePerlinNoiseChannelContainer {}
#[repr(C, align(8))]
pub struct UMovieSceneFloatPerlinNoiseChannelContainer {
    __padding_end: [u8; 240],
}
impl UMovieSceneFloatPerlinNoiseChannelContainer {}
#[repr(C, align(8))]
pub struct UMovieSceneDirectorBlueprintCondition {
    __padding_end: [u8; 296],
}
impl UMovieSceneDirectorBlueprintCondition {}
#[repr(C, align(8))]
pub struct UMovieScenePlatformCondition {
    __padding_end: [u8; 136],
}
impl UMovieScenePlatformCondition {}
#[repr(C, align(8))]
pub struct UMovieSceneScalabilityCondition {
    __padding_end: [u8; 128],
}
impl UMovieSceneScalabilityCondition {}
#[repr(C, align(8))]
pub struct UMovieSceneInterrogatedPropertyInstantiatorSystem {
    __padding_end: [u8; 536],
}
impl UMovieSceneInterrogatedPropertyInstantiatorSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneTracksSettings {
    __padding_end: [u8; 56],
}
impl UMovieSceneTracksSettings {}
#[repr(C, align(8))]
pub struct UMovieScene3DConstraintSection {
    __padding_end: [u8; 408],
}
impl UMovieScene3DConstraintSection {}
#[repr(C, align(8))]
pub struct UMovieScene3DAttachSection {
    #[doc(hidden)]
    __padding_416: [u8; 416],
    pub attach_socket_name: FName,
    pub attach_component_name: FName,
    #[doc(hidden)]
    __padding_496: [u8; 56],
    pub attachment_location_rule: crate::bindings::engine::EAttachmentRule,
    pub attachment_rotation_rule: crate::bindings::engine::EAttachmentRule,
    pub attachment_scale_rule: crate::bindings::engine::EAttachmentRule,
    pub detachment_location_rule: crate::bindings::engine::EDetachmentRule,
    pub detachment_rotation_rule: crate::bindings::engine::EDetachmentRule,
    pub detachment_scale_rule: crate::bindings::engine::EDetachmentRule,
    __padding_end: [u8; 2],
}
impl UMovieScene3DAttachSection {}
#[repr(C, align(8))]
pub struct UMovieScene3DPathSection {
    #[doc(hidden)]
    __padding_712: [u8; 712],
    pub front_axis_enum: MovieScene3DPathSection_Axis,
    pub up_axis_enum: MovieScene3DPathSection_Axis,
    #[doc(hidden)]
    __padding_716: [u8; 2],
    pub flags_716: u8,
    __padding_end: [u8; 3],
}
impl UMovieScene3DPathSection {}
#[repr(C, align(8))]
pub struct UMovieScene3DTransformSectionConstraints {
    __padding_end: [u8; 64],
}
impl UMovieScene3DTransformSectionConstraints {}
#[repr(C, align(8))]
pub struct UMovieScene3DTransformSection {
    __padding_end: [u8; 3576],
}
impl UMovieScene3DTransformSection {}
#[repr(C, align(8))]
pub struct UMovieSceneActorReferenceSection {
    __padding_end: [u8; 832],
}
impl UMovieSceneActorReferenceSection {}
#[repr(C, align(8))]
pub struct UMovieSceneAudioSection {
    #[doc(hidden)]
    __padding_376: [u8; 376],
    pub sound: UPtr<crate::bindings::engine::USoundBase>,
    __padding_end: [u8; 1456],
}
impl UMovieSceneAudioSection {}
#[repr(C, align(8))]
pub struct UMovieSceneBaseCacheSection {
    __padding_end: [u8; 376],
}
impl UMovieSceneBaseCacheSection {}
#[repr(C, align(8))]
pub struct UMovieSceneByteSection {
    __padding_end: [u8; 664],
}
impl UMovieSceneByteSection {}
#[repr(C, align(16))]
pub struct UMovieSceneCameraCutSection {
    __padding_end: [u8; 528],
}
impl UMovieSceneCameraCutSection {}
#[repr(C, align(8))]
pub struct UMovieSceneCameraShakeSection {
    __padding_end: [u8; 448],
}
impl UMovieSceneCameraShakeSection {}
#[repr(C, align(8))]
pub struct UMovieSceneCameraShakeSourceShakeSection {
    __padding_end: [u8; 408],
}
impl UMovieSceneCameraShakeSourceShakeSection {}
#[repr(C, align(8))]
pub struct UMovieSceneCameraShakeSourceTriggerSection {
    __padding_end: [u8; 640],
}
impl UMovieSceneCameraShakeSourceTriggerSection {}
#[repr(C, align(8))]
pub struct UMovieSceneCinematicShotSection {
    __padding_end: [u8; 2464],
}
impl UMovieSceneCinematicShotSection {}
#[repr(C, align(8))]
pub struct UMovieSceneColorSection {
    __padding_end: [u8; 1584],
}
impl UMovieSceneColorSection {}
#[repr(C, align(8))]
pub struct UMovieSceneComponentMaterialParameterSection {
    __padding_end: [u8; 400],
}
impl UMovieSceneComponentMaterialParameterSection {}
pub struct UMovieSceneConstrainedSection {}
pub struct IMovieSceneConstrainedSection {}
#[repr(C, align(8))]
pub struct UMovieSceneParameterSection {
    __padding_end: [u8; 464],
}
impl UMovieSceneParameterSection {}
#[repr(C, align(8))]
pub struct UMovieSceneCustomPrimitiveDataSection {
    __padding_end: [u8; 472],
}
impl UMovieSceneCustomPrimitiveDataSection {}
#[repr(C, align(8))]
pub struct UMovieSceneCVarSection {
    __padding_end: [u8; 464],
}
impl UMovieSceneCVarSection {}
#[repr(C, align(8))]
pub struct UMovieSceneDataLayerSection {
    __padding_end: [u8; 392],
}
impl UMovieSceneDataLayerSection {}
#[repr(C, align(8))]
pub struct UMovieSceneDoubleSection {
    __padding_end: [u8; 680],
}
impl UMovieSceneDoubleSection {}
#[repr(C, align(8))]
pub struct UMovieSceneEnumSection {
    __padding_end: [u8; 664],
}
impl UMovieSceneEnumSection {}
#[repr(C, align(8))]
pub struct UMovieSceneEventSectionBase {
    __padding_end: [u8; 368],
}
impl UMovieSceneEventSectionBase {}
#[repr(C, align(8))]
pub struct UMovieSceneEventRepeaterSection {
    __padding_end: [u8; 584],
}
impl UMovieSceneEventRepeaterSection {}
#[repr(C, align(8))]
pub struct UMovieSceneEventSection {
    __padding_end: [u8; 768],
}
impl UMovieSceneEventSection {}
#[repr(C, align(8))]
pub struct UMovieSceneEventTriggerSection {
    __padding_end: [u8; 648],
}
impl UMovieSceneEventTriggerSection {}
#[repr(C, align(8))]
pub struct UMovieSceneFadeSection {
    __padding_end: [u8; 696],
}
impl UMovieSceneFadeSection {}
#[repr(C, align(8))]
pub struct UMovieSceneFloatSection {
    __padding_end: [u8; 688],
}
impl UMovieSceneFloatSection {}
#[repr(C, align(8))]
pub struct UMovieSceneIntegerSection {
    __padding_end: [u8; 656],
}
impl UMovieSceneIntegerSection {}
#[repr(C, align(8))]
pub struct UMovieSceneLevelVisibilitySection {
    __padding_end: [u8; 392],
}
impl UMovieSceneLevelVisibilitySection {}
#[repr(C, align(8))]
pub struct UMovieSceneObjectPropertySection {
    __padding_end: [u8; 704],
}
impl UMovieSceneObjectPropertySection {}
pub struct UMovieSceneParameterSectionExtender {}
pub struct IMovieSceneParameterSectionExtender {}
#[repr(C, align(8))]
pub struct UMovieSceneParticleSection {
    __padding_end: [u8; 656],
}
impl UMovieSceneParticleSection {}
#[repr(C, align(8))]
pub struct UMovieScenePrimitiveMaterialSection {
    __padding_end: [u8; 704],
}
impl UMovieScenePrimitiveMaterialSection {}
#[repr(C, align(8))]
pub struct UMovieSceneRotatorSection {
    __padding_end: [u8; 1304],
}
impl UMovieSceneRotatorSection {}
#[repr(C, align(16))]
pub struct UMovieSceneSkeletalAnimationSection {
    #[doc(hidden)]
    __padding_368: [u8; 368],
    pub params: FMovieSceneSkeletalAnimationParams,
    #[doc(hidden)]
    __padding_808: [u8; 56],
    pub start_location_offset: crate::bindings::core_u_object::FVector,
    pub start_rotation_offset: crate::bindings::core_u_object::FRotator,
    #[doc(hidden)]
    __padding_872: [u8; 16],
    pub matched_location_offset: crate::bindings::core_u_object::FVector,
    pub matched_rotation_offset: crate::bindings::core_u_object::FRotator,
    pub flags_920: u8,
    __padding_end: [u8; 151],
}
impl UMovieSceneSkeletalAnimationSection {}
#[repr(C, align(8))]
pub struct UMovieSceneSlomoSection {
    __padding_end: [u8; 672],
}
impl UMovieSceneSlomoSection {}
#[repr(C, align(8))]
pub struct UMovieSceneStringSection {
    __padding_end: [u8; 664],
}
impl UMovieSceneStringSection {}
#[repr(C, align(8))]
pub struct UMovieSceneTextSection {
    __padding_end: [u8; 672],
}
impl UMovieSceneTextSection {}
#[repr(C, align(8))]
pub struct UMovieSceneFloatVectorSection {
    __padding_end: [u8; 1592],
}
impl UMovieSceneFloatVectorSection {}
#[repr(C, align(8))]
pub struct UMovieSceneDoubleVectorSection {
    __padding_end: [u8; 1624],
}
impl UMovieSceneDoubleVectorSection {}
#[repr(C, align(8))]
pub struct UMovieSceneVisibilitySection {
    __padding_end: [u8; 672],
}
impl UMovieSceneVisibilitySection {}
#[repr(C, align(8))]
pub struct UBoolChannelEvaluatorSystem {
    __padding_end: [u8; 80],
}
impl UBoolChannelEvaluatorSystem {}
#[repr(C, align(8))]
pub struct UByteChannelEvaluatorSystem {
    __padding_end: [u8; 80],
}
impl UByteChannelEvaluatorSystem {}
#[repr(C, align(8))]
pub struct UDoubleChannelEvaluatorSystem {
    __padding_end: [u8; 80],
}
impl UDoubleChannelEvaluatorSystem {}
#[repr(C, align(8))]
pub struct UDoublePerlinNoiseChannelEvaluatorSystem {
    __padding_end: [u8; 80],
}
impl UDoublePerlinNoiseChannelEvaluatorSystem {}
#[repr(C, align(8))]
pub struct UFloatChannelEvaluatorSystem {
    __padding_end: [u8; 80],
}
impl UFloatChannelEvaluatorSystem {}
#[repr(C, align(8))]
pub struct UFloatPerlinNoiseChannelEvaluatorSystem {
    __padding_end: [u8; 80],
}
impl UFloatPerlinNoiseChannelEvaluatorSystem {}
#[repr(C, align(8))]
pub struct UIntegerChannelEvaluatorSystem {
    __padding_end: [u8; 80],
}
impl UIntegerChannelEvaluatorSystem {}
#[repr(C, align(8))]
pub struct UMovieScene3DTransformPropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieScene3DTransformPropertySystem {}
#[repr(C, align(8))]
pub struct UMovieSceneAudioSystem {
    __padding_end: [u8; 256],
}
impl UMovieSceneAudioSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneBaseValueEvaluatorSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneBaseValueEvaluatorSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneBoolPropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneBoolPropertySystem {}
#[repr(C, align(8))]
pub struct UMovieSceneBytePropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneBytePropertySystem {}
#[repr(C, align(8))]
pub struct UMovieSceneCameraShakeInstantiatorSystem {
    __padding_end: [u8; 224],
}
impl UMovieSceneCameraShakeInstantiatorSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneCameraShakeEvaluatorSystem {
    __padding_end: [u8; 96],
}
impl UMovieSceneCameraShakeEvaluatorSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneColorPropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneColorPropertySystem {}
#[repr(C, align(8))]
pub struct UMovieSceneComponentAttachmentInvalidatorSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneComponentAttachmentInvalidatorSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneComponentAttachmentSystem {
    __padding_end: [u8; 480],
}
impl UMovieSceneComponentAttachmentSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneComponentMaterialSystem {
    __padding_end: [u8; 536],
}
impl UMovieSceneComponentMaterialSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneComponentMobilitySystem {
    __padding_end: [u8; 608],
}
impl UMovieSceneComponentMobilitySystem {}
#[repr(C, align(8))]
pub struct UMovieSceneComponentTransformSystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneComponentTransformSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneConstraintSystem {
    __padding_end: [u8; 96],
}
impl UMovieSceneConstraintSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneCustomPrimitiveDataSystem {
    __padding_end: [u8; 488],
}
impl UMovieSceneCustomPrimitiveDataSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneDataLayerSystem {
    __padding_end: [u8; 256],
}
impl UMovieSceneDataLayerSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneDeferredComponentMovementSystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneDeferredComponentMovementSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneDoublePropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneDoublePropertySystem {}
#[repr(C, align(8))]
pub struct UMovieSceneEnumPropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneEnumPropertySystem {}
#[repr(C, align(8))]
pub struct UMovieSceneEulerTransformPropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneEulerTransformPropertySystem {}
#[repr(C, align(8))]
pub struct UMovieSceneEventSystem {
    __padding_end: [u8; 160],
}
impl UMovieSceneEventSystem {}
#[repr(C, align(8))]
pub struct UMovieScenePreSpawnEventSystem {
    __padding_end: [u8; 160],
}
impl UMovieScenePreSpawnEventSystem {}
#[repr(C, align(8))]
pub struct UMovieScenePostSpawnEventSystem {
    __padding_end: [u8; 160],
}
impl UMovieScenePostSpawnEventSystem {}
#[repr(C, align(8))]
pub struct UMovieScenePostEvalEventSystem {
    __padding_end: [u8; 160],
}
impl UMovieScenePostEvalEventSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneFadeSystem {
    __padding_end: [u8; 96],
}
impl UMovieSceneFadeSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneFloatPropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneFloatPropertySystem {}
#[repr(C, align(8))]
pub struct UMovieSceneHierarchicalBiasSystem {
    __padding_end: [u8; 88],
}
impl UMovieSceneHierarchicalBiasSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneIntegerPropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneIntegerPropertySystem {}
#[repr(C, align(8))]
pub struct UMovieSceneLevelVisibilitySystem {
    __padding_end: [u8; 504],
}
impl UMovieSceneLevelVisibilitySystem {}
#[repr(C, align(8))]
pub struct UMovieSceneMaterialParameterCollectionSystem {
    __padding_end: [u8; 120],
}
impl UMovieSceneMaterialParameterCollectionSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneMaterialParameterInstantiatorSystem {
    __padding_end: [u8; 872],
}
impl UMovieSceneMaterialParameterInstantiatorSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneMaterialParameterEvaluationSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneMaterialParameterEvaluationSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneMotionVectorSimulationSystem {
    __padding_end: [u8; 168],
}
impl UMovieSceneMotionVectorSimulationSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneObjectPropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneObjectPropertySystem {}
#[repr(C, align(8))]
pub struct UMovieScenePiecewiseBoolBlenderSystem {
    __padding_end: [u8; 160],
}
impl UMovieScenePiecewiseBoolBlenderSystem {}
#[repr(C, align(8))]
pub struct UMovieScenePiecewiseByteBlenderSystem {
    __padding_end: [u8; 160],
}
impl UMovieScenePiecewiseByteBlenderSystem {}
#[repr(C, align(8))]
pub struct UMovieScenePiecewiseDoubleBlenderSystem {
    __padding_end: [u8; 368],
}
impl UMovieScenePiecewiseDoubleBlenderSystem {}
#[repr(C, align(8))]
pub struct UMovieScenePiecewiseEnumBlenderSystem {
    __padding_end: [u8; 160],
}
impl UMovieScenePiecewiseEnumBlenderSystem {}
#[repr(C, align(8))]
pub struct UMovieScenePiecewiseIntegerBlenderSystem {
    __padding_end: [u8; 192],
}
impl UMovieScenePiecewiseIntegerBlenderSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneAsyncAction_SequencePrediction {
    __padding_end: [u8; 152],
}
impl UMovieSceneAsyncAction_SequencePrediction {}
#[repr(C, align(8))]
pub struct UMovieScenePredictionSystem {
    __padding_end: [u8; 256],
}
impl UMovieScenePredictionSystem {}
#[repr(C, align(8))]
pub struct UMovieScenePropertyInstantiatorSystem {
    __padding_end: [u8; 504],
}
impl UMovieScenePropertyInstantiatorSystem {}
#[repr(C, align(8))]
pub struct UMovieScenePropertyNotificationSystem {
    __padding_end: [u8; 80],
}
impl UMovieScenePropertyNotificationSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneQuaternionBlenderSystem {
    __padding_end: [u8; 216],
}
impl UMovieSceneQuaternionBlenderSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneQuaternionInterpolationRotationSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneQuaternionInterpolationRotationSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneRotatorPropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneRotatorPropertySystem {}
#[repr(C, align(8))]
pub struct UMovieSceneSkeletalAnimationSystem {
    __padding_end: [u8; 248],
}
impl UMovieSceneSkeletalAnimationSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneSlomoSystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneSlomoSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneStringPropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneStringPropertySystem {}
#[repr(C, align(8))]
pub struct UMovieSceneTextPropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneTextPropertySystem {}
#[repr(C, align(8))]
pub struct UMovieSceneTransformOriginInstantiatorSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneTransformOriginInstantiatorSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneTransformOriginSystem {
    __padding_end: [u8; 360],
}
impl UMovieSceneTransformOriginSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneFloatVectorPropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneFloatVectorPropertySystem {}
#[repr(C, align(8))]
pub struct UMovieSceneDoubleVectorPropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneDoubleVectorPropertySystem {}
#[repr(C, align(8))]
pub struct UMovieSceneVisibilitySystem {
    __padding_end: [u8; 96],
}
impl UMovieSceneVisibilitySystem {}
#[repr(C, align(8))]
pub struct UObjectPathChannelEvaluatorSystem {
    __padding_end: [u8; 80],
}
impl UObjectPathChannelEvaluatorSystem {}
#[repr(C, align(8))]
pub struct UStringChannelEvaluatorSystem {
    __padding_end: [u8; 80],
}
impl UStringChannelEvaluatorSystem {}
#[repr(C, align(8))]
pub struct UTextChannelEvaluatorSystem {
    __padding_end: [u8; 80],
}
impl UTextChannelEvaluatorSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneHierarchicalEasingInstantiatorSystem {
    __padding_end: [u8; 232],
}
impl UMovieSceneHierarchicalEasingInstantiatorSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneHierarchicalEasingFinalizationSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneHierarchicalEasingFinalizationSystem {}
#[repr(C, align(8))]
pub struct UWeightAndEasingEvaluatorSystem {
    __padding_end: [u8; 104],
}
impl UWeightAndEasingEvaluatorSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneDecomposerTestObject {
    __padding_end: [u8; 56],
}
impl UMovieSceneDecomposerTestObject {}
#[repr(C, align(8))]
pub struct UMovieSceneFadeTrackTestLibrary {
    __padding_end: [u8; 48],
}
impl UMovieSceneFadeTrackTestLibrary {}
#[repr(C, align(8))]
pub struct UMovieScenePartialEvaluationTestObject {
    __padding_end: [u8; 80],
}
impl UMovieScenePartialEvaluationTestObject {}
#[repr(C, align(8))]
pub struct UMovieSceneTestSequence {
    __padding_end: [u8; 168],
}
impl UMovieSceneTestSequence {}
#[repr(C, align(8))]
pub struct UMovieSceneCameraCutTrackInstance {
    __padding_end: [u8; 136],
}
impl UMovieSceneCameraCutTrackInstance {}
#[repr(C, align(8))]
pub struct UMovieSceneCVarTrackInstance {
    __padding_end: [u8; 168],
}
impl UMovieSceneCVarTrackInstance {}
pub struct UMovieSceneSectionsToKey {}
pub struct IMovieSceneSectionsToKey {}
#[repr(C, align(8))]
pub struct UMovieScene3DConstraintTrack {
    __padding_end: [u8; 368],
}
impl UMovieScene3DConstraintTrack {}
#[repr(C, align(8))]
pub struct UMovieScene3DAttachTrack {
    __padding_end: [u8; 368],
}
impl UMovieScene3DAttachTrack {}
#[repr(C, align(8))]
pub struct UMovieScene3DPathTrack {
    __padding_end: [u8; 376],
}
impl UMovieScene3DPathTrack {}
#[repr(C, align(8))]
pub struct UMovieScene3DTransformTrack {
    __padding_end: [u8; 496],
}
impl UMovieScene3DTransformTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneActorReferenceTrack {
    __padding_end: [u8; 488],
}
impl UMovieSceneActorReferenceTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneAudioTrack {
    __padding_end: [u8; 408],
}
impl UMovieSceneAudioTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneBoolTrack {
    __padding_end: [u8; 496],
}
impl UMovieSceneBoolTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneByteTrack {
    __padding_end: [u8; 488],
}
impl UMovieSceneByteTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneCameraCutTrack {
    __padding_end: [u8; 416],
}
impl UMovieSceneCameraCutTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneCameraShakeSourceShakeTrack {
    __padding_end: [u8; 400],
}
impl UMovieSceneCameraShakeSourceShakeTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneCameraShakeSourceTriggerTrack {
    __padding_end: [u8; 368],
}
impl UMovieSceneCameraShakeSourceTriggerTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneCameraShakeTrack {
    __padding_end: [u8; 400],
}
impl UMovieSceneCameraShakeTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneCinematicShotTrack {
    __padding_end: [u8; 416],
}
impl UMovieSceneCinematicShotTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneColorTrack {
    __padding_end: [u8; 488],
}
impl UMovieSceneColorTrack {}
#[repr(C, align(16))]
pub struct UMovieSceneCommonAnimationTrack {
    __padding_end: [u8; 576],
}
impl UMovieSceneCommonAnimationTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneCustomPrimitiveDataTrack {
    __padding_end: [u8; 424],
}
impl UMovieSceneCustomPrimitiveDataTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneCVarTrack {
    __padding_end: [u8; 400],
}
impl UMovieSceneCVarTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneDataLayerTrack {
    __padding_end: [u8; 400],
}
impl UMovieSceneDataLayerTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneDoubleTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneDoubleTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneEnumTrack {
    __padding_end: [u8; 488],
}
impl UMovieSceneEnumTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneEulerTransformTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneEulerTransformTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneEventTrack {
    __padding_end: [u8; 424],
}
impl UMovieSceneEventTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneFloatTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneFloatTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneFadeTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneFadeTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneIntegerTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneIntegerTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneLevelVisibilityTrack {
    __padding_end: [u8; 400],
}
impl UMovieSceneLevelVisibilityTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneMaterialParameterCollectionTrack {
    __padding_end: [u8; 432],
}
impl UMovieSceneMaterialParameterCollectionTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneComponentMaterialTrack {
    __padding_end: [u8; 448],
}
impl UMovieSceneComponentMaterialTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneObjectPropertyTrack {
    __padding_end: [u8; 496],
}
impl UMovieSceneObjectPropertyTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneParticleParameterTrack {
    __padding_end: [u8; 408],
}
impl UMovieSceneParticleParameterTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneParticleTrack {
    __padding_end: [u8; 408],
}
impl UMovieSceneParticleTrack {}
#[repr(C, align(8))]
pub struct UMovieScenePrimitiveMaterialTrack {
    __padding_end: [u8; 504],
}
impl UMovieScenePrimitiveMaterialTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneRotatorTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneRotatorTrack {}
#[repr(C, align(16))]
pub struct UMovieSceneSkeletalAnimationTrack {
    #[doc(hidden)]
    __padding_569: [u8; 569],
    pub swap_root_bone: crate::bindings::anim_graph_runtime::ESwapRootBone,
    __padding_end: [u8; 6],
}
impl UMovieSceneSkeletalAnimationTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneSlomoTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneSlomoTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneStringTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneStringTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneTextTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneTextTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneTransformTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneTransformTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneFloatVectorTrack {
    __padding_end: [u8; 488],
}
impl UMovieSceneFloatVectorTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneDoubleVectorTrack {
    __padding_end: [u8; 488],
}
impl UMovieSceneDoubleVectorTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneVisibilityTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneVisibilityTrack {}
#[repr(transparent)]
pub struct FMovieSceneAudioSection_OnQueueSubtitles {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMovieSceneAudioSection_OnAudioFinished {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMovieSceneAudioSection_OnAudioPlaybackPercent {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMovieSceneAsyncAction_SequencePrediction_Result {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMovieSceneAsyncAction_SequencePrediction_Failure {
    _opague: u8,
}
#[repr(transparent)]
pub struct MovieScene3DPathSection_Axis(pub u8);
impl MovieScene3DPathSection_Axis {
    pub const X: MovieScene3DPathSection_Axis = MovieScene3DPathSection_Axis(0);
    pub const Y: MovieScene3DPathSection_Axis = MovieScene3DPathSection_Axis(1);
    pub const Z: MovieScene3DPathSection_Axis = MovieScene3DPathSection_Axis(2);
    pub const NEG_X: MovieScene3DPathSection_Axis = MovieScene3DPathSection_Axis(3);
    pub const NEG_Y: MovieScene3DPathSection_Axis = MovieScene3DPathSection_Axis(4);
    pub const NEG_Z: MovieScene3DPathSection_Axis = MovieScene3DPathSection_Axis(5);
}
#[repr(transparent)]
pub struct EComponentMaterialType(pub u8);
impl EComponentMaterialType {
    pub const EMPTY: EComponentMaterialType = EComponentMaterialType(0);
    pub const INDEXED_MATERIAL: EComponentMaterialType = EComponentMaterialType(1);
    pub const OVERLAY_MATERIAL: EComponentMaterialType = EComponentMaterialType(2);
    pub const DECAL_MATERIAL: EComponentMaterialType = EComponentMaterialType(3);
    pub const VOLUMETRIC_CLOUD_MATERIAL: EComponentMaterialType = EComponentMaterialType(
        4,
    );
}
#[repr(transparent)]
pub struct ELevelVisibility(pub u8);
impl ELevelVisibility {
    pub const VISIBLE: ELevelVisibility = ELevelVisibility(0);
    pub const HIDDEN: ELevelVisibility = ELevelVisibility(1);
}
#[repr(transparent)]
pub struct EParticleKey(pub u8);
impl EParticleKey {
    pub const ACTIVATE: EParticleKey = EParticleKey(0);
    pub const DEACTIVATE: EParticleKey = EParticleKey(1);
    pub const TRIGGER: EParticleKey = EParticleKey(2);
}
#[repr(transparent)]
pub struct EMovieSceneScalabilityConditionGroup(pub u8);
impl EMovieSceneScalabilityConditionGroup {
    pub const VIEW_DISTANCE: EMovieSceneScalabilityConditionGroup = EMovieSceneScalabilityConditionGroup(
        0,
    );
    pub const ANTI_ALIASING: EMovieSceneScalabilityConditionGroup = EMovieSceneScalabilityConditionGroup(
        1,
    );
    pub const SHADOW: EMovieSceneScalabilityConditionGroup = EMovieSceneScalabilityConditionGroup(
        2,
    );
    pub const GLOBAL_ILLUMINATION: EMovieSceneScalabilityConditionGroup = EMovieSceneScalabilityConditionGroup(
        3,
    );
    pub const REFLECTION: EMovieSceneScalabilityConditionGroup = EMovieSceneScalabilityConditionGroup(
        4,
    );
    pub const POST_PROCESS: EMovieSceneScalabilityConditionGroup = EMovieSceneScalabilityConditionGroup(
        5,
    );
    pub const TEXTURE: EMovieSceneScalabilityConditionGroup = EMovieSceneScalabilityConditionGroup(
        6,
    );
    pub const EFFECTS: EMovieSceneScalabilityConditionGroup = EMovieSceneScalabilityConditionGroup(
        7,
    );
    pub const FOLIAGE: EMovieSceneScalabilityConditionGroup = EMovieSceneScalabilityConditionGroup(
        8,
    );
    pub const SHADING: EMovieSceneScalabilityConditionGroup = EMovieSceneScalabilityConditionGroup(
        9,
    );
    pub const LANDSCAPE: EMovieSceneScalabilityConditionGroup = EMovieSceneScalabilityConditionGroup(
        10,
    );
}
#[repr(transparent)]
pub struct EMovieSceneScalabilityConditionOperator(pub u8);
impl EMovieSceneScalabilityConditionOperator {
    pub const LESS_THAN: EMovieSceneScalabilityConditionOperator = EMovieSceneScalabilityConditionOperator(
        0,
    );
    pub const LESS_THAN_OR_EQUAL_TO: EMovieSceneScalabilityConditionOperator = EMovieSceneScalabilityConditionOperator(
        1,
    );
    pub const EQUAL_TO: EMovieSceneScalabilityConditionOperator = EMovieSceneScalabilityConditionOperator(
        2,
    );
    pub const GREATER_THAN_OR_EQUAL_TO: EMovieSceneScalabilityConditionOperator = EMovieSceneScalabilityConditionOperator(
        3,
    );
    pub const GREATER_THAN: EMovieSceneScalabilityConditionOperator = EMovieSceneScalabilityConditionOperator(
        4,
    );
}
#[repr(transparent)]
pub struct EMovieSceneScalabilityConditionLevel(pub u8);
impl EMovieSceneScalabilityConditionLevel {
    pub const LOW: EMovieSceneScalabilityConditionLevel = EMovieSceneScalabilityConditionLevel(
        0,
    );
    pub const MEDIUM: EMovieSceneScalabilityConditionLevel = EMovieSceneScalabilityConditionLevel(
        1,
    );
    pub const HIGH: EMovieSceneScalabilityConditionLevel = EMovieSceneScalabilityConditionLevel(
        2,
    );
    pub const EPIC: EMovieSceneScalabilityConditionLevel = EMovieSceneScalabilityConditionLevel(
        3,
    );
    pub const CINEMATIC: EMovieSceneScalabilityConditionLevel = EMovieSceneScalabilityConditionLevel(
        4,
    );
}
#[repr(transparent)]
pub struct EShow3DTrajectory(pub u8);
impl EShow3DTrajectory {
    pub const EST_ONLY_WHEN_SELECTED: EShow3DTrajectory = EShow3DTrajectory(0);
    pub const EST_ALWAYS: EShow3DTrajectory = EShow3DTrajectory(1);
    pub const EST_NEVER: EShow3DTrajectory = EShow3DTrajectory(2);
}
#[repr(transparent)]
pub struct EFireEventsAtPosition(pub u8);
impl EFireEventsAtPosition {
    pub const AT_START_OF_EVALUATION: EFireEventsAtPosition = EFireEventsAtPosition(0);
    pub const AT_END_OF_EVALUATION: EFireEventsAtPosition = EFireEventsAtPosition(1);
    pub const AFTER_SPAWN: EFireEventsAtPosition = EFireEventsAtPosition(2);
}
