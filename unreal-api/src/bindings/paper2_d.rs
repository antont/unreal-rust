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
impl FIntMargin {}
#[repr(C, align(16))]
pub struct FPaperSpriteSocket {
    pub local_transform: crate::bindings::core_u_object::FTransform,
    pub socket_name: FName,
    __padding_end: [u8; 4],
}
impl FPaperSpriteSocket {}
#[repr(C, align(8))]
pub struct FPaperTileInfo {
    __padding_end: [u8; 16],
}
impl FPaperTileInfo {}
#[repr(C, align(8))]
pub struct FPaperTileMetadata {
    pub user_data_name: FName,
    __padding_end: [u8; 60],
}
impl FPaperTileMetadata {}
#[repr(C, align(8))]
pub struct FPaperTerrainMaterialRule {
    __padding_end: [u8; 72],
}
impl FPaperTerrainMaterialRule {}
#[repr(C, align(16))]
pub struct APaperCharacter {
    #[doc(hidden)]
    __padding_2112: [u8; 2112],
    pub sprite: UPtr<UPaperFlipbookComponent>,
    __padding_end: [u8; 8],
}
impl APaperCharacter {}
#[repr(C, align(8))]
pub struct UPaperFlipbook {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub frames_per_second: f32,
    #[doc(hidden)]
    __padding_72: [u8; 16],
    pub default_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub collision_source: EFlipbookCollisionMode,
    __padding_end: [u8; 7],
}
impl UPaperFlipbook {}
#[repr(C, align(8))]
pub struct APaperFlipbookActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub render_component: UPtr<UPaperFlipbookComponent>,
}
impl APaperFlipbookActor {}
#[repr(C, align(16))]
pub struct UPaperFlipbookComponent {
    #[doc(hidden)]
    __padding_1608: [u8; 1608],
    pub sprite_color: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 40],
}
impl UPaperFlipbookComponent {}
#[repr(C, align(8))]
pub struct APaperGroupedSpriteActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub render_component: UPtr<UPaperGroupedSpriteComponent>,
}
impl APaperGroupedSpriteActor {}
#[repr(C, align(16))]
pub struct UPaperGroupedSpriteComponent {
    __padding_end: [u8; 1632],
}
impl UPaperGroupedSpriteComponent {}
#[repr(C, align(8))]
pub struct UPaperRuntimeSettings {
    __padding_end: [u8; 56],
}
impl UPaperRuntimeSettings {}
#[repr(C, align(8))]
pub struct UPaperSprite {
    #[doc(hidden)]
    __padding_264: [u8; 264],
    pub default_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub alternate_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    __padding_end: [u8; 248],
}
impl UPaperSprite {}
#[repr(C, align(8))]
pub struct APaperSpriteActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub render_component: UPtr<UPaperSpriteComponent>,
}
impl APaperSpriteActor {}
#[repr(C, align(8))]
pub struct UPaperSpriteAtlas {
    __padding_end: [u8; 160],
}
impl UPaperSpriteAtlas {}
#[repr(C, align(8))]
pub struct UPaperSpriteBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UPaperSpriteBlueprintLibrary {}
#[repr(C, align(16))]
pub struct UPaperSpriteComponent {
    #[doc(hidden)]
    __padding_1576: [u8; 1576],
    pub source_sprite: UPtr<UPaperSprite>,
    #[doc(hidden)]
    __padding_1592: [u8; 8],
    pub sprite_color: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 8],
}
impl UPaperSpriteComponent {}
#[repr(C, align(8))]
pub struct UPaperTileLayer {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub layer_name: FText,
    pub layer_width: i32,
    pub layer_height: i32,
    pub flags_72: u8,
    pub collision_thickness_override: f32,
    pub collision_offset_override: f32,
    pub layer_color: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 52],
}
impl UPaperTileLayer {}
#[repr(C, align(8))]
pub struct UPaperTileMap {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub map_width: i32,
    pub map_height: i32,
    pub tile_width: i32,
    pub tile_height: i32,
    #[doc(hidden)]
    __padding_76: [u8; 12],
    pub separation_per_layer: f32,
    #[doc(hidden)]
    __padding_128: [u8; 48],
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub tile_layers: TArray<UPtr<UPaperTileLayer>>,
    pub collision_thickness: f32,
    pub sprite_collision_domain: ESpriteCollisionMode,
    pub projection_mode: ETileMapProjectionMode,
    __padding_end: [u8; 114],
}
impl UPaperTileMap {}
#[repr(C, align(8))]
pub struct APaperTileMapActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub render_component: UPtr<UPaperTileMapComponent>,
}
impl APaperTileMapActor {}
#[repr(C, align(16))]
pub struct UPaperTileMapComponent {
    #[doc(hidden)]
    __padding_1656: [u8; 1656],
    pub tile_map: UPtr<UPaperTileMap>,
    __padding_end: [u8; 16],
}
impl UPaperTileMapComponent {}
#[repr(C, align(8))]
pub struct UPaperTileSet {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub tile_size: crate::bindings::core_u_object::FIntPoint,
    pub tile_sheet: UPtr<crate::bindings::engine::UTexture2D>,
    #[doc(hidden)]
    __padding_80: [u8; 16],
    pub border_margin: FIntMargin,
    pub per_tile_spacing: crate::bindings::core_u_object::FIntPoint,
    pub drawing_offset: crate::bindings::core_u_object::FIntPoint,
    __padding_end: [u8; 80],
}
impl UPaperTileSet {}
#[repr(C, align(8))]
pub struct UMaterialExpressionSpriteTextureSampler {
    __padding_end: [u8; 656],
}
impl UMaterialExpressionSpriteTextureSampler {}
#[repr(C, align(8))]
pub struct APaperTerrainActor {
    #[doc(hidden)]
    __padding_1152: [u8; 1152],
    pub render_component: UPtr<UPaperTerrainComponent>,
}
impl APaperTerrainActor {}
#[repr(C, align(16))]
pub struct UPaperTerrainComponent {
    #[doc(hidden)]
    __padding_1504: [u8; 1504],
    pub terrain_material: UPtr<UPaperTerrainMaterial>,
    pub b_closed_spline: bool,
    pub b_filled_spline: bool,
    #[doc(hidden)]
    __padding_1536: [u8; 20],
    pub terrain_color: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 48],
}
impl UPaperTerrainComponent {}
#[repr(C, align(8))]
pub struct UPaperTerrainMaterial {
    __padding_end: [u8; 80],
}
impl UPaperTerrainMaterial {}
#[repr(C, align(16))]
pub struct UPaperTerrainSplineComponent {
    __padding_end: [u8; 2192],
}
impl UPaperTerrainSplineComponent {}
#[repr(C, align(8))]
pub struct UTileMapBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UTileMapBlueprintLibrary {}
#[repr(transparent)]
pub struct FPaperFlipbookComponent_OnFinishedPlaying {
    _opague: u8,
}
#[repr(transparent)]
pub struct ESpriteShapeType(pub u8);
impl ESpriteShapeType {
    pub const BOX: ESpriteShapeType = ESpriteShapeType(0);
    pub const CIRCLE: ESpriteShapeType = ESpriteShapeType(1);
    pub const POLYGON: ESpriteShapeType = ESpriteShapeType(2);
}
#[repr(transparent)]
pub struct ESpritePolygonMode(pub u8);
impl ESpritePolygonMode {
    pub const SOURCE_BOUNDING_BOX: ESpritePolygonMode = ESpritePolygonMode(0);
    pub const TIGHT_BOUNDING_BOX: ESpritePolygonMode = ESpritePolygonMode(1);
    pub const SHRINK_WRAPPED: ESpritePolygonMode = ESpritePolygonMode(2);
    pub const FULLY_CUSTOM: ESpritePolygonMode = ESpritePolygonMode(3);
    pub const DICED: ESpritePolygonMode = ESpritePolygonMode(4);
}
#[repr(transparent)]
pub struct EFlipbookCollisionMode(pub u8);
impl EFlipbookCollisionMode {
    pub const NO_COLLISION: EFlipbookCollisionMode = EFlipbookCollisionMode(0);
    pub const FIRST_FRAME_COLLISION: EFlipbookCollisionMode = EFlipbookCollisionMode(1);
    pub const EACH_FRAME_COLLISION: EFlipbookCollisionMode = EFlipbookCollisionMode(2);
}
#[repr(transparent)]
pub struct ESpriteCollisionMode(pub u8);
impl ESpriteCollisionMode {
    pub const NONE: ESpriteCollisionMode = ESpriteCollisionMode(0);
    pub const USE2_D_PHYSICS: ESpriteCollisionMode = ESpriteCollisionMode(1);
    pub const USE3_D_PHYSICS: ESpriteCollisionMode = ESpriteCollisionMode(2);
}
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
#[repr(transparent)]
pub struct EPaperSpriteAtlasPadding(pub u8);
impl EPaperSpriteAtlasPadding {
    pub const DILATE_BORDER: EPaperSpriteAtlasPadding = EPaperSpriteAtlasPadding(0);
    pub const PAD_WITH_ZERO: EPaperSpriteAtlasPadding = EPaperSpriteAtlasPadding(1);
}
#[repr(transparent)]
pub struct ETileMapProjectionMode(pub u8);
impl ETileMapProjectionMode {
    pub const ORTHOGONAL: ETileMapProjectionMode = ETileMapProjectionMode(0);
    pub const ISOMETRIC_DIAMOND: ETileMapProjectionMode = ETileMapProjectionMode(1);
    pub const ISOMETRIC_STAGGERED: ETileMapProjectionMode = ETileMapProjectionMode(2);
    pub const HEXAGONAL_STAGGERED: ETileMapProjectionMode = ETileMapProjectionMode(3);
}
