#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FMediaPlayerRecordingSettings {
    pub b_active: bool,
    pub b_record_media_frame: bool,
    pub base_filename: FString,
    pub numeration_style: EMediaPlayerRecordingNumerationStyle,
    pub image_format: EMediaPlayerRecordingImageFormat,
    pub compression_quality: i32,
    pub b_reset_alpha: bool,
}
pub struct UMediaPlayerRecording {
    pub recording_settings: FMediaPlayerRecordingSettings,
    pub media_player_to_record: TWeakObjectPtr<
        crate::bindings::media_assets::UMediaPlayer,
    >,
}
pub struct UMediaSequenceRecorderSettings {
    pub b_record_media_player_enabled: bool,
    pub media_player_sub_directory: FString,
}
pub struct UMediaCompositingTrackFilter {}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
