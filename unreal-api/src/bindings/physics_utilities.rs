#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FPhysAssetCreateParams {
    pub min_bone_size: f32,
    pub min_weld_size: f32,
    pub geom_type: EPhysAssetFitGeomType,
    pub vert_weight: EPhysAssetFitVertWeight,
    pub b_auto_orient_to_bone: bool,
    pub b_create_constraints: bool,
    pub b_walk_past_small: bool,
    pub b_body_for_all: bool,
    pub b_disable_collisions_by_default: bool,
    pub angular_constraint_mode: crate::bindings::physics_core::EAngularConstraintMotion,
    pub lod_index: i32,
    pub hull_count: i32,
    pub max_hull_verts: i32,
    pub level_set_resolution: i32,
    pub lattice_resolution: i32,
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPhysAssetFitVertWeight(pub u8);
impl EPhysAssetFitVertWeight {
    pub const EVW_ANY_WEIGHT: EPhysAssetFitVertWeight = EPhysAssetFitVertWeight(0);
    pub const EVW_DOMINANT_WEIGHT: EPhysAssetFitVertWeight = EPhysAssetFitVertWeight(1);
}
