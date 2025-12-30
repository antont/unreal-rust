#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UWmfMediaSettings {
    pub allow_non_standard_codecs: bool,
    pub low_latency: bool,
    pub native_audio_out: bool,
    pub hardware_accelerated_video_decoding: bool,
}
