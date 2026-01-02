#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UClothConfigCommon {
    __padding_end: [u8; 48],
}
impl UClothConfigCommon {}
#[repr(C, align(8))]
pub struct UClothSharedConfigCommon {
    __padding_end: [u8; 48],
}
impl UClothSharedConfigCommon {}
#[repr(C, align(8))]
pub struct UClothingAssetCustomData {
    __padding_end: [u8; 48],
}
impl UClothingAssetCustomData {}
#[repr(C, align(8))]
pub struct UClothingAssetCommon {
    #[doc(hidden)]
    __padding_88: [u8; 88],
    pub cloth_configs: TMap<
        FName,
        UPtr<crate::bindings::clothing_system_runtime_interface::UClothConfigBase>,
    >,
    __padding_end: [u8; 416],
}
impl UClothingAssetCommon {}
#[repr(C, align(8))]
pub struct UClothLODDataCommon_Legacy {
    __padding_end: [u8; 544],
}
impl UClothLODDataCommon_Legacy {}
#[repr(transparent)]
pub struct EClothingWindMethod_Legacy(pub u8);
impl EClothingWindMethod_Legacy {
    pub const LEGACY: EClothingWindMethod_Legacy = EClothingWindMethod_Legacy(0);
    pub const ACCURATE: EClothingWindMethod_Legacy = EClothingWindMethod_Legacy(1);
}
#[repr(transparent)]
pub struct EWeightMapTargetCommon(pub u8);
impl EWeightMapTargetCommon {
    pub const NONE: EWeightMapTargetCommon = EWeightMapTargetCommon(0);
    pub const MAX_DISTANCE: EWeightMapTargetCommon = EWeightMapTargetCommon(1);
    pub const BACKSTOP_DISTANCE: EWeightMapTargetCommon = EWeightMapTargetCommon(2);
    pub const BACKSTOP_RADIUS: EWeightMapTargetCommon = EWeightMapTargetCommon(3);
    pub const ANIM_DRIVE_STIFFNESS: EWeightMapTargetCommon = EWeightMapTargetCommon(4);
    pub const ANIM_DRIVE_DAMPING_DEPRECATED: EWeightMapTargetCommon = EWeightMapTargetCommon(
        5,
    );
    pub const FIRST_USER_TARGET: EWeightMapTargetCommon = EWeightMapTargetCommon(6);
    pub const LAST_USER_TARGET: EWeightMapTargetCommon = EWeightMapTargetCommon(200);
    pub const TETHER_ENDS_MASK: EWeightMapTargetCommon = EWeightMapTargetCommon(201);
}
#[repr(transparent)]
pub struct EClothMassMode(pub u8);
impl EClothMassMode {
    pub const UNIFORM_MASS: EClothMassMode = EClothMassMode(0);
    pub const TOTAL_MASS: EClothMassMode = EClothMassMode(1);
    pub const DENSITY: EClothMassMode = EClothMassMode(2);
    pub const MAX_CLOTH_MASS_MODE: EClothMassMode = EClothMassMode(3);
}
