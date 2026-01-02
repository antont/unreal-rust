#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FSlatePostSettings {
    pub flags_0: u8,
    pub resolution: ESlatePostResolution,
    pub post_processor_class: TSubclassOf<USlateRHIPostBufferProcessor>,
    __padding_end: [u8; 32],
}
impl FSlatePostSettings {}
#[repr(C, align(8))]
pub struct USlateFXSubsystem {
    __padding_end: [u8; 216],
}
impl USlateFXSubsystem {}
#[repr(C, align(8))]
pub struct USlateRHIPostBufferProcessor {
    __padding_end: [u8; 48],
}
impl USlateRHIPostBufferProcessor {}
#[repr(C, align(8))]
pub struct USlatePostBufferBlur {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub gaussian_blur_strength: f32,
    __padding_end: [u8; 20],
}
impl USlatePostBufferBlur {}
#[repr(C, align(8))]
pub struct USlateFontBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl USlateFontBlueprintLibrary {}
#[repr(C, align(8))]
pub struct USlateRHIRendererSettings {
    __padding_end: [u8; 184],
}
impl USlateRHIRendererSettings {}
#[repr(transparent)]
pub struct ESlatePostResolution(pub u8);
impl ESlatePostResolution {
    pub const FULL: ESlatePostResolution = ESlatePostResolution(0);
    pub const HALF: ESlatePostResolution = ESlatePostResolution(1);
}
