#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FUDIMSpecifier {
    pub udim: i32,
    pub u_coord: i32,
    pub v_coord: i32,
    pub texture_resolution: i32,
}
pub struct UUVEditorInitializationContext {}
pub struct UUVEditorUnwrappedUXProperties {
    pub boundary_line_colors: TArray<FColor>,
    pub boundary_line_thickness: f32,
    pub wireframe_thickness: f32,
}
pub struct UUVEditorLivePreviewUXProperties {
    pub selection_color: FColor,
    pub selection_line_thickness: f32,
    pub selection_point_size: f32,
}
pub struct UUnsetUVsAction {}
pub struct UUVEditor {
    pub original_objects_to_edit: TArray<UPtr<UObject>>,
    pub object_worldspace_offsets: TArray<FTransform3d>,
}
pub struct UUVEditor2DViewportContext {}
pub struct UUVEditor3DViewportMode {}
pub struct UUVEditorBackgroundPreviewProperties {
    pub b_visible: bool,
    pub source_type: EUVEditorBackgroundSourceType,
    pub source_texture: UPtr<UTexture2D>,
    pub source_material: UPtr<UMaterial>,
    pub udim_blocks: TArray<i32>,
    pub b_udi_ms_enabled: bool,
}
pub struct UUVEditorBackgroundPreview {
    pub settings: UPtr<UUVEditorBackgroundPreviewProperties>,
    pub background_component: UPtr<UTriangleSetComponent>,
    pub background_material: UPtr<UMaterialInstanceDynamic>,
}
pub struct UUVEditorDistortionVisualizationProperties {
    pub b_visible: bool,
    pub metric: EDistortionMetric,
    pub b_compare_to_average_density: bool,
    pub b_respect_udim_texture_resolutions: bool,
    pub map_size: i32,
    pub target_texel_density: f32,
    pub per_udim_texture_resolution: TMap<i32, i32>,
}
pub struct UUVEditorDistortionVisualization {
    pub settings: UPtr<UUVEditorDistortionVisualizationProperties>,
    pub targets: TArray<UPtr<UUVEditorToolMeshInput>>,
}
pub struct UUVEditorGridProperties {
    pub b_draw_grid: bool,
    pub b_draw_rulers: bool,
}
pub struct UUVEditorUDIMProperties {
    pub udim_source_asset: FString,
    pub udim_source_texture: UPtr<UTexture2D>,
    pub active_udi_ms: TArray<FUDIMSpecifier>,
}
pub struct UUVEditorMode {
    pub background_visualization: UPtr<UUVEditorBackgroundPreview>,
    pub distortion_visualization: UPtr<UUVEditorDistortionVisualization>,
    pub uv_editor_grid_properties: UPtr<UUVEditorGridProperties>,
    pub uv_editor_udim_properties: UPtr<UUVEditorUDIMProperties>,
    pub registered_actions: TArray<UPtr<UUVToolAction>>,
    pub original_objects_to_edit: TArray<UPtr<UObject>>,
    pub tool_targets: TArray<UPtr<UToolTarget>>,
    pub tool_input_objects: TArray<UPtr<UUVEditorToolMeshInput>>,
    pub live_preview_world: UPtr<UWorld>,
    pub selection_api: UPtr<UUVToolSelectionAPI>,
    pub property_objects_to_tick: TArray<UPtr<UInteractiveToolPropertySet>>,
    pub uv_editor_unwrapped_ux_properties: UPtr<UUVEditorUnwrappedUXProperties>,
    pub uv_editor_live_preview_ux_properties: UPtr<UUVEditorLivePreviewUXProperties>,
}
pub struct UUVEditorUVChannelProperties {
    pub asset: FString,
    pub uv_channel: FString,
}
pub struct UUVEditorUISubsystem {}
pub struct UUVEditorSubsystem {
    pub tool_target_manager: UPtr<UToolTargetManager>,
}
