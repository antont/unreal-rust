#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
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
    pub angular_constraint_mode: EAngularConstraintMotion,
    pub lod_index: i32,
    pub hull_count: i32,
    pub max_hull_verts: i32,
    pub level_set_resolution: i32,
    pub lattice_resolution: i32,
}
