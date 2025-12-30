#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FBakeMultiMeshDetailProperties {
    pub source_mesh: UPtr<UStaticMesh>,
    pub source_texture: UPtr<UTexture2D>,
    pub source_texture_uv_layer: i32,
}
#[repr(C, align(8))]
pub struct FEditPivotTarget {
    pub transform_proxy: UPtr<UTransformProxy>,
    pub transform_gizmo: UPtr<UCombinedTransformGizmo>,
}
#[repr(C, align(16))]
pub struct FPhysicsSphereData {
    pub radius: f32,
    pub transform: FTransform,
    pub element: FKShapeElem,
}
#[repr(C, align(16))]
pub struct FPhysicsBoxData {
    pub dimensions: FVector,
    pub transform: FTransform,
    pub element: FKShapeElem,
}
#[repr(C, align(16))]
pub struct FPhysicsCapsuleData {
    pub radius: f32,
    pub length: f32,
    pub transform: FTransform,
    pub element: FKShapeElem,
}
#[repr(C, align(8))]
pub struct FPhysicsConvexData {
    pub num_vertices: i32,
    pub num_faces: i32,
    pub element: FKShapeElem,
}
#[repr(C, align(8))]
pub struct FPhysicsLevelSetData {
    pub element: FKShapeElem,
}
#[repr(C, align(8))]
pub struct FTransformMeshesTarget {
    pub transform_proxy: UPtr<UTransformProxy>,
    pub transform_gizmo: UPtr<UCombinedTransformGizmo>,
}
pub struct UBakeInputMeshProperties {
    pub target_static_mesh: UPtr<UStaticMesh>,
    pub target_skeletal_mesh: UPtr<USkeletalMesh>,
    pub target_dynamic_mesh: UPtr<AActor>,
    pub target_uv_layer: FString,
    pub b_has_target_uv_layer: bool,
    pub source_static_mesh: UPtr<UStaticMesh>,
    pub source_skeletal_mesh: UPtr<USkeletalMesh>,
    pub source_dynamic_mesh: UPtr<AActor>,
    pub b_hide_source_mesh: bool,
    pub source_normal_map: UPtr<UTexture2D>,
    pub source_normal_map_uv_layer: FString,
    pub source_normal_space: EBakeNormalSpace,
    pub b_has_source_normal_map: bool,
    pub projection_distance: f32,
    pub b_projection_in_world_space: bool,
    pub target_uv_layer_names_list: TArray<FString>,
    pub source_uv_layer_names_list: TArray<FString>,
}
pub struct UBakeNormalMapToolProperties {}
pub struct UBakeOcclusionMapToolProperties {
    pub occlusion_rays: i32,
    pub max_distance: f32,
    pub spread_angle: f32,
    pub bias_angle: f32,
    pub normal_space: EBakeNormalSpace,
}
pub struct UBakeCurvatureMapToolProperties {
    pub curvature_type: EBakeCurvatureTypeMode,
    pub color_mapping: EBakeCurvatureColorMode,
    pub color_range_multiplier: f32,
    pub min_range_multiplier: f32,
    pub clamping: EBakeCurvatureClampMode,
}
pub struct UBakeUVShellMapToolProperties {
    pub uv_layer: i32,
    pub wireframe_thickness: f32,
    pub wireframe_color: FLinearColor,
    pub shell_color: FLinearColor,
    pub background_color: FLinearColor,
}
pub struct UBakeHeightMapToolProperties {
    pub height_range_mode: EBakeHeightRangeMode,
    pub inner_distance: f32,
    pub outer_distance: f32,
    pub inner_bounds_distance: f32,
    pub outer_bounds_distance: f32,
}
pub struct UBakeTexture2DProperties {
    pub source_texture: UPtr<UTexture2D>,
    pub uv_layer: FString,
    pub uv_layer_names_list: TArray<FString>,
}
pub struct UBakeMultiTexture2DProperties {
    pub material_id_source_textures: TArray<UPtr<UTexture2D>>,
    pub uv_layer: FString,
    pub uv_layer_names_list: TArray<FString>,
    pub all_source_textures: TArray<UPtr<UTexture2D>>,
}
pub struct UBakeVisualizationProperties {
    pub b_preview_as_material: bool,
    pub brightness: f32,
    pub ao_multiplier: f32,
}
pub struct UAddPatchToolBuilder {}
pub struct UAddPatchToolProperties {
    pub width: f32,
    pub rotation: f32,
    pub subdivisions: i32,
    pub shift: f32,
}
pub struct UAddPatchTool {
    pub shape_settings: UPtr<UAddPatchToolProperties>,
    pub material_properties: UPtr<UNewMeshMaterialProperties>,
    pub preview_mesh: UPtr<UPreviewMesh>,
}
pub struct UAlignObjectsToolBuilder {}
pub struct UAlignObjectsToolProperties {
    pub align_type: EAlignObjectsAlignTypes,
    pub align_to: EAlignObjectsAlignToOptions,
    pub box_position: EAlignObjectsBoxPoint,
    pub b_align_x: bool,
    pub b_align_y: bool,
    pub b_align_z: bool,
}
pub struct UAlignObjectsTool {
    pub align_props: UPtr<UAlignObjectsToolProperties>,
}
pub struct UBakeMeshAttributeMapsToolBuilder {}
pub struct UBakeMeshAttributeMapsToolProperties {
    pub map_types: i32,
    pub map_preview: FString,
    pub resolution: EBakeTextureResolution,
    pub bit_depth: EBakeTextureBitDepth,
    pub samples_per_pixel: EBakeTextureSamplesPerPixel,
    pub sample_filter_mask: UPtr<UTexture2D>,
    pub map_preview_names_list: TArray<FString>,
}
pub struct UBakeMeshAttributeTool {
    pub occlusion_settings: UPtr<UBakeOcclusionMapToolProperties>,
    pub curvature_settings: UPtr<UBakeCurvatureMapToolProperties>,
    pub texture_settings: UPtr<UBakeTexture2DProperties>,
    pub multi_texture_settings: UPtr<UBakeMultiTexture2DProperties>,
    pub height_settings: UPtr<UBakeHeightMapToolProperties>,
    pub working_preview_material: UPtr<UMaterialInstanceDynamic>,
    pub error_preview_material: UPtr<UMaterialInstanceDynamic>,
}
pub struct UBakeMeshAttributeMapsToolBase {
    pub visualization_props: UPtr<UBakeVisualizationProperties>,
    pub preview_mesh: UPtr<UPreviewMesh>,
    pub preview_material: UPtr<UMaterialInstanceDynamic>,
    pub bent_normal_preview_material: UPtr<UMaterialInstanceDynamic>,
    pub cached_maps: TMap<EBakeMapType, UPtr<UTexture2D>>,
    pub empty_normal_map: UPtr<UTexture2D>,
    pub empty_color_map_black: UPtr<UTexture2D>,
    pub empty_color_map_white: UPtr<UTexture2D>,
}
pub struct UBakeMeshAttributeMapsTool {
    pub input_mesh_settings: UPtr<UBakeInputMeshProperties>,
    pub settings: UPtr<UBakeMeshAttributeMapsToolProperties>,
    pub result_settings: UPtr<UBakeMeshAttributeMapsResultToolProperties>,
    pub uv_shell_settings: UPtr<UBakeUVShellMapToolProperties>,
}
pub struct UBakeMeshAttributeMapsResultToolProperties {
    pub result: TMap<EBakeMapType, UPtr<UTexture2D>>,
}
pub struct UBakeMeshAttributeVertexToolBuilder {}
pub struct UBakeMeshAttributeVertexToolProperties {
    pub output_mode: EBakeVertexOutput,
    pub output_type: i32,
    pub output_type_r: i32,
    pub output_type_g: i32,
    pub output_type_b: i32,
    pub output_type_a: i32,
    pub preview_mode: EBakeVertexChannel,
    pub topology_mode: EBakeVertexTopology,
    pub b_split_at_normal_seams: bool,
    pub b_split_at_uv_seams: bool,
}
pub struct UBakeMeshAttributeVertexTool {
    pub input_mesh_settings: UPtr<UBakeInputMeshProperties>,
    pub settings: UPtr<UBakeMeshAttributeVertexToolProperties>,
    pub preview_mesh: UPtr<UPreviewMesh>,
    pub preview_material: UPtr<UMaterialInstanceDynamic>,
    pub preview_alpha_material: UPtr<UMaterialInstanceDynamic>,
}
pub struct UBakeMultiMeshAttributeMapsToolBuilder {}
pub struct UBakeMultiMeshAttributeMapsToolProperties {
    pub map_types: i32,
    pub map_preview: FString,
    pub resolution: EBakeTextureResolution,
    pub bit_depth: EBakeTextureBitDepth,
    pub samples_per_pixel: EBakeTextureSamplesPerPixel,
    pub sample_filter_mask: UPtr<UTexture2D>,
    pub map_preview_names_list: TArray<FString>,
}
pub struct UBakeMultiMeshInputToolProperties {
    pub target_static_mesh: UPtr<UStaticMesh>,
    pub target_skeletal_mesh: UPtr<USkeletalMesh>,
    pub target_dynamic_mesh: UPtr<AActor>,
    pub target_uv_layer: FString,
    pub source_meshes: TArray<FBakeMultiMeshDetailProperties>,
    pub projection_distance: f32,
    pub target_uv_layer_names_list: TArray<FString>,
}
pub struct UBakeMultiMeshAttributeMapsTool {
    pub settings: UPtr<UBakeMultiMeshAttributeMapsToolProperties>,
    pub input_mesh_settings: UPtr<UBakeMultiMeshInputToolProperties>,
    pub result_settings: UPtr<UBakeMeshAttributeMapsResultToolProperties>,
}
pub struct UBakeTransformToolBuilder {}
pub struct UBakeTransformToolProperties {
    pub b_apply_to_all_lo_ds: bool,
    pub b_bake_rotation: bool,
    pub bake_scale: EBakeScaleMethod,
    pub b_recenter_pivot: bool,
    pub b_allow_no_scale: bool,
}
pub struct UBakeTransformTool {
    pub basic_properties: UPtr<UBakeTransformToolProperties>,
}
pub struct UConvertMeshesToolBuilder {}
pub struct UConvertMeshesToolProperties {
    pub b_transfer_materials: bool,
    pub b_show_transfer_materials: bool,
    pub b_transfer_collision: bool,
}
pub struct UConvertMeshesTool {
    pub basic_properties: UPtr<UConvertMeshesToolProperties>,
    pub output_type_properties: UPtr<UCreateMeshObjectTypeProperties>,
    pub inputs: TArray<TWeakObjectPtr<UPrimitiveComponent>>,
}
pub struct UCubeGridToolBuilder {}
pub struct UCubeGridToolProperties {
    pub grid_frame_origin: FVector,
    pub grid_frame_orientation: FRotator,
    pub b_show_grid: bool,
    pub b_show_gizmo: bool,
    pub grid_power: u8,
    pub current_block_size: f64,
    pub blocks_per_step: i32,
    pub b_power_of_two_block_sizes: bool,
    pub block_base_size: f64,
    pub b_crosswise_diagonal: bool,
    pub b_keep_side_groups: bool,
    pub b_show_selection_measurements: bool,
    pub plane_tolerance: f64,
    pub b_hit_unrelated_geometry: bool,
    pub b_hit_grid_ground_plane_if_closer: bool,
    pub face_selection_mode: ECubeGridToolFaceSelectionMode,
    pub toggle_corner_mode: FString,
    pub push_pull: FString,
    pub resize_grid: FString,
    pub flip_selection: FString,
    pub grid_gizmo: FString,
    pub quick_shift_gizmo: FString,
    pub align_gizmo: FString,
    pub b_in_corner_mode: bool,
    pub b_allowed_to_edit_grid: bool,
}
pub struct UCubeGridToolActions {
    pub grid_source_actor: UPtr<AActor>,
}
pub struct UCubeGridTool {
    pub grid_gizmo: UPtr<UCombinedTransformGizmo>,
    pub grid_gizmo_alignment_mechanic: UPtr<UDragAlignmentMechanic>,
    pub grid_gizmo_transform_proxy: UPtr<UTransformProxy>,
    pub line_sets: UPtr<UPreviewGeometry>,
    pub click_drag_behavior: UPtr<UClickDragInputBehavior>,
    pub hover_behavior: UPtr<UMouseHoverBehavior>,
    pub ctrl_middle_click_behavior: UPtr<ULocalSingleClickInputBehavior>,
    pub middle_click_drag_behavior: UPtr<ULocalClickDragInputBehavior>,
    pub settings: UPtr<UCubeGridToolProperties>,
    pub tool_actions: UPtr<UCubeGridToolActions>,
    pub material_properties: UPtr<UNewMeshMaterialProperties>,
    pub output_type_properties: UPtr<UCreateMeshObjectTypeProperties>,
    pub target: UPtr<UToolTarget>,
    pub preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
}
pub struct UDrawPolyPathToolBuilder {}
pub struct UDrawPolyPathProperties {
    pub width_mode: EDrawPolyPathWidthMode,
    pub width: f32,
    pub b_rounded_corners: bool,
    pub radius_mode: EDrawPolyPathRadiusMode,
    pub corner_radius: f32,
    pub radial_slices: i32,
    pub b_single_poly_group: bool,
    pub extrude_mode: EDrawPolyPathExtrudeMode,
    pub extrude_height: f32,
    pub ramp_start_ratio: f32,
}
pub struct UDrawPolyPathExtrudeProperties {
    pub direction: EDrawPolyPathExtrudeDirection,
}
pub struct UDrawPolyPathTool {
    pub output_type_properties: UPtr<UCreateMeshObjectTypeProperties>,
    pub transform_props: UPtr<UDrawPolyPathProperties>,
    pub extrude_properties: UPtr<UDrawPolyPathExtrudeProperties>,
    pub material_properties: UPtr<UNewMeshMaterialProperties>,
    pub plane_mechanic: UPtr<UConstructionPlaneMechanic>,
    pub edit_preview: UPtr<UPolyEditPreviewMesh>,
    pub extrude_height_mechanic: UPtr<UPlaneDistanceFromHitMechanic>,
    pub curve_dist_mechanic: UPtr<USpatialCurveDistanceMechanic>,
    pub surface_path_mechanic: UPtr<UCollectSurfacePathMechanic>,
}
pub struct UEditNormalsToolBuilder {}
pub struct UEditNormalsToolProperties {
    pub b_recompute_normals: bool,
    pub normal_calculation_method: ENormalCalculationMethod,
    pub b_fix_inconsistent_normals: bool,
    pub b_invert_normals: bool,
    pub split_normal_method: ESplitNormalMethod,
    pub sharp_edge_angle_threshold: f32,
    pub b_allow_sharp_vertices: bool,
    pub b_tool_has_selection: bool,
}
pub struct UEditNormalsOperatorFactory {
    pub tool: UPtr<UEditNormalsTool>,
}
pub struct UEditNormalsTool {
    pub basic_properties: UPtr<UEditNormalsToolProperties>,
    pub polygroup_layer_properties: UPtr<UPolygroupLayersProperties>,
    pub previews: TArray<UPtr<UMeshOpPreviewWithBackgroundCompute>>,
    pub geometry_selection_viz_properties: UPtr<
        UGeometrySelectionVisualizationProperties,
    >,
    pub geometry_selection_viz: UPtr<UPreviewGeometry>,
}
pub struct UEditPivotToolBuilder {}
pub struct UEditPivotToolProperties {
    pub b_apply_to_all_lo_ds: bool,
    pub b_snap_drag_position: bool,
    pub b_snap_drag_rotation: bool,
    pub rotation_mode: EEditPivotSnapDragRotationMode,
}
pub struct UEditPivotToolActionPropertySet {
    pub b_use_world_box: bool,
}
pub struct UEditPivotTool {
    pub transform_props: UPtr<UEditPivotToolProperties>,
    pub edit_pivot_actions: UPtr<UEditPivotToolActionPropertySet>,
    pub active_gizmos: TArray<FEditPivotTarget>,
    pub drag_alignment_mechanic: UPtr<UDragAlignmentMechanic>,
}
pub struct UEditUVIslandsToolBuilder {}
pub struct UEditUVIslandsTool {
    pub material_settings: UPtr<UExistingMeshMaterialProperties>,
    pub checker_material: UPtr<UMaterialInstanceDynamic>,
    pub preview_mesh_actor: UPtr<AInternalToolFrameworkActor>,
    pub dynamic_mesh_component: UPtr<UDynamicMeshComponent>,
    pub selection_mechanic: UPtr<UPolygonSelectionMechanic>,
    pub transform_gizmo: UPtr<UCombinedTransformGizmo>,
    pub transform_proxy: UPtr<UTransformProxy>,
}
pub struct UExtractSplineToolProperties {
    pub extraction_mode: EExtractSplineMode,
}
pub struct UExtractSplineTool {
    pub settings: UPtr<UExtractSplineToolProperties>,
    pub plane_mechanic: UPtr<UConstructionPlaneMechanic>,
    pub selection_mechanic: UPtr<UPolygonSelectionMechanic>,
    pub preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub factory: UPtr<UGenerateCrossSectionOpFactory>,
    pub cutline_geometry: UPtr<UPreviewGeometry>,
}
pub struct UExtractSplineToolBuilder {}
pub struct UMeshBoundaryToolBase {
    pub selection_mechanic: UPtr<UPolygonSelectionMechanic>,
}
pub struct UMeshInspectorToolBuilder {}
pub struct UMeshInspectorProperties {
    pub b_wireframe: bool,
    pub b_boundary_edges: bool,
    pub b_bowtie_vertices: bool,
    pub b_polygon_borders: bool,
    pub b_uv_seams: bool,
    pub b_uv_bowties: bool,
    pub b_missing_u_vs: bool,
    pub b_normal_seams: bool,
    pub b_tangent_seams: bool,
    pub b_normal_vectors: bool,
    pub b_tangent_vectors: bool,
    pub b_draw_hidden_edges_and_seams: bool,
    pub normal_length: f32,
    pub tangent_length: f32,
    pub show_indices: EMeshInspectorToolDrawIndexMode,
}
pub struct UMeshInspectorMaterialProperties {
    pub material_mode: EMeshInspectorMaterialMode,
    pub checker_density: f32,
    pub override_material: UPtr<UMaterialInterface>,
    pub uv_channel: FString,
    pub uv_channel_names_list: TArray<FString>,
    pub b_flat_shading: bool,
    pub color: FLinearColor,
    pub opacity: f64,
    pub transparent_material_color: FLinearColor,
    pub b_two_sided: bool,
    pub checker_material: UPtr<UMaterialInstanceDynamic>,
    pub active_custom_material: UPtr<UMaterialInstanceDynamic>,
}
pub struct UMeshInspectorTool {
    pub settings: UPtr<UMeshInspectorProperties>,
    pub polygroup_layer_properties: UPtr<UPolygroupLayersProperties>,
    pub material_settings: UPtr<UMeshInspectorMaterialProperties>,
    pub preview_mesh: UPtr<UPreviewMesh>,
    pub drawn_line_set: UPtr<ULineSetComponent>,
    pub default_material: UPtr<UMaterialInterface>,
}
pub struct UMeshSelectionToolBuilder {}
pub struct UMeshSelectionToolActionPropertySet {}
pub struct UMeshSelectionEditActions {}
pub struct UMeshSelectionMeshEditActions {}
pub struct UMeshSelectionToolProperties {
    pub selection_mode: EMeshSelectionToolPrimaryMode,
    pub angle_tolerance: f32,
    pub b_hit_back_faces: bool,
    pub b_show_points: bool,
    pub face_color_mode: EMeshFacesColorMode,
}
pub struct UMeshSelectionTool {
    pub selection_props: UPtr<UMeshSelectionToolProperties>,
    pub selection_actions: UPtr<UMeshSelectionEditActions>,
    pub edit_actions: UPtr<UMeshSelectionToolActionPropertySet>,
    pub mesh_statistics_properties: UPtr<UMeshStatisticsProperties>,
    pub mesh_elements_display: UPtr<UMeshElementsVisualizer>,
    pub uv_channel_properties: UPtr<UMeshUVChannelProperties>,
    pub polygroup_layer_properties: UPtr<UPolygroupLayersProperties>,
    pub selection: UPtr<UMeshSelectionSet>,
    pub spawned_actors: TArray<UPtr<AActor>>,
}
pub struct UMirrorToolBuilder {}
pub struct UMirrorToolProperties {
    pub operation_mode: EMirrorOperationMode,
    pub b_crop_along_mirror_plane_first: bool,
    pub b_simplify_along_crop: bool,
    pub b_weld_vertices_on_mirror_plane: bool,
    pub weld_vertices_normal_mode: EMeshMirrorWeldNormalMode,
    pub plane_tolerance: f64,
    pub b_allow_bowtie_vertex_creation: bool,
    pub b_show_preview: bool,
    pub write_to: EMirrorSaveMode,
}
pub struct UMirrorOperatorFactory {
    pub mirror_tool: UPtr<UMirrorTool>,
}
pub struct UMirrorToolActionPropertySet {
    pub b_buttons_only_change_orientation: bool,
}
pub struct UMirrorTool {
    pub settings: UPtr<UMirrorToolProperties>,
    pub output_type_properties: UPtr<UCreateMeshObjectTypeProperties>,
    pub handle_sources_properties: UPtr<UOnAcceptHandleSourcesProperties>,
    pub tool_actions: UPtr<UMirrorToolActionPropertySet>,
    pub meshes_to_mirror: TArray<UPtr<UDynamicMeshReplacementChangeTarget>>,
    pub previews: TArray<UPtr<UMeshOpPreviewWithBackgroundCompute>>,
    pub plane_mechanic: UPtr<UConstructionPlaneMechanic>,
}
pub struct UPatternToolBuilder {}
pub struct UPatternToolSettings {
    pub seed: i32,
    pub b_project_elements_down: bool,
    pub projection_offset: f32,
    pub b_hide_sources: bool,
    pub b_use_relative_transforms: bool,
    pub b_randomly_pick_elements: bool,
    pub shape: EPatternToolShape,
    pub single_axis: EPatternToolSingleAxis,
    pub single_plane: EPatternToolSinglePlane,
}
pub struct UPatternTool_BoundingBoxSettings {
    pub b_ignore_transforms: bool,
    pub adjustment: f32,
    pub b_visualize: bool,
}
pub struct UPatternTool_LinearSettings {
    pub spacing_mode: EPatternToolAxisSpacingMode,
    pub count: i32,
    pub step_size: f64,
    pub extent: f64,
    pub b_centered: bool,
}
pub struct UPatternTool_GridSettings {
    pub spacing_x: EPatternToolAxisSpacingMode,
    pub count_x: i32,
    pub step_size_x: f64,
    pub extent_x: f64,
    pub b_centered_x: bool,
    pub spacing_y: EPatternToolAxisSpacingMode,
    pub count_y: i32,
    pub step_size_y: f64,
    pub extent_y: f64,
    pub b_centered_y: bool,
}
pub struct UPatternTool_RadialSettings {
    pub spacing_mode: EPatternToolAxisSpacingMode,
    pub count: i32,
    pub step_size: f64,
    pub radius: f64,
    pub start_angle: f64,
    pub end_angle: f64,
    pub angle_shift: f64,
    pub b_oriented: bool,
}
pub struct UPatternTool_RotationSettings {
    pub b_interpolate: bool,
    pub b_jitter: bool,
    pub start_rotation: FRotator,
    pub end_rotation: FRotator,
    pub jitter: FRotator,
}
pub struct UPatternTool_TranslationSettings {
    pub b_interpolate: bool,
    pub b_jitter: bool,
    pub start_translation: FVector,
    pub end_translation: FVector,
    pub jitter: FVector,
}
pub struct UPatternTool_ScaleSettings {
    pub b_proportional: bool,
    pub b_interpolate: bool,
    pub b_jitter: bool,
    pub start_scale: FVector,
    pub end_scale: FVector,
    pub jitter: FVector,
}
pub struct UPatternTool_OutputSettings {
    pub b_separate_actors: bool,
    pub b_convert_to_dynamic: bool,
    pub b_create_ism_cs: bool,
    pub b_have_static_meshes: bool,
    pub b_enable_create_ism_cs: bool,
}
pub struct UPatternTool {
    pub settings: UPtr<UPatternToolSettings>,
    pub bounding_box_settings: UPtr<UPatternTool_BoundingBoxSettings>,
    pub linear_settings: UPtr<UPatternTool_LinearSettings>,
    pub grid_settings: UPtr<UPatternTool_GridSettings>,
    pub radial_settings: UPtr<UPatternTool_RadialSettings>,
    pub rotation_settings: UPtr<UPatternTool_RotationSettings>,
    pub translation_settings: UPtr<UPatternTool_TranslationSettings>,
    pub scale_settings: UPtr<UPatternTool_ScaleSettings>,
    pub output_settings: UPtr<UPatternTool_OutputSettings>,
    pub pattern_gizmo_proxy: UPtr<UComponentBoundTransformProxy>,
    pub pattern_gizmo: UPtr<UInteractiveGizmo>,
    pub pattern_gizmo_component: UPtr<UPrimitiveComponent>,
    pub drag_alignment_mechanic: UPtr<UDragAlignmentMechanic>,
    pub plane_mechanic: UPtr<UConstructionPlaneMechanic>,
    pub all_components: TSet<UPtr<UPrimitiveComponent>>,
    pub preview_geometry: UPtr<UPreviewGeometry>,
}
pub struct UPhysicsObjectToolPropertySet {
    pub object_name: FString,
    pub collision_type: ECollisionGeometryMode,
    pub spheres: TArray<FPhysicsSphereData>,
    pub boxes: TArray<FPhysicsBoxData>,
    pub capsules: TArray<FPhysicsCapsuleData>,
    pub convexes: TArray<FPhysicsConvexData>,
    pub level_sets: TArray<FPhysicsLevelSetData>,
}
pub struct UCollisionGeometryVisualizationProperties {
    pub b_show_collision: bool,
    pub b_show_solid: bool,
    pub line_thickness: f32,
    pub b_show_hidden: bool,
    pub b_random_colors: bool,
    pub color: FColor,
    pub line_material: UPtr<UMaterialInterface>,
    pub line_material_showing_hidden: UPtr<UMaterialInterface>,
    pub triangle_material: UPtr<UMaterialInterface>,
    pub b_enable_show_collision: bool,
    pub b_enable_show_solid: bool,
}
pub struct UExtractCollisionGeometryToolBuilder {}
pub struct UExtractCollisionToolProperties {
    pub collision_type: EExtractCollisionOutputType,
    pub b_output_separate_meshes: bool,
    pub b_show_preview: bool,
    pub b_show_input_mesh: bool,
    pub b_weld_edges: bool,
}
pub struct UExtractCollisionGeometryTool {
    pub output_type_properties: UPtr<UCreateMeshObjectTypeProperties>,
    pub settings: UPtr<UExtractCollisionToolProperties>,
    pub viz_settings: UPtr<UCollisionGeometryVisualizationProperties>,
    pub object_props: UPtr<UPhysicsObjectToolPropertySet>,
    pub preview_elements: UPtr<UPreviewGeometry>,
    pub preview_mesh: UPtr<UPreviewMesh>,
}
pub struct UPhysicsInspectorToolBuilder {}
pub struct UPhysicsInspectorToolProperties {
    pub b_show_target_mesh: bool,
}
pub struct UPhysicsInspectorTool {
    pub viz_settings: UPtr<UCollisionGeometryVisualizationProperties>,
    pub settings: UPtr<UPhysicsInspectorToolProperties>,
    pub object_data: TArray<UPtr<UPhysicsObjectToolPropertySet>>,
    pub preview_elements: TArray<UPtr<UPreviewGeometry>>,
}
pub struct USetCollisionGeometryToolBuilder {}
pub struct USetCollisionGeometryToolProperties {
    pub geometry_type: ECollisionGeometryType,
    pub b_append_to_existing: bool,
    pub b_use_world_space: bool,
    pub input_mode: ESetCollisionGeometryInputMode,
    pub b_remove_contained: bool,
    pub b_enable_max_count: bool,
    pub max_count: i32,
    pub min_thickness: f32,
    pub b_detect_boxes: bool,
    pub b_detect_spheres: bool,
    pub b_detect_capsules: bool,
    pub b_merge_collision_shapes: bool,
    pub merge_above_count: i32,
    pub b_use_negative_space_in_merge: bool,
    pub b_simplify_hulls: bool,
    pub hull_target_face_count: i32,
    pub b_pre_simplify_to_edge_length: bool,
    pub decomposition_target_edge_length: f64,
    pub decomposition_method: EConvexDecompositionMethod,
    pub b_limit_hulls_per_shape: bool,
    pub max_hulls_per_shape: i32,
    pub convex_decomposition_search_factor: f32,
    pub add_hulls_error_tolerance: f32,
    pub min_part_thickness: f32,
    pub negative_space_tolerance: f64,
    pub negative_space_min_radius: f64,
    pub b_ignore_internal_negative_space: bool,
    pub hull_tolerance: f32,
    pub sweep_axis: EProjectedHullAxis,
    pub level_set_resolution: i32,
    pub set_collision_type: ECollisionGeometryMode,
    pub b_show_target_mesh: bool,
    pub b_using_multiple_inputs: bool,
}
pub struct USetCollisionGeometryTool {
    pub settings: UPtr<USetCollisionGeometryToolProperties>,
    pub polygroup_layer_properties: UPtr<UPolygroupLayersProperties>,
    pub viz_settings: UPtr<UCollisionGeometryVisualizationProperties>,
    pub collision_props: UPtr<UPhysicsObjectToolPropertySet>,
    pub preview_geom: UPtr<UPreviewGeometry>,
    pub geometry_selection_viz_properties: UPtr<
        UGeometrySelectionVisualizationProperties,
    >,
    pub geometry_selection_viz: UPtr<UPreviewGeometry>,
}
pub struct USimpleCollisionEditorToolBuilder {}
pub struct USimpleCollisionEditorToolActionProperties {}
pub struct USimpleCollisionEditorTool {
    pub action_properties: UPtr<USimpleCollisionEditorToolActionProperties>,
}
pub struct UPlaneCutToolBuilder {}
pub struct UPlaneCutToolProperties {
    pub b_keep_both_halves: bool,
    pub spacing_between_halves: f32,
    pub b_export_separated_pieces_as_new_mesh_assets: bool,
    pub b_show_preview: bool,
    pub b_fill_cut_hole: bool,
    pub b_fill_spans: bool,
    pub b_simplify_along_cut: bool,
}
pub struct UPlaneCutOperatorFactory {
    pub cut_tool: UPtr<UPlaneCutTool>,
}
pub struct UPlaneCutTool {
    pub basic_properties: UPtr<UPlaneCutToolProperties>,
    pub previews: TArray<UPtr<UMeshOpPreviewWithBackgroundCompute>>,
    pub meshes_to_cut: TArray<UPtr<UDynamicMeshReplacementChangeTarget>>,
    pub plane_mechanic: UPtr<UConstructionPlaneMechanic>,
}
pub struct UExtrudeMeshSelectionToolBuilder {}
pub struct UExtrudeMeshSelectionToolProperties {
    pub input_mode: EExtrudeMeshSelectionInteractionMode,
    pub extrude_distance: f64,
    pub region_mode: EExtrudeMeshSelectionRegionModifierMode,
    pub num_subdivisions: i32,
    pub crease_angle: f64,
    pub raycast_max_distance: f64,
    pub b_shells_to_solids: bool,
    pub b_infer_groups_from_nbrs: bool,
    pub b_group_per_subdivision: bool,
    pub b_replace_selection_groups: bool,
    pub uv_scale: f64,
    pub b_uv_island_per_group: bool,
    pub b_infer_material_id: bool,
    pub set_material_id: i32,
    pub b_show_input_materials: bool,
}
pub struct UExtrudeMeshSelectionTool {
    pub extrude_properties: UPtr<UExtrudeMeshSelectionToolProperties>,
    pub source_preview: UPtr<UPreviewMesh>,
    pub edit_compute: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub transform_gizmo: UPtr<UCombinedTransformGizmo>,
    pub transform_proxy: UPtr<UTransformProxy>,
}
pub struct UOffsetMeshSelectionToolBuilder {}
pub struct UOffsetMeshSelectionToolProperties {
    pub offset_distance: f64,
    pub direction: EOffsetMeshSelectionDirectionMode,
    pub num_subdivisions: i32,
    pub crease_angle: f64,
    pub b_shells_to_solids: bool,
    pub b_infer_groups_from_nbrs: bool,
    pub b_group_per_subdivision: bool,
    pub b_replace_selection_groups: bool,
    pub uv_scale: f64,
    pub b_uv_island_per_group: bool,
    pub b_infer_material_id: bool,
    pub set_material_id: i32,
    pub b_show_input_materials: bool,
}
pub struct UOffsetMeshSelectionTool {
    pub offset_properties: UPtr<UOffsetMeshSelectionToolProperties>,
    pub source_preview: UPtr<UPreviewMesh>,
    pub edit_compute: UPtr<UMeshOpPreviewWithBackgroundCompute>,
}
pub struct UMeshAnalysisProperties {
    pub surface_area: FString,
    pub volume: FString,
}
pub struct URevolveBoundaryToolBuilder {}
pub struct URevolveBoundaryOperatorFactory {
    pub revolve_boundary_tool: UPtr<URevolveBoundaryTool>,
}
pub struct URevolveBoundaryToolProperties {
    pub cap_fill_mode: ERevolvePropertiesCapFillMode,
    pub b_display_input_mesh: bool,
    pub axis_origin: FVector,
    pub axis_orientation: FVector2D,
}
pub struct URevolveBoundaryTool {
    pub output_type_properties: UPtr<UCreateMeshObjectTypeProperties>,
    pub settings: UPtr<URevolveBoundaryToolProperties>,
    pub material_properties: UPtr<UNewMeshMaterialProperties>,
    pub plane_mechanic: UPtr<UConstructionPlaneMechanic>,
    pub preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
}
pub struct URevolveSplineToolProperties {
    pub sample_mode: ERevolveSplineSampleMode,
    pub error_tolerance: f64,
    pub max_sample_distance: f64,
    pub cap_fill_mode: ERevolvePropertiesCapFillMode,
    pub b_close_path_to_axis: bool,
    pub axis_origin: FVector,
    pub axis_orientation: FVector2D,
    pub b_reset_axis_on_start: bool,
}
pub struct URevolveSplineToolActionPropertySet {}
pub struct UBaseMeshFromSplinesTool {
    pub output_type_properties: UPtr<UCreateMeshObjectTypeProperties>,
    pub material_properties: UPtr<UNewMeshMaterialProperties>,
    pub preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub target_world: TWeakObjectPtr<UWorld>,
    pub actors_with_splines: TArray<TWeakObjectPtr<AActor>>,
}
pub struct URevolveSplineTool {
    pub settings: UPtr<URevolveSplineToolProperties>,
    pub tool_actions: UPtr<URevolveSplineToolActionPropertySet>,
    pub plane_mechanic: UPtr<UConstructionPlaneMechanic>,
}
pub struct UBaseMeshFromSplinesToolBuilder {}
pub struct URevolveSplineToolBuilder {}
pub struct USeamSculptToolBuilder {}
pub struct USeamSculptToolProperties {
    pub b_show_wireframe: bool,
    pub b_hit_back_faces: bool,
    pub path_similarity_weight: f64,
}
pub struct USeamSculptTool {
    pub settings: UPtr<USeamSculptToolProperties>,
    pub preview_geom: UPtr<UPreviewGeometry>,
}
pub struct USelfUnionMeshesToolProperties {
    pub b_trim_flaps: bool,
    pub b_try_fix_holes: bool,
    pub b_try_collapse_edges: bool,
    pub winding_threshold: f32,
    pub b_show_new_boundary_edges: bool,
    pub b_only_use_first_mesh_materials: bool,
}
pub struct USelfUnionMeshesTool {
    pub properties: UPtr<USelfUnionMeshesToolProperties>,
    pub drawn_line_set: UPtr<ULineSetComponent>,
}
pub struct USelfUnionMeshesToolBuilder {}
pub struct USplitMeshesToolBuilder {}
pub struct USplitMeshesToolProperties {
    pub split_method: ESplitMeshesMethod,
    pub connect_vertices_threshold: f64,
    pub b_transfer_materials: bool,
    pub b_show_preview: bool,
    pub b_is_in_selection_mode: bool,
}
pub struct USplitMeshesTool {
    pub basic_properties: UPtr<USplitMeshesToolProperties>,
    pub output_type_properties: UPtr<UCreateMeshObjectTypeProperties>,
    pub per_target_previews: TArray<UPtr<UPreviewGeometry>>,
    pub preview_material: UPtr<UMaterialInterface>,
}
pub struct UTransferMeshToolBuilder {}
pub struct UTransferMeshToolProperties {
    pub b_transfer_materials: bool,
    pub b_transfer_collision: bool,
    pub source_lod: FString,
    pub target_lod: FString,
    pub b_is_static_mesh_source: bool,
    pub source_lod_names_list: TArray<FString>,
    pub target_lod_names_list: TArray<FString>,
    pub b_is_static_mesh_target: bool,
}
pub struct UTransferMeshTool {
    pub basic_properties: UPtr<UTransferMeshToolProperties>,
}
pub struct UTransformMeshesToolBuilder {}
pub struct UTransformMeshesToolProperties {
    pub transform_mode: ETransformMeshesTransformMode,
    pub b_apply_to_instances: bool,
    pub b_set_pivot_mode: bool,
    pub b_enable_snap_dragging: bool,
    pub snap_drag_source: ETransformMeshesSnapDragSource,
    pub rotation_mode: ETransformMeshesSnapDragRotationMode,
    pub b_have_instances: bool,
}
pub struct UTransformMeshesTool {
    pub transform_props: UPtr<UTransformMeshesToolProperties>,
    pub active_gizmos: TArray<FTransformMeshesTarget>,
    pub drag_alignment_mechanic: UPtr<UDragAlignmentMechanic>,
}
pub struct UTriangulateSplinesToolProperties {
    pub error_tolerance: f64,
    pub flatten_method: EFlattenCurveMethod,
    pub combine_method: ECombineCurvesMethod,
    pub thickness: f64,
    pub b_flip_result: bool,
    pub open_curves: EOffsetOpenCurvesMethod,
    pub curve_offset: f64,
    pub offset_closed_curves: EOffsetClosedCurvesMethod,
    pub end_shapes: EOpenCurveEndShapes,
    pub join_method: EOffsetJoinMethod,
    pub miter_limit: f64,
}
pub struct UTriangulateSplinesTool {
    pub triangulate_properties: UPtr<UTriangulateSplinesToolProperties>,
}
pub struct UTriangulateSplinesToolBuilder {}
pub struct UUVTransferToolBuilder {}
pub struct UUVTransferToolProperties {
    pub b_reverse_direction: bool,
    pub b_transfer_seams_only: bool,
    pub b_clear_existing_seams: bool,
    pub path_similarity_weight: f64,
    pub b_show_wireframes: bool,
    pub b_show_seams: bool,
    pub vertex_search_distance: f64,
    pub source_uv_channel: FString,
    pub b_same_destination_channel: bool,
    pub dest_uv_channel: FString,
}
pub struct UUVTransferTool {
    pub settings: UPtr<UUVTransferToolProperties>,
    pub destination_material_settings: UPtr<UExistingMeshMaterialProperties>,
    pub destination_preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub source_preview: UPtr<UPreviewMesh>,
    pub source_seam_visualizer: UPtr<UMeshElementsVisualizer>,
    pub destination_seam_visualizer: UPtr<UMeshElementsVisualizer>,
}
pub struct UVolumeToMeshToolBuilder {}
pub struct UVolumeToMeshToolProperties {
    pub b_weld_edges: bool,
    pub b_auto_repair: bool,
    pub b_optimize_mesh: bool,
    pub b_show_wireframe: bool,
}
pub struct UVolumeToMeshTool {
    pub settings: UPtr<UVolumeToMeshToolProperties>,
    pub output_type_properties: UPtr<UCreateMeshObjectTypeProperties>,
    pub preview_mesh: UPtr<UPreviewMesh>,
    pub target_volume: TLazyObjectPtr<AVolume>,
    pub volume_edges_set: UPtr<ULineSetComponent>,
}
pub struct UVoxelBlendMeshesToolProperties {
    pub blend_power: f64,
    pub blend_falloff: f64,
    pub operation: EVoxelBlendOperation,
    pub b_vox_wrap: bool,
    pub b_remove_internals_after_vox_wrap: bool,
    pub thicken_shells: f64,
}
pub struct UVoxelBlendMeshesTool {
    pub blend_properties: UPtr<UVoxelBlendMeshesToolProperties>,
}
pub struct UVoxelBlendMeshesToolBuilder {}
pub struct UVoxelMorphologyMeshesToolProperties {
    pub operation: EMorphologyOperation,
    pub distance: f64,
    pub b_vox_wrap: bool,
    pub b_remove_internals_after_vox_wrap: bool,
    pub thicken_shells: f64,
}
pub struct UVoxelMorphologyMeshesTool {
    pub morphology_properties: UPtr<UVoxelMorphologyMeshesToolProperties>,
}
pub struct UVoxelMorphologyMeshesToolBuilder {}
pub struct UVoxelSolidifyMeshesToolProperties {
    pub winding_threshold: f64,
    pub extend_bounds: f64,
    pub surface_search_steps: i32,
    pub b_solid_at_boundaries: bool,
    pub b_apply_thicken_shells: bool,
    pub thicken_shells: f64,
}
pub struct UVoxelSolidifyMeshesTool {
    pub solidify_properties: UPtr<UVoxelSolidifyMeshesToolProperties>,
}
pub struct UVoxelSolidifyMeshesToolBuilder {}
