#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FUVMapSettings {
    pub size: FVector,
    pub uv_tile: FVector2D,
    pub position: FVector,
    pub rotation: FRotator,
    pub scale: FVector,
}
pub struct UStaticMeshDescription {}
