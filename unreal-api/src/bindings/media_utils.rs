#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
impl FMediaPlayerTrackOptions {}
#[repr(C, align(8))]
pub struct FMediaPlayerInitialTrackLanguageSelection {
    pub video: FString,
    pub audio: FString,
    pub subtitle: FString,
    pub caption: FString,
}
impl FMediaPlayerInitialTrackLanguageSelection {}
#[repr(C, align(8))]
pub struct FMediaPlayerOptions {
    pub tracks: FMediaPlayerTrackOptions,
    pub tracks_by_language: FMediaPlayerInitialTrackLanguageSelection,
    pub track_selection: EMediaPlayerOptionTrackSelectMode,
    pub seek_time: crate::bindings::core_u_object::FTimespan,
    pub seek_time_type: EMediaPlayerOptionSeekTimeType,
    pub play_on_open: EMediaPlayerOptionBooleanOverride,
    pub loop_: EMediaPlayerOptionBooleanOverride,
    pub(crate) __padding_end: [u8; 85],
}
impl FMediaPlayerOptions {}
#[repr(transparent)]
pub struct EMediaPlayerOptionTrackSelectMode(pub u8);
impl EMediaPlayerOptionTrackSelectMode {
    pub const USE_MEDIA_PLAYER_DEFAULTS: EMediaPlayerOptionTrackSelectMode = EMediaPlayerOptionTrackSelectMode(
        0,
    );
    pub const USE_TRACK_OPTION_INDICES: EMediaPlayerOptionTrackSelectMode = EMediaPlayerOptionTrackSelectMode(
        1,
    );
    pub const USE_LANGUAGE_CODES: EMediaPlayerOptionTrackSelectMode = EMediaPlayerOptionTrackSelectMode(
        2,
    );
}
#[repr(transparent)]
pub struct EMediaPlayerOptionSeekTimeType(pub u8);
impl EMediaPlayerOptionSeekTimeType {
    pub const IGNORED: EMediaPlayerOptionSeekTimeType = EMediaPlayerOptionSeekTimeType(
        0,
    );
    pub const RELATIVE_TO_START_TIME: EMediaPlayerOptionSeekTimeType = EMediaPlayerOptionSeekTimeType(
        1,
    );
}
#[repr(transparent)]
pub struct EMediaPlayerOptionBooleanOverride(pub u8);
impl EMediaPlayerOptionBooleanOverride {
    pub const USE_MEDIA_PLAYER_SETTING: EMediaPlayerOptionBooleanOverride = EMediaPlayerOptionBooleanOverride(
        0,
    );
    pub const ENABLED: EMediaPlayerOptionBooleanOverride = EMediaPlayerOptionBooleanOverride(
        1,
    );
    pub const DISABLED: EMediaPlayerOptionBooleanOverride = EMediaPlayerOptionBooleanOverride(
        2,
    );
}
