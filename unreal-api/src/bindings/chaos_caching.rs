#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FCacheEventBase {}
#[repr(C, align(8))]
pub struct FEnableStateEvent {
    pub index: i32,
    pub b_enable: bool,
}
#[repr(C, align(16))]
pub struct FBreakingEvent {
    pub index: i32,
    pub location: crate::bindings::core_u_object::FVector,
    pub orientation: crate::bindings::core_u_object::FQuat,
    pub velocity: crate::bindings::core_u_object::FVector,
    pub angular_velocity: crate::bindings::core_u_object::FVector,
    pub mass: f32,
    pub bounding_box_min: crate::bindings::core_u_object::FVector,
    pub bounding_box_max: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FCollisionEvent {
    pub location: crate::bindings::core_u_object::FVector,
    pub accumulated_impulse: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub velocity1: crate::bindings::core_u_object::FVector,
    pub velocity2: crate::bindings::core_u_object::FVector,
    pub delta_velocity1: crate::bindings::core_u_object::FVector,
    pub delta_velocity2: crate::bindings::core_u_object::FVector,
    pub angular_velocity1: crate::bindings::core_u_object::FVector,
    pub angular_velocity2: crate::bindings::core_u_object::FVector,
    pub mass1: f32,
    pub mass2: f32,
    pub penetration_depth: f32,
}
#[repr(C, align(16))]
pub struct FTrailingEvent {
    pub index: i32,
    pub location: crate::bindings::core_u_object::FVector,
    pub orientation: crate::bindings::core_u_object::FQuat,
    pub velocity: crate::bindings::core_u_object::FVector,
    pub angular_velocity: crate::bindings::core_u_object::FVector,
    pub bounding_box_min: crate::bindings::core_u_object::FVector,
    pub bounding_box_max: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FCacheEventTrack {
    pub name: FName,
    pub _struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub time_stamps: TArray<f32>,
}
#[repr(C, align(16))]
pub struct FObservedComponent {
    pub cache_name: FName,
    pub component_ref: crate::bindings::engine::FComponentReference,
    pub soft_component_ref: crate::bindings::engine::FSoftComponentReference,
    pub b_is_simulating: bool,
    pub b_playback_enabled: bool,
    pub usd_cache_directory: crate::bindings::core_u_object::FDirectoryPath,
    pub b_has_notify_breaks: bool,
}
#[repr(C, align(8))]
pub struct FParticleTransformTrack {
    pub raw_transform_track: crate::bindings::engine::FRawAnimSequenceTrack,
    pub begin_offset: f32,
    pub b_deactivate_on_end: bool,
    pub key_timestamps: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FPerParticleCacheData {
    pub transform_data: FParticleTransformTrack,
    pub curve_data: TMap<FName, crate::bindings::engine::FRichCurve>,
}
#[repr(C, align(16))]
pub struct FCacheSpawnableTemplate {
    pub duplicated_template: UPtr<crate::bindings::core_u_object::UObject>,
    pub initial_transform: crate::bindings::core_u_object::FTransform,
    pub component_transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FRichCurves {
    pub rich_curves: TArray<crate::bindings::engine::FRichCurve>,
}
#[repr(C, align(8))]
pub struct FCompressedRichCurves {
    pub compressed_rich_curves: TArray<crate::bindings::engine::FCompressedRichCurve>,
}
#[repr(C, align(8))]
pub struct FMovieSceneChaosCacheParams {
    pub cache_collection: UPtr<UChaosCacheCollection>,
}
#[repr(C, align(8))]
pub struct FMovieSceneChaosCacheSectionTemplateParameters {
    pub chaos_cache_params: FMovieSceneChaosCacheParams,
}
#[repr(C, align(8))]
pub struct FMovieSceneChaosCacheSectionTemplate {
    pub params: FMovieSceneChaosCacheSectionTemplateParameters,
}
pub struct UChaosCacheCollection {
    pub caches: TArray<UPtr<UChaosCache>>,
    pub interpolation_mode: EChaosCacheInterpolationMode,
}
pub struct AChaosCacheManager {
    pub cache_collection: UPtr<UChaosCacheCollection>,
    pub b_create_new_cache_collection_asset_on_record: bool,
    pub cache_mode: ECacheMode,
    pub start_mode: EStartMode,
    pub start_time: f32,
    pub b_start_on_begin_play: bool,
    pub observed_components: TArray<FObservedComponent>,
    pub editor_icon_component: UPtr<crate::bindings::engine::UBillboardComponent>,
}
pub struct AChaosCachePlayer {}
pub struct UChaosCacheData {}
pub struct IChaosCacheData {}
pub struct UChaosCache {
    pub recorded_duration: f32,
    pub num_recorded_frames: u32,
    pub interpolation_mode: EChaosCacheInterpolationMode,
    pub track_to_particle: TArray<i32>,
    pub particle_tracks: TArray<FPerParticleCacheData>,
    pub channel_curve_to_particle: TArray<i32>,
    pub channels_tracks: TMap<FName, FRichCurves>,
    pub compressed_channels_tracks: TMap<FName, FCompressedRichCurves>,
    pub curve_data: TMap<FName, crate::bindings::engine::FRichCurve>,
    pub named_transform_tracks: TMap<FName, FParticleTransformTrack>,
    pub b_compress_channels: bool,
    pub channels_compression_error_threshold: f32,
    pub channels_compression_sample_rate: f32,
    pub cache_data: TScriptInterface<IChaosCacheData>,
    pub event_tracks: TMap<FName, FCacheEventTrack>,
    pub spawnable: FCacheSpawnableTemplate,
    pub adapter_guid: crate::bindings::core_u_object::FGuid,
    pub version: i32,
}
pub struct UMovieSceneChaosCacheSection {
    pub params: FMovieSceneChaosCacheParams,
}
pub struct UMovieSceneChaosCacheTrack {
    pub animation_sections: TArray<
        UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    >,
}
pub struct UMovieSceneSpawnableChaosCacheBinding {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EChaosCacheInterpolationMode(pub u8);
impl EChaosCacheInterpolationMode {
    pub const QUAT_INTERP: EChaosCacheInterpolationMode = EChaosCacheInterpolationMode(
        0,
    );
    pub const EULER_INTERP: EChaosCacheInterpolationMode = EChaosCacheInterpolationMode(
        1,
    );
    pub const DUAL_QUAT_INTERP: EChaosCacheInterpolationMode = EChaosCacheInterpolationMode(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECacheMode(pub u8);
impl ECacheMode {
    pub const NONE: ECacheMode = ECacheMode(0);
    pub const PLAY: ECacheMode = ECacheMode(1);
    pub const RECORD: ECacheMode = ECacheMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStartMode(pub u8);
impl EStartMode {
    pub const TIMED: EStartMode = EStartMode(0);
    pub const TRIGGERED: EStartMode = EStartMode(1);
}
