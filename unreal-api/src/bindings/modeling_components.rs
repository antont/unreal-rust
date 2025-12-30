#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(1))]
pub struct FModelingToolsAxisFilter {
    pub b_axis_x: bool,
    pub b_axis_y: bool,
    pub b_axis_z: bool,
}
#[repr(C, align(1))]
pub struct FModelingToolsColorChannelFilter {
    pub b_red: bool,
    pub b_green: bool,
    pub b_blue: bool,
    pub b_alpha: bool,
}
#[repr(C, align(8))]
pub struct FRenderableTriangleVertex {
    pub position: FVector,
    pub uv: FVector2D,
    pub normal: FVector,
    pub color: FColor,
}
#[repr(C, align(8))]
pub struct FRenderableTriangle {
    pub material: UPtr<UMaterialInterface>,
    pub vertex0: FRenderableTriangleVertex,
    pub vertex1: FRenderableTriangleVertex,
    pub vertex2: FRenderableTriangleVertex,
}
#[repr(C, align(16))]
pub struct FCreateMeshObjectParams {
    pub source_component: UPtr<UPrimitiveComponent>,
    pub type_hint: ECreateObjectTypeHint,
    pub type_hint_class: TSubclassOf<UObject>,
    pub type_hint_extended: i32,
    pub target_world: UPtr<UWorld>,
    pub transform: FTransform,
    pub base_name: FString,
    pub materials: TArray<UPtr<UMaterialInterface>>,
    pub asset_materials: TArray<UPtr<UMaterialInterface>>,
    pub b_enable_collision: bool,
    pub collision_mode: ECollisionTraceFlag,
    pub b_enable_raytracing_support: bool,
    pub b_generate_lightmap_u_vs: bool,
    pub b_enable_recompute_normals: bool,
    pub b_enable_recompute_tangents: bool,
    pub b_enable_nanite: bool,
    pub nanite_proxy_triangle_percent_deprecated: f32,
    pub nanite_settings: FMeshNaniteSettings,
}
#[repr(C, align(8))]
pub struct FCreateMeshObjectResult {
    pub result_code: ECreateModelingObjectResult,
    pub new_actor: UPtr<AActor>,
    pub new_component: UPtr<UPrimitiveComponent>,
    pub new_asset: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FCreateTextureObjectParams {
    pub type_hint_extended: i32,
    pub target_world: UPtr<UWorld>,
    pub store_relative_to_object: UPtr<UObject>,
    pub base_name: FString,
    pub generated_transient_texture: UPtr<UTexture2D>,
    pub full_asset_path: FString,
}
#[repr(C, align(8))]
pub struct FCreateTextureObjectResult {
    pub result_code: ECreateModelingObjectResult,
    pub new_asset: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FCreateMaterialObjectParams {
    pub target_world: UPtr<UWorld>,
    pub store_relative_to_object: UPtr<UObject>,
    pub base_name: FString,
    pub material_to_duplicate: UPtr<UMaterialInterface>,
}
#[repr(C, align(8))]
pub struct FCreateMaterialObjectResult {
    pub result_code: ECreateModelingObjectResult,
    pub new_asset: UPtr<UObject>,
}
#[repr(C, align(16))]
pub struct FCreateActorParams {
    pub target_world: UPtr<UWorld>,
    pub base_name: FString,
    pub transform: FTransform,
    pub template_actor_deprecated: UPtr<AActor>,
    pub template_asset: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FCreateActorResult {
    pub result_code: ECreateModelingObjectResult,
    pub new_actor: UPtr<AActor>,
}
#[repr(C, align(8))]
pub struct FCreateComponentParams {
    pub host_actor: UPtr<AActor>,
    pub component_class: TSubclassOf<UObject>,
    pub base_name: FString,
    pub b_set_as_root: bool,
    pub b_transact: bool,
}
#[repr(C, align(8))]
pub struct FCreateComponentResult {
    pub result_code: ECreateModelingObjectResult,
    pub new_component: UPtr<UActorComponent>,
}
#[repr(C, align(8))]
pub struct FMeshElementSelectionParams {
    pub selection_fill_color: UPtr<UMaterialInstanceDynamic>,
}
pub struct UDynamicMeshProvider {}
pub struct IDynamicMeshProvider {}
pub struct UDynamicMeshCommitter {}
pub struct IDynamicMeshCommitter {}
pub struct UPersistentDynamicMeshSource {}
pub struct IPersistentDynamicMeshSource {}
pub struct UInteractiveToolActivity {}
pub struct UToolActivityHost {}
pub struct IToolActivityHost {}
pub struct ULatticeStateStorage {}
pub struct ILatticeStateStorage {}
pub struct UMeshSculptLayersManager {}
pub struct IMeshSculptLayersManager {}
pub struct UModelingToolExternalDynamicMeshUpdateAPI {}
pub struct IModelingToolExternalDynamicMeshUpdateAPI {}
pub struct UGeometrySelectionEditCommandArguments {}
pub struct UGeometrySelectionEditCommandResult {}
pub struct UGeometrySelectionEditCommand {}
pub struct UVoxelProperties {
    pub voxel_count: i32,
    pub b_auto_simplify: bool,
    pub b_remove_internal_surfaces: bool,
    pub simplify_max_error_factor: f64,
    pub cube_root_min_component_volume: f64,
}
pub struct UVolumetricBrushStampIndicatorBuilder {}
pub struct UVolumetricBrushStampIndicator {
    pub sphere_mesh: UPtr<UPreviewMesh>,
}
pub struct UMultiSelectionMeshEditingToolBuilder {}
pub struct UBaseCreateFromSelectedToolBuilder {}
pub struct UOnAcceptHandleSourcesPropertiesBase {}
pub struct UOnAcceptHandleSourcesProperties {
    pub handle_inputs: EHandleSourcesMethod,
}
pub struct UBaseCreateFromSelectedHandleSourceProperties {
    pub output_write_to: EBaseCreateFromSelectedTargetType,
    pub output_new_name: FString,
    pub output_existing_name: FString,
}
pub struct UBaseCreateFromSelectedCollisionProperties {
    pub b_transfer_collision: bool,
}
pub struct UTransformInputsToolProperties {
    pub b_show_transform_gizmo: bool,
}
pub struct UMultiSelectionMeshEditingTool {
    pub target_world: TWeakObjectPtr<UWorld>,
}
pub struct UBaseCreateFromSelectedTool {
    pub transform_properties: UPtr<UTransformInputsToolProperties>,
    pub output_type_properties: UPtr<UCreateMeshObjectTypeProperties>,
    pub handle_sources_properties: UPtr<UBaseCreateFromSelectedHandleSourceProperties>,
    pub collision_properties: UPtr<UBaseCreateFromSelectedCollisionProperties>,
    pub preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub transform_proxies: TArray<UPtr<UTransformProxy>>,
    pub transform_gizmos: TArray<UPtr<UCombinedTransformGizmo>>,
}
pub struct USingleTargetWithSelectionToolBuilder {}
pub struct UBaseMeshProcessingToolBuilder {}
pub struct USingleTargetWithSelectionTool {
    pub target_world: TWeakObjectPtr<UWorld>,
    pub geometry_selection_viz_properties: UPtr<
        UGeometrySelectionVisualizationProperties,
    >,
    pub geometry_selection_viz: UPtr<UPreviewGeometry>,
}
pub struct UBaseMeshProcessingTool {
    pub preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
}
pub struct UBaseVoxelTool {
    pub vox_properties: UPtr<UVoxelProperties>,
}
pub struct UMeshSurfacePointMeshEditingToolBuilder {}
pub struct UMultiTargetWithSelectionToolBuilder {}
pub struct UMultiTargetWithSelectionTool {
    pub target_world: TWeakObjectPtr<UWorld>,
    pub geometry_selection_viz_properties: UPtr<
        UGeometrySelectionVisualizationProperties,
    >,
    pub geometry_selection_viz: UPtr<UPreviewGeometry>,
}
pub struct USingleSelectionMeshEditingToolBuilder {}
pub struct USingleSelectionMeshEditingTool {
    pub target_world: TWeakObjectPtr<UWorld>,
}
pub struct UDynamicMeshReplacementChangeTarget {}
pub struct UOctreeDynamicMeshComponent {
    pub mesh_object: UPtr<UDynamicMesh>,
}
pub struct ULineSetComponent {
    pub line_material: UPtr<UMaterialInterface>,
    pub bounds: FBoxSphereBounds,
    pub b_bounds_dirty: bool,
}
pub struct UMeshElementsVisualizerProperties {
    pub b_visible: bool,
    pub b_show_wireframe: bool,
    pub b_show_borders: bool,
    pub b_show_uv_seams: bool,
    pub b_show_normal_seams: bool,
    pub b_show_tangent_seams: bool,
    pub b_show_color_seams: bool,
    pub thickness_scale: f32,
    pub wireframe_color: FColor,
    pub boundary_edge_color: FColor,
    pub uv_seam_color: FColor,
    pub normal_seam_color: FColor,
    pub tangent_seam_color: FColor,
    pub color_seam_color: FColor,
    pub depth_bias: f32,
    pub b_adjust_depth_bias_using_mesh_size: bool,
}
pub struct UPreviewGeometry {
    pub parent_actor: UPtr<APreviewGeometryActor>,
    pub triangle_sets: TMap<FString, UPtr<UTriangleSetComponent>>,
    pub line_sets: TMap<FString, UPtr<ULineSetComponent>>,
    pub point_sets: TMap<FString, UPtr<UPointSetComponent>>,
}
pub struct UMeshElementsVisualizer {
    pub settings: UPtr<UMeshElementsVisualizerProperties>,
    pub wireframe_component: UPtr<UMeshWireframeComponent>,
}
pub struct UMeshWireframeComponent {
    pub line_depth_bias: f32,
    pub line_depth_bias_size_scale: f32,
    pub thickness_scale: f32,
    pub b_enable_wireframe: bool,
    pub wireframe_color: FColor,
    pub wireframe_thickness: f32,
    pub b_enable_boundary_edges: bool,
    pub boundary_edge_color: FColor,
    pub boundary_edge_thickness: f32,
    pub b_enable_uv_seams: bool,
    pub uv_seam_color: FColor,
    pub uv_seam_thickness: f32,
    pub b_enable_normal_seams: bool,
    pub normal_seam_color: FColor,
    pub normal_seam_thickness: f32,
    pub b_enable_tangent_seams: bool,
    pub tangent_seam_color: FColor,
    pub tangent_seam_thickness: f32,
    pub b_enable_color_seams: bool,
    pub color_seam_color: FColor,
    pub color_seam_thickness: f32,
    pub line_material: UPtr<UMaterialInterface>,
    pub local_bounds: FBoxSphereBounds,
}
pub struct UPointSetComponent {
    pub point_material: UPtr<UMaterialInterface>,
    pub bounds: FBoxSphereBounds,
    pub b_bounds_dirty: bool,
}
pub struct UPreviewMesh {
    pub b_build_spatial_data_structure: bool,
    pub dynamic_mesh_component: UPtr<UDynamicMeshComponent>,
}
pub struct UPolyEditPreviewMesh {}
pub struct APreviewGeometryActor {}
pub struct UTriangleSetComponent {
    pub bounds: FBoxSphereBounds,
    pub b_bounds_dirty: bool,
}
pub struct UUVLayoutPreviewProperties {
    pub b_enabled: bool,
    pub side: EUVLayoutPreviewSide,
    pub scale: f32,
    pub offset: FVector2D,
    pub b_show_wireframe: bool,
}
pub struct UUVLayoutPreview {
    pub settings: UPtr<UUVLayoutPreviewProperties>,
    pub preview_mesh: UPtr<UPreviewMesh>,
    pub mesh_elements_visualizer: UPtr<UMeshElementsVisualizer>,
    pub triangle_component: UPtr<UTriangleSetComponent>,
    pub b_show_backing_rectangle: bool,
    pub backing_rectangle_material: UPtr<UMaterialInterface>,
}
pub struct UCollectSurfacePathMechanic {}
pub struct UCollisionPrimitivesMechanic {
    pub preview_geometry: UPtr<UPreviewGeometry>,
    pub drawn_primitive_edges: UPtr<ULineSetComponent>,
    pub translate_transform_proxy: UPtr<UTransformProxy>,
    pub sphere_transform_proxy: UPtr<UTransformProxy>,
    pub box_transform_proxy: UPtr<UTransformProxy>,
    pub capsule_transform_proxy: UPtr<UTransformProxy>,
    pub full_transform_proxy: UPtr<UTransformProxy>,
    pub current_active_proxy: UPtr<UTransformProxy>,
    pub translate_transform_gizmo: UPtr<UCombinedTransformGizmo>,
    pub sphere_transform_gizmo: UPtr<UCombinedTransformGizmo>,
    pub box_transform_gizmo: UPtr<UCombinedTransformGizmo>,
    pub capsule_transform_gizmo: UPtr<UCombinedTransformGizmo>,
    pub full_transform_gizmo: UPtr<UCombinedTransformGizmo>,
    pub box_interval_gizmo: UPtr<UIntervalGizmo>,
    pub box_x_interval_source: UPtr<UGizmoLocalFloatParameterSource>,
    pub box_y_interval_source: UPtr<UGizmoLocalFloatParameterSource>,
    pub box_z_interval_source: UPtr<UGizmoLocalFloatParameterSource>,
    pub capsule_interval_gizmo: UPtr<UIntervalGizmo>,
    pub capsule_radius_interval_source: UPtr<UGizmoLocalFloatParameterSource>,
    pub capsule_length_interval_source: UPtr<UGizmoLocalFloatParameterSource>,
    pub marquee_mechanic: UPtr<URectangleMarqueeMechanic>,
}
pub struct UConstructionPlaneMechanic {
    pub plane_transform_gizmo: UPtr<UCombinedTransformGizmo>,
    pub plane_transform_proxy: UPtr<UTransformProxy>,
    pub click_to_set_plane_behavior: UPtr<USingleClickInputBehavior>,
    pub middle_click_to_set_gizmo_behavior: UPtr<ULocalSingleClickInputBehavior>,
}
pub struct UCurveControlPointsMechanic {
    pub click_behavior: UPtr<USingleClickInputBehavior>,
    pub hover_behavior: UPtr<UMouseHoverBehavior>,
    pub preview_geometry_actor: UPtr<APreviewGeometryActor>,
    pub drawn_control_points: UPtr<UPointSetComponent>,
    pub drawn_control_segments: UPtr<ULineSetComponent>,
    pub preview_point: UPtr<UPointSetComponent>,
    pub preview_segment: UPtr<ULineSetComponent>,
    pub point_transform_proxy: UPtr<UTransformProxy>,
    pub point_transform_gizmo: UPtr<UCombinedTransformGizmo>,
}
pub struct UDragAlignmentMechanic {}
pub struct UDragAlignmentInteraction {}
pub struct ULatticeControlPointsMechanic {
    pub preview_geometry_actor: UPtr<APreviewGeometryActor>,
    pub drawn_control_points: UPtr<UPointSetComponent>,
    pub drawn_lattice_edges: UPtr<ULineSetComponent>,
    pub point_transform_proxy: UPtr<UTransformProxy>,
    pub point_transform_gizmo: UPtr<UCombinedTransformGizmo>,
    pub marquee_mechanic: UPtr<URectangleMarqueeMechanic>,
}
pub struct UPlaneDistanceFromHitMechanic {}
pub struct UPolyLassoMarqueeMechanic {
    pub spacing_tolerance: f32,
    pub line_thickness: f32,
    pub line_color: FLinearColor,
    pub closed_color: FLinearColor,
    pub b_enable_freehand_polygons: bool,
    pub b_enable_multi_click_polygons: bool,
    pub click_drag_behavior: UPtr<UClickDragInputBehavior>,
    pub hover_behavior: UPtr<UMouseHoverBehavior>,
}
pub struct URectangleMarqueeMechanic {
    pub b_use_external_click_drag_behavior: bool,
    pub b_use_external_update_camera_state: bool,
    pub on_drag_rectangle_changed_deferred_threshold: f64,
    pub click_drag_behavior: UPtr<UClickDragInputBehavior>,
}
pub struct URectangleMarqueeInteraction {}
pub struct USpaceCurveDeformationMechanicPropertySet {
    pub transform_mode: ESpaceCurveControlPointTransformMode,
    pub transform_origin: ESpaceCurveControlPointOriginMode,
    pub softness: f32,
    pub soft_falloff: ESpaceCurveControlPointFalloffType,
}
pub struct USpaceCurveDeformationMechanic {
    pub click_behavior: UPtr<USingleClickInputBehavior>,
    pub hover_behavior: UPtr<UMouseHoverBehavior>,
    pub transform_properties: UPtr<USpaceCurveDeformationMechanicPropertySet>,
    pub preview_geometry_actor: UPtr<APreviewGeometryActor>,
    pub render_points: UPtr<UPointSetComponent>,
    pub render_segments: UPtr<ULineSetComponent>,
    pub point_transform_proxy: UPtr<UTransformProxy>,
    pub point_transform_gizmo: UPtr<UCombinedTransformGizmo>,
}
pub struct USpatialCurveDistanceMechanic {}
pub struct UMeshOpPreviewWithBackgroundCompute {
    pub preview_mesh: UPtr<UPreviewMesh>,
    pub standard_materials: TArray<UPtr<UMaterialInterface>>,
    pub override_material: UPtr<UMaterialInterface>,
    pub working_material: UPtr<UMaterialInterface>,
    pub secondary_material: UPtr<UMaterialInterface>,
    pub preview_world: TWeakObjectPtr<UWorld>,
}
pub struct UModelingComponentsSettings {
    pub b_enable_ray_tracing_while_editing: bool,
    pub b_enable_ray_tracing: bool,
    pub b_generate_lightmap_u_vs: bool,
    pub b_enable_collision: bool,
    pub collision_mode: ECollisionTraceFlag,
}
pub struct UModelingComponentsEditorSettings {
    pub grid_mode: EModelingComponentsPlaneVisualizationMode,
    pub num_grid_lines: i32,
    pub grid_spacing: f32,
    pub grid_scale: f32,
    pub grid_size: f32,
}
pub struct UModelingObjectsCreationAPI {}
pub struct APreviewMeshActor {}
pub struct UCreateMeshObjectTypeProperties {
    pub output_type: FString,
    pub volume_type: TSubclassOf<AVolume>,
    pub output_type_names_list: TArray<FString>,
    pub b_show_volume_list: bool,
}
pub struct UGeometrySelectionVisualizationProperties {
    pub b_enable_show_triangle_roi_border: bool,
    pub b_enable_show_edge_selection_vertices: bool,
    pub selection_element_type: EGeometrySelectionElementType,
    pub selection_topology_type: EGeometrySelectionTopologyType,
    pub b_show_selection: bool,
    pub b_show_triangle_roi_border: bool,
    pub b_show_hidden: bool,
    pub b_show_edge_selection_vertices: bool,
    pub line_thickness: f32,
    pub point_size: f32,
    pub depth_bias: f32,
    pub face_color: FColor,
    pub line_color: FColor,
    pub point_color: FColor,
    pub triangle_roi_border_color: FColor,
    pub triangle_material: UPtr<UMaterialInterface>,
    pub line_material: UPtr<UMaterialInterface>,
    pub point_material: UPtr<UMaterialInterface>,
    pub triangle_material_showing_hidden: UPtr<UMaterialInterface>,
    pub line_material_showing_hidden: UPtr<UMaterialInterface>,
    pub point_material_showing_hidden: UPtr<UMaterialInterface>,
}
pub struct UOnAcceptHandleSourcesPropertiesSingle {
    pub handle_inputs: EHandleSourcesMethod,
}
pub struct UPolygroupLayersProperties {
    pub active_group_layer: FName,
    pub group_layers_list: TArray<FString>,
}
pub struct UWeightMapSetProperties {
    pub weight_map: FName,
    pub weight_maps_list: TArray<FString>,
    pub b_invert_weight_map: bool,
}
pub struct UMeshTopologySelectionMechanic {
    pub properties: UPtr<UMeshTopologySelectionMechanicProperties>,
    pub hover_behavior: UPtr<UMouseHoverBehavior>,
    pub click_or_drag_behavior: UPtr<USingleClickOrDragInputBehavior>,
    pub marquee_mechanic: UPtr<URectangleMarqueeMechanic>,
    pub marquee_selection_update_type: EMarqueeSelectionUpdateType,
    pub preview_geometry_actor: UPtr<APreviewGeometryActor>,
    pub drawn_triangle_set_component: UPtr<UTriangleSetComponent>,
    pub highlighted_face_material: UPtr<UMaterialInterface>,
}
pub struct UBoundarySelectionMechanic {}
pub struct UGeometrySelectionManager {
    pub selection_arguments: UPtr<UGeometrySelectionEditCommandArguments>,
    pub tools_context: UPtr<UInteractiveToolsContext>,
    pub preview_geometry: UPtr<UPreviewGeometry>,
    pub unselected_params: FMeshElementSelectionParams,
    pub hover_over_selected_params: FMeshElementSelectionParams,
    pub hover_over_unselected_params: FMeshElementSelectionParams,
    pub selected_params: FMeshElementSelectionParams,
}
pub struct UMeshTopologySelectionMechanicProperties {
    pub b_select_vertices: bool,
    pub b_select_edges: bool,
    pub b_select_faces: bool,
    pub b_select_edge_loops: bool,
    pub b_select_edge_rings: bool,
    pub b_hit_back_faces: bool,
    pub b_enable_marquee: bool,
    pub b_marquee_ignore_occlusion: bool,
    pub b_prefer_projected_element: bool,
    pub b_select_down_ray: bool,
    pub b_ignore_occlusion: bool,
}
pub struct UDEPRECATED_PolygonSelectionMechanicProperties {}
pub struct UPolygonSelectionMechanic {}
pub struct UModelingSceneSnappingManager {
    pub parent_context: UPtr<UInteractiveToolsContext>,
}
pub struct UToolHostCustomizationAPI {}
pub struct IToolHostCustomizationAPI {}
pub struct UMultiTransformer {
    pub gizmo_manager: UPtr<UInteractiveGizmoManager>,
    pub transform_gizmo: UPtr<UCombinedTransformGizmo>,
    pub transform_proxy: UPtr<UTransformProxy>,
    pub drag_alignment_mechanic: UPtr<UDragAlignmentMechanic>,
}
