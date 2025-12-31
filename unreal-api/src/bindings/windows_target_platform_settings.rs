#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UWindowsTargetSettings {
    pub default_graphics_rhi: EDefaultGraphicsRHI,
    pub targeted_rh_is_deprecated: TArray<FString>,
    pub d3d12_targeted_shader_formats: TArray<FString>,
    pub d3d11_targeted_shader_formats: TArray<FString>,
    pub vulkan_targeted_shader_formats: TArray<FString>,
    pub b_generate_nanite_fallback_meshes: bool,
    pub compiler: ECompilerVersion,
    pub audio_sample_rate: i32,
    pub audio_callback_buffer_frame_size: i32,
    pub audio_num_buffers_to_enqueue: i32,
    pub audio_max_channels: i32,
    pub audio_num_source_workers: i32,
    pub spatialization_plugin: FString,
    pub source_data_override_plugin: FString,
    pub reverb_plugin: FString,
    pub occlusion_plugin: FString,
    pub compression_overrides: crate::bindings::audio_platform_configuration::FPlatformRuntimeAudioCompressionOverrides,
    pub cache_size_kb: i32,
    pub max_chunk_size_override_kb: i32,
    pub b_resample_for_device: bool,
    pub max_sample_rate: f32,
    pub high_sample_rate: f32,
    pub med_sample_rate: f32,
    pub low_sample_rate: f32,
    pub min_sample_rate: f32,
    pub compression_quality_modifier: f32,
    pub auto_streaming_threshold: f32,
    pub sound_cue_cook_quality_index: i32,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDefaultGraphicsRHI(pub u8);
impl EDefaultGraphicsRHI {
    pub const DEFAULT_GRAPHICS_RHI_DEFAULT: EDefaultGraphicsRHI = EDefaultGraphicsRHI(0);
    pub const DEFAULT_GRAPHICS_RHI_DX11: EDefaultGraphicsRHI = EDefaultGraphicsRHI(1);
    pub const DEFAULT_GRAPHICS_RHI_DX12: EDefaultGraphicsRHI = EDefaultGraphicsRHI(2);
    pub const DEFAULT_GRAPHICS_RHI_VULKAN: EDefaultGraphicsRHI = EDefaultGraphicsRHI(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECompilerVersion(pub u8);
impl ECompilerVersion {
    pub const DEFAULT: ECompilerVersion = ECompilerVersion(0);
    pub const VISUAL_STUDIO2015: ECompilerVersion = ECompilerVersion(1);
    pub const VISUAL_STUDIO2017: ECompilerVersion = ECompilerVersion(2);
    pub const VISUAL_STUDIO2019: ECompilerVersion = ECompilerVersion(3);
    pub const VISUAL_STUDIO2022: ECompilerVersion = ECompilerVersion(4);
    pub const VISUAL_STUDIO2026: ECompilerVersion = ECompilerVersion(5);
}
