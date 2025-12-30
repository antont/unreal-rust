#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FMovieSceneMediaPlayerPropertySectionTemplate {
    pub media_source: UPtr<UMediaSource>,
    pub section_start_frame: FFrameNumber,
    pub b_loop: bool,
}
#[repr(C, align(8))]
pub struct FMovieSceneMediaSectionParams {
    pub media_sound_component: UPtr<UMediaSoundComponent>,
    pub media_source: UPtr<UMediaSource>,
    pub media_source_proxy: FMovieSceneObjectBindingID,
    pub media_source_proxy_index: i32,
    pub media_texture: UPtr<UMediaTexture>,
    pub media_player: UPtr<UMediaPlayer>,
    pub section_start_frame: FFrameNumber,
    pub section_end_frame: FFrameNumber,
    pub b_looping: bool,
    pub start_frame_offset: FFrameNumber,
    pub proxy_texture_blend: FMovieSceneFloatChannel,
    pub cache_settings: FMediaSourceCacheSettings,
}
#[repr(C, align(8))]
pub struct FMovieSceneMediaSectionTemplate {
    pub params: FMovieSceneMediaSectionParams,
    pub media_section: UPtr<UMovieSceneMediaSection>,
}
pub struct UMovieSceneMediaPlayerPropertySection {
    pub media_source: UPtr<UMediaSource>,
    pub b_loop: bool,
}
pub struct UMovieSceneMediaPlayerPropertyTrack {}
pub struct UMovieSceneMediaSection {
    pub media_source: UPtr<UMediaSource>,
    pub media_source_proxy_index: i32,
    pub b_looping: bool,
    pub start_frame_offset: FFrameNumber,
    pub media_texture: UPtr<UMediaTexture>,
    pub media_sound_component: UPtr<UMediaSoundComponent>,
    pub b_use_external_media_player: bool,
    pub external_media_player: UPtr<UMediaPlayer>,
    pub cache_settings: FMediaSourceCacheSettings,
    pub texture_index: i32,
    pub b_manual_frame_rate_alignment: bool,
    pub frame_rate_alignment: FFrameRate,
    pub b_has_media_player_proxy: bool,
    pub channel_can_player_be_open: FMovieSceneBoolChannel,
    pub thumbnail_reference_offset: f32,
    pub media_source_proxy_binding_id: FMovieSceneObjectBindingID,
}
pub struct UMovieSceneMediaTrack {
    pub b_synchronous_scrubbing: bool,
    pub media_sections: TArray<UPtr<UMovieSceneSection>>,
}
