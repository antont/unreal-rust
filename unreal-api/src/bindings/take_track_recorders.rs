#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UMovieSceneTrackRecorder {
    __padding_end: [u8; 136],
}
impl UMovieSceneTrackRecorder {}
#[repr(C, align(8))]
pub struct UMovieSceneTrackRecorderSettings {
    __padding_end: [u8; 48],
}
impl UMovieSceneTrackRecorderSettings {}
#[repr(C, align(8))]
pub struct UMovieScene3DAttachTrackRecorder {
    __padding_end: [u8; 192],
}
impl UMovieScene3DAttachTrackRecorder {}
#[repr(C, align(16))]
pub struct UMovieScene3DTransformTrackRecorder {
    __padding_end: [u8; 656],
}
impl UMovieScene3DTransformTrackRecorder {}
#[repr(C, align(16))]
pub struct UMovieSceneAnimationTrackRecorder {
    __padding_end: [u8; 544],
}
impl UMovieSceneAnimationTrackRecorder {}
#[repr(C, align(8))]
pub struct UMovieSceneAnimationTrackRecorderEditorSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub animation_track_name: FText,
    pub animation_asset_name: FString,
    pub animation_sub_directory: FString,
    #[doc(hidden)]
    __padding_98: [u8; 2],
    pub b_remove_root_animation: bool,
    pub timecode_bone_method: crate::bindings::sequence_recorder::FTimecodeBoneMethod,
    __padding_end: [u8; 4],
}
impl UMovieSceneAnimationTrackRecorderEditorSettings {}
#[repr(C, align(8))]
pub struct UMovieSceneAnimationTrackRecorderSettings {
    __padding_end: [u8; 120],
}
impl UMovieSceneAnimationTrackRecorderSettings {}
#[repr(C, align(8))]
pub struct UMovieSceneParticleTrackRecorder {
    __padding_end: [u8; 176],
}
impl UMovieSceneParticleTrackRecorder {}
#[repr(C, align(8))]
pub struct UMovieScenePropertyTrackRecorder {
    __padding_end: [u8; 184],
}
impl UMovieScenePropertyTrackRecorder {}
#[repr(C, align(8))]
pub struct UMovieSceneSpawnTrackRecorder {
    __padding_end: [u8; 232],
}
impl UMovieSceneSpawnTrackRecorder {}
#[repr(C, align(8))]
pub struct UMovieSceneVisibilityTrackRecorder {
    __padding_end: [u8; 152],
}
impl UMovieSceneVisibilityTrackRecorder {}
