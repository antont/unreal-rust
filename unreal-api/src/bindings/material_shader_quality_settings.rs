#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FMaterialQualityOverrides {
    pub b_discard_quality_during_cook: bool,
    pub b_enable_override: bool,
    pub b_force_fully_rough: bool,
    pub b_force_non_metal: bool,
    pub b_force_disable_lm_directionality: bool,
    pub b_force_disable_preintegrated_gf: bool,
    pub b_disable_material_normal_calculation: bool,
    pub mobile_shadow_quality: EMobileShadowQuality,
}
pub struct UShaderPlatformQualitySettings {
    pub quality_overrides: FMaterialQualityOverrides,
}
pub struct UMaterialShaderQualitySettings {
    pub forward_setting_map: TMap<FName, UPtr<UShaderPlatformQualitySettings>>,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMobileShadowQuality(pub u8);
impl EMobileShadowQuality {
    pub const NO_FILTERING: EMobileShadowQuality = EMobileShadowQuality(0);
    pub const PCF_1X1: EMobileShadowQuality = EMobileShadowQuality(1);
    pub const PCF_3X3: EMobileShadowQuality = EMobileShadowQuality(2);
    pub const PCF_5X5: EMobileShadowQuality = EMobileShadowQuality(3);
}
