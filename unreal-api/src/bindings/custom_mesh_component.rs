#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FCustomMeshTriangle {
    pub vertex0: FVector,
    pub vertex1: FVector,
    pub vertex2: FVector,
}
pub struct UCustomMeshComponent {}
