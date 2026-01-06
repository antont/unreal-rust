#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(transparent)]
pub struct EFractureBrickBondEnum(pub u8);
impl EFractureBrickBondEnum {
    pub const DATAFLOW_FRACTURE_BRICK_BOND_STRETCHER: EFractureBrickBondEnum = EFractureBrickBondEnum(
        0,
    );
    pub const DATAFLOW_FRACTURE_BRICK_BOND_STACK: EFractureBrickBondEnum = EFractureBrickBondEnum(
        1,
    );
    pub const DATAFLOW_FRACTURE_BRICK_BOND_ENGLISH: EFractureBrickBondEnum = EFractureBrickBondEnum(
        2,
    );
    pub const DATAFLOW_FRACTURE_BRICK_BOND_HEADER: EFractureBrickBondEnum = EFractureBrickBondEnum(
        3,
    );
    pub const DATAFLOW_FRACTURE_BRICK_BOND_FLEMISH: EFractureBrickBondEnum = EFractureBrickBondEnum(
        4,
    );
}
#[repr(transparent)]
pub struct EMeshCutterCutDistribution(pub u8);
impl EMeshCutterCutDistribution {
    pub const SINGLE_CUT: EMeshCutterCutDistribution = EMeshCutterCutDistribution(0);
    pub const UNIFORM_RANDOM: EMeshCutterCutDistribution = EMeshCutterCutDistribution(1);
    pub const GRID: EMeshCutterCutDistribution = EMeshCutterCutDistribution(2);
}
#[repr(transparent)]
pub struct EMeshCutterPerCutMeshSelection(pub u8);
impl EMeshCutterPerCutMeshSelection {
    pub const ALL: EMeshCutterPerCutMeshSelection = EMeshCutterPerCutMeshSelection(0);
    pub const RANDOM: EMeshCutterPerCutMeshSelection = EMeshCutterPerCutMeshSelection(1);
    pub const SEQUENTIAL: EMeshCutterPerCutMeshSelection = EMeshCutterPerCutMeshSelection(
        2,
    );
}
#[repr(transparent)]
pub struct ENonUniformSamplingDistributionMode(pub u8);
impl ENonUniformSamplingDistributionMode {
    pub const E_NON_UNIFORM_SAMPLING_DISTRIBUTION_MODE_UNIFORM: ENonUniformSamplingDistributionMode = ENonUniformSamplingDistributionMode(
        0,
    );
    pub const E_NON_UNIFORM_SAMPLING_DISTRIBUTION_MODE_SMALLER: ENonUniformSamplingDistributionMode = ENonUniformSamplingDistributionMode(
        1,
    );
    pub const E_NON_UNIFORM_SAMPLING_DISTRIBUTION_MODE_LARGER: ENonUniformSamplingDistributionMode = ENonUniformSamplingDistributionMode(
        2,
    );
}
#[repr(transparent)]
pub struct ENonUniformSamplingWeightMode(pub u8);
impl ENonUniformSamplingWeightMode {
    pub const E_NON_UNIFORM_SAMPLING_WEIGHT_MODE_WEIGHT_TO_RADIUS: ENonUniformSamplingWeightMode = ENonUniformSamplingWeightMode(
        0,
    );
    pub const E_NON_UNIFORM_SAMPLING_WEIGHT_MODE_FILLED_WEIGHT_TO_RADIUS: ENonUniformSamplingWeightMode = ENonUniformSamplingWeightMode(
        1,
    );
    pub const E_NON_UNIFORM_SAMPLING_WEIGHT_MODE_WEIGHTED_RANDOM: ENonUniformSamplingWeightMode = ENonUniformSamplingWeightMode(
        2,
    );
}
#[repr(transparent)]
pub struct EConvexHullSimplifyMethod(pub i32);
impl EConvexHullSimplifyMethod {
    pub const MESH_Q_SLIM: EConvexHullSimplifyMethod = EConvexHullSimplifyMethod(0);
    pub const ANGLE_TOLERANCE: EConvexHullSimplifyMethod = EConvexHullSimplifyMethod(1);
}
#[repr(transparent)]
pub struct EFixTinyGeoMergeType(pub u8);
impl EFixTinyGeoMergeType {
    pub const MERGE_GEOMETRY: EFixTinyGeoMergeType = EFixTinyGeoMergeType(0);
    pub const MERGE_CLUSTERS: EFixTinyGeoMergeType = EFixTinyGeoMergeType(1);
}
#[repr(transparent)]
pub struct EFixTinyGeoNeighborSelectionMethod(pub u8);
impl EFixTinyGeoNeighborSelectionMethod {
    pub const LARGEST_NEIGHBOR: EFixTinyGeoNeighborSelectionMethod = EFixTinyGeoNeighborSelectionMethod(
        0,
    );
    pub const NEAREST_CENTER: EFixTinyGeoNeighborSelectionMethod = EFixTinyGeoNeighborSelectionMethod(
        1,
    );
}
#[repr(transparent)]
pub struct EFixTinyGeoUseBoneSelection(pub u8);
impl EFixTinyGeoUseBoneSelection {
    pub const NO_EFFECT: EFixTinyGeoUseBoneSelection = EFixTinyGeoUseBoneSelection(0);
    pub const ALSO_MERGE_SELECTED: EFixTinyGeoUseBoneSelection = EFixTinyGeoUseBoneSelection(
        1,
    );
    pub const ONLY_MERGE_SELECTED: EFixTinyGeoUseBoneSelection = EFixTinyGeoUseBoneSelection(
        2,
    );
}
#[repr(transparent)]
pub struct EFixTinyGeoGeometrySelectionMethod(pub u8);
impl EFixTinyGeoGeometrySelectionMethod {
    pub const VOLUME_CUBE_ROOT: EFixTinyGeoGeometrySelectionMethod = EFixTinyGeoGeometrySelectionMethod(
        0,
    );
    pub const RELATIVE_VOLUME: EFixTinyGeoGeometrySelectionMethod = EFixTinyGeoGeometrySelectionMethod(
        1,
    );
}
