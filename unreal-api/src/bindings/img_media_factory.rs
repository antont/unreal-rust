#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UImgMediaSettings {
    pub default_frame_rate: crate::bindings::core_u_object::FFrameRate,
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
