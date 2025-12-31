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
#[repr(C, align(8))]
pub struct FMovieSceneBindingPropertyInfo {
    pub locator: crate::bindings::universal_object_locator::FUniversalObjectLocator,
    pub resolve_flags: crate::bindings::universal_object_locator::ELocatorResolveFlags,
    pub custom_binding: UPtr<crate::bindings::movie_scene::UMovieSceneCustomBinding>,
}
#[repr(C, align(8))]
pub struct FLevelSequencePropertyTrackSettings {
    pub component_path: FString,
    pub property_path: FString,
}
#[repr(C, align(8))]
pub struct FLevelSequenceTrackSettings {
    pub matching_actor_class: crate::bindings::core_u_object::FSoftClassPath,
    pub default_tracks: TArray<crate::bindings::core_u_object::FSoftClassPath>,
    pub exclude_default_tracks: TArray<crate::bindings::core_u_object::FSoftClassPath>,
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
    pub track_row_metadata_helper_list: TArray<
        UPtr<crate::bindings::movie_scene_tools::UMovieSceneTrackRowMetadataHelper>,
    >,
    pub curve_editor_array: TArray<
        UPtr<crate::bindings::sequencer_scripting_editor::USequencerCurveEditorObject>,
    >,
}
pub struct ULevelSequenceEditorSettings {
    pub track_settings: TArray<FLevelSequenceTrackSettings>,
    pub b_auto_bind_to_pie: bool,
    pub b_auto_bind_to_simulate: bool,
}
pub struct ULevelSequenceWithShotsSettings {
    pub name: FString,
    pub suffix: FString,
    pub base_path: crate::bindings::core_u_object::FDirectoryPath,
    pub num_shots: u32,
    pub sequence_to_duplicate: TLazyObjectPtr<
        crate::bindings::level_sequence::ULevelSequence,
    >,
    pub sub_sequence_names: TArray<FName>,
    pub b_instance_sub_sequences: bool,
}
