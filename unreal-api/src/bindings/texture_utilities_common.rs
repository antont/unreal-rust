#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETextureImportFloatingPointFormat(pub u8);
impl ETextureImportFloatingPointFormat {
    pub const HDR_F16: ETextureImportFloatingPointFormat = ETextureImportFloatingPointFormat(
        0,
    );
    pub const HDR_COMPRESSED_BC6: ETextureImportFloatingPointFormat = ETextureImportFloatingPointFormat(
        1,
    );
    pub const HDR_F32_OR_F16: ETextureImportFloatingPointFormat = ETextureImportFloatingPointFormat(
        2,
    );
    pub const PREVIOUS_DEFAULT: ETextureImportFloatingPointFormat = ETextureImportFloatingPointFormat(
        0,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETextureImportPNGInfill(pub u8);
impl ETextureImportPNGInfill {
    pub const DEFAULT: ETextureImportPNGInfill = ETextureImportPNGInfill(0);
    pub const NEVER: ETextureImportPNGInfill = ETextureImportPNGInfill(1);
    pub const ONLY_ON_BINARY_TRANSPARENCY: ETextureImportPNGInfill = ETextureImportPNGInfill(
        2,
    );
    pub const ALWAYS: ETextureImportPNGInfill = ETextureImportPNGInfill(3);
}
