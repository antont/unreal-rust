#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UMediaPlayerEditorMediaContext {
    pub selected_asset: UPtr<UObject>,
    pub style_set_name: FName,
}
pub struct UMediaPlayerEditorSettings {
    pub desired_player_name: FName,
    pub show_text_overlays: bool,
    pub viewport_scale: EMediaPlayerEditorScale,
}
pub struct UFileMediaSourceFactoryNew {}
pub struct UMediaPlayerFactoryNew {}
pub struct UMediaPlaylistFactoryNew {}
pub struct UMediaTextureFactoryNew {}
pub struct UPlatformMediaSourceFactoryNew {}
pub struct UStreamMediaSourceFactoryNew {}
pub struct UMediaSourceRenderer {
    pub media_player: UPtr<UMediaPlayer>,
    pub media_source: UPtr<UMediaSource>,
    pub media_texture: UPtr<UMediaTexture>,
}
pub struct UMediaSourceThumbnailRenderer {}
