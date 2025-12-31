#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FIntMargin {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[repr(C, align(16))]
pub struct FSpriteDrawCallRecord {
    pub destination: crate::bindings::core_u_object::FVector,
    pub base_texture: UPtr<crate::bindings::engine::UTexture>,
    pub color: crate::bindings::core_u_object::FColor,
}
#[repr(C, align(8))]
pub struct FSpriteGeometryShape {
    pub shape_type: ESpriteShapeType,
    pub vertices: TArray<crate::bindings::core_u_object::FVector2D>,
    pub box_size: crate::bindings::core_u_object::FVector2D,
    pub box_position: crate::bindings::core_u_object::FVector2D,
    pub rotation: f32,
    pub b_negative_winding: bool,
}
#[repr(C, align(8))]
pub struct FSpriteGeometryCollection {
    pub shapes: TArray<FSpriteGeometryShape>,
    pub geometry_type: ESpritePolygonMode,
    pub pixels_per_subdivision_x: i32,
    pub pixels_per_subdivision_y: i32,
    pub b_avoid_vertex_merging: bool,
    pub alpha_threshold: f32,
    pub detail_amount: f32,
    pub simplify_epsilon: f32,
}
#[repr(C, align(8))]
pub struct FSpriteAssetInitParameters {}
#[repr(C, align(8))]
pub struct FPaperFlipbookKeyFrame {
    pub sprite: UPtr<UPaperSprite>,
    pub frame_run: i32,
}
#[repr(C, align(16))]
pub struct FSpriteInstanceData {
    pub transform: crate::bindings::core_u_object::FMatrix,
    pub source_sprite: UPtr<UPaperSprite>,
    pub vertex_color: crate::bindings::core_u_object::FColor,
    pub material_index: i32,
}
#[repr(C, align(16))]
pub struct FPaperSpriteSocket {
    pub local_transform: crate::bindings::core_u_object::FTransform,
    pub socket_name: FName,
}
#[repr(C, align(8))]
pub struct FPaperSpriteAtlasSlot {
    pub sprite_ref: TSoftObjectPtr<UPaperSprite>,
    pub atlas_index: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
#[repr(C, align(8))]
pub struct FPaperTileInfo {
    pub tile_set: UPtr<UPaperTileSet>,
    pub packed_tile_index: i32,
}
#[repr(C, align(8))]
pub struct FPaperTileMetadata {
    pub user_data_name: FName,
    pub collision_data: FSpriteGeometryCollection,
    pub terrain_membership: u8,
}
#[repr(C, align(8))]
pub struct FPaperTileSetTerrain {
    pub terrain_name: FString,
    pub center_tile_index: i32,
}
#[repr(C, align(8))]
pub struct FPaperTerrainMaterialRule {
    pub start_cap: UPtr<UPaperSprite>,
    pub body: TArray<UPtr<UPaperSprite>>,
    pub end_cap: UPtr<UPaperSprite>,
    pub minimum_angle: f32,
    pub maximum_angle: f32,
    pub b_enable_collision: bool,
    pub collision_offset: f32,
    pub draw_order: i32,
    pub description: FText,
}
pub struct APaperCharacter {
    pub sprite: UPtr<UPaperFlipbookComponent>,
}
pub struct UPaperFlipbook {
    pub frames_per_second: f32,
    pub key_frames: TArray<FPaperFlipbookKeyFrame>,
    pub default_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub collision_source: EFlipbookCollisionMode,
}
pub struct APaperFlipbookActor {
    pub render_component: UPtr<UPaperFlipbookComponent>,
}
pub struct UPaperFlipbookComponent {
    pub source_flipbook: UPtr<UPaperFlipbook>,
    pub material_deprecated: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub play_rate: f32,
    pub flags_1596: u8,
    pub accumulated_time: f32,
    pub cached_frame_index: i32,
    pub sprite_color: crate::bindings::core_u_object::FLinearColor,
    pub cached_body_setup: UPtr<crate::bindings::engine::UBodySetup>,
    pub on_finished_playing: FPaperFlipbookComponent_OnFinishedPlaying,
}
pub struct APaperGroupedSpriteActor {
    pub render_component: UPtr<UPaperGroupedSpriteComponent>,
}
pub struct UPaperGroupedSpriteComponent {
    pub instance_materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub per_instance_sprite_data: TArray<FSpriteInstanceData>,
}
pub struct UPaperRuntimeSettings {
    pub b_enable_sprite_atlas_groups: bool,
    pub b_enable_terrain_spline_editing: bool,
    pub b_resize_sprite_data_to_match_textures: bool,
}
pub struct UPaperSprite {
    pub origin_in_source_image_before_trimming: crate::bindings::core_u_object::FVector2D,
    pub source_image_dimension_before_trimming: crate::bindings::core_u_object::FVector2D,
    pub b_trimmed_in_source_image: bool,
    pub b_rotated_in_source_image: bool,
    pub source_texture_dimension: crate::bindings::core_u_object::FVector2D,
    pub source_uv: crate::bindings::core_u_object::FVector2D,
    pub source_dimension: crate::bindings::core_u_object::FVector2D,
    pub source_texture: TSoftObjectPtr<crate::bindings::engine::UTexture2D>,
    pub source_texture_cache_never_serialized: UPtr<crate::bindings::engine::UTexture2D>,
    pub additional_source_textures: TArray<UPtr<crate::bindings::engine::UTexture>>,
    pub baked_source_uv: crate::bindings::core_u_object::FVector2D,
    pub baked_source_dimension: crate::bindings::core_u_object::FVector2D,
    pub baked_source_texture: UPtr<crate::bindings::engine::UTexture2D>,
    pub default_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub alternate_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub sockets: TArray<FPaperSpriteSocket>,
    pub sprite_collision_domain: ESpriteCollisionMode,
    pub pixels_per_unreal_unit: f32,
    pub body_setup: UPtr<crate::bindings::engine::UBodySetup>,
    pub pivot_mode: ESpritePivotMode,
    pub custom_pivot_point: crate::bindings::core_u_object::FVector2D,
    pub b_snap_pivot_to_pixel_grid: bool,
    pub collision_geometry: FSpriteGeometryCollection,
    pub collision_thickness: f32,
    pub render_geometry: FSpriteGeometryCollection,
    pub atlas_group: UPtr<UPaperSpriteAtlas>,
    pub alternate_material_split_index: i32,
    pub baked_render_data: TArray<crate::bindings::core_u_object::FVector4>,
}
pub struct APaperSpriteActor {
    pub render_component: UPtr<UPaperSpriteComponent>,
}
pub struct UPaperSpriteAtlas {
    pub atlas_description: FString,
    pub max_width: i32,
    pub max_height: i32,
    pub mip_count: i32,
    pub padding_type: EPaperSpriteAtlasPadding,
    pub padding: i32,
    pub compression_settings: crate::bindings::engine::TextureCompressionSettings,
    pub filter: crate::bindings::engine::TextureFilter,
    pub generated_textures: TArray<UPtr<crate::bindings::engine::UTexture>>,
    pub atlas_guid: crate::bindings::core_u_object::FGuid,
    pub b_rebuild_atlas: bool,
    pub atlas_slots: TArray<FPaperSpriteAtlasSlot>,
    pub num_incremental_builds: i32,
    pub built_width: i32,
    pub built_height: i32,
    pub built_padding: i32,
}
pub struct UPaperSpriteBlueprintLibrary {}
pub struct UPaperSpriteComponent {
    pub source_sprite: UPtr<UPaperSprite>,
    pub material_override_deprecated: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub sprite_color: crate::bindings::core_u_object::FLinearColor,
}
pub struct UPaperTileLayer {
    pub layer_name: FText,
    pub layer_width: i32,
    pub layer_height: i32,
    pub flags_72: u8,
    pub collision_thickness_override: f32,
    pub collision_offset_override: f32,
    pub layer_color: crate::bindings::core_u_object::FLinearColor,
    pub allocated_width: i32,
    pub allocated_height: i32,
    pub allocated_cells: TArray<FPaperTileInfo>,
    pub tile_set_deprecated: UPtr<UPaperTileSet>,
    pub allocated_grid_deprecated: TArray<i32>,
}
pub struct UPaperTileMap {
    pub map_width: i32,
    pub map_height: i32,
    pub tile_width: i32,
    pub tile_height: i32,
    pub pixels_per_unreal_unit: f32,
    pub separation_per_tile_x: f32,
    pub separation_per_tile_y: f32,
    pub separation_per_layer: f32,
    pub selected_tile_set: TSoftObjectPtr<UPaperTileSet>,
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub tile_layers: TArray<UPtr<UPaperTileLayer>>,
    pub collision_thickness: f32,
    pub sprite_collision_domain: ESpriteCollisionMode,
    pub projection_mode: ETileMapProjectionMode,
    pub hex_side_length: i32,
    pub body_setup: UPtr<crate::bindings::engine::UBodySetup>,
    pub asset_import_data: UPtr<crate::bindings::engine::UAssetImportData>,
    pub selected_layer_index: i32,
    pub background_color: crate::bindings::core_u_object::FLinearColor,
    pub tile_grid_color: crate::bindings::core_u_object::FLinearColor,
    pub multi_tile_grid_color: crate::bindings::core_u_object::FLinearColor,
    pub multi_tile_grid_width: i32,
    pub multi_tile_grid_height: i32,
    pub multi_tile_grid_offset_x: i32,
    pub multi_tile_grid_offset_y: i32,
    pub layer_grid_color: crate::bindings::core_u_object::FLinearColor,
    pub layer_name_index: i32,
}
pub struct APaperTileMapActor {
    pub render_component: UPtr<UPaperTileMapComponent>,
}
pub struct UPaperTileMapComponent {
    pub map_width_deprecated: i32,
    pub map_height_deprecated: i32,
    pub tile_width_deprecated: i32,
    pub tile_height_deprecated: i32,
    pub default_layer_tile_set_deprecated: UPtr<UPaperTileSet>,
    pub material_deprecated: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub tile_layers_deprecated: TArray<UPtr<UPaperTileLayer>>,
    pub tile_map_color: crate::bindings::core_u_object::FLinearColor,
    pub use_single_layer_index: i32,
    pub b_use_single_layer: bool,
    pub tile_map: UPtr<UPaperTileMap>,
    pub b_show_per_tile_grid_when_selected: bool,
    pub b_show_per_layer_grid_when_selected: bool,
    pub b_show_outline_when_unselected: bool,
    pub b_show_per_tile_grid_when_unselected: bool,
    pub b_show_per_layer_grid_when_unselected: bool,
}
pub struct UPaperTileSet {
    pub tile_size: crate::bindings::core_u_object::FIntPoint,
    pub tile_sheet: UPtr<crate::bindings::engine::UTexture2D>,
    pub additional_source_textures: TArray<UPtr<crate::bindings::engine::UTexture>>,
    pub border_margin: FIntMargin,
    pub per_tile_spacing: crate::bindings::core_u_object::FIntPoint,
    pub drawing_offset: crate::bindings::core_u_object::FIntPoint,
    pub background_color: crate::bindings::core_u_object::FLinearColor,
    pub width_in_tiles: i32,
    pub height_in_tiles: i32,
    pub allocated_width: i32,
    pub allocated_height: i32,
    pub per_tile_data: TArray<FPaperTileMetadata>,
    pub terrains: TArray<FPaperTileSetTerrain>,
    pub tile_width_deprecated: i32,
    pub tile_height_deprecated: i32,
    pub margin_deprecated: i32,
    pub spacing_deprecated: i32,
}
pub struct UMaterialExpressionSpriteTextureSampler {
    pub b_sample_additional_textures: bool,
    pub additional_slot_index: i32,
    pub slot_display_name: FText,
}
pub struct APaperTerrainActor {
    pub dummy_root: UPtr<crate::bindings::engine::USceneComponent>,
    pub spline_component: UPtr<UPaperTerrainSplineComponent>,
    pub render_component: UPtr<UPaperTerrainComponent>,
}
pub struct UPaperTerrainComponent {
    pub terrain_material: UPtr<UPaperTerrainMaterial>,
    pub b_closed_spline: bool,
    pub b_filled_spline: bool,
    pub associated_spline: UPtr<UPaperTerrainSplineComponent>,
    pub random_seed: i32,
    pub segment_overlap_amount: f32,
    pub terrain_color: crate::bindings::core_u_object::FLinearColor,
    pub reparam_steps_per_segment: i32,
    pub sprite_collision_domain: ESpriteCollisionMode,
    pub collision_thickness: f32,
    pub cached_body_setup: UPtr<crate::bindings::engine::UBodySetup>,
}
pub struct UPaperTerrainMaterial {
    pub rules: TArray<FPaperTerrainMaterialRule>,
    pub interior_fill: UPtr<UPaperSprite>,
}
pub struct UPaperTerrainSplineComponent {}
pub struct UTileMapBlueprintLibrary {}
pub struct FPaperFlipbookComponent_OnFinishedPlaying;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESpriteShapeType(pub u8);
impl ESpriteShapeType {
    pub const BOX: ESpriteShapeType = ESpriteShapeType(0);
    pub const CIRCLE: ESpriteShapeType = ESpriteShapeType(1);
    pub const POLYGON: ESpriteShapeType = ESpriteShapeType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESpritePolygonMode(pub u8);
impl ESpritePolygonMode {
    pub const SOURCE_BOUNDING_BOX: ESpritePolygonMode = ESpritePolygonMode(0);
    pub const TIGHT_BOUNDING_BOX: ESpritePolygonMode = ESpritePolygonMode(1);
    pub const SHRINK_WRAPPED: ESpritePolygonMode = ESpritePolygonMode(2);
    pub const FULLY_CUSTOM: ESpritePolygonMode = ESpritePolygonMode(3);
    pub const DICED: ESpritePolygonMode = ESpritePolygonMode(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFlipbookCollisionMode(pub u8);
impl EFlipbookCollisionMode {
    pub const NO_COLLISION: EFlipbookCollisionMode = EFlipbookCollisionMode(0);
    pub const FIRST_FRAME_COLLISION: EFlipbookCollisionMode = EFlipbookCollisionMode(1);
    pub const EACH_FRAME_COLLISION: EFlipbookCollisionMode = EFlipbookCollisionMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESpriteCollisionMode(pub u8);
impl ESpriteCollisionMode {
    pub const NONE: ESpriteCollisionMode = ESpriteCollisionMode(0);
    pub const USE2_D_PHYSICS: ESpriteCollisionMode = ESpriteCollisionMode(1);
    pub const USE3_D_PHYSICS: ESpriteCollisionMode = ESpriteCollisionMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESpritePivotMode(pub u8);
impl ESpritePivotMode {
    pub const TOP_LEFT: ESpritePivotMode = ESpritePivotMode(0);
    pub const TOP_CENTER: ESpritePivotMode = ESpritePivotMode(1);
    pub const TOP_RIGHT: ESpritePivotMode = ESpritePivotMode(2);
    pub const CENTER_LEFT: ESpritePivotMode = ESpritePivotMode(3);
    pub const CENTER_CENTER: ESpritePivotMode = ESpritePivotMode(4);
    pub const CENTER_RIGHT: ESpritePivotMode = ESpritePivotMode(5);
    pub const BOTTOM_LEFT: ESpritePivotMode = ESpritePivotMode(6);
    pub const BOTTOM_CENTER: ESpritePivotMode = ESpritePivotMode(7);
    pub const BOTTOM_RIGHT: ESpritePivotMode = ESpritePivotMode(8);
    pub const CUSTOM: ESpritePivotMode = ESpritePivotMode(9);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPaperSpriteAtlasPadding(pub u8);
impl EPaperSpriteAtlasPadding {
    pub const DILATE_BORDER: EPaperSpriteAtlasPadding = EPaperSpriteAtlasPadding(0);
    pub const PAD_WITH_ZERO: EPaperSpriteAtlasPadding = EPaperSpriteAtlasPadding(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETileMapProjectionMode(pub u8);
impl ETileMapProjectionMode {
    pub const ORTHOGONAL: ETileMapProjectionMode = ETileMapProjectionMode(0);
    pub const ISOMETRIC_DIAMOND: ETileMapProjectionMode = ETileMapProjectionMode(1);
    pub const ISOMETRIC_STAGGERED: ETileMapProjectionMode = ETileMapProjectionMode(2);
    pub const HEXAGONAL_STAGGERED: ETileMapProjectionMode = ETileMapProjectionMode(3);
}
