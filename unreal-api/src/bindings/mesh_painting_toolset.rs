#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FPaintTexture2DData {
    pub painting_texture2_d: UPtr<crate::bindings::engine::UTexture2D>,
    pub paint_render_target_texture: UPtr<
        crate::bindings::engine::UTextureRenderTarget2D,
    >,
    pub paint_render_target_texture_adapter: UPtr<
        crate::bindings::engine::UVirtualTextureAdapter,
    >,
    pub paint_brush_render_target_texture: UPtr<
        crate::bindings::engine::UTextureRenderTarget2D,
    >,
    pub brush_render_target_texture: UPtr<
        crate::bindings::engine::UTextureRenderTarget2D,
    >,
    pub brush_mask_render_target_texture: UPtr<
        crate::bindings::engine::UTextureRenderTarget2D,
    >,
    pub seam_mask_render_target_texture: UPtr<
        crate::bindings::engine::UTextureRenderTarget2D,
    >,
    pub texture_override_components: TArray<
        UPtr<crate::bindings::engine::UMeshComponent>,
    >,
    pub painted_components: TArray<UPtr<crate::bindings::engine::UMeshComponent>>,
}
pub struct UMeshPaintingToolProperties {
    pub paint_color: crate::bindings::core_u_object::FLinearColor,
    pub erase_color: crate::bindings::core_u_object::FLinearColor,
    pub b_enable_flow: bool,
    pub b_only_front_facing_triangles: bool,
}
pub struct UMeshPaintingSubsystem {
    pub last_painted_component: UPtr<crate::bindings::engine::UMeshComponent>,
}
pub struct UMeshPaintSelectionInterface {}
pub struct IMeshPaintSelectionInterface {}
pub struct UMeshPaintSelectionMechanic {
    pub cached_clicked_components: TArray<UPtr<crate::bindings::engine::UMeshComponent>>,
    pub cached_clicked_actors: TArray<UPtr<crate::bindings::engine::AActor>>,
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
    pub paint_brush: UPtr<crate::bindings::engine::UTexture2D>,
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
    pub paint_texture: UPtr<crate::bindings::engine::UTexture2D>,
}
pub struct UMeshTexturePaintingTool {
    pub paint_target_data: TMap<
        UPtr<crate::bindings::engine::UTexture2D>,
        FPaintTexture2DData,
    >,
    pub selection_mechanic: UPtr<UMeshPaintSelectionMechanic>,
    pub texture_properties: UPtr<UMeshTexturePaintingToolProperties>,
    pub painting_texture2_d: UPtr<crate::bindings::engine::UTexture2D>,
}
pub struct UMeshTextureColorPaintingTool {
    pub color_properties: UPtr<UMeshTextureColorPaintingToolProperties>,
    pub mesh_paint_dummy_texture: UPtr<crate::bindings::engine::UTexture>,
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshPaintWeightTypes(pub u8);
impl EMeshPaintWeightTypes {
    pub const ALPHA_LERP: EMeshPaintWeightTypes = EMeshPaintWeightTypes(2);
    pub const RGB: EMeshPaintWeightTypes = EMeshPaintWeightTypes(3);
    pub const ARGB: EMeshPaintWeightTypes = EMeshPaintWeightTypes(4);
    pub const ONE_MINUS_ARGB: EMeshPaintWeightTypes = EMeshPaintWeightTypes(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshPaintTextureIndex(pub u8);
impl EMeshPaintTextureIndex {
    pub const TEXTURE_ONE: EMeshPaintTextureIndex = EMeshPaintTextureIndex(0);
    pub const TEXTURE_TWO: EMeshPaintTextureIndex = EMeshPaintTextureIndex(1);
    pub const TEXTURE_THREE: EMeshPaintTextureIndex = EMeshPaintTextureIndex(2);
    pub const TEXTURE_FOUR: EMeshPaintTextureIndex = EMeshPaintTextureIndex(3);
    pub const TEXTURE_FIVE: EMeshPaintTextureIndex = EMeshPaintTextureIndex(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshPaintDataColorViewMode(pub u8);
impl EMeshPaintDataColorViewMode {
    pub const NORMAL: EMeshPaintDataColorViewMode = EMeshPaintDataColorViewMode(0);
    pub const RGB: EMeshPaintDataColorViewMode = EMeshPaintDataColorViewMode(1);
    pub const ALPHA: EMeshPaintDataColorViewMode = EMeshPaintDataColorViewMode(2);
    pub const RED: EMeshPaintDataColorViewMode = EMeshPaintDataColorViewMode(3);
    pub const GREEN: EMeshPaintDataColorViewMode = EMeshPaintDataColorViewMode(4);
    pub const BLUE: EMeshPaintDataColorViewMode = EMeshPaintDataColorViewMode(5);
}
