#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FMovieSceneMixedAnimationTarget {}
#[repr(C, align(8))]
pub struct FMovieSceneCameraShakeSourceTrigger {
    pub shake_class: TSubclassOf<crate::bindings::engine::UCameraShakeBase>,
    pub play_scale: f32,
    pub play_space: crate::bindings::engine::ECameraShakePlaySpace,
    pub user_defined_play_space: crate::bindings::core_u_object::FRotator,
}
#[repr(C, align(8))]
pub struct FMovieSceneCameraShakeSourceTriggerChannel {
    pub key_times: TArray<crate::bindings::core_u_object::FFrameNumber>,
    pub key_values: TArray<FMovieSceneCameraShakeSourceTrigger>,
    pub key_handles: crate::bindings::movie_scene::FMovieSceneKeyHandleMap,
}
#[repr(C, align(8))]
pub struct FMovieSceneDoublePerlinNoiseChannel {
    pub perlin_noise_params: FPerlinNoiseParams,
}
#[repr(C, align(8))]
pub struct FPerlinNoiseParams {
    pub frequency: f32,
    pub amplitude: f64,
    pub offset: f32,
}
#[repr(C, align(8))]
pub struct FMovieSceneEventPayloadVariable {
    pub object_value: crate::bindings::core_u_object::FSoftObjectPath,
    pub value: FString,
}
#[repr(C, align(8))]
pub struct FMovieSceneEventPtrs {
    pub function: UPtr<crate::bindings::core_u_object::UFunction>,
    pub bound_object_property: TFieldPath<FProperty>,
}
#[repr(C, align(8))]
pub struct FMovieSceneEvent {
    pub ptrs: FMovieSceneEventPtrs,
    pub payload_variables: TMap<FName, FMovieSceneEventPayloadVariable>,
    pub compiled_function_name: FName,
    pub bound_object_pin_name: FName,
    pub weak_endpoint: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
    pub graph_guid_deprecated: crate::bindings::core_u_object::FGuid,
    pub node_guid_deprecated: crate::bindings::core_u_object::FGuid,
    pub function_entry_deprecated: TWeakObjectPtr<
        crate::bindings::core_u_object::UObject,
    >,
}
#[repr(C, align(8))]
pub struct FMovieSceneEventChannel {
    pub key_times: TArray<crate::bindings::core_u_object::FFrameNumber>,
    pub key_values: TArray<FMovieSceneEvent>,
    pub key_handles: crate::bindings::movie_scene::FMovieSceneKeyHandleMap,
}
#[repr(C, align(8))]
pub struct FMovieSceneFloatPerlinNoiseChannel {
    pub perlin_noise_params: FPerlinNoiseParams,
}
#[repr(C, align(8))]
pub struct FMovieSceneStringChannel {
    pub times: TArray<crate::bindings::core_u_object::FFrameNumber>,
    pub values: TArray<FString>,
    pub default_value: FString,
    pub b_has_default_value: bool,
    pub key_handles: crate::bindings::movie_scene::FMovieSceneKeyHandleMap,
}
#[repr(C, align(8))]
pub struct FMovieSceneTextChannel {
    pub times: TArray<crate::bindings::core_u_object::FFrameNumber>,
    pub values: TArray<FText>,
    pub default_value: FText,
    pub b_has_default_value: bool,
    pub key_handles: crate::bindings::movie_scene::FMovieSceneKeyHandleMap,
}
#[repr(C, align(8))]
pub struct FMovieSceneDirectorBlueprintConditionPayloadVariable {
    pub object_value: crate::bindings::core_u_object::FSoftObjectPath,
    pub value: FString,
}
#[repr(C, align(8))]
pub struct FMovieSceneDirectorBlueprintConditionData {
    pub function: UPtr<crate::bindings::core_u_object::UFunction>,
    pub condition_context_property: TFieldPath<FProperty>,
    pub payload_variables: TMap<
        FName,
        FMovieSceneDirectorBlueprintConditionPayloadVariable,
    >,
    pub compiled_function_name: FName,
    pub condition_context_pin_name: FName,
    pub weak_endpoint: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FMovieScene3DPathSectionTemplate {
    pub path_binding_id: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
    pub timing_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub front_axis_enum: MovieScene3DPathSection_Axis,
    pub up_axis_enum: MovieScene3DPathSection_Axis,
    pub flags_372: u8,
}
#[repr(C, align(8))]
pub struct FMovieSceneActorReferenceSectionTemplate {
    pub property_data: crate::bindings::movie_scene::FMovieScenePropertySectionData,
    pub actor_reference_data: FMovieSceneActorReferenceData,
}
#[repr(C, align(8))]
pub struct FMovieSceneActorReferenceData {
    pub key_times: TArray<crate::bindings::core_u_object::FFrameNumber>,
    pub default_value: FMovieSceneActorReferenceKey,
    pub key_values: TArray<FMovieSceneActorReferenceKey>,
    pub key_handles: crate::bindings::movie_scene::FMovieSceneKeyHandleMap,
}
#[repr(C, align(4))]
pub struct FMovieSceneActorReferenceKey {
    pub object: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
    pub component_name: FName,
    pub socket_name: FName,
}
#[repr(C, align(4))]
pub struct FMovieSceneBaseCacheSectionTemplateParameters {
    pub section_start_time: crate::bindings::core_u_object::FFrameNumber,
    pub section_end_time: crate::bindings::core_u_object::FFrameNumber,
}
#[repr(C, align(8))]
pub struct FMovieSceneEventSectionTemplate {
    pub event_data: FMovieSceneEventSectionData,
    pub flags_320: u8,
}
#[repr(C, align(8))]
pub struct FMovieSceneEventSectionData {
    pub times: TArray<crate::bindings::core_u_object::FFrameNumber>,
    pub key_values: TArray<FEventPayload>,
    pub key_handles: crate::bindings::movie_scene::FMovieSceneKeyHandleMap,
    pub key_times_deprecated: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FEventPayload {
    pub event_name: FName,
    pub parameters: FMovieSceneEventParameters,
}
#[repr(C, align(8))]
pub struct FMovieSceneEventParameters {}
#[repr(C, align(8))]
pub struct FMovieSceneParameterSectionTemplate {
    pub scalars: TArray<FScalarParameterNameAndCurve>,
    pub bools: TArray<FBoolParameterNameAndCurve>,
    pub vector2_ds: TArray<FVector2DParameterNameAndCurves>,
    pub vectors: TArray<FVectorParameterNameAndCurves>,
    pub colors: TArray<FColorParameterNameAndCurves>,
    pub transforms: TArray<FTransformParameterNameAndCurves>,
}
#[repr(C, align(4))]
pub struct FBaseParameterNameAndValue {
    pub parameter_name: FName,
}
#[repr(C, align(8))]
pub struct FTransformParameterNameAndCurves {
    pub translation: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub rotation: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub scale: crate::bindings::movie_scene::FMovieSceneFloatChannel,
}
#[repr(C, align(8))]
pub struct FColorParameterNameAndCurves {
    pub red_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub green_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub blue_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub alpha_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
}
#[repr(C, align(8))]
pub struct FVectorParameterNameAndCurves {
    pub x_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub y_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub z_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
}
#[repr(C, align(8))]
pub struct FVector2DParameterNameAndCurves {
    pub x_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub y_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
}
#[repr(C, align(8))]
pub struct FBoolParameterNameAndCurve {
    pub parameter_curve: crate::bindings::movie_scene::FMovieSceneBoolChannel,
}
#[repr(C, align(8))]
pub struct FScalarParameterNameAndCurve {
    pub parameter_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
}
#[repr(C, align(8))]
pub struct FMovieSceneParticleParameterSectionTemplate {}
#[repr(C, align(8))]
pub struct FMovieSceneParticleSectionTemplate {
    pub particle_keys: FMovieSceneParticleChannel,
}
#[repr(C, align(8))]
pub struct FMovieSceneParticleChannel {}
#[repr(C, align(8))]
pub struct FMovieSceneBoolPropertySectionTemplate {
    pub bool_curve: crate::bindings::movie_scene::FMovieSceneBoolChannel,
}
#[repr(C, align(8))]
pub struct FLevelVisibilityComponentData {
    pub section: UPtr<UMovieSceneLevelVisibilitySection>,
}
#[repr(C, align(8))]
pub struct FMovieSceneDataLayerComponentData {
    pub section: UPtr<UMovieSceneDataLayerSection>,
}
#[repr(C, align(8))]
pub struct FConstraintComponentData {
    pub constraint_id: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FMovieSceneSkeletalAnimationComponentData {
    pub section: UPtr<UMovieSceneSkeletalAnimationSection>,
}
#[repr(C, align(8))]
pub struct FMovieSceneAudioComponentData {
    pub section: UPtr<UMovieSceneAudioSection>,
}
#[repr(C, align(8))]
pub struct FMovieSceneCameraShakeComponentData {
    pub section_data: FMovieSceneCameraShakeSectionData,
    pub section_start_time: crate::bindings::core_u_object::FFrameNumber,
    pub section_end_time: crate::bindings::core_u_object::FFrameNumber,
    pub section_signature: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FMovieSceneCameraShakeSectionData {
    pub shake_class: TSubclassOf<crate::bindings::engine::UCameraShakeBase>,
    pub play_scale: f32,
    pub play_space: crate::bindings::engine::ECameraShakePlaySpace,
    pub user_defined_play_space: crate::bindings::core_u_object::FRotator,
}
#[repr(C, align(8))]
pub struct FMovieSceneCameraShakeInstanceData {
    pub shake_instance: UPtr<crate::bindings::engine::UCameraShakeBase>,
    pub section_signature: crate::bindings::core_u_object::FGuid,
    pub b_managed_by_previewer: bool,
}
#[repr(C, align(4))]
pub struct FMovieSceneAudioInputData {
    pub float_inputs: FName,
    pub string_input: FName,
    pub bool_input: FName,
    pub int_input: FName,
}
#[repr(C, align(8))]
pub struct FMovieScene3DLocationKeyStruct {
    pub location: crate::bindings::core_u_object::FVector,
    pub time: crate::bindings::core_u_object::FFrameNumber,
}
#[repr(C, align(8))]
pub struct FMovieScene3DRotationKeyStruct {
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub time: crate::bindings::core_u_object::FFrameNumber,
}
#[repr(C, align(8))]
pub struct FMovieScene3DScaleKeyStruct {
    pub scale: crate::bindings::core_u_object::FVector,
    pub time: crate::bindings::core_u_object::FFrameNumber,
}
#[repr(C, align(8))]
pub struct FMovieScene3DTransformKeyStruct {
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub scale: crate::bindings::core_u_object::FVector,
    pub time: crate::bindings::core_u_object::FFrameNumber,
}
#[repr(C, align(4))]
pub struct FMovieSceneTransformMask {
    pub mask: u32,
}
#[repr(C, align(8))]
pub struct FMovieSceneBaseCacheParams {
    pub first_loop_start_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub start_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub end_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub play_rate: f32,
    pub flags_24: u8,
}
#[repr(C, align(8))]
pub struct FMovieSceneColorKeyStruct {
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub time: crate::bindings::core_u_object::FFrameNumber,
}
#[repr(C, align(8))]
pub struct FScalarMaterialParameterInfoAndCurve {
    pub parameter_info: crate::bindings::engine::FMaterialParameterInfo,
    pub parameter_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub parameter_layer_name: FString,
    pub parameter_asset_name: FString,
}
#[repr(C, align(8))]
pub struct FColorMaterialParameterInfoAndCurves {
    pub parameter_info: crate::bindings::engine::FMaterialParameterInfo,
    pub red_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub green_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub blue_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub alpha_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub parameter_layer_name: FString,
    pub parameter_asset_name: FString,
    pub parameter_channel_names: crate::bindings::engine::FParameterChannelNames,
}
#[repr(C, align(8))]
pub struct FMovieSceneConsoleVariableCollection {
    pub interface: TScriptInterface<IMovieSceneConsoleVariableTrackInterface>,
    pub b_only_include_checked: bool,
}
#[repr(C, align(8))]
pub struct FMovieSceneCVarOverrides {
    pub values_by_c_var: TMap<FString, FString>,
}
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
    pub weight: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub b_skip_anim_notifiers: bool,
    pub b_force_custom_mode: bool,
    pub swap_root_bone: crate::bindings::anim_graph_runtime::ESwapRootBone,
    pub b_linear_playback_when_scaled: bool,
    pub start_offset_deprecated: f32,
    pub end_offset_deprecated: f32,
}
#[repr(C, align(8))]
pub struct FMovieSceneFloatVectorKeyStructBase {
    pub time: crate::bindings::core_u_object::FFrameNumber,
}
#[repr(C, align(8))]
pub struct FMovieSceneVector2fKeyStruct {
    pub vector: crate::bindings::core_u_object::FVector2f,
}
#[repr(C, align(8))]
pub struct FMovieSceneVector3fKeyStruct {
    pub vector: crate::bindings::core_u_object::FVector3f,
}
#[repr(C, align(16))]
pub struct FMovieSceneVector4fKeyStruct {
    pub vector: crate::bindings::core_u_object::FVector4f,
}
#[repr(C, align(8))]
pub struct FMovieSceneDoubleVectorKeyStructBase {
    pub time: crate::bindings::core_u_object::FFrameNumber,
}
#[repr(C, align(8))]
pub struct FMovieSceneVector2DKeyStruct {
    pub vector: crate::bindings::core_u_object::FVector2D,
}
#[repr(C, align(8))]
pub struct FMovieSceneVector3dKeyStruct {
    pub vector: crate::bindings::core_u_object::FVector3d,
}
#[repr(C, align(16))]
pub struct FMovieSceneVector4dKeyStruct {
    pub vector: crate::bindings::core_u_object::FVector4d,
}
#[repr(C, align(8))]
pub struct FMovieSceneEventTriggerData {
    pub ptrs: FMovieSceneEventPtrs,
    pub object_binding_id: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FMovieScenePreAnimatedMaterialParameters {
    pub previous_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub soft_previous_material: TSoftObjectPtr<
        crate::bindings::engine::UMaterialInterface,
    >,
}
#[repr(C, align(16))]
pub struct FMovieSceneSkeletalAnimRootMotionTrackParams {}
#[repr(C, align(4))]
pub struct FComponentMaterialInfo {
    pub material_slot_name: FName,
    pub material_slot_index: i32,
    pub material_type: EComponentMaterialType,
}
pub struct UMovieScenePropertySystem {
    pub instantiator_system: UPtr<UMovieScenePropertyInstantiatorSystem>,
}
pub struct UMovieScenePropertyTrack {
    pub unique_track_name: FName,
    pub property_name_deprecated: FName,
    pub property_path_deprecated: FString,
    pub section_to_key: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    pub property_binding: crate::bindings::movie_scene::FMovieScenePropertyBinding,
    pub sections: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
}
pub struct UMovieSceneMaterialTrack {
    pub sections: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
    pub section_to_key: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
}
pub struct UMovieSceneTransformOrigin {}
pub struct IMovieSceneTransformOrigin {}
pub struct UMovieSceneConsoleVariableTrackInterface {}
pub struct IMovieSceneConsoleVariableTrackInterface {}
pub struct UMovieSceneReplaceableActorBinding {}
pub struct UMovieSceneReplaceableActorBinding_BPBase {
    pub binding_type_pretty_name: FText,
    pub binding_type_tooltip: FText,
    pub custom_binding_priority: i32,
    pub preview_spawnable_type: TSubclassOf<
        crate::bindings::movie_scene::UMovieSceneSpawnableBindingBase,
    >,
}
pub struct UMovieSceneReplaceableDirectorBlueprintBinding {
    pub dynamic_binding: crate::bindings::movie_scene::FMovieSceneDynamicBinding,
    pub preview_spawnable_type: TSubclassOf<
        crate::bindings::movie_scene::UMovieSceneSpawnableBindingBase,
    >,
}
pub struct UMovieSceneSpawnableActorBindingBase {
    pub b_net_addressable_name: bool,
    pub level_name: FName,
}
pub struct UMovieSceneSpawnableActorBinding {
    pub actor_template: UPtr<crate::bindings::engine::AActor>,
}
pub struct UMovieSceneSpawnableDirectorBlueprintBinding {
    pub dynamic_binding: crate::bindings::movie_scene::FMovieSceneDynamicBinding,
}
pub struct UMovieSceneDoublePerlinNoiseChannelContainer {
    pub perlin_noise_channel: FMovieSceneDoublePerlinNoiseChannel,
}
pub struct UMovieSceneFloatPerlinNoiseChannelContainer {
    pub perlin_noise_channel: FMovieSceneFloatPerlinNoiseChannel,
}
pub struct UMovieSceneDirectorBlueprintCondition {
    pub director_blueprint_condition_data: FMovieSceneDirectorBlueprintConditionData,
    pub scope: crate::bindings::movie_scene::EMovieSceneConditionScope,
    pub check_frequency: crate::bindings::movie_scene::EMovieSceneConditionCheckFrequency,
}
pub struct UMovieScenePlatformCondition {
    pub valid_platforms: TArray<FName>,
}
pub struct UMovieSceneScalabilityCondition {
    pub group: EMovieSceneScalabilityConditionGroup,
    pub operator: EMovieSceneScalabilityConditionOperator,
    pub level: EMovieSceneScalabilityConditionLevel,
}
pub struct UMovieSceneInterrogatedPropertyInstantiatorSystem {}
pub struct UMovieSceneTracksSettings {
    pub b_preview_camera_cuts_in_simulate: bool,
}
pub struct UMovieScene3DConstraintSection {
    pub constraint_id_deprecated: crate::bindings::core_u_object::FGuid,
    pub constraint_binding_id: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
}
pub struct UMovieScene3DAttachSection {
    pub attach_socket_name: FName,
    pub attach_component_name: FName,
    pub b_full_revert_on_detach: bool,
    pub re_attach_on_detach: TSoftObjectPtr<crate::bindings::engine::USceneComponent>,
    pub attachment_location_rule: crate::bindings::engine::EAttachmentRule,
    pub attachment_rotation_rule: crate::bindings::engine::EAttachmentRule,
    pub attachment_scale_rule: crate::bindings::engine::EAttachmentRule,
    pub detachment_location_rule: crate::bindings::engine::EDetachmentRule,
    pub detachment_rotation_rule: crate::bindings::engine::EDetachmentRule,
    pub detachment_scale_rule: crate::bindings::engine::EDetachmentRule,
}
pub struct UMovieScene3DPathSection {
    pub timing_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub front_axis_enum: MovieScene3DPathSection_Axis,
    pub up_axis_enum: MovieScene3DPathSection_Axis,
    pub flags_716: u8,
}
pub struct UMovieScene3DTransformSectionConstraints {
    pub constraints_channels: TArray<
        crate::bindings::constraints::FConstraintAndActiveChannel,
    >,
}
pub struct UMovieScene3DTransformSection {
    pub transform_mask: FMovieSceneTransformMask,
    pub translation: crate::bindings::movie_scene::FMovieSceneDoubleChannel,
    pub rotation: crate::bindings::movie_scene::FMovieSceneDoubleChannel,
    pub scale: crate::bindings::movie_scene::FMovieSceneDoubleChannel,
    pub manual_weight: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub override_registry: UPtr<
        crate::bindings::movie_scene::UMovieSceneSectionChannelOverrideRegistry,
    >,
    pub constraints: UPtr<UMovieScene3DTransformSectionConstraints>,
    pub b_use_quaternion_interpolation: bool,
    pub show3_d_trajectory: EShow3DTrajectory,
}
pub struct UMovieSceneActorReferenceSection {
    pub actor_reference_data: FMovieSceneActorReferenceData,
    pub actor_guid_index_curve_deprecated: crate::bindings::engine::FIntegralCurve,
    pub actor_guid_strings_deprecated: TArray<FString>,
}
pub struct UMovieSceneAudioSection {
    pub sound: UPtr<crate::bindings::engine::USoundBase>,
    pub start_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub start_offset_deprecated: f32,
    pub audio_start_time_deprecated: f32,
    pub audio_dilation_factor_deprecated: f32,
    pub audio_volume_deprecated: f32,
    pub sound_volume: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub pitch_multiplier: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub inputs_float: TMap<FName, crate::bindings::movie_scene::FMovieSceneFloatChannel>,
    pub inputs_string: TMap<FName, FMovieSceneStringChannel>,
    pub inputs_bool: TMap<FName, crate::bindings::movie_scene::FMovieSceneBoolChannel>,
    pub inputs_int: TMap<FName, crate::bindings::movie_scene::FMovieSceneIntegerChannel>,
    pub inputs_trigger: TMap<
        FName,
        crate::bindings::movie_scene::FMovieSceneAudioTriggerChannel,
    >,
    pub attach_actor_data: FMovieSceneActorReferenceData,
    pub b_looping: bool,
    pub b_suppress_subtitles: bool,
    pub b_override_attenuation: bool,
    pub attenuation_settings: UPtr<crate::bindings::engine::USoundAttenuation>,
    pub on_queue_subtitles: FMovieSceneAudioSection_OnQueueSubtitles,
    pub on_audio_finished: FMovieSceneAudioSection_OnAudioFinished,
    pub on_audio_playback_percent: FMovieSceneAudioSection_OnAudioPlaybackPercent,
}
pub struct UMovieSceneBaseCacheSection {}
pub struct UMovieSceneByteSection {
    pub byte_curve: crate::bindings::movie_scene::FMovieSceneByteChannel,
}
pub struct UMovieSceneCameraCutSection {
    pub b_lock_previous_camera: bool,
    pub camera_guid_deprecated: crate::bindings::core_u_object::FGuid,
    pub camera_binding_id: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
    pub initial_camera_cut_transform: crate::bindings::core_u_object::FTransform,
    pub b_has_initial_camera_cut_transform: bool,
    pub thumbnail_reference_offset: f32,
}
pub struct UMovieSceneCameraShakeSection {
    pub shake_data: FMovieSceneCameraShakeSectionData,
    pub shake_class_deprecated: TSubclassOf<crate::bindings::engine::UCameraShakeBase>,
    pub play_scale_deprecated: f32,
    pub play_space_deprecated: crate::bindings::engine::ECameraShakePlaySpace,
    pub user_defined_play_space_deprecated: crate::bindings::core_u_object::FRotator,
}
pub struct UMovieSceneCameraShakeSourceShakeSection {
    pub shake_data: FMovieSceneCameraShakeSectionData,
}
pub struct UMovieSceneCameraShakeSourceTriggerSection {
    pub channel: FMovieSceneCameraShakeSourceTriggerChannel,
}
pub struct UMovieSceneCinematicShotSection {
    pub shot_display_name: FString,
    pub display_name_deprecated: FText,
    pub thumbnail_reference_offset: f32,
}
pub struct UMovieSceneColorSection {
    pub red_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub green_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub blue_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub alpha_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
}
pub struct UMovieSceneComponentMaterialParameterSection {
    pub scalar_parameter_infos_and_curves: TArray<FScalarMaterialParameterInfoAndCurve>,
    pub color_parameter_infos_and_curves: TArray<FColorMaterialParameterInfoAndCurves>,
}
pub struct UMovieSceneConstrainedSection {}
pub struct IMovieSceneConstrainedSection {}
pub struct UMovieSceneParameterSection {
    pub bool_parameter_names_and_curves: TArray<FBoolParameterNameAndCurve>,
    pub scalar_parameter_names_and_curves: TArray<FScalarParameterNameAndCurve>,
    pub vector2_d_parameter_names_and_curves: TArray<FVector2DParameterNameAndCurves>,
    pub vector_parameter_names_and_curves: TArray<FVectorParameterNameAndCurves>,
    pub color_parameter_names_and_curves: TArray<FColorParameterNameAndCurves>,
    pub transform_parameter_names_and_curves: TArray<FTransformParameterNameAndCurves>,
}
pub struct UMovieSceneCustomPrimitiveDataSection {
    pub channels_used_bitmap: u64,
}
pub struct UMovieSceneCVarSection {
    pub console_variable_collections: TArray<FMovieSceneConsoleVariableCollection>,
    pub console_variables: FMovieSceneCVarOverrides,
}
pub struct UMovieSceneDataLayerSection {
    pub data_layer_assets: TArray<UPtr<crate::bindings::engine::UDataLayerAsset>>,
    pub desired_state: crate::bindings::engine::EDataLayerRuntimeState,
    pub preroll_state: crate::bindings::engine::EDataLayerRuntimeState,
    pub b_flush_on_activated: bool,
    pub b_flush_on_unload: bool,
}
pub struct UMovieSceneDoubleSection {
    pub double_curve: crate::bindings::movie_scene::FMovieSceneDoubleChannel,
}
pub struct UMovieSceneEnumSection {
    pub enum_curve: crate::bindings::movie_scene::FMovieSceneByteChannel,
}
pub struct UMovieSceneEventSectionBase {}
pub struct UMovieSceneEventRepeaterSection {
    pub event: FMovieSceneEvent,
}
pub struct UMovieSceneEventSection {
    pub events_deprecated: crate::bindings::engine::FNameCurve,
    pub event_data: FMovieSceneEventSectionData,
}
pub struct UMovieSceneEventTriggerSection {
    pub event_channel: FMovieSceneEventChannel,
}
pub struct UMovieSceneFadeSection {
    pub float_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub fade_color: crate::bindings::core_u_object::FLinearColor,
    pub flags_688: u8,
}
pub struct UMovieSceneFloatSection {
    pub float_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub override_registry: UPtr<
        crate::bindings::movie_scene::UMovieSceneSectionChannelOverrideRegistry,
    >,
}
pub struct UMovieSceneIntegerSection {
    pub integer_curve: crate::bindings::movie_scene::FMovieSceneIntegerChannel,
}
pub struct UMovieSceneLevelVisibilitySection {
    pub visibility: ELevelVisibility,
    pub level_names: TArray<FName>,
}
pub struct UMovieSceneObjectPropertySection {
    pub object_channel: crate::bindings::movie_scene::FMovieSceneObjectPathChannel,
}
pub struct UMovieSceneParameterSectionExtender {}
pub struct IMovieSceneParameterSectionExtender {}
pub struct UMovieSceneParticleSection {
    pub particle_keys: FMovieSceneParticleChannel,
}
pub struct UMovieScenePrimitiveMaterialSection {
    pub material_channel: crate::bindings::movie_scene::FMovieSceneObjectPathChannel,
}
pub struct UMovieSceneRotatorSection {
    pub rotation: crate::bindings::movie_scene::FMovieSceneDoubleChannel,
}
pub struct UMovieSceneSkeletalAnimationSection {
    pub params: FMovieSceneSkeletalAnimationParams,
    pub anim_sequence_deprecated: UPtr<crate::bindings::engine::UAnimSequence>,
    pub animation_deprecated: UPtr<crate::bindings::engine::UAnimSequenceBase>,
    pub start_offset_deprecated: f32,
    pub end_offset_deprecated: f32,
    pub play_rate_deprecated: f32,
    pub flags_788: u8,
    pub slot_name_deprecated: FName,
    pub start_location_offset: crate::bindings::core_u_object::FVector,
    pub start_rotation_offset: crate::bindings::core_u_object::FRotator,
    pub matched_bone_name: FName,
    pub matched_location_offset: crate::bindings::core_u_object::FVector,
    pub matched_rotation_offset: crate::bindings::core_u_object::FRotator,
    pub flags_920: u8,
    pub mixed_animation_target: crate::bindings::core_u_object::FInstancedStruct,
    pub mixed_animation_priority: i32,
}
pub struct UMovieSceneSlomoSection {
    pub float_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
}
pub struct UMovieSceneStringSection {
    pub string_curve: FMovieSceneStringChannel,
}
pub struct UMovieSceneTextSection {
    pub text_channel: FMovieSceneTextChannel,
}
pub struct UMovieSceneFloatVectorSection {
    pub curves: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub channels_used: i32,
}
pub struct UMovieSceneDoubleVectorSection {
    pub curves: crate::bindings::movie_scene::FMovieSceneDoubleChannel,
    pub channels_used: i32,
}
pub struct UMovieSceneVisibilitySection {}
pub struct UBoolChannelEvaluatorSystem {}
pub struct UByteChannelEvaluatorSystem {}
pub struct UDoubleChannelEvaluatorSystem {}
pub struct UDoublePerlinNoiseChannelEvaluatorSystem {}
pub struct UFloatChannelEvaluatorSystem {}
pub struct UFloatPerlinNoiseChannelEvaluatorSystem {}
pub struct UIntegerChannelEvaluatorSystem {}
pub struct UMovieScene3DTransformPropertySystem {}
pub struct UMovieSceneAudioSystem {}
pub struct UMovieSceneBaseValueEvaluatorSystem {}
pub struct UMovieSceneBoolPropertySystem {}
pub struct UMovieSceneBytePropertySystem {}
pub struct UMovieSceneCameraShakeInstantiatorSystem {}
pub struct UMovieSceneCameraShakeEvaluatorSystem {}
pub struct UMovieSceneColorPropertySystem {}
pub struct UMovieSceneComponentAttachmentInvalidatorSystem {}
pub struct UMovieSceneComponentAttachmentSystem {}
pub struct UMovieSceneComponentMaterialSystem {}
pub struct UMovieSceneComponentMobilitySystem {}
pub struct UMovieSceneComponentTransformSystem {}
pub struct UMovieSceneConstraintSystem {}
pub struct UMovieSceneCustomPrimitiveDataSystem {
    pub double_blender_system: UPtr<UMovieScenePiecewiseDoubleBlenderSystem>,
}
pub struct UMovieSceneDataLayerSystem {}
pub struct UMovieSceneDeferredComponentMovementSystem {}
pub struct UMovieSceneDoublePropertySystem {}
pub struct UMovieSceneEnumPropertySystem {}
pub struct UMovieSceneEulerTransformPropertySystem {}
pub struct UMovieSceneEventSystem {}
pub struct UMovieScenePreSpawnEventSystem {}
pub struct UMovieScenePostSpawnEventSystem {}
pub struct UMovieScenePostEvalEventSystem {}
pub struct UMovieSceneFadeSystem {}
pub struct UMovieSceneFloatPropertySystem {}
pub struct UMovieSceneHierarchicalBiasSystem {
    pub grouping_system: UPtr<
        crate::bindings::movie_scene::UMovieSceneEntityGroupingSystem,
    >,
}
pub struct UMovieSceneIntegerPropertySystem {}
pub struct UMovieSceneLevelVisibilitySystem {}
pub struct UMovieSceneMaterialParameterCollectionSystem {}
pub struct UMovieSceneMaterialParameterInstantiatorSystem {
    pub double_blender_system: UPtr<UMovieScenePiecewiseDoubleBlenderSystem>,
}
pub struct UMovieSceneMaterialParameterEvaluationSystem {}
pub struct UMovieSceneMotionVectorSimulationSystem {}
pub struct UMovieSceneObjectPropertySystem {}
pub struct UMovieScenePiecewiseBoolBlenderSystem {}
pub struct UMovieScenePiecewiseByteBlenderSystem {}
pub struct UMovieScenePiecewiseDoubleBlenderSystem {}
pub struct UMovieScenePiecewiseEnumBlenderSystem {}
pub struct UMovieScenePiecewiseIntegerBlenderSystem {}
pub struct UMovieSceneAsyncAction_SequencePrediction {
    pub result: FMovieSceneAsyncAction_SequencePrediction_Result,
    pub failure: FMovieSceneAsyncAction_SequencePrediction_Failure,
    pub sequence_player: UPtr<crate::bindings::movie_scene::UMovieSceneSequencePlayer>,
    pub scene_component: UPtr<crate::bindings::engine::USceneComponent>,
}
pub struct UMovieScenePredictionSystem {
    pub pending_predictions: TArray<UPtr<UMovieSceneAsyncAction_SequencePrediction>>,
    pub processing_predictions: TArray<UPtr<UMovieSceneAsyncAction_SequencePrediction>>,
}
pub struct UMovieScenePropertyInstantiatorSystem {}
pub struct UMovieScenePropertyNotificationSystem {}
pub struct UMovieSceneQuaternionBlenderSystem {}
pub struct UMovieSceneQuaternionInterpolationRotationSystem {}
pub struct UMovieSceneRotatorPropertySystem {}
pub struct UMovieSceneSkeletalAnimationSystem {}
pub struct UMovieSceneSlomoSystem {}
pub struct UMovieSceneStringPropertySystem {}
pub struct UMovieSceneTextPropertySystem {}
pub struct UMovieSceneTransformOriginInstantiatorSystem {}
pub struct UMovieSceneTransformOriginSystem {}
pub struct UMovieSceneFloatVectorPropertySystem {}
pub struct UMovieSceneDoubleVectorPropertySystem {}
pub struct UMovieSceneVisibilitySystem {}
pub struct UObjectPathChannelEvaluatorSystem {}
pub struct UStringChannelEvaluatorSystem {}
pub struct UTextChannelEvaluatorSystem {}
pub struct UMovieSceneHierarchicalEasingInstantiatorSystem {
    pub evaluator_system: UPtr<UWeightAndEasingEvaluatorSystem>,
}
pub struct UMovieSceneHierarchicalEasingFinalizationSystem {}
pub struct UWeightAndEasingEvaluatorSystem {}
pub struct UMovieSceneDecomposerTestObject {
    pub float_property: f32,
}
pub struct UMovieSceneFadeTrackTestLibrary {}
pub struct UMovieScenePartialEvaluationTestObject {
    pub float_property: f32,
    pub vector_property: crate::bindings::core_u_object::FVector,
}
pub struct UMovieSceneTestSequence {
    pub movie_scene: UPtr<crate::bindings::movie_scene::UMovieScene>,
    pub bound_objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub binding_guids: TArray<crate::bindings::core_u_object::FGuid>,
}
pub struct UMovieSceneCameraCutTrackInstance {}
pub struct UMovieSceneCVarTrackInstance {}
pub struct UMovieSceneSectionsToKey {}
pub struct IMovieSceneSectionsToKey {}
pub struct UMovieScene3DConstraintTrack {
    pub constraint_sections: TArray<
        UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    >,
}
pub struct UMovieScene3DAttachTrack {}
pub struct UMovieScene3DPathTrack {}
pub struct UMovieScene3DTransformTrack {
    pub blender_system_class: TSubclassOf<
        crate::bindings::movie_scene::UMovieSceneBlenderSystem,
    >,
}
pub struct UMovieSceneActorReferenceTrack {}
pub struct UMovieSceneAudioTrack {
    pub audio_sections: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
    pub row_height: i32,
}
pub struct UMovieSceneBoolTrack {}
pub struct UMovieSceneByteTrack {
    pub enum_: UPtr<crate::bindings::core_u_object::UEnum>,
}
pub struct UMovieSceneCameraCutTrack {
    pub b_can_blend: bool,
    pub sections: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
    pub b_auto_arrange_sections: bool,
}
pub struct UMovieSceneCameraShakeSourceShakeTrack {
    pub camera_shake_sections: TArray<
        UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    >,
}
pub struct UMovieSceneCameraShakeSourceTriggerTrack {
    pub sections: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
}
pub struct UMovieSceneCameraShakeTrack {
    pub camera_shake_sections: TArray<
        UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    >,
}
pub struct UMovieSceneCinematicShotTrack {}
pub struct UMovieSceneColorTrack {
    pub b_is_slate_color_deprecated: bool,
}
pub struct UMovieSceneCommonAnimationTrack {
    pub animation_sections: TArray<
        UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    >,
    pub root_motion_params: FMovieSceneSkeletalAnimRootMotionTrackParams,
    pub b_blend_first_child_of_root: bool,
    pub b_show_root_motion_trail: bool,
}
pub struct UMovieSceneCustomPrimitiveDataTrack {
    pub sections: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
    pub section_to_key: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
}
pub struct UMovieSceneCVarTrack {
    pub sections: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
}
pub struct UMovieSceneDataLayerTrack {
    pub sections: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
}
pub struct UMovieSceneDoubleTrack {}
pub struct UMovieSceneEnumTrack {
    pub enum_: UPtr<crate::bindings::core_u_object::UEnum>,
}
pub struct UMovieSceneEulerTransformTrack {}
pub struct UMovieSceneEventTrack {
    pub flags_400: u8,
    pub event_position: EFireEventsAtPosition,
    pub sections: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
}
pub struct UMovieSceneFloatTrack {}
pub struct UMovieSceneFadeTrack {}
pub struct UMovieSceneIntegerTrack {}
pub struct UMovieSceneLevelVisibilityTrack {
    pub sections: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
}
pub struct UMovieSceneMaterialParameterCollectionTrack {
    pub mpc: UPtr<crate::bindings::engine::UMaterialParameterCollection>,
}
pub struct UMovieSceneComponentMaterialTrack {
    pub material_index_deprecated: i32,
    pub material_info: FComponentMaterialInfo,
}
pub struct UMovieSceneObjectPropertyTrack {
    pub property_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub b_class_property: bool,
}
pub struct UMovieSceneParticleParameterTrack {
    pub sections: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
}
pub struct UMovieSceneParticleTrack {
    pub particle_sections: TArray<
        UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    >,
}
pub struct UMovieScenePrimitiveMaterialTrack {
    pub material_index_deprecated: i32,
    pub material_info: FComponentMaterialInfo,
}
pub struct UMovieSceneRotatorTrack {}
pub struct UMovieSceneSkeletalAnimationTrack {
    pub b_use_legacy_section_index_blend: bool,
    pub swap_root_bone: crate::bindings::anim_graph_runtime::ESwapRootBone,
}
pub struct UMovieSceneSlomoTrack {}
pub struct UMovieSceneStringTrack {}
pub struct UMovieSceneTextTrack {}
pub struct UMovieSceneTransformTrack {}
pub struct UMovieSceneFloatVectorTrack {
    pub num_channels_used: i32,
}
pub struct UMovieSceneDoubleVectorTrack {
    pub num_channels_used: i32,
}
pub struct UMovieSceneVisibilityTrack {}
pub struct FMovieSceneAudioSection_OnQueueSubtitles;
pub struct FMovieSceneAudioSection_OnAudioFinished;
pub struct FMovieSceneAudioSection_OnAudioPlaybackPercent;
pub struct FMovieSceneAsyncAction_SequencePrediction_Result;
pub struct FMovieSceneAsyncAction_SequencePrediction_Failure;
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELevelVisibility(pub u8);
impl ELevelVisibility {
    pub const VISIBLE: ELevelVisibility = ELevelVisibility(0);
    pub const HIDDEN: ELevelVisibility = ELevelVisibility(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EParticleKey(pub u8);
impl EParticleKey {
    pub const ACTIVATE: EParticleKey = EParticleKey(0);
    pub const DEACTIVATE: EParticleKey = EParticleKey(1);
    pub const TRIGGER: EParticleKey = EParticleKey(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EShow3DTrajectory(pub u8);
impl EShow3DTrajectory {
    pub const EST_ONLY_WHEN_SELECTED: EShow3DTrajectory = EShow3DTrajectory(0);
    pub const EST_ALWAYS: EShow3DTrajectory = EShow3DTrajectory(1);
    pub const EST_NEVER: EShow3DTrajectory = EShow3DTrajectory(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFireEventsAtPosition(pub u8);
impl EFireEventsAtPosition {
    pub const AT_START_OF_EVALUATION: EFireEventsAtPosition = EFireEventsAtPosition(0);
    pub const AT_END_OF_EVALUATION: EFireEventsAtPosition = EFireEventsAtPosition(1);
    pub const AFTER_SPAWN: EFireEventsAtPosition = EFireEventsAtPosition(2);
}
