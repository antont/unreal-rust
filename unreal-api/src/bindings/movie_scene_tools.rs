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
    __padding_end: [u8; 3],
}
impl FBakingAnimationKeySettings {}
#[repr(C, align(1))]
pub struct FControlToTransformMappings {
    __padding_end: [u8; 3],
}
impl FControlToTransformMappings {}
#[repr(C, align(8))]
pub struct FControlFindReplaceString {
    __padding_end: [u8; 32],
}
impl FControlFindReplaceString {}
#[repr(C, align(8))]
pub struct UMovieSceneTextKeyStruct {
    __padding_end: [u8; 416],
}
impl UMovieSceneTextKeyStruct {}
#[repr(C, align(8))]
pub struct USequencerExportTask {
    #[doc(hidden)]
    __padding_128: [u8; 128],
    pub sequencer_context: UPtr<crate::bindings::core_u_object::UObject>,
}
impl USequencerExportTask {}
#[repr(C, align(8))]
pub struct UAutomatedLevelSequenceCapture {
    #[doc(hidden)]
    __padding_600: [u8; 600],
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
    __padding_end: [u8; 326],
}
impl UAutomatedLevelSequenceCapture {}
#[repr(C, align(8))]
pub struct UBoolChannelKeyProxy {
    __padding_end: [u8; 120],
}
impl UBoolChannelKeyProxy {}
#[repr(C, align(8))]
pub struct UByteChannelKeyProxy {
    __padding_end: [u8; 120],
}
impl UByteChannelKeyProxy {}
#[repr(C, align(8))]
pub struct UDoubleChannelKeyProxy {
    __padding_end: [u8; 152],
}
impl UDoubleChannelKeyProxy {}
#[repr(C, align(8))]
pub struct UFloatChannelKeyProxy {
    __padding_end: [u8; 144],
}
impl UFloatChannelKeyProxy {}
#[repr(C, align(8))]
pub struct UIntegerChannelKeyProxy {
    __padding_end: [u8; 120],
}
impl UIntegerChannelKeyProxy {}
#[repr(C, align(8))]
pub struct UMovieSceneTrackRowMetadataHelper {
    __padding_end: [u8; 64],
}
impl UMovieSceneTrackRowMetadataHelper {}
#[repr(C, align(8))]
pub struct UMovieSceneDirectorBlueprintConditionExtension {
    __padding_end: [u8; 64],
}
impl UMovieSceneDirectorBlueprintConditionExtension {}
#[repr(C, align(8))]
pub struct UMovieSceneDirectorBlueprintConditionEndpointUtil {
    __padding_end: [u8; 48],
}
impl UMovieSceneDirectorBlueprintConditionEndpointUtil {}
#[repr(C, align(8))]
pub struct UK2Node_GetSequenceBinding {
    __padding_end: [u8; 672],
}
impl UK2Node_GetSequenceBinding {}
#[repr(C, align(8))]
pub struct UMovieSceneDynamicBindingBlueprintExtension {
    __padding_end: [u8; 72],
}
impl UMovieSceneDynamicBindingBlueprintExtension {}
#[repr(C, align(8))]
pub struct UMovieSceneDynamicBindingEndpointUtil {
    __padding_end: [u8; 48],
}
impl UMovieSceneDynamicBindingEndpointUtil {}
#[repr(C, align(8))]
pub struct UMovieSceneEventBlueprintExtension {
    __padding_end: [u8; 64],
}
impl UMovieSceneEventBlueprintExtension {}
#[repr(C, align(8))]
pub struct UMovieSceneToolsProjectSettings {
    __padding_end: [u8; 176],
}
impl UMovieSceneToolsProjectSettings {}
#[repr(C, align(8))]
pub struct UMovieSceneUserThumbnailSettings {
    __padding_end: [u8; 152],
}
impl UMovieSceneUserThumbnailSettings {}
#[repr(C, align(8))]
pub struct UMovieSceneUserImportFBXSettings {
    __padding_end: [u8; 64],
}
impl UMovieSceneUserImportFBXSettings {}
#[repr(C, align(8))]
pub struct UMovieSceneUserImportFBXControlRigSettings {
    __padding_end: [u8; 168],
}
impl UMovieSceneUserImportFBXControlRigSettings {}
#[repr(C, align(8))]
pub struct UMovieSceneUserExportFBXControlRigSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub fbx_export_compatibility: crate::bindings::unreal_ed::EFbxExportCompatibility,
    #[doc(hidden)]
    __padding_68: [u8; 3],
    pub flags_68: u8,
    #[doc(hidden)]
    __padding_96: [u8; 27],
    pub flags_96: u8,
    __padding_end: [u8; 7],
}
impl UMovieSceneUserExportFBXControlRigSettings {}
#[repr(transparent)]
pub struct EBakingKeySettings(pub u8);
impl EBakingKeySettings {
    pub const KEYS_ONLY: EBakingKeySettings = EBakingKeySettings(0);
    pub const ALL_FRAMES: EBakingKeySettings = EBakingKeySettings(1);
}
#[repr(transparent)]
pub struct ECreationTime(pub u8);
impl ECreationTime {
    pub const CURRENT_FRAME: ECreationTime = ECreationTime(0);
    pub const FROM_START: ECreationTime = ECreationTime(1);
    pub const INFINITE: ECreationTime = ECreationTime(2);
}
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
#[repr(transparent)]
pub struct EThumbnailQuality(pub u8);
impl EThumbnailQuality {
    pub const DRAFT: EThumbnailQuality = EThumbnailQuality(0);
    pub const NORMAL: EThumbnailQuality = EThumbnailQuality(1);
    pub const BEST: EThumbnailQuality = EThumbnailQuality(2);
}
