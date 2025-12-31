#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FSkinWeightBrushConfig {
    pub strength: f32,
    pub radius: f32,
    pub falloff: f32,
    pub falloff_mode: EWeightBrushFalloffMode,
}
pub struct UToolMeshSelector {
    pub parent_tool: UPtr<
        crate::bindings::interactive_tools_framework::UInteractiveTool,
    >,
    pub world: UPtr<crate::bindings::engine::UWorld>,
    pub preview_mesh: UPtr<crate::bindings::modeling_components::UPreviewMesh>,
    pub polygon_selection_mechanic: UPtr<
        crate::bindings::modeling_components::UPolygonSelectionMechanic,
    >,
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
    pub uv_channel_properties: UPtr<
        crate::bindings::mesh_modeling_tools::UMeshUVChannelProperties,
    >,
    pub settings: UPtr<
        crate::bindings::modeling_operators_editor_only::UParameterizeMeshToolProperties,
    >,
    pub uv_atlas_properties: UPtr<
        crate::bindings::modeling_operators_editor_only::UParameterizeMeshToolUVAtlasProperties,
    >,
    pub x_atlas_properties: UPtr<
        crate::bindings::modeling_operators_editor_only::UParameterizeMeshToolXAtlasProperties,
    >,
    pub patch_builder_properties: UPtr<
        crate::bindings::modeling_operators_editor_only::UParameterizeMeshToolPatchBuilderProperties,
    >,
    pub polygroup_layer_properties: UPtr<
        crate::bindings::modeling_components::UPolygroupLayersProperties,
    >,
    pub material_settings: UPtr<
        crate::bindings::mesh_modeling_tools::UExistingMeshMaterialProperties,
    >,
    pub b_create_uv_layout_view_on_setup: bool,
    pub uv_layout_view: UPtr<crate::bindings::modeling_components::UUVLayoutPreview>,
    pub preview: UPtr<
        crate::bindings::modeling_components::UMeshOpPreviewWithBackgroundCompute,
    >,
    pub factory: UPtr<
        crate::bindings::modeling_operators_editor_only::UParameterizeMeshOperatorFactory,
    >,
}
pub struct UPolygonOnMeshToolBuilder {}
pub struct UPolygonOnMeshToolProperties {
    pub operation: crate::bindings::modeling_operators_editor_only::EEmbeddedPolygonOpMethod,
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
    pub preview: UPtr<
        crate::bindings::modeling_components::UMeshOpPreviewWithBackgroundCompute,
    >,
    pub drawn_line_set: UPtr<crate::bindings::modeling_components::ULineSetComponent>,
    pub plane_mechanic: UPtr<
        crate::bindings::modeling_components::UConstructionPlaneMechanic,
    >,
    pub draw_polygon_mechanic: UPtr<
        crate::bindings::modeling_components::UCollectSurfacePathMechanic,
    >,
}
pub struct USimplifyMeshToolBuilder {}
pub struct USimplifyMeshToolProperties {
    pub simplifier_type: crate::bindings::modeling_operators_editor_only::ESimplifyType,
    pub target_mode: crate::bindings::modeling_operators_editor_only::ESimplifyTargetType,
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
    pub mesh_statistics_properties: UPtr<
        crate::bindings::mesh_modeling_tools::UMeshStatisticsProperties,
    >,
    pub preview: UPtr<
        crate::bindings::modeling_components::UMeshOpPreviewWithBackgroundCompute,
    >,
    pub mesh_elements_display: UPtr<
        crate::bindings::modeling_components::UMeshElementsVisualizer,
    >,
}
pub struct URefSkeletonPoser {}
pub struct USkeletalMeshEditingInterface {}
pub struct ISkeletalMeshEditingInterface {}
pub struct USkeletalMeshGizmoContextObjectBase {}
pub struct USkeletalMeshGizmoWrapperBase {
    pub component: TWeakObjectPtr<crate::bindings::engine::USceneComponent>,
}
pub struct USkeletalMeshEditorContextObjectBase {}
pub struct USkeletonEditingToolBuilder {}
pub struct USkeletonEditingTool {
    pub properties: UPtr<USkeletonEditingProperties>,
    pub projection_properties: UPtr<UProjectionProperties>,
    pub mirroring_properties: UPtr<UMirroringProperties>,
    pub orienting_properties: UPtr<UOrientingProperties>,
    pub selection_mechanic: UPtr<
        crate::bindings::modeling_components::UPolygonSelectionMechanic,
    >,
    pub modifier: UPtr<crate::bindings::skeletal_mesh_modifiers::USkeletonModifier>,
    pub left_click_behavior: UPtr<
        crate::bindings::interactive_tools_framework::USingleClickInputBehavior,
    >,
    pub preview_mesh: UPtr<crate::bindings::modeling_components::UPreviewMesh>,
    pub target_world: TWeakObjectPtr<crate::bindings::engine::UWorld>,
    pub view_context: TWeakObjectPtr<
        crate::bindings::interactive_tools_framework::UGizmoViewContext,
    >,
    pub operation: EEditingOperation,
    pub gizmo_context: TWeakObjectPtr<USkeletalMeshGizmoContextObjectBase>,
    pub gizmo_wrapper: UPtr<USkeletalMeshGizmoWrapperBase>,
    pub editor_context: TWeakObjectPtr<USkeletalMeshEditorContextObjectBase>,
}
pub struct USkeletonEditingProperties {
    pub name: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub b_update_children: bool,
    pub axis_length: f32,
    pub axis_thickness: f32,
    pub b_enable_component_selection: bool,
}
pub struct UProjectionProperties {
    pub projection_type: EProjectionType,
    pub plane_origin: crate::bindings::core_u_object::FVector,
    pub plane_normal: crate::bindings::core_u_object::FVector,
}
pub struct UMirroringProperties {
    pub options: crate::bindings::skeletal_mesh_modifiers::FMirrorOptions,
}
pub struct UOrientingProperties {
    pub b_auto_orient: bool,
    pub options: crate::bindings::skeletal_mesh_modifiers::FOrientOptions,
}
pub struct USkeletonTransformProxy {
    pub transform_mode: crate::bindings::interactive_tools_framework::EToolContextCoordinateSystem,
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
    pub preview: UPtr<
        crate::bindings::modeling_components::UMeshOpPreviewWithBackgroundCompute,
    >,
    pub editor_context: TWeakObjectPtr<USkeletalMeshEditorContextObjectBase>,
}
pub struct USkinWeightsPaintToolBuilder {}
pub struct USkinWeightsPaintToolProperties {
    pub editing_mode: EWeightEditMode,
    pub brush_mode: EWeightEditOperation,
    pub component_selection_mode: EComponentSelectionMode,
    pub color_mode: EWeightColorMode,
    pub color_ramp: TArray<crate::bindings::core_u_object::FLinearColor>,
    pub mirror_axis: crate::bindings::core_u_object::EAxis,
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
    pub source_skeletal_mesh: TWeakObjectPtr<crate::bindings::engine::USkeletalMesh>,
    pub mesh_select_mode: EMeshTransferOption,
    pub source_lod: FName,
    pub source_skin_weight_profile: FName,
    pub source_preview_offset: crate::bindings::core_u_object::FTransform,
}
pub struct UDEPRECATED_WeightToolMeshSelector {}
pub struct UWeightToolTransferManager {
    pub source_preview_mesh: UPtr<crate::bindings::modeling_components::UPreviewMesh>,
    pub source_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub source_target: UPtr<crate::bindings::interactive_tools_framework::UToolTarget>,
    pub mesh_selector: UPtr<UToolMeshSelector>,
}
pub struct UWeightToolSelectionIsolator {
    pub weight_tool: UPtr<USkinWeightsPaintTool>,
}
pub struct USkinWeightsPaintTool {
    pub weight_tool_properties: UPtr<USkinWeightsPaintToolProperties>,
    pub mesh_selector: UPtr<UToolMeshSelector>,
    pub editor_context: TWeakObjectPtr<USkeletalMeshEditorContextObjectBase>,
    pub persona_mode_manager_context: TWeakObjectPtr<
        crate::bindings::persona::UPersonaEditorModeManagerContext,
    >,
    pub target_manager: TWeakObjectPtr<
        crate::bindings::interactive_tools_framework::UToolTargetManager,
    >,
    pub transfer_manager: UPtr<UWeightToolTransferManager>,
    pub selection_isolator: UPtr<UWeightToolSelectionIsolator>,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWeightBrushFalloffMode(pub u8);
impl EWeightBrushFalloffMode {
    pub const SURFACE: EWeightBrushFalloffMode = EWeightBrushFalloffMode(0);
    pub const VOLUME: EWeightBrushFalloffMode = EWeightBrushFalloffMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EComponentSelectionMode(pub u8);
impl EComponentSelectionMode {
    pub const VERTICES: EComponentSelectionMode = EComponentSelectionMode(0);
    pub const EDGES: EComponentSelectionMode = EComponentSelectionMode(1);
    pub const FACES: EComponentSelectionMode = EComponentSelectionMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAttributeEditorElementType(pub u8);
impl EAttributeEditorElementType {
    pub const VERTEX: EAttributeEditorElementType = EAttributeEditorElementType(0);
    pub const VERTEX_INSTANCE: EAttributeEditorElementType = EAttributeEditorElementType(
        1,
    );
    pub const TRIANGLE: EAttributeEditorElementType = EAttributeEditorElementType(2);
    pub const POLYGON: EAttributeEditorElementType = EAttributeEditorElementType(3);
    pub const EDGE: EAttributeEditorElementType = EAttributeEditorElementType(4);
    pub const POLYGON_GROUP: EAttributeEditorElementType = EAttributeEditorElementType(
        5,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAttributeEditorAttribType(pub u8);
impl EAttributeEditorAttribType {
    pub const INT32: EAttributeEditorAttribType = EAttributeEditorAttribType(0);
    pub const BOOLEAN: EAttributeEditorAttribType = EAttributeEditorAttribType(1);
    pub const FLOAT: EAttributeEditorAttribType = EAttributeEditorAttribType(2);
    pub const VECTOR2: EAttributeEditorAttribType = EAttributeEditorAttribType(3);
    pub const VECTOR3: EAttributeEditorAttribType = EAttributeEditorAttribType(4);
    pub const VECTOR4: EAttributeEditorAttribType = EAttributeEditorAttribType(5);
    pub const STRING: EAttributeEditorAttribType = EAttributeEditorAttribType(6);
    pub const UNKNOWN: EAttributeEditorAttribType = EAttributeEditorAttribType(7);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPolygonType(pub i32);
impl EPolygonType {
    pub const CIRCLE: EPolygonType = EPolygonType(0);
    pub const SQUARE: EPolygonType = EPolygonType(1);
    pub const RECTANGLE: EPolygonType = EPolygonType(2);
    pub const ROUND_RECT: EPolygonType = EPolygonType(3);
    pub const CUSTOM: EPolygonType = EPolygonType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EEditingOperation(pub u8);
impl EEditingOperation {
    pub const SELECT: EEditingOperation = EEditingOperation(0);
    pub const CREATE: EEditingOperation = EEditingOperation(1);
    pub const REMOVE: EEditingOperation = EEditingOperation(2);
    pub const TRANSFORM: EEditingOperation = EEditingOperation(3);
    pub const PARENT: EEditingOperation = EEditingOperation(4);
    pub const RENAME: EEditingOperation = EEditingOperation(5);
    pub const MIRROR: EEditingOperation = EEditingOperation(6);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EProjectionType(pub u8);
impl EProjectionType {
    pub const CAMERA_PLANE: EProjectionType = EProjectionType(0);
    pub const ON_MESH: EProjectionType = EProjectionType(1);
    pub const WITHIN_MESH: EProjectionType = EProjectionType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESkinWeightsBindType(pub u8);
impl ESkinWeightsBindType {
    pub const DIRECT_DISTANCE: ESkinWeightsBindType = ESkinWeightsBindType(0);
    pub const GEODESIC_VOXEL: ESkinWeightsBindType = ESkinWeightsBindType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWeightEditMode(pub u8);
impl EWeightEditMode {
    pub const BRUSH: EWeightEditMode = EWeightEditMode(0);
    pub const MESH: EWeightEditMode = EWeightEditMode(1);
    pub const BONES: EWeightEditMode = EWeightEditMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWeightEditOperation(pub u8);
impl EWeightEditOperation {
    pub const ADD: EWeightEditOperation = EWeightEditOperation(0);
    pub const REPLACE: EWeightEditOperation = EWeightEditOperation(1);
    pub const MULTIPLY: EWeightEditOperation = EWeightEditOperation(2);
    pub const RELAX: EWeightEditOperation = EWeightEditOperation(3);
    pub const RELATIVE_SCALE: EWeightEditOperation = EWeightEditOperation(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWeightColorMode(pub u8);
impl EWeightColorMode {
    pub const GREYSCALE: EWeightColorMode = EWeightColorMode(0);
    pub const RAMP: EWeightColorMode = EWeightColorMode(1);
    pub const BONE_COLORS: EWeightColorMode = EWeightColorMode(2);
    pub const FULL_MATERIAL: EWeightColorMode = EWeightColorMode(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMirrorDirection(pub u8);
impl EMirrorDirection {
    pub const POSITIVE_TO_NEGATIVE: EMirrorDirection = EMirrorDirection(0);
    pub const NEGATIVE_TO_POSITIVE: EMirrorDirection = EMirrorDirection(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshTransferOption(pub u8);
impl EMeshTransferOption {
    pub const SOURCE: EMeshTransferOption = EMeshTransferOption(0);
    pub const TARGET: EMeshTransferOption = EMeshTransferOption(1);
}
