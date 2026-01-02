#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UShaderPlatformQualitySettings {
    __padding_end: [u8; 96],
}
impl UShaderPlatformQualitySettings {}
#[repr(C, align(8))]
pub struct UMaterialShaderQualitySettings {
    __padding_end: [u8; 152],
}
impl UMaterialShaderQualitySettings {}
#[repr(transparent)]
pub struct EMobileShadowQuality(pub u8);
impl EMobileShadowQuality {
    pub const NO_FILTERING: EMobileShadowQuality = EMobileShadowQuality(0);
    pub const PCF_1X1: EMobileShadowQuality = EMobileShadowQuality(1);
    pub const PCF_3X3: EMobileShadowQuality = EMobileShadowQuality(2);
    pub const PCF_5X5: EMobileShadowQuality = EMobileShadowQuality(3);
}
