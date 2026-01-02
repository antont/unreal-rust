#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FMovieSceneScriptingParams {
    pub time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
}
impl FMovieSceneScriptingParams {}
#[repr(C, align(8))]
pub struct UCinematicLevelViewportToolbarContext {
    __padding_end: [u8; 64],
}
impl UCinematicLevelViewportToolbarContext {}
#[repr(C, align(8))]
pub struct ULevelSequenceEditorMenuContext {
    __padding_end: [u8; 64],
}
impl ULevelSequenceEditorMenuContext {}
#[repr(C, align(8))]
pub struct UAssetDefinition_LevelSequence {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_LevelSequence {}
#[repr(C, align(8))]
pub struct ULevelSequenceFactoryNew {
    __padding_end: [u8; 136],
}
impl ULevelSequenceFactoryNew {}
#[repr(C, align(8))]
pub struct UFilmOverlayToolkit {
    __padding_end: [u8; 48],
}
impl UFilmOverlayToolkit {}
#[repr(C, align(8))]
pub struct ULevelSequenceEditorBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl ULevelSequenceEditorBlueprintLibrary {}
#[repr(C, align(8))]
pub struct UMovieSceneBindingPropertyInfoList {
    __padding_end: [u8; 64],
}
impl UMovieSceneBindingPropertyInfoList {}
#[repr(C, align(8))]
pub struct ULevelSequenceEditorSubsystem {
    __padding_end: [u8; 328],
}
impl ULevelSequenceEditorSubsystem {}
#[repr(C, align(8))]
pub struct ULevelSequenceEditorSettings {
    __padding_end: [u8; 72],
}
impl ULevelSequenceEditorSettings {}
#[repr(C, align(8))]
pub struct ULevelSequenceWithShotsSettings {
    #[doc(hidden)]
    __padding_128: [u8; 128],
    pub sub_sequence_names: TArray<FName>,
    __padding_end: [u8; 8],
}
impl ULevelSequenceWithShotsSettings {}
