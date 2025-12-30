#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FPIERHIOverrideState {
    pub max_shadow_depth_buffer_size_x: i32,
    pub max_shadow_depth_buffer_size_y: i32,
    pub max_texture_dimensions: i32,
    pub max_cube_texture_dimensions: i32,
    pub supports_render_target_format_pf_g8: bool,
    pub supports_render_target_format_pf_float_rgba: bool,
    pub supports_multiple_render_targets: bool,
}
#[repr(C, align(8))]
pub struct FPIEAndroidDeviceProperties {
    pub gpu_family: FString,
    pub gl_version: FString,
    pub vulkan_version: FString,
    pub android_version: FString,
    pub device_make: FString,
    pub device_model: FString,
    pub device_build_number: FString,
    pub vulkan_available: bool,
    pub using_houdini: bool,
    pub hardware: FString,
    pub chipset: FString,
    pub total_physical_gb: FString,
    pub hmd_system_name: FString,
    pub gles31rhi_state: FPIERHIOverrideState,
    pub sm5_available: bool,
}
#[repr(C, align(8))]
pub struct FPIEIOSDeviceProperties {
    pub device_model: FString,
    pub native_scale_factor: f32,
    pub metal_rhi_state: FPIERHIOverrideState,
}
#[repr(C, align(1))]
pub struct FPIESwitchDeviceProperties {
    pub docked: bool,
}
#[repr(C, align(4))]
pub struct FPIEPreviewDeviceBezelViewportRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
#[repr(C, align(8))]
pub struct FPIEBezelProperties {
    pub device_bezel_file: FString,
    pub bezel_viewport_rect: FPIEPreviewDeviceBezelViewportRect,
}
#[repr(C, align(8))]
pub struct FPIEPreviewDeviceSpecifications {
    pub device_platform: EPIEPreviewDeviceType,
    pub resolution_x: i32,
    pub resolution_y: i32,
    pub resolution_y_immersive_mode: i32,
    pub insets_left: f32,
    pub insets_top: f32,
    pub insets_right: f32,
    pub insets_bottom: f32,
    pub ppi: i32,
    pub scale_factors: TArray<f32>,
    pub bezel_properties: FPIEBezelProperties,
    pub android_properties: FPIEAndroidDeviceProperties,
    pub ios_properties: FPIEIOSDeviceProperties,
    pub switch_properties: FPIESwitchDeviceProperties,
}
pub struct UPIEPreviewDeviceSpecification {
    pub preview_device_type: EPIEPreviewDeviceType,
    pub gpu_family: FString,
    pub gl_version: FString,
    pub vulkan_version: FString,
    pub android_version: FString,
    pub device_make: FString,
    pub device_model: FString,
    pub device_build_number: FString,
    pub using_houdini: bool,
    pub hardware: FString,
    pub chipset: FString,
}
