#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(transparent)]
pub struct EPhysAssetFitGeomType(pub u8);
impl EPhysAssetFitGeomType {
    pub const EFG_BOX: EPhysAssetFitGeomType = EPhysAssetFitGeomType(0);
    pub const EFG_SPHYL: EPhysAssetFitGeomType = EPhysAssetFitGeomType(1);
    pub const EFG_SPHERE: EPhysAssetFitGeomType = EPhysAssetFitGeomType(2);
    pub const EFG_TAPERED_CAPSULE: EPhysAssetFitGeomType = EPhysAssetFitGeomType(3);
    pub const EFG_SINGLE_CONVEX_HULL: EPhysAssetFitGeomType = EPhysAssetFitGeomType(4);
    pub const EFG_MULTI_CONVEX_HULL: EPhysAssetFitGeomType = EPhysAssetFitGeomType(5);
    pub const EFG_LEVEL_SET: EPhysAssetFitGeomType = EPhysAssetFitGeomType(6);
    pub const EFG_SKINNED_LEVEL_SET: EPhysAssetFitGeomType = EPhysAssetFitGeomType(7);
    pub const EFG_ML_LEVEL_SET: EPhysAssetFitGeomType = EPhysAssetFitGeomType(8);
    pub const EFG_SKINNED_TRIANGLE_MESH: EPhysAssetFitGeomType = EPhysAssetFitGeomType(
        9,
    );
}
#[repr(transparent)]
pub struct EPhysAssetFitVertWeight(pub u8);
impl EPhysAssetFitVertWeight {
    pub const EVW_ANY_WEIGHT: EPhysAssetFitVertWeight = EPhysAssetFitVertWeight(0);
    pub const EVW_DOMINANT_WEIGHT: EPhysAssetFitVertWeight = EPhysAssetFitVertWeight(1);
}
