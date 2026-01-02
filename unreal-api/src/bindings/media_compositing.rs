#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UMovieSceneMediaPlayerPropertySection {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub media_source: UPtr<crate::bindings::media_assets::UMediaSource>,
    pub b_loop: bool,
    __padding_end: [u8; 7],
}
impl UMovieSceneMediaPlayerPropertySection {}
#[repr(C, align(8))]
pub struct UMovieSceneMediaPlayerPropertyTrack {
    __padding_end: [u8; 488],
}
impl UMovieSceneMediaPlayerPropertyTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneMediaSection {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub media_source: UPtr<crate::bindings::media_assets::UMediaSource>,
    pub media_source_proxy_index: i32,
    pub b_looping: bool,
    pub start_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub media_texture: UPtr<crate::bindings::media_assets::UMediaTexture>,
    pub media_sound_component: UPtr<crate::bindings::media_assets::UMediaSoundComponent>,
    pub b_use_external_media_player: bool,
    pub external_media_player: UPtr<crate::bindings::media_assets::UMediaPlayer>,
    pub cache_settings: crate::bindings::media_assets::FMediaSourceCacheSettings,
    pub texture_index: i32,
    pub b_manual_frame_rate_alignment: bool,
    pub frame_rate_alignment: crate::bindings::core_u_object::FFrameRate,
    __padding_end: [u8; 328],
}
impl UMovieSceneMediaSection {}
#[repr(C, align(8))]
pub struct UMovieSceneMediaTrack {
    __padding_end: [u8; 456],
}
impl UMovieSceneMediaTrack {}
