#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UMacTargetSettings {
    __padding_end: [u8; 176],
}
impl UMacTargetSettings {}
#[repr(C, align(8))]
pub struct UXcodeProjectSettings {
    __padding_end: [u8; 392],
}
impl UXcodeProjectSettings {}
#[repr(transparent)]
pub struct EMacTargetArchitecture(pub u8);
impl EMacTargetArchitecture {
    pub const MAC_TARGET_ARCHITECTURE_INTEL: EMacTargetArchitecture = EMacTargetArchitecture(
        0,
    );
    pub const MAC_TARGET_ARCHITECTURE_UNIVERSAL: EMacTargetArchitecture = EMacTargetArchitecture(
        1,
    );
    pub const MAC_TARGET_ARCHITECTURE_APPLE_SILICON: EMacTargetArchitecture = EMacTargetArchitecture(
        2,
    );
    pub const MAC_TARGET_ARCHITECTURE_HOST: EMacTargetArchitecture = EMacTargetArchitecture(
        3,
    );
}
