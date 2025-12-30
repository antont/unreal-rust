#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FCameraLookatTrackingSettings {
    pub flags_0: u8,
    pub look_at_tracking_interp_speed: f32,
    pub actor_to_track: TSoftObjectPtr<AActor>,
    pub relative_offset: FVector,
    pub flags_112: u8,
}
#[repr(C, align(4))]
pub struct FCameraFilmbackSettings {
    pub sensor_width: f32,
    pub sensor_height: f32,
    pub sensor_horizontal_offset: f32,
    pub sensor_vertical_offset: f32,
    pub sensor_aspect_ratio: f32,
}
#[repr(C, align(8))]
pub struct FNamedFilmbackPreset {
    pub name: FString,
    pub display_name: FText,
    pub filmback_settings: FCameraFilmbackSettings,
}
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
#[repr(C, align(8))]
pub struct FNamedLensPreset {
    pub name: FString,
    pub lens_settings: FCameraLensSettings,
}
#[repr(C, align(4))]
pub struct FPlateCropSettings {
    pub aspect_ratio: f32,
}
#[repr(C, align(8))]
pub struct FNamedPlateCropPreset {
    pub name: FString,
    pub crop_settings: FPlateCropSettings,
}
#[repr(C, align(8))]
pub struct FCameraTrackingFocusSettings {
    pub actor_to_track: TSoftObjectPtr<AActor>,
    pub relative_offset: FVector,
    pub flags_72: u8,
}
#[repr(C, align(8))]
pub struct FCameraFocusSettings {
    pub focus_method: ECameraFocusMethod,
    pub manual_focus_distance: f32,
    pub tracking_focus_settings: FCameraTrackingFocusSettings,
    pub flags_88: u8,
    pub debug_focus_plane_color: FColor,
    pub flags_96: u8,
    pub focus_smoothing_interp_speed: f32,
    pub focus_offset: f32,
}
pub struct ACameraRig_Crane {
    pub crane_pitch: f32,
    pub crane_yaw: f32,
    pub crane_arm_length: f32,
    pub b_lock_mount_pitch: bool,
    pub b_lock_mount_yaw: bool,
    pub transform_component: UPtr<USceneComponent>,
    pub crane_yaw_control: UPtr<USceneComponent>,
    pub crane_pitch_control: UPtr<USceneComponent>,
    pub crane_camera_mount: UPtr<USceneComponent>,
    pub preview_mesh_crane_arm: UPtr<UStaticMeshComponent>,
    pub preview_mesh_crane_base: UPtr<UStaticMeshComponent>,
    pub preview_mesh_crane_mount: UPtr<UStaticMeshComponent>,
    pub preview_mesh_crane_counter_weight: UPtr<UStaticMeshComponent>,
}
pub struct ACameraRig_Rail {
    pub current_position_on_rail: f32,
    pub b_lock_orientation_to_rail: bool,
    pub b_show_rail_visualization: bool,
    pub preview_mesh_scale: f32,
    pub transform_component: UPtr<USceneComponent>,
    pub rail_spline_component: UPtr<USplineComponent>,
    pub rail_camera_mount: UPtr<USceneComponent>,
    pub preview_mesh_rail: UPtr<USplineMeshComponent>,
    pub preview_rail_mesh_segments: TArray<UPtr<USplineMeshComponent>>,
    pub preview_rail_static_mesh: UPtr<UStaticMesh>,
    pub preview_mesh_mount: UPtr<UStaticMeshComponent>,
}
pub struct ACineCameraActor {
    pub lookat_tracking_settings: FCameraLookatTrackingSettings,
}
pub struct UCineCameraComponent {
    pub filmback_settings_deprecated: FCameraFilmbackSettings,
    pub filmback: FCameraFilmbackSettings,
    pub lens_settings: FCameraLensSettings,
    pub focus_settings: FCameraFocusSettings,
    pub crop_settings: FPlateCropSettings,
    pub current_focal_length: f32,
    pub current_aperture: f32,
    pub current_focus_distance: f32,
    pub exposure_method: ECameraExposureMethod,
    pub flags_3060: u8,
    pub custom_near_clipping_plane: f32,
    pub current_horizontal_fov: f32,
    pub focus_plane_visualization_mesh: UPtr<UStaticMesh>,
    pub focus_plane_visualization_material: UPtr<UMaterial>,
    pub debug_focus_plane_component: UPtr<UStaticMeshComponent>,
    pub debug_focus_plane_mid: UPtr<UMaterialInstanceDynamic>,
    pub filmback_presets: TArray<FNamedFilmbackPreset>,
    pub lens_presets: TArray<FNamedLensPreset>,
    pub default_filmback_preset_name_deprecated: FString,
    pub default_filmback_preset: FString,
    pub default_lens_preset_name: FString,
    pub default_lens_focal_length: f32,
    pub default_lens_f_stop: f32,
}
pub struct UCineCameraSettings {
    pub default_lens_preset_name: FString,
    pub default_lens_focal_length: f32,
    pub default_lens_f_stop: f32,
    pub lens_presets: TArray<FNamedLensPreset>,
    pub default_filmback_preset: FString,
    pub filmback_presets: TArray<FNamedFilmbackPreset>,
    pub default_crop_preset_name: FString,
    pub crop_presets: TArray<FNamedPlateCropPreset>,
}
