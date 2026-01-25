#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {}
    }
}
pub fn initialize() {}
#[repr(C, align(16))]
pub struct USparseVolumeTextureViewerComponent {
    #[doc(hidden)]
    pub(crate) __padding_1504: [u8; 1504],
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
    __padding_end: [u8; 28],
}
impl USparseVolumeTextureViewerComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USparseVolumeTextureViewerComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ASparseVolumeTextureViewer {
    #[doc(hidden)]
    pub(crate) __padding_1144: [u8; 1144],
    pub sparse_volume_texture_viewer_component: UPtr<
        USparseVolumeTextureViewerComponent,
    >,
}
impl ASparseVolumeTextureViewer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ASparseVolumeTextureViewer")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
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
