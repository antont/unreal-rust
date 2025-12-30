#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UImgMediaSettings {
    pub default_frame_rate: FFrameRate,
    pub bandwidth_throttling_enabled: bool,
    pub cache_behind_percentage: f32,
    pub cache_size_gb: f32,
    pub cache_threads: i32,
    pub cache_thread_stack_size_kb: i32,
    pub global_cache_size_gb: f32,
    pub use_global_cache: bool,
    pub exr_decoder_threads: u32,
    pub default_proxy: FString,
    pub use_default_proxy: bool,
}
