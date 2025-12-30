#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FMediaPlayerTrackOptions {
    pub audio: i32,
    pub caption: i32,
    pub metadata: i32,
    pub script: i32,
    pub subtitle: i32,
    pub text: i32,
    pub video: i32,
}
#[repr(C, align(8))]
pub struct FMediaPlayerInitialTrackLanguageSelection {
    pub video: FString,
    pub audio: FString,
    pub subtitle: FString,
    pub caption: FString,
}
#[repr(C, align(8))]
pub struct FMediaPlayerOptions {
    pub tracks: FMediaPlayerTrackOptions,
    pub tracks_by_language: FMediaPlayerInitialTrackLanguageSelection,
    pub track_selection: EMediaPlayerOptionTrackSelectMode,
    pub seek_time: FTimespan,
    pub seek_time_type: EMediaPlayerOptionSeekTimeType,
    pub play_on_open: EMediaPlayerOptionBooleanOverride,
    pub loop_: EMediaPlayerOptionBooleanOverride,
}
