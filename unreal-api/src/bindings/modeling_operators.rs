#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct URecomputeUVsToolProperties {
    pub b_enable_polygroup_support: bool,
    pub island_generation: ERecomputeUVsPropertiesIslandMode,
    pub unwrap_type: ERecomputeUVsPropertiesUnwrapType,
    pub auto_rotation: ERecomputeUVsToolOrientationMode,
    pub b_preserve_irregularity: bool,
    pub smoothing_steps: i32,
    pub smoothing_alpha: f32,
    pub merging_distortion_threshold: f32,
    pub merging_angle_threshold: f32,
    pub layout_type: ERecomputeUVsPropertiesLayoutType,
    pub texture_resolution: i32,
    pub normalize_scale: f32,
    pub b_enable_udim_layout: bool,
    pub b_udimcvar_enabled: bool,
}
pub struct UUVLayoutProperties {
    pub layout_type: EUVLayoutType,
    pub texture_resolution: i32,
    pub scale: f32,
    pub translation: FVector2D,
    pub b_preserve_scale: bool,
    pub b_preserve_rotation: bool,
    pub b_allow_flips: bool,
    pub b_enable_udim_layout: bool,
    pub b_udimcvar_enabled: bool,
}
pub struct UGenerateCrossSectionOpFactory {}
pub struct URecomputeUVsOpFactory {
    pub settings: UPtr<URecomputeUVsToolProperties>,
}
pub struct UUVEditorTexelDensitySettings {
    pub texel_density_mode: ETexelDensityToolMode,
    pub target_world_units: f32,
    pub target_pixel_count: f32,
    pub texture_resolution: f32,
    pub b_enable_udim_layout: bool,
}
pub struct UUVTexelDensityOperatorFactory {
    pub settings: UPtr<UUVEditorTexelDensitySettings>,
}
pub struct UUVLayoutOperatorFactory {
    pub settings: UPtr<UUVLayoutProperties>,
}
