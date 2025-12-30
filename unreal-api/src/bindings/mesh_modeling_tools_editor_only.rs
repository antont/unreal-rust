#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FSkinWeightBrushConfig {
    pub strength: f32,
    pub radius: f32,
    pub falloff: f32,
    pub falloff_mode: EWeightBrushFalloffMode,
}
pub struct UToolMeshSelector {
    pub parent_tool: UPtr<UInteractiveTool>,
    pub world: UPtr<UWorld>,
    pub preview_mesh: UPtr<UPreviewMesh>,
    pub polygon_selection_mechanic: UPtr<UPolygonSelectionMechanic>,
}
pub struct UAttributeEditorToolBuilder {}
pub struct UAttributeEditorAttribProperties {
    pub vertex_attributes: TArray<FString>,
    pub instance_attributes: TArray<FString>,
    pub triangle_attributes: TArray<FString>,
    pub polygon_attributes: TArray<FString>,
    pub edge_attributes: TArray<FString>,
    pub group_attributes: TArray<FString>,
}
pub struct UAttributeEditorActionPropertySet {}
pub struct UAttributeEditorNormalsActions {}
pub struct UAttributeEditorUVActions {
    pub uv_layer: FString,
    pub uv_layer_names_list: TArray<FString>,
}
pub struct UAttributeEditorLightmapUVActions {
    pub b_generate_lightmap_u_vs: bool,
    pub source_uv_index: i32,
    pub destination_uv_index: i32,
}
pub struct UAttributeEditorNewAttributeActions {
    pub new_name: FString,
    pub element_type: EAttributeEditorElementType,
    pub data_type: EAttributeEditorAttribType,
}
pub struct UAttributeEditorModifyAttributeActions {
    pub attribute: FString,
    pub attribute_names_list: TArray<FString>,
}
pub struct UAttributeEditorCopyAttributeActions {
    pub from_attribute: TArray<FString>,
    pub to_attribute: TArray<FString>,
}
pub struct UAttributeEditorTool {
    pub normals_actions: UPtr<UAttributeEditorNormalsActions>,
    pub uv_actions: UPtr<UAttributeEditorUVActions>,
    pub lightmap_uv_actions: UPtr<UAttributeEditorLightmapUVActions>,
    pub attribute_props: UPtr<UAttributeEditorAttribProperties>,
    pub new_attribute_props: UPtr<UAttributeEditorNewAttributeActions>,
    pub modify_attribute_props: UPtr<UAttributeEditorModifyAttributeActions>,
    pub copy_attribute_props: UPtr<UAttributeEditorCopyAttributeActions>,
}
pub struct UParameterizeMeshToolBuilder {}
pub struct UParameterizeMeshTool {
    pub uv_channel_properties: UPtr<UMeshUVChannelProperties>,
    pub settings: UPtr<UParameterizeMeshToolProperties>,
    pub uv_atlas_properties: UPtr<UParameterizeMeshToolUVAtlasProperties>,
    pub x_atlas_properties: UPtr<UParameterizeMeshToolXAtlasProperties>,
    pub patch_builder_properties: UPtr<UParameterizeMeshToolPatchBuilderProperties>,
    pub polygroup_layer_properties: UPtr<UPolygroupLayersProperties>,
    pub material_settings: UPtr<UExistingMeshMaterialProperties>,
    pub b_create_uv_layout_view_on_setup: bool,
    pub uv_layout_view: UPtr<UUVLayoutPreview>,
    pub preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub factory: UPtr<UParameterizeMeshOperatorFactory>,
}
pub struct UPolygonOnMeshToolBuilder {}
pub struct UPolygonOnMeshToolProperties {
    pub operation: EEmbeddedPolygonOpMethod,
    pub shape: EPolygonType,
    pub b_cut_with_boolean: bool,
    pub b_try_to_fix_holes: bool,
    pub polygon_scale: f32,
    pub width: f32,
    pub height: f32,
    pub corner_ratio: f32,
    pub subdivisions: i32,
    pub b_can_accept_failed_result: bool,
    pub b_show_intermediate_result_on_failure: bool,
}
pub struct UPolygonOnMeshToolActionPropertySet {}
pub struct UPolygonOnMeshTool {
    pub basic_properties: UPtr<UPolygonOnMeshToolProperties>,
    pub action_properties: UPtr<UPolygonOnMeshToolActionPropertySet>,
    pub preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub drawn_line_set: UPtr<ULineSetComponent>,
    pub plane_mechanic: UPtr<UConstructionPlaneMechanic>,
    pub draw_polygon_mechanic: UPtr<UCollectSurfacePathMechanic>,
}
pub struct USimplifyMeshToolBuilder {}
pub struct USimplifyMeshToolProperties {
    pub simplifier_type: ESimplifyType,
    pub target_mode: ESimplifyTargetType,
    pub target_percentage: i32,
    pub target_edge_length: f32,
    pub target_triangle_count: i32,
    pub target_vertex_count: i32,
    pub minimal_angle_threshold: f32,
    pub poly_edge_angle_tolerance: f32,
    pub boundary_edge_angle_tolerance: f32,
    pub b_discard_attributes: bool,
    pub b_geometric_constraint: bool,
    pub geometric_tolerance: f32,
    pub b_show_group_colors: bool,
    pub b_reproject: bool,
}
pub struct USimplifyMeshTool {
    pub simplify_properties: UPtr<USimplifyMeshToolProperties>,
    pub mesh_statistics_properties: UPtr<UMeshStatisticsProperties>,
    pub preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub mesh_elements_display: UPtr<UMeshElementsVisualizer>,
}
pub struct URefSkeletonPoser {}
pub struct USkeletalMeshEditingInterface {}
pub struct ISkeletalMeshEditingInterface {}
pub struct USkeletalMeshGizmoContextObjectBase {}
pub struct USkeletalMeshGizmoWrapperBase {
    pub component: TWeakObjectPtr<USceneComponent>,
}
pub struct USkeletalMeshEditorContextObjectBase {}
pub struct USkeletonEditingToolBuilder {}
pub struct USkeletonEditingTool {
    pub properties: UPtr<USkeletonEditingProperties>,
    pub projection_properties: UPtr<UProjectionProperties>,
    pub mirroring_properties: UPtr<UMirroringProperties>,
    pub orienting_properties: UPtr<UOrientingProperties>,
    pub selection_mechanic: UPtr<UPolygonSelectionMechanic>,
    pub modifier: UPtr<USkeletonModifier>,
    pub left_click_behavior: UPtr<USingleClickInputBehavior>,
    pub preview_mesh: UPtr<UPreviewMesh>,
    pub target_world: TWeakObjectPtr<UWorld>,
    pub view_context: TWeakObjectPtr<UGizmoViewContext>,
    pub operation: EEditingOperation,
    pub gizmo_context: TWeakObjectPtr<USkeletalMeshGizmoContextObjectBase>,
    pub gizmo_wrapper: UPtr<USkeletalMeshGizmoWrapperBase>,
    pub editor_context: TWeakObjectPtr<USkeletalMeshEditorContextObjectBase>,
}
pub struct USkeletonEditingProperties {
    pub name: FName,
    pub transform: FTransform,
    pub b_update_children: bool,
    pub axis_length: f32,
    pub axis_thickness: f32,
    pub b_enable_component_selection: bool,
}
pub struct UProjectionProperties {
    pub projection_type: EProjectionType,
    pub plane_origin: FVector,
    pub plane_normal: FVector,
}
pub struct UMirroringProperties {
    pub options: FMirrorOptions,
}
pub struct UOrientingProperties {
    pub b_auto_orient: bool,
    pub options: FOrientOptions,
}
pub struct USkeletonTransformProxy {
    pub transform_mode: EToolContextCoordinateSystem,
}
pub struct USkinWeightsBindingToolBuilder {}
pub struct USkinWeightsBindingToolProperties {
    pub current_bone: FName,
    pub binding_type: ESkinWeightsBindType,
    pub stiffness: f32,
    pub max_influences: i32,
    pub voxel_resolution: i32,
}
pub struct USkinWeightsBindingTool {
    pub properties: UPtr<USkinWeightsBindingToolProperties>,
    pub preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub editor_context: TWeakObjectPtr<USkeletalMeshEditorContextObjectBase>,
}
pub struct USkinWeightsPaintToolBuilder {}
pub struct USkinWeightsPaintToolProperties {
    pub editing_mode: EWeightEditMode,
    pub brush_mode: EWeightEditOperation,
    pub component_selection_mode: EComponentSelectionMode,
    pub color_mode: EWeightColorMode,
    pub color_ramp: TArray<FLinearColor>,
    pub mirror_axis: EAxis,
    pub mirror_direction: EMirrorDirection,
    pub prune_value: f32,
    pub clamp_value: i32,
    pub clamp_select_value: i32,
    pub add_strength: f32,
    pub replace_value: f32,
    pub relax_strength: f32,
    pub average_strength: f32,
    pub brush_config_add: FSkinWeightBrushConfig,
    pub brush_config_replace: FSkinWeightBrushConfig,
    pub brush_config_multiply: FSkinWeightBrushConfig,
    pub brush_config_relax: FSkinWeightBrushConfig,
    pub active_skin_weight_profile: FName,
    pub b_show_new_profile_name: bool,
    pub new_skin_weight_profile: FName,
    pub source_skeletal_mesh: TWeakObjectPtr<USkeletalMesh>,
    pub mesh_select_mode: EMeshTransferOption,
    pub source_lod: FName,
    pub source_skin_weight_profile: FName,
    pub source_preview_offset: FTransform,
}
pub struct UDEPRECATED_WeightToolMeshSelector {}
pub struct UWeightToolTransferManager {
    pub source_preview_mesh: UPtr<UPreviewMesh>,
    pub source_skeletal_mesh: UPtr<USkeletalMesh>,
    pub source_target: UPtr<UToolTarget>,
    pub mesh_selector: UPtr<UToolMeshSelector>,
}
pub struct UWeightToolSelectionIsolator {
    pub weight_tool: UPtr<USkinWeightsPaintTool>,
}
pub struct USkinWeightsPaintTool {
    pub weight_tool_properties: UPtr<USkinWeightsPaintToolProperties>,
    pub mesh_selector: UPtr<UToolMeshSelector>,
    pub editor_context: TWeakObjectPtr<USkeletalMeshEditorContextObjectBase>,
    pub persona_mode_manager_context: TWeakObjectPtr<UPersonaEditorModeManagerContext>,
    pub target_manager: TWeakObjectPtr<UToolTargetManager>,
    pub transfer_manager: UPtr<UWeightToolTransferManager>,
    pub selection_isolator: UPtr<UWeightToolSelectionIsolator>,
}
