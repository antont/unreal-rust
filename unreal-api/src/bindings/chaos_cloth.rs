#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UChaosClothConfig {
    __padding_end: [u8; 424],
}
impl UChaosClothConfig {}
#[repr(C, align(8))]
pub struct UChaosClothSharedSimConfig {
    __padding_end: [u8; 120],
}
impl UChaosClothSharedSimConfig {}
#[repr(C, align(8))]
pub struct UChaosClothingSimulationFactory {
    __padding_end: [u8; 48],
}
impl UChaosClothingSimulationFactory {}
#[repr(C, align(8))]
pub struct UChaosClothingInteractor {
    __padding_end: [u8; 88],
}
impl UChaosClothingInteractor {}
#[repr(C, align(8))]
pub struct UChaosClothingSimulationInteractor {
    __padding_end: [u8; 184],
}
impl UChaosClothingSimulationInteractor {}
#[repr(transparent)]
pub struct EChaosClothTetherMode(pub u8);
impl EChaosClothTetherMode {
    pub const FAST_TETHER_FAST_LENGTH: EChaosClothTetherMode = EChaosClothTetherMode(0);
    pub const ACCURATE_TETHER_FAST_LENGTH: EChaosClothTetherMode = EChaosClothTetherMode(
        1,
    );
    pub const ACCURATE_TETHER_ACCURATE_LENGTH: EChaosClothTetherMode = EChaosClothTetherMode(
        2,
    );
    pub const MAX_CHAOS_CLOTH_TETHER_MODE: EChaosClothTetherMode = EChaosClothTetherMode(
        3,
    );
}
