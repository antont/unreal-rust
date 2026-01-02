#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UMediaPlayerEditorMediaContext {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub selected_asset: UPtr<crate::bindings::core_u_object::UObject>,
    pub style_set_name: FName,
    __padding_end: [u8; 20],
}
impl UMediaPlayerEditorMediaContext {}
#[repr(C, align(8))]
pub struct UMediaPlayerEditorSettings {
    __padding_end: [u8; 64],
}
impl UMediaPlayerEditorSettings {}
#[repr(C, align(8))]
pub struct UFileMediaSourceFactoryNew {
    __padding_end: [u8; 136],
}
impl UFileMediaSourceFactoryNew {}
#[repr(C, align(8))]
pub struct UMediaPlayerFactoryNew {
    __padding_end: [u8; 144],
}
impl UMediaPlayerFactoryNew {}
#[repr(C, align(8))]
pub struct UMediaPlaylistFactoryNew {
    __padding_end: [u8; 136],
}
impl UMediaPlaylistFactoryNew {}
#[repr(C, align(8))]
pub struct UMediaTextureFactoryNew {
    __padding_end: [u8; 136],
}
impl UMediaTextureFactoryNew {}
#[repr(C, align(8))]
pub struct UPlatformMediaSourceFactoryNew {
    __padding_end: [u8; 136],
}
impl UPlatformMediaSourceFactoryNew {}
#[repr(C, align(8))]
pub struct UStreamMediaSourceFactoryNew {
    __padding_end: [u8; 136],
}
impl UStreamMediaSourceFactoryNew {}
#[repr(C, align(8))]
pub struct UMediaSourceRenderer {
    __padding_end: [u8; 96],
}
impl UMediaSourceRenderer {}
#[repr(C, align(8))]
pub struct UMediaSourceThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UMediaSourceThumbnailRenderer {}
#[repr(transparent)]
pub struct EMediaPlayerEditorScale(pub u8);
impl EMediaPlayerEditorScale {
    pub const FILL: EMediaPlayerEditorScale = EMediaPlayerEditorScale(0);
    pub const FIT: EMediaPlayerEditorScale = EMediaPlayerEditorScale(1);
    pub const ORIGINAL: EMediaPlayerEditorScale = EMediaPlayerEditorScale(2);
}
