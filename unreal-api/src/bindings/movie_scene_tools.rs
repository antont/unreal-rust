#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FBakingAnimationKeySettings {
    pub start_frame: FFrameNumber,
    pub end_frame: FFrameNumber,
    pub baking_key_settings: EBakingKeySettings,
    pub frame_increment: i32,
    pub b_reduce_keys: bool,
    pub tolerance: f32,
    pub b_time_warp: bool,
}
#[repr(C, align(1))]
pub struct FSequencerCreationOptions {
    pub creation_time: ECreationTime,
}
#[repr(C, align(8))]
pub struct FMovieSceneToolsPropertyTrackSettings {
    pub component_name: FString,
    pub property_name: FString,
}
#[repr(C, align(8))]
pub struct FMovieSceneToolsFbxSettings {
    pub fbx_property_name: FString,
    pub property_path: FMovieSceneToolsPropertyTrackSettings,
    pub property_type: EMovieSceneToolsPropertyTrackType,
}
#[repr(C, align(1))]
pub struct FControlToTransformMappings {
    pub control_channel: FControlRigChannelEnum,
    pub fbx_channel: FTransformChannelEnum,
    pub b_negate: bool,
}
#[repr(C, align(8))]
pub struct FControlFindReplaceString {
    pub find: FString,
    pub replace: FString,
}
pub struct UMovieSceneTextKeyStruct {}
pub struct USequencerExportTask {
    pub sequencer_context: UPtr<UObject>,
}
pub struct UAutomatedLevelSequenceCapture {
    pub level_sequence_asset: FSoftObjectPath,
    pub shot_name: FString,
    pub b_use_custom_start_frame: bool,
    pub custom_start_frame: FFrameNumber,
    pub b_use_custom_end_frame: bool,
    pub custom_end_frame: FFrameNumber,
    pub warm_up_frame_count: i32,
    pub delay_before_warm_up: f32,
    pub delay_before_shot_warm_up: f32,
    pub delay_every_frame: f32,
    pub burn_in_options: UPtr<ULevelSequenceBurnInOptions>,
    pub b_write_edit_decision_list: bool,
    pub b_write_final_cut_pro_xml: bool,
    pub level_sequence_actor: TWeakObjectPtr<ALevelSequenceActor>,
}
pub struct UBoolChannelKeyProxy {
    pub time: FFrameNumber,
    pub b_value: bool,
}
pub struct UByteChannelKeyProxy {
    pub time: FFrameNumber,
    pub value: u8,
}
pub struct UDoubleChannelKeyProxy {
    pub time: FFrameNumber,
    pub value: FMovieSceneDoubleValue,
}
pub struct UFloatChannelKeyProxy {
    pub time: FFrameNumber,
    pub value: FMovieSceneFloatValue,
}
pub struct UIntegerChannelKeyProxy {
    pub time: FFrameNumber,
    pub value: i32,
}
pub struct UMovieSceneTrackRowMetadataHelper {
    pub track_row_metadata: FMovieSceneTrackRowMetadata,
    pub owner_track: TWeakObjectPtr<UMovieSceneTrack>,
}
pub struct UMovieSceneDirectorBlueprintConditionExtension {
    pub weak_movie_scene_sequences: TArray<TWeakObjectPtr<UMovieSceneSequence>>,
}
pub struct UMovieSceneDirectorBlueprintConditionEndpointUtil {}
pub struct UK2Node_GetSequenceBinding {
    pub source_sequence_deprecated: FSoftObjectPath,
    pub binding: FMovieSceneObjectBindingID,
    pub source_movie_sequence: UPtr<UMovieSceneSequence>,
}
pub struct UMovieSceneDynamicBindingBlueprintExtension {
    pub weak_movie_scene_sequences: TArray<TWeakObjectPtr<UMovieSceneSequence>>,
    pub weak_movie_scene_sequence_deprecated: TWeakObjectPtr<UMovieSceneSequence>,
}
pub struct UMovieSceneDynamicBindingEndpointUtil {}
pub struct UMovieSceneEventBlueprintExtension {
    pub event_sections: TArray<TWeakObjectPtr<UMovieSceneEventSectionBase>>,
}
pub struct UMovieSceneToolsProjectSettings {
    pub default_start_time: f32,
    pub default_duration: f32,
    pub subsequence_directory: FString,
    pub shot_directory: FString,
    pub subsequence_prefix: FString,
    pub shot_prefix: FString,
    pub first_shot_number: u32,
    pub shot_increment: u32,
    pub shot_num_digits: u32,
    pub take_num_digits: u32,
    pub first_take_number: u32,
    pub take_separator: FString,
    pub fbx_settings: TArray<FMovieSceneToolsFbxSettings>,
}
pub struct UMovieSceneUserThumbnailSettings {
    pub b_draw_thumbnails: bool,
    pub b_draw_single_thumbnails: bool,
    pub thumbnail_size: FIntPoint,
    pub quality: EThumbnailQuality,
}
pub struct UMovieSceneUserImportFBXSettings {
    pub b_match_by_name_only: bool,
    pub b_force_front_x_axis: bool,
    pub b_convert_scene_unit: bool,
    pub import_uniform_scale: f32,
    pub b_create_cameras: bool,
    pub b_replace_transform_track: bool,
    pub b_correct_for_transform_origin: bool,
    pub b_reduce_keys: bool,
    pub reduce_keys_tolerance: f32,
}
pub struct UMovieSceneUserImportFBXControlRigSettings {
    pub imported_file_name: FString,
    pub imported_start_time: FFrameNumber,
    pub imported_end_time: FFrameNumber,
    pub imported_node_names: TArray<FString>,
    pub imported_frame_rate: FString,
    pub find_and_replace_strings: TArray<FControlFindReplaceString>,
    pub b_strip_namespace: bool,
    pub b_force_front_x_axis: bool,
    pub b_convert_scene_unit: bool,
    pub import_uniform_scale: f32,
    pub b_import_onto_selected_controls: bool,
    pub time_to_insert_or_replace_animation: FFrameNumber,
    pub b_insert_animation: bool,
    pub b_specify_time_range: bool,
    pub start_time_range: FFrameNumber,
    pub end_time_range: FFrameNumber,
    pub control_channel_mappings: TArray<FControlToTransformMappings>,
}
pub struct UMovieSceneUserExportFBXControlRigSettings {
    pub export_file_name: FString,
    pub fbx_export_compatibility: EFbxExportCompatibility,
    pub flags_68: u8,
    pub b_force_front_x_axis: bool,
    pub b_export_only_selected_controls: bool,
    pub control_channel_mappings: TArray<FControlToTransformMappings>,
    pub flags_96: u8,
}
