#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FVariantDependency {
    pub variant_set: TSoftObjectPtr<UVariantSet>,
    pub variant: TSoftObjectPtr<UVariant>,
    pub b_enabled: bool,
    __padding_end: [u8; 7],
}
impl FVariantDependency {}
#[repr(C, align(8))]
pub struct ULevelVariantSets {
    __padding_end: [u8; 192],
}
impl ULevelVariantSets {}
#[repr(C, align(8))]
pub struct ALevelVariantSetsActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub level_variant_sets: crate::bindings::core_u_object::FSoftObjectPath,
    __padding_end: [u8; 80],
}
impl ALevelVariantSetsActor {}
#[repr(C, align(8))]
pub struct ULevelVariantSetsFunctionDirector {
    __padding_end: [u8; 80],
}
impl ULevelVariantSetsFunctionDirector {}
#[repr(C, align(8))]
pub struct UPropertyValue {
    __padding_end: [u8; 488],
}
impl UPropertyValue {}
#[repr(C, align(8))]
pub struct UPropertyValueTransform {
    __padding_end: [u8; 488],
}
impl UPropertyValueTransform {}
#[repr(C, align(8))]
pub struct UPropertyValueVisibility {
    __padding_end: [u8; 488],
}
impl UPropertyValueVisibility {}
#[repr(C, align(8))]
pub struct UPropertyValueColor {
    __padding_end: [u8; 488],
}
impl UPropertyValueColor {}
#[repr(C, align(8))]
pub struct UPropertyValueMaterial {
    __padding_end: [u8; 488],
}
impl UPropertyValueMaterial {}
#[repr(C, align(8))]
pub struct UPropertyValueOption {
    __padding_end: [u8; 488],
}
impl UPropertyValueOption {}
#[repr(C, align(8))]
pub struct UPropertyValueSoftObject {
    __padding_end: [u8; 488],
}
impl UPropertyValueSoftObject {}
#[repr(C, align(8))]
pub struct ASwitchActor {
    __padding_end: [u8; 1184],
}
impl ASwitchActor {}
#[repr(C, align(8))]
pub struct UVariant {
    __padding_end: [u8; 128],
}
impl UVariant {}
#[repr(C, align(8))]
pub struct UVariantObjectBinding {
    __padding_end: [u8; 160],
}
impl UVariantObjectBinding {}
#[repr(C, align(8))]
pub struct UVariantSet {
    __padding_end: [u8; 112],
}
impl UVariantSet {}
#[repr(transparent)]
pub struct EPropertyValueCategory(pub u8);
impl EPropertyValueCategory {
    pub const UNDEFINED: EPropertyValueCategory = EPropertyValueCategory(0);
    pub const GENERIC: EPropertyValueCategory = EPropertyValueCategory(1);
    pub const RELATIVE_LOCATION: EPropertyValueCategory = EPropertyValueCategory(2);
    pub const RELATIVE_ROTATION: EPropertyValueCategory = EPropertyValueCategory(4);
    pub const RELATIVE_SCALE3_D: EPropertyValueCategory = EPropertyValueCategory(8);
    pub const VISIBILITY: EPropertyValueCategory = EPropertyValueCategory(16);
    pub const MATERIAL: EPropertyValueCategory = EPropertyValueCategory(32);
    pub const COLOR: EPropertyValueCategory = EPropertyValueCategory(64);
    pub const OPTION: EPropertyValueCategory = EPropertyValueCategory(128);
}
