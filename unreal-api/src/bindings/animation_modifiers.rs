#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAnimationModifierSettings {
    __padding_end: [u8; 128],
}
impl UAnimationModifierSettings {}
#[repr(C, align(8))]
pub struct UAnimationModifier {
    __padding_end: [u8; 120],
}
impl UAnimationModifier {}
#[repr(C, align(8))]
pub struct UAnimationModifiersAssetUserData {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub animation_modifier_instances: TArray<UPtr<UAnimationModifier>>,
    pub applied_modifiers: TMap<
        crate::bindings::core_u_object::FSoftObjectPath,
        UPtr<UAnimationModifier>,
    >,
}
impl UAnimationModifiersAssetUserData {}
