#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FDataflowCorrectSkinWeightsNode {
    pub collection: FManagedArrayCollection,
    pub bone_indices_name: FString,
    pub bone_weights_name: FString,
    pub selection_map_name: FString,
    pub vertex_group: FScalarVertexPropertyGroup,
    pub bone_indices_key: FCollectionAttributeKey,
    pub bone_weights_key: FCollectionAttributeKey,
    pub selection_map_key: FCollectionAttributeKey,
    pub correction_type: ESkinWeightsCorrectionType,
    pub smoothing_iterations: i32,
    pub smoothing_factor: f32,
    pub b_use_selection_as_per_vertex_factor: bool,
    pub pruning_threshold: f32,
    pub clamping_number: i32,
    pub selection_threshold: f32,
}
#[repr(C, align(8))]
pub struct FDataflowSetSkinningSelectionNode {
    pub collection: FManagedArrayCollection,
    pub selection_map_name: FString,
    pub vertex_group: FScalarVertexPropertyGroup,
    pub selection_map_key: FCollectionAttributeKey,
    pub correction_type: ESkinWeightsCorrectionType,
}
#[repr(C, align(8))]
pub struct FDataflowGetSkinningSelectionNode {
    pub collection: FManagedArrayCollection,
    pub vertex_group: FScalarVertexPropertyGroup,
    pub selection_map_key: FCollectionAttributeKey,
    pub correction_type: ESkinWeightsCorrectionType,
}
#[repr(C, align(8))]
pub struct FDataflowEditorVertexAttributePaintToolBrushProperties {
    pub brush_mode: EDataflowEditorToolEditOperation,
    pub brush_size: f32,
    pub attribute_value: f32,
    pub brush_area_mode: EMeshVertexPaintBrushAreaType,
    pub angle_threshold: f32,
    pub b_uv_seams: bool,
    pub b_normal_seams: bool,
    pub visibility_filter: EDataflowEditorToolVisibilityType,
    pub value_at_brush: f64,
}
#[repr(C, align(4))]
pub struct FDataflowEditorVertexAttributePaintToolBrushConfig {
    pub brush_size: f32,
    pub value: f32,
}
#[repr(C, align(8))]
pub struct FDataflowEditorVertexAttributePaintToolGradientProperties {
    pub gradient_high_value: f64,
    pub gradient_low_value: f64,
}
#[repr(C, align(1))]
pub struct FDataflowEditorVertexAttributePaintToolSelectionProperties {
    pub component_selection_mode: EComponentSelectionMode,
}
#[repr(C, align(8))]
pub struct FDataflowEditorVertexAttributePaintToolDisplayProperties {
    pub color_mode: EDataflowEditorToolColorMode,
    pub color_ramp: FDataflowColorRamp,
}
#[repr(C, align(1))]
pub struct FDataflowEditorVertexAttributePaintToolMirrorProperties {
    pub mirror_axis: EAxis,
    pub mirror_direction: EDataflowEditorToolMirrorDirection,
}
#[repr(C, align(8))]
pub struct FDataflowNodeData {
    pub ty: FString,
    pub name: FString,
    pub properties: FString,
    pub position: FVector2D,
}
#[repr(C, align(8))]
pub struct FDataflowCommentNodeData {
    pub name: FString,
    pub size: FVector2D,
    pub color: FLinearColor,
    pub position: FVector2D,
    pub font_size: i32,
}
#[repr(C, align(8))]
pub struct FDataflowConnectionData {
    pub out: FString,
    pub in_: FString,
}
#[repr(C, align(8))]
pub struct FDataflowCopyPasteContent {
    pub node_data: TArray<FDataflowNodeData>,
    pub comment_node_data: TArray<FDataflowCommentNodeData>,
    pub connection_data: TArray<FDataflowConnectionData>,
}
#[repr(C, align(1))]
pub struct FDataflowConstructionObjectTag {}
#[repr(C, align(1))]
pub struct FDataflowSimulationObjectTag {}
#[repr(C, align(1))]
pub struct FDataflowSceneObjectTag {}
#[repr(C, align(1))]
pub struct FDataflowSceneStructTag {}
#[repr(C, align(4))]
pub struct FDataflowSceneTypeColumn {}
#[repr(C, align(8))]
pub struct FAssetSchemaAction_Dataflow_CreateNode_DataflowEdNode {}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_DataflowVariable {}
#[repr(C, align(8))]
pub struct FEdGraphSchemaAction_DataflowSubGraph {}
#[repr(C, align(8))]
pub struct FAssetSchemaAction_Dataflow_CreateCommentNode_DataflowEdNode {}
pub struct UDataflowEditorToolBuilder {}
pub struct IDataflowEditorToolBuilder {}
pub struct UDataflowEditorSettings {}
pub struct UDataflowEvaluationSettings {
    pub b_allow_evaluation_in_pie: bool,
}
pub struct UDataflowBoneManipulator {
    pub transform_proxy: UPtr<UTransformProxy>,
    pub transform_gizmo: UPtr<UCombinedTransformGizmo>,
    pub ref_skeleton_poser: UPtr<URefSkeletonPoser>,
    pub bone_index: i32,
}
pub struct UDataflowEditorEditSkeletonBonesToolBuilder {}
pub struct UDataflowEditorEditSkeletonBonesTool {}
pub struct UDataflowTransformGizmoSource {}
pub struct UDataflowEditorSkinWeightsPaintToolBuilder {}
pub struct UDataflowEditorSkinWeightsPaintTool {}
pub struct UDataflowEditorVertexAttributePaintToolBuilder {
    pub fallback_tool_builder: UPtr<UMeshSurfacePointMeshEditingToolBuilder>,
}
pub struct UDataflowEditorVertexAttributePaintToolProperties {
    pub editing_mode: EDataflowEditorToolEditMode,
    pub brush_properties: FDataflowEditorVertexAttributePaintToolBrushProperties,
    pub gradient_properties: FDataflowEditorVertexAttributePaintToolGradientProperties,
    pub selection_properties: FDataflowEditorVertexAttributePaintToolSelectionProperties,
    pub display_properties: FDataflowEditorVertexAttributePaintToolDisplayProperties,
    pub mirror_properties: FDataflowEditorVertexAttributePaintToolMirrorProperties,
    pub brush_config_add: FDataflowEditorVertexAttributePaintToolBrushConfig,
    pub brush_config_replace: FDataflowEditorVertexAttributePaintToolBrushConfig,
    pub brush_config_multiply: FDataflowEditorVertexAttributePaintToolBrushConfig,
    pub brush_config_relax: FDataflowEditorVertexAttributePaintToolBrushConfig,
}
pub struct UDataflowEditorVertexAttributePaintTool {
    pub tool_properties: UPtr<UDataflowEditorVertexAttributePaintToolProperties>,
    pub paint_brush_op_properties: UPtr<UDataflowVertexAttributePaintBrushOpProps>,
    pub smooth_brush_op_properties: UPtr<UDataflowWeightMapSmoothBrushOpProps>,
    pub poly_lasso_mechanic: UPtr<UPolyLassoMarqueeMechanic>,
    pub mesh_elements_display: UPtr<UMeshElementsVisualizer>,
    pub dataflow_editor_context_object: UPtr<UDataflowContextObject>,
    pub preview_mesh: UPtr<UPreviewMesh>,
    pub mesh_selector: UPtr<UToolMeshSelector>,
}
pub struct UDataflowWeightMapEraseBrushOpProps {
    pub attribute_value: f64,
}
pub struct UDataflowWeightMapPaintBrushOpProps {
    pub attribute_value: f64,
    pub strength: f32,
}
pub struct UDataflowWeightMapSmoothBrushOpProps {
    pub strength: f32,
}
pub struct UDataflowVertexAttributePaintBrushOpProps {
    pub attribute_value: f64,
    pub strength: f32,
}
pub struct UDataflowEditorWeightMapPaintToolBuilder {}
pub struct UDataflowEditorWeightMapPaintBrushFilterProperties {
    pub sub_tool_type: EDataflowEditorWeightMapPaintInteractionType,
    pub primary_brush_type: EDataflowEditorWeightMapPaintBrushType,
    pub brush_size: f32,
    pub attribute_value: f64,
    pub strength: f64,
    pub gradient_high_value: f64,
    pub gradient_low_value: f64,
    pub brush_area_mode: EMeshVertexPaintBrushAreaType,
    pub angle_threshold: f32,
    pub b_uv_seams: bool,
    pub b_normal_seams: bool,
    pub visibility_filter: EDataflowEditorWeightMapPaintVisibilityType,
    pub value_at_brush: f64,
}
pub struct UDataflowEditorMeshWeightMapPaintToolActions {}
pub struct UDataflowEditorUpdateWeightMapProperties {
    pub name: FString,
}
pub struct UDataflowEditorWeightMapPaintTool {
    pub filter_properties: UPtr<UDataflowEditorWeightMapPaintBrushFilterProperties>,
    pub paint_brush_op_properties: UPtr<UDataflowWeightMapPaintBrushOpProps>,
    pub smooth_brush_op_properties: UPtr<UDataflowWeightMapSmoothBrushOpProps>,
    pub erase_brush_op_properties: UPtr<UDataflowWeightMapEraseBrushOpProps>,
    pub actions_props: UPtr<UDataflowEditorMeshWeightMapPaintToolActions>,
    pub update_weight_map_properties: UPtr<UDataflowEditorUpdateWeightMapProperties>,
    pub poly_lasso_mechanic: UPtr<UPolyLassoMarqueeMechanic>,
    pub polygon_selection_mechanic: UPtr<UPolygonSelectionMechanic>,
    pub preview_mesh_actor: UPtr<AInternalToolFrameworkActor>,
    pub dynamic_mesh_component: UPtr<UDynamicMeshComponent>,
    pub mesh_elements_display: UPtr<UMeshElementsVisualizer>,
    pub dataflow_editor_context_object: UPtr<UDataflowContextObject>,
}
pub struct UAssetDefinition_DataflowAsset {}
pub struct UAssetDefinition_DataflowContext {}
pub struct UDataflowAssetFactory {}
pub struct UDataflowComponentReadOnlyToolTarget {}
pub struct UDataflowComponentToolTarget {}
pub struct UDataflowComponentReadOnlyToolTargetFactory {}
pub struct UDataflowComponentToolTargetFactory {}
pub struct UDataflowEditor {
    pub editor_content: UPtr<UDataflowBaseContent>,
    pub terminal_contents: TArray<UPtr<UDataflowBaseContent>>,
    pub editor_settings: TArray<UPtr<UDataflowEditorSettings>>,
}
pub struct UDataflowEditorBlueprintLibrary {}
pub struct UDataflowEditorCollectionComponent {
    pub mesh_index: i32,
    pub node: UPtr<UDataflowEdNode>,
    pub wireframe_component: UPtr<UMeshWireframeComponent>,
}
pub struct UDataflowEditorMode {
    pub active_tools_context: UPtr<UEditorInteractiveToolsContext>,
}
pub struct UDataflowEditorUISubsystem {}
pub struct UDataflowEditorOptions {
    pub construction_view_fov: f32,
    pub simulation_view_fov: f32,
    pub b_construction_view_fixed_exposure: bool,
    pub b_simulation_view_fixed_exposure: bool,
    pub construction_profile_name: FString,
    pub simulation_profile_name: FString,
    pub construction_viewport_mouse_pan_button: EDataflowConstructionViewportMousePanButton,
    pub editor_evaluation_mode: EDataflowEditorEvaluationMode,
}
pub struct UDataflowObjectFactory {}
pub struct UDataflowSchema {}
pub struct UDataflowSimulationSettings {
    pub b_is_simulation_playing_by_default: bool,
    pub b_is_async_caching_supported: bool,
    pub b_is_async_caching_enabled_by_default: bool,
    pub b_is_geometry_cache_output_supported: bool,
}
pub struct UDataflowSimulationSceneDescription {
    pub blueprint_class: TSubclassOf<AActor>,
    pub blueprint_transform: FTransform,
    pub b_pause_simulation_viewport_when_playing_in_editor: bool,
    pub b_pause_simulation_viewport_when_not_focused: bool,
    pub cache_asset: UPtr<UChaosCacheCollection>,
    pub cache_params: FDataflowPreviewCacheParams,
    pub b_is_geometry_cache_output_supported: bool,
    pub geometry_cache_asset: UPtr<UGeometryCache>,
    pub embedded_skeletal_mesh: UPtr<USkeletalMesh>,
    pub embedded_static_mesh: UPtr<UStaticMesh>,
    pub b_skeletal_mesh_visibility: bool,
}
pub struct UDataflowReadOnlyToolTarget {
    pub dataflow: UPtr<UDataflow>,
    pub asset: UPtr<UObject>,
}
pub struct UDataflowToolTarget {}
pub struct UDataflowReadOnlyToolTargetFactory {}
pub struct UDataflowToolTargetFactory {}
