#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UMediaPlayerEditorMediaContext {
    pub selected_asset: UPtr<crate::bindings::core_u_object::UObject>,
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
    pub media_player: UPtr<crate::bindings::media_assets::UMediaPlayer>,
    pub media_source: UPtr<crate::bindings::media_assets::UMediaSource>,
    pub media_texture: UPtr<crate::bindings::media_assets::UMediaTexture>,
}
pub struct UMediaSourceThumbnailRenderer {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMediaPlayerEditorScale(pub u8);
impl EMediaPlayerEditorScale {
    pub const FILL: EMediaPlayerEditorScale = EMediaPlayerEditorScale(0);
    pub const FIT: EMediaPlayerEditorScale = EMediaPlayerEditorScale(1);
    pub const ORIGINAL: EMediaPlayerEditorScale = EMediaPlayerEditorScale(2);
}
