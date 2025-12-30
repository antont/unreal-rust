#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(1))]
pub struct FMovieSceneScriptingParams {
    pub time_unit: EMovieSceneTimeUnit,
}
#[repr(C, align(8))]
pub struct FMovieSceneBindingPropertyInfo {
    pub locator: FUniversalObjectLocator,
    pub resolve_flags: ELocatorResolveFlags,
    pub custom_binding: UPtr<UMovieSceneCustomBinding>,
}
#[repr(C, align(8))]
pub struct FLevelSequencePropertyTrackSettings {
    pub component_path: FString,
    pub property_path: FString,
}
#[repr(C, align(8))]
pub struct FLevelSequenceTrackSettings {
    pub matching_actor_class: FSoftClassPath,
    pub default_tracks: TArray<FSoftClassPath>,
    pub exclude_default_tracks: TArray<FSoftClassPath>,
    pub default_property_tracks: TArray<FLevelSequencePropertyTrackSettings>,
    pub exclude_default_property_tracks: TArray<FLevelSequencePropertyTrackSettings>,
}
pub struct UCinematicLevelViewportToolbarContext {}
pub struct ULevelSequenceEditorMenuContext {}
pub struct UAssetDefinition_LevelSequence {}
pub struct ULevelSequenceFactoryNew {}
pub struct UFilmOverlayToolkit {}
pub struct ULevelSequenceEditorBlueprintLibrary {}
pub struct UMovieSceneBindingPropertyInfoList {
    pub bindings: TArray<FMovieSceneBindingPropertyInfo>,
}
pub struct ULevelSequenceEditorSubsystem {
    pub binding_property_info_list: UPtr<UMovieSceneBindingPropertyInfoList>,
    pub track_row_metadata_helper_list: TArray<UPtr<UMovieSceneTrackRowMetadataHelper>>,
    pub curve_editor_array: TArray<UPtr<USequencerCurveEditorObject>>,
}
pub struct ULevelSequenceEditorSettings {
    pub track_settings: TArray<FLevelSequenceTrackSettings>,
    pub b_auto_bind_to_pie: bool,
    pub b_auto_bind_to_simulate: bool,
}
pub struct ULevelSequenceWithShotsSettings {
    pub name: FString,
    pub suffix: FString,
    pub base_path: FDirectoryPath,
    pub num_shots: u32,
    pub sequence_to_duplicate: TLazyObjectPtr<ULevelSequence>,
    pub sub_sequence_names: TArray<FName>,
    pub b_instance_sub_sequences: bool,
}
