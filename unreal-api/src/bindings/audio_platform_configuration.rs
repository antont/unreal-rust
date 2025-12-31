#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FPlatformRuntimeAudioCompressionOverrides {
    pub b_override_compression_times: bool,
    pub duration_threshold: f32,
    pub max_num_random_branches: i32,
    pub sound_cue_quality_index: i32,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESoundwaveSampleRateSettings(pub u8);
impl ESoundwaveSampleRateSettings {
    pub const MAX: ESoundwaveSampleRateSettings = ESoundwaveSampleRateSettings(0);
    pub const HIGH: ESoundwaveSampleRateSettings = ESoundwaveSampleRateSettings(1);
    pub const MEDIUM: ESoundwaveSampleRateSettings = ESoundwaveSampleRateSettings(2);
    pub const LOW: ESoundwaveSampleRateSettings = ESoundwaveSampleRateSettings(3);
    pub const MIN: ESoundwaveSampleRateSettings = ESoundwaveSampleRateSettings(4);
}
