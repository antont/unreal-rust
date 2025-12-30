#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(1))]
pub struct FStateStreamHandle {}
#[repr(C, align(4))]
pub struct FTransformStaticState {}
#[repr(C, align(16))]
pub struct FTransformDynamicState {
    pub local_transform: FTransform,
    pub bone_transforms: TArray<FTransform>,
    pub parent: FTransformHandle,
    pub b_visible: bool,
}
#[repr(C, align(8))]
pub struct FTransformHandle {}
