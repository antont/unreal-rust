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
    __padding_end: [u8; 7],
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
