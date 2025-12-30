#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FUDIMBlock {
    pub udim: i32,
    pub texture_resolution: i32,
}
pub struct UUVEditorMechanicAdapterTool {}
pub struct UUVToolContextObject {}
pub struct UUVToolAssetInputsContext {}
pub struct UUVToolViewportButtonsAPI {}
pub struct UUVUnwrapDynamicMesh {}
pub struct IVUnwrapDynamicMesh {}
pub struct UUVToolAction {
    pub selection_api: UPtr<UUVToolSelectionAPI>,
    pub emit_change_api: UPtr<UUVToolEmitChangeAPI>,
}
pub struct UUVMakeIslandAction {}
pub struct UUVSeamSewAction {}
pub struct UUVSplitAction {}
pub struct UUVToolEmitChangeAPI {}
pub struct UUVToolLivePreviewAPI {
    pub world: TWeakObjectPtr<UWorld>,
    pub input_router: TWeakObjectPtr<UInputRouter>,
}
pub struct UUVTool2DViewportAPI {
    pub udim_blocks: TArray<FUDIMBlock>,
    pub b_draw_grid: bool,
    pub b_draw_rulers: bool,
}
pub struct UUVToolAssetAndChannelAPI {}
pub struct UUVToolAABBTreeStorage {}
pub struct UUVEditorToolPropertiesAPI {
    pub tool_display_properties: UPtr<UObject>,
}
pub struct UBasicLineSetComponentBase {
    pub line_material: UPtr<UMaterialInterface>,
    pub bounds: FBoxSphereBounds,
    pub b_bounds_dirty: bool,
    pub color: FColor,
    pub size: f32,
    pub depth_bias: f32,
}
pub struct UBasic2DLineSetComponent {}
pub struct UBasic3DLineSetComponent {}
pub struct UBasicPointSetComponentBase {
    pub point_material: UPtr<UMaterialInterface>,
    pub bounds: FBoxSphereBounds,
    pub b_bounds_dirty: bool,
    pub color: FColor,
    pub size: f32,
    pub depth_bias: f32,
}
pub struct UBasic2DPointSetComponent {}
pub struct UBasic3DPointSetComponent {}
pub struct UBasicTriangleSetComponentBase {
    pub triangle_material: UPtr<UMaterialInterface>,
    pub bounds: FBoxSphereBounds,
    pub b_bounds_dirty: bool,
    pub color: FColor,
    pub normal: FVector3f,
}
pub struct UBasic2DTriangleSetComponent {}
pub struct UBasic3DTriangleSetComponent {}
pub struct UUVEditorUVTransformPropertiesBase {}
pub struct UUVEditorUVTransformProperties {
    pub scale: FVector2D,
    pub rotation: f32,
    pub translation: FVector2D,
    pub translation_mode: EUVEditorTranslationMode,
    pub pivot_mode: EUVEditorPivotType,
    pub manual_pivot: FVector2D,
    pub quick_translate_offset: f32,
    pub quick_rotation_offset: f32,
    pub quick_translation: FVector2D,
    pub quick_rotation: f32,
}
pub struct UUVEditorUVQuickTransformProperties {}
pub struct UUVEditorUVAlignProperties {
    pub align_anchor: EUVEditorAlignAnchor,
    pub manual_anchor: FVector2D,
    pub align_direction: EUVEditorAlignDirection,
    pub grouping: EUVEditorAlignDistributeGroupingMode,
}
pub struct UUVEditorUVDistributeProperties {
    pub distribute_mode: EUVEditorDistributeMode,
    pub grouping: EUVEditorAlignDistributeGroupingMode,
    pub b_enable_manual_distances: bool,
    pub manual_extent: f32,
    pub manual_spacing: f32,
}
pub struct UUVEditorUVTransformOperatorFactory {
    pub settings: UPtr<UUVEditorUVTransformPropertiesBase>,
}
pub struct UUVEditorMeshSelectionMechanic {
    pub selection_api: UPtr<UUVToolSelectionAPI>,
    pub viewport_buttons_api: UPtr<UUVToolViewportButtonsAPI>,
    pub emit_change_api: UPtr<UUVToolEmitChangeAPI>,
    pub live_preview_api: UPtr<UUVToolLivePreviewAPI>,
    pub unwrap_click_target_router: UPtr<ULocalSingleClickInputBehavior>,
    pub live_preview_click_target_router: UPtr<ULocalSingleClickInputBehavior>,
    pub unwrap_hover_behavior_target_router: UPtr<ULocalMouseHoverBehavior>,
    pub live_preview_hover_behavior_target_router: UPtr<ULocalMouseHoverBehavior>,
    pub marquee_mechanic: UPtr<URectangleMarqueeMechanic>,
    pub live_preview_marquee_mechanic: UPtr<URectangleMarqueeMechanic>,
    pub hover_triangle_set_material: UPtr<UMaterialInstanceDynamic>,
    pub hover_geometry_actor: UPtr<APreviewGeometryActor>,
    pub live_preview_behavior_set: UPtr<UInputBehaviorSet>,
    pub live_preview_behavior_source: UPtr<ULocalInputBehaviorSource>,
    pub live_preview_hover_geometry_actor: UPtr<APreviewGeometryActor>,
}
pub struct UUVToolSelectionAPI {
    pub targets: TArray<UPtr<UUVEditorToolMeshInput>>,
    pub mechanic_adapter: UPtr<UUVEditorMechanicAdapterTool>,
    pub highlight_mechanic: UPtr<UUVToolSelectionHighlightMechanic>,
    pub selection_mechanic: UPtr<UUVEditorMeshSelectionMechanic>,
    pub emit_change_api: UPtr<UUVToolEmitChangeAPI>,
}
pub struct UUVToolSupportsSelection {}
pub struct IVToolSupportsSelection {}
pub struct UUVToolSelectionHighlightMechanic {
    pub unwrap_geometry_actor: UPtr<APreviewGeometryActor>,
    pub triangle_set_material: UPtr<UMaterialInstanceDynamic>,
    pub unwrap_stationary_geometry_actor: UPtr<APreviewGeometryActor>,
    pub live_preview_geometry_actor: UPtr<APreviewGeometryActor>,
}
pub struct UUVEditorToolMeshInput {
    pub unwrap_preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub applied_preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub wireframe_display: UPtr<UMeshElementsVisualizer>,
    pub b_enable_triangle_vertex_colors: bool,
}
pub struct UUVEditorBrushSelectToolProperties {
    pub b_clear_selection_on_each_drag: bool,
    pub b_expand_to_islands: bool,
    pub unwrap_brush_radius: f32,
    pub live_preview_brush_radius: f32,
}
pub struct UUVEditorBrushSelectTool {
    pub targets: TArray<UPtr<UUVEditorToolMeshInput>>,
    pub emit_change_api: UPtr<UUVToolEmitChangeAPI>,
    pub selection_api: UPtr<UUVToolSelectionAPI>,
    pub live_preview_api: UPtr<UUVToolLivePreviewAPI>,
    pub live_preview_behavior_set: UPtr<UInputBehaviorSet>,
    pub live_preview_behavior_source: UPtr<ULocalInputBehaviorSource>,
    pub settings: UPtr<UUVEditorBrushSelectToolProperties>,
    pub unwrap_brush_indicator: UPtr<UBrushStampIndicator>,
    pub live_preview_brush_indicator: UPtr<UBrushStampIndicator>,
}
pub struct UUVEditorChannelEditToolBuilder {}
pub struct UUVEditorChannelEditSettings {
    pub action: EChannelEditToolAction,
}
pub struct UUVEditorChannelEditTargetProperties {
    pub asset: FString,
    pub target_channel: FString,
    pub reference_channel: FString,
    pub b_action_needs_asset: bool,
    pub b_action_needs_reference: bool,
    pub b_action_needs_target: bool,
}
pub struct UUVEditorChannelEditAddProperties {}
pub struct UUVEditorChannelEditCopyProperties {}
pub struct UUVEditorChannelEditDeleteProperties {}
pub struct UUVEditorChannelEditToolActionPropertySet {}
pub struct UUVEditorChannelEditTool {
    pub targets: TArray<UPtr<UUVEditorToolMeshInput>>,
    pub tool_actions: UPtr<UUVEditorChannelEditToolActionPropertySet>,
    pub action_selection_properties: UPtr<UUVEditorChannelEditSettings>,
    pub source_channel_properties: UPtr<UUVEditorChannelEditTargetProperties>,
    pub add_action_properties: UPtr<UUVEditorChannelEditAddProperties>,
    pub copy_action_properties: UPtr<UUVEditorChannelEditCopyProperties>,
    pub delete_action_properties: UPtr<UUVEditorChannelEditDeleteProperties>,
    pub emit_change_api: UPtr<UUVToolEmitChangeAPI>,
}
pub struct UUVEditorLayoutToolBuilder {}
pub struct UUVEditorLayoutTool {
    pub targets: TArray<UPtr<UUVEditorToolMeshInput>>,
    pub settings: UPtr<UUVLayoutProperties>,
    pub factories: TArray<UPtr<UUVLayoutOperatorFactory>>,
    pub uv_tool_selection_api: UPtr<UUVToolSelectionAPI>,
    pub uv_tool2_d_viewport_api: UPtr<UUVTool2DViewportAPI>,
}
pub struct UUVEditorRecomputeUVsToolBuilder {}
pub struct UUVEditorRecomputeUVsTool {
    pub targets: TArray<UPtr<UUVEditorToolMeshInput>>,
    pub settings: UPtr<URecomputeUVsToolProperties>,
    pub polygroup_layer_properties: UPtr<UPolygroupLayersProperties>,
    pub factories: TArray<UPtr<URecomputeUVsOpFactory>>,
    pub uv_tool_selection_api: UPtr<UUVToolSelectionAPI>,
}
pub struct UUVEditorSeamToolProperties {
    pub mode: EUVEditorSeamMode,
    pub path_similarity_weight: f64,
}
pub struct UUVEditorSeamToolBuilder {}
pub struct UUVEditorSeamTool {
    pub targets: TArray<UPtr<UUVEditorToolMeshInput>>,
    pub settings: UPtr<UUVEditorSeamToolProperties>,
    pub live_preview_api: UPtr<UUVToolLivePreviewAPI>,
    pub emit_change_api: UPtr<UUVToolEmitChangeAPI>,
    pub live_preview_behavior_set: UPtr<UInputBehaviorSet>,
    pub live_preview_behavior_source: UPtr<ULocalInputBehaviorSource>,
    pub unwrap_geometry: UPtr<UPreviewGeometry>,
    pub live_preview_geometry: UPtr<UPreviewGeometry>,
}
pub struct UUVEditorTexelDensityToolBuilder {}
pub struct UUVEditorTexelDensityActionSettings {}
pub struct UUVEditorTexelDensityToolSettings {}
pub struct UUVEditorTexelDensityTool {
    pub targets: TArray<UPtr<UUVEditorToolMeshInput>>,
    pub settings: UPtr<UUVEditorTexelDensitySettings>,
    pub action_settings: UPtr<UUVEditorTexelDensityActionSettings>,
    pub uv_tool_selection_api: UPtr<UUVToolSelectionAPI>,
    pub live_preview_api: UPtr<UUVToolLivePreviewAPI>,
    pub uv_tool2_d_viewport_api: UPtr<UUVTool2DViewportAPI>,
    pub emit_change_api: UPtr<UUVToolEmitChangeAPI>,
    pub factories: TArray<UPtr<UUVTexelDensityOperatorFactory>>,
    pub live_preview_behavior_set: UPtr<UInputBehaviorSet>,
    pub live_preview_behavior_source: UPtr<ULocalInputBehaviorSource>,
    pub unwrap_geometry: UPtr<UPreviewGeometry>,
    pub live_preview_geometry: UPtr<UPreviewGeometry>,
    pub triangle_set_material: UPtr<UMaterialInstanceDynamic>,
}
pub struct UUVEditorGenericBuildableTool {}
pub struct IVEditorGenericBuildableTool {}
pub struct UGenericUVEditorToolBuilder {}
pub struct UUVEditorTransformToolDisplayProperties {
    pub b_draw_pivots: bool,
}
pub struct UUVEditorBaseTransformToolBuilder {}
pub struct UUVEditorTransformToolBuilder {}
pub struct UUVEditorAlignToolBuilder {}
pub struct UUVEditorDistributeToolBuilder {}
pub struct UUVEditorTransformTool {
    pub targets: TArray<UPtr<UUVEditorToolMeshInput>>,
    pub settings: UPtr<UUVEditorUVTransformPropertiesBase>,
    pub display_settings: UPtr<UUVEditorTransformToolDisplayProperties>,
    pub factories: TArray<UPtr<UUVEditorUVTransformOperatorFactory>>,
    pub uv_tool_selection_api: UPtr<UUVToolSelectionAPI>,
}
pub struct UUVEditorUVSnapshotToolBuilder {}
pub struct UUVEditorUVSnapshotTool {
    pub target: UPtr<UUVEditorToolMeshInput>,
    pub uv_shell_settings: UPtr<UUVEditorBakeUVShellProperties>,
    pub preview_geo_background_quad: UPtr<UPreviewGeometry>,
    pub cached_uv_map: UPtr<UTexture2D>,
}
pub struct UUVEditorBakeUVShellProperties {
    pub uv_layer: FString,
    pub wireframe_thickness: f32,
    pub wireframe_color: FLinearColor,
    pub shell_color: FLinearColor,
    pub background_color: FLinearColor,
    pub resolution: EBakeTextureResolution,
    pub samples_per_pixel: EBakeTextureSamplesPerPixel,
    pub saved_path: FString,
    pub result: UPtr<UTexture2D>,
    pub target_uv_layer_names_list: TArray<FString>,
}
pub struct UUVSelectToolBuilder {}
pub struct UUVSelectTool {
    pub targets: TArray<UPtr<UUVEditorToolMeshInput>>,
    pub viewport_buttons_api: UPtr<UUVToolViewportButtonsAPI>,
    pub emit_change_api: UPtr<UUVToolEmitChangeAPI>,
    pub selection_api: UPtr<UUVToolSelectionAPI>,
    pub selection_mechanic: UPtr<UUVEditorMeshSelectionMechanic>,
    pub transform_gizmo: UPtr<UCombinedTransformGizmo>,
}
