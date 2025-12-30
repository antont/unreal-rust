#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
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
