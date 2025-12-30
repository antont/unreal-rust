#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(1))]
pub struct FMediaTextureResourceSettings {
    pub b_enable_gen_mips: bool,
    pub current_num_mips_deprecated: u8,
}
#[repr(C, align(8))]
pub struct FMediaPlateResource {
    pub ty: EMediaPlateResourceType,
    pub external_media_path: FString,
    pub media_asset: TSoftObjectPtr<UMediaSource>,
    pub source_playlist: TSoftObjectPtr<UMediaPlaylist>,
    pub external_media_deprecated: UPtr<UMediaSource>,
}
pub struct AMediaPlate {
    pub media_plate_component: UPtr<UMediaPlateComponent>,
    pub static_mesh_component: UPtr<UStaticMeshComponent>,
    pub b_enable_holdout_composite: bool,
}
pub struct UMediaPlateAssetUserData {}
pub struct UMediaPlateComponent {
    pub b_play_on_open: bool,
    pub b_auto_play: bool,
    pub b_enable_audio: bool,
    pub start_time: f32,
    pub sound_component: UPtr<UMediaSoundComponent>,
    pub static_mesh_component: UPtr<UStaticMeshComponent>,
    pub letterboxes: TArray<UPtr<UStaticMeshComponent>>,
    pub media_playlist_deprecated: UPtr<UMediaPlaylist>,
    pub media_plate_resource: FMediaPlateResource,
    pub playlist_index: i32,
    pub cache_settings: FMediaSourceCacheSettings,
    pub b_is_media_plate_playing: bool,
    pub b_play_only_when_visible: bool,
    pub b_loop: bool,
    pub visible_mips_tiles_calculations: EMediaTextureVisibleMipsTiles,
    pub mip_map_bias: f32,
    pub b_is_aspect_ratio_auto: bool,
    pub b_enable_mip_map_upscaling: bool,
    pub mip_level_to_upscale: i32,
    pub b_adaptive_pole_mip_upscaling: bool,
    pub letterbox_aspect_ratio: f32,
    pub mesh_range: FVector2D,
    pub media_texture_deprecated: UPtr<UMediaTexture>,
    pub media_textures: TArray<UPtr<UMediaTexture>>,
    pub media_texture_settings: FMediaTextureResourceSettings,
    pub media_player: UPtr<UMediaPlayer>,
    pub external_media_source: UPtr<UMediaSource>,
    pub active_playlist: UPtr<UMediaPlaylist>,
}
