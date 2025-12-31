#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FTakeRecorderPropertyTrackSettings {
    pub component_path: FString,
    pub property_path: FString,
}
#[repr(C, align(8))]
pub struct FTakeRecorderTrackSettings {
    pub matching_actor_class: crate::bindings::core_u_object::FSoftClassPath,
    pub default_property_tracks: TArray<FTakeRecorderPropertyTrackSettings>,
    pub exclude_property_tracks: TArray<FTakeRecorderPropertyTrackSettings>,
}
pub struct UMovieSceneTrackRecorder {}
pub struct UMovieSceneTrackRecorderSettings {}
pub struct UMovieScene3DAttachTrackRecorder {}
pub struct UMovieScene3DTransformTrackRecorder {}
pub struct UMovieSceneAnimationTrackRecorder {}
pub struct UMovieSceneAnimationTrackRecorderEditorSettings {
    pub animation_track_name: FText,
    pub animation_asset_name: FString,
    pub animation_sub_directory: FString,
    pub interp_mode: crate::bindings::engine::ERichCurveInterpMode,
    pub tangent_mode: crate::bindings::engine::ERichCurveTangentMode,
    pub b_remove_root_animation: bool,
    pub timecode_bone_method: crate::bindings::sequence_recorder::FTimecodeBoneMethod,
}
pub struct UMovieSceneAnimationTrackRecorderSettings {}
pub struct UMovieSceneParticleTrackRecorder {}
pub struct UMovieScenePropertyTrackRecorder {}
pub struct UMovieSceneSpawnTrackRecorder {}
pub struct UMovieSceneVisibilityTrackRecorder {}
