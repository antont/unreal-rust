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
#[repr(C, align(8))]
pub struct FLevelSequenceLegacyObjectReference {}
#[repr(C, align(8))]
pub struct FLevelSequenceObjectReferenceMap {}
#[repr(C, align(4))]
pub struct FLegacyLazyObjectPtrFragment {
    pub lazy_object_id: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FBoundActorProxy {
    pub bound_actor: UPtr<crate::bindings::engine::AActor>,
}
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
#[repr(C, align(8))]
pub struct FLevelSequenceBindingReference {
    pub package_name_deprecated: FString,
    pub external_object_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub object_path: FString,
}
#[repr(C, align(8))]
pub struct FLevelSequenceBindingReferenceArray {
    pub references: TArray<FLevelSequenceBindingReference>,
}
#[repr(C, align(8))]
pub struct FUpgradedLevelSequenceBindingReferences {}
#[repr(C, align(8))]
pub struct FLevelSequenceBindingReferences {
    pub binding_id_to_references: TMap<
        crate::bindings::core_u_object::FGuid,
        FLevelSequenceBindingReferenceArray,
    >,
    pub anim_sequence_instances: TSet<crate::bindings::core_u_object::FGuid>,
    pub post_process_instances: TSet<crate::bindings::core_u_object::FGuid>,
}
#[repr(C, align(8))]
pub struct FLevelSequenceObject {
    pub object_or_owner: TLazyObjectPtr<crate::bindings::core_u_object::UObject>,
    pub component_name: FString,
    pub cached_component: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
}
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
    pub shot_id: crate::bindings::movie_scene::FMovieSceneSequenceID,
}
pub struct UDefaultLevelSequenceInstanceData {
    pub transform_origin_actor: UPtr<crate::bindings::engine::AActor>,
    pub transform_origin: crate::bindings::core_u_object::FTransform,
}
pub struct UAnimSequenceLevelSequenceLink {
    pub skel_track_guid: crate::bindings::core_u_object::FGuid,
    pub path_to_level_sequence: crate::bindings::core_u_object::FSoftObjectPath,
}
pub struct ULevelSequence {
    pub movie_scene: UPtr<crate::bindings::movie_scene::UMovieScene>,
    pub binding_references: FUpgradedLevelSequenceBindingReferences,
    pub object_references_deprecated: FLevelSequenceObjectReferenceMap,
    pub director_blueprint: UPtr<crate::bindings::engine::UBlueprint>,
    pub director_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub meta_data_objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub asset_user_data: TArray<UPtr<crate::bindings::engine::UAssetUserData>>,
}
pub struct ULevelSequenceBurnInInitSettings {}
pub struct ULevelSequenceBurnInOptions {
    pub b_use_burn_in: bool,
    pub burn_in_class: crate::bindings::core_u_object::FSoftClassPath,
    pub settings: UPtr<ULevelSequenceBurnInInitSettings>,
}
pub struct ALevelSequenceActor {
    pub playback_settings: crate::bindings::movie_scene::FMovieSceneSequencePlaybackSettings,
    pub sequence_player: UPtr<ULevelSequencePlayer>,
    pub level_sequence_asset: UPtr<ULevelSequence>,
    pub level_sequence_deprecated: crate::bindings::core_u_object::FSoftObjectPath,
    pub camera_settings: FLevelSequenceCameraSettings,
    pub burn_in_options: UPtr<ULevelSequenceBurnInOptions>,
    pub binding_overrides: UPtr<
        crate::bindings::movie_scene::UMovieSceneBindingOverrides,
    >,
    pub flags_1280: u8,
    pub default_instance_data: UPtr<crate::bindings::core_u_object::UObject>,
    pub burn_in_instance: UPtr<ULevelSequenceBurnIn>,
    pub b_show_burnin: bool,
    pub world_partition_resolve_data: crate::bindings::engine::FWorldPartitionResolveData,
}
pub struct AReplicatedLevelSequenceActor {}
pub struct ULevelSequenceAnimSequenceLink {
    pub anim_sequence_links: TArray<FLevelSequenceAnimSequenceLinkItem>,
}
pub struct ULevelSequenceBurnIn {
    pub frame_information: FLevelSequencePlayerSnapshot,
    pub level_sequence_actor: UPtr<ALevelSequenceActor>,
}
pub struct ULevelSequenceDirector {
    pub sub_sequence_id: i32,
    pub weak_linker: TWeakObjectPtr<
        crate::bindings::movie_scene::UMovieSceneEntitySystemLinker,
    >,
    pub instance_id: u16,
    pub instance_serial: u16,
    pub player: UPtr<ULevelSequencePlayer>,
    pub movie_scene_player_index: i32,
}
pub struct ULegacyLevelSequenceDirectorBlueprint {}
pub struct ULevelSequencePlayer {
    pub on_camera_cut: FLevelSequencePlayer_OnCameraCut,
}
pub struct ULevelSequenceProjectSettings {
    pub b_default_lock_engine_to_display_rate: bool,
    pub default_display_rate: FString,
    pub default_tick_resolution: FString,
    pub default_clock_source: crate::bindings::movie_scene::EUpdateClockSource,
}
pub struct ULevelSequenceShotMetaDataLibrary {}
pub struct ALevelSequenceMediaController {
    pub sequence: UPtr<ALevelSequenceActor>,
    pub media_component: UPtr<crate::bindings::media_assets::UMediaComponent>,
    pub server_start_time_seconds: f32,
}
pub struct FLevelSequencePlayer_OnCameraCut;
