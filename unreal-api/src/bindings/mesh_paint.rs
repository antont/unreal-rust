#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UPaintBrushSettings {
    pub brush_radius: f32,
    pub brush_strength: f32,
    pub brush_falloff_amount: f32,
    pub b_enable_flow: bool,
    pub b_only_front_facing_triangles: bool,
    pub color_view_mode: EMeshPaintColorViewMode,
}
pub struct UMeshPaintSettings {
    pub vertex_preview_size: f32,
}
pub struct UVertexColorImportOptions {
    pub uv_index: i32,
    pub lod_index: i32,
    pub b_red: bool,
    pub b_blue: bool,
    pub b_green: bool,
    pub b_alpha: bool,
    pub b_import_to_instance: bool,
    pub b_can_import_to_instance: bool,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshPaintColorViewMode(pub u8);
impl EMeshPaintColorViewMode {
    pub const NORMAL: EMeshPaintColorViewMode = EMeshPaintColorViewMode(0);
    pub const RGB: EMeshPaintColorViewMode = EMeshPaintColorViewMode(1);
    pub const ALPHA: EMeshPaintColorViewMode = EMeshPaintColorViewMode(2);
    pub const RED: EMeshPaintColorViewMode = EMeshPaintColorViewMode(3);
    pub const GREEN: EMeshPaintColorViewMode = EMeshPaintColorViewMode(4);
    pub const BLUE: EMeshPaintColorViewMode = EMeshPaintColorViewMode(5);
}
