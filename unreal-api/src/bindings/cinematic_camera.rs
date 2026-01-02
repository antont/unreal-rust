#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FCameraLookatTrackingSettings {
    pub flags_0: u8,
    pub look_at_tracking_interp_speed: f32,
    #[doc(hidden)]
    __padding_40: [u8; 32],
    pub actor_to_track: TSoftObjectPtr<crate::bindings::engine::AActor>,
    pub relative_offset: crate::bindings::core_u_object::FVector,
    pub flags_112: u8,
    __padding_end: [u8; 7],
}
impl FCameraLookatTrackingSettings {}
#[repr(C, align(4))]
pub struct FCameraFilmbackSettings {
    pub sensor_width: f32,
    pub sensor_height: f32,
    pub sensor_horizontal_offset: f32,
    pub sensor_vertical_offset: f32,
    pub sensor_aspect_ratio: f32,
}
impl FCameraFilmbackSettings {}
#[repr(C, align(8))]
pub struct FNamedFilmbackPreset {
    pub name: FString,
    pub display_name: FText,
    pub filmback_settings: FCameraFilmbackSettings,
    __padding_end: [u8; 4],
}
impl FNamedFilmbackPreset {}
#[repr(C, align(4))]
pub struct FCameraLensSettings {
    pub min_focal_length: f32,
    pub max_focal_length: f32,
    pub min_f_stop: f32,
    pub max_f_stop: f32,
    pub minimum_focus_distance: f32,
    pub squeeze_factor: f32,
    pub diaphragm_blade_count: i32,
}
impl FCameraLensSettings {}
#[repr(C, align(8))]
pub struct FNamedLensPreset {
    pub name: FString,
    pub lens_settings: FCameraLensSettings,
    __padding_end: [u8; 4],
}
impl FNamedLensPreset {}
#[repr(C, align(4))]
pub struct FPlateCropSettings {
    pub aspect_ratio: f32,
}
impl FPlateCropSettings {}
#[repr(C, align(8))]
pub struct FNamedPlateCropPreset {
    pub name: FString,
    pub crop_settings: FPlateCropSettings,
    __padding_end: [u8; 4],
}
impl FNamedPlateCropPreset {}
#[repr(C, align(8))]
pub struct FCameraTrackingFocusSettings {
    pub actor_to_track: TSoftObjectPtr<crate::bindings::engine::AActor>,
    pub relative_offset: crate::bindings::core_u_object::FVector,
    pub flags_72: u8,
    __padding_end: [u8; 7],
}
impl FCameraTrackingFocusSettings {}
#[repr(C, align(8))]
pub struct FCameraFocusSettings {
    pub focus_method: ECameraFocusMethod,
    pub manual_focus_distance: f32,
    pub tracking_focus_settings: FCameraTrackingFocusSettings,
    #[doc(hidden)]
    __padding_96: [u8; 8],
    pub flags_96: u8,
    pub focus_smoothing_interp_speed: f32,
    pub focus_offset: f32,
    __padding_end: [u8; 4],
}
impl FCameraFocusSettings {}
#[repr(C, align(8))]
pub struct ACameraRig_Crane {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub crane_pitch: f32,
    pub crane_yaw: f32,
    pub crane_arm_length: f32,
    pub b_lock_mount_pitch: bool,
    pub b_lock_mount_yaw: bool,
    __padding_end: [u8; 66],
}
impl ACameraRig_Crane {}
#[repr(C, align(8))]
pub struct ACameraRig_Rail {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub current_position_on_rail: f32,
    pub b_lock_orientation_to_rail: bool,
    __padding_end: [u8; 75],
}
impl ACameraRig_Rail {}
#[repr(C, align(16))]
pub struct ACineCameraActor {
    #[doc(hidden)]
    __padding_3136: [u8; 3136],
    pub lookat_tracking_settings: FCameraLookatTrackingSettings,
    __padding_end: [u8; 24],
}
impl ACineCameraActor {}
#[repr(C, align(16))]
pub struct UCineCameraComponent {
    #[doc(hidden)]
    __padding_2876: [u8; 2876],
    pub filmback: FCameraFilmbackSettings,
    pub lens_settings: FCameraLensSettings,
    pub focus_settings: FCameraFocusSettings,
    pub crop_settings: FPlateCropSettings,
    pub current_focal_length: f32,
    pub current_aperture: f32,
    pub current_focus_distance: f32,
    pub exposure_method: ECameraExposureMethod,
    #[doc(hidden)]
    __padding_3060: [u8; 3],
    pub flags_3060: u8,
    pub custom_near_clipping_plane: f32,
    __padding_end: [u8; 132],
}
impl UCineCameraComponent {}
#[repr(C, align(8))]
pub struct UCineCameraSettings {
    #[doc(hidden)]
    __padding_104: [u8; 104],
    pub default_lens_preset_name: FString,
    pub default_lens_focal_length: f32,
    pub default_lens_f_stop: f32,
    pub lens_presets: TArray<FNamedLensPreset>,
    pub default_filmback_preset: FString,
    pub filmback_presets: TArray<FNamedFilmbackPreset>,
    pub default_crop_preset_name: FString,
    pub crop_presets: TArray<FNamedPlateCropPreset>,
    __padding_end: [u8; 16],
}
impl UCineCameraSettings {}
#[repr(transparent)]
pub struct ECameraFocusMethod(pub u8);
impl ECameraFocusMethod {
    pub const DO_NOT_OVERRIDE: ECameraFocusMethod = ECameraFocusMethod(0);
    pub const MANUAL: ECameraFocusMethod = ECameraFocusMethod(1);
    pub const TRACKING: ECameraFocusMethod = ECameraFocusMethod(2);
    pub const DISABLE: ECameraFocusMethod = ECameraFocusMethod(3);
    pub const MAX: ECameraFocusMethod = ECameraFocusMethod(4);
}
#[repr(transparent)]
pub struct ECameraExposureMethod(pub u8);
impl ECameraExposureMethod {
    pub const DO_NOT_OVERRIDE: ECameraExposureMethod = ECameraExposureMethod(0);
    pub const ENABLED: ECameraExposureMethod = ECameraExposureMethod(1);
    pub const MAX: ECameraExposureMethod = ECameraExposureMethod(2);
}
