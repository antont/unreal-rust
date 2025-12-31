#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FStateStreamHandle {}
#[repr(C, align(4))]
pub struct FTransformStaticState {}
#[repr(C, align(16))]
pub struct FTransformDynamicState {
    pub local_transform: crate::bindings::core_u_object::FTransform,
    pub bone_transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub parent: FTransformHandle,
    pub b_visible: bool,
}
#[repr(C, align(8))]
pub struct FTransformHandle {}
