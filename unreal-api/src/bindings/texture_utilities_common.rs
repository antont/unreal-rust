#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UTextureImportSettings {
    pub auto_vt_size: i32,
    pub auto_limit_dimension: i32,
    pub b_enable_normalize_normals: bool,
    pub b_enable_fast_mip_filter: bool,
    pub compressed_format_for_float_textures: ETextureImportFloatingPointFormat,
    pub png_infill: ETextureImportPNGInfill,
    pub b_do_automatic_texture_settings_for_non_pow2_textures: bool,
}
pub struct UTextureImportUserSettings {
    pub png_infill: ETextureImportPNGInfill,
}
