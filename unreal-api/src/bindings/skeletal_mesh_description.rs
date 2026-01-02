#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FBoneID {
    __padding_end: [u8; 4],
}
impl FBoneID {}
#[repr(C, align(4))]
pub struct FSourceGeometryPartID {
    __padding_end: [u8; 4],
}
impl FSourceGeometryPartID {}
#[repr(C, align(8))]
pub struct USkeletalMeshDescription {
    __padding_end: [u8; 760],
}
impl USkeletalMeshDescription {}
