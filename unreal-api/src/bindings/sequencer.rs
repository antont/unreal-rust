#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FColumnVisibilitySetting {
    __padding_end: [u8; 16],
}
impl FColumnVisibilitySetting {}
#[repr(C, align(8))]
pub struct FMovieScenePasteFoldersParams {
    pub sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    pub parent_folder: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
}
impl FMovieScenePasteFoldersParams {}
#[repr(C, align(8))]
pub struct FMovieScenePasteSectionsParams {
    pub tracks: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>,
    pub track_row_indices: TArray<i32>,
    pub time: crate::bindings::core_u_object::FFrameTime,
}
impl FMovieScenePasteSectionsParams {}
#[repr(C, align(8))]
pub struct FMovieScenePasteTracksParams {
    pub sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    pub bindings: TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>,
    pub parent_folder: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
    pub folders: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>,
}
impl FMovieScenePasteTracksParams {}
#[repr(C, align(8))]
pub struct FMovieScenePasteBindingsParams {
    pub bindings: TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>,
    pub parent_folder: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
    pub folders: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>,
    pub b_duplicate_existing_actors: bool,
    pub pasted_actors: TMap<FName, UPtr<crate::bindings::engine::AActor>>,
}
impl FMovieScenePasteBindingsParams {}
#[repr(C, align(8))]
pub struct UMotionTrailToolOptions {
    __padding_end: [u8; 600],
}
impl UMotionTrailToolOptions {}
#[repr(C, align(16))]
pub struct UMovieSceneCopyableBinding {
    __padding_end: [u8; 944],
}
impl UMovieSceneCopyableBinding {}
#[repr(C, align(8))]
pub struct UMovieSceneCopyableTrack {
    __padding_end: [u8; 80],
}
impl UMovieSceneCopyableTrack {}
#[repr(C, align(8))]
pub struct USequencerFilterBarContext {
    __padding_end: [u8; 88],
}
impl USequencerFilterBarContext {}
#[repr(C, align(8))]
pub struct USequencerFilterMenuContext {
    __padding_end: [u8; 88],
}
impl USequencerFilterMenuContext {}
#[repr(C, align(8))]
pub struct USequencerMenuContext {
    __padding_end: [u8; 88],
}
impl USequencerMenuContext {}
#[repr(C, align(8))]
pub struct USequencerToolMenuContext {
    __padding_end: [u8; 64],
}
impl USequencerToolMenuContext {}
#[repr(C, align(8))]
pub struct USequencerClockSourceMenuContext {
    __padding_end: [u8; 80],
}
impl USequencerClockSourceMenuContext {}
#[repr(C, align(8))]
pub struct USequencerTimeSliderControllerMenuContext {
    __padding_end: [u8; 264],
}
impl USequencerTimeSliderControllerMenuContext {}
#[repr(C, align(8))]
pub struct USequencerTrackFilterExtension {
    __padding_end: [u8; 48],
}
impl USequencerTrackFilterExtension {}
#[repr(C, align(8))]
pub struct USequencerTrackFilterTextExpressionExtension {
    __padding_end: [u8; 48],
}
impl USequencerTrackFilterTextExpressionExtension {}
#[repr(C, align(8))]
pub struct USequencerModuleOutlinerScriptingObject {
    __padding_end: [u8; 88],
}
impl USequencerModuleOutlinerScriptingObject {}
#[repr(C, align(8))]
pub struct USequencerModuleScriptingLayer {
    __padding_end: [u8; 56],
}
impl USequencerModuleScriptingLayer {}
#[repr(C, align(8))]
pub struct UMovieSceneKeyStructType {
    __padding_end: [u8; 416],
}
impl UMovieSceneKeyStructType {}
#[repr(C, align(8))]
pub struct USequencerSettingsContainer {
    __padding_end: [u8; 48],
}
impl USequencerSettingsContainer {}
#[repr(C, align(8))]
pub struct USequencerSettings {
    __padding_end: [u8; 960],
}
impl USequencerSettings {}
#[repr(C, align(8))]
pub struct USequencerTimeChangeUndoRedoProxy {
    __padding_end: [u8; 88],
}
impl USequencerTimeChangeUndoRedoProxy {}
#[repr(transparent)]
pub struct ESequencerThumbnailCaptureTimeLocation(pub u8);
impl ESequencerThumbnailCaptureTimeLocation {
    pub const FIRST_FRAME: ESequencerThumbnailCaptureTimeLocation = ESequencerThumbnailCaptureTimeLocation(
        0,
    );
    pub const MIDDLE_FRAME: ESequencerThumbnailCaptureTimeLocation = ESequencerThumbnailCaptureTimeLocation(
        1,
    );
    pub const LAST_FRAME: ESequencerThumbnailCaptureTimeLocation = ESequencerThumbnailCaptureTimeLocation(
        2,
    );
    pub const CURRENT_FRAME: ESequencerThumbnailCaptureTimeLocation = ESequencerThumbnailCaptureTimeLocation(
        3,
    );
}
#[repr(transparent)]
pub struct ESequencerLoopMode(pub u8);
impl ESequencerLoopMode {
    pub const SLM_NO_LOOP: ESequencerLoopMode = ESequencerLoopMode(0);
    pub const SLM_LOOP: ESequencerLoopMode = ESequencerLoopMode(1);
    pub const SLM_LOOP_SELECTION_RANGE: ESequencerLoopMode = ESequencerLoopMode(2);
}
#[repr(transparent)]
pub struct EMotionTrailTrailStyle(pub u8);
impl EMotionTrailTrailStyle {
    pub const DEFAULT: EMotionTrailTrailStyle = EMotionTrailTrailStyle(0);
    pub const DASHED: EMotionTrailTrailStyle = EMotionTrailTrailStyle(1);
    pub const TIME: EMotionTrailTrailStyle = EMotionTrailTrailStyle(2);
    pub const HEAT_MAP: EMotionTrailTrailStyle = EMotionTrailTrailStyle(3);
}
#[repr(transparent)]
pub struct EAutoChangeMode(pub u8);
impl EAutoChangeMode {
    pub const AUTO_KEY: EAutoChangeMode = EAutoChangeMode(0);
    pub const AUTO_TRACK: EAutoChangeMode = EAutoChangeMode(1);
    pub const ALL: EAutoChangeMode = EAutoChangeMode(2);
    pub const NONE: EAutoChangeMode = EAutoChangeMode(3);
}
#[repr(transparent)]
pub struct EAllowEditsMode(pub u8);
impl EAllowEditsMode {
    pub const ALL_EDITS: EAllowEditsMode = EAllowEditsMode(0);
    pub const ALLOW_SEQUENCER_EDITS_ONLY: EAllowEditsMode = EAllowEditsMode(1);
    pub const ALLOW_LEVEL_EDITS_ONLY: EAllowEditsMode = EAllowEditsMode(2);
}
#[repr(transparent)]
pub struct EKeyGroupMode(pub u8);
impl EKeyGroupMode {
    pub const KEY_CHANGED: EKeyGroupMode = EKeyGroupMode(0);
    pub const KEY_GROUP: EKeyGroupMode = EKeyGroupMode(1);
    pub const KEY_ALL: EKeyGroupMode = EKeyGroupMode(2);
}
#[repr(transparent)]
pub struct ESequencerSpawnPosition(pub u8);
impl ESequencerSpawnPosition {
    pub const SSP_ORIGIN: ESequencerSpawnPosition = ESequencerSpawnPosition(0);
    pub const SSP_PLACE_IN_FRONT_OF_CAMERA: ESequencerSpawnPosition = ESequencerSpawnPosition(
        1,
    );
}
#[repr(transparent)]
pub struct ESequencerZoomPosition(pub u8);
impl ESequencerZoomPosition {
    pub const SZP_CURRENT_TIME: ESequencerZoomPosition = ESequencerZoomPosition(0);
    pub const SZP_MOUSE_POSITION: ESequencerZoomPosition = ESequencerZoomPosition(1);
}
#[repr(transparent)]
pub struct ESequencerTimeWarpDisplay(pub u8);
impl ESequencerTimeWarpDisplay {
    pub const UNWARPED_TIME: ESequencerTimeWarpDisplay = ESequencerTimeWarpDisplay(1);
    pub const WARPED_TIME: ESequencerTimeWarpDisplay = ESequencerTimeWarpDisplay(2);
    pub const BOTH: ESequencerTimeWarpDisplay = ESequencerTimeWarpDisplay(3);
}
