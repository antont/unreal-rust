#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UClothConfigNv {
    __padding_end: [u8; 424],
}
impl UClothConfigNv {}
#[repr(C, align(8))]
pub struct UClothingSimulationFactoryNv {
    __padding_end: [u8; 48],
}
impl UClothingSimulationFactoryNv {}
#[repr(C, align(8))]
pub struct UClothingSimulationInteractorNv {
    __padding_end: [u8; 152],
}
impl UClothingSimulationInteractorNv {}
#[repr(C, align(8))]
pub struct UClothPhysicalMeshDataNv_Legacy {
    __padding_end: [u8; 312],
}
impl UClothPhysicalMeshDataNv_Legacy {}
#[repr(transparent)]
pub struct EClothingWindMethodNv(pub u8);
impl EClothingWindMethodNv {
    pub const LEGACY: EClothingWindMethodNv = EClothingWindMethodNv(0);
    pub const ACCURATE: EClothingWindMethodNv = EClothingWindMethodNv(1);
}
