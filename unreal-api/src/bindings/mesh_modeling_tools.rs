#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FPerlinLayerProperties {
    pub frequency: f32,
    pub intensity: f32,
}
#[repr(C, align(4))]
pub struct FBrushToolRadius {
    pub size_type: EBrushToolSizeType,
    pub adaptive_size: f32,
    pub world_radius: f32,
    pub b_tool_supports_pressure_sensitivity: bool,
    pub b_enable_pressure_sensitivity: bool,
}
pub struct UMeshSculptBrushOpProps {
    pub b_is_strength_pressure_enabled: bool,
}
pub struct UBaseKelvinletBrushOpProps {
    pub stiffness: f32,
    pub incompressiblity: f32,
    pub brush_steps: i32,
}
pub struct UScaleKelvinletBrushOpProps {
    pub strength: f32,
    pub falloff: f32,
}
pub struct UPullKelvinletBrushOpProps {
    pub falloff: f32,
    pub depth: f32,
}
pub struct USharpPullKelvinletBrushOpProps {
    pub falloff: f32,
    pub depth: f32,
}
pub struct UTwistKelvinletBrushOpProps {
    pub strength: f32,
    pub falloff: f32,
}
pub struct UEraseSculptLayerBrushOpProps {
    pub strength: f32,
    pub falloff: f32,
}
pub struct UGroupEraseBrushOpProps {
    pub group: i32,
    pub b_only_erase_current: bool,
}
pub struct UGroupPaintBrushOpProps {
    pub group: i32,
    pub b_only_paint_ungrouped: bool,
}
pub struct UInflateBrushOpProps {
    pub strength: f32,
    pub falloff: f32,
}
pub struct UMoveBrushOpProps {
    pub strength: f32,
    pub falloff: f32,
    pub depth: f32,
    pub axis_filters: FModelingToolsAxisFilter,
}
pub struct UPinchBrushOpProps {
    pub strength: f32,
    pub falloff: f32,
    pub depth: f32,
    pub b_perp_damping: bool,
}
pub struct UBasePlaneBrushOpProps {}
pub struct UPlaneBrushOpProps {
    pub strength: f32,
    pub falloff: f32,
    pub depth: f32,
    pub which_side: EPlaneBrushSideMode,
}
pub struct UViewAlignedPlaneBrushOpProps {
    pub strength: f32,
    pub falloff: f32,
    pub depth: f32,
    pub which_side: EPlaneBrushSideMode,
}
pub struct UFixedPlaneBrushOpProps {
    pub strength: f32,
    pub falloff: f32,
    pub depth: f32,
    pub which_side: EPlaneBrushSideMode,
}
pub struct UStandardSculptBrushOpProps {
    pub strength: f32,
    pub falloff: f32,
}
pub struct UViewAlignedSculptBrushOpProps {
    pub strength: f32,
    pub falloff: f32,
}
pub struct USculptMaxBrushOpProps {
    pub strength: f32,
    pub falloff: f32,
    pub max_height: f32,
    pub b_use_fixed_height: bool,
    pub fixed_height: f32,
}
pub struct UBaseSmoothBrushOpProps {}
pub struct USmoothBrushOpProps {
    pub strength: f32,
    pub falloff: f32,
    pub b_preserve_uv_flow: bool,
}
pub struct USecondarySmoothBrushOpProps {
    pub strength: f32,
    pub falloff: f32,
    pub b_preserve_uv_flow: bool,
}
pub struct USmoothFillBrushOpProps {
    pub strength: f32,
    pub falloff: f32,
    pub b_preserve_uv_flow: bool,
}
pub struct UFlattenBrushOpProps {
    pub strength: f32,
    pub falloff: f32,
    pub depth: f32,
    pub which_side: EPlaneBrushSideMode,
}
pub struct UEraseBrushOpProps {
    pub strength: f32,
    pub falloff: f32,
}
pub struct UVertexColorBaseBrushOpProps {
    pub strength: f32,
    pub falloff: f32,
    pub blend_mode: EVertexColorPaintBrushOpBlendMode,
    pub b_apply_falloff: bool,
}
pub struct UVertexColorPaintBrushOpProps {
    pub color: FLinearColor,
}
pub struct UVertexColorSoftenBrushOpProps {}
pub struct UVertexColorSmoothBrushOpProps {}
pub struct UPolyEditActivityContext {
    pub common_properties: UPtr<UPolyEditCommonProperties>,
    pub preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub selection_mechanic: UPtr<UPolygonSelectionMechanic>,
}
pub struct UMeshConstraintProperties {
    pub b_preserve_sharp_edges: bool,
    pub mesh_boundary_constraint: EMeshBoundaryConstraint,
    pub group_boundary_constraint: EGroupBoundaryConstraint,
    pub material_boundary_constraint: EMaterialBoundaryConstraint,
    pub b_prevent_normal_flips: bool,
    pub b_prevent_tiny_triangles: bool,
}
pub struct URemeshProperties {
    pub smoothing_strength: f32,
    pub b_flips: bool,
    pub b_splits: bool,
    pub b_collapses: bool,
}
pub struct UAddPrimitiveToolBuilder {}
pub struct UProceduralShapeToolProperties {
    pub polygroup_mode: EMakeMeshPolygroupMode,
    pub target_surface: EMakeMeshPlacementType,
    pub pivot_location: EMakeMeshPivotLocation,
    pub rotation: f32,
    pub b_align_to_normal: bool,
    pub b_show_gizmo: bool,
    pub b_show_gizmo_options: bool,
}
pub struct UProceduralBoxToolProperties {
    pub width: f32,
    pub depth: f32,
    pub height: f32,
    pub width_subdivisions: i32,
    pub depth_subdivisions: i32,
    pub height_subdivisions: i32,
}
pub struct UProceduralRectangleToolProperties {
    pub rectangle_type: EProceduralRectType,
    pub width: f32,
    pub depth: f32,
    pub width_subdivisions: i32,
    pub depth_subdivisions: i32,
    pub b_maintain_dimension: bool,
    pub corner_radius: f32,
    pub corner_slices: i32,
}
pub struct UProceduralDiscToolProperties {
    pub disc_type: EProceduralDiscType,
    pub radius: f32,
    pub radial_slices: i32,
    pub radial_subdivisions: i32,
    pub hole_radius: f32,
}
pub struct UProceduralTorusToolProperties {
    pub major_radius: f32,
    pub minor_radius: f32,
    pub major_slices: i32,
    pub minor_slices: i32,
}
pub struct UProceduralCylinderToolProperties {
    pub radius: f32,
    pub height: f32,
    pub radial_slices: i32,
    pub height_subdivisions: i32,
}
pub struct UProceduralConeToolProperties {
    pub radius: f32,
    pub height: f32,
    pub radial_slices: i32,
    pub height_subdivisions: i32,
}
pub struct UProceduralArrowToolProperties {
    pub shaft_radius: f32,
    pub shaft_height: f32,
    pub head_radius: f32,
    pub head_height: f32,
    pub radial_slices: i32,
    pub height_subdivisions: i32,
}
pub struct UProceduralSphereToolProperties {
    pub radius: f32,
    pub subdivision_type: EProceduralSphereType,
    pub subdivisions: i32,
    pub horizontal_slices: i32,
    pub vertical_slices: i32,
}
pub struct UProceduralCapsuleToolProperties {
    pub radius: f32,
    pub cylinder_length: f32,
    pub hemisphere_slices: i32,
    pub cylinder_slices: i32,
    pub cylinder_subdivisions: i32,
}
pub struct UProceduralStairsToolProperties {
    pub stairs_type: EProceduralStairsType,
    pub num_steps: i32,
    pub step_width: f32,
    pub step_height: f32,
    pub step_depth: f32,
    pub curve_angle: f32,
    pub spiral_angle: f32,
    pub inner_radius: f32,
}
pub struct UAddPrimitiveTool {
    pub output_type_properties: UPtr<UCreateMeshObjectTypeProperties>,
    pub shape_settings: UPtr<UProceduralShapeToolProperties>,
    pub material_properties: UPtr<UNewMeshMaterialProperties>,
    pub preview_mesh: UPtr<UPreviewMesh>,
    pub gizmo: UPtr<UCombinedTransformGizmo>,
    pub drag_alignment_mechanic: UPtr<UDragAlignmentMechanic>,
    pub asset_name: FString,
}
pub struct UAddBoxPrimitiveTool {}
pub struct UAddCylinderPrimitiveTool {}
pub struct UAddCapsulePrimitiveTool {}
pub struct UAddConePrimitiveTool {}
pub struct UAddRectanglePrimitiveTool {}
pub struct UAddDiscPrimitiveTool {}
pub struct UAddTorusPrimitiveTool {}
pub struct UAddArrowPrimitiveTool {}
pub struct UAddSpherePrimitiveTool {}
pub struct UAddStairsPrimitiveTool {}
pub struct UCombineMeshesToolBuilder {}
pub struct UCombineMeshesToolProperties {
    pub b_is_duplicate_mode: bool,
    pub output_write_to: EBaseCreateFromSelectedTargetType,
    pub output_new_name: FString,
    pub output_existing_name: FString,
}
pub struct UCombineMeshesTool {
    pub basic_properties: UPtr<UCombineMeshesToolProperties>,
    pub output_type_properties: UPtr<UCreateMeshObjectTypeProperties>,
    pub handle_source_properties: UPtr<UOnAcceptHandleSourcesPropertiesBase>,
}
pub struct UDeleteGeometrySelectionCommand {}
pub struct UDisconnectGeometrySelectionCommand {}
pub struct UModifyGeometrySelectionCommand {}
pub struct UModifyGeometrySelectionCommand_Invert {}
pub struct UModifyGeometrySelectionCommand_ExpandToConnected {}
pub struct UModifyGeometrySelectionCommand_InvertConnected {}
pub struct UModifyGeometrySelectionCommand_Expand {}
pub struct UModifyGeometrySelectionCommand_Contract {}
pub struct URetriangulateGeometrySelectionCommand {}
pub struct UConvertToPolygonsToolBuilder {}
pub struct UConvertToPolygonsToolProperties {
    pub conversion_mode: EConvertToPolygonsMode,
    pub angle_tolerance: f32,
    pub b_use_average_group_normal: bool,
    pub num_points: i32,
    pub b_split_existing: bool,
    pub b_normal_weighted: bool,
    pub normal_weighting: f32,
    pub quad_adjacency_weight: f32,
    pub quad_metric_clamp: f32,
    pub quad_search_rounds: i32,
    pub b_respect_uv_seams: bool,
    pub b_respect_hard_normals: bool,
    pub min_group_size: i32,
    pub b_show_group_colors: bool,
    pub b_calculate_normals: bool,
    pub group_layer: FName,
    pub options_list: TArray<FString>,
    pub b_show_new_layer_name: bool,
    pub new_layer_name: FString,
}
pub struct UConvertToPolygonsOperatorFactory {
    pub convert_to_polygons_tool: UPtr<UConvertToPolygonsTool>,
}
pub struct UConvertToPolygonsTool {
    pub settings: UPtr<UConvertToPolygonsToolProperties>,
    pub copy_from_layer_properties: UPtr<UPolygroupLayersProperties>,
    pub preview_compute: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub preview_geometry: UPtr<UPreviewGeometry>,
    pub unmodified_area_preview_mesh: UPtr<UPreviewMesh>,
}
pub struct UCSGMeshesToolProperties {
    pub operation: ECSGOperation,
    pub b_try_fix_holes: bool,
    pub b_try_collapse_edges: bool,
    pub winding_threshold: f32,
    pub b_show_new_boundaries: bool,
    pub b_show_subtracted_mesh: bool,
    pub subtracted_mesh_opacity: f32,
    pub subtracted_mesh_color: FLinearColor,
    pub b_use_first_mesh_materials: bool,
}
pub struct UTrimMeshesToolProperties {
    pub which_mesh: ETrimOperation,
    pub trim_side: ETrimSide,
    pub winding_threshold: f32,
    pub b_show_trimming_mesh: bool,
    pub opacity_of_trimming_mesh: f32,
    pub color_of_trimming_mesh: FLinearColor,
}
pub struct UCSGMeshesTool {
    pub csg_properties: UPtr<UCSGMeshesToolProperties>,
    pub trim_properties: UPtr<UTrimMeshesToolProperties>,
    pub original_mesh_previews: TArray<UPtr<UPreviewMesh>>,
    pub previews_ghost_material: UPtr<UMaterialInstanceDynamic>,
    pub drawn_line_set: UPtr<ULineSetComponent>,
}
pub struct UCSGMeshesToolBuilder {}
pub struct UCutMeshWithMeshToolProperties {
    pub b_try_fix_holes: bool,
    pub b_try_collapse_edges: bool,
    pub winding_threshold: f32,
    pub b_show_new_boundaries: bool,
    pub b_use_first_mesh_materials: bool,
}
pub struct UCutMeshWithMeshTool {
    pub cut_properties: UPtr<UCutMeshWithMeshToolProperties>,
    pub intersect_preview_mesh: UPtr<UPreviewMesh>,
    pub drawn_line_set: UPtr<ULineSetComponent>,
}
pub struct UCutMeshWithMeshToolBuilder {}
pub struct UDeformMeshPolygonsToolBuilder {}
pub struct UDeformMeshPolygonsTransformProperties {
    pub deformation_strategy: EGroupTopologyDeformationStrategy,
    pub transform_mode: EQuickTransformerMode,
    pub b_select_faces: bool,
    pub b_select_edges: bool,
    pub b_select_vertices: bool,
    pub b_show_wireframe: bool,
    pub selected_weight_scheme: EWeightScheme,
    pub handle_weight: f64,
    pub b_post_fix_handles: bool,
}
pub struct UDeformMeshPolygonsTool {
    pub preview_mesh_actor: UPtr<AInternalToolFrameworkActor>,
    pub dynamic_mesh_component: UPtr<UDynamicMeshComponent>,
    pub transform_props: UPtr<UDeformMeshPolygonsTransformProperties>,
}
pub struct UDisplaceMeshCommonProperties {
    pub displacement_type: EDisplaceMeshToolDisplaceType,
    pub displace_intensity: f32,
    pub random_seed: i32,
    pub subdivision_type: EDisplaceMeshToolSubdivisionType,
    pub subdivisions: i32,
    pub weight_map: FName,
    pub weight_maps_list: TArray<FString>,
    pub b_invert_weight_map: bool,
    pub b_show_wireframe: bool,
    pub b_disable_size_warning: bool,
}
pub struct USelectiveTessellationProperties {
    pub selection_type: EDisplaceMeshToolTriangleSelectionType,
    pub active_material: FName,
    pub material_id_list: TArray<FString>,
}
pub struct UDisplaceMeshTextureMapProperties {
    pub displacement_map: UPtr<UTexture2D>,
    pub channel: EDisplaceMeshToolChannelType,
    pub displacement_map_base_value: f32,
    pub uv_scale: FVector2D,
    pub uv_offset: FVector2D,
    pub b_apply_adjustment_curve: bool,
    pub adjustment_curve: UPtr<UCurveFloat>,
    pub b_recalc_normals: bool,
}
pub struct UDisplaceMeshDirectionalFilterProperties {
    pub b_enable_filter: bool,
    pub filter_direction: FVector,
    pub filter_width: f32,
}
pub struct UDisplaceMeshPerlinNoiseProperties {
    pub perlin_layer_properties: TArray<FPerlinLayerProperties>,
}
pub struct UDisplaceMeshSineWaveProperties {
    pub sine_wave_frequency: f32,
    pub sine_wave_phase_shift: f32,
    pub sine_wave_direction: FVector,
}
pub struct UDisplaceMeshToolBuilder {}
pub struct UDisplaceMeshTool {
    pub common_properties: UPtr<UDisplaceMeshCommonProperties>,
    pub directional_filter_properties: UPtr<UDisplaceMeshDirectionalFilterProperties>,
    pub texture_map_properties: UPtr<UDisplaceMeshTextureMapProperties>,
    pub noise_properties: UPtr<UDisplaceMeshPerlinNoiseProperties>,
    pub sine_wave_properties: UPtr<UDisplaceMeshSineWaveProperties>,
    pub selective_tessellation_properties: UPtr<USelectiveTessellationProperties>,
    pub active_contrast_curve_target: UPtr<UCurveFloat>,
    pub mesh_statistics: UPtr<UMeshStatisticsProperties>,
    pub preview_mesh_actor: UPtr<AInternalToolFrameworkActor>,
    pub dynamic_mesh_component: UPtr<UDynamicMeshComponent>,
}
pub struct UDrawAndRevolveToolBuilder {}
pub struct URevolveProperties {
    pub revolve_degrees_clamped: f64,
    pub revolve_degrees: f64,
    pub revolve_degrees_offset: f64,
    pub steps_max_degrees: f64,
    pub b_explicit_steps: bool,
    pub num_explicit_steps: i32,
    pub height_offset_per_degree: f64,
    pub b_reverse_revolution_direction: bool,
    pub b_flip_mesh: bool,
    pub b_sharp_normals: bool,
    pub sharp_normals_degree_threshold: f64,
    pub b_path_at_midpoint_of_step: bool,
    pub polygroup_mode: ERevolvePropertiesPolygroupMode,
    pub quad_split_mode: ERevolvePropertiesQuadSplit,
}
pub struct URevolveToolProperties {
    pub cap_fill_mode: ERevolvePropertiesCapFillMode,
    pub b_close_path_to_axis: bool,
    pub draw_plane_origin: FVector,
    pub draw_plane_orientation: FRotator,
    pub b_enable_snapping: bool,
    pub b_allowed_to_edit_draw_plane: bool,
}
pub struct URevolveOperatorFactory {
    pub revolve_tool: UPtr<UDrawAndRevolveTool>,
}
pub struct UDrawAndRevolveTool {
    pub control_points_mechanic: UPtr<UCurveControlPointsMechanic>,
    pub plane_mechanic: UPtr<UConstructionPlaneMechanic>,
    pub output_type_properties: UPtr<UCreateMeshObjectTypeProperties>,
    pub settings: UPtr<URevolveToolProperties>,
    pub material_properties: UPtr<UNewMeshMaterialProperties>,
    pub preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
}
pub struct UDrawPolygonToolBuilder {}
pub struct UDrawPolygonToolStandardProperties {
    pub polygon_draw_mode: EDrawPolygonDrawMode,
    pub b_allow_self_intersections: bool,
    pub feature_size_ratio: f32,
    pub radial_slices: i32,
    pub distance: f32,
    pub b_show_grid_gizmo: bool,
    pub extrude_mode: EDrawPolygonExtrudeMode,
    pub extrude_height: f32,
}
pub struct UDrawPolygonToolSnapProperties {
    pub b_enable_snapping: bool,
    pub b_snap_to_world_grid: bool,
    pub b_snap_to_vertices: bool,
    pub b_snap_to_edges: bool,
    pub b_snap_to_axes: bool,
    pub b_snap_to_lengths: bool,
    pub b_snap_to_surfaces: bool,
    pub snap_to_surfaces_offset: f32,
}
pub struct UDrawPolygonTool {
    pub output_type_properties: UPtr<UCreateMeshObjectTypeProperties>,
    pub polygon_properties: UPtr<UDrawPolygonToolStandardProperties>,
    pub snap_properties: UPtr<UDrawPolygonToolSnapProperties>,
    pub material_properties: UPtr<UNewMeshMaterialProperties>,
    pub preview_mesh: UPtr<UPreviewMesh>,
    pub height_mechanic: UPtr<UPlaneDistanceFromHitMechanic>,
    pub drag_alignment_mechanic: UPtr<UDragAlignmentMechanic>,
    pub plane_mechanic: UPtr<UConstructionPlaneMechanic>,
}
pub struct UDynamicMeshBrushTool {
    pub preview_mesh: UPtr<UPreviewMesh>,
}
pub struct UDynamicMeshSculptToolBuilder {}
pub struct UDynamicMeshBrushProperties {
    pub brush_size: FBrushToolRadius,
    pub brush_falloff_amount: f32,
    pub depth: f32,
    pub b_hit_back_faces: bool,
}
pub struct UDynamicMeshBrushSculptProperties {
    pub b_is_remeshing_enabled: bool,
    pub primary_brush_type: EDynamicMeshSculptBrushType,
    pub primary_brush_speed: f32,
    pub b_preserve_uv_flow: bool,
    pub b_freeze_target: bool,
    pub smooth_brush_speed: f32,
    pub b_detail_preserving_smooth: bool,
}
pub struct UDynamicSculptToolActions {}
pub struct UBrushRemeshProperties {
    pub b_enable_remeshing: bool,
    pub triangle_size: i32,
    pub preserve_detail: i32,
    pub iterations: i32,
}
pub struct UFixedPlaneBrushProperties {
    pub b_property_set_enabled: bool,
    pub b_show_gizmo: bool,
    pub position: FVector,
    pub rotation: FQuat,
}
pub struct UDynamicMeshSculptTool {
    pub brush_properties: UPtr<UDynamicMeshBrushProperties>,
    pub sculpt_properties: UPtr<UDynamicMeshBrushSculptProperties>,
    pub sculpt_max_brush_properties: UPtr<USculptMaxBrushProperties>,
    pub kelvin_brush_properties: UPtr<UKelvinBrushProperties>,
    pub remesh_properties: UPtr<UBrushRemeshProperties>,
    pub gizmo_properties: UPtr<UFixedPlaneBrushProperties>,
    pub view_properties: UPtr<UMeshEditingViewProperties>,
    pub sculpt_tool_actions: UPtr<UDynamicSculptToolActions>,
    pub brush_indicator: UPtr<UBrushStampIndicator>,
    pub brush_indicator_material: UPtr<UMaterialInstanceDynamic>,
    pub brush_indicator_mesh: UPtr<UPreviewMesh>,
    pub preview_mesh_actor: UPtr<AInternalToolFrameworkActor>,
    pub dynamic_mesh_component: UPtr<UOctreeDynamicMeshComponent>,
    pub active_override_material: UPtr<UMaterialInstanceDynamic>,
    pub plane_transform_gizmo: UPtr<UCombinedTransformGizmo>,
    pub plane_transform_proxy: UPtr<UTransformProxy>,
}
pub struct UEditMeshPolygonsToolBuilder {}
pub struct UPolyEditCommonProperties {
    pub b_show_wireframe: bool,
    pub b_show_selectable_corners: bool,
    pub b_gizmo_visible: bool,
    pub local_frame_mode: ELocalFrameMode,
    pub b_lock_rotation: bool,
    pub b_local_coord_system: bool,
}
pub struct UEditMeshPolygonsActionModeToolBuilder {}
pub struct UEditMeshPolygonsSelectionModeToolBuilder {}
pub struct UEditMeshPolygonsToolActionPropertySet {}
pub struct UPolyEditTopologyProperties {
    pub b_add_extra_corners: bool,
    pub extra_corner_angle_threshold_degrees: f64,
}
pub struct UEditMeshPolygonsToolActions {}
pub struct UEditMeshPolygonsToolActions_Triangles {}
pub struct UEditMeshPolygonsToolUVActions {}
pub struct UEditMeshPolygonsToolEdgeActions {}
pub struct UEditMeshPolygonsToolEdgeActions_Triangles {}
pub struct UEditMeshPolygonsTool {
    pub preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub common_props: UPtr<UPolyEditCommonProperties>,
    pub edit_actions: UPtr<UEditMeshPolygonsToolActions>,
    pub edit_actions_triangles: UPtr<UEditMeshPolygonsToolActions_Triangles>,
    pub edit_edge_actions: UPtr<UEditMeshPolygonsToolEdgeActions>,
    pub edit_edge_actions_triangles: UPtr<UEditMeshPolygonsToolEdgeActions_Triangles>,
    pub edit_uv_actions: UPtr<UEditMeshPolygonsToolUVActions>,
    pub topology_properties: UPtr<UPolyEditTopologyProperties>,
    pub extrude_activity: UPtr<UPolyEditExtrudeActivity>,
    pub inset_outset_activity: UPtr<UPolyEditInsetOutsetActivity>,
    pub cut_faces_activity: UPtr<UPolyEditCutFacesActivity>,
    pub planar_projection_uv_activity: UPtr<UPolyEditPlanarProjectionUVActivity>,
    pub insert_edge_activity: UPtr<UPolyEditInsertEdgeActivity>,
    pub insert_edge_loop_activity: UPtr<UPolyEditInsertEdgeLoopActivity>,
    pub bevel_edge_activity: UPtr<UPolyEditBevelEdgeActivity>,
    pub extrude_edge_activity: UPtr<UPolyEditExtrudeEdgeActivity>,
    pub activity_context: UPtr<UPolyEditActivityContext>,
    pub selection_mechanic: UPtr<UPolygonSelectionMechanic>,
    pub drag_alignment_mechanic: UPtr<UDragAlignmentMechanic>,
    pub transform_gizmo: UPtr<UCombinedTransformGizmo>,
    pub transform_proxy: UPtr<UTransformProxy>,
}
pub struct UHoleFillToolBuilder {}
pub struct USmoothHoleFillProperties {
    pub b_constrain_to_hole_interior: bool,
    pub remeshing_exterior_region_width: i32,
    pub smoothing_exterior_region_width: i32,
    pub smoothing_interior_region_width: i32,
    pub interior_smoothness: f32,
    pub fill_density_scalar: f64,
    pub b_project_during_remesh: bool,
}
pub struct UHoleFillToolProperties {
    pub fill_type: EHoleFillOpFillType,
    pub b_remove_isolated_triangles: bool,
    pub b_quick_fill_small_holes: bool,
}
pub struct UHoleFillToolActions {}
pub struct UHoleFillStatisticsProperties {
    pub initial_holes: FString,
    pub selected_holes: FString,
    pub successful_fills: FString,
    pub failed_fills: FString,
    pub remaining_holes: FString,
}
pub struct UHoleFillOperatorFactory {
    pub fill_tool: UPtr<UHoleFillTool>,
}
pub struct UHoleFillTool {
    pub smooth_hole_fill_properties: UPtr<USmoothHoleFillProperties>,
    pub properties: UPtr<UHoleFillToolProperties>,
    pub actions: UPtr<UHoleFillToolActions>,
    pub statistics: UPtr<UHoleFillStatisticsProperties>,
    pub preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub selection_mechanic: UPtr<UBoundarySelectionMechanic>,
}
pub struct ULatticeDeformerToolBuilder {}
pub struct ULatticeDeformerToolProperties {
    pub x_axis_resolution: i32,
    pub y_axis_resolution: i32,
    pub z_axis_resolution: i32,
    pub padding: f32,
    pub interpolation_type: ELatticeInterpolationType,
    pub b_deform_normals: bool,
    pub b_can_change_resolution: bool,
    pub gizmo_coordinate_system: EToolContextCoordinateSystem,
    pub b_set_pivot_mode: bool,
    pub b_soft_deformation: bool,
}
pub struct ULatticeDeformerOperatorFactory {
    pub lattice_deformer_tool: UPtr<ULatticeDeformerTool>,
}
pub struct ULatticeDeformerTool {
    pub control_points_mechanic: UPtr<ULatticeControlPointsMechanic>,
    pub settings: UPtr<ULatticeDeformerToolProperties>,
    pub preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub sculpt_layer_properties: UPtr<UMeshSculptLayerProperties>,
    pub lattice_storage: TScriptInterface<ILatticeStateStorage>,
    pub b_lattice_deformed: bool,
}
pub struct UMeshAttributePaintToolBuilder {}
pub struct UMeshAttributePaintBrushOperationProperties {
    pub b_isolate_geometry_selection: bool,
    pub b_tool_has_selection: bool,
    pub brush_action: EBrushActionMode,
    pub brush_value: f32,
}
pub struct UMeshAttributePaintToolVisualizationProperties {
    pub material_mode: EMeshAttributePaintMaterialMode,
    pub b_flat_shading: bool,
}
pub struct UMeshAttributePaintToolProperties {
    pub attribute: FString,
}
pub struct UMeshAttributePaintEditActions {}
pub struct UMeshAttributePaintTool {
    pub brush_action_props: UPtr<UMeshAttributePaintBrushOperationProperties>,
    pub attrib_props: UPtr<UMeshAttributePaintToolProperties>,
    pub view_properties: UPtr<UMeshAttributePaintToolVisualizationProperties>,
    pub active_override_material: UPtr<UMaterialInstanceDynamic>,
}
pub struct UMeshGroupPaintToolBuilder {}
pub struct UGroupPaintBrushFilterProperties {
    pub primary_brush_type: EMeshGroupPaintBrushType,
    pub sub_tool_type: EMeshGroupPaintInteractionType,
    pub brush_size: f32,
    pub brush_area_mode: EMeshGroupPaintBrushAreaType,
    pub b_hit_back_faces: bool,
    pub set_group: i32,
    pub b_only_set_ungrouped: bool,
    pub erase_group: i32,
    pub b_only_erase_current: bool,
    pub angle_threshold: f32,
    pub b_uv_seams: bool,
    pub b_normal_seams: bool,
    pub visibility_filter: EMeshGroupPaintVisibilityType,
    pub min_tri_vert_count: i32,
    pub b_show_hit_group: bool,
    pub b_show_all_groups: bool,
}
pub struct UMeshGroupPaintToolActionPropertySet {}
pub struct UMeshGroupPaintToolFreezeActions {}
pub struct UMeshSculptToolBase {
    pub brush_properties: UPtr<USculptBrushProperties>,
    pub gizmo_properties: UPtr<UWorkPlaneProperties>,
    pub brush_op_prop_sets: TMap<i32, UPtr<UMeshSculptBrushOpProps>>,
    pub secondary_brush_op_prop_sets: TMap<i32, UPtr<UMeshSculptBrushOpProps>>,
    pub stroke_geometry: UPtr<UPreviewGeometry>,
    pub view_properties: UPtr<UMeshEditingViewProperties>,
    pub active_override_material: UPtr<UMaterialInstanceDynamic>,
    pub brush_indicator: UPtr<UBrushStampIndicator>,
    pub b_is_volumetric_indicator: bool,
    pub brush_indicator_material: UPtr<UMaterialInstanceDynamic>,
    pub brush_indicator_mesh: UPtr<UPreviewMesh>,
    pub plane_transform_gizmo: UPtr<UCombinedTransformGizmo>,
    pub plane_transform_proxy: UPtr<UTransformProxy>,
}
pub struct UMeshGroupPaintTool {
    pub polygroup_layer_properties: UPtr<UPolygroupLayersProperties>,
    pub filter_properties: UPtr<UGroupPaintBrushFilterProperties>,
    pub paint_brush_op_properties: UPtr<UGroupPaintBrushOpProps>,
    pub erase_brush_op_properties: UPtr<UGroupEraseBrushOpProps>,
    pub freeze_actions: UPtr<UMeshGroupPaintToolFreezeActions>,
    pub poly_lasso_mechanic: UPtr<UPolyLassoMarqueeMechanic>,
    pub preview_mesh_actor: UPtr<AInternalToolFrameworkActor>,
    pub dynamic_mesh_component: UPtr<UDynamicMeshComponent>,
    pub mesh_elements_display: UPtr<UMeshElementsVisualizer>,
}
pub struct UMeshSpaceDeformerToolBuilder {}
pub struct UMeshSpaceDeformerToolProperties {
    pub selected_operation_type: ENonlinearOperationType,
    pub upper_bounds_interval: f32,
    pub lower_bounds_interval: f32,
    pub bend_degrees: f32,
    pub twist_degrees: f32,
    pub flare_profile_type: EFlareProfileType,
    pub flare_percent_y: f32,
    pub b_lock_x_and_y_flaring: bool,
    pub flare_percent_x: f32,
    pub b_lock_bottom: bool,
    pub b_show_original_mesh: bool,
    pub b_draw_visualization: bool,
    pub b_align_to_normal_on_ctrl_click: bool,
}
pub struct UMeshSpaceDeformerToolActionPropertySet {}
pub struct USpaceDeformerOperatorFactory {
    pub space_deformer_tool: UPtr<UMeshSpaceDeformerTool>,
}
pub struct UMeshSpaceDeformerTool {
    pub settings: UPtr<UMeshSpaceDeformerToolProperties>,
    pub tool_actions: UPtr<UMeshSpaceDeformerToolActionPropertySet>,
    pub state_target: UPtr<UGizmoTransformChangeStateTarget>,
    pub drag_alignment_mechanic: UPtr<UDragAlignmentMechanic>,
    pub preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub original_mesh_preview: UPtr<UPreviewMesh>,
    pub interval_gizmo: UPtr<UIntervalGizmo>,
    pub transform_gizmo: UPtr<UCombinedTransformGizmo>,
    pub transform_proxy: UPtr<UTransformProxy>,
    pub up_interval_source: UPtr<UGizmoLocalFloatParameterSource>,
    pub down_interval_source: UPtr<UGizmoLocalFloatParameterSource>,
    pub forward_interval_source: UPtr<UGizmoLocalFloatParameterSource>,
}
pub struct UMeshVertexPaintToolBuilder {}
pub struct UVertexPaintBasicProperties {
    pub primary_brush_type: EMeshVertexPaintBrushType,
    pub sub_tool_type: EMeshVertexPaintInteractionType,
    pub strength: f64,
    pub paint_color: FLinearColor,
    pub b_is_paint_pressure_enabled: bool,
    pub blend_mode: EMeshVertexPaintColorBlendMode,
    pub secondary_action_type: EMeshVertexPaintSecondaryActionType,
    pub erase_color: FLinearColor,
    pub b_is_erase_pressure_enabled: bool,
    pub smooth_strength: f32,
    pub channel_filter: FModelingToolsColorChannelFilter,
    pub b_hard_edges: bool,
}
pub struct UVertexPaintBrushFilterProperties {
    pub brush_area_mode: EMeshVertexPaintBrushAreaType,
    pub angle_threshold: f32,
    pub b_uv_seams: bool,
    pub b_normal_seams: bool,
    pub visibility_filter: EMeshVertexPaintVisibilityType,
    pub b_isolate_geometry_selection: bool,
    pub b_tool_has_selection: bool,
    pub min_tri_vert_count: i32,
    pub material_mode: EMeshVertexPaintMaterialMode,
    pub b_show_hit_color: bool,
    pub current_sub_tool_type: EMeshVertexPaintInteractionType,
}
pub struct UMeshVertexPaintToolActionPropertySet {}
pub struct UMeshVertexPaintToolQuickActions {}
pub struct UMeshVertexPaintToolUtilityActions {
    pub operation: EMeshVertexPaintToolUtilityOperations,
    pub source_channel: EMeshVertexPaintColorChannel,
    pub source_value: f32,
    pub weight_map: FName,
    pub weight_maps_list: TArray<FString>,
    pub target_channels: FModelingToolsColorChannelFilter,
    pub target_channel: EMeshVertexPaintColorChannel,
    pub b_copy_to_hi_res: bool,
    pub copy_to_lod_name: FString,
    pub lod_names_list: TArray<FString>,
}
pub struct UMeshVertexPaintTool {
    pub polygroup_layer_properties: UPtr<UPolygroupLayersProperties>,
    pub basic_properties: UPtr<UVertexPaintBasicProperties>,
    pub filter_properties: UPtr<UVertexPaintBrushFilterProperties>,
    pub symmetry_properties: UPtr<UMeshSymmetryProperties>,
    pub paint_brush_op_properties: UPtr<UVertexColorPaintBrushOpProps>,
    pub erase_brush_op_properties: UPtr<UVertexColorPaintBrushOpProps>,
    pub quick_actions: UPtr<UMeshVertexPaintToolQuickActions>,
    pub utility_actions: UPtr<UMeshVertexPaintToolUtilityActions>,
    pub poly_lasso_mechanic: UPtr<UPolyLassoMarqueeMechanic>,
    pub preview_mesh_actor: UPtr<AInternalToolFrameworkActor>,
    pub dynamic_mesh_component: UPtr<UDynamicMeshComponent>,
    pub mesh_elements_display: UPtr<UMeshElementsVisualizer>,
}
pub struct UMeshVertexSculptToolBuilder {}
pub struct UVertexBrushSculptProperties {
    pub primary_brush_type: EMeshVertexSculptBrushType,
    pub primary_brush_id: i32,
    pub primary_falloff_type: EMeshSculptFalloffType,
    pub brush_filter: EMeshVertexSculptBrushFilterType,
    pub b_freeze_target: bool,
    pub b_can_freeze_target: bool,
    pub tool: TWeakObjectPtr<UMeshVertexSculptTool>,
}
pub struct UVertexBrushAlphaProperties {
    pub alpha: UPtr<UTexture2D>,
    pub rotation_angle: f32,
    pub b_randomize: bool,
    pub random_range: f32,
    pub tool: TWeakObjectPtr<UMeshVertexSculptTool>,
}
pub struct UMeshSymmetryProperties {
    pub b_enable_symmetry: bool,
    pub b_symmetry_can_be_enabled: bool,
}
pub struct UMeshVertexSculptTool {
    pub sculpt_properties: UPtr<UVertexBrushSculptProperties>,
    pub alpha_properties: UPtr<UVertexBrushAlphaProperties>,
    pub brush_alpha: UPtr<UTexture2D>,
    pub symmetry_properties: UPtr<UMeshSymmetryProperties>,
    pub sculpt_layer_properties: UPtr<UMeshSculptLayerProperties>,
    pub preview_mesh_actor: UPtr<AInternalToolFrameworkActor>,
    pub dynamic_mesh_component: UPtr<UDynamicMeshComponent>,
    pub octree_geometry: UPtr<UPreviewGeometry>,
}
pub struct UOffsetMeshToolProperties {
    pub offset_type: EOffsetMeshToolOffsetType,
    pub distance: f32,
    pub b_create_shell: bool,
}
pub struct UOffsetWeightMapSetProperties {
    pub min_distance: f32,
}
pub struct UIterativeOffsetProperties {
    pub steps: i32,
    pub b_offset_boundaries: bool,
    pub smoothing_per_step: f32,
    pub b_reproject_smooth: bool,
}
pub struct UImplicitOffsetProperties {
    pub smoothness: f32,
    pub b_preserve_u_vs: bool,
}
pub struct UOffsetMeshTool {
    pub offset_properties: UPtr<UOffsetMeshToolProperties>,
    pub iterative_properties: UPtr<UIterativeOffsetProperties>,
    pub implicit_properties: UPtr<UImplicitOffsetProperties>,
    pub weight_map_properties: UPtr<UOffsetWeightMapSetProperties>,
}
pub struct UOffsetMeshToolBuilder {}
pub struct UProjectToTargetToolBuilder {}
pub struct URemeshMeshToolProperties {
    pub target_triangle_count: i32,
    pub smoothing_type: ERemeshSmoothingType,
    pub b_discard_attributes: bool,
    pub b_show_group_colors: bool,
    pub remesh_type: ERemeshType,
    pub remesh_iterations: i32,
    pub max_remesh_iterations: i32,
    pub extra_projection_iterations: i32,
    pub b_use_target_edge_length: bool,
    pub target_edge_length: f32,
    pub b_reproject: bool,
    pub b_reproject_constraints: bool,
    pub boundary_corner_angle_threshold: f32,
}
pub struct UProjectToTargetToolProperties {
    pub b_world_space: bool,
    pub b_parallel: bool,
    pub face_projection_passes_per_remesh_iteration: i32,
    pub surface_projection_speed: f32,
    pub normal_alignment_speed: f32,
    pub b_smooth_in_fill_areas: bool,
    pub fill_area_distance_multiplier: f32,
    pub fill_area_smooth_multiplier: f32,
}
pub struct URemeshMeshTool {
    pub basic_properties: UPtr<URemeshMeshToolProperties>,
    pub mesh_statistics_properties: UPtr<UMeshStatisticsProperties>,
    pub preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub mesh_elements_display: UPtr<UMeshElementsVisualizer>,
}
pub struct UProjectToTargetTool {}
pub struct UNewMeshMaterialProperties {
    pub material: TWeakObjectPtr<UMaterialInterface>,
    pub uv_scale: f32,
    pub b_world_space_uv_scale: bool,
    pub b_show_wireframe: bool,
    pub b_show_extended_options: bool,
}
pub struct UExistingMeshMaterialProperties {
    pub material_mode: ESetMeshMaterialMode,
    pub checker_density: f32,
    pub override_material: UPtr<UMaterialInterface>,
    pub uv_channel: FString,
    pub uv_channel_names_list: TArray<FString>,
    pub checker_material: UPtr<UMaterialInstanceDynamic>,
}
pub struct UMeshEditingViewProperties {
    pub b_show_wireframe: bool,
    pub material_mode: EMeshEditingMaterialModes,
    pub b_flat_shading: bool,
    pub color: FLinearColor,
    pub image: UPtr<UTexture2D>,
    pub opacity: f64,
    pub transparent_material_color: FLinearColor,
    pub b_two_sided: bool,
    pub custom_material: TWeakObjectPtr<UMaterialInterface>,
}
pub struct UMeshSculptLayerProperties {
    pub active_layer: i32,
    pub layer_weights: TArray<f64>,
    pub b_can_edit_layers: bool,
}
pub struct UMeshStatisticsProperties {
    pub mesh: FString,
    pub uv: FString,
    pub attributes: FString,
}
pub struct UMeshUVChannelProperties {
    pub uv_channel: FString,
    pub uv_channel_names_list: TArray<FString>,
}
pub struct URecomputeUVsToolBuilder {}
pub struct URecomputeUVsTool {
    pub uv_channel_properties: UPtr<UMeshUVChannelProperties>,
    pub settings: UPtr<URecomputeUVsToolProperties>,
    pub polygroup_layer_properties: UPtr<UPolygroupLayersProperties>,
    pub material_settings: UPtr<UExistingMeshMaterialProperties>,
    pub b_create_uv_layout_view_on_setup: bool,
    pub uv_layout_view: UPtr<UUVLayoutPreview>,
    pub recompute_u_vs_op_factory: UPtr<URecomputeUVsOpFactory>,
    pub preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
}
pub struct URemeshMeshToolBuilder {}
pub struct URemoveOccludedTrianglesToolBuilder {}
pub struct URemoveOccludedTrianglesToolProperties {
    pub occlusion_test_method: EOcclusionCalculationUIMode,
    pub triangle_sampling: EOcclusionTriangleSamplingUIMode,
    pub winding_iso_value: f64,
    pub add_random_rays: i32,
    pub add_triangle_samples: i32,
    pub b_only_self_occlude: bool,
    pub shrink_removal: i32,
    pub min_area_island: f64,
    pub min_tri_count_island: i32,
    pub action: EOccludedAction,
}
pub struct URemoveOccludedTrianglesAdvancedProperties {}
pub struct URemoveOccludedTrianglesOperatorFactory {
    pub tool: UPtr<URemoveOccludedTrianglesTool>,
}
pub struct URemoveOccludedTrianglesTool {
    pub basic_properties: UPtr<URemoveOccludedTrianglesToolProperties>,
    pub polygroup_layers_properties: UPtr<UPolygroupLayersProperties>,
    pub advanced_properties: UPtr<URemoveOccludedTrianglesAdvancedProperties>,
    pub previews: TArray<UPtr<UMeshOpPreviewWithBackgroundCompute>>,
    pub preview_copies: TArray<UPtr<UPreviewMesh>>,
}
pub struct USculptBrushProperties {
    pub stroke_type: EMeshSculptStrokeType,
    pub brush_size: FBrushToolRadius,
    pub brush_falloff_amount: f32,
    pub b_show_falloff: bool,
    pub depth: f32,
    pub b_hit_back_faces: bool,
    pub flow_rate: f32,
    pub ray_density: f32,
    pub spacing: f32,
    pub lazyness: f32,
    pub b_show_per_brush_props: bool,
    pub b_show_lazyness: bool,
    pub b_show_flow_rate: bool,
    pub b_show_spacing: bool,
}
pub struct UKelvinBrushProperties {
    pub fall_off_distance: f32,
    pub stiffness: f32,
    pub incompressiblity: f32,
    pub brush_steps: i32,
}
pub struct UWorkPlaneProperties {
    pub b_property_set_enabled: bool,
    pub b_show_gizmo: bool,
    pub position: FVector,
    pub rotation: FQuat,
}
pub struct USculptMaxBrushProperties {
    pub max_height: f32,
    pub b_freeze_current_height: bool,
}
pub struct USmoothMeshToolProperties {
    pub smoothing_type: ESmoothMeshToolSmoothType,
}
pub struct UIterativeSmoothProperties {
    pub smoothing_per_step: f32,
    pub steps: i32,
    pub b_smooth_boundary: bool,
}
pub struct UDiffusionSmoothProperties {
    pub smoothing_per_step: f32,
    pub steps: i32,
    pub b_preserve_u_vs: bool,
}
pub struct UImplicitSmoothProperties {
    pub smooth_speed: f32,
    pub smoothness: f32,
    pub b_preserve_u_vs: bool,
    pub volume_correction: f32,
}
pub struct USmoothWeightMapSetProperties {
    pub min_smooth_multiplier: f32,
}
pub struct USmoothMeshTool {
    pub smooth_properties: UPtr<USmoothMeshToolProperties>,
    pub iterative_properties: UPtr<UIterativeSmoothProperties>,
    pub diffusion_properties: UPtr<UDiffusionSmoothProperties>,
    pub implicit_properties: UPtr<UImplicitSmoothProperties>,
    pub weight_map_properties: UPtr<USmoothWeightMapSetProperties>,
}
pub struct USmoothMeshToolBuilder {}
pub struct UPolyEditBevelEdgeProperties {
    pub bevel_distance: f64,
    pub subdivisions: i32,
    pub round_weight: f32,
    pub b_infer_material_id: bool,
    pub set_material_id: i32,
}
pub struct UPolyEditBevelEdgeActivity {
    pub bevel_properties: UPtr<UPolyEditBevelEdgeProperties>,
    pub activity_context: UPtr<UPolyEditActivityContext>,
}
pub struct UPolyEditCutProperties {
    pub orientation: EPolyEditCutPlaneOrientation,
    pub b_snap_to_vertices: bool,
}
pub struct UPolyEditCutFacesActivity {
    pub cut_properties: UPtr<UPolyEditCutProperties>,
    pub edit_preview: UPtr<UPolyEditPreviewMesh>,
    pub surface_path_mechanic: UPtr<UCollectSurfacePathMechanic>,
    pub activity_context: UPtr<UPolyEditActivityContext>,
}
pub struct UPolyEditExtrudeProperties {
    pub distance_mode: EPolyEditExtrudeDistanceMode,
    pub distance: f64,
    pub direction_mode: EPolyEditExtrudeModeOptions,
    pub direction: EPolyEditExtrudeDirection,
    pub max_distance_scale_factor: f64,
    pub b_shells_to_solids: bool,
    pub measure_direction: EPolyEditExtrudeDirection,
    pub b_use_colinearity_for_setting_border_groups: bool,
}
pub struct UPolyEditOffsetProperties {
    pub distance_mode: EPolyEditExtrudeDistanceMode,
    pub distance: f64,
    pub direction_mode: EPolyEditOffsetModeOptions,
    pub max_distance_scale_factor: f64,
    pub b_shells_to_solids: bool,
    pub measure_direction: EPolyEditExtrudeDirection,
    pub b_use_colinearity_for_setting_border_groups: bool,
}
pub struct UPolyEditPushPullProperties {
    pub distance_mode: EPolyEditExtrudeDistanceMode,
    pub distance: f64,
    pub direction_mode: EPolyEditPushPullModeOptions,
    pub single_direction: EPolyEditExtrudeDirection,
    pub max_distance_scale_factor: f64,
    pub b_shells_to_solids: bool,
    pub measure_direction: EPolyEditExtrudeDirection,
    pub b_use_colinearity_for_setting_border_groups: bool,
}
pub struct UPolyEditExtrudeActivity {
    pub extrude_properties: UPtr<UPolyEditExtrudeProperties>,
    pub offset_properties: UPtr<UPolyEditOffsetProperties>,
    pub push_pull_properties: UPtr<UPolyEditPushPullProperties>,
    pub extrude_height_mechanic: UPtr<UPlaneDistanceFromHitMechanic>,
    pub activity_context: UPtr<UPolyEditActivityContext>,
}
pub struct UPolyEditExtrudeEdgeActivityProperties {
    pub direction_mode: EPolyEditExtrudeEdgeDirectionMode,
    pub distance_mode: EPolyEditExtrudeEdgeDistanceMode,
    pub distance: f64,
    pub b_use_unselected_for_frames: bool,
    pub b_adjust_to_extrude_evenly: bool,
}
pub struct UPolyEditExtrudeEdgeActivity {
    pub settings: UPtr<UPolyEditExtrudeEdgeActivityProperties>,
    pub activity_context: UPtr<UPolyEditActivityContext>,
    pub extrude_frame_proxy: UPtr<UTransformProxy>,
    pub extrude_frame_gizmo: UPtr<UCombinedTransformGizmo>,
    pub single_direction_proxy: UPtr<UTransformProxy>,
    pub single_direction_gizmo: UPtr<UCombinedTransformGizmo>,
    pub preview_geometry: UPtr<UPreviewGeometry>,
}
pub struct UGroupEdgeInsertionProperties {
    pub insertion_mode: EGroupEdgeInsertionMode,
    pub b_continuous_insertion: bool,
    pub vertex_tolerance: f64,
}
pub struct UPolyEditInsertEdgeActivity {
    pub settings: UPtr<UGroupEdgeInsertionProperties>,
    pub activity_context: UPtr<UPolyEditActivityContext>,
}
pub struct UEdgeLoopInsertionProperties {
    pub position_mode: EEdgeLoopPositioningMode,
    pub insertion_mode: EEdgeLoopInsertionMode,
    pub num_loops: i32,
    pub proportion_offset: f64,
    pub distance_offset: f64,
    pub b_interactive: bool,
    pub b_flip_offset_direction: bool,
    pub b_highlight_problem_groups: bool,
    pub vertex_tolerance: f64,
}
pub struct UPolyEditInsertEdgeLoopActivity {
    pub settings: UPtr<UEdgeLoopInsertionProperties>,
    pub activity_context: UPtr<UPolyEditActivityContext>,
}
pub struct UPolyEditInsetOutsetProperties {
    pub softness: f32,
    pub b_boundary_only: bool,
    pub area_scale: f32,
    pub b_reproject: bool,
    pub b_outset: bool,
}
pub struct UPolyEditInsetOutsetActivity {
    pub settings: UPtr<UPolyEditInsetOutsetProperties>,
    pub edit_preview: UPtr<UPolyEditPreviewMesh>,
    pub curve_dist_mechanic: UPtr<USpatialCurveDistanceMechanic>,
    pub activity_context: UPtr<UPolyEditActivityContext>,
}
pub struct UPolyEditSetUVProperties {
    pub b_show_material: bool,
}
pub struct UPolyEditPlanarProjectionUVActivity {
    pub set_uv_properties: UPtr<UPolyEditSetUVProperties>,
    pub edit_preview: UPtr<UPolyEditPreviewMesh>,
    pub surface_path_mechanic: UPtr<UCollectSurfacePathMechanic>,
    pub activity_context: UPtr<UPolyEditActivityContext>,
}
pub struct UUVLayoutToolBuilder {}
pub struct UUVLayoutTool {
    pub uv_channel_properties: UPtr<UMeshUVChannelProperties>,
    pub basic_properties: UPtr<UUVLayoutProperties>,
    pub material_settings: UPtr<UExistingMeshMaterialProperties>,
    pub previews: TArray<UPtr<UMeshOpPreviewWithBackgroundCompute>>,
    pub factories: TArray<UPtr<UUVLayoutOperatorFactory>>,
    pub uv_layout_view: UPtr<UUVLayoutPreview>,
}
pub struct UUVProjectionToolBuilder {}
pub struct UUVProjectionToolEditActions {}
pub struct UUVProjectionToolProperties {
    pub projection_type: EUVProjectionMethod,
    pub dimensions: FVector,
    pub b_proportional_dimensions: bool,
    pub initialization: EUVProjectionToolInitializationMode,
    pub cylinder_split_angle: f32,
    pub exp_map_normal_blending: f32,
    pub exp_map_smoothing_steps: i32,
    pub exp_map_smoothing_alpha: f32,
    pub rotation: f32,
    pub scale: FVector2D,
    pub translation: FVector2D,
    pub saved_dimensions: FVector,
    pub b_saved_proportional_dimensions: bool,
    pub saved_transform: FTransform,
}
pub struct UUVProjectionOperatorFactory {
    pub tool: UPtr<UUVProjectionTool>,
}
pub struct UUVProjectionTool {
    pub uv_channel_properties: UPtr<UMeshUVChannelProperties>,
    pub basic_properties: UPtr<UUVProjectionToolProperties>,
    pub edit_actions: UPtr<UUVProjectionToolEditActions>,
    pub material_settings: UPtr<UExistingMeshMaterialProperties>,
    pub preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub checker_material: UPtr<UMaterialInstanceDynamic>,
    pub transform_gizmo: UPtr<UCombinedTransformGizmo>,
    pub transform_proxy: UPtr<UTransformProxy>,
    pub operator_factory: UPtr<UUVProjectionOperatorFactory>,
    pub edge_renderer: UPtr<UPreviewGeometry>,
    pub click_to_set_plane_behavior: UPtr<USingleClickInputBehavior>,
}
pub struct UWeldMeshEdgesToolBuilder {}
pub struct UWeldMeshEdgesToolProperties {
    pub tolerance: f32,
    pub b_only_unique: bool,
    pub b_resolve_t_junctions: bool,
    pub b_split_bowties: bool,
    pub initial_edges: i32,
    pub remaining_edges: i32,
    pub attr_welding_mode: EWeldMeshEdgesAttributeUIMode,
    pub split_normal_threshold: f32,
    pub split_tangents_threshold: f32,
    pub split_uv_threshold: f32,
    pub split_color_threshold: f32,
}
pub struct UWeldMeshEdgesOperatorFactory {
    pub weld_mesh_edges_tool: UPtr<UWeldMeshEdgesTool>,
}
pub struct UWeldMeshEdgesTool {
    pub settings: UPtr<UWeldMeshEdgesToolProperties>,
    pub preview_compute: UPtr<UMeshOpPreviewWithBackgroundCompute>,
    pub mesh_elements_display: UPtr<UMeshElementsVisualizer>,
    pub operator_factory: UPtr<UWeldMeshEdgesOperatorFactory>,
}
