#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UHardwareTargetingSettings {
    __padding_end: [u8; 88],
}
impl UHardwareTargetingSettings {}
#[repr(transparent)]
pub struct EHardwareClass(pub u8);
impl EHardwareClass {
    pub const UNSPECIFIED: EHardwareClass = EHardwareClass(0);
    pub const DESKTOP: EHardwareClass = EHardwareClass(1);
    pub const MOBILE: EHardwareClass = EHardwareClass(2);
}
#[repr(transparent)]
pub struct EGraphicsPreset(pub u8);
impl EGraphicsPreset {
    pub const UNSPECIFIED: EGraphicsPreset = EGraphicsPreset(0);
    pub const MAXIMUM: EGraphicsPreset = EGraphicsPreset(1);
    pub const SCALABLE: EGraphicsPreset = EGraphicsPreset(2);
}
