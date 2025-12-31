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
    pub path_to_slate_post_rt: FString,
    pub cached_slate_post_rt: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
}
pub struct USlateFXSubsystem {
    pub slate_post_buffer_processors: TMap<
        crate::bindings::slate_core::ESlatePostRT,
        UPtr<USlateRHIPostBufferProcessor>,
    >,
}
pub struct USlateRHIPostBufferProcessor {}
pub struct USlatePostBufferBlur {
    pub gaussian_blur_strength: f32,
}
pub struct USlateFontBlueprintLibrary {}
pub struct USlateRHIRendererSettings {
    pub slate_post_settings: TMap<
        crate::bindings::slate_core::ESlatePostRT,
        FSlatePostSettings,
    >,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESlatePostResolution(pub u8);
impl ESlatePostResolution {
    pub const FULL: ESlatePostResolution = ESlatePostResolution(0);
    pub const HALF: ESlatePostResolution = ESlatePostResolution(1);
}
