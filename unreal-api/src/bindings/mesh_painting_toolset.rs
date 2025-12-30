#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FPaintTexture2DData {
    pub painting_texture2_d: UPtr<UTexture2D>,
    pub paint_render_target_texture: UPtr<UTextureRenderTarget2D>,
    pub paint_render_target_texture_adapter: UPtr<UVirtualTextureAdapter>,
    pub paint_brush_render_target_texture: UPtr<UTextureRenderTarget2D>,
    pub brush_render_target_texture: UPtr<UTextureRenderTarget2D>,
    pub brush_mask_render_target_texture: UPtr<UTextureRenderTarget2D>,
    pub seam_mask_render_target_texture: UPtr<UTextureRenderTarget2D>,
    pub texture_override_components: TArray<UPtr<UMeshComponent>>,
    pub painted_components: TArray<UPtr<UMeshComponent>>,
}
pub struct UMeshPaintingToolProperties {
    pub paint_color: FLinearColor,
    pub erase_color: FLinearColor,
    pub b_enable_flow: bool,
    pub b_only_front_facing_triangles: bool,
}
pub struct UMeshPaintingSubsystem {
    pub last_painted_component: UPtr<UMeshComponent>,
}
pub struct UMeshPaintSelectionInterface {}
pub struct IMeshPaintSelectionInterface {}
pub struct UMeshPaintSelectionMechanic {
    pub cached_clicked_components: TArray<UPtr<UMeshComponent>>,
    pub cached_clicked_actors: TArray<UPtr<AActor>>,
}
pub struct UVertexAdapterClickToolBuilder {}
pub struct UTextureColorAdapterClickToolBuilder {}
pub struct UTextureAssetAdapterClickToolBuilder {}
pub struct UMeshClickTool {
    pub selection_mechanic: UPtr<UMeshPaintSelectionMechanic>,
}
pub struct UVertexAdapterClickTool {}
pub struct UTextureColorAdapterClickTool {}
pub struct UTextureAssetAdapterClickTool {}
pub struct UMeshTextureColorPaintingToolBuilder {}
pub struct UMeshTextureAssetPaintingToolBuilder {}
pub struct UMeshTexturePaintingToolProperties {
    pub b_enable_seam_painting: bool,
    pub paint_brush: UPtr<UTexture2D>,
    pub paint_brush_rotation_offset: f32,
    pub b_rotate_brush_towards_direction: bool,
    pub b_write_red: bool,
    pub b_write_green: bool,
    pub b_write_blue: bool,
    pub b_write_alpha: bool,
}
pub struct UMeshTextureColorPaintingToolProperties {
    pub b_propagate_to_vertex_color: bool,
}
pub struct UMeshTextureAssetPaintingToolProperties {
    pub uv_channel: i32,
    pub paint_texture: UPtr<UTexture2D>,
}
pub struct UMeshTexturePaintingTool {
    pub paint_target_data: TMap<UPtr<UTexture2D>, FPaintTexture2DData>,
    pub selection_mechanic: UPtr<UMeshPaintSelectionMechanic>,
    pub texture_properties: UPtr<UMeshTexturePaintingToolProperties>,
    pub painting_texture2_d: UPtr<UTexture2D>,
}
pub struct UMeshTextureColorPaintingTool {
    pub color_properties: UPtr<UMeshTextureColorPaintingToolProperties>,
    pub mesh_paint_dummy_texture: UPtr<UTexture>,
}
pub struct UMeshTextureAssetPaintingTool {
    pub asset_properties: UPtr<UMeshTextureAssetPaintingToolProperties>,
}
pub struct UMeshVertexColorPaintingToolBuilder {}
pub struct UMeshVertexWeightPaintingToolBuilder {}
pub struct UMeshVertexPaintingToolProperties {
    pub b_paint_on_specific_lod: bool,
    pub lod_index: i32,
    pub vertex_preview_size: f32,
}
pub struct UMeshVertexColorPaintingToolProperties {
    pub b_write_red: bool,
    pub b_write_green: bool,
    pub b_write_blue: bool,
    pub b_write_alpha: bool,
}
pub struct UMeshVertexWeightPaintingToolProperties {
    pub texture_weight_type: EMeshPaintWeightTypes,
    pub paint_texture_weight_index: EMeshPaintTextureIndex,
    pub erase_texture_weight_index: EMeshPaintTextureIndex,
}
pub struct UMeshVertexPaintingTool {
    pub selection_mechanic: UPtr<UMeshPaintSelectionMechanic>,
    pub vertex_properties: UPtr<UMeshVertexPaintingToolProperties>,
}
pub struct UMeshVertexColorPaintingTool {
    pub color_properties: UPtr<UMeshVertexColorPaintingToolProperties>,
}
pub struct UMeshVertexWeightPaintingTool {
    pub weight_properties: UPtr<UMeshVertexWeightPaintingToolProperties>,
}
pub struct UTexturePaintToolset {}
