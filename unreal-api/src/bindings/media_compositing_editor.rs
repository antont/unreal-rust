#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UMediaPlayerRecording {
    __padding_end: [u8; 112],
}
impl UMediaPlayerRecording {}
#[repr(C, align(8))]
pub struct UMediaSequenceRecorderSettings {
    __padding_end: [u8; 72],
}
impl UMediaSequenceRecorderSettings {}
#[repr(C, align(8))]
pub struct UMediaCompositingTrackFilter {
    __padding_end: [u8; 48],
}
impl UMediaCompositingTrackFilter {}
#[repr(transparent)]
pub struct EMediaPlayerRecordingNumerationStyle(pub u8);
impl EMediaPlayerRecordingNumerationStyle {
    pub const APPEND_FRAME_NUMBER: EMediaPlayerRecordingNumerationStyle = EMediaPlayerRecordingNumerationStyle(
        0,
    );
    pub const APPEND_SAMPLE_TIME: EMediaPlayerRecordingNumerationStyle = EMediaPlayerRecordingNumerationStyle(
        1,
    );
}
#[repr(transparent)]
pub struct EMediaPlayerRecordingImageFormat(pub u8);
impl EMediaPlayerRecordingImageFormat {
    pub const PNG: EMediaPlayerRecordingImageFormat = EMediaPlayerRecordingImageFormat(
        0,
    );
    pub const JPEG: EMediaPlayerRecordingImageFormat = EMediaPlayerRecordingImageFormat(
        1,
    );
    pub const BMP: EMediaPlayerRecordingImageFormat = EMediaPlayerRecordingImageFormat(
        2,
    );
    pub const EXR: EMediaPlayerRecordingImageFormat = EMediaPlayerRecordingImageFormat(
        3,
    );
}
