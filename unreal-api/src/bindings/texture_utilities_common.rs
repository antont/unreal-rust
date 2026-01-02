#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UTextureImportSettings {
    __padding_end: [u8; 120],
}
impl UTextureImportSettings {}
#[repr(C, align(8))]
pub struct UTextureImportUserSettings {
    __padding_end: [u8; 112],
}
impl UTextureImportUserSettings {}
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
