#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FMediaMetadataItemBPT {
    pub language_code: FString,
    pub mime_type: FString,
    pub string_data: FString,
    pub binary_data: TArray<u8>,
}
#[repr(C, align(8))]
pub struct FMediaMetadataItemsBPT {
    pub items: TArray<FMediaMetadataItemBPT>,
}
#[repr(C, align(4))]
pub struct FMediaSoundComponentSpectralData {
    pub frequency_hz: f32,
    pub magnitude: f32,
}
#[repr(C, align(4))]
pub struct FMediaSourceCacheSettings {
    pub b_override: bool,
    pub time_to_look_ahead: f32,
}
#[repr(C, align(8))]
pub struct FMediaCaptureDevice {
    pub display_name: FText,
    pub url: FString,
}
pub struct UMediaSourceRendererInterface {}
pub struct IMediaSourceRendererInterface {}
pub struct UMediaSource {
    pub thumbnail_image: UPtr<UTexture>,
    pub media_source_renderer: UPtr<UObject>,
}
pub struct UBaseMediaSource {
    pub platform_player_names: TMap<FString, FName>,
    pub player_name: FName,
}
pub struct UFileMediaSource {
    pub file_path: FString,
    pub precache_file: bool,
}
pub struct UMediaComponent {
    pub media_texture: UPtr<UMediaTexture>,
    pub media_player: UPtr<UMediaPlayer>,
}
pub struct UMediaTimeStampInfo {
    pub time: FTimespan,
    pub sequence_index: i64,
}
pub struct UMediaPlayer {
    pub on_end_reached: FMediaPlayer_OnEndReached,
    pub on_media_closed: FMediaPlayer_OnMediaClosed,
    pub on_media_opened: FMediaPlayer_OnMediaOpened,
    pub on_media_open_failed: FMediaPlayer_OnMediaOpenFailed,
    pub on_playback_resumed: FMediaPlayer_OnPlaybackResumed,
    pub on_playback_suspended: FMediaPlayer_OnPlaybackSuspended,
    pub on_seek_completed: FMediaPlayer_OnSeekCompleted,
    pub on_tracks_changed: FMediaPlayer_OnTracksChanged,
    pub on_metadata_changed: FMediaPlayer_OnMetadataChanged,
    pub on_buffering_start: FMediaPlayer_OnBufferingStart,
    pub on_buffering_completed: FMediaPlayer_OnBufferingCompleted,
    pub cache_ahead: FTimespan,
    pub cache_behind: FTimespan,
    pub cache_behind_game: FTimespan,
    pub native_audio_out: bool,
    pub play_on_open: bool,
    pub flags_348: u8,
    pub playlist: UPtr<UMediaPlaylist>,
    pub playlist_index: i32,
    pub time_delay: FTimespan,
    pub horizontal_field_of_view: f32,
    pub vertical_field_of_view: f32,
    pub view_rotation: FRotator,
    pub player_guid: FGuid,
    pub affected_by_pie_handling: bool,
}
pub struct UMediaPlayerProxyInterface {}
pub struct IMediaPlayerProxyInterface {}
pub struct UMediaPlaylist {
    pub items: TArray<UPtr<UMediaSource>>,
}
pub struct UMediaSoundComponent {
    pub channels: EMediaSoundChannels,
    pub dynamic_rate_adjustment: bool,
    pub rate_adjustment_factor: f32,
    pub rate_adjustment_range: FFloatRange,
    pub media_player: UPtr<UMediaPlayer>,
}
pub struct UMediaTexture {
    pub address_x: TextureAddress,
    pub address_y: TextureAddress,
    pub auto_clear: bool,
    pub clear_color: FLinearColor,
    pub enable_gen_mips: bool,
    pub num_mips_deprecated: u8,
    pub new_style_output: bool,
    pub current_aspect_ratio: f32,
    pub current_orientation: MediaTextureOrientation,
    pub media_player: UPtr<UMediaPlayer>,
}
pub struct UPlatformMediaSource {
    pub platform_media_sources: TMap<FString, UPtr<UMediaSource>>,
    pub media_source: UPtr<UMediaSource>,
}
pub struct UStreamMediaSource {
    pub stream_url: FString,
}
pub struct UTimeSynchronizableMediaSource {
    pub b_use_time_synchronization: bool,
    pub frame_delay: i32,
    pub time_delay: f64,
    pub b_auto_detect_input: bool,
}
pub struct UMediaBlueprintFunctionLibrary {}
