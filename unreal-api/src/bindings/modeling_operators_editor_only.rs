#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UParameterizeMeshToolProperties {
    pub method: EParameterizeMeshUVMethod,
}
pub struct UParameterizeMeshToolUVAtlasProperties {
    pub island_stretch: f32,
    pub num_islands: i32,
    pub texture_resolution: i32,
    pub b_use_polygroups: bool,
    pub b_layout_udim_per_polygroup: bool,
    pub b_polygroups_enabled: bool,
    pub b_udi_ms_enabled: bool,
}
pub struct UParameterizeMeshToolXAtlasProperties {
    pub max_iterations: i32,
}
pub struct UParameterizeMeshToolPatchBuilderProperties {
    pub initial_patches: i32,
    pub curvature_alignment: f32,
    pub merging_distortion_threshold: f32,
    pub merging_angle_threshold: f32,
    pub smoothing_steps: i32,
    pub smoothing_alpha: f32,
    pub b_repack: bool,
    pub texture_resolution: i32,
    pub b_use_polygroups: bool,
    pub b_layout_udim_per_polygroup: bool,
    pub b_polygroups_enabled: bool,
    pub b_udi_ms_enabled: bool,
}
pub struct UParameterizeMeshOperatorFactory {
    pub settings: UPtr<UParameterizeMeshToolProperties>,
    pub uv_atlas_properties: UPtr<UParameterizeMeshToolUVAtlasProperties>,
    pub x_atlas_properties: UPtr<UParameterizeMeshToolXAtlasProperties>,
    pub patch_builder_properties: UPtr<UParameterizeMeshToolPatchBuilderProperties>,
}
