#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(transparent)]
pub struct EInterchangeSparseVolumeTextureFormat(pub u8);
impl EInterchangeSparseVolumeTextureFormat {
    pub const UNORM8: EInterchangeSparseVolumeTextureFormat = EInterchangeSparseVolumeTextureFormat(
        0,
    );
    pub const FLOAT16: EInterchangeSparseVolumeTextureFormat = EInterchangeSparseVolumeTextureFormat(
        1,
    );
    pub const FLOAT32: EInterchangeSparseVolumeTextureFormat = EInterchangeSparseVolumeTextureFormat(
        2,
    );
}
#[repr(transparent)]
pub struct EInterchangeMaterialXShaders(pub u8);
impl EInterchangeMaterialXShaders {
    pub const OPEN_PBR_SURFACE: EInterchangeMaterialXShaders = EInterchangeMaterialXShaders(
        0,
    );
    pub const OPEN_PBR_SURFACE_TRANSMISSION: EInterchangeMaterialXShaders = EInterchangeMaterialXShaders(
        1,
    );
    pub const STANDARD_SURFACE: EInterchangeMaterialXShaders = EInterchangeMaterialXShaders(
        2,
    );
    pub const STANDARD_SURFACE_TRANSMISSION: EInterchangeMaterialXShaders = EInterchangeMaterialXShaders(
        3,
    );
    pub const SURFACE_UNLIT: EInterchangeMaterialXShaders = EInterchangeMaterialXShaders(
        4,
    );
    pub const USD_PREVIEW_SURFACE: EInterchangeMaterialXShaders = EInterchangeMaterialXShaders(
        5,
    );
    pub const SURFACE: EInterchangeMaterialXShaders = EInterchangeMaterialXShaders(6);
    pub const DISPLACEMENT: EInterchangeMaterialXShaders = EInterchangeMaterialXShaders(
        7,
    );
    pub const VOLUME: EInterchangeMaterialXShaders = EInterchangeMaterialXShaders(8);
    pub const MAX_SHADER_COUNT: EInterchangeMaterialXShaders = EInterchangeMaterialXShaders(
        9,
    );
}
#[repr(transparent)]
pub struct EInterchangeMaterialXBSDF(pub u8);
impl EInterchangeMaterialXBSDF {
    pub const OREN_NAYAR_DIFFUSE: EInterchangeMaterialXBSDF = EInterchangeMaterialXBSDF(
        0,
    );
    pub const BURLEY_DIFFUSE: EInterchangeMaterialXBSDF = EInterchangeMaterialXBSDF(1);
    pub const TRANSLUCENT: EInterchangeMaterialXBSDF = EInterchangeMaterialXBSDF(2);
    pub const DIELECTRIC: EInterchangeMaterialXBSDF = EInterchangeMaterialXBSDF(3);
    pub const CONDUCTOR: EInterchangeMaterialXBSDF = EInterchangeMaterialXBSDF(4);
    pub const GENERALIZED_SCHLICK: EInterchangeMaterialXBSDF = EInterchangeMaterialXBSDF(
        5,
    );
    pub const SUBSURFACE: EInterchangeMaterialXBSDF = EInterchangeMaterialXBSDF(6);
    pub const SHEEN: EInterchangeMaterialXBSDF = EInterchangeMaterialXBSDF(7);
    pub const CHIANG_HAIR: EInterchangeMaterialXBSDF = EInterchangeMaterialXBSDF(8);
    pub const THIN_FILM: EInterchangeMaterialXBSDF = EInterchangeMaterialXBSDF(9);
    pub const MAX_BSDF_COUNT: EInterchangeMaterialXBSDF = EInterchangeMaterialXBSDF(10);
}
#[repr(transparent)]
pub struct EInterchangeMaterialXEDF(pub u8);
impl EInterchangeMaterialXEDF {
    pub const UNIFORM: EInterchangeMaterialXEDF = EInterchangeMaterialXEDF(0);
    pub const CONICAL: EInterchangeMaterialXEDF = EInterchangeMaterialXEDF(1);
    pub const MEASURED: EInterchangeMaterialXEDF = EInterchangeMaterialXEDF(2);
    pub const MAX_EDF_COUNT: EInterchangeMaterialXEDF = EInterchangeMaterialXEDF(3);
}
#[repr(transparent)]
pub struct EInterchangeMaterialXVDF(pub u8);
impl EInterchangeMaterialXVDF {
    pub const ABSORPTION: EInterchangeMaterialXVDF = EInterchangeMaterialXVDF(0);
    pub const ANISOTROPIC: EInterchangeMaterialXVDF = EInterchangeMaterialXVDF(1);
    pub const MAX_VDF_COUNT: EInterchangeMaterialXVDF = EInterchangeMaterialXVDF(2);
}
