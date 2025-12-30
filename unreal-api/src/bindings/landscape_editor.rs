#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
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
    pub thumbnail_mic: UPtr<ULandscapeMaterialInstanceConstant>,
    pub import_result: ELandscapeImportResult,
    pub error_message: FText,
    pub export_file_path: FString,
    pub b_selected: bool,
}
#[repr(C, align(8))]
pub struct FLandscapePatternBrushWorldSpaceSettings {
    pub origin: FVector2D,
    pub rotation: f32,
    pub b_center_texture_on_origin: bool,
    pub repeat_size: f32,
}
#[repr(C, align(8))]
pub struct FLandscapeTargetLayerAssetFilePath {
    pub b_use_asset_directory_path: bool,
    pub directory_path: FDirectoryPath,
}
#[repr(C, align(8))]
pub struct FLandscapeImportFileDescriptor {
    pub coord: FIntPoint,
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
    pub scale: FVector,
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
    pub current_gizmo_actor: TWeakObjectPtr<ALandscapeGizmoActiveActor>,
    pub snap_mode: ELandscapeGizmoSnapType,
    pub b_smooth_gizmo_brush: bool,
    pub gizmo_heightmap_filename_string: FString,
    pub gizmo_import_size: FIntPoint,
    pub gizmo_import_layers: TArray<FGizmoImportLayer>,
    pub mirror_point: FVector2D,
    pub mirror_op: ELandscapeMirrorOperation,
    pub mirror_smoothing_width: i32,
    pub blueprint_brush: TSubclassOf<ALandscapeBlueprintBrushBase>,
    pub resize_landscape_quads_per_section: i32,
    pub resize_landscape_sections_per_component: i32,
    pub resize_landscape_component_count: FIntPoint,
    pub resize_landscape_convert_mode: ELandscapeConvertMode,
    pub new_landscape_material: TWeakObjectPtr<UMaterialInterface>,
    pub new_landscape_quads_per_section: i32,
    pub new_landscape_sections_per_component: i32,
    pub new_landscape_component_count: FIntPoint,
    pub new_landscape_location: FVector,
    pub new_landscape_rotation: FRotator,
    pub new_landscape_scale: FVector,
    pub import_landscape_heightmap_import_result: ELandscapeImportResult,
    pub import_landscape_heightmap_error_message: FText,
    pub import_landscape_heightmap_filename: FString,
    pub import_landscape_width: u32,
    pub import_landscape_height: u32,
    pub heightmap_export_filename: FString,
    pub import_landscape_gizmo_local_position: FIntPoint,
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
    pub import_landscape_alphamap_type: ELandscapeImportAlphamapType,
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
    pub alpha_texture: UPtr<UTexture2D>,
    pub alpha_texture_channel: ELandscapeTextureColorChannel,
    pub alpha_texture_size_x: i32,
    pub alpha_texture_size_y: i32,
    pub alpha_texture_data: TArray<u8>,
    pub brush_component_size: i32,
    pub b_brush_component_include_border: bool,
    pub painting_restriction: ELandscapeLayerPaintingRestriction,
    pub target_layer_asset_file_path: FLandscapeTargetLayerAssetFilePath,
    pub target_layers_filter_string: FString,
    pub target_display_order: ELandscapeLayerDisplayMode,
    pub b_target_display_orders_ascending: bool,
    pub show_unused_layers: bool,
}
pub struct ULandscapeSplineSelection {
    pub selected_spline_control_points: TArray<UPtr<ULandscapeSplineControlPoint>>,
    pub selected_spline_segments: TArray<UPtr<ULandscapeSplineSegment>>,
    pub linear_control_points: TArray<UPtr<ULandscapeSplineControlPoint>>,
    pub linear_segments: TArray<UPtr<ULandscapeSplineSegment>>,
}
