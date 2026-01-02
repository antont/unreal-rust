#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UClothConfigBase {
    __padding_end: [u8; 48],
}
impl UClothConfigBase {}
#[repr(C, align(8))]
pub struct UDEPRECATED_ClothSharedSimConfigBase {
    __padding_end: [u8; 48],
}
impl UDEPRECATED_ClothSharedSimConfigBase {}
#[repr(C, align(8))]
pub struct UClothingAssetBase {
    __padding_end: [u8; 80],
}
impl UClothingAssetBase {}
#[repr(C, align(8))]
pub struct UClothingSimulationFactory {
    __padding_end: [u8; 48],
}
impl UClothingSimulationFactory {}
#[repr(C, align(8))]
pub struct UClothingInteractor {
    __padding_end: [u8; 56],
}
impl UClothingInteractor {}
#[repr(C, align(8))]
pub struct UClothingSimulationInteractor {
    __padding_end: [u8; 152],
}
impl UClothingSimulationInteractor {}
#[repr(C, align(8))]
pub struct UClothPhysicalMeshDataBase_Legacy {
    __padding_end: [u8; 248],
}
impl UClothPhysicalMeshDataBase_Legacy {}
#[repr(transparent)]
pub struct EClothingTeleportMode(pub u8);
impl EClothingTeleportMode {
    pub const NONE: EClothingTeleportMode = EClothingTeleportMode(0);
    pub const TELEPORT: EClothingTeleportMode = EClothingTeleportMode(1);
    pub const TELEPORT_AND_RESET: EClothingTeleportMode = EClothingTeleportMode(2);
    pub const HARD_RESET: EClothingTeleportMode = EClothingTeleportMode(3);
}
