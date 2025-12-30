#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct USparseVolumeTextureViewerComponent {
    pub sparse_volume_texture_preview: UPtr<USparseVolumeTexture>,
    pub frame: f32,
    pub frame_rate: f32,
    pub flags_1520: u8,
    pub voxel_size: f32,
    pub preview_attribute: ESparseVolumeTexturePreviewAttribute,
    pub mip_level: i32,
    pub extinction: f32,
}
pub struct ASparseVolumeTextureViewer {
    pub sparse_volume_texture_viewer_component: UPtr<
        USparseVolumeTextureViewerComponent,
    >,
}
