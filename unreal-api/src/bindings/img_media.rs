#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(1))]
pub struct FImgMediaSourceCustomizationSequenceProxy {}
#[repr(C, align(8))]
pub struct FMediaSourceColorSettings {
    pub encoding_override: EMediaSourceEncoding,
    pub color_space_override: ETextureColorSpace,
    pub red_chromaticity_coordinate: FVector2D,
    pub green_chromaticity_coordinate: FVector2D,
    pub blue_chromaticity_coordinate: FVector2D,
    pub white_chromaticity_coordinate: FVector2D,
    pub chromatic_adaptation_method: ETextureChromaticAdaptationMethod,
}
pub struct UImgMediaSource {
    pub is_path_relative_to_project_root_deprecated: bool,
    pub frame_rate_override: FFrameRate,
    pub proxy_override: FString,
    pub b_fill_gaps_in_sequence: bool,
    pub sequence_proxy: FImgMediaSourceCustomizationSequenceProxy,
    pub start_timecode: FTimecode,
    pub source_color_settings: FMediaSourceColorSettings,
    pub sequence_path: FDirectoryPath,
}
