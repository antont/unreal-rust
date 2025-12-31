#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EParameterizeMeshUVMethod(pub i32);
impl EParameterizeMeshUVMethod {
    pub const PATCH_BUILDER: EParameterizeMeshUVMethod = EParameterizeMeshUVMethod(0);
    pub const UV_ATLAS: EParameterizeMeshUVMethod = EParameterizeMeshUVMethod(1);
    pub const X_ATLAS: EParameterizeMeshUVMethod = EParameterizeMeshUVMethod(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EEmbeddedPolygonOpMethod(pub u8);
impl EEmbeddedPolygonOpMethod {
    pub const TRIM_OUTSIDE: EEmbeddedPolygonOpMethod = EEmbeddedPolygonOpMethod(0);
    pub const TRIM_INSIDE: EEmbeddedPolygonOpMethod = EEmbeddedPolygonOpMethod(1);
    pub const INSERT_POLYGON: EEmbeddedPolygonOpMethod = EEmbeddedPolygonOpMethod(2);
    pub const CUT_THROUGH: EEmbeddedPolygonOpMethod = EEmbeddedPolygonOpMethod(3);
    pub const CUT_OUTSIDE: EEmbeddedPolygonOpMethod = EEmbeddedPolygonOpMethod(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESimplifyType(pub u8);
impl ESimplifyType {
    pub const QEM: ESimplifyType = ESimplifyType(0);
    pub const ATTRIBUTE: ESimplifyType = ESimplifyType(1);
    pub const UE_STANDARD: ESimplifyType = ESimplifyType(2);
    pub const MINIMAL_EXISTING_VERTEX: ESimplifyType = ESimplifyType(3);
    pub const MINIMAL_PLANAR: ESimplifyType = ESimplifyType(4);
    pub const MINIMAL_POLYGROUP: ESimplifyType = ESimplifyType(5);
    pub const CLUSTER_BASED: ESimplifyType = ESimplifyType(6);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESimplifyTargetType(pub u8);
impl ESimplifyTargetType {
    pub const PERCENTAGE: ESimplifyTargetType = ESimplifyTargetType(0);
    pub const TRIANGLE_COUNT: ESimplifyTargetType = ESimplifyTargetType(1);
    pub const VERTEX_COUNT: ESimplifyTargetType = ESimplifyTargetType(2);
    pub const EDGE_LENGTH: ESimplifyTargetType = ESimplifyTargetType(3);
    pub const MINIMAL_PLANAR: ESimplifyTargetType = ESimplifyTargetType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshTangentsType(pub u8);
impl EMeshTangentsType {
    pub const MIKK_T_SPACE: EMeshTangentsType = EMeshTangentsType(0);
    pub const FAST_MIKK_T_SPACE: EMeshTangentsType = EMeshTangentsType(1);
    pub const PER_TRIANGLE: EMeshTangentsType = EMeshTangentsType(2);
    pub const COPY_EXISTING: EMeshTangentsType = EMeshTangentsType(3);
}
