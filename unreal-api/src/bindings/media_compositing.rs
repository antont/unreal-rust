#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FMovieSceneMediaPlayerPropertySectionTemplate {
    pub media_source: UPtr<crate::bindings::media_assets::UMediaSource>,
    pub section_start_frame: crate::bindings::core_u_object::FFrameNumber,
    pub b_loop: bool,
}
#[repr(C, align(8))]
pub struct FMovieSceneMediaSectionParams {
    pub media_sound_component: UPtr<crate::bindings::media_assets::UMediaSoundComponent>,
    pub media_source: UPtr<crate::bindings::media_assets::UMediaSource>,
    pub media_source_proxy: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
    pub media_source_proxy_index: i32,
    pub media_texture: UPtr<crate::bindings::media_assets::UMediaTexture>,
    pub media_player: UPtr<crate::bindings::media_assets::UMediaPlayer>,
    pub section_start_frame: crate::bindings::core_u_object::FFrameNumber,
    pub section_end_frame: crate::bindings::core_u_object::FFrameNumber,
    pub b_looping: bool,
    pub start_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub proxy_texture_blend: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub cache_settings: crate::bindings::media_assets::FMediaSourceCacheSettings,
}
#[repr(C, align(8))]
pub struct FMovieSceneMediaSectionTemplate {
    pub params: FMovieSceneMediaSectionParams,
    pub media_section: UPtr<UMovieSceneMediaSection>,
}
pub struct UMovieSceneMediaPlayerPropertySection {
    pub media_source: UPtr<crate::bindings::media_assets::UMediaSource>,
    pub b_loop: bool,
}
pub struct UMovieSceneMediaPlayerPropertyTrack {}
pub struct UMovieSceneMediaSection {
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
    pub b_has_media_player_proxy: bool,
    pub channel_can_player_be_open: crate::bindings::movie_scene::FMovieSceneBoolChannel,
    pub thumbnail_reference_offset: f32,
    pub media_source_proxy_binding_id: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
}
pub struct UMovieSceneMediaTrack {
    pub b_synchronous_scrubbing: bool,
    pub media_sections: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
}
