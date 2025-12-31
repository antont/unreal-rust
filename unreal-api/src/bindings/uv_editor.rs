#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FUDIMSpecifier {
    pub udim: i32,
    pub u_coord: i32,
    pub v_coord: i32,
    pub texture_resolution: i32,
}
pub struct UUVEditorInitializationContext {}
pub struct UUVEditorUnwrappedUXProperties {
    pub boundary_line_colors: TArray<crate::bindings::core_u_object::FColor>,
    pub boundary_line_thickness: f32,
    pub wireframe_thickness: f32,
}
pub struct UUVEditorLivePreviewUXProperties {
    pub selection_color: crate::bindings::core_u_object::FColor,
    pub selection_line_thickness: f32,
    pub selection_point_size: f32,
}
pub struct UUnsetUVsAction {}
pub struct UUVEditor {
    pub original_objects_to_edit: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub object_worldspace_offsets: TArray<crate::bindings::core_u_object::FTransform3d>,
}
pub struct UUVEditor2DViewportContext {}
pub struct UUVEditor3DViewportMode {}
pub struct UUVEditorBackgroundPreviewProperties {
    pub b_visible: bool,
    pub source_type: EUVEditorBackgroundSourceType,
    pub source_texture: UPtr<crate::bindings::engine::UTexture2D>,
    pub source_material: UPtr<crate::bindings::engine::UMaterial>,
    pub udim_blocks: TArray<i32>,
    pub b_udi_ms_enabled: bool,
}
pub struct UUVEditorBackgroundPreview {
    pub settings: UPtr<UUVEditorBackgroundPreviewProperties>,
    pub background_component: UPtr<
        crate::bindings::modeling_components::UTriangleSetComponent,
    >,
    pub background_material: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
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
    pub targets: TArray<UPtr<crate::bindings::uv_editor_tools::UUVEditorToolMeshInput>>,
}
pub struct UUVEditorGridProperties {
    pub b_draw_grid: bool,
    pub b_draw_rulers: bool,
}
pub struct UUVEditorUDIMProperties {
    pub udim_source_asset: FString,
    pub udim_source_texture: UPtr<crate::bindings::engine::UTexture2D>,
    pub active_udi_ms: TArray<FUDIMSpecifier>,
}
pub struct UUVEditorMode {
    pub background_visualization: UPtr<UUVEditorBackgroundPreview>,
    pub distortion_visualization: UPtr<UUVEditorDistortionVisualization>,
    pub uv_editor_grid_properties: UPtr<UUVEditorGridProperties>,
    pub uv_editor_udim_properties: UPtr<UUVEditorUDIMProperties>,
    pub registered_actions: TArray<
        UPtr<crate::bindings::uv_editor_tools::UUVToolAction>,
    >,
    pub original_objects_to_edit: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub tool_targets: TArray<
        UPtr<crate::bindings::interactive_tools_framework::UToolTarget>,
    >,
    pub tool_input_objects: TArray<
        UPtr<crate::bindings::uv_editor_tools::UUVEditorToolMeshInput>,
    >,
    pub live_preview_world: UPtr<crate::bindings::engine::UWorld>,
    pub selection_api: UPtr<crate::bindings::uv_editor_tools::UUVToolSelectionAPI>,
    pub property_objects_to_tick: TArray<
        UPtr<crate::bindings::interactive_tools_framework::UInteractiveToolPropertySet>,
    >,
    pub uv_editor_unwrapped_ux_properties: UPtr<UUVEditorUnwrappedUXProperties>,
    pub uv_editor_live_preview_ux_properties: UPtr<UUVEditorLivePreviewUXProperties>,
}
pub struct UUVEditorUVChannelProperties {
    pub asset: FString,
    pub uv_channel: FString,
}
pub struct UUVEditorUISubsystem {}
pub struct UUVEditorSubsystem {
    pub tool_target_manager: UPtr<
        crate::bindings::interactive_tools_framework::UToolTargetManager,
    >,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EUVEditorBackgroundSourceType(pub i32);
impl EUVEditorBackgroundSourceType {
    pub const CHECKERBOARD: EUVEditorBackgroundSourceType = EUVEditorBackgroundSourceType(
        0,
    );
    pub const TEXTURE: EUVEditorBackgroundSourceType = EUVEditorBackgroundSourceType(1);
    pub const MATERIAL: EUVEditorBackgroundSourceType = EUVEditorBackgroundSourceType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDistortionMetric(pub u8);
impl EDistortionMetric {
    pub const REED_BETA: EDistortionMetric = EDistortionMetric(0);
    pub const SANDER_L2: EDistortionMetric = EDistortionMetric(1);
    pub const SANDER_L_INF: EDistortionMetric = EDistortionMetric(2);
    pub const TEXEL_DENSITY: EDistortionMetric = EDistortionMetric(3);
}
