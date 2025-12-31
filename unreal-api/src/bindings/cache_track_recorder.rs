#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
    pub recording_clock_source: crate::bindings::movie_scene::EUpdateClockSource,
    pub b_start_at_current_timecode: bool,
    pub b_record_timecode: bool,
    pub b_show_notifications: bool,
}
#[repr(C, align(8))]
pub struct FCacheRecorderParameters {
    pub user: FCacheRecorderUserParameters,
    pub project: FCacheRecorderProjectParameters,
    pub start_frame: crate::bindings::core_u_object::FFrameNumber,
}
#[repr(C, align(8))]
pub struct FCachedTrackSource {
    pub recorder: UPtr<crate::bindings::take_track_recorders::UMovieSceneTrackRecorder>,
}
pub struct UCacheTrackRecorder {
    pub sequence_asset: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    pub weak_world: TWeakObjectPtr<crate::bindings::engine::UWorld>,
    pub parameters: FCacheRecorderParameters,
    pub cache_tracks: TArray<FCachedTrackSource>,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECacheTrackRecorderState(pub u8);
impl ECacheTrackRecorderState {
    pub const STARTING: ECacheTrackRecorderState = ECacheTrackRecorderState(0);
    pub const PRE_RECORD: ECacheTrackRecorderState = ECacheTrackRecorderState(1);
    pub const TICKING_AFTER_PRE: ECacheTrackRecorderState = ECacheTrackRecorderState(2);
    pub const STARTED: ECacheTrackRecorderState = ECacheTrackRecorderState(3);
    pub const STOPPED: ECacheTrackRecorderState = ECacheTrackRecorderState(4);
    pub const CANCELLED: ECacheTrackRecorderState = ECacheTrackRecorderState(5);
}
