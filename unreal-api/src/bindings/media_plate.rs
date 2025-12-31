#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FMediaTextureResourceSettings {
    pub b_enable_gen_mips: bool,
    pub current_num_mips_deprecated: u8,
}
#[repr(C, align(8))]
pub struct FMediaPlateResource {
    pub ty: EMediaPlateResourceType,
    pub external_media_path: FString,
    pub media_asset: TSoftObjectPtr<crate::bindings::media_assets::UMediaSource>,
    pub source_playlist: TSoftObjectPtr<crate::bindings::media_assets::UMediaPlaylist>,
    pub external_media_deprecated: UPtr<crate::bindings::media_assets::UMediaSource>,
}
pub struct AMediaPlate {
    pub media_plate_component: UPtr<UMediaPlateComponent>,
    pub static_mesh_component: UPtr<crate::bindings::engine::UStaticMeshComponent>,
    pub b_enable_holdout_composite: bool,
}
pub struct UMediaPlateAssetUserData {}
pub struct UMediaPlateComponent {
    pub b_play_on_open: bool,
    pub b_auto_play: bool,
    pub b_enable_audio: bool,
    pub start_time: f32,
    pub sound_component: UPtr<crate::bindings::media_assets::UMediaSoundComponent>,
    pub static_mesh_component: UPtr<crate::bindings::engine::UStaticMeshComponent>,
    pub letterboxes: TArray<UPtr<crate::bindings::engine::UStaticMeshComponent>>,
    pub media_playlist_deprecated: UPtr<crate::bindings::media_assets::UMediaPlaylist>,
    pub media_plate_resource: FMediaPlateResource,
    pub playlist_index: i32,
    pub cache_settings: crate::bindings::media_assets::FMediaSourceCacheSettings,
    pub b_is_media_plate_playing: bool,
    pub b_play_only_when_visible: bool,
    pub b_loop: bool,
    pub visible_mips_tiles_calculations: crate::bindings::media_assets::EMediaTextureVisibleMipsTiles,
    pub mip_map_bias: f32,
    pub b_is_aspect_ratio_auto: bool,
    pub b_enable_mip_map_upscaling: bool,
    pub mip_level_to_upscale: i32,
    pub b_adaptive_pole_mip_upscaling: bool,
    pub letterbox_aspect_ratio: f32,
    pub mesh_range: crate::bindings::core_u_object::FVector2D,
    pub media_texture_deprecated: UPtr<crate::bindings::media_assets::UMediaTexture>,
    pub media_textures: TArray<UPtr<crate::bindings::media_assets::UMediaTexture>>,
    pub media_texture_settings: FMediaTextureResourceSettings,
    pub media_player: UPtr<crate::bindings::media_assets::UMediaPlayer>,
    pub external_media_source: UPtr<crate::bindings::media_assets::UMediaSource>,
    pub active_playlist: UPtr<crate::bindings::media_assets::UMediaPlaylist>,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMediaPlateResourceType(pub u8);
impl EMediaPlateResourceType {
    pub const PLAYLIST: EMediaPlateResourceType = EMediaPlateResourceType(0);
    pub const EXTERNAL: EMediaPlateResourceType = EMediaPlateResourceType(1);
    pub const ASSET: EMediaPlateResourceType = EMediaPlateResourceType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMediaPlateEventState(pub u8);
impl EMediaPlateEventState {
    pub const PLAY: EMediaPlateEventState = EMediaPlateEventState(0);
    pub const OPEN: EMediaPlateEventState = EMediaPlateEventState(1);
    pub const CLOSE: EMediaPlateEventState = EMediaPlateEventState(2);
    pub const PAUSE: EMediaPlateEventState = EMediaPlateEventState(3);
    pub const REVERSE: EMediaPlateEventState = EMediaPlateEventState(4);
    pub const FORWARD: EMediaPlateEventState = EMediaPlateEventState(5);
    pub const REWIND: EMediaPlateEventState = EMediaPlateEventState(6);
    pub const NEXT: EMediaPlateEventState = EMediaPlateEventState(7);
    pub const PREVIOUS: EMediaPlateEventState = EMediaPlateEventState(8);
    pub const MAX: EMediaPlateEventState = EMediaPlateEventState(9);
}
