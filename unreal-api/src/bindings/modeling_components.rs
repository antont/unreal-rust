#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
    pub position: crate::bindings::core_u_object::FVector,
    pub uv: crate::bindings::core_u_object::FVector2D,
    pub normal: crate::bindings::core_u_object::FVector,
    pub color: crate::bindings::core_u_object::FColor,
}
#[repr(C, align(8))]
pub struct FRenderableTriangle {
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub vertex0: FRenderableTriangleVertex,
    pub vertex1: FRenderableTriangleVertex,
    pub vertex2: FRenderableTriangleVertex,
}
#[repr(C, align(16))]
pub struct FCreateMeshObjectParams {
    pub source_component: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub type_hint: ECreateObjectTypeHint,
    pub type_hint_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub type_hint_extended: i32,
    pub target_world: UPtr<crate::bindings::engine::UWorld>,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub base_name: FString,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub asset_materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub b_enable_collision: bool,
    pub collision_mode: crate::bindings::physics_core::ECollisionTraceFlag,
    pub b_enable_raytracing_support: bool,
    pub b_generate_lightmap_u_vs: bool,
    pub b_enable_recompute_normals: bool,
    pub b_enable_recompute_tangents: bool,
    pub b_enable_nanite: bool,
    pub nanite_proxy_triangle_percent_deprecated: f32,
    pub nanite_settings: crate::bindings::engine::FMeshNaniteSettings,
}
#[repr(C, align(8))]
pub struct FCreateMeshObjectResult {
    pub result_code: ECreateModelingObjectResult,
    pub new_actor: UPtr<crate::bindings::engine::AActor>,
    pub new_component: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub new_asset: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FCreateTextureObjectParams {
    pub type_hint_extended: i32,
    pub target_world: UPtr<crate::bindings::engine::UWorld>,
    pub store_relative_to_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub base_name: FString,
    pub generated_transient_texture: UPtr<crate::bindings::engine::UTexture2D>,
    pub full_asset_path: FString,
}
#[repr(C, align(8))]
pub struct FCreateTextureObjectResult {
    pub result_code: ECreateModelingObjectResult,
    pub new_asset: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FCreateMaterialObjectParams {
    pub target_world: UPtr<crate::bindings::engine::UWorld>,
    pub store_relative_to_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub base_name: FString,
    pub material_to_duplicate: UPtr<crate::bindings::engine::UMaterialInterface>,
}
#[repr(C, align(8))]
pub struct FCreateMaterialObjectResult {
    pub result_code: ECreateModelingObjectResult,
    pub new_asset: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(16))]
pub struct FCreateActorParams {
    pub target_world: UPtr<crate::bindings::engine::UWorld>,
    pub base_name: FString,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub template_actor_deprecated: UPtr<crate::bindings::engine::AActor>,
    pub template_asset: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FCreateActorResult {
    pub result_code: ECreateModelingObjectResult,
    pub new_actor: UPtr<crate::bindings::engine::AActor>,
}
#[repr(C, align(8))]
pub struct FCreateComponentParams {
    pub host_actor: UPtr<crate::bindings::engine::AActor>,
    pub component_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub base_name: FString,
    pub b_set_as_root: bool,
    pub b_transact: bool,
}
#[repr(C, align(8))]
pub struct FCreateComponentResult {
    pub result_code: ECreateModelingObjectResult,
    pub new_component: UPtr<crate::bindings::engine::UActorComponent>,
}
#[repr(C, align(8))]
pub struct FMeshElementSelectionParams {
    pub selection_fill_color: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
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
    pub target_world: TWeakObjectPtr<crate::bindings::engine::UWorld>,
}
pub struct UBaseCreateFromSelectedTool {
    pub transform_properties: UPtr<UTransformInputsToolProperties>,
    pub output_type_properties: UPtr<UCreateMeshObjectTypeProperties>,
    pub handle_sources_properties: UPtr<UBaseCreateFromSelectedHandleSourceProperties>,
    pub collision_properties: UPtr<UBaseCreateFromSelectedCollisionProperties>,
    pub preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub transform_proxies: TArray<
        UPtr<crate::bindings::interactive_tools_framework::UTransformProxy>,
    >,
    pub transform_gizmos: TArray<
        UPtr<crate::bindings::interactive_tools_framework::UCombinedTransformGizmo>,
    >,
}
pub struct USingleTargetWithSelectionToolBuilder {}
pub struct UBaseMeshProcessingToolBuilder {}
pub struct USingleTargetWithSelectionTool {
    pub target_world: TWeakObjectPtr<crate::bindings::engine::UWorld>,
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
    pub target_world: TWeakObjectPtr<crate::bindings::engine::UWorld>,
    pub geometry_selection_viz_properties: UPtr<
        UGeometrySelectionVisualizationProperties,
    >,
    pub geometry_selection_viz: UPtr<UPreviewGeometry>,
}
pub struct USingleSelectionMeshEditingToolBuilder {}
pub struct USingleSelectionMeshEditingTool {
    pub target_world: TWeakObjectPtr<crate::bindings::engine::UWorld>,
}
pub struct UDynamicMeshReplacementChangeTarget {}
pub struct UOctreeDynamicMeshComponent {
    pub mesh_object: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
}
pub struct ULineSetComponent {
    pub line_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub bounds: crate::bindings::core_u_object::FBoxSphereBounds,
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
    pub wireframe_color: crate::bindings::core_u_object::FColor,
    pub boundary_edge_color: crate::bindings::core_u_object::FColor,
    pub uv_seam_color: crate::bindings::core_u_object::FColor,
    pub normal_seam_color: crate::bindings::core_u_object::FColor,
    pub tangent_seam_color: crate::bindings::core_u_object::FColor,
    pub color_seam_color: crate::bindings::core_u_object::FColor,
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
    pub wireframe_color: crate::bindings::core_u_object::FColor,
    pub wireframe_thickness: f32,
    pub b_enable_boundary_edges: bool,
    pub boundary_edge_color: crate::bindings::core_u_object::FColor,
    pub boundary_edge_thickness: f32,
    pub b_enable_uv_seams: bool,
    pub uv_seam_color: crate::bindings::core_u_object::FColor,
    pub uv_seam_thickness: f32,
    pub b_enable_normal_seams: bool,
    pub normal_seam_color: crate::bindings::core_u_object::FColor,
    pub normal_seam_thickness: f32,
    pub b_enable_tangent_seams: bool,
    pub tangent_seam_color: crate::bindings::core_u_object::FColor,
    pub tangent_seam_thickness: f32,
    pub b_enable_color_seams: bool,
    pub color_seam_color: crate::bindings::core_u_object::FColor,
    pub color_seam_thickness: f32,
    pub line_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub local_bounds: crate::bindings::core_u_object::FBoxSphereBounds,
}
pub struct UPointSetComponent {
    pub point_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub bounds: crate::bindings::core_u_object::FBoxSphereBounds,
    pub b_bounds_dirty: bool,
}
pub struct UPreviewMesh {
    pub b_build_spatial_data_structure: bool,
    pub dynamic_mesh_component: UPtr<
        crate::bindings::geometry_framework::UDynamicMeshComponent,
    >,
}
pub struct UPolyEditPreviewMesh {}
pub struct APreviewGeometryActor {}
pub struct UTriangleSetComponent {
    pub bounds: crate::bindings::core_u_object::FBoxSphereBounds,
    pub b_bounds_dirty: bool,
}
pub struct UUVLayoutPreviewProperties {
    pub b_enabled: bool,
    pub side: EUVLayoutPreviewSide,
    pub scale: f32,
    pub offset: crate::bindings::core_u_object::FVector2D,
    pub b_show_wireframe: bool,
}
pub struct UUVLayoutPreview {
    pub settings: UPtr<UUVLayoutPreviewProperties>,
    pub preview_mesh: UPtr<UPreviewMesh>,
    pub mesh_elements_visualizer: UPtr<UMeshElementsVisualizer>,
    pub triangle_component: UPtr<UTriangleSetComponent>,
    pub b_show_backing_rectangle: bool,
    pub backing_rectangle_material: UPtr<crate::bindings::engine::UMaterialInterface>,
}
pub struct UCollectSurfacePathMechanic {}
pub struct UCollisionPrimitivesMechanic {
    pub preview_geometry: UPtr<UPreviewGeometry>,
    pub drawn_primitive_edges: UPtr<ULineSetComponent>,
    pub translate_transform_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub sphere_transform_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub box_transform_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub capsule_transform_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub full_transform_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub current_active_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub translate_transform_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
    pub sphere_transform_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
    pub box_transform_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
    pub capsule_transform_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
    pub full_transform_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
    pub box_interval_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UIntervalGizmo,
    >,
    pub box_x_interval_source: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoLocalFloatParameterSource,
    >,
    pub box_y_interval_source: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoLocalFloatParameterSource,
    >,
    pub box_z_interval_source: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoLocalFloatParameterSource,
    >,
    pub capsule_interval_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UIntervalGizmo,
    >,
    pub capsule_radius_interval_source: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoLocalFloatParameterSource,
    >,
    pub capsule_length_interval_source: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoLocalFloatParameterSource,
    >,
    pub marquee_mechanic: UPtr<URectangleMarqueeMechanic>,
}
pub struct UConstructionPlaneMechanic {
    pub plane_transform_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
    pub plane_transform_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub click_to_set_plane_behavior: UPtr<
        crate::bindings::interactive_tools_framework::USingleClickInputBehavior,
    >,
    pub middle_click_to_set_gizmo_behavior: UPtr<
        crate::bindings::interactive_tools_framework::ULocalSingleClickInputBehavior,
    >,
}
pub struct UCurveControlPointsMechanic {
    pub click_behavior: UPtr<
        crate::bindings::interactive_tools_framework::USingleClickInputBehavior,
    >,
    pub hover_behavior: UPtr<
        crate::bindings::interactive_tools_framework::UMouseHoverBehavior,
    >,
    pub preview_geometry_actor: UPtr<APreviewGeometryActor>,
    pub drawn_control_points: UPtr<UPointSetComponent>,
    pub drawn_control_segments: UPtr<ULineSetComponent>,
    pub preview_point: UPtr<UPointSetComponent>,
    pub preview_segment: UPtr<ULineSetComponent>,
    pub point_transform_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub point_transform_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
}
pub struct UDragAlignmentMechanic {}
pub struct UDragAlignmentInteraction {}
pub struct ULatticeControlPointsMechanic {
    pub preview_geometry_actor: UPtr<APreviewGeometryActor>,
    pub drawn_control_points: UPtr<UPointSetComponent>,
    pub drawn_lattice_edges: UPtr<ULineSetComponent>,
    pub point_transform_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub point_transform_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
    pub marquee_mechanic: UPtr<URectangleMarqueeMechanic>,
}
pub struct UPlaneDistanceFromHitMechanic {}
pub struct UPolyLassoMarqueeMechanic {
    pub spacing_tolerance: f32,
    pub line_thickness: f32,
    pub line_color: crate::bindings::core_u_object::FLinearColor,
    pub closed_color: crate::bindings::core_u_object::FLinearColor,
    pub b_enable_freehand_polygons: bool,
    pub b_enable_multi_click_polygons: bool,
    pub click_drag_behavior: UPtr<
        crate::bindings::interactive_tools_framework::UClickDragInputBehavior,
    >,
    pub hover_behavior: UPtr<
        crate::bindings::interactive_tools_framework::UMouseHoverBehavior,
    >,
}
pub struct URectangleMarqueeMechanic {
    pub b_use_external_click_drag_behavior: bool,
    pub b_use_external_update_camera_state: bool,
    pub on_drag_rectangle_changed_deferred_threshold: f64,
    pub click_drag_behavior: UPtr<
        crate::bindings::interactive_tools_framework::UClickDragInputBehavior,
    >,
}
pub struct URectangleMarqueeInteraction {}
pub struct USpaceCurveDeformationMechanicPropertySet {
    pub transform_mode: ESpaceCurveControlPointTransformMode,
    pub transform_origin: ESpaceCurveControlPointOriginMode,
    pub softness: f32,
    pub soft_falloff: ESpaceCurveControlPointFalloffType,
}
pub struct USpaceCurveDeformationMechanic {
    pub click_behavior: UPtr<
        crate::bindings::interactive_tools_framework::USingleClickInputBehavior,
    >,
    pub hover_behavior: UPtr<
        crate::bindings::interactive_tools_framework::UMouseHoverBehavior,
    >,
    pub transform_properties: UPtr<USpaceCurveDeformationMechanicPropertySet>,
    pub preview_geometry_actor: UPtr<APreviewGeometryActor>,
    pub render_points: UPtr<UPointSetComponent>,
    pub render_segments: UPtr<ULineSetComponent>,
    pub point_transform_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub point_transform_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
}
pub struct USpatialCurveDistanceMechanic {}
pub struct UMeshOpPreviewWithBackgroundCompute {
    pub preview_mesh: UPtr<UPreviewMesh>,
    pub standard_materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub override_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub working_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub secondary_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub preview_world: TWeakObjectPtr<crate::bindings::engine::UWorld>,
}
pub struct UModelingComponentsSettings {
    pub b_enable_ray_tracing_while_editing: bool,
    pub b_enable_ray_tracing: bool,
    pub b_generate_lightmap_u_vs: bool,
    pub b_enable_collision: bool,
    pub collision_mode: crate::bindings::physics_core::ECollisionTraceFlag,
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
    pub volume_type: TSubclassOf<crate::bindings::engine::AVolume>,
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
    pub face_color: crate::bindings::core_u_object::FColor,
    pub line_color: crate::bindings::core_u_object::FColor,
    pub point_color: crate::bindings::core_u_object::FColor,
    pub triangle_roi_border_color: crate::bindings::core_u_object::FColor,
    pub triangle_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub line_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub point_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub triangle_material_showing_hidden: UPtr<
        crate::bindings::engine::UMaterialInterface,
    >,
    pub line_material_showing_hidden: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub point_material_showing_hidden: UPtr<crate::bindings::engine::UMaterialInterface>,
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
    pub hover_behavior: UPtr<
        crate::bindings::interactive_tools_framework::UMouseHoverBehavior,
    >,
    pub click_or_drag_behavior: UPtr<
        crate::bindings::interactive_tools_framework::USingleClickOrDragInputBehavior,
    >,
    pub marquee_mechanic: UPtr<URectangleMarqueeMechanic>,
    pub marquee_selection_update_type: EMarqueeSelectionUpdateType,
    pub preview_geometry_actor: UPtr<APreviewGeometryActor>,
    pub drawn_triangle_set_component: UPtr<UTriangleSetComponent>,
    pub highlighted_face_material: UPtr<crate::bindings::engine::UMaterialInterface>,
}
pub struct UBoundarySelectionMechanic {}
pub struct UGeometrySelectionManager {
    pub selection_arguments: UPtr<UGeometrySelectionEditCommandArguments>,
    pub tools_context: UPtr<
        crate::bindings::interactive_tools_framework::UInteractiveToolsContext,
    >,
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
    pub parent_context: UPtr<
        crate::bindings::interactive_tools_framework::UInteractiveToolsContext,
    >,
}
pub struct UToolHostCustomizationAPI {}
pub struct IToolHostCustomizationAPI {}
pub struct UMultiTransformer {
    pub gizmo_manager: UPtr<
        crate::bindings::interactive_tools_framework::UInteractiveGizmoManager,
    >,
    pub transform_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
    pub transform_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub drag_alignment_mechanic: UPtr<UDragAlignmentMechanic>,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECreateObjectTypeHint(pub u8);
impl ECreateObjectTypeHint {
    pub const UNDEFINED: ECreateObjectTypeHint = ECreateObjectTypeHint(0);
    pub const STATIC_MESH: ECreateObjectTypeHint = ECreateObjectTypeHint(1);
    pub const VOLUME: ECreateObjectTypeHint = ECreateObjectTypeHint(2);
    pub const DYNAMIC_MESH_ACTOR: ECreateObjectTypeHint = ECreateObjectTypeHint(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECreateModelingObjectResult(pub u8);
impl ECreateModelingObjectResult {
    pub const OK: ECreateModelingObjectResult = ECreateModelingObjectResult(0);
    pub const CANCELLED: ECreateModelingObjectResult = ECreateModelingObjectResult(1);
    pub const FAILED_UNKNOWN: ECreateModelingObjectResult = ECreateModelingObjectResult(
        2,
    );
    pub const FAILED_NO_API_FOUND: ECreateModelingObjectResult = ECreateModelingObjectResult(
        3,
    );
    pub const FAILED_INVALID_WORLD: ECreateModelingObjectResult = ECreateModelingObjectResult(
        4,
    );
    pub const FAILED_INVALID_MESH: ECreateModelingObjectResult = ECreateModelingObjectResult(
        5,
    );
    pub const FAILED_INVALID_TEXTURE: ECreateModelingObjectResult = ECreateModelingObjectResult(
        6,
    );
    pub const FAILED_ASSET_CREATION_FAILED: ECreateModelingObjectResult = ECreateModelingObjectResult(
        7,
    );
    pub const FAILED_ACTOR_CREATION_FAILED: ECreateModelingObjectResult = ECreateModelingObjectResult(
        8,
    );
    pub const FAILED_INVALID_MATERIAL: ECreateModelingObjectResult = ECreateModelingObjectResult(
        9,
    );
    pub const FAILED_INVALID_ACTOR: ECreateModelingObjectResult = ECreateModelingObjectResult(
        10,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EHandleSourcesMethod(pub u8);
impl EHandleSourcesMethod {
    pub const DELETE_SOURCES: EHandleSourcesMethod = EHandleSourcesMethod(0);
    pub const HIDE_SOURCES: EHandleSourcesMethod = EHandleSourcesMethod(1);
    pub const KEEP_SOURCES: EHandleSourcesMethod = EHandleSourcesMethod(2);
    pub const KEEP_FIRST_SOURCE: EHandleSourcesMethod = EHandleSourcesMethod(3);
    pub const KEEP_LAST_SOURCE: EHandleSourcesMethod = EHandleSourcesMethod(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBaseCreateFromSelectedTargetType(pub i32);
impl EBaseCreateFromSelectedTargetType {
    pub const NEW_OBJECT: EBaseCreateFromSelectedTargetType = EBaseCreateFromSelectedTargetType(
        0,
    );
    pub const FIRST_INPUT_OBJECT: EBaseCreateFromSelectedTargetType = EBaseCreateFromSelectedTargetType(
        1,
    );
    pub const LAST_INPUT_OBJECT: EBaseCreateFromSelectedTargetType = EBaseCreateFromSelectedTargetType(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EUVLayoutPreviewSide(pub i32);
impl EUVLayoutPreviewSide {
    pub const LEFT: EUVLayoutPreviewSide = EUVLayoutPreviewSide(0);
    pub const RIGHT: EUVLayoutPreviewSide = EUVLayoutPreviewSide(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESpaceCurveControlPointTransformMode(pub i32);
impl ESpaceCurveControlPointTransformMode {
    pub const SHARED: ESpaceCurveControlPointTransformMode = ESpaceCurveControlPointTransformMode(
        0,
    );
    pub const PER_VERTEX: ESpaceCurveControlPointTransformMode = ESpaceCurveControlPointTransformMode(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESpaceCurveControlPointOriginMode(pub i32);
impl ESpaceCurveControlPointOriginMode {
    pub const SHARED: ESpaceCurveControlPointOriginMode = ESpaceCurveControlPointOriginMode(
        0,
    );
    pub const FIRST: ESpaceCurveControlPointOriginMode = ESpaceCurveControlPointOriginMode(
        1,
    );
    pub const LAST: ESpaceCurveControlPointOriginMode = ESpaceCurveControlPointOriginMode(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESpaceCurveControlPointFalloffType(pub i32);
impl ESpaceCurveControlPointFalloffType {
    pub const LINEAR: ESpaceCurveControlPointFalloffType = ESpaceCurveControlPointFalloffType(
        0,
    );
    pub const SMOOTH: ESpaceCurveControlPointFalloffType = ESpaceCurveControlPointFalloffType(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EModelingComponentsPlaneVisualizationMode(pub u8);
impl EModelingComponentsPlaneVisualizationMode {
    pub const SIMPLE_GRID: EModelingComponentsPlaneVisualizationMode = EModelingComponentsPlaneVisualizationMode(
        0,
    );
    pub const HIERARCHICAL_GRID: EModelingComponentsPlaneVisualizationMode = EModelingComponentsPlaneVisualizationMode(
        1,
    );
    pub const FIXED_SCREEN_AREA_GRID: EModelingComponentsPlaneVisualizationMode = EModelingComponentsPlaneVisualizationMode(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGeometrySelectionElementType(pub u8);
impl EGeometrySelectionElementType {
    pub const VERTEX: EGeometrySelectionElementType = EGeometrySelectionElementType(1);
    pub const EDGE: EGeometrySelectionElementType = EGeometrySelectionElementType(2);
    pub const FACE: EGeometrySelectionElementType = EGeometrySelectionElementType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGeometrySelectionTopologyType(pub u8);
impl EGeometrySelectionTopologyType {
    pub const TRIANGLE: EGeometrySelectionTopologyType = EGeometrySelectionTopologyType(
        1,
    );
    pub const POLYGROUP: EGeometrySelectionTopologyType = EGeometrySelectionTopologyType(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMarqueeSelectionUpdateType(pub i32);
impl EMarqueeSelectionUpdateType {
    pub const ON_DRAG: EMarqueeSelectionUpdateType = EMarqueeSelectionUpdateType(0);
    pub const ON_TICK_AND_RELEASE: EMarqueeSelectionUpdateType = EMarqueeSelectionUpdateType(
        1,
    );
    pub const ON_RELEASE: EMarqueeSelectionUpdateType = EMarqueeSelectionUpdateType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELatticeInterpolationType(pub u8);
impl ELatticeInterpolationType {
    pub const LINEAR: ELatticeInterpolationType = ELatticeInterpolationType(0);
    pub const CUBIC: ELatticeInterpolationType = ELatticeInterpolationType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBakeTextureResolution(pub i32);
impl EBakeTextureResolution {
    pub const RESOLUTION16: EBakeTextureResolution = EBakeTextureResolution(16);
    pub const RESOLUTION32: EBakeTextureResolution = EBakeTextureResolution(32);
    pub const RESOLUTION64: EBakeTextureResolution = EBakeTextureResolution(64);
    pub const RESOLUTION128: EBakeTextureResolution = EBakeTextureResolution(128);
    pub const RESOLUTION256: EBakeTextureResolution = EBakeTextureResolution(256);
    pub const RESOLUTION512: EBakeTextureResolution = EBakeTextureResolution(512);
    pub const RESOLUTION1024: EBakeTextureResolution = EBakeTextureResolution(1024);
    pub const RESOLUTION2048: EBakeTextureResolution = EBakeTextureResolution(2048);
    pub const RESOLUTION4096: EBakeTextureResolution = EBakeTextureResolution(4096);
    pub const RESOLUTION8192: EBakeTextureResolution = EBakeTextureResolution(8192);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBakeTextureBitDepth(pub i32);
impl EBakeTextureBitDepth {
    pub const CHANNEL_BITS8: EBakeTextureBitDepth = EBakeTextureBitDepth(0);
    pub const CHANNEL_BITS16: EBakeTextureBitDepth = EBakeTextureBitDepth(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBakeTextureSamplesPerPixel(pub i32);
impl EBakeTextureSamplesPerPixel {
    pub const SAMPLE1: EBakeTextureSamplesPerPixel = EBakeTextureSamplesPerPixel(1);
    pub const SAMPLE4: EBakeTextureSamplesPerPixel = EBakeTextureSamplesPerPixel(4);
    pub const SAMPLE16: EBakeTextureSamplesPerPixel = EBakeTextureSamplesPerPixel(16);
    pub const SAMPLE64: EBakeTextureSamplesPerPixel = EBakeTextureSamplesPerPixel(64);
    pub const SAMPLE256: EBakeTextureSamplesPerPixel = EBakeTextureSamplesPerPixel(256);
}
