#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FPlatformRuntimeAudioCompressionOverrides {
    pub b_override_compression_times: bool,
    pub duration_threshold: f32,
    pub max_num_random_branches: i32,
    pub sound_cue_quality_index: i32,
}
