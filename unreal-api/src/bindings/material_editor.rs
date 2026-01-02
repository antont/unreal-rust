#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FMaterialStatistics {
    pub num_vertex_shader_instructions: i32,
    pub num_pixel_shader_instructions: i32,
    pub num_samplers: i32,
    pub num_vertex_texture_samples: i32,
    pub num_pixel_texture_samples: i32,
    pub num_virtual_texture_samples: i32,
    pub num_uv_scalars: i32,
    pub num_interpolator_scalars: i32,
}
impl FMaterialStatistics {}
#[repr(C, align(8))]
pub struct UMaterialEditorMenuContext {
    __padding_end: [u8; 64],
}
impl UMaterialEditorMenuContext {}
#[repr(C, align(8))]
pub struct UMaterialEditorSettings {
    __padding_end: [u8; 216],
}
impl UMaterialEditorSettings {}
#[repr(C, align(8))]
pub struct UMaterialEditingLibrary {
    __padding_end: [u8; 48],
}
impl UMaterialEditingLibrary {}
#[repr(transparent)]
pub struct EBackgroundType(pub u8);
impl EBackgroundType {
    pub const SOLID_COLOR: EBackgroundType = EBackgroundType(0);
    pub const CHECKERED: EBackgroundType = EBackgroundType(1);
}
#[repr(transparent)]
pub struct EOfflineShaderCompiler(pub u8);
impl EOfflineShaderCompiler {
    pub const MALI: EOfflineShaderCompiler = EOfflineShaderCompiler(0);
    pub const ADRENO: EOfflineShaderCompiler = EOfflineShaderCompiler(1);
}
