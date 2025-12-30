#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FLiveLinkSourceHandle {}
#[repr(C, align(8))]
pub struct FLiveLinkBaseStaticData {
    pub property_names: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FLiveLinkSkeletonStaticData {
    pub bone_names: TArray<FName>,
    pub bone_parents: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FLiveLinkBaseFrameData {
    pub world_time: FLiveLinkWorldTime,
    pub meta_data: FLiveLinkMetaData,
    pub property_values: TArray<f32>,
    pub timestamps: TMap<FName, f64>,
}
#[repr(C, align(8))]
pub struct FLiveLinkMetaData {
    pub string_meta_data: TMap<FName, FString>,
    pub scene_time: FQualifiedFrameTime,
}
#[repr(C, align(8))]
pub struct FLiveLinkWorldTime {
    pub time: f64,
    pub offset: f64,
}
#[repr(C, align(8))]
pub struct FLiveLinkAnimationFrameData {
    pub transforms: TArray<FTransform>,
}
#[repr(C, align(8))]
pub struct FLiveLinkFloatAnimationFrameData {
    pub transforms: TArray<FTransform3f>,
}
#[repr(C, align(8))]
pub struct FLiveLinkBaseBlueprintData {}
#[repr(C, align(8))]
pub struct FLiveLinkBasicBlueprintData {
    pub static_data: FLiveLinkBaseStaticData,
    pub frame_data: FLiveLinkBaseFrameData,
}
#[repr(C, align(8))]
pub struct FLiveLinkTransformStaticData {
    pub b_is_location_supported: bool,
    pub b_is_rotation_supported: bool,
    pub b_is_scale_supported: bool,
}
#[repr(C, align(8))]
pub struct FLiveLinkCameraStaticData {
    pub b_is_field_of_view_supported: bool,
    pub b_is_aspect_ratio_supported: bool,
    pub b_is_focal_length_supported: bool,
    pub b_is_projection_mode_supported: bool,
    pub film_back_width: f32,
    pub film_back_height: f32,
    pub b_is_aperture_supported: bool,
    pub b_is_focus_distance_supported: bool,
    pub b_is_depth_of_field_supported: bool,
    pub b_is_dynamic_filmback_supported: bool,
}
#[repr(C, align(16))]
pub struct FLiveLinkTransformFrameData {
    pub transform: FTransform,
}
#[repr(C, align(16))]
pub struct FLiveLinkCameraFrameData {
    pub field_of_view: f32,
    pub aspect_ratio: f32,
    pub focal_length: f32,
    pub aperture: f32,
    pub focus_distance: f32,
    pub projection_mode: ELiveLinkCameraProjectionMode,
    pub film_back_width: f32,
    pub film_back_height: f32,
}
#[repr(C, align(16))]
pub struct FLiveLinkCameraBlueprintData {
    pub static_data: FLiveLinkCameraStaticData,
    pub frame_data: FLiveLinkCameraFrameData,
}
#[repr(C, align(8))]
pub struct FLiveLinkLightStaticData {
    pub b_is_temperature_supported: bool,
    pub b_is_intensity_supported: bool,
    pub b_is_light_color_supported: bool,
    pub b_is_inner_cone_angle_supported: bool,
    pub b_is_outer_cone_angle_supported: bool,
    pub b_is_attenuation_radius_supported: bool,
    pub b_is_source_lenght_supported: bool,
    pub b_is_source_radius_supported: bool,
    pub b_is_soft_source_radius_supported: bool,
}
#[repr(C, align(16))]
pub struct FLiveLinkLightFrameData {
    pub temperature: f32,
    pub intensity: f32,
    pub light_color: FColor,
    pub inner_cone_angle: f32,
    pub outer_cone_angle: f32,
    pub attenuation_radius: f32,
    pub source_radius: f32,
    pub soft_source_radius: f32,
    pub source_length: f32,
}
#[repr(C, align(16))]
pub struct FLiveLinkLightBlueprintData {
    pub static_data: FLiveLinkLightStaticData,
    pub frame_data: FLiveLinkLightFrameData,
}
#[repr(C, align(8))]
pub struct FLiveLinkSourcePreset {
    pub guid: FGuid,
    pub settings: UPtr<ULiveLinkSourceSettings>,
    pub source_type: FText,
}
#[repr(C, align(8))]
pub struct FLiveLinkSubjectPreset {
    pub key: FLiveLinkSubjectKey,
    pub role: TSubclassOf<ULiveLinkRole>,
    pub settings: UPtr<ULiveLinkSubjectSettings>,
    pub virtual_subject: UPtr<ULiveLinkVirtualSubject>,
    pub b_enabled: bool,
}
#[repr(C, align(4))]
pub struct FLiveLinkSubjectKey {
    pub source: FGuid,
    pub subject_name: FLiveLinkSubjectName,
}
#[repr(C, align(4))]
pub struct FLiveLinkSubjectName {
    pub name: FName,
}
#[repr(C, align(8))]
pub struct FLiveLinkRefSkeleton {
    pub bone_names: TArray<FName>,
    pub bone_parents: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FLiveLinkSubjectRepresentation {
    pub subject: FLiveLinkSubjectName,
    pub role: TSubclassOf<ULiveLinkRole>,
}
#[repr(C, align(16))]
pub struct FLiveLinkTransformBlueprintData {
    pub static_data: FLiveLinkTransformStaticData,
    pub frame_data: FLiveLinkTransformFrameData,
}
#[repr(C, align(8))]
pub struct FLiveLinkCurveConversionSettings {
    pub curve_conversion_asset_map: TMap<FString, FSoftObjectPath>,
}
#[repr(C, align(8))]
pub struct FLiveLinkSourceBufferManagementSettings {
    pub b_valid_engine_time_enabled: bool,
    pub valid_engine_time: f32,
    pub engine_time_offset: f32,
    pub engine_time_clock_offset: f64,
    pub smooth_engine_time_offset: f64,
    pub timecode_frame_rate_deprecated: FFrameRate,
    pub b_generate_sub_frame: bool,
    pub detected_frame_rate: FFrameRate,
    pub b_use_timecode_smooth_latest: bool,
    pub source_timecode_frame_rate: FFrameRate,
    pub b_valid_timecode_frame_enabled: bool,
    pub valid_timecode_frame: i32,
    pub timecode_frame_offset: f32,
    pub timecode_clock_offset: f64,
    pub latest_offset: i32,
    pub max_number_of_frame_to_buffered: i32,
    pub b_keep_at_least_one_frame: bool,
}
#[repr(C, align(4))]
pub struct FLiveLinkSourceDebugInfo {
    pub subject_name: FLiveLinkSubjectName,
    pub snapshot_index: i32,
    pub number_of_buffer_at_snapshot: i32,
}
#[repr(C, align(4))]
pub struct FLiveLinkTimeSynchronizationSettings {
    pub frame_rate: FFrameRate,
    pub frame_offset: FFrameNumber,
}
#[repr(C, align(4))]
pub struct FLiveLinkInterpolationSettings {
    pub b_use_interpolation_deprecated: bool,
    pub interpolation_offset: f32,
}
#[repr(C, align(8))]
pub struct FLiveLinkTime {
    pub world_time: f64,
    pub scene_time: FQualifiedFrameTime,
}
#[repr(C, align(4))]
pub struct FLiveLinkFrameRate {}
#[repr(C, align(4))]
pub struct FLiveLinkTimeCode_Base_DEPRECATED {
    pub seconds: i32,
    pub frames: i32,
    pub frame_rate: FLiveLinkFrameRate,
}
#[repr(C, align(4))]
pub struct FLiveLinkTimeCode {}
#[repr(C, align(4))]
pub struct FLiveLinkCurveElement {
    pub curve_name: FName,
    pub curve_value: f32,
}
#[repr(C, align(8))]
pub struct FLiveLinkFrameData {
    pub transforms: TArray<FTransform>,
    pub curve_elements: TArray<FLiveLinkCurveElement>,
    pub world_time: FLiveLinkWorldTime,
    pub meta_data: FLiveLinkMetaData,
}
#[repr(C, align(8))]
pub struct FSubjectMetadata {
    pub string_metadata: TMap<FName, FString>,
    pub scene_timecode: FTimecode,
    pub scene_framerate: FFrameRate,
}
#[repr(C, align(8))]
pub struct FCachedSubjectFrame {}
#[repr(C, align(8))]
pub struct FLiveLinkTransform {}
#[repr(C, align(8))]
pub struct FSubjectFrameHandle {}
#[repr(C, align(8))]
pub struct FLiveLinkGamepadInputDeviceStaticData {}
#[repr(C, align(8))]
pub struct FLiveLinkGamepadInputDeviceFrameData {
    pub left_analog_x: f32,
    pub left_analog_y: f32,
    pub right_analog_x: f32,
    pub right_analog_y: f32,
    pub left_trigger_analog: f32,
    pub right_trigger_analog: f32,
    pub left_thumb: f32,
    pub right_thumb: f32,
    pub special_left: f32,
    pub special_left_x: f32,
    pub special_left_y: f32,
    pub special_right: f32,
    pub face_button_bottom: f32,
    pub face_button_right: f32,
    pub face_button_left: f32,
    pub face_button_top: f32,
    pub left_shoulder: f32,
    pub right_shoulder: f32,
    pub left_trigger_threshold: f32,
    pub right_trigger_threshold: f32,
    pub d_pad_up: f32,
    pub d_pad_down: f32,
    pub d_pad_right: f32,
    pub d_pad_left: f32,
    pub left_stick_up: f32,
    pub left_stick_down: f32,
    pub left_stick_right: f32,
    pub left_stick_left: f32,
    pub right_stick_up: f32,
    pub right_stick_down: f32,
    pub right_stick_right: f32,
    pub right_stick_left: f32,
}
#[repr(C, align(8))]
pub struct FLiveLinkGamepadInputDeviceBlueprintData {
    pub static_data: FLiveLinkGamepadInputDeviceStaticData,
    pub frame_data: FLiveLinkGamepadInputDeviceFrameData,
}
#[repr(C, align(8))]
pub struct FLiveLinkLocatorStaticData {
    pub locator_names: TArray<FName>,
    pub b_unlabelled_data: bool,
}
#[repr(C, align(8))]
pub struct FLiveLinkLocatorFrameData {
    pub locators: TArray<FVector>,
}
#[repr(C, align(8))]
pub struct FLiveLinkLocatorBlueprintData {
    pub static_data: FLiveLinkLocatorStaticData,
    pub frame_data: FLiveLinkLocatorFrameData,
}
pub struct ULiveLinkController {}
pub struct ULiveLinkFrameInterpolationProcessor {}
pub struct ULiveLinkFramePreProcessor {}
pub struct ULiveLinkRole {}
pub struct ULiveLinkSubjectRemapper {
    pub bone_name_map: TMap<FName, FName>,
}
pub struct ULiveLinkSourceSettings {
    pub mode: ELiveLinkSourceMode,
    pub buffer_settings: FLiveLinkSourceBufferManagementSettings,
    pub b_transmit_evaluated_data: bool,
    pub connection_string: FString,
    pub factory: TSubclassOf<ULiveLinkSourceFactory>,
    pub parent_subject: FLiveLinkSubjectName,
    pub source_debug_infos_deprecated: TArray<FLiveLinkSourceDebugInfo>,
}
pub struct ULiveLinkCurveRemapSettings {
    pub curve_conversion_settings: FLiveLinkCurveConversionSettings,
}
pub struct ULiveLinkFrameTranslator {}
pub struct ULiveLinkSourceFactory {}
pub struct ULiveLinkDefaultSourceSettings {
    pub default_source_frame_buffer_size: i32,
}
pub struct ULiveLinkDefaultSubjectSettings {
    pub b_rebroadcast_subjects_by_default: bool,
    pub b_allow_editing_rebroadcast_property_deprecated: bool,
}
pub struct ULiveLinkSubjectSettings {
    pub pre_processors: TArray<UPtr<ULiveLinkFramePreProcessor>>,
    pub interpolation_processor: UPtr<ULiveLinkFrameInterpolationProcessor>,
    pub translators: TArray<UPtr<ULiveLinkFrameTranslator>>,
    pub remapper: UPtr<ULiveLinkSubjectRemapper>,
    pub role: TSubclassOf<ULiveLinkRole>,
    pub frame_rate: FFrameRate,
    pub b_rebroadcast_subject: bool,
    pub original_source_name: FName,
    pub key: FLiveLinkSubjectKey,
    pub b_allow_modifying_rebroadcast_deprecated: bool,
}
pub struct ULiveLinkVirtualSubject {
    pub sync_subject: FLiveLinkSubjectName,
    pub role: TSubclassOf<ULiveLinkRole>,
    pub subjects: TArray<FLiveLinkSubjectName>,
    pub frame_translators: TArray<UPtr<ULiveLinkFrameTranslator>>,
    pub b_rebroadcast_subject: bool,
}
pub struct ULiveLinkBasicRole {}
pub struct ULiveLinkAnimationRole {}
pub struct ULiveLinkTransformRole {}
pub struct ULiveLinkCameraRole {}
pub struct ULiveLinkInputDeviceRole {}
pub struct ULiveLinkLightRole {}
pub struct ULiveLinkLocatorRole {}
