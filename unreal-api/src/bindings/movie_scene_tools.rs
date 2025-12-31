#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FBakingAnimationKeySettings {
    pub start_frame: crate::bindings::core_u_object::FFrameNumber,
    pub end_frame: crate::bindings::core_u_object::FFrameNumber,
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
    pub sequencer_context: UPtr<crate::bindings::core_u_object::UObject>,
}
pub struct UAutomatedLevelSequenceCapture {
    pub level_sequence_asset: crate::bindings::core_u_object::FSoftObjectPath,
    pub shot_name: FString,
    pub b_use_custom_start_frame: bool,
    pub custom_start_frame: crate::bindings::core_u_object::FFrameNumber,
    pub b_use_custom_end_frame: bool,
    pub custom_end_frame: crate::bindings::core_u_object::FFrameNumber,
    pub warm_up_frame_count: i32,
    pub delay_before_warm_up: f32,
    pub delay_before_shot_warm_up: f32,
    pub delay_every_frame: f32,
    pub burn_in_options: UPtr<
        crate::bindings::level_sequence::ULevelSequenceBurnInOptions,
    >,
    pub b_write_edit_decision_list: bool,
    pub b_write_final_cut_pro_xml: bool,
    pub level_sequence_actor: TWeakObjectPtr<
        crate::bindings::level_sequence::ALevelSequenceActor,
    >,
}
pub struct UBoolChannelKeyProxy {
    pub time: crate::bindings::core_u_object::FFrameNumber,
    pub b_value: bool,
}
pub struct UByteChannelKeyProxy {
    pub time: crate::bindings::core_u_object::FFrameNumber,
    pub value: u8,
}
pub struct UDoubleChannelKeyProxy {
    pub time: crate::bindings::core_u_object::FFrameNumber,
    pub value: crate::bindings::movie_scene::FMovieSceneDoubleValue,
}
pub struct UFloatChannelKeyProxy {
    pub time: crate::bindings::core_u_object::FFrameNumber,
    pub value: crate::bindings::movie_scene::FMovieSceneFloatValue,
}
pub struct UIntegerChannelKeyProxy {
    pub time: crate::bindings::core_u_object::FFrameNumber,
    pub value: i32,
}
pub struct UMovieSceneTrackRowMetadataHelper {
    pub track_row_metadata: crate::bindings::movie_scene::FMovieSceneTrackRowMetadata,
    pub owner_track: TWeakObjectPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
}
pub struct UMovieSceneDirectorBlueprintConditionExtension {
    pub weak_movie_scene_sequences: TArray<
        TWeakObjectPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    >,
}
pub struct UMovieSceneDirectorBlueprintConditionEndpointUtil {}
pub struct UK2Node_GetSequenceBinding {
    pub source_sequence_deprecated: crate::bindings::core_u_object::FSoftObjectPath,
    pub binding: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
    pub source_movie_sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
}
pub struct UMovieSceneDynamicBindingBlueprintExtension {
    pub weak_movie_scene_sequences: TArray<
        TWeakObjectPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    >,
    pub weak_movie_scene_sequence_deprecated: TWeakObjectPtr<
        crate::bindings::movie_scene::UMovieSceneSequence,
    >,
}
pub struct UMovieSceneDynamicBindingEndpointUtil {}
pub struct UMovieSceneEventBlueprintExtension {
    pub event_sections: TArray<
        TWeakObjectPtr<crate::bindings::movie_scene_tracks::UMovieSceneEventSectionBase>,
    >,
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
    pub thumbnail_size: crate::bindings::core_u_object::FIntPoint,
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
    pub imported_start_time: crate::bindings::core_u_object::FFrameNumber,
    pub imported_end_time: crate::bindings::core_u_object::FFrameNumber,
    pub imported_node_names: TArray<FString>,
    pub imported_frame_rate: FString,
    pub find_and_replace_strings: TArray<FControlFindReplaceString>,
    pub b_strip_namespace: bool,
    pub b_force_front_x_axis: bool,
    pub b_convert_scene_unit: bool,
    pub import_uniform_scale: f32,
    pub b_import_onto_selected_controls: bool,
    pub time_to_insert_or_replace_animation: crate::bindings::core_u_object::FFrameNumber,
    pub b_insert_animation: bool,
    pub b_specify_time_range: bool,
    pub start_time_range: crate::bindings::core_u_object::FFrameNumber,
    pub end_time_range: crate::bindings::core_u_object::FFrameNumber,
    pub control_channel_mappings: TArray<FControlToTransformMappings>,
}
pub struct UMovieSceneUserExportFBXControlRigSettings {
    pub export_file_name: FString,
    pub fbx_export_compatibility: crate::bindings::unreal_ed::EFbxExportCompatibility,
    pub flags_68: u8,
    pub b_force_front_x_axis: bool,
    pub b_export_only_selected_controls: bool,
    pub control_channel_mappings: TArray<FControlToTransformMappings>,
    pub flags_96: u8,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBakingKeySettings(pub u8);
impl EBakingKeySettings {
    pub const KEYS_ONLY: EBakingKeySettings = EBakingKeySettings(0);
    pub const ALL_FRAMES: EBakingKeySettings = EBakingKeySettings(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECreationTime(pub u8);
impl ECreationTime {
    pub const CURRENT_FRAME: ECreationTime = ECreationTime(0);
    pub const FROM_START: ECreationTime = ECreationTime(1);
    pub const INFINITE: ECreationTime = ECreationTime(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMovieSceneToolsPropertyTrackType(pub i32);
impl EMovieSceneToolsPropertyTrackType {
    pub const FLOAT_TRACK: EMovieSceneToolsPropertyTrackType = EMovieSceneToolsPropertyTrackType(
        0,
    );
    pub const DOUBLE_TRACK: EMovieSceneToolsPropertyTrackType = EMovieSceneToolsPropertyTrackType(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct FControlRigChannelEnum(pub u8);
impl FControlRigChannelEnum {
    pub const BOOL: FControlRigChannelEnum = FControlRigChannelEnum(0);
    pub const ENUM: FControlRigChannelEnum = FControlRigChannelEnum(1);
    pub const INTEGER: FControlRigChannelEnum = FControlRigChannelEnum(2);
    pub const FLOAT: FControlRigChannelEnum = FControlRigChannelEnum(3);
    pub const VECTOR2_DX: FControlRigChannelEnum = FControlRigChannelEnum(4);
    pub const VECTOR2_DY: FControlRigChannelEnum = FControlRigChannelEnum(5);
    pub const POSITION_X: FControlRigChannelEnum = FControlRigChannelEnum(6);
    pub const POSITION_Y: FControlRigChannelEnum = FControlRigChannelEnum(7);
    pub const POSITION_Z: FControlRigChannelEnum = FControlRigChannelEnum(8);
    pub const ROTATOR_X: FControlRigChannelEnum = FControlRigChannelEnum(9);
    pub const ROTATOR_Y: FControlRigChannelEnum = FControlRigChannelEnum(10);
    pub const ROTATOR_Z: FControlRigChannelEnum = FControlRigChannelEnum(11);
    pub const SCALE_X: FControlRigChannelEnum = FControlRigChannelEnum(12);
    pub const SCALE_Y: FControlRigChannelEnum = FControlRigChannelEnum(13);
    pub const SCALE_Z: FControlRigChannelEnum = FControlRigChannelEnum(14);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct FTransformChannelEnum(pub u8);
impl FTransformChannelEnum {
    pub const TRANSLATE_X: FTransformChannelEnum = FTransformChannelEnum(0);
    pub const TRANSLATE_Y: FTransformChannelEnum = FTransformChannelEnum(1);
    pub const TRANSLATE_Z: FTransformChannelEnum = FTransformChannelEnum(2);
    pub const ROTATE_X: FTransformChannelEnum = FTransformChannelEnum(3);
    pub const ROTATE_Y: FTransformChannelEnum = FTransformChannelEnum(4);
    pub const ROTATE_Z: FTransformChannelEnum = FTransformChannelEnum(5);
    pub const SCALE_X: FTransformChannelEnum = FTransformChannelEnum(6);
    pub const SCALE_Y: FTransformChannelEnum = FTransformChannelEnum(7);
    pub const SCALE_Z: FTransformChannelEnum = FTransformChannelEnum(8);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EThumbnailQuality(pub u8);
impl EThumbnailQuality {
    pub const DRAFT: EThumbnailQuality = EThumbnailQuality(0);
    pub const NORMAL: EThumbnailQuality = EThumbnailQuality(1);
    pub const BEST: EThumbnailQuality = EThumbnailQuality(2);
}
