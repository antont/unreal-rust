#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UImportVertexColorOptions {
    pub uv_index: i32,
    pub lod_index: i32,
    pub b_red: bool,
    pub b_blue: bool,
    pub b_green: bool,
    pub b_alpha: bool,
    pub b_import_to_instance: bool,
    pub b_can_import_to_instance: bool,
}
pub struct UMeshPaintMode {
    pub mode_settings: UPtr<UMeshPaintModeSettings>,
}
pub struct UMeshPaintModeSettings {
    pub color_view_mode: crate::bindings::mesh_painting_toolset::EMeshPaintDataColorViewMode,
    pub default_palette: FName,
}
