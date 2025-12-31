#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct USparseVolumeTextureViewerComponent {
    pub sparse_volume_texture_preview: UPtr<
        crate::bindings::engine::USparseVolumeTexture,
    >,
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESparseVolumeTexturePreviewAttribute(pub u8);
impl ESparseVolumeTexturePreviewAttribute {
    pub const ESVTPA_ATTRIBUTES_A_R: ESparseVolumeTexturePreviewAttribute = ESparseVolumeTexturePreviewAttribute(
        0,
    );
    pub const ESVTPA_ATTRIBUTES_A_G: ESparseVolumeTexturePreviewAttribute = ESparseVolumeTexturePreviewAttribute(
        1,
    );
    pub const ESVTPA_ATTRIBUTES_A_B: ESparseVolumeTexturePreviewAttribute = ESparseVolumeTexturePreviewAttribute(
        2,
    );
    pub const ESVTPA_ATTRIBUTES_A_A: ESparseVolumeTexturePreviewAttribute = ESparseVolumeTexturePreviewAttribute(
        3,
    );
    pub const ESVTPA_ATTRIBUTES_B_R: ESparseVolumeTexturePreviewAttribute = ESparseVolumeTexturePreviewAttribute(
        4,
    );
    pub const ESVTPA_ATTRIBUTES_B_G: ESparseVolumeTexturePreviewAttribute = ESparseVolumeTexturePreviewAttribute(
        5,
    );
    pub const ESVTPA_ATTRIBUTES_B_B: ESparseVolumeTexturePreviewAttribute = ESparseVolumeTexturePreviewAttribute(
        6,
    );
    pub const ESVTPA_ATTRIBUTES_B_A: ESparseVolumeTexturePreviewAttribute = ESparseVolumeTexturePreviewAttribute(
        7,
    );
}
