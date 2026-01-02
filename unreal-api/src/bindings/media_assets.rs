#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
pub struct UMediaSourceRendererInterface {}
pub struct IMediaSourceRendererInterface {}
#[repr(C, align(8))]
pub struct UMediaSource {
    __padding_end: [u8; 152],
}
impl UMediaSource {}
#[repr(C, align(8))]
pub struct UBaseMediaSource {
    #[doc(hidden)]
    __padding_152: [u8; 152],
    pub platform_player_names: TMap<FString, FName>,
    __padding_end: [u8; 96],
}
impl UBaseMediaSource {}
#[repr(C, align(8))]
pub struct UFileMediaSource {
    #[doc(hidden)]
    __padding_328: [u8; 328],
    pub file_path: FString,
    pub precache_file: bool,
    __padding_end: [u8; 23],
}
impl UFileMediaSource {}
#[repr(C, align(8))]
pub struct UMediaComponent {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub media_texture: UPtr<UMediaTexture>,
    pub media_player: UPtr<UMediaPlayer>,
}
impl UMediaComponent {}
#[repr(C, align(8))]
pub struct UMediaTimeStampInfo {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub time: crate::bindings::core_u_object::FTimespan,
    pub sequence_index: i64,
}
impl UMediaTimeStampInfo {}
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
impl UMediaPlayer {}
pub struct UMediaPlayerProxyInterface {}
pub struct IMediaPlayerProxyInterface {}
#[repr(C, align(8))]
pub struct UMediaPlaylist {
    __padding_end: [u8; 64],
}
impl UMediaPlaylist {}
#[repr(C, align(16))]
pub struct UMediaSoundComponent {
    __padding_end: [u8; 2608],
}
impl UMediaSoundComponent {}
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
impl UMediaTexture {}
#[repr(C, align(8))]
pub struct UPlatformMediaSource {
    __padding_end: [u8; 320],
}
impl UPlatformMediaSource {}
#[repr(C, align(8))]
pub struct UStreamMediaSource {
    #[doc(hidden)]
    __padding_328: [u8; 328],
    pub stream_url: FString,
}
impl UStreamMediaSource {}
#[repr(C, align(8))]
pub struct UTimeSynchronizableMediaSource {
    #[doc(hidden)]
    __padding_328: [u8; 328],
    pub b_use_time_synchronization: bool,
    __padding_end: [u8; 23],
}
impl UTimeSynchronizableMediaSource {}
#[repr(C, align(8))]
pub struct UMediaBlueprintFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UMediaBlueprintFunctionLibrary {}
#[repr(transparent)]
pub struct FMediaPlayer_OnEndReached {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMediaPlayer_OnMediaClosed {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMediaPlayer_OnMediaOpened {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMediaPlayer_OnMediaOpenFailed {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMediaPlayer_OnPlaybackResumed {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMediaPlayer_OnPlaybackSuspended {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMediaPlayer_OnSeekCompleted {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMediaPlayer_OnTracksChanged {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMediaPlayer_OnMetadataChanged {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMediaPlayer_OnBufferingStart {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMediaPlayer_OnBufferingCompleted {
    _opague: u8,
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
