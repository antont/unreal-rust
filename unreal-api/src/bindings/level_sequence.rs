#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FLevelSequenceCameraSettings {
    pub b_override_aspect_ratio_axis_constraint: bool,
    pub aspect_ratio_axis_constraint: crate::bindings::engine::EAspectRatioAxisConstraint,
}
impl FLevelSequenceCameraSettings {}
#[repr(C, align(8))]
pub struct FLevelSequenceAnimSequenceLinkItem {
    pub skel_track_guid: crate::bindings::core_u_object::FGuid,
    pub path_to_anim_sequence: crate::bindings::core_u_object::FSoftObjectPath,
    pub b_export_transforms: bool,
    pub b_export_morph_targets: bool,
    pub b_export_attribute_curves: bool,
    pub b_export_material_curves: bool,
    pub interpolation: crate::bindings::engine::EAnimInterpolationType,
    pub curve_interpolation: crate::bindings::engine::ERichCurveInterpMode,
    pub b_record_in_world_space: bool,
    pub b_evaluate_all_skeletal_mesh_components: bool,
    pub include_animation_names: TArray<FString>,
    pub exclude_animation_names: TArray<FString>,
    pub warm_up_frames: crate::bindings::core_u_object::FFrameNumber,
    pub delay_before_start: crate::bindings::core_u_object::FFrameNumber,
    pub b_use_custom_time_range: bool,
    pub custom_start_frame: crate::bindings::core_u_object::FFrameNumber,
    pub custom_end_frame: crate::bindings::core_u_object::FFrameNumber,
    pub custom_display_rate: crate::bindings::core_u_object::FFrameRate,
    pub b_use_custom_frame_rate: bool,
    pub custom_frame_rate: crate::bindings::core_u_object::FFrameRate,
}
impl FLevelSequenceAnimSequenceLinkItem {}
#[repr(C, align(8))]
pub struct FLevelSequencePlayerSnapshot {
    pub root_name: FString,
    pub root_time: crate::bindings::core_u_object::FQualifiedFrameTime,
    pub source_time: crate::bindings::core_u_object::FQualifiedFrameTime,
    pub current_shot_name: FString,
    pub current_shot_local_time: crate::bindings::core_u_object::FQualifiedFrameTime,
    pub current_shot_source_time: crate::bindings::core_u_object::FQualifiedFrameTime,
    pub source_timecode: FString,
    pub camera_component: TSoftObjectPtr<crate::bindings::engine::UCameraComponent>,
    pub active_shot: UPtr<ULevelSequence>,
    __padding_end: [u8; 8],
}
impl FLevelSequencePlayerSnapshot {}
#[repr(C, align(16))]
pub struct UDefaultLevelSequenceInstanceData {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub transform_origin_actor: UPtr<crate::bindings::engine::AActor>,
    pub transform_origin: crate::bindings::core_u_object::FTransform,
}
impl UDefaultLevelSequenceInstanceData {}
#[repr(C, align(8))]
pub struct UAnimSequenceLevelSequenceLink {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub skel_track_guid: crate::bindings::core_u_object::FGuid,
    pub path_to_level_sequence: crate::bindings::core_u_object::FSoftObjectPath,
}
impl UAnimSequenceLevelSequenceLink {}
#[repr(C, align(8))]
pub struct ULevelSequence {
    __padding_end: [u8; 288],
}
impl ULevelSequence {}
#[repr(C, align(8))]
pub struct ULevelSequenceBurnInInitSettings {
    __padding_end: [u8; 48],
}
impl ULevelSequenceBurnInInitSettings {}
#[repr(C, align(8))]
pub struct ULevelSequenceBurnInOptions {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub b_use_burn_in: bool,
    pub burn_in_class: crate::bindings::core_u_object::FSoftClassPath,
    pub settings: UPtr<ULevelSequenceBurnInInitSettings>,
}
impl ULevelSequenceBurnInOptions {}
#[repr(C, align(8))]
pub struct ALevelSequenceActor {
    #[doc(hidden)]
    __padding_1160: [u8; 1160],
    pub playback_settings: crate::bindings::movie_scene::FMovieSceneSequencePlaybackSettings,
    pub sequence_player: UPtr<ULevelSequencePlayer>,
    pub level_sequence_asset: UPtr<ULevelSequence>,
    #[doc(hidden)]
    __padding_1256: [u8; 40],
    pub camera_settings: FLevelSequenceCameraSettings,
    pub burn_in_options: UPtr<ULevelSequenceBurnInOptions>,
    pub binding_overrides: UPtr<
        crate::bindings::movie_scene::UMovieSceneBindingOverrides,
    >,
    pub flags_1280: u8,
    pub default_instance_data: UPtr<crate::bindings::core_u_object::UObject>,
    __padding_end: [u8; 56],
}
impl ALevelSequenceActor {}
#[repr(C, align(8))]
pub struct AReplicatedLevelSequenceActor {
    __padding_end: [u8; 1352],
}
impl AReplicatedLevelSequenceActor {}
#[repr(C, align(8))]
pub struct ULevelSequenceAnimSequenceLink {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub anim_sequence_links: TArray<FLevelSequenceAnimSequenceLinkItem>,
}
impl ULevelSequenceAnimSequenceLink {}
#[repr(C, align(8))]
pub struct ULevelSequenceBurnIn {
    #[doc(hidden)]
    __padding_1288: [u8; 1288],
    pub frame_information: FLevelSequencePlayerSnapshot,
    pub level_sequence_actor: UPtr<ALevelSequenceActor>,
}
impl ULevelSequenceBurnIn {}
#[repr(C, align(8))]
pub struct ULevelSequenceDirector {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub player: UPtr<ULevelSequencePlayer>,
    __padding_end: [u8; 8],
}
impl ULevelSequenceDirector {}
#[repr(C, align(8))]
pub struct ULegacyLevelSequenceDirectorBlueprint {
    __padding_end: [u8; 1432],
}
impl ULegacyLevelSequenceDirectorBlueprint {}
#[repr(C, align(8))]
pub struct ULevelSequencePlayer {
    __padding_end: [u8; 1520],
}
impl ULevelSequencePlayer {}
#[repr(C, align(8))]
pub struct ULevelSequenceProjectSettings {
    __padding_end: [u8; 152],
}
impl ULevelSequenceProjectSettings {}
#[repr(C, align(8))]
pub struct ULevelSequenceShotMetaDataLibrary {
    __padding_end: [u8; 48],
}
impl ULevelSequenceShotMetaDataLibrary {}
#[repr(C, align(8))]
pub struct ALevelSequenceMediaController {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub sequence: UPtr<ALevelSequenceActor>,
    pub media_component: UPtr<crate::bindings::media_assets::UMediaComponent>,
    pub server_start_time_seconds: f32,
    __padding_end: [u8; 12],
}
impl ALevelSequenceMediaController {}
#[repr(C, align(8))]
pub struct FLevelSequencePlayer_OnCameraCut {
    _opague: [u8; 24],
}
