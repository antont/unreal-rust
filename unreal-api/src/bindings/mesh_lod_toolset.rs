#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAssetDefinition_StaticMeshLODGenerationSettings {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_StaticMeshLODGenerationSettings {}
#[repr(C, align(8))]
pub struct UGenerateStaticMeshLODProcess {
    __padding_end: [u8; 2096],
}
impl UGenerateStaticMeshLODProcess {}
#[repr(C, align(8))]
pub struct UGenerateStaticMeshLODAssetToolBuilder {
    __padding_end: [u8; 56],
}
impl UGenerateStaticMeshLODAssetToolBuilder {}
#[repr(C, align(8))]
pub struct UGenerateStaticMeshLODAssetToolOutputProperties {
    __padding_end: [u8; 240],
}
impl UGenerateStaticMeshLODAssetToolOutputProperties {}
#[repr(C, align(8))]
pub struct UGenerateStaticMeshLODAssetToolPresetProperties {
    __padding_end: [u8; 200],
}
impl UGenerateStaticMeshLODAssetToolPresetProperties {}
#[repr(C, align(8))]
pub struct UGenerateStaticMeshLODAssetToolProperties {
    __padding_end: [u8; 376],
}
impl UGenerateStaticMeshLODAssetToolProperties {}
#[repr(C, align(8))]
pub struct UGenerateStaticMeshLODAssetToolTextureProperties {
    __padding_end: [u8; 232],
}
impl UGenerateStaticMeshLODAssetToolTextureProperties {}
#[repr(C, align(8))]
pub struct UGenerateStaticMeshLODAssetTool {
    __padding_end: [u8; 352],
}
impl UGenerateStaticMeshLODAssetTool {}
#[repr(C, align(8))]
pub struct UStaticMeshLODGenerationSettings {
    __padding_end: [u8; 216],
}
impl UStaticMeshLODGenerationSettings {}
#[repr(C, align(8))]
pub struct UStaticMeshLODGenerationSettingsFactory {
    __padding_end: [u8; 136],
}
impl UStaticMeshLODGenerationSettingsFactory {}
#[repr(C, align(8))]
pub struct ULODManagerToolBuilder {
    __padding_end: [u8; 48],
}
impl ULODManagerToolBuilder {}
#[repr(C, align(8))]
pub struct ULODManagerLODProperties {
    __padding_end: [u8; 256],
}
impl ULODManagerLODProperties {}
#[repr(C, align(8))]
pub struct ULODManagerPreviewLODProperties {
    __padding_end: [u8; 224],
}
impl ULODManagerPreviewLODProperties {}
#[repr(C, align(8))]
pub struct ULODManagerActionPropertySet {
    __padding_end: [u8; 192],
}
impl ULODManagerActionPropertySet {}
#[repr(C, align(8))]
pub struct ULODManagerHiResSourceModelActions {
    __padding_end: [u8; 192],
}
impl ULODManagerHiResSourceModelActions {}
#[repr(C, align(8))]
pub struct ULODManagerMaterialActions {
    __padding_end: [u8; 192],
}
impl ULODManagerMaterialActions {}
pub struct ULODManagerToolChangeTarget {}
pub struct ILODManagerToolChangeTarget {}
#[repr(C, align(8))]
pub struct ULODManagerTool {
    __padding_end: [u8; 408],
}
impl ULODManagerTool {}
#[repr(transparent)]
pub struct EGenerateStaticMeshLODProcess_MeshGeneratorModes(pub u8);
impl EGenerateStaticMeshLODProcess_MeshGeneratorModes {
    pub const SOLIDIFY: EGenerateStaticMeshLODProcess_MeshGeneratorModes = EGenerateStaticMeshLODProcess_MeshGeneratorModes(
        0,
    );
    pub const SOLIDIFY_AND_CLOSE: EGenerateStaticMeshLODProcess_MeshGeneratorModes = EGenerateStaticMeshLODProcess_MeshGeneratorModes(
        1,
    );
    pub const CLEAN_AND_SIMPLIFY: EGenerateStaticMeshLODProcess_MeshGeneratorModes = EGenerateStaticMeshLODProcess_MeshGeneratorModes(
        2,
    );
    pub const CONVEX_HULL: EGenerateStaticMeshLODProcess_MeshGeneratorModes = EGenerateStaticMeshLODProcess_MeshGeneratorModes(
        3,
    );
}
#[repr(transparent)]
pub struct EGenerateStaticMeshLODProcess_SimplifyMethod(pub u8);
impl EGenerateStaticMeshLODProcess_SimplifyMethod {
    pub const TRIANGLE_COUNT: EGenerateStaticMeshLODProcess_SimplifyMethod = EGenerateStaticMeshLODProcess_SimplifyMethod(
        0,
    );
    pub const VERTEX_COUNT: EGenerateStaticMeshLODProcess_SimplifyMethod = EGenerateStaticMeshLODProcess_SimplifyMethod(
        1,
    );
    pub const TRIANGLE_PERCENTAGE: EGenerateStaticMeshLODProcess_SimplifyMethod = EGenerateStaticMeshLODProcess_SimplifyMethod(
        2,
    );
    pub const GEOMETRIC_TOLERANCE: EGenerateStaticMeshLODProcess_SimplifyMethod = EGenerateStaticMeshLODProcess_SimplifyMethod(
        3,
    );
}
#[repr(transparent)]
pub struct EGenerateStaticMeshLODProcess_NormalsMethod(pub u8);
impl EGenerateStaticMeshLODProcess_NormalsMethod {
    pub const FROM_ANGLE_THRESHOLD: EGenerateStaticMeshLODProcess_NormalsMethod = EGenerateStaticMeshLODProcess_NormalsMethod(
        0,
    );
    pub const PER_VERTEX: EGenerateStaticMeshLODProcess_NormalsMethod = EGenerateStaticMeshLODProcess_NormalsMethod(
        1,
    );
    pub const PER_TRIANGLE: EGenerateStaticMeshLODProcess_NormalsMethod = EGenerateStaticMeshLODProcess_NormalsMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EGenerateStaticMeshLODProcess_AutoUVMethod(pub u8);
impl EGenerateStaticMeshLODProcess_AutoUVMethod {
    pub const PATCH_BUILDER: EGenerateStaticMeshLODProcess_AutoUVMethod = EGenerateStaticMeshLODProcess_AutoUVMethod(
        0,
    );
    pub const UV_ATLAS: EGenerateStaticMeshLODProcess_AutoUVMethod = EGenerateStaticMeshLODProcess_AutoUVMethod(
        1,
    );
    pub const X_ATLAS: EGenerateStaticMeshLODProcess_AutoUVMethod = EGenerateStaticMeshLODProcess_AutoUVMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EGenerateStaticMeshLODBakeResolution(pub i32);
impl EGenerateStaticMeshLODBakeResolution {
    pub const RESOLUTION16: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        16,
    );
    pub const RESOLUTION32: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        32,
    );
    pub const RESOLUTION64: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        64,
    );
    pub const RESOLUTION128: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        128,
    );
    pub const RESOLUTION256: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        256,
    );
    pub const RESOLUTION512: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        512,
    );
    pub const RESOLUTION1024: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        1024,
    );
    pub const RESOLUTION2048: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        2048,
    );
    pub const RESOLUTION4096: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        4096,
    );
    pub const RESOLUTION8192: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        8192,
    );
}
#[repr(transparent)]
pub struct EGenerateStaticMeshLODSimpleCollisionGeometryType(pub u8);
impl EGenerateStaticMeshLODSimpleCollisionGeometryType {
    pub const ALIGNED_BOXES: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        0,
    );
    pub const ORIENTED_BOXES: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        1,
    );
    pub const MINIMAL_SPHERES: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        2,
    );
    pub const CAPSULES: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        3,
    );
    pub const CONVEX_HULLS: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        4,
    );
    pub const SWEPT_HULLS: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        5,
    );
    pub const MIN_VOLUME: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        6,
    );
    pub const NONE: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        7,
    );
}
#[repr(transparent)]
pub struct EGenerateStaticMeshLODProjectedHullAxisMode(pub u8);
impl EGenerateStaticMeshLODProjectedHullAxisMode {
    pub const X: EGenerateStaticMeshLODProjectedHullAxisMode = EGenerateStaticMeshLODProjectedHullAxisMode(
        0,
    );
    pub const Y: EGenerateStaticMeshLODProjectedHullAxisMode = EGenerateStaticMeshLODProjectedHullAxisMode(
        1,
    );
    pub const Z: EGenerateStaticMeshLODProjectedHullAxisMode = EGenerateStaticMeshLODProjectedHullAxisMode(
        2,
    );
    pub const SMALLEST_BOX_DIMENSION: EGenerateStaticMeshLODProjectedHullAxisMode = EGenerateStaticMeshLODProjectedHullAxisMode(
        3,
    );
    pub const SMALLEST_VOLUME: EGenerateStaticMeshLODProjectedHullAxisMode = EGenerateStaticMeshLODProjectedHullAxisMode(
        4,
    );
}
#[repr(transparent)]
pub struct EGenerateStaticMeshLOD_BakeConstraint(pub i32);
impl EGenerateStaticMeshLOD_BakeConstraint {
    pub const NO_CONSTRAINT: EGenerateStaticMeshLOD_BakeConstraint = EGenerateStaticMeshLOD_BakeConstraint(
        0,
    );
    pub const DO_NOT_BAKE: EGenerateStaticMeshLOD_BakeConstraint = EGenerateStaticMeshLOD_BakeConstraint(
        1,
    );
}
#[repr(transparent)]
pub struct EGenerateLODAssetOutputMode(pub u8);
impl EGenerateLODAssetOutputMode {
    pub const CREATE_NEW_ASSET: EGenerateLODAssetOutputMode = EGenerateLODAssetOutputMode(
        0,
    );
    pub const UPDATE_EXISTING_ASSET: EGenerateLODAssetOutputMode = EGenerateLODAssetOutputMode(
        1,
    );
}
