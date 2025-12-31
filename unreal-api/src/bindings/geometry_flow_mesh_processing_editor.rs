#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FMeshAutoGenerateUVsSettings {
    pub method: EGeometryFlow_AutoUVMethod,
    pub uv_atlas_stretch: f64,
    pub uv_atlas_num_charts: i32,
    pub x_atlas_max_iterations: i32,
    pub num_initial_patches: i32,
    pub curvature_alignment: f64,
    pub merging_threshold: f64,
    pub max_angle_deviation_deg: f64,
    pub smoothing_steps: i32,
    pub smoothing_alpha: f64,
    pub b_auto_pack: bool,
    pub packing_target_width: i32,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGeometryFlow_AutoUVMethod(pub u8);
impl EGeometryFlow_AutoUVMethod {
    pub const PATCH_BUILDER: EGeometryFlow_AutoUVMethod = EGeometryFlow_AutoUVMethod(0);
    pub const UV_ATLAS: EGeometryFlow_AutoUVMethod = EGeometryFlow_AutoUVMethod(1);
    pub const X_ATLAS: EGeometryFlow_AutoUVMethod = EGeometryFlow_AutoUVMethod(2);
}
