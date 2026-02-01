#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_media_source_validate: *mut crate::ffi::UFunctionOpague,
    pub u_media_source_set_media_option_string: *mut crate::ffi::UFunctionOpague,
    pub u_media_source_set_media_option_int64: *mut crate::ffi::UFunctionOpague,
    pub u_media_source_set_media_option_float: *mut crate::ffi::UFunctionOpague,
    pub u_media_source_set_media_option_bool: *mut crate::ffi::UFunctionOpague,
    pub u_media_source_get_url: *mut crate::ffi::UFunctionOpague,
    pub u_file_media_source_set_file_path: *mut crate::ffi::UFunctionOpague,
    pub u_media_component_get_media_texture: *mut crate::ffi::UFunctionOpague,
    pub u_media_component_get_media_player: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_supports_seeking: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_supports_scrubbing: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_supports_rate: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_supports_playback_time_range: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_set_view_rotation: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_set_view_field: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_set_video_track_frame_rate: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_set_track_format: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_set_time_delay: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_set_rate: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_set_playback_time_range: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_set_native_volume: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_set_media_options: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_set_looping: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_set_desired_player_name: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_set_block_on_time: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_select_track: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_seek: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_scrub: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_rewind: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_reopen: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_previous: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_play_and_seek: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_play: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_pause: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_open_url: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_open_source_with_options: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_open_source_latent: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_open_source: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_open_playlist_index: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_open_playlist: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_open_file: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_next: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_is_ready: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_is_preparing: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_is_playing: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_is_paused: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_is_looping: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_is_connecting: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_is_closed: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_is_buffering: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_has_error: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_view_rotation: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_video_track_type: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_video_track_frame_rates: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_video_track_frame_rate: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_video_track_dimensions: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_video_track_aspect_ratio: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_vertical_field_of_view: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_url: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_track_language: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_track_format: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_track_display_name: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_time_stamp: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_time_delay: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_time: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_supported_rates: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_selected_track: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_rate: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_playlist_index: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_playlist: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_player_name: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_playback_time_range: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_num_tracks: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_num_track_formats: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_media_name: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_media_metadata_items: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_horizontal_field_of_view: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_duration: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_display_time_stamp: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_display_time: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_desired_player_name: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_audio_track_type: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_audio_track_sample_rate: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_get_audio_track_channels: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_close: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_can_play_url: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_can_play_source: *mut crate::ffi::UFunctionOpague,
    pub u_media_player_can_pause: *mut crate::ffi::UFunctionOpague,
    pub u_media_playlist_replace: *mut crate::ffi::UFunctionOpague,
    pub u_media_playlist_remove_at: *mut crate::ffi::UFunctionOpague,
    pub u_media_playlist_remove: *mut crate::ffi::UFunctionOpague,
    pub u_media_playlist_num: *mut crate::ffi::UFunctionOpague,
    pub u_media_playlist_insert: *mut crate::ffi::UFunctionOpague,
    pub u_media_playlist_get_random: *mut crate::ffi::UFunctionOpague,
    pub u_media_playlist_get_previous: *mut crate::ffi::UFunctionOpague,
    pub u_media_playlist_get_next: *mut crate::ffi::UFunctionOpague,
    pub u_media_playlist_get: *mut crate::ffi::UFunctionOpague,
    pub u_media_playlist_add_url: *mut crate::ffi::UFunctionOpague,
    pub u_media_playlist_add_file: *mut crate::ffi::UFunctionOpague,
    pub u_media_playlist_add: *mut crate::ffi::UFunctionOpague,
    pub u_media_sound_component_set_spectral_analysis_settings: *mut crate::ffi::UFunctionOpague,
    pub u_media_sound_component_set_media_player: *mut crate::ffi::UFunctionOpague,
    pub u_media_sound_component_set_envelope_followingsettings: *mut crate::ffi::UFunctionOpague,
    pub u_media_sound_component_set_enable_spectral_analysis: *mut crate::ffi::UFunctionOpague,
    pub u_media_sound_component_set_enable_envelope_following: *mut crate::ffi::UFunctionOpague,
    pub u_media_sound_component_get_spectral_data: *mut crate::ffi::UFunctionOpague,
    pub u_media_sound_component_get_normalized_spectral_data: *mut crate::ffi::UFunctionOpague,
    pub u_media_sound_component_get_media_player: *mut crate::ffi::UFunctionOpague,
    pub u_media_sound_component_get_envelope_value: *mut crate::ffi::UFunctionOpague,
    pub u_media_sound_component_bp_get_attenuation_settings_to_apply: *mut crate::ffi::UFunctionOpague,
    pub u_media_texture_update_resource: *mut crate::ffi::UFunctionOpague,
    pub u_media_texture_set_media_player: *mut crate::ffi::UFunctionOpague,
    pub u_media_texture_get_width: *mut crate::ffi::UFunctionOpague,
    pub u_media_texture_get_texture_num_mips: *mut crate::ffi::UFunctionOpague,
    pub u_media_texture_get_media_player: *mut crate::ffi::UFunctionOpague,
    pub u_media_texture_get_height: *mut crate::ffi::UFunctionOpague,
    pub u_media_texture_get_aspect_ratio: *mut crate::ffi::UFunctionOpague,
    pub u_media_blueprint_function_library_enumerate_webcam_capture_devices: *mut crate::ffi::UFunctionOpague,
    pub u_media_blueprint_function_library_enumerate_video_capture_devices: *mut crate::ffi::UFunctionOpague,
    pub u_media_blueprint_function_library_enumerate_audio_capture_devices: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_media_source_validate: std::ptr::null_mut(),
            u_media_source_set_media_option_string: std::ptr::null_mut(),
            u_media_source_set_media_option_int64: std::ptr::null_mut(),
            u_media_source_set_media_option_float: std::ptr::null_mut(),
            u_media_source_set_media_option_bool: std::ptr::null_mut(),
            u_media_source_get_url: std::ptr::null_mut(),
            u_file_media_source_set_file_path: std::ptr::null_mut(),
            u_media_component_get_media_texture: std::ptr::null_mut(),
            u_media_component_get_media_player: std::ptr::null_mut(),
            u_media_player_supports_seeking: std::ptr::null_mut(),
            u_media_player_supports_scrubbing: std::ptr::null_mut(),
            u_media_player_supports_rate: std::ptr::null_mut(),
            u_media_player_supports_playback_time_range: std::ptr::null_mut(),
            u_media_player_set_view_rotation: std::ptr::null_mut(),
            u_media_player_set_view_field: std::ptr::null_mut(),
            u_media_player_set_video_track_frame_rate: std::ptr::null_mut(),
            u_media_player_set_track_format: std::ptr::null_mut(),
            u_media_player_set_time_delay: std::ptr::null_mut(),
            u_media_player_set_rate: std::ptr::null_mut(),
            u_media_player_set_playback_time_range: std::ptr::null_mut(),
            u_media_player_set_native_volume: std::ptr::null_mut(),
            u_media_player_set_media_options: std::ptr::null_mut(),
            u_media_player_set_looping: std::ptr::null_mut(),
            u_media_player_set_desired_player_name: std::ptr::null_mut(),
            u_media_player_set_block_on_time: std::ptr::null_mut(),
            u_media_player_select_track: std::ptr::null_mut(),
            u_media_player_seek: std::ptr::null_mut(),
            u_media_player_scrub: std::ptr::null_mut(),
            u_media_player_rewind: std::ptr::null_mut(),
            u_media_player_reopen: std::ptr::null_mut(),
            u_media_player_previous: std::ptr::null_mut(),
            u_media_player_play_and_seek: std::ptr::null_mut(),
            u_media_player_play: std::ptr::null_mut(),
            u_media_player_pause: std::ptr::null_mut(),
            u_media_player_open_url: std::ptr::null_mut(),
            u_media_player_open_source_with_options: std::ptr::null_mut(),
            u_media_player_open_source_latent: std::ptr::null_mut(),
            u_media_player_open_source: std::ptr::null_mut(),
            u_media_player_open_playlist_index: std::ptr::null_mut(),
            u_media_player_open_playlist: std::ptr::null_mut(),
            u_media_player_open_file: std::ptr::null_mut(),
            u_media_player_next: std::ptr::null_mut(),
            u_media_player_is_ready: std::ptr::null_mut(),
            u_media_player_is_preparing: std::ptr::null_mut(),
            u_media_player_is_playing: std::ptr::null_mut(),
            u_media_player_is_paused: std::ptr::null_mut(),
            u_media_player_is_looping: std::ptr::null_mut(),
            u_media_player_is_connecting: std::ptr::null_mut(),
            u_media_player_is_closed: std::ptr::null_mut(),
            u_media_player_is_buffering: std::ptr::null_mut(),
            u_media_player_has_error: std::ptr::null_mut(),
            u_media_player_get_view_rotation: std::ptr::null_mut(),
            u_media_player_get_video_track_type: std::ptr::null_mut(),
            u_media_player_get_video_track_frame_rates: std::ptr::null_mut(),
            u_media_player_get_video_track_frame_rate: std::ptr::null_mut(),
            u_media_player_get_video_track_dimensions: std::ptr::null_mut(),
            u_media_player_get_video_track_aspect_ratio: std::ptr::null_mut(),
            u_media_player_get_vertical_field_of_view: std::ptr::null_mut(),
            u_media_player_get_url: std::ptr::null_mut(),
            u_media_player_get_track_language: std::ptr::null_mut(),
            u_media_player_get_track_format: std::ptr::null_mut(),
            u_media_player_get_track_display_name: std::ptr::null_mut(),
            u_media_player_get_time_stamp: std::ptr::null_mut(),
            u_media_player_get_time_delay: std::ptr::null_mut(),
            u_media_player_get_time: std::ptr::null_mut(),
            u_media_player_get_supported_rates: std::ptr::null_mut(),
            u_media_player_get_selected_track: std::ptr::null_mut(),
            u_media_player_get_rate: std::ptr::null_mut(),
            u_media_player_get_playlist_index: std::ptr::null_mut(),
            u_media_player_get_playlist: std::ptr::null_mut(),
            u_media_player_get_player_name: std::ptr::null_mut(),
            u_media_player_get_playback_time_range: std::ptr::null_mut(),
            u_media_player_get_num_tracks: std::ptr::null_mut(),
            u_media_player_get_num_track_formats: std::ptr::null_mut(),
            u_media_player_get_media_name: std::ptr::null_mut(),
            u_media_player_get_media_metadata_items: std::ptr::null_mut(),
            u_media_player_get_horizontal_field_of_view: std::ptr::null_mut(),
            u_media_player_get_duration: std::ptr::null_mut(),
            u_media_player_get_display_time_stamp: std::ptr::null_mut(),
            u_media_player_get_display_time: std::ptr::null_mut(),
            u_media_player_get_desired_player_name: std::ptr::null_mut(),
            u_media_player_get_audio_track_type: std::ptr::null_mut(),
            u_media_player_get_audio_track_sample_rate: std::ptr::null_mut(),
            u_media_player_get_audio_track_channels: std::ptr::null_mut(),
            u_media_player_close: std::ptr::null_mut(),
            u_media_player_can_play_url: std::ptr::null_mut(),
            u_media_player_can_play_source: std::ptr::null_mut(),
            u_media_player_can_pause: std::ptr::null_mut(),
            u_media_playlist_replace: std::ptr::null_mut(),
            u_media_playlist_remove_at: std::ptr::null_mut(),
            u_media_playlist_remove: std::ptr::null_mut(),
            u_media_playlist_num: std::ptr::null_mut(),
            u_media_playlist_insert: std::ptr::null_mut(),
            u_media_playlist_get_random: std::ptr::null_mut(),
            u_media_playlist_get_previous: std::ptr::null_mut(),
            u_media_playlist_get_next: std::ptr::null_mut(),
            u_media_playlist_get: std::ptr::null_mut(),
            u_media_playlist_add_url: std::ptr::null_mut(),
            u_media_playlist_add_file: std::ptr::null_mut(),
            u_media_playlist_add: std::ptr::null_mut(),
            u_media_sound_component_set_spectral_analysis_settings: std::ptr::null_mut(),
            u_media_sound_component_set_media_player: std::ptr::null_mut(),
            u_media_sound_component_set_envelope_followingsettings: std::ptr::null_mut(),
            u_media_sound_component_set_enable_spectral_analysis: std::ptr::null_mut(),
            u_media_sound_component_set_enable_envelope_following: std::ptr::null_mut(),
            u_media_sound_component_get_spectral_data: std::ptr::null_mut(),
            u_media_sound_component_get_normalized_spectral_data: std::ptr::null_mut(),
            u_media_sound_component_get_media_player: std::ptr::null_mut(),
            u_media_sound_component_get_envelope_value: std::ptr::null_mut(),
            u_media_sound_component_bp_get_attenuation_settings_to_apply: std::ptr::null_mut(),
            u_media_texture_update_resource: std::ptr::null_mut(),
            u_media_texture_set_media_player: std::ptr::null_mut(),
            u_media_texture_get_width: std::ptr::null_mut(),
            u_media_texture_get_texture_num_mips: std::ptr::null_mut(),
            u_media_texture_get_media_player: std::ptr::null_mut(),
            u_media_texture_get_height: std::ptr::null_mut(),
            u_media_texture_get_aspect_ratio: std::ptr::null_mut(),
            u_media_blueprint_function_library_enumerate_webcam_capture_devices: std::ptr::null_mut(),
            u_media_blueprint_function_library_enumerate_video_capture_devices: std::ptr::null_mut(),
            u_media_blueprint_function_library_enumerate_audio_capture_devices: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMediaSource::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Validate"),
                &raw mut __FUNCTION_PTRS.u_media_source_validate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMediaOptionString"),
                &raw mut __FUNCTION_PTRS.u_media_source_set_media_option_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMediaOptionInt64"),
                &raw mut __FUNCTION_PTRS.u_media_source_set_media_option_int64,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMediaOptionFloat"),
                &raw mut __FUNCTION_PTRS.u_media_source_set_media_option_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMediaOptionBool"),
                &raw mut __FUNCTION_PTRS.u_media_source_set_media_option_bool,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUrl"),
                &raw mut __FUNCTION_PTRS.u_media_source_get_url,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UFileMediaSource::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetFilePath"),
                &raw mut __FUNCTION_PTRS.u_file_media_source_set_file_path,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMediaComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMediaTexture"),
                &raw mut __FUNCTION_PTRS.u_media_component_get_media_texture,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMediaPlayer"),
                &raw mut __FUNCTION_PTRS.u_media_component_get_media_player,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMediaPlayer::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SupportsSeeking"),
                &raw mut __FUNCTION_PTRS.u_media_player_supports_seeking,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SupportsScrubbing"),
                &raw mut __FUNCTION_PTRS.u_media_player_supports_scrubbing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SupportsRate"),
                &raw mut __FUNCTION_PTRS.u_media_player_supports_rate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SupportsPlaybackTimeRange"),
                &raw mut __FUNCTION_PTRS.u_media_player_supports_playback_time_range,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetViewRotation"),
                &raw mut __FUNCTION_PTRS.u_media_player_set_view_rotation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetViewField"),
                &raw mut __FUNCTION_PTRS.u_media_player_set_view_field,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetVideoTrackFrameRate"),
                &raw mut __FUNCTION_PTRS.u_media_player_set_video_track_frame_rate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTrackFormat"),
                &raw mut __FUNCTION_PTRS.u_media_player_set_track_format,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTimeDelay"),
                &raw mut __FUNCTION_PTRS.u_media_player_set_time_delay,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetRate"),
                &raw mut __FUNCTION_PTRS.u_media_player_set_rate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPlaybackTimeRange"),
                &raw mut __FUNCTION_PTRS.u_media_player_set_playback_time_range,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNativeVolume"),
                &raw mut __FUNCTION_PTRS.u_media_player_set_native_volume,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMediaOptions"),
                &raw mut __FUNCTION_PTRS.u_media_player_set_media_options,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLooping"),
                &raw mut __FUNCTION_PTRS.u_media_player_set_looping,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDesiredPlayerName"),
                &raw mut __FUNCTION_PTRS.u_media_player_set_desired_player_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBlockOnTime"),
                &raw mut __FUNCTION_PTRS.u_media_player_set_block_on_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectTrack"),
                &raw mut __FUNCTION_PTRS.u_media_player_select_track,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Seek"),
                &raw mut __FUNCTION_PTRS.u_media_player_seek,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Scrub"),
                &raw mut __FUNCTION_PTRS.u_media_player_scrub,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Rewind"),
                &raw mut __FUNCTION_PTRS.u_media_player_rewind,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Reopen"),
                &raw mut __FUNCTION_PTRS.u_media_player_reopen,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Previous"),
                &raw mut __FUNCTION_PTRS.u_media_player_previous,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PlayAndSeek"),
                &raw mut __FUNCTION_PTRS.u_media_player_play_and_seek,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Play"),
                &raw mut __FUNCTION_PTRS.u_media_player_play,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Pause"),
                &raw mut __FUNCTION_PTRS.u_media_player_pause,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OpenUrl"),
                &raw mut __FUNCTION_PTRS.u_media_player_open_url,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OpenSourceWithOptions"),
                &raw mut __FUNCTION_PTRS.u_media_player_open_source_with_options,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OpenSourceLatent"),
                &raw mut __FUNCTION_PTRS.u_media_player_open_source_latent,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OpenSource"),
                &raw mut __FUNCTION_PTRS.u_media_player_open_source,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OpenPlaylistIndex"),
                &raw mut __FUNCTION_PTRS.u_media_player_open_playlist_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OpenPlaylist"),
                &raw mut __FUNCTION_PTRS.u_media_player_open_playlist,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OpenFile"),
                &raw mut __FUNCTION_PTRS.u_media_player_open_file,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Next"),
                &raw mut __FUNCTION_PTRS.u_media_player_next,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsReady"),
                &raw mut __FUNCTION_PTRS.u_media_player_is_ready,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsPreparing"),
                &raw mut __FUNCTION_PTRS.u_media_player_is_preparing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsPlaying"),
                &raw mut __FUNCTION_PTRS.u_media_player_is_playing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsPaused"),
                &raw mut __FUNCTION_PTRS.u_media_player_is_paused,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsLooping"),
                &raw mut __FUNCTION_PTRS.u_media_player_is_looping,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsConnecting"),
                &raw mut __FUNCTION_PTRS.u_media_player_is_connecting,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsClosed"),
                &raw mut __FUNCTION_PTRS.u_media_player_is_closed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsBuffering"),
                &raw mut __FUNCTION_PTRS.u_media_player_is_buffering,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasError"),
                &raw mut __FUNCTION_PTRS.u_media_player_has_error,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetViewRotation"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_view_rotation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVideoTrackType"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_video_track_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVideoTrackFrameRates"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_video_track_frame_rates,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVideoTrackFrameRate"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_video_track_frame_rate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVideoTrackDimensions"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_video_track_dimensions,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVideoTrackAspectRatio"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_video_track_aspect_ratio,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVerticalFieldOfView"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_vertical_field_of_view,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUrl"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_url,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTrackLanguage"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_track_language,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTrackFormat"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_track_format,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTrackDisplayName"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_track_display_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTimeStamp"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_time_stamp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTimeDelay"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_time_delay,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTime"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSupportedRates"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_supported_rates,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectedTrack"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_selected_track,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRate"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_rate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPlaylistIndex"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_playlist_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPlaylist"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_playlist,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPlayerName"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_player_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPlaybackTimeRange"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_playback_time_range,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumTracks"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_num_tracks,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumTrackFormats"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_num_track_formats,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMediaName"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_media_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMediaMetadataItems"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_media_metadata_items,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetHorizontalFieldOfView"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_horizontal_field_of_view,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDuration"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_duration,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDisplayTimeStamp"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_display_time_stamp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDisplayTime"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_display_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDesiredPlayerName"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_desired_player_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAudioTrackType"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_audio_track_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAudioTrackSampleRate"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_audio_track_sample_rate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAudioTrackChannels"),
                &raw mut __FUNCTION_PTRS.u_media_player_get_audio_track_channels,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Close"),
                &raw mut __FUNCTION_PTRS.u_media_player_close,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanPlayUrl"),
                &raw mut __FUNCTION_PTRS.u_media_player_can_play_url,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanPlaySource"),
                &raw mut __FUNCTION_PTRS.u_media_player_can_play_source,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanPause"),
                &raw mut __FUNCTION_PTRS.u_media_player_can_pause,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMediaPlaylist::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Replace"),
                &raw mut __FUNCTION_PTRS.u_media_playlist_replace,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveAt"),
                &raw mut __FUNCTION_PTRS.u_media_playlist_remove_at,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Remove"),
                &raw mut __FUNCTION_PTRS.u_media_playlist_remove,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Num"),
                &raw mut __FUNCTION_PTRS.u_media_playlist_num,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Insert"),
                &raw mut __FUNCTION_PTRS.u_media_playlist_insert,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRandom"),
                &raw mut __FUNCTION_PTRS.u_media_playlist_get_random,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPrevious"),
                &raw mut __FUNCTION_PTRS.u_media_playlist_get_previous,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNext"),
                &raw mut __FUNCTION_PTRS.u_media_playlist_get_next,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Get"),
                &raw mut __FUNCTION_PTRS.u_media_playlist_get,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddUrl"),
                &raw mut __FUNCTION_PTRS.u_media_playlist_add_url,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddFile"),
                &raw mut __FUNCTION_PTRS.u_media_playlist_add_file,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Add"),
                &raw mut __FUNCTION_PTRS.u_media_playlist_add,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMediaSoundComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSpectralAnalysisSettings"),
                &raw mut __FUNCTION_PTRS
                    .u_media_sound_component_set_spectral_analysis_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMediaPlayer"),
                &raw mut __FUNCTION_PTRS.u_media_sound_component_set_media_player,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetEnvelopeFollowingsettings"),
                &raw mut __FUNCTION_PTRS
                    .u_media_sound_component_set_envelope_followingsettings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetEnableSpectralAnalysis"),
                &raw mut __FUNCTION_PTRS
                    .u_media_sound_component_set_enable_spectral_analysis,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetEnableEnvelopeFollowing"),
                &raw mut __FUNCTION_PTRS
                    .u_media_sound_component_set_enable_envelope_following,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSpectralData"),
                &raw mut __FUNCTION_PTRS.u_media_sound_component_get_spectral_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNormalizedSpectralData"),
                &raw mut __FUNCTION_PTRS
                    .u_media_sound_component_get_normalized_spectral_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMediaPlayer"),
                &raw mut __FUNCTION_PTRS.u_media_sound_component_get_media_player,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEnvelopeValue"),
                &raw mut __FUNCTION_PTRS.u_media_sound_component_get_envelope_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_GetAttenuationSettingsToApply"),
                &raw mut __FUNCTION_PTRS
                    .u_media_sound_component_bp_get_attenuation_settings_to_apply,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMediaTexture::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UpdateResource"),
                &raw mut __FUNCTION_PTRS.u_media_texture_update_resource,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMediaPlayer"),
                &raw mut __FUNCTION_PTRS.u_media_texture_set_media_player,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWidth"),
                &raw mut __FUNCTION_PTRS.u_media_texture_get_width,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTextureNumMips"),
                &raw mut __FUNCTION_PTRS.u_media_texture_get_texture_num_mips,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMediaPlayer"),
                &raw mut __FUNCTION_PTRS.u_media_texture_get_media_player,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetHeight"),
                &raw mut __FUNCTION_PTRS.u_media_texture_get_height,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAspectRatio"),
                &raw mut __FUNCTION_PTRS.u_media_texture_get_aspect_ratio,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMediaBlueprintFunctionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EnumerateWebcamCaptureDevices"),
                &raw mut __FUNCTION_PTRS
                    .u_media_blueprint_function_library_enumerate_webcam_capture_devices,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EnumerateVideoCaptureDevices"),
                &raw mut __FUNCTION_PTRS
                    .u_media_blueprint_function_library_enumerate_video_capture_devices,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EnumerateAudioCaptureDevices"),
                &raw mut __FUNCTION_PTRS
                    .u_media_blueprint_function_library_enumerate_audio_capture_devices,
            );
        }
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
    pub(crate) __padding_end: [u8; 8],
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaSourceRendererInterface")
            .copied()
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaSource")
            .copied()
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_source_validate,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_source_validate,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_source_set_media_option_string,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_source_set_media_option_string,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_source_set_media_option_int64,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_source_set_media_option_int64,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_source_set_media_option_float,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_source_set_media_option_float,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_source_set_media_option_bool,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_source_set_media_option_bool,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_source_get_url,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_source_get_url,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct UBaseMediaSource {
    #[doc(hidden)]
    pub(crate) __padding_152: [u8; 152],
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseMediaSource")
            .copied()
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
    pub(crate) __padding_328: [u8; 328],
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFileMediaSource")
            .copied()
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_file_media_source_set_file_path,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_file_media_source_set_file_path,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UMediaComponent {
    #[doc(hidden)]
    pub(crate) __padding_240: [u8; 240],
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaComponent")
            .copied()
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_component_get_media_texture,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_component_get_media_texture,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_component_get_media_player,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_component_get_media_player,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UMediaPlayer>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMediaTimeStampInfo {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaTimeStampInfo")
            .copied()
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
    pub(crate) __padding_320: [u8; 320],
    pub cache_ahead: crate::bindings::core_u_object::FTimespan,
    pub cache_behind: crate::bindings::core_u_object::FTimespan,
    pub cache_behind_game: crate::bindings::core_u_object::FTimespan,
    pub native_audio_out: bool,
    pub play_on_open: bool,
    #[doc(hidden)]
    pub(crate) __padding_348: [u8; 2],
    pub flags_348: u8,
    pub playlist: UPtr<UMediaPlaylist>,
    pub playlist_index: i32,
    pub time_delay: crate::bindings::core_u_object::FTimespan,
    #[doc(hidden)]
    pub(crate) __padding_496: [u8; 120],
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaPlayer")
            .copied()
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_supports_seeking,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_supports_seeking,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_supports_scrubbing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_supports_scrubbing,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_supports_rate,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_supports_rate,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_supports_playback_time_range,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_supports_playback_time_range,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_view_rotation,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_view_rotation,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_view_field,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_view_field,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_video_track_frame_rate,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_video_track_frame_rate,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_track_format,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_track_format,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_time_delay,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_time_delay,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_set_rate,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_set_rate,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_playback_time_range,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_playback_time_range,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_native_volume,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_native_volume,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_media_options,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_media_options,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_looping,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_looping,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_desired_player_name,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_desired_player_name,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_block_on_time,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_set_block_on_time,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_select_track,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_select_track,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_seek,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_seek,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_scrub,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_scrub,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_rewind,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_rewind,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_reopen,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_reopen,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_previous,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_previous,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_play_and_seek,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_play_and_seek,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_play,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_play,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_pause,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_pause,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_open_url,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_open_url,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_open_source_with_options,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_open_source_with_options,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_open_source_latent,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_open_source_latent,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_open_source,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_open_source,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_open_playlist_index,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_open_playlist_index,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_open_playlist,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_open_playlist,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_open_file,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_open_file,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_next,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_next,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_is_ready,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_is_ready,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_is_preparing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_is_preparing,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_is_playing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_is_playing,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_is_paused,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_is_paused,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_is_looping,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_is_looping,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_is_connecting,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_is_connecting,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_is_closed,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_is_closed,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_is_buffering,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_is_buffering,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_has_error,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_has_error,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_view_rotation,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_view_rotation,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_video_track_type,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_video_track_type,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_video_track_frame_rates,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_video_track_frame_rates,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_video_track_frame_rate,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_video_track_frame_rate,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_video_track_dimensions,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_video_track_dimensions,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_video_track_aspect_ratio,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_video_track_aspect_ratio,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_vertical_field_of_view,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_vertical_field_of_view,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_get_url,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_get_url,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_track_language,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_track_language,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_track_format,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_track_format,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_track_display_name,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_track_display_name,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_time_stamp,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_time_stamp,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_time_delay,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_time_delay,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_get_time,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_get_time,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_supported_rates,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_supported_rates,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_selected_track,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_selected_track,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_get_rate,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_get_rate,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_playlist_index,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_playlist_index,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_playlist,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_playlist,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_player_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_player_name,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_playback_time_range,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_playback_time_range,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_num_tracks,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_num_tracks,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_num_track_formats,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_num_track_formats,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_media_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_media_name,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_media_metadata_items,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_media_metadata_items,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_horizontal_field_of_view,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_horizontal_field_of_view,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_duration,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_duration,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_display_time_stamp,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_display_time_stamp,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_display_time,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_display_time,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_desired_player_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_desired_player_name,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_audio_track_type,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_audio_track_type,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_audio_track_sample_rate,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_audio_track_sample_rate,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_audio_track_channels,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_get_audio_track_channels,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_close,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_close,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_can_play_url,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_can_play_url,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_can_play_source,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_player_can_play_source,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_can_pause,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_player_can_pause,
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaPlayerProxyInterface")
            .copied()
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaPlaylist")
            .copied()
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_playlist_replace,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_playlist_replace,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_playlist_remove_at,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_playlist_remove_at,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_playlist_remove,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_playlist_remove,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_playlist_num,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_playlist_num,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_playlist_insert,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_playlist_insert,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_playlist_get_random,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_playlist_get_random,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_playlist_get_previous,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_playlist_get_previous,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_playlist_get_next,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_playlist_get_next,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_playlist_get,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_playlist_get,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_playlist_add_url,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_playlist_add_url,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_playlist_add_file,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_playlist_add_file,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_playlist_add,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_playlist_add,
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaSoundComponent")
            .copied()
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_sound_component_set_spectral_analysis_settings,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_sound_component_set_spectral_analysis_settings,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_sound_component_set_media_player,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_sound_component_set_media_player,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_sound_component_set_envelope_followingsettings,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_sound_component_set_envelope_followingsettings,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_sound_component_set_enable_spectral_analysis,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_sound_component_set_enable_spectral_analysis,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_sound_component_set_enable_envelope_following,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_sound_component_set_enable_envelope_following,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_sound_component_get_spectral_data,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_sound_component_get_spectral_data,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_sound_component_get_normalized_spectral_data,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_sound_component_get_normalized_spectral_data,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_sound_component_get_media_player,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_sound_component_get_media_player,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_sound_component_get_envelope_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_sound_component_get_envelope_value,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_sound_component_bp_get_attenuation_settings_to_apply,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_sound_component_bp_get_attenuation_settings_to_apply,
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
    pub(crate) __padding_1240: [u8; 1240],
    pub address_x: crate::bindings::engine::TextureAddress,
    pub address_y: crate::bindings::engine::TextureAddress,
    pub auto_clear: bool,
    pub clear_color: crate::bindings::core_u_object::FLinearColor,
    pub enable_gen_mips: bool,
    #[doc(hidden)]
    pub(crate) __padding_1262: [u8; 1],
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaTexture")
            .copied()
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_texture_update_resource,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_texture_update_resource,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_texture_set_media_player,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_texture_set_media_player,
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
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_texture_get_width,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS.u_media_texture_get_width,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_texture_get_texture_num_mips,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_texture_get_texture_num_mips,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_texture_get_media_player,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_texture_get_media_player,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_texture_get_height,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_texture_get_height,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_texture_get_aspect_ratio,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_texture_get_aspect_ratio,
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlatformMediaSource")
            .copied()
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
    pub(crate) __padding_328: [u8; 328],
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStreamMediaSource")
            .copied()
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
    pub(crate) __padding_328: [u8; 328],
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTimeSynchronizableMediaSource")
            .copied()
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaBlueprintFunctionLibrary")
            .copied()
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_blueprint_function_library_enumerate_webcam_capture_devices,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_blueprint_function_library_enumerate_webcam_capture_devices,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_blueprint_function_library_enumerate_video_capture_devices,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_blueprint_function_library_enumerate_video_capture_devices,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_blueprint_function_library_enumerate_audio_capture_devices,
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
                crate::bindings::media_assets::__FUNCTION_PTRS
                    .u_media_blueprint_function_library_enumerate_audio_capture_devices,
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
