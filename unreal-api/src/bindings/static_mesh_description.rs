#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FUVMapSettings {
    pub size: crate::bindings::core_u_object::FVector,
    pub uv_tile: crate::bindings::core_u_object::FVector2D,
    pub position: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub scale: crate::bindings::core_u_object::FVector,
}
impl FUVMapSettings {}
#[repr(C, align(8))]
pub struct UStaticMeshDescription {
    __padding_end: [u8; 760],
}
impl UStaticMeshDescription {}
