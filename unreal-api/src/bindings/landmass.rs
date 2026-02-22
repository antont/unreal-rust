#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(forgetting_copy_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FBrushEffectBlurring {
    pub b_blur_shape: bool,
    pub radius: i32,
}
impl FBrushEffectBlurring {}
#[repr(C, align(4))]
pub struct FBrushEffectCurlNoise {
    pub curl1_amount: f32,
    pub curl2_amount: f32,
    pub curl1_tiling: f32,
    pub curl2_tiling: f32,
}
impl FBrushEffectCurlNoise {}
#[repr(C, align(8))]
pub struct FBrushEffectCurves {
    pub b_use_curve_channel: bool,
    pub elevation_curve_asset: UPtr<crate::bindings::engine::UCurveFloat>,
    pub channel_edge_offset: f32,
    pub channel_depth: f32,
    pub curve_ramp_width: f32,
}
impl FBrushEffectCurves {}
#[repr(C, align(8))]
pub struct FBrushEffectDisplacement {
    pub displacement_height: f32,
    pub displacement_tiling: f32,
    pub texture: UPtr<crate::bindings::engine::UTexture2D>,
    pub midpoint: f32,
    pub channel: crate::bindings::core_u_object::FLinearColor,
    pub weightmap_influence: f32,
}
impl FBrushEffectDisplacement {}
#[repr(C, align(4))]
pub struct FBrushEffectSmoothBlending {
    pub inner_smooth_distance: f32,
    pub outer_smooth_distance: f32,
}
impl FBrushEffectSmoothBlending {}
#[repr(C, align(4))]
pub struct FBrushEffectTerracing {
    pub terrace_alpha: f32,
    pub terrace_spacing: f32,
    pub terrace_smoothness: f32,
    pub mask_length: f32,
    pub mask_start_offset: f32,
}
impl FBrushEffectTerracing {}
#[repr(C, align(8))]
pub struct FLandmassBrushEffectsList {
    pub blurring: FBrushEffectBlurring,
    pub curl_noise: FBrushEffectCurlNoise,
    pub displacement: FBrushEffectDisplacement,
    pub smooth_blending: FBrushEffectSmoothBlending,
    pub terracing: FBrushEffectTerracing,
}
impl FLandmassBrushEffectsList {}
#[repr(C, align(4))]
pub struct FLandmassFalloffSettings {
    pub falloff_mode: EBrushFalloffMode,
    pub falloff_angle: f32,
    pub falloff_width: f32,
    pub edge_offset: f32,
    pub z_offset: f32,
}
impl FLandmassFalloffSettings {}
#[repr(C, align(8))]
pub struct FLandmassTerrainCarvingSettings {
    pub blend_mode: EBrushBlendType,
    pub b_invert_shape: bool,
    pub falloff_settings: FLandmassFalloffSettings,
    pub effects: FLandmassBrushEffectsList,
    pub priority: i32,
}
impl FLandmassTerrainCarvingSettings {}
#[repr(transparent)]
pub struct EBrushFalloffMode(pub u8);
impl EBrushFalloffMode {
    pub const ANGLE: EBrushFalloffMode = EBrushFalloffMode(0);
    pub const WIDTH: EBrushFalloffMode = EBrushFalloffMode(1);
}
#[repr(transparent)]
pub struct EBrushBlendType(pub u8);
impl EBrushBlendType {
    pub const ALPHA_BLEND: EBrushBlendType = EBrushBlendType(0);
    pub const MIN: EBrushBlendType = EBrushBlendType(1);
    pub const MAX: EBrushBlendType = EBrushBlendType(2);
    pub const ADDITIVE: EBrushBlendType = EBrushBlendType(3);
}
