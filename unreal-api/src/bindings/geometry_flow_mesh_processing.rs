#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(transparent)]
pub struct EGeometryFlow_ComputeNormalsType(pub i32);
impl EGeometryFlow_ComputeNormalsType {
    pub const PER_TRIANGLE: EGeometryFlow_ComputeNormalsType = EGeometryFlow_ComputeNormalsType(
        0,
    );
    pub const PER_VERTEX: EGeometryFlow_ComputeNormalsType = EGeometryFlow_ComputeNormalsType(
        1,
    );
    pub const RECOMPUTE_EXISTING_TOPOLOGY: EGeometryFlow_ComputeNormalsType = EGeometryFlow_ComputeNormalsType(
        2,
    );
    pub const FROM_FACE_ANGLE_THRESHOLD: EGeometryFlow_ComputeNormalsType = EGeometryFlow_ComputeNormalsType(
        3,
    );
    pub const FROM_GROUPS: EGeometryFlow_ComputeNormalsType = EGeometryFlow_ComputeNormalsType(
        4,
    );
}
#[repr(transparent)]
pub struct EGeometryFlow_MeshSimplifyType(pub i32);
impl EGeometryFlow_MeshSimplifyType {
    pub const STANDARD: EGeometryFlow_MeshSimplifyType = EGeometryFlow_MeshSimplifyType(
        0,
    );
    pub const VOLUME_PRESERVING: EGeometryFlow_MeshSimplifyType = EGeometryFlow_MeshSimplifyType(
        1,
    );
    pub const ATTRIBUTE_AWARE: EGeometryFlow_MeshSimplifyType = EGeometryFlow_MeshSimplifyType(
        2,
    );
}
#[repr(transparent)]
pub struct EGeomtryFlow_MeshSimplifyTargetType(pub i32);
impl EGeomtryFlow_MeshSimplifyTargetType {
    pub const TRIANGLE_COUNT: EGeomtryFlow_MeshSimplifyTargetType = EGeomtryFlow_MeshSimplifyTargetType(
        0,
    );
    pub const VERTEX_COUNT: EGeomtryFlow_MeshSimplifyTargetType = EGeomtryFlow_MeshSimplifyTargetType(
        1,
    );
    pub const TRIANGLE_PERCENTAGE: EGeomtryFlow_MeshSimplifyTargetType = EGeomtryFlow_MeshSimplifyTargetType(
        2,
    );
    pub const GEOMETRIC_DEVIATION: EGeomtryFlow_MeshSimplifyTargetType = EGeomtryFlow_MeshSimplifyTargetType(
        3,
    );
}
#[repr(transparent)]
pub struct EGeometryFlow_EdgeRefineFlags(pub i32);
impl EGeometryFlow_EdgeRefineFlags {
    pub const NO_CONSTRAINT: EGeometryFlow_EdgeRefineFlags = EGeometryFlow_EdgeRefineFlags(
        0,
    );
    pub const NO_FLIP: EGeometryFlow_EdgeRefineFlags = EGeometryFlow_EdgeRefineFlags(1);
    pub const NO_SPLIT: EGeometryFlow_EdgeRefineFlags = EGeometryFlow_EdgeRefineFlags(2);
    pub const NO_COLLAPSE: EGeometryFlow_EdgeRefineFlags = EGeometryFlow_EdgeRefineFlags(
        4,
    );
    pub const FULLY_CONSTRAINED: EGeometryFlow_EdgeRefineFlags = EGeometryFlow_EdgeRefineFlags(
        7,
    );
    pub const SPLITS_ONLY: EGeometryFlow_EdgeRefineFlags = EGeometryFlow_EdgeRefineFlags(
        5,
    );
    pub const FLIP_ONLY: EGeometryFlow_EdgeRefineFlags = EGeometryFlow_EdgeRefineFlags(
        6,
    );
    pub const COLLAPSE_ONLY: EGeometryFlow_EdgeRefineFlags = EGeometryFlow_EdgeRefineFlags(
        3,
    );
}
#[repr(transparent)]
pub struct EGeometryFlow_ComputeTangentsType(pub i32);
impl EGeometryFlow_ComputeTangentsType {
    pub const PER_TRIANGLE: EGeometryFlow_ComputeTangentsType = EGeometryFlow_ComputeTangentsType(
        0,
    );
    pub const FAST_MIKK_T: EGeometryFlow_ComputeTangentsType = EGeometryFlow_ComputeTangentsType(
        1,
    );
}
#[repr(transparent)]
pub struct EGeometryFlow_SmoothTypes(pub i32);
impl EGeometryFlow_SmoothTypes {
    pub const UNIFORM: EGeometryFlow_SmoothTypes = EGeometryFlow_SmoothTypes(0);
    pub const COTAN: EGeometryFlow_SmoothTypes = EGeometryFlow_SmoothTypes(1);
    pub const MEAN_VALUE: EGeometryFlow_SmoothTypes = EGeometryFlow_SmoothTypes(2);
}
#[repr(transparent)]
pub struct EGeometryFlow_RecalculateUVsUnwrapType(pub u8);
impl EGeometryFlow_RecalculateUVsUnwrapType {
    pub const AUTO: EGeometryFlow_RecalculateUVsUnwrapType = EGeometryFlow_RecalculateUVsUnwrapType(
        0,
    );
    pub const EXP_MAP: EGeometryFlow_RecalculateUVsUnwrapType = EGeometryFlow_RecalculateUVsUnwrapType(
        1,
    );
    pub const CONFORMAL: EGeometryFlow_RecalculateUVsUnwrapType = EGeometryFlow_RecalculateUVsUnwrapType(
        2,
    );
}
#[repr(transparent)]
pub struct EGeometryFlow_ProjectedHullAxisMode(pub i32);
impl EGeometryFlow_ProjectedHullAxisMode {
    pub const X: EGeometryFlow_ProjectedHullAxisMode = EGeometryFlow_ProjectedHullAxisMode(
        0,
    );
    pub const Y: EGeometryFlow_ProjectedHullAxisMode = EGeometryFlow_ProjectedHullAxisMode(
        1,
    );
    pub const Z: EGeometryFlow_ProjectedHullAxisMode = EGeometryFlow_ProjectedHullAxisMode(
        2,
    );
    pub const SMALLEST_BOX_DIMENSION: EGeometryFlow_ProjectedHullAxisMode = EGeometryFlow_ProjectedHullAxisMode(
        3,
    );
    pub const SMALLEST_VOLUME: EGeometryFlow_ProjectedHullAxisMode = EGeometryFlow_ProjectedHullAxisMode(
        4,
    );
}
