#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FCacheRecorderUserParameters {
    pub b_maximize_viewport: bool,
    pub engine_time_dilation: f32,
    pub b_reset_playhead: bool,
    pub b_stop_at_playback_end: bool,
}
#[repr(C, align(8))]
pub struct FCacheRecorderProjectParameters {
    pub default_slate: FString,
    pub b_cache_track_recorder_controls_clock_time: bool,
    pub recording_clock_source: EUpdateClockSource,
    pub b_start_at_current_timecode: bool,
    pub b_record_timecode: bool,
    pub b_show_notifications: bool,
}
#[repr(C, align(8))]
pub struct FCacheRecorderParameters {
    pub user: FCacheRecorderUserParameters,
    pub project: FCacheRecorderProjectParameters,
    pub start_frame: FFrameNumber,
}
#[repr(C, align(8))]
pub struct FCachedTrackSource {
    pub recorder: UPtr<UMovieSceneTrackRecorder>,
}
pub struct UCacheTrackRecorder {
    pub sequence_asset: UPtr<ULevelSequence>,
    pub weak_world: TWeakObjectPtr<UWorld>,
    pub parameters: FCacheRecorderParameters,
    pub cache_tracks: TArray<FCachedTrackSource>,
}
