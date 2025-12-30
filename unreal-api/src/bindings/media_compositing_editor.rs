#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
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
    pub media_player_to_record: TWeakObjectPtr<UMediaPlayer>,
}
pub struct UMediaSequenceRecorderSettings {
    pub b_record_media_player_enabled: bool,
    pub media_player_sub_directory: FString,
}
pub struct UMediaCompositingTrackFilter {}
