#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FCopyVertexColorToClothParams {
    pub color_channel: ESourceColorChannel,
    pub scaling_factor: f32,
}
pub struct UClothPainterSettings {
    pub view_min: f32,
    pub view_max: f32,
    pub b_auto_view_range: bool,
    pub auto_calculated_view_min: f32,
    pub auto_calculated_view_max: f32,
    pub clothing_assets: TArray<
        UPtr<crate::bindings::clothing_system_runtime_common::UClothingAssetCommon>,
    >,
    pub b_flip_normal: bool,
    pub b_cull_backface: bool,
    pub opacity: f32,
}
pub struct UClothingAssetExporter {}
pub struct UClothPaintTool_BrushSettings {
    pub paint_value: f32,
}
pub struct UClothPaintTool_GradientSettings {
    pub gradient_start_value: f32,
    pub gradient_end_value: f32,
    pub b_use_regular_brush: bool,
}
pub struct UClothPaintTool_SmoothSettings {
    pub strength: f32,
}
pub struct UClothPaintTool_FillSettings {
    pub threshold: f32,
    pub fill_value: f32,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESourceColorChannel(pub u8);
impl ESourceColorChannel {
    pub const RED: ESourceColorChannel = ESourceColorChannel(0);
    pub const GREEN: ESourceColorChannel = ESourceColorChannel(1);
    pub const BLUE: ESourceColorChannel = ESourceColorChannel(2);
    pub const ALPHA: ESourceColorChannel = ESourceColorChannel(3);
}
