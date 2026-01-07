#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut U_MEDIA_SOURCE_VALIDATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_SOURCE_SET_MEDIA_OPTION_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_SOURCE_SET_MEDIA_OPTION_INT64: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_SOURCE_SET_MEDIA_OPTION_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_SOURCE_SET_MEDIA_OPTION_BOOL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_SOURCE_GET_URL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_FILE_MEDIA_SOURCE_SET_FILE_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_COMPONENT_GET_MEDIA_TEXTURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_COMPONENT_GET_MEDIA_PLAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_SUPPORTS_SEEKING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_SUPPORTS_SCRUBBING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_SUPPORTS_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_SUPPORTS_PLAYBACK_TIME_RANGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_SET_VIEW_ROTATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_SET_VIEW_FIELD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_SET_VIDEO_TRACK_FRAME_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_SET_TRACK_FORMAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_SET_TIME_DELAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_SET_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_SET_PLAYBACK_TIME_RANGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_SET_NATIVE_VOLUME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_SET_MEDIA_OPTIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_SET_LOOPING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_SET_DESIRED_PLAYER_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_SET_BLOCK_ON_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_SELECT_TRACK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_SEEK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_SCRUB: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_REWIND: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_REOPEN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_PREVIOUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_PLAY_AND_SEEK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_PLAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_PAUSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_OPEN_URL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_OPEN_SOURCE_WITH_OPTIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_OPEN_SOURCE_LATENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_OPEN_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_OPEN_PLAYLIST_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_OPEN_PLAYLIST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_OPEN_FILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_NEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_IS_READY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_IS_PREPARING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_IS_PLAYING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_IS_PAUSED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_IS_LOOPING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_IS_CONNECTING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_IS_CLOSED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_IS_BUFFERING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_HAS_ERROR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_VIEW_ROTATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_VIDEO_TRACK_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_VIDEO_TRACK_FRAME_RATES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_VIDEO_TRACK_FRAME_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_VIDEO_TRACK_DIMENSIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_VIDEO_TRACK_ASPECT_RATIO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_VERTICAL_FIELD_OF_VIEW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_URL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_TRACK_LANGUAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_TRACK_FORMAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_TRACK_DISPLAY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_TIME_STAMP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_TIME_DELAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_SUPPORTED_RATES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_SELECTED_TRACK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_PLAYLIST_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_PLAYLIST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_PLAYER_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_PLAYBACK_TIME_RANGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_NUM_TRACKS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_NUM_TRACK_FORMATS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_MEDIA_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_MEDIA_METADATA_ITEMS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_HORIZONTAL_FIELD_OF_VIEW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_DURATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_DISPLAY_TIME_STAMP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_DISPLAY_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_DESIRED_PLAYER_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_AUDIO_TRACK_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_AUDIO_TRACK_SAMPLE_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_GET_AUDIO_TRACK_CHANNELS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_CLOSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_CAN_PLAY_URL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_CAN_PLAY_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYER_CAN_PAUSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYLIST_REPLACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYLIST_REMOVE_AT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYLIST_REMOVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYLIST_NUM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYLIST_INSERT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYLIST_GET_RANDOM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYLIST_GET_PREVIOUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYLIST_GET_NEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYLIST_GET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYLIST_ADD_URL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYLIST_ADD_FILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLAYLIST_ADD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_SOUND_COMPONENT_SET_SPECTRAL_ANALYSIS_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_SOUND_COMPONENT_SET_MEDIA_PLAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_SOUND_COMPONENT_SET_ENVELOPE_FOLLOWINGSETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_SOUND_COMPONENT_SET_ENABLE_SPECTRAL_ANALYSIS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_SOUND_COMPONENT_SET_ENABLE_ENVELOPE_FOLLOWING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_SOUND_COMPONENT_GET_SPECTRAL_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_SOUND_COMPONENT_GET_NORMALIZED_SPECTRAL_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_SOUND_COMPONENT_GET_MEDIA_PLAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_SOUND_COMPONENT_GET_ENVELOPE_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_SOUND_COMPONENT_BP_GET_ATTENUATION_SETTINGS_TO_APPLY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_TEXTURE_UPDATE_RESOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_TEXTURE_SET_MEDIA_PLAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_TEXTURE_GET_WIDTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_TEXTURE_GET_TEXTURE_NUM_MIPS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_TEXTURE_GET_MEDIA_PLAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_TEXTURE_GET_HEIGHT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_TEXTURE_GET_ASPECT_RATIO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_BLUEPRINT_FUNCTION_LIBRARY_ENUMERATE_WEBCAM_CAPTURE_DEVICES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_BLUEPRINT_FUNCTION_LIBRARY_ENUMERATE_VIDEO_CAPTURE_DEVICES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_BLUEPRINT_FUNCTION_LIBRARY_ENUMERATE_AUDIO_CAPTURE_DEVICES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMediaSource::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Validate"),
            &raw mut U_MEDIA_SOURCE_VALIDATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMediaOptionString"),
            &raw mut U_MEDIA_SOURCE_SET_MEDIA_OPTION_STRING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMediaOptionInt64"),
            &raw mut U_MEDIA_SOURCE_SET_MEDIA_OPTION_INT64,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMediaOptionFloat"),
            &raw mut U_MEDIA_SOURCE_SET_MEDIA_OPTION_FLOAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMediaOptionBool"),
            &raw mut U_MEDIA_SOURCE_SET_MEDIA_OPTION_BOOL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUrl"),
            &raw mut U_MEDIA_SOURCE_GET_URL,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UFileMediaSource::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilePath"),
            &raw mut U_FILE_MEDIA_SOURCE_SET_FILE_PATH,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMediaComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMediaTexture"),
            &raw mut U_MEDIA_COMPONENT_GET_MEDIA_TEXTURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMediaPlayer"),
            &raw mut U_MEDIA_COMPONENT_GET_MEDIA_PLAYER,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMediaPlayer::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SupportsSeeking"),
            &raw mut U_MEDIA_PLAYER_SUPPORTS_SEEKING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SupportsScrubbing"),
            &raw mut U_MEDIA_PLAYER_SUPPORTS_SCRUBBING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SupportsRate"),
            &raw mut U_MEDIA_PLAYER_SUPPORTS_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SupportsPlaybackTimeRange"),
            &raw mut U_MEDIA_PLAYER_SUPPORTS_PLAYBACK_TIME_RANGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetViewRotation"),
            &raw mut U_MEDIA_PLAYER_SET_VIEW_ROTATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetViewField"),
            &raw mut U_MEDIA_PLAYER_SET_VIEW_FIELD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVideoTrackFrameRate"),
            &raw mut U_MEDIA_PLAYER_SET_VIDEO_TRACK_FRAME_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTrackFormat"),
            &raw mut U_MEDIA_PLAYER_SET_TRACK_FORMAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTimeDelay"),
            &raw mut U_MEDIA_PLAYER_SET_TIME_DELAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRate"),
            &raw mut U_MEDIA_PLAYER_SET_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlaybackTimeRange"),
            &raw mut U_MEDIA_PLAYER_SET_PLAYBACK_TIME_RANGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNativeVolume"),
            &raw mut U_MEDIA_PLAYER_SET_NATIVE_VOLUME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMediaOptions"),
            &raw mut U_MEDIA_PLAYER_SET_MEDIA_OPTIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLooping"),
            &raw mut U_MEDIA_PLAYER_SET_LOOPING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDesiredPlayerName"),
            &raw mut U_MEDIA_PLAYER_SET_DESIRED_PLAYER_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlockOnTime"),
            &raw mut U_MEDIA_PLAYER_SET_BLOCK_ON_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectTrack"),
            &raw mut U_MEDIA_PLAYER_SELECT_TRACK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Seek"),
            &raw mut U_MEDIA_PLAYER_SEEK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Scrub"),
            &raw mut U_MEDIA_PLAYER_SCRUB,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Rewind"),
            &raw mut U_MEDIA_PLAYER_REWIND,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Reopen"),
            &raw mut U_MEDIA_PLAYER_REOPEN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Previous"),
            &raw mut U_MEDIA_PLAYER_PREVIOUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayAndSeek"),
            &raw mut U_MEDIA_PLAYER_PLAY_AND_SEEK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Play"),
            &raw mut U_MEDIA_PLAYER_PLAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Pause"),
            &raw mut U_MEDIA_PLAYER_PAUSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OpenUrl"),
            &raw mut U_MEDIA_PLAYER_OPEN_URL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OpenSourceWithOptions"),
            &raw mut U_MEDIA_PLAYER_OPEN_SOURCE_WITH_OPTIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OpenSourceLatent"),
            &raw mut U_MEDIA_PLAYER_OPEN_SOURCE_LATENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OpenSource"),
            &raw mut U_MEDIA_PLAYER_OPEN_SOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OpenPlaylistIndex"),
            &raw mut U_MEDIA_PLAYER_OPEN_PLAYLIST_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OpenPlaylist"),
            &raw mut U_MEDIA_PLAYER_OPEN_PLAYLIST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OpenFile"),
            &raw mut U_MEDIA_PLAYER_OPEN_FILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Next"),
            &raw mut U_MEDIA_PLAYER_NEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsReady"),
            &raw mut U_MEDIA_PLAYER_IS_READY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPreparing"),
            &raw mut U_MEDIA_PLAYER_IS_PREPARING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPlaying"),
            &raw mut U_MEDIA_PLAYER_IS_PLAYING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPaused"),
            &raw mut U_MEDIA_PLAYER_IS_PAUSED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLooping"),
            &raw mut U_MEDIA_PLAYER_IS_LOOPING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsConnecting"),
            &raw mut U_MEDIA_PLAYER_IS_CONNECTING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsClosed"),
            &raw mut U_MEDIA_PLAYER_IS_CLOSED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsBuffering"),
            &raw mut U_MEDIA_PLAYER_IS_BUFFERING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasError"),
            &raw mut U_MEDIA_PLAYER_HAS_ERROR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetViewRotation"),
            &raw mut U_MEDIA_PLAYER_GET_VIEW_ROTATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVideoTrackType"),
            &raw mut U_MEDIA_PLAYER_GET_VIDEO_TRACK_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVideoTrackFrameRates"),
            &raw mut U_MEDIA_PLAYER_GET_VIDEO_TRACK_FRAME_RATES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVideoTrackFrameRate"),
            &raw mut U_MEDIA_PLAYER_GET_VIDEO_TRACK_FRAME_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVideoTrackDimensions"),
            &raw mut U_MEDIA_PLAYER_GET_VIDEO_TRACK_DIMENSIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVideoTrackAspectRatio"),
            &raw mut U_MEDIA_PLAYER_GET_VIDEO_TRACK_ASPECT_RATIO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVerticalFieldOfView"),
            &raw mut U_MEDIA_PLAYER_GET_VERTICAL_FIELD_OF_VIEW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUrl"),
            &raw mut U_MEDIA_PLAYER_GET_URL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTrackLanguage"),
            &raw mut U_MEDIA_PLAYER_GET_TRACK_LANGUAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTrackFormat"),
            &raw mut U_MEDIA_PLAYER_GET_TRACK_FORMAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTrackDisplayName"),
            &raw mut U_MEDIA_PLAYER_GET_TRACK_DISPLAY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTimeStamp"),
            &raw mut U_MEDIA_PLAYER_GET_TIME_STAMP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTimeDelay"),
            &raw mut U_MEDIA_PLAYER_GET_TIME_DELAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTime"),
            &raw mut U_MEDIA_PLAYER_GET_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSupportedRates"),
            &raw mut U_MEDIA_PLAYER_GET_SUPPORTED_RATES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedTrack"),
            &raw mut U_MEDIA_PLAYER_GET_SELECTED_TRACK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRate"),
            &raw mut U_MEDIA_PLAYER_GET_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlaylistIndex"),
            &raw mut U_MEDIA_PLAYER_GET_PLAYLIST_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlaylist"),
            &raw mut U_MEDIA_PLAYER_GET_PLAYLIST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlayerName"),
            &raw mut U_MEDIA_PLAYER_GET_PLAYER_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlaybackTimeRange"),
            &raw mut U_MEDIA_PLAYER_GET_PLAYBACK_TIME_RANGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumTracks"),
            &raw mut U_MEDIA_PLAYER_GET_NUM_TRACKS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumTrackFormats"),
            &raw mut U_MEDIA_PLAYER_GET_NUM_TRACK_FORMATS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMediaName"),
            &raw mut U_MEDIA_PLAYER_GET_MEDIA_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMediaMetadataItems"),
            &raw mut U_MEDIA_PLAYER_GET_MEDIA_METADATA_ITEMS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHorizontalFieldOfView"),
            &raw mut U_MEDIA_PLAYER_GET_HORIZONTAL_FIELD_OF_VIEW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDuration"),
            &raw mut U_MEDIA_PLAYER_GET_DURATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDisplayTimeStamp"),
            &raw mut U_MEDIA_PLAYER_GET_DISPLAY_TIME_STAMP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDisplayTime"),
            &raw mut U_MEDIA_PLAYER_GET_DISPLAY_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDesiredPlayerName"),
            &raw mut U_MEDIA_PLAYER_GET_DESIRED_PLAYER_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAudioTrackType"),
            &raw mut U_MEDIA_PLAYER_GET_AUDIO_TRACK_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAudioTrackSampleRate"),
            &raw mut U_MEDIA_PLAYER_GET_AUDIO_TRACK_SAMPLE_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAudioTrackChannels"),
            &raw mut U_MEDIA_PLAYER_GET_AUDIO_TRACK_CHANNELS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Close"),
            &raw mut U_MEDIA_PLAYER_CLOSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanPlayUrl"),
            &raw mut U_MEDIA_PLAYER_CAN_PLAY_URL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanPlaySource"),
            &raw mut U_MEDIA_PLAYER_CAN_PLAY_SOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanPause"),
            &raw mut U_MEDIA_PLAYER_CAN_PAUSE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMediaPlaylist::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Replace"),
            &raw mut U_MEDIA_PLAYLIST_REPLACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAt"),
            &raw mut U_MEDIA_PLAYLIST_REMOVE_AT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Remove"),
            &raw mut U_MEDIA_PLAYLIST_REMOVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Num"),
            &raw mut U_MEDIA_PLAYLIST_NUM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Insert"),
            &raw mut U_MEDIA_PLAYLIST_INSERT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRandom"),
            &raw mut U_MEDIA_PLAYLIST_GET_RANDOM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPrevious"),
            &raw mut U_MEDIA_PLAYLIST_GET_PREVIOUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNext"),
            &raw mut U_MEDIA_PLAYLIST_GET_NEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Get"),
            &raw mut U_MEDIA_PLAYLIST_GET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddUrl"),
            &raw mut U_MEDIA_PLAYLIST_ADD_URL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddFile"),
            &raw mut U_MEDIA_PLAYLIST_ADD_FILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Add"),
            &raw mut U_MEDIA_PLAYLIST_ADD,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMediaSoundComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSpectralAnalysisSettings"),
            &raw mut U_MEDIA_SOUND_COMPONENT_SET_SPECTRAL_ANALYSIS_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMediaPlayer"),
            &raw mut U_MEDIA_SOUND_COMPONENT_SET_MEDIA_PLAYER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnvelopeFollowingsettings"),
            &raw mut U_MEDIA_SOUND_COMPONENT_SET_ENVELOPE_FOLLOWINGSETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnableSpectralAnalysis"),
            &raw mut U_MEDIA_SOUND_COMPONENT_SET_ENABLE_SPECTRAL_ANALYSIS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnableEnvelopeFollowing"),
            &raw mut U_MEDIA_SOUND_COMPONENT_SET_ENABLE_ENVELOPE_FOLLOWING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSpectralData"),
            &raw mut U_MEDIA_SOUND_COMPONENT_GET_SPECTRAL_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNormalizedSpectralData"),
            &raw mut U_MEDIA_SOUND_COMPONENT_GET_NORMALIZED_SPECTRAL_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMediaPlayer"),
            &raw mut U_MEDIA_SOUND_COMPONENT_GET_MEDIA_PLAYER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEnvelopeValue"),
            &raw mut U_MEDIA_SOUND_COMPONENT_GET_ENVELOPE_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_GetAttenuationSettingsToApply"),
            &raw mut U_MEDIA_SOUND_COMPONENT_BP_GET_ATTENUATION_SETTINGS_TO_APPLY,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMediaTexture::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpdateResource"),
            &raw mut U_MEDIA_TEXTURE_UPDATE_RESOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMediaPlayer"),
            &raw mut U_MEDIA_TEXTURE_SET_MEDIA_PLAYER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWidth"),
            &raw mut U_MEDIA_TEXTURE_GET_WIDTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTextureNumMips"),
            &raw mut U_MEDIA_TEXTURE_GET_TEXTURE_NUM_MIPS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMediaPlayer"),
            &raw mut U_MEDIA_TEXTURE_GET_MEDIA_PLAYER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHeight"),
            &raw mut U_MEDIA_TEXTURE_GET_HEIGHT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAspectRatio"),
            &raw mut U_MEDIA_TEXTURE_GET_ASPECT_RATIO,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMediaBlueprintFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnumerateWebcamCaptureDevices"),
            &raw mut U_MEDIA_BLUEPRINT_FUNCTION_LIBRARY_ENUMERATE_WEBCAM_CAPTURE_DEVICES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnumerateVideoCaptureDevices"),
            &raw mut U_MEDIA_BLUEPRINT_FUNCTION_LIBRARY_ENUMERATE_VIDEO_CAPTURE_DEVICES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnumerateAudioCaptureDevices"),
            &raw mut U_MEDIA_BLUEPRINT_FUNCTION_LIBRARY_ENUMERATE_AUDIO_CAPTURE_DEVICES,
        );
    }
}
#[repr(C, align(8))]
pub struct FMediaMetadataItemBPT {
    pub language_code: FString,
    pub mime_type: FString,
    pub string_data: FString,
    pub binary_data: TArray<u8>,
}
impl FMediaMetadataItemBPT {}
#[repr(C, align(8))]
pub struct FMediaMetadataItemsBPT {
    pub items: TArray<FMediaMetadataItemBPT>,
}
impl FMediaMetadataItemsBPT {}
#[repr(C, align(4))]
pub struct FMediaSoundComponentSpectralData {
    pub frequency_hz: f32,
    pub magnitude: f32,
}
impl FMediaSoundComponentSpectralData {}
#[repr(C, align(4))]
pub struct FMediaSourceCacheSettings {
    __padding_end: [u8; 8],
}
impl FMediaSourceCacheSettings {}
#[repr(C, align(8))]
pub struct FMediaCaptureDevice {
    pub display_name: FText,
    pub url: FString,
}
impl FMediaCaptureDevice {}
pub struct IMediaSourceRendererInterface {}
#[repr(C, align(8))]
pub struct UMediaSourceRendererInterface {
    __padding_end: [u8; 48],
}
impl UMediaSourceRendererInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaSourceRendererInterface")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMediaSource {
    __padding_end: [u8; 152],
}
impl UMediaSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaSource")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn validate(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_SOURCE_VALIDATE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_SOURCE_VALIDATE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn set_media_option_string(&mut self, key: &FName, value: FString) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_SOURCE_SET_MEDIA_OPTION_STRING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(16).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_SOURCE_SET_MEDIA_OPTION_STRING,
                __buffer,
            )
        };
    }
    pub fn set_media_option_int64(&mut self, key: &FName, value: i64) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_SOURCE_SET_MEDIA_OPTION_INT64,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(16).cast::<i64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_SOURCE_SET_MEDIA_OPTION_INT64,
                __buffer,
            )
        };
    }
    pub fn set_media_option_float(&mut self, key: &FName, value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_SOURCE_SET_MEDIA_OPTION_FLOAT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(12).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_SOURCE_SET_MEDIA_OPTION_FLOAT,
                __buffer,
            )
        };
    }
    pub fn set_media_option_bool(&mut self, key: &FName, value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_SOURCE_SET_MEDIA_OPTION_BOOL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(12).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_SOURCE_SET_MEDIA_OPTION_BOOL,
                __buffer,
            )
        };
    }
    pub fn get_url(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_SOURCE_GET_URL,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_SOURCE_GET_URL,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct UBaseMediaSource {
    #[doc(hidden)]
    __padding_152: [u8; 152],
    pub platform_player_names: TMap<FString, FName>,
    __padding_end: [u8; 96],
}
impl UBaseMediaSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseMediaSource")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UFileMediaSource {
    #[doc(hidden)]
    __padding_328: [u8; 328],
    pub file_path: FString,
    pub precache_file: bool,
    __padding_end: [u8; 23],
}
impl UFileMediaSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFileMediaSource")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_file_path(&mut self, path: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_FILE_MEDIA_SOURCE_SET_FILE_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&path, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_FILE_MEDIA_SOURCE_SET_FILE_PATH,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UMediaComponent {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub media_texture: UPtr<UMediaTexture>,
    pub media_player: UPtr<UMediaPlayer>,
}
impl UMediaComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_media_texture(&self) -> UPtr<UMediaTexture> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_COMPONENT_GET_MEDIA_TEXTURE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_COMPONENT_GET_MEDIA_TEXTURE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UMediaTexture>>().read() }
    }
    pub fn get_media_player(&self) -> UPtr<UMediaPlayer> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_COMPONENT_GET_MEDIA_PLAYER,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_COMPONENT_GET_MEDIA_PLAYER,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UMediaPlayer>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMediaTimeStampInfo {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub time: crate::bindings::core_u_object::FTimespan,
    pub sequence_index: i64,
}
impl UMediaTimeStampInfo {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaTimeStampInfo")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMediaPlayer {
    #[doc(hidden)]
    __padding_320: [u8; 320],
    pub cache_ahead: crate::bindings::core_u_object::FTimespan,
    pub cache_behind: crate::bindings::core_u_object::FTimespan,
    pub cache_behind_game: crate::bindings::core_u_object::FTimespan,
    pub native_audio_out: bool,
    pub play_on_open: bool,
    #[doc(hidden)]
    __padding_348: [u8; 2],
    pub flags_348: u8,
    pub playlist: UPtr<UMediaPlaylist>,
    pub playlist_index: i32,
    pub time_delay: crate::bindings::core_u_object::FTimespan,
    #[doc(hidden)]
    __padding_496: [u8; 120],
    pub affected_by_pie_handling: bool,
}
impl UMediaPlayer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaPlayer")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn supports_seeking(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_SUPPORTS_SEEKING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_SUPPORTS_SEEKING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn supports_scrubbing(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_SUPPORTS_SCRUBBING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_SUPPORTS_SCRUBBING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn supports_rate(&self, rate: f32, unthinned: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<6>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_SUPPORTS_RATE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&rate, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&unthinned, __buffer.add(4).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_SUPPORTS_RATE,
                __buffer,
            )
        };
        unsafe { __buffer.add(5).cast::<bool>().read() }
    }
    pub fn supports_playback_time_range(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_SUPPORTS_PLAYBACK_TIME_RANGE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_SUPPORTS_PLAYBACK_TIME_RANGE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn set_view_rotation(
        &mut self,
        rotation: &crate::bindings::core_u_object::FRotator,
        absolute: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_VIEW_ROTATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                rotation,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&absolute, __buffer.add(24).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_VIEW_ROTATION,
                __buffer,
            )
        };
        unsafe { __buffer.add(25).cast::<bool>().read() }
    }
    pub fn set_view_field(
        &mut self,
        horizontal: f32,
        vertical: f32,
        absolute: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_VIEW_FIELD,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&horizontal, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&vertical, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&absolute, __buffer.add(8).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_VIEW_FIELD,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn set_video_track_frame_rate(
        &mut self,
        track_index: i32,
        format_index: i32,
        frame_rate: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_VIDEO_TRACK_FRAME_RATE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &format_index,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&frame_rate, __buffer.add(8).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_VIDEO_TRACK_FRAME_RATE,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn set_track_format(
        &mut self,
        track_type: EMediaPlayerTrack,
        track_index: i32,
        format_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_TRACK_FORMAT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_type,
                __buffer.add(0).cast::<EMediaPlayerTrack>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_index,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &format_index,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_TRACK_FORMAT,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn set_time_delay(
        &mut self,
        time_delay: crate::bindings::core_u_object::FTimespan,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_TIME_DELAY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_delay,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTimespan>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_TIME_DELAY,
                __buffer,
            )
        };
    }
    pub fn set_rate(&mut self, rate: f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_RATE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&rate, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_RATE,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_playback_time_range(
        &mut self,
        in_time_range: crate::bindings::core_u_object::FFloatInterval,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_PLAYBACK_TIME_RANGE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_time_range,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFloatInterval>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_PLAYBACK_TIME_RANGE,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_native_volume(&mut self, volume: f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_NATIVE_VOLUME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&volume, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_NATIVE_VOLUME,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_media_options(&mut self, options: UPtr<UMediaSource>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_MEDIA_OPTIONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &options,
                __buffer.add(0).cast::<UPtr<UMediaSource>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_MEDIA_OPTIONS,
                __buffer,
            )
        };
    }
    pub fn set_looping(&mut self, looping: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_LOOPING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&looping, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_LOOPING,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_desired_player_name(&mut self, player_name: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_DESIRED_PLAYER_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_DESIRED_PLAYER_NAME,
                __buffer,
            )
        };
    }
    pub fn set_block_on_time(
        &mut self,
        time: &crate::bindings::core_u_object::FTimespan,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_BLOCK_ON_TIME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                time,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTimespan>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_SET_BLOCK_ON_TIME,
                __buffer,
            )
        };
    }
    pub fn select_track(
        &mut self,
        track_type: EMediaPlayerTrack,
        track_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_SELECT_TRACK,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_type,
                __buffer.add(0).cast::<EMediaPlayerTrack>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_index,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_SELECT_TRACK,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn seek(&mut self, time: &crate::bindings::core_u_object::FTimespan) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_SEEK,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                time,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTimespan>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_SEEK,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn scrub(&mut self, time: &crate::bindings::core_u_object::FTimespan) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_SCRUB,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                time,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTimespan>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_SCRUB,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn rewind(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_REWIND,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_REWIND,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn reopen(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_REOPEN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_REOPEN,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn previous(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_PREVIOUS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_PREVIOUS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn play_and_seek(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_PLAY_AND_SEEK,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_PLAY_AND_SEEK,
                __buffer,
            )
        };
    }
    pub fn play(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_PLAY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_PLAY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn pause(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_PAUSE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_PAUSE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn open_url(&mut self, url: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_OPEN_URL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&url, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_OPEN_URL,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn open_source_with_options(
        &mut self,
        media_source: UPtr<UMediaSource>,
        options: &crate::bindings::media_utils::FMediaPlayerOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<209>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_OPEN_SOURCE_WITH_OPTIONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &media_source,
                __buffer.add(0).cast::<UPtr<UMediaSource>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::media_utils::FMediaPlayerOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_OPEN_SOURCE_WITH_OPTIONS,
                __buffer,
            )
        };
        unsafe { __buffer.add(208).cast::<bool>().read() }
    }
    pub fn open_source_latent(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        latent_info: crate::bindings::engine::FLatentActionInfo,
        media_source: UPtr<UMediaSource>,
        options: &crate::bindings::media_utils::FMediaPlayerOptions,
        b_success: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<249>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_OPEN_SOURCE_LATENT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &latent_info,
                __buffer.add(8).cast::<crate::bindings::engine::FLatentActionInfo>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &media_source,
                __buffer.add(40).cast::<UPtr<UMediaSource>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::media_utils::FMediaPlayerOptions>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_success,
                __buffer.add(248).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_OPEN_SOURCE_LATENT,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(248).cast::<bool>().swap(b_success);
        }
    }
    pub fn open_source(&mut self, media_source: UPtr<UMediaSource>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_OPEN_SOURCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &media_source,
                __buffer.add(0).cast::<UPtr<UMediaSource>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_OPEN_SOURCE,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn open_playlist_index(
        &mut self,
        in_playlist: UPtr<UMediaPlaylist>,
        index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_OPEN_PLAYLIST_INDEX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_playlist,
                __buffer.add(0).cast::<UPtr<UMediaPlaylist>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_OPEN_PLAYLIST_INDEX,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn open_playlist(&mut self, in_playlist: UPtr<UMediaPlaylist>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_OPEN_PLAYLIST,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_playlist,
                __buffer.add(0).cast::<UPtr<UMediaPlaylist>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_OPEN_PLAYLIST,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn open_file(&mut self, file_path: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_OPEN_FILE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &file_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_OPEN_FILE,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn next(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_NEXT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_NEXT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_ready(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_IS_READY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_IS_READY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_preparing(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_IS_PREPARING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_IS_PREPARING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_playing(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_IS_PLAYING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_IS_PLAYING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_paused(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_IS_PAUSED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_IS_PAUSED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_looping(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_IS_LOOPING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_IS_LOOPING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_connecting(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_IS_CONNECTING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_IS_CONNECTING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_closed(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_IS_CLOSED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_IS_CLOSED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_buffering(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_IS_BUFFERING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_IS_BUFFERING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_error(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_HAS_ERROR,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_HAS_ERROR,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_view_rotation(&self) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_VIEW_ROTATION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_VIEW_ROTATION,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_video_track_type(&self, track_index: i32, format_index: i32) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_VIDEO_TRACK_TYPE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &format_index,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_VIDEO_TRACK_TYPE,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn get_video_track_frame_rates(
        &self,
        track_index: i32,
        format_index: i32,
    ) -> crate::bindings::core_u_object::FFloatRange {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_VIDEO_TRACK_FRAME_RATES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &format_index,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_VIDEO_TRACK_FRAME_RATES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FFloatRange>().read()
        }
    }
    pub fn get_video_track_frame_rate(
        &self,
        track_index: i32,
        format_index: i32,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_VIDEO_TRACK_FRAME_RATE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &format_index,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_VIDEO_TRACK_FRAME_RATE,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_video_track_dimensions(
        &self,
        track_index: i32,
        format_index: i32,
    ) -> crate::bindings::core_u_object::FIntPoint {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_VIDEO_TRACK_DIMENSIONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &format_index,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_VIDEO_TRACK_DIMENSIONS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FIntPoint>().read()
        }
    }
    pub fn get_video_track_aspect_ratio(
        &self,
        track_index: i32,
        format_index: i32,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_VIDEO_TRACK_ASPECT_RATIO,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &format_index,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_VIDEO_TRACK_ASPECT_RATIO,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_vertical_field_of_view(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_VERTICAL_FIELD_OF_VIEW,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_VERTICAL_FIELD_OF_VIEW,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_url(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_URL,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_URL,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_track_language(
        &self,
        track_type: EMediaPlayerTrack,
        track_index: i32,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_TRACK_LANGUAGE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_type,
                __buffer.add(0).cast::<EMediaPlayerTrack>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_index,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_TRACK_LANGUAGE,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn get_track_format(
        &self,
        track_type: EMediaPlayerTrack,
        track_index: i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_TRACK_FORMAT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_type,
                __buffer.add(0).cast::<EMediaPlayerTrack>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_index,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_TRACK_FORMAT,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_track_display_name(
        &self,
        track_type: EMediaPlayerTrack,
        track_index: i32,
    ) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_TRACK_DISPLAY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_type,
                __buffer.add(0).cast::<EMediaPlayerTrack>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_index,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_TRACK_DISPLAY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FText>().read() }
    }
    pub fn get_time_stamp(&self) -> UPtr<UMediaTimeStampInfo> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_TIME_STAMP,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_TIME_STAMP,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UMediaTimeStampInfo>>().read() }
    }
    pub fn get_time_delay(&self) -> crate::bindings::core_u_object::FTimespan {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_TIME_DELAY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_TIME_DELAY,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FTimespan>().read()
        }
    }
    pub fn get_time(&self) -> crate::bindings::core_u_object::FTimespan {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_TIME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_TIME,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FTimespan>().read()
        }
    }
    pub fn get_supported_rates(
        &self,
        out_rates: &mut TArray<crate::bindings::core_u_object::FFloatRange>,
        unthinned: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_SUPPORTED_RATES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_rates,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FFloatRange>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &unthinned,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_SUPPORTED_RATES,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FFloatRange>>()
                .swap(out_rates);
        }
    }
    pub fn get_selected_track(&self, track_type: EMediaPlayerTrack) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_SELECTED_TRACK,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_type,
                __buffer.add(0).cast::<EMediaPlayerTrack>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_SELECTED_TRACK,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn get_rate(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_RATE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_RATE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_playlist_index(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_PLAYLIST_INDEX,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_PLAYLIST_INDEX,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_playlist(&self) -> UPtr<UMediaPlaylist> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_PLAYLIST,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_PLAYLIST,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UMediaPlaylist>>().read() }
    }
    pub fn get_player_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_PLAYER_NAME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_PLAYER_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_playback_time_range(
        &mut self,
        in_range_to_get: EMediaTimeRangeBPType,
    ) -> crate::bindings::core_u_object::FFloatInterval {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_PLAYBACK_TIME_RANGE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_range_to_get,
                __buffer.add(0).cast::<EMediaTimeRangeBPType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_PLAYBACK_TIME_RANGE,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(4)
                .cast::<crate::bindings::core_u_object::FFloatInterval>()
                .read()
        }
    }
    pub fn get_num_tracks(&self, track_type: EMediaPlayerTrack) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_NUM_TRACKS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_type,
                __buffer.add(0).cast::<EMediaPlayerTrack>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_NUM_TRACKS,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn get_num_track_formats(
        &self,
        track_type: EMediaPlayerTrack,
        track_index: i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_NUM_TRACK_FORMATS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_type,
                __buffer.add(0).cast::<EMediaPlayerTrack>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_index,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_NUM_TRACK_FORMATS,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_media_name(&self) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_MEDIA_NAME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_MEDIA_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
    pub fn get_media_metadata_items(&self) -> TMap<FString, FMediaMetadataItemsBPT> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_MEDIA_METADATA_ITEMS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_MEDIA_METADATA_ITEMS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TMap<FString, FMediaMetadataItemsBPT>>().read() }
    }
    pub fn get_horizontal_field_of_view(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_HORIZONTAL_FIELD_OF_VIEW,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_HORIZONTAL_FIELD_OF_VIEW,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_duration(&self) -> crate::bindings::core_u_object::FTimespan {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_DURATION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_DURATION,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FTimespan>().read()
        }
    }
    pub fn get_display_time_stamp(&self) -> UPtr<UMediaTimeStampInfo> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_DISPLAY_TIME_STAMP,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_DISPLAY_TIME_STAMP,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UMediaTimeStampInfo>>().read() }
    }
    pub fn get_display_time(&self) -> crate::bindings::core_u_object::FTimespan {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_DISPLAY_TIME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_DISPLAY_TIME,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FTimespan>().read()
        }
    }
    pub fn get_desired_player_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_DESIRED_PLAYER_NAME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_DESIRED_PLAYER_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_audio_track_type(&self, track_index: i32, format_index: i32) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_AUDIO_TRACK_TYPE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &format_index,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_AUDIO_TRACK_TYPE,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn get_audio_track_sample_rate(
        &self,
        track_index: i32,
        format_index: i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_AUDIO_TRACK_SAMPLE_RATE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &format_index,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_AUDIO_TRACK_SAMPLE_RATE,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_audio_track_channels(&self, track_index: i32, format_index: i32) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_AUDIO_TRACK_CHANNELS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &format_index,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_GET_AUDIO_TRACK_CHANNELS,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn close(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_CLOSE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_CLOSE,
                __buffer,
            )
        };
    }
    pub fn can_play_url(&mut self, url: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_CAN_PLAY_URL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&url, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_CAN_PLAY_URL,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn can_play_source(&mut self, media_source: UPtr<UMediaSource>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_CAN_PLAY_SOURCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &media_source,
                __buffer.add(0).cast::<UPtr<UMediaSource>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_CAN_PLAY_SOURCE,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn can_pause(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYER_CAN_PAUSE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYER_CAN_PAUSE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
pub struct IMediaPlayerProxyInterface {}
#[repr(C, align(8))]
pub struct UMediaPlayerProxyInterface {
    __padding_end: [u8; 48],
}
impl UMediaPlayerProxyInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaPlayerProxyInterface")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMediaPlaylist {
    __padding_end: [u8; 64],
}
impl UMediaPlaylist {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaPlaylist")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn replace(&mut self, index: i32, replacement: UPtr<UMediaSource>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_REPLACE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &replacement,
                __buffer.add(8).cast::<UPtr<UMediaSource>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_REPLACE,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_at(&mut self, index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_REMOVE_AT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_REMOVE_AT,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn remove(&mut self, media_source: UPtr<UMediaSource>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_REMOVE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &media_source,
                __buffer.add(0).cast::<UPtr<UMediaSource>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_REMOVE,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn num(&mut self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_NUM,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_NUM,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn insert(&mut self, media_source: UPtr<UMediaSource>, index: i32) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_INSERT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &media_source,
                __buffer.add(0).cast::<UPtr<UMediaSource>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_INSERT,
                __buffer,
            )
        };
    }
    pub fn get_random(&mut self, out_index: &mut i32) -> UPtr<UMediaSource> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_GET_RANDOM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(out_index, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_GET_RANDOM,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(out_index);
        }
        unsafe { __buffer.add(8).cast::<UPtr<UMediaSource>>().read() }
    }
    pub fn get_previous(&mut self, in_out_index: &mut i32) -> UPtr<UMediaSource> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_GET_PREVIOUS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_GET_PREVIOUS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(in_out_index);
        }
        unsafe { __buffer.add(8).cast::<UPtr<UMediaSource>>().read() }
    }
    pub fn get_next(&mut self, in_out_index: &mut i32) -> UPtr<UMediaSource> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_GET_NEXT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_GET_NEXT,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(in_out_index);
        }
        unsafe { __buffer.add(8).cast::<UPtr<UMediaSource>>().read() }
    }
    pub fn get(&mut self, index: i32) -> UPtr<UMediaSource> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_GET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_GET,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UMediaSource>>().read() }
    }
    pub fn add_url(&mut self, url: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_ADD_URL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&url, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_ADD_URL,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_file(&mut self, file_path: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_ADD_FILE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &file_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_ADD_FILE,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add(&mut self, media_source: UPtr<UMediaSource>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_ADD,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &media_source,
                __buffer.add(0).cast::<UPtr<UMediaSource>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_PLAYLIST_ADD,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
}
#[repr(C, align(16))]
pub struct UMediaSoundComponent {
    __padding_end: [u8; 2608],
}
impl UMediaSoundComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaSoundComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_spectral_analysis_settings(
        &mut self,
        in_frequencies_to_analyze: TArray<f32>,
        in_fft_size: EMediaSoundComponentFFTSize,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_SOUND_COMPONENT_SET_SPECTRAL_ANALYSIS_SETTINGS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_frequencies_to_analyze,
                __buffer.add(0).cast::<TArray<f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_fft_size,
                __buffer.add(16).cast::<EMediaSoundComponentFFTSize>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_SOUND_COMPONENT_SET_SPECTRAL_ANALYSIS_SETTINGS,
                __buffer,
            )
        };
    }
    pub fn set_media_player(&mut self, new_media_player: UPtr<UMediaPlayer>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_SOUND_COMPONENT_SET_MEDIA_PLAYER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_media_player,
                __buffer.add(0).cast::<UPtr<UMediaPlayer>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_SOUND_COMPONENT_SET_MEDIA_PLAYER,
                __buffer,
            )
        };
    }
    pub fn set_envelope_followingsettings(
        &mut self,
        attack_time_msec: i32,
        release_time_msec: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_SOUND_COMPONENT_SET_ENVELOPE_FOLLOWINGSETTINGS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attack_time_msec,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &release_time_msec,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_SOUND_COMPONENT_SET_ENVELOPE_FOLLOWINGSETTINGS,
                __buffer,
            )
        };
    }
    pub fn set_enable_spectral_analysis(
        &mut self,
        b_in_spectral_analysis_enabled: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_SOUND_COMPONENT_SET_ENABLE_SPECTRAL_ANALYSIS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_spectral_analysis_enabled,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_SOUND_COMPONENT_SET_ENABLE_SPECTRAL_ANALYSIS,
                __buffer,
            )
        };
    }
    pub fn set_enable_envelope_following(&mut self, b_in_envelope_following: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_SOUND_COMPONENT_SET_ENABLE_ENVELOPE_FOLLOWING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_envelope_following,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_SOUND_COMPONENT_SET_ENABLE_ENVELOPE_FOLLOWING,
                __buffer,
            )
        };
    }
    pub fn get_spectral_data(&mut self) -> TArray<FMediaSoundComponentSpectralData> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_SOUND_COMPONENT_GET_SPECTRAL_DATA,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_SOUND_COMPONENT_GET_SPECTRAL_DATA,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FMediaSoundComponentSpectralData>>().read()
        }
    }
    pub fn get_normalized_spectral_data(
        &mut self,
    ) -> TArray<FMediaSoundComponentSpectralData> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_SOUND_COMPONENT_GET_NORMALIZED_SPECTRAL_DATA,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_SOUND_COMPONENT_GET_NORMALIZED_SPECTRAL_DATA,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FMediaSoundComponentSpectralData>>().read()
        }
    }
    pub fn get_media_player(&self) -> UPtr<UMediaPlayer> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_SOUND_COMPONENT_GET_MEDIA_PLAYER,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_SOUND_COMPONENT_GET_MEDIA_PLAYER,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UMediaPlayer>>().read() }
    }
    pub fn get_envelope_value(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_SOUND_COMPONENT_GET_ENVELOPE_VALUE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_SOUND_COMPONENT_GET_ENVELOPE_VALUE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_attenuation_settings_to_apply(
        &mut self,
        out_attenuation_settings: &mut crate::bindings::engine::FSoundAttenuationSettings,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1025>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_SOUND_COMPONENT_BP_GET_ATTENUATION_SETTINGS_TO_APPLY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_attenuation_settings,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::engine::FSoundAttenuationSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_SOUND_COMPONENT_BP_GET_ATTENUATION_SETTINGS_TO_APPLY,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::engine::FSoundAttenuationSettings>()
                .swap(out_attenuation_settings);
        }
        unsafe { __buffer.add(1024).cast::<bool>().read() }
    }
}
#[repr(C, align(16))]
pub struct UMediaTexture {
    #[doc(hidden)]
    __padding_1240: [u8; 1240],
    pub address_x: crate::bindings::engine::TextureAddress,
    pub address_y: crate::bindings::engine::TextureAddress,
    pub auto_clear: bool,
    pub clear_color: crate::bindings::core_u_object::FLinearColor,
    pub enable_gen_mips: bool,
    #[doc(hidden)]
    __padding_1262: [u8; 1],
    pub new_style_output: bool,
    pub current_aspect_ratio: f32,
    pub current_orientation: MediaTextureOrientation,
    __padding_end: [u8; 203],
}
impl UMediaTexture {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaTexture")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn update_resource(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_TEXTURE_UPDATE_RESOURCE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_TEXTURE_UPDATE_RESOURCE,
                __buffer,
            )
        };
    }
    pub fn set_media_player(&mut self, new_media_player: UPtr<UMediaPlayer>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_TEXTURE_SET_MEDIA_PLAYER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_media_player,
                __buffer.add(0).cast::<UPtr<UMediaPlayer>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_TEXTURE_SET_MEDIA_PLAYER,
                __buffer,
            )
        };
    }
    pub fn get_width(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_TEXTURE_GET_WIDTH,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_TEXTURE_GET_WIDTH,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_texture_num_mips(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_TEXTURE_GET_TEXTURE_NUM_MIPS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_TEXTURE_GET_TEXTURE_NUM_MIPS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_media_player(&self) -> UPtr<UMediaPlayer> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_TEXTURE_GET_MEDIA_PLAYER,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_TEXTURE_GET_MEDIA_PLAYER,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UMediaPlayer>>().read() }
    }
    pub fn get_height(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_TEXTURE_GET_HEIGHT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_TEXTURE_GET_HEIGHT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_aspect_ratio(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_TEXTURE_GET_ASPECT_RATIO,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_TEXTURE_GET_ASPECT_RATIO,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPlatformMediaSource {
    __padding_end: [u8; 320],
}
impl UPlatformMediaSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlatformMediaSource")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UStreamMediaSource {
    #[doc(hidden)]
    __padding_328: [u8; 328],
    pub stream_url: FString,
}
impl UStreamMediaSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStreamMediaSource")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTimeSynchronizableMediaSource {
    #[doc(hidden)]
    __padding_328: [u8; 328],
    pub b_use_time_synchronization: bool,
    __padding_end: [u8; 23],
}
impl UTimeSynchronizableMediaSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTimeSynchronizableMediaSource")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMediaBlueprintFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UMediaBlueprintFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaBlueprintFunctionLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn enumerate_webcam_capture_devices(
        out_devices: &mut TArray<FMediaCaptureDevice>,
        filter: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_BLUEPRINT_FUNCTION_LIBRARY_ENUMERATE_WEBCAM_CAPTURE_DEVICES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_devices,
                __buffer.add(0).cast::<TArray<FMediaCaptureDevice>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&filter, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::media_assets::UMediaBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_BLUEPRINT_FUNCTION_LIBRARY_ENUMERATE_WEBCAM_CAPTURE_DEVICES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FMediaCaptureDevice>>().swap(out_devices);
        }
    }
    pub fn enumerate_video_capture_devices(
        out_devices: &mut TArray<FMediaCaptureDevice>,
        filter: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_BLUEPRINT_FUNCTION_LIBRARY_ENUMERATE_VIDEO_CAPTURE_DEVICES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_devices,
                __buffer.add(0).cast::<TArray<FMediaCaptureDevice>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&filter, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::media_assets::UMediaBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_BLUEPRINT_FUNCTION_LIBRARY_ENUMERATE_VIDEO_CAPTURE_DEVICES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FMediaCaptureDevice>>().swap(out_devices);
        }
    }
    pub fn enumerate_audio_capture_devices(
        out_devices: &mut TArray<FMediaCaptureDevice>,
        filter: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_assets::U_MEDIA_BLUEPRINT_FUNCTION_LIBRARY_ENUMERATE_AUDIO_CAPTURE_DEVICES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_devices,
                __buffer.add(0).cast::<TArray<FMediaCaptureDevice>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&filter, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::media_assets::UMediaBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::U_MEDIA_BLUEPRINT_FUNCTION_LIBRARY_ENUMERATE_AUDIO_CAPTURE_DEVICES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FMediaCaptureDevice>>().swap(out_devices);
        }
    }
}
#[repr(C, align(8))]
pub struct FMediaPlayer_OnEndReached {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMediaPlayer_OnMediaClosed {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMediaPlayer_OnMediaOpened {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMediaPlayer_OnMediaOpenFailed {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMediaPlayer_OnPlaybackResumed {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMediaPlayer_OnPlaybackSuspended {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMediaPlayer_OnSeekCompleted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMediaPlayer_OnTracksChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMediaPlayer_OnMetadataChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMediaPlayer_OnBufferingStart {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMediaPlayer_OnBufferingCompleted {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EMediaPlayerTrack(pub u8);
impl EMediaPlayerTrack {
    pub const AUDIO: EMediaPlayerTrack = EMediaPlayerTrack(0);
    pub const CAPTION: EMediaPlayerTrack = EMediaPlayerTrack(1);
    pub const METADATA: EMediaPlayerTrack = EMediaPlayerTrack(2);
    pub const SCRIPT: EMediaPlayerTrack = EMediaPlayerTrack(3);
    pub const SUBTITLE: EMediaPlayerTrack = EMediaPlayerTrack(4);
    pub const TEXT: EMediaPlayerTrack = EMediaPlayerTrack(5);
    pub const VIDEO: EMediaPlayerTrack = EMediaPlayerTrack(6);
}
#[repr(transparent)]
pub struct EMediaTimeRangeBPType(pub u8);
impl EMediaTimeRangeBPType {
    pub const ABSOLUTE: EMediaTimeRangeBPType = EMediaTimeRangeBPType(0);
    pub const CURRENT: EMediaTimeRangeBPType = EMediaTimeRangeBPType(1);
}
#[repr(transparent)]
pub struct EMediaSoundComponentFFTSize(pub u8);
impl EMediaSoundComponentFFTSize {
    pub const MIN_64: EMediaSoundComponentFFTSize = EMediaSoundComponentFFTSize(0);
    pub const SMALL_256: EMediaSoundComponentFFTSize = EMediaSoundComponentFFTSize(1);
    pub const MEDIUM_512: EMediaSoundComponentFFTSize = EMediaSoundComponentFFTSize(2);
    pub const LARGE_1024: EMediaSoundComponentFFTSize = EMediaSoundComponentFFTSize(3);
}
#[repr(transparent)]
pub struct EMediaSoundChannels(pub i32);
impl EMediaSoundChannels {
    pub const MONO: EMediaSoundChannels = EMediaSoundChannels(0);
    pub const STEREO: EMediaSoundChannels = EMediaSoundChannels(1);
    pub const SURROUND: EMediaSoundChannels = EMediaSoundChannels(2);
}
#[repr(transparent)]
pub struct MediaTextureOrientation(pub u8);
impl MediaTextureOrientation {
    pub const MTORI_ORIGINAL: MediaTextureOrientation = MediaTextureOrientation(0);
    pub const MTORI_CW90: MediaTextureOrientation = MediaTextureOrientation(1);
    pub const MTORI_CW180: MediaTextureOrientation = MediaTextureOrientation(2);
    pub const MTORI_CW270: MediaTextureOrientation = MediaTextureOrientation(3);
}
#[repr(transparent)]
pub struct EMediaTextureVisibleMipsTiles(pub u8);
impl EMediaTextureVisibleMipsTiles {
    pub const NONE: EMediaTextureVisibleMipsTiles = EMediaTextureVisibleMipsTiles(0);
    pub const PLANE: EMediaTextureVisibleMipsTiles = EMediaTextureVisibleMipsTiles(1);
    pub const SPHERE: EMediaTextureVisibleMipsTiles = EMediaTextureVisibleMipsTiles(2);
}
