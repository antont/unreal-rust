#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(transparent)]
pub struct EGroomCollectionType(pub u8);
impl EGroomCollectionType {
    pub const STRANDS: EGroomCollectionType = EGroomCollectionType(0);
    pub const GUIDES: EGroomCollectionType = EGroomCollectionType(1);
}
#[repr(transparent)]
pub struct EGroomAttributeType(pub u8);
impl EGroomAttributeType {
    pub const KINEMATIC_WEIGHTS: EGroomAttributeType = EGroomAttributeType(0);
    pub const BONE_INDICES: EGroomAttributeType = EGroomAttributeType(1);
    pub const BONE_WEIGHTS: EGroomAttributeType = EGroomAttributeType(2);
    pub const CURVE_PARENTS: EGroomAttributeType = EGroomAttributeType(3);
    pub const CURVE_LODS: EGroomAttributeType = EGroomAttributeType(4);
}
#[repr(transparent)]
pub struct EGroomNumPoints(pub u8);
impl EGroomNumPoints {
    pub const DEFAULT: EGroomNumPoints = EGroomNumPoints(0);
    pub const SIZE4: EGroomNumPoints = EGroomNumPoints(4);
    pub const SIZE8: EGroomNumPoints = EGroomNumPoints(8);
    pub const SIZE16: EGroomNumPoints = EGroomNumPoints(16);
    pub const SIZE32: EGroomNumPoints = EGroomNumPoints(32);
    pub const SIZE64: EGroomNumPoints = EGroomNumPoints(64);
}
