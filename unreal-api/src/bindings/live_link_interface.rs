#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FLiveLinkSourceHandle {
    __padding_end: [u8; 24],
}
impl FLiveLinkSourceHandle {}
#[repr(C, align(8))]
pub struct FLiveLinkBaseStaticData {
    pub property_names: TArray<FName>,
}
impl FLiveLinkBaseStaticData {}
#[repr(C, align(8))]
pub struct FLiveLinkSkeletonStaticData {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub bone_names: TArray<FName>,
    pub bone_parents: TArray<i32>,
}
impl FLiveLinkSkeletonStaticData {}
#[repr(C, align(8))]
pub struct FLiveLinkBaseFrameData {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub meta_data: FLiveLinkMetaData,
    pub property_values: TArray<f32>,
    __padding_end: [u8; 112],
}
impl FLiveLinkBaseFrameData {}
#[repr(C, align(8))]
pub struct FLiveLinkMetaData {
    pub string_meta_data: TMap<FName, FString>,
    pub scene_time: crate::bindings::core_u_object::FQualifiedFrameTime,
}
impl FLiveLinkMetaData {}
#[repr(C, align(8))]
pub struct FLiveLinkAnimationFrameData {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
}
impl FLiveLinkAnimationFrameData {}
#[repr(C, align(8))]
pub struct FLiveLinkFloatAnimationFrameData {
    __padding_end: [u8; 256],
}
impl FLiveLinkFloatAnimationFrameData {}
#[repr(C, align(8))]
pub struct FLiveLinkBaseBlueprintData {
    __padding_end: [u8; 8],
}
impl FLiveLinkBaseBlueprintData {}
#[repr(C, align(8))]
pub struct FLiveLinkBasicBlueprintData {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub static_data: FLiveLinkBaseStaticData,
    pub frame_data: FLiveLinkBaseFrameData,
}
impl FLiveLinkBasicBlueprintData {}
#[repr(C, align(8))]
pub struct FLiveLinkTransformStaticData {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub b_is_location_supported: bool,
    pub b_is_rotation_supported: bool,
    pub b_is_scale_supported: bool,
    __padding_end: [u8; 5],
}
impl FLiveLinkTransformStaticData {}
#[repr(C, align(8))]
pub struct FLiveLinkCameraStaticData {
    #[doc(hidden)]
    __padding_24: [u8; 24],
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
impl FLiveLinkCameraStaticData {}
#[repr(C, align(16))]
pub struct FLiveLinkTransformFrameData {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub transform: crate::bindings::core_u_object::FTransform,
}
impl FLiveLinkTransformFrameData {}
#[repr(C, align(16))]
pub struct FLiveLinkCameraFrameData {
    #[doc(hidden)]
    __padding_336: [u8; 336],
    pub field_of_view: f32,
    pub aspect_ratio: f32,
    pub focal_length: f32,
    pub aperture: f32,
    pub focus_distance: f32,
    pub projection_mode: ELiveLinkCameraProjectionMode,
    pub film_back_width: f32,
    pub film_back_height: f32,
}
impl FLiveLinkCameraFrameData {}
#[repr(C, align(16))]
pub struct FLiveLinkCameraBlueprintData {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub static_data: FLiveLinkCameraStaticData,
    pub frame_data: FLiveLinkCameraFrameData,
}
impl FLiveLinkCameraBlueprintData {}
#[repr(C, align(8))]
pub struct FLiveLinkLightStaticData {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub b_is_temperature_supported: bool,
    pub b_is_intensity_supported: bool,
    pub b_is_light_color_supported: bool,
    pub b_is_inner_cone_angle_supported: bool,
    pub b_is_outer_cone_angle_supported: bool,
    pub b_is_attenuation_radius_supported: bool,
    pub b_is_source_lenght_supported: bool,
    pub b_is_source_radius_supported: bool,
    pub b_is_soft_source_radius_supported: bool,
    __padding_end: [u8; 7],
}
impl FLiveLinkLightStaticData {}
#[repr(C, align(16))]
pub struct FLiveLinkLightFrameData {
    #[doc(hidden)]
    __padding_336: [u8; 336],
    pub temperature: f32,
    pub intensity: f32,
    pub light_color: crate::bindings::core_u_object::FColor,
    pub inner_cone_angle: f32,
    pub outer_cone_angle: f32,
    pub attenuation_radius: f32,
    pub source_radius: f32,
    pub soft_source_radius: f32,
    pub source_length: f32,
    __padding_end: [u8; 12],
}
impl FLiveLinkLightFrameData {}
#[repr(C, align(16))]
pub struct FLiveLinkLightBlueprintData {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub static_data: FLiveLinkLightStaticData,
    pub frame_data: FLiveLinkLightFrameData,
}
impl FLiveLinkLightBlueprintData {}
#[repr(C, align(4))]
pub struct FLiveLinkSubjectKey {
    pub source: crate::bindings::core_u_object::FGuid,
    pub subject_name: FLiveLinkSubjectName,
}
impl FLiveLinkSubjectKey {}
#[repr(C, align(4))]
pub struct FLiveLinkSubjectName {
    pub name: FName,
}
impl FLiveLinkSubjectName {}
#[repr(C, align(8))]
pub struct FLiveLinkSubjectRepresentation {
    pub subject: FLiveLinkSubjectName,
    pub role: TSubclassOf<ULiveLinkRole>,
}
impl FLiveLinkSubjectRepresentation {}
#[repr(C, align(16))]
pub struct FLiveLinkTransformBlueprintData {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub static_data: FLiveLinkTransformStaticData,
    pub frame_data: FLiveLinkTransformFrameData,
}
impl FLiveLinkTransformBlueprintData {}
#[repr(C, align(8))]
pub struct FLiveLinkCurveConversionSettings {
    __padding_end: [u8; 80],
}
impl FLiveLinkCurveConversionSettings {}
#[repr(C, align(8))]
pub struct FLiveLinkTime {
    __padding_end: [u8; 24],
}
impl FLiveLinkTime {}
#[repr(C, align(4))]
pub struct FLiveLinkFrameRate {
    __padding_end: [u8; 8],
}
impl FLiveLinkFrameRate {}
#[repr(C, align(8))]
pub struct FSubjectMetadata {
    pub string_metadata: TMap<FName, FString>,
    pub scene_timecode: crate::bindings::core_u_object::FTimecode,
    pub scene_framerate: crate::bindings::core_u_object::FFrameRate,
}
impl FSubjectMetadata {}
#[repr(C, align(8))]
pub struct FLiveLinkTransform {
    __padding_end: [u8; 32],
}
impl FLiveLinkTransform {}
#[repr(C, align(8))]
pub struct FSubjectFrameHandle {
    __padding_end: [u8; 24],
}
impl FSubjectFrameHandle {}
#[repr(C, align(8))]
pub struct FLiveLinkGamepadInputDeviceStaticData {
    __padding_end: [u8; 16],
}
impl FLiveLinkGamepadInputDeviceStaticData {}
#[repr(C, align(8))]
pub struct FLiveLinkGamepadInputDeviceFrameData {
    #[doc(hidden)]
    __padding_240: [u8; 240],
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
impl FLiveLinkGamepadInputDeviceFrameData {}
#[repr(C, align(8))]
pub struct FLiveLinkGamepadInputDeviceBlueprintData {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub static_data: FLiveLinkGamepadInputDeviceStaticData,
    pub frame_data: FLiveLinkGamepadInputDeviceFrameData,
}
impl FLiveLinkGamepadInputDeviceBlueprintData {}
#[repr(C, align(8))]
pub struct FLiveLinkLocatorStaticData {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub locator_names: TArray<FName>,
    pub b_unlabelled_data: bool,
    __padding_end: [u8; 7],
}
impl FLiveLinkLocatorStaticData {}
#[repr(C, align(8))]
pub struct FLiveLinkLocatorFrameData {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub locators: TArray<crate::bindings::core_u_object::FVector>,
}
impl FLiveLinkLocatorFrameData {}
#[repr(C, align(8))]
pub struct FLiveLinkLocatorBlueprintData {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub static_data: FLiveLinkLocatorStaticData,
    pub frame_data: FLiveLinkLocatorFrameData,
}
impl FLiveLinkLocatorBlueprintData {}
#[repr(C, align(8))]
pub struct ULiveLinkController {
    __padding_end: [u8; 48],
}
impl ULiveLinkController {}
#[repr(C, align(8))]
pub struct ULiveLinkFrameInterpolationProcessor {
    __padding_end: [u8; 48],
}
impl ULiveLinkFrameInterpolationProcessor {}
#[repr(C, align(8))]
pub struct ULiveLinkFramePreProcessor {
    __padding_end: [u8; 48],
}
impl ULiveLinkFramePreProcessor {}
#[repr(C, align(8))]
pub struct ULiveLinkRole {
    __padding_end: [u8; 48],
}
impl ULiveLinkRole {}
#[repr(C, align(8))]
pub struct ULiveLinkSubjectRemapper {
    __padding_end: [u8; 136],
}
impl ULiveLinkSubjectRemapper {}
#[repr(C, align(8))]
pub struct ULiveLinkSourceSettings {
    __padding_end: [u8; 224],
}
impl ULiveLinkSourceSettings {}
#[repr(C, align(8))]
pub struct ULiveLinkCurveRemapSettings {
    __padding_end: [u8; 304],
}
impl ULiveLinkCurveRemapSettings {}
#[repr(C, align(8))]
pub struct ULiveLinkFrameTranslator {
    __padding_end: [u8; 48],
}
impl ULiveLinkFrameTranslator {}
#[repr(C, align(8))]
pub struct ULiveLinkSourceFactory {
    __padding_end: [u8; 48],
}
impl ULiveLinkSourceFactory {}
#[repr(C, align(8))]
pub struct ULiveLinkDefaultSourceSettings {
    __padding_end: [u8; 56],
}
impl ULiveLinkDefaultSourceSettings {}
#[repr(C, align(8))]
pub struct ULiveLinkDefaultSubjectSettings {
    __padding_end: [u8; 56],
}
impl ULiveLinkDefaultSubjectSettings {}
#[repr(C, align(8))]
pub struct ULiveLinkSubjectSettings {
    __padding_end: [u8; 176],
}
impl ULiveLinkSubjectSettings {}
#[repr(C, align(8))]
pub struct ULiveLinkVirtualSubject {
    __padding_end: [u8; 400],
}
impl ULiveLinkVirtualSubject {}
#[repr(C, align(8))]
pub struct ULiveLinkBasicRole {
    __padding_end: [u8; 48],
}
impl ULiveLinkBasicRole {}
#[repr(C, align(8))]
pub struct ULiveLinkAnimationRole {
    __padding_end: [u8; 48],
}
impl ULiveLinkAnimationRole {}
#[repr(C, align(8))]
pub struct ULiveLinkTransformRole {
    __padding_end: [u8; 48],
}
impl ULiveLinkTransformRole {}
#[repr(C, align(8))]
pub struct ULiveLinkCameraRole {
    __padding_end: [u8; 48],
}
impl ULiveLinkCameraRole {}
#[repr(C, align(8))]
pub struct ULiveLinkInputDeviceRole {
    __padding_end: [u8; 48],
}
impl ULiveLinkInputDeviceRole {}
#[repr(C, align(8))]
pub struct ULiveLinkLightRole {
    __padding_end: [u8; 48],
}
impl ULiveLinkLightRole {}
#[repr(C, align(8))]
pub struct ULiveLinkLocatorRole {
    __padding_end: [u8; 48],
}
impl ULiveLinkLocatorRole {}
#[repr(transparent)]
pub struct ELiveLinkCameraProjectionMode(pub u8);
impl ELiveLinkCameraProjectionMode {
    pub const PERSPECTIVE: ELiveLinkCameraProjectionMode = ELiveLinkCameraProjectionMode(
        0,
    );
    pub const ORTHOGRAPHIC: ELiveLinkCameraProjectionMode = ELiveLinkCameraProjectionMode(
        1,
    );
}
#[repr(transparent)]
pub struct ELiveLinkSourceMode(pub u8);
impl ELiveLinkSourceMode {
    pub const LATEST: ELiveLinkSourceMode = ELiveLinkSourceMode(0);
    pub const ENGINE_TIME: ELiveLinkSourceMode = ELiveLinkSourceMode(1);
    pub const TIMECODE: ELiveLinkSourceMode = ELiveLinkSourceMode(2);
}
