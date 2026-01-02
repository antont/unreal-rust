#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FCustomMeshTriangle {
    pub vertex0: crate::bindings::core_u_object::FVector,
    pub vertex1: crate::bindings::core_u_object::FVector,
    pub vertex2: crate::bindings::core_u_object::FVector,
}
impl FCustomMeshTriangle {}
#[repr(C, align(16))]
pub struct UCustomMeshComponent {
    __padding_end: [u8; 1600],
}
impl UCustomMeshComponent {}
