#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
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
    pub compression_overrides: FPlatformRuntimeAudioCompressionOverrides,
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
