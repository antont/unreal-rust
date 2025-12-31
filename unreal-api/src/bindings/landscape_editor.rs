#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FLandscapeFileResolution {
    pub width: u32,
    pub height: u32,
}
#[repr(C, align(8))]
pub struct FGizmoImportLayer {
    pub layer_filename: FString,
    pub layer_name: FString,
    pub b_no_import: bool,
}
#[repr(C, align(8))]
pub struct FLandscapeImportLayer {
    pub thumbnail_mic: UPtr<
        crate::bindings::landscape::ULandscapeMaterialInstanceConstant,
    >,
    pub import_result: ELandscapeImportResult,
    pub error_message: FText,
    pub export_file_path: FString,
    pub b_selected: bool,
}
#[repr(C, align(8))]
pub struct FLandscapePatternBrushWorldSpaceSettings {
    pub origin: crate::bindings::core_u_object::FVector2D,
    pub rotation: f32,
    pub b_center_texture_on_origin: bool,
    pub repeat_size: f32,
}
#[repr(C, align(8))]
pub struct FLandscapeTargetLayerAssetFilePath {
    pub b_use_asset_directory_path: bool,
    pub directory_path: crate::bindings::core_u_object::FDirectoryPath,
}
#[repr(C, align(8))]
pub struct FLandscapeImportFileDescriptor {
    pub coord: crate::bindings::core_u_object::FIntPoint,
    pub file_path: FString,
}
#[repr(C, align(4))]
pub struct FLandscapeImportResolution {
    pub width: u32,
    pub height: u32,
}
#[repr(C, align(8))]
pub struct FLandscapeImportDescriptor {
    pub import_resolutions: TArray<FLandscapeImportResolution>,
    pub file_resolutions: TArray<FLandscapeFileResolution>,
    pub file_descriptors: TArray<FLandscapeImportFileDescriptor>,
    pub scale: crate::bindings::core_u_object::FVector,
}
pub struct ALandscapePlaceholder {}
pub struct UActorFactoryLandscape {}
pub struct ULandscapeEditorObject {
    pub tool_strength: f32,
    pub paint_tool_strength: f32,
    pub b_use_weight_target_value: bool,
    pub weight_target_value: f32,
    pub maximum_value_radius: f32,
    pub b_combined_layers_operation: bool,
    pub flatten_mode: ELandscapeToolFlattenMode,
    pub b_use_slope_flatten: bool,
    pub b_pick_value_per_apply: bool,
    pub b_use_flatten_target: bool,
    pub flatten_target: f32,
    pub b_show_flatten_target_preview: bool,
    pub terrace_interval: f32,
    pub terrace_smooth: f32,
    pub b_flatten_eye_dropper_mode_activated: bool,
    pub b_flatten_eye_dropper_mode_mousing_over_viewport: bool,
    pub flatten_eye_dropper_mode_desired_target: f32,
    pub ramp_width: f32,
    pub ramp_side_falloff: f32,
    pub smooth_filter_kernel_size: i32,
    pub b_detail_smooth: bool,
    pub detail_scale: f32,
    pub erode_thresh: i32,
    pub erode_surface_thickness: i32,
    pub erode_iteration_num: i32,
    pub erosion_noise_mode: ELandscapeToolErosionMode,
    pub erosion_noise_scale: f32,
    pub b_erosion_use_layer_hardness: bool,
    pub rain_amount: i32,
    pub sediment_capacity: f32,
    pub h_erode_iteration_num: i32,
    pub rain_dist_mode: ELandscapeToolHydroErosionMode,
    pub rain_dist_scale: f32,
    pub b_h_erosion_detail_smooth: bool,
    pub h_erosion_detail_scale: f32,
    pub noise_mode: ELandscapeToolNoiseMode,
    pub noise_scale: f32,
    pub b_use_selected_region: bool,
    pub b_use_negative_mask: bool,
    pub paste_mode: ELandscapeToolPasteMode,
    pub b_apply_to_all_targets: bool,
    pub current_gizmo_actor: TWeakObjectPtr<
        crate::bindings::landscape::ALandscapeGizmoActiveActor,
    >,
    pub snap_mode: crate::bindings::landscape::ELandscapeGizmoSnapType,
    pub b_smooth_gizmo_brush: bool,
    pub gizmo_heightmap_filename_string: FString,
    pub gizmo_import_size: crate::bindings::core_u_object::FIntPoint,
    pub gizmo_import_layers: TArray<FGizmoImportLayer>,
    pub mirror_point: crate::bindings::core_u_object::FVector2D,
    pub mirror_op: ELandscapeMirrorOperation,
    pub mirror_smoothing_width: i32,
    pub blueprint_brush: TSubclassOf<
        crate::bindings::landscape::ALandscapeBlueprintBrushBase,
    >,
    pub resize_landscape_quads_per_section: i32,
    pub resize_landscape_sections_per_component: i32,
    pub resize_landscape_component_count: crate::bindings::core_u_object::FIntPoint,
    pub resize_landscape_convert_mode: ELandscapeConvertMode,
    pub new_landscape_material: TWeakObjectPtr<
        crate::bindings::engine::UMaterialInterface,
    >,
    pub new_landscape_quads_per_section: i32,
    pub new_landscape_sections_per_component: i32,
    pub new_landscape_component_count: crate::bindings::core_u_object::FIntPoint,
    pub new_landscape_location: crate::bindings::core_u_object::FVector,
    pub new_landscape_rotation: crate::bindings::core_u_object::FRotator,
    pub new_landscape_scale: crate::bindings::core_u_object::FVector,
    pub import_landscape_heightmap_import_result: ELandscapeImportResult,
    pub import_landscape_heightmap_error_message: FText,
    pub import_landscape_heightmap_filename: FString,
    pub import_landscape_width: u32,
    pub import_landscape_height: u32,
    pub heightmap_export_filename: FString,
    pub import_landscape_gizmo_local_position: crate::bindings::core_u_object::FIntPoint,
    pub import_type: ELandscapeImportTransformType,
    pub b_heightmap_selected: bool,
    pub b_export_edit_layer: bool,
    pub b_export_single_file: bool,
    pub import_export_mode: ELandscapeImportExportMode,
    pub heightmap_import_descriptor: FLandscapeImportDescriptor,
    pub heightmap_import_descriptor_index: i32,
    pub import_landscape_data: TArray<u16>,
    pub b_can_have_layers_content_deprecated: bool,
    pub b_flip_y_axis: bool,
    pub world_partition_grid_size: u32,
    pub world_partition_region_size: u32,
    pub import_landscape_alphamap_type: crate::bindings::landscape::ELandscapeImportAlphamapType,
    pub import_landscape_layers: TArray<FLandscapeImportLayer>,
    pub new_landscape_layers: TArray<FLandscapeImportLayer>,
    pub brush_radius: f32,
    pub paint_brush_radius: f32,
    pub brush_falloff: f32,
    pub paint_brush_falloff: f32,
    pub b_use_clay_brush: bool,
    pub b_apply_without_moving_sculpt: bool,
    pub b_apply_without_moving_paint: bool,
    pub alpha_brush_scale: f32,
    pub b_alpha_brush_auto_rotate: bool,
    pub alpha_brush_rotation: f32,
    pub alpha_brush_pan_u: f32,
    pub alpha_brush_pan_v: f32,
    pub b_use_world_space_pattern_brush: bool,
    pub world_space_pattern_brush_settings: FLandscapePatternBrushWorldSpaceSettings,
    pub alpha_texture: UPtr<crate::bindings::engine::UTexture2D>,
    pub alpha_texture_channel: ELandscapeTextureColorChannel,
    pub alpha_texture_size_x: i32,
    pub alpha_texture_size_y: i32,
    pub alpha_texture_data: TArray<u8>,
    pub brush_component_size: i32,
    pub b_brush_component_include_border: bool,
    pub painting_restriction: crate::bindings::landscape::ELandscapeLayerPaintingRestriction,
    pub target_layer_asset_file_path: FLandscapeTargetLayerAssetFilePath,
    pub target_layers_filter_string: FString,
    pub target_display_order: crate::bindings::landscape::ELandscapeLayerDisplayMode,
    pub b_target_display_orders_ascending: bool,
    pub show_unused_layers: bool,
}
pub struct ULandscapeSplineSelection {
    pub selected_spline_control_points: TArray<
        UPtr<crate::bindings::landscape::ULandscapeSplineControlPoint>,
    >,
    pub selected_spline_segments: TArray<
        UPtr<crate::bindings::landscape::ULandscapeSplineSegment>,
    >,
    pub linear_control_points: TArray<
        UPtr<crate::bindings::landscape::ULandscapeSplineControlPoint>,
    >,
    pub linear_segments: TArray<
        UPtr<crate::bindings::landscape::ULandscapeSplineSegment>,
    >,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeImportResult(pub u8);
impl ELandscapeImportResult {
    pub const SUCCESS: ELandscapeImportResult = ELandscapeImportResult(0);
    pub const WARNING: ELandscapeImportResult = ELandscapeImportResult(1);
    pub const ERROR: ELandscapeImportResult = ELandscapeImportResult(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeToolFlattenMode(pub i8);
impl ELandscapeToolFlattenMode {
    pub const INVALID: ELandscapeToolFlattenMode = ELandscapeToolFlattenMode(-1);
    pub const BOTH: ELandscapeToolFlattenMode = ELandscapeToolFlattenMode(0);
    pub const RAISE: ELandscapeToolFlattenMode = ELandscapeToolFlattenMode(1);
    pub const LOWER: ELandscapeToolFlattenMode = ELandscapeToolFlattenMode(2);
    pub const INTERVAL: ELandscapeToolFlattenMode = ELandscapeToolFlattenMode(3);
    pub const TERRACE: ELandscapeToolFlattenMode = ELandscapeToolFlattenMode(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeToolErosionMode(pub i8);
impl ELandscapeToolErosionMode {
    pub const INVALID: ELandscapeToolErosionMode = ELandscapeToolErosionMode(-1);
    pub const BOTH: ELandscapeToolErosionMode = ELandscapeToolErosionMode(0);
    pub const RAISE: ELandscapeToolErosionMode = ELandscapeToolErosionMode(1);
    pub const LOWER: ELandscapeToolErosionMode = ELandscapeToolErosionMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeToolHydroErosionMode(pub i8);
impl ELandscapeToolHydroErosionMode {
    pub const INVALID: ELandscapeToolHydroErosionMode = ELandscapeToolHydroErosionMode(
        -1,
    );
    pub const BOTH: ELandscapeToolHydroErosionMode = ELandscapeToolHydroErosionMode(0);
    pub const POSITIVE: ELandscapeToolHydroErosionMode = ELandscapeToolHydroErosionMode(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeToolNoiseMode(pub i8);
impl ELandscapeToolNoiseMode {
    pub const INVALID: ELandscapeToolNoiseMode = ELandscapeToolNoiseMode(-1);
    pub const BOTH: ELandscapeToolNoiseMode = ELandscapeToolNoiseMode(0);
    pub const ADD: ELandscapeToolNoiseMode = ELandscapeToolNoiseMode(1);
    pub const SUB: ELandscapeToolNoiseMode = ELandscapeToolNoiseMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeToolPasteMode(pub i8);
impl ELandscapeToolPasteMode {
    pub const INVALID: ELandscapeToolPasteMode = ELandscapeToolPasteMode(-1);
    pub const BOTH: ELandscapeToolPasteMode = ELandscapeToolPasteMode(0);
    pub const RAISE: ELandscapeToolPasteMode = ELandscapeToolPasteMode(1);
    pub const LOWER: ELandscapeToolPasteMode = ELandscapeToolPasteMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeMirrorOperation(pub u8);
impl ELandscapeMirrorOperation {
    pub const MINUS_X_TO_PLUS_X: ELandscapeMirrorOperation = ELandscapeMirrorOperation(
        0,
    );
    pub const PLUS_X_TO_MINUS_X: ELandscapeMirrorOperation = ELandscapeMirrorOperation(
        1,
    );
    pub const MINUS_Y_TO_PLUS_Y: ELandscapeMirrorOperation = ELandscapeMirrorOperation(
        2,
    );
    pub const PLUS_Y_TO_MINUS_Y: ELandscapeMirrorOperation = ELandscapeMirrorOperation(
        3,
    );
    pub const ROTATE_MINUS_X_TO_PLUS_X: ELandscapeMirrorOperation = ELandscapeMirrorOperation(
        4,
    );
    pub const ROTATE_PLUS_X_TO_MINUS_X: ELandscapeMirrorOperation = ELandscapeMirrorOperation(
        5,
    );
    pub const ROTATE_MINUS_Y_TO_PLUS_Y: ELandscapeMirrorOperation = ELandscapeMirrorOperation(
        6,
    );
    pub const ROTATE_PLUS_Y_TO_MINUS_Y: ELandscapeMirrorOperation = ELandscapeMirrorOperation(
        7,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeConvertMode(pub i8);
impl ELandscapeConvertMode {
    pub const INVALID: ELandscapeConvertMode = ELandscapeConvertMode(-1);
    pub const EXPAND: ELandscapeConvertMode = ELandscapeConvertMode(0);
    pub const CLIP: ELandscapeConvertMode = ELandscapeConvertMode(1);
    pub const RESAMPLE: ELandscapeConvertMode = ELandscapeConvertMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeImportTransformType(pub i8);
impl ELandscapeImportTransformType {
    pub const NONE: ELandscapeImportTransformType = ELandscapeImportTransformType(0);
    pub const EXPAND_OFFSET: ELandscapeImportTransformType = ELandscapeImportTransformType(
        1,
    );
    pub const EXPAND_CENTERED: ELandscapeImportTransformType = ELandscapeImportTransformType(
        2,
    );
    pub const RESAMPLE: ELandscapeImportTransformType = ELandscapeImportTransformType(3);
    pub const SUBREGION: ELandscapeImportTransformType = ELandscapeImportTransformType(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeImportExportMode(pub i32);
impl ELandscapeImportExportMode {
    pub const LOADED_ONLY: ELandscapeImportExportMode = ELandscapeImportExportMode(0);
    pub const ALL: ELandscapeImportExportMode = ELandscapeImportExportMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeTextureColorChannel(pub i32);
impl ELandscapeTextureColorChannel {
    pub const RED: ELandscapeTextureColorChannel = ELandscapeTextureColorChannel(0);
    pub const GREEN: ELandscapeTextureColorChannel = ELandscapeTextureColorChannel(1);
    pub const BLUE: ELandscapeTextureColorChannel = ELandscapeTextureColorChannel(2);
    pub const ALPHA: ELandscapeTextureColorChannel = ELandscapeTextureColorChannel(3);
}
