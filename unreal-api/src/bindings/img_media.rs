#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FImgMediaSourceCustomizationSequenceProxy {}
#[repr(C, align(8))]
pub struct FMediaSourceColorSettings {
    pub encoding_override: EMediaSourceEncoding,
    pub color_space_override: crate::bindings::engine::ETextureColorSpace,
    pub red_chromaticity_coordinate: crate::bindings::core_u_object::FVector2D,
    pub green_chromaticity_coordinate: crate::bindings::core_u_object::FVector2D,
    pub blue_chromaticity_coordinate: crate::bindings::core_u_object::FVector2D,
    pub white_chromaticity_coordinate: crate::bindings::core_u_object::FVector2D,
    pub chromatic_adaptation_method: crate::bindings::engine::ETextureChromaticAdaptationMethod,
}
pub struct UImgMediaSource {
    pub is_path_relative_to_project_root_deprecated: bool,
    pub frame_rate_override: crate::bindings::core_u_object::FFrameRate,
    pub proxy_override: FString,
    pub b_fill_gaps_in_sequence: bool,
    pub sequence_proxy: FImgMediaSourceCustomizationSequenceProxy,
    pub start_timecode: crate::bindings::core_u_object::FTimecode,
    pub source_color_settings: FMediaSourceColorSettings,
    pub sequence_path: crate::bindings::core_u_object::FDirectoryPath,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMediaSourceEncoding(pub u8);
impl EMediaSourceEncoding {
    pub const MSE_NONE: EMediaSourceEncoding = EMediaSourceEncoding(0);
    pub const MSE_LINEAR: EMediaSourceEncoding = EMediaSourceEncoding(1);
    pub const MSE_S_RGB: EMediaSourceEncoding = EMediaSourceEncoding(2);
    pub const MSE_ST2084: EMediaSourceEncoding = EMediaSourceEncoding(3);
    pub const MSE_S_LOG3: EMediaSourceEncoding = EMediaSourceEncoding(12);
}
