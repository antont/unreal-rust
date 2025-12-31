#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
    pub axis_filters: crate::bindings::modeling_components::FModelingToolsAxisFilter,
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
    pub color: crate::bindings::core_u_object::FLinearColor,
}
pub struct UVertexColorSoftenBrushOpProps {}
pub struct UVertexColorSmoothBrushOpProps {}
pub struct UPolyEditActivityContext {
    pub common_properties: UPtr<UPolyEditCommonProperties>,
    pub preview: UPtr<
        crate::bindings::modeling_components::UMeshOpPreviewWithBackgroundCompute,
    >,
    pub selection_mechanic: UPtr<
        crate::bindings::modeling_components::UPolygonSelectionMechanic,
    >,
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
    pub output_type_properties: UPtr<
        crate::bindings::modeling_components::UCreateMeshObjectTypeProperties,
    >,
    pub shape_settings: UPtr<UProceduralShapeToolProperties>,
    pub material_properties: UPtr<UNewMeshMaterialProperties>,
    pub preview_mesh: UPtr<crate::bindings::modeling_components::UPreviewMesh>,
    pub gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
    pub drag_alignment_mechanic: UPtr<
        crate::bindings::modeling_components::UDragAlignmentMechanic,
    >,
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
    pub output_write_to: crate::bindings::modeling_components::EBaseCreateFromSelectedTargetType,
    pub output_new_name: FString,
    pub output_existing_name: FString,
}
pub struct UCombineMeshesTool {
    pub basic_properties: UPtr<UCombineMeshesToolProperties>,
    pub output_type_properties: UPtr<
        crate::bindings::modeling_components::UCreateMeshObjectTypeProperties,
    >,
    pub handle_source_properties: UPtr<
        crate::bindings::modeling_components::UOnAcceptHandleSourcesPropertiesBase,
    >,
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
    pub copy_from_layer_properties: UPtr<
        crate::bindings::modeling_components::UPolygroupLayersProperties,
    >,
    pub preview_compute: UPtr<
        crate::bindings::modeling_components::UMeshOpPreviewWithBackgroundCompute,
    >,
    pub preview_geometry: UPtr<crate::bindings::modeling_components::UPreviewGeometry>,
    pub unmodified_area_preview_mesh: UPtr<
        crate::bindings::modeling_components::UPreviewMesh,
    >,
}
pub struct UCSGMeshesToolProperties {
    pub operation: crate::bindings::modeling_operators::ECSGOperation,
    pub b_try_fix_holes: bool,
    pub b_try_collapse_edges: bool,
    pub winding_threshold: f32,
    pub b_show_new_boundaries: bool,
    pub b_show_subtracted_mesh: bool,
    pub subtracted_mesh_opacity: f32,
    pub subtracted_mesh_color: crate::bindings::core_u_object::FLinearColor,
    pub b_use_first_mesh_materials: bool,
}
pub struct UTrimMeshesToolProperties {
    pub which_mesh: crate::bindings::modeling_operators::ETrimOperation,
    pub trim_side: crate::bindings::modeling_operators::ETrimSide,
    pub winding_threshold: f32,
    pub b_show_trimming_mesh: bool,
    pub opacity_of_trimming_mesh: f32,
    pub color_of_trimming_mesh: crate::bindings::core_u_object::FLinearColor,
}
pub struct UCSGMeshesTool {
    pub csg_properties: UPtr<UCSGMeshesToolProperties>,
    pub trim_properties: UPtr<UTrimMeshesToolProperties>,
    pub original_mesh_previews: TArray<
        UPtr<crate::bindings::modeling_components::UPreviewMesh>,
    >,
    pub previews_ghost_material: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    pub drawn_line_set: UPtr<crate::bindings::modeling_components::ULineSetComponent>,
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
    pub intersect_preview_mesh: UPtr<crate::bindings::modeling_components::UPreviewMesh>,
    pub drawn_line_set: UPtr<crate::bindings::modeling_components::ULineSetComponent>,
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
    pub preview_mesh_actor: UPtr<
        crate::bindings::interactive_tools_framework::AInternalToolFrameworkActor,
    >,
    pub dynamic_mesh_component: UPtr<
        crate::bindings::geometry_framework::UDynamicMeshComponent,
    >,
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
    pub displacement_map: UPtr<crate::bindings::engine::UTexture2D>,
    pub channel: EDisplaceMeshToolChannelType,
    pub displacement_map_base_value: f32,
    pub uv_scale: crate::bindings::core_u_object::FVector2D,
    pub uv_offset: crate::bindings::core_u_object::FVector2D,
    pub b_apply_adjustment_curve: bool,
    pub adjustment_curve: UPtr<crate::bindings::engine::UCurveFloat>,
    pub b_recalc_normals: bool,
}
pub struct UDisplaceMeshDirectionalFilterProperties {
    pub b_enable_filter: bool,
    pub filter_direction: crate::bindings::core_u_object::FVector,
    pub filter_width: f32,
}
pub struct UDisplaceMeshPerlinNoiseProperties {
    pub perlin_layer_properties: TArray<FPerlinLayerProperties>,
}
pub struct UDisplaceMeshSineWaveProperties {
    pub sine_wave_frequency: f32,
    pub sine_wave_phase_shift: f32,
    pub sine_wave_direction: crate::bindings::core_u_object::FVector,
}
pub struct UDisplaceMeshToolBuilder {}
pub struct UDisplaceMeshTool {
    pub common_properties: UPtr<UDisplaceMeshCommonProperties>,
    pub directional_filter_properties: UPtr<UDisplaceMeshDirectionalFilterProperties>,
    pub texture_map_properties: UPtr<UDisplaceMeshTextureMapProperties>,
    pub noise_properties: UPtr<UDisplaceMeshPerlinNoiseProperties>,
    pub sine_wave_properties: UPtr<UDisplaceMeshSineWaveProperties>,
    pub selective_tessellation_properties: UPtr<USelectiveTessellationProperties>,
    pub active_contrast_curve_target: UPtr<crate::bindings::engine::UCurveFloat>,
    pub mesh_statistics: UPtr<UMeshStatisticsProperties>,
    pub preview_mesh_actor: UPtr<
        crate::bindings::interactive_tools_framework::AInternalToolFrameworkActor,
    >,
    pub dynamic_mesh_component: UPtr<
        crate::bindings::geometry_framework::UDynamicMeshComponent,
    >,
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
    pub draw_plane_origin: crate::bindings::core_u_object::FVector,
    pub draw_plane_orientation: crate::bindings::core_u_object::FRotator,
    pub b_enable_snapping: bool,
    pub b_allowed_to_edit_draw_plane: bool,
}
pub struct URevolveOperatorFactory {
    pub revolve_tool: UPtr<UDrawAndRevolveTool>,
}
pub struct UDrawAndRevolveTool {
    pub control_points_mechanic: UPtr<
        crate::bindings::modeling_components::UCurveControlPointsMechanic,
    >,
    pub plane_mechanic: UPtr<
        crate::bindings::modeling_components::UConstructionPlaneMechanic,
    >,
    pub output_type_properties: UPtr<
        crate::bindings::modeling_components::UCreateMeshObjectTypeProperties,
    >,
    pub settings: UPtr<URevolveToolProperties>,
    pub material_properties: UPtr<UNewMeshMaterialProperties>,
    pub preview: UPtr<
        crate::bindings::modeling_components::UMeshOpPreviewWithBackgroundCompute,
    >,
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
    pub output_type_properties: UPtr<
        crate::bindings::modeling_components::UCreateMeshObjectTypeProperties,
    >,
    pub polygon_properties: UPtr<UDrawPolygonToolStandardProperties>,
    pub snap_properties: UPtr<UDrawPolygonToolSnapProperties>,
    pub material_properties: UPtr<UNewMeshMaterialProperties>,
    pub preview_mesh: UPtr<crate::bindings::modeling_components::UPreviewMesh>,
    pub height_mechanic: UPtr<
        crate::bindings::modeling_components::UPlaneDistanceFromHitMechanic,
    >,
    pub drag_alignment_mechanic: UPtr<
        crate::bindings::modeling_components::UDragAlignmentMechanic,
    >,
    pub plane_mechanic: UPtr<
        crate::bindings::modeling_components::UConstructionPlaneMechanic,
    >,
}
pub struct UDynamicMeshBrushTool {
    pub preview_mesh: UPtr<crate::bindings::modeling_components::UPreviewMesh>,
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
    pub position: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FQuat,
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
    pub brush_indicator: UPtr<
        crate::bindings::interactive_tools_framework::UBrushStampIndicator,
    >,
    pub brush_indicator_material: UPtr<
        crate::bindings::engine::UMaterialInstanceDynamic,
    >,
    pub brush_indicator_mesh: UPtr<crate::bindings::modeling_components::UPreviewMesh>,
    pub preview_mesh_actor: UPtr<
        crate::bindings::interactive_tools_framework::AInternalToolFrameworkActor,
    >,
    pub dynamic_mesh_component: UPtr<
        crate::bindings::modeling_components::UOctreeDynamicMeshComponent,
    >,
    pub active_override_material: UPtr<
        crate::bindings::engine::UMaterialInstanceDynamic,
    >,
    pub plane_transform_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
    pub plane_transform_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
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
    pub preview: UPtr<
        crate::bindings::modeling_components::UMeshOpPreviewWithBackgroundCompute,
    >,
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
    pub selection_mechanic: UPtr<
        crate::bindings::modeling_components::UPolygonSelectionMechanic,
    >,
    pub drag_alignment_mechanic: UPtr<
        crate::bindings::modeling_components::UDragAlignmentMechanic,
    >,
    pub transform_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
    pub transform_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
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
    pub fill_type: crate::bindings::modeling_operators::EHoleFillOpFillType,
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
    pub preview: UPtr<
        crate::bindings::modeling_components::UMeshOpPreviewWithBackgroundCompute,
    >,
    pub selection_mechanic: UPtr<
        crate::bindings::modeling_components::UBoundarySelectionMechanic,
    >,
}
pub struct ULatticeDeformerToolBuilder {}
pub struct ULatticeDeformerToolProperties {
    pub x_axis_resolution: i32,
    pub y_axis_resolution: i32,
    pub z_axis_resolution: i32,
    pub padding: f32,
    pub interpolation_type: crate::bindings::modeling_components::ELatticeInterpolationType,
    pub b_deform_normals: bool,
    pub b_can_change_resolution: bool,
    pub gizmo_coordinate_system: crate::bindings::interactive_tools_framework::EToolContextCoordinateSystem,
    pub b_set_pivot_mode: bool,
    pub b_soft_deformation: bool,
}
pub struct ULatticeDeformerOperatorFactory {
    pub lattice_deformer_tool: UPtr<ULatticeDeformerTool>,
}
pub struct ULatticeDeformerTool {
    pub control_points_mechanic: UPtr<
        crate::bindings::modeling_components::ULatticeControlPointsMechanic,
    >,
    pub settings: UPtr<ULatticeDeformerToolProperties>,
    pub preview: UPtr<
        crate::bindings::modeling_components::UMeshOpPreviewWithBackgroundCompute,
    >,
    pub sculpt_layer_properties: UPtr<UMeshSculptLayerProperties>,
    pub lattice_storage: TScriptInterface<
        crate::bindings::modeling_components::ILatticeStateStorage,
    >,
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
    pub active_override_material: UPtr<
        crate::bindings::engine::UMaterialInstanceDynamic,
    >,
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
    pub stroke_geometry: UPtr<crate::bindings::modeling_components::UPreviewGeometry>,
    pub view_properties: UPtr<UMeshEditingViewProperties>,
    pub active_override_material: UPtr<
        crate::bindings::engine::UMaterialInstanceDynamic,
    >,
    pub brush_indicator: UPtr<
        crate::bindings::interactive_tools_framework::UBrushStampIndicator,
    >,
    pub b_is_volumetric_indicator: bool,
    pub brush_indicator_material: UPtr<
        crate::bindings::engine::UMaterialInstanceDynamic,
    >,
    pub brush_indicator_mesh: UPtr<crate::bindings::modeling_components::UPreviewMesh>,
    pub plane_transform_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
    pub plane_transform_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
}
pub struct UMeshGroupPaintTool {
    pub polygroup_layer_properties: UPtr<
        crate::bindings::modeling_components::UPolygroupLayersProperties,
    >,
    pub filter_properties: UPtr<UGroupPaintBrushFilterProperties>,
    pub paint_brush_op_properties: UPtr<UGroupPaintBrushOpProps>,
    pub erase_brush_op_properties: UPtr<UGroupEraseBrushOpProps>,
    pub freeze_actions: UPtr<UMeshGroupPaintToolFreezeActions>,
    pub poly_lasso_mechanic: UPtr<
        crate::bindings::modeling_components::UPolyLassoMarqueeMechanic,
    >,
    pub preview_mesh_actor: UPtr<
        crate::bindings::interactive_tools_framework::AInternalToolFrameworkActor,
    >,
    pub dynamic_mesh_component: UPtr<
        crate::bindings::geometry_framework::UDynamicMeshComponent,
    >,
    pub mesh_elements_display: UPtr<
        crate::bindings::modeling_components::UMeshElementsVisualizer,
    >,
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
    pub state_target: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoTransformChangeStateTarget,
    >,
    pub drag_alignment_mechanic: UPtr<
        crate::bindings::modeling_components::UDragAlignmentMechanic,
    >,
    pub preview: UPtr<
        crate::bindings::modeling_components::UMeshOpPreviewWithBackgroundCompute,
    >,
    pub original_mesh_preview: UPtr<crate::bindings::modeling_components::UPreviewMesh>,
    pub interval_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UIntervalGizmo,
    >,
    pub transform_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
    pub transform_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub up_interval_source: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoLocalFloatParameterSource,
    >,
    pub down_interval_source: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoLocalFloatParameterSource,
    >,
    pub forward_interval_source: UPtr<
        crate::bindings::interactive_tools_framework::UGizmoLocalFloatParameterSource,
    >,
}
pub struct UMeshVertexPaintToolBuilder {}
pub struct UVertexPaintBasicProperties {
    pub primary_brush_type: EMeshVertexPaintBrushType,
    pub sub_tool_type: EMeshVertexPaintInteractionType,
    pub strength: f64,
    pub paint_color: crate::bindings::core_u_object::FLinearColor,
    pub b_is_paint_pressure_enabled: bool,
    pub blend_mode: EMeshVertexPaintColorBlendMode,
    pub secondary_action_type: EMeshVertexPaintSecondaryActionType,
    pub erase_color: crate::bindings::core_u_object::FLinearColor,
    pub b_is_erase_pressure_enabled: bool,
    pub smooth_strength: f32,
    pub channel_filter: crate::bindings::modeling_components::FModelingToolsColorChannelFilter,
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
    pub target_channels: crate::bindings::modeling_components::FModelingToolsColorChannelFilter,
    pub target_channel: EMeshVertexPaintColorChannel,
    pub b_copy_to_hi_res: bool,
    pub copy_to_lod_name: FString,
    pub lod_names_list: TArray<FString>,
}
pub struct UMeshVertexPaintTool {
    pub polygroup_layer_properties: UPtr<
        crate::bindings::modeling_components::UPolygroupLayersProperties,
    >,
    pub basic_properties: UPtr<UVertexPaintBasicProperties>,
    pub filter_properties: UPtr<UVertexPaintBrushFilterProperties>,
    pub symmetry_properties: UPtr<UMeshSymmetryProperties>,
    pub paint_brush_op_properties: UPtr<UVertexColorPaintBrushOpProps>,
    pub erase_brush_op_properties: UPtr<UVertexColorPaintBrushOpProps>,
    pub quick_actions: UPtr<UMeshVertexPaintToolQuickActions>,
    pub utility_actions: UPtr<UMeshVertexPaintToolUtilityActions>,
    pub poly_lasso_mechanic: UPtr<
        crate::bindings::modeling_components::UPolyLassoMarqueeMechanic,
    >,
    pub preview_mesh_actor: UPtr<
        crate::bindings::interactive_tools_framework::AInternalToolFrameworkActor,
    >,
    pub dynamic_mesh_component: UPtr<
        crate::bindings::geometry_framework::UDynamicMeshComponent,
    >,
    pub mesh_elements_display: UPtr<
        crate::bindings::modeling_components::UMeshElementsVisualizer,
    >,
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
    pub alpha: UPtr<crate::bindings::engine::UTexture2D>,
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
    pub brush_alpha: UPtr<crate::bindings::engine::UTexture2D>,
    pub symmetry_properties: UPtr<UMeshSymmetryProperties>,
    pub sculpt_layer_properties: UPtr<UMeshSculptLayerProperties>,
    pub preview_mesh_actor: UPtr<
        crate::bindings::interactive_tools_framework::AInternalToolFrameworkActor,
    >,
    pub dynamic_mesh_component: UPtr<
        crate::bindings::geometry_framework::UDynamicMeshComponent,
    >,
    pub octree_geometry: UPtr<crate::bindings::modeling_components::UPreviewGeometry>,
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
    pub smoothing_type: crate::bindings::modeling_operators::ERemeshSmoothingType,
    pub b_discard_attributes: bool,
    pub b_show_group_colors: bool,
    pub remesh_type: crate::bindings::modeling_operators::ERemeshType,
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
    pub preview: UPtr<
        crate::bindings::modeling_components::UMeshOpPreviewWithBackgroundCompute,
    >,
    pub mesh_elements_display: UPtr<
        crate::bindings::modeling_components::UMeshElementsVisualizer,
    >,
}
pub struct UProjectToTargetTool {}
pub struct UNewMeshMaterialProperties {
    pub material: TWeakObjectPtr<crate::bindings::engine::UMaterialInterface>,
    pub uv_scale: f32,
    pub b_world_space_uv_scale: bool,
    pub b_show_wireframe: bool,
    pub b_show_extended_options: bool,
}
pub struct UExistingMeshMaterialProperties {
    pub material_mode: ESetMeshMaterialMode,
    pub checker_density: f32,
    pub override_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub uv_channel: FString,
    pub uv_channel_names_list: TArray<FString>,
    pub checker_material: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
}
pub struct UMeshEditingViewProperties {
    pub b_show_wireframe: bool,
    pub material_mode: EMeshEditingMaterialModes,
    pub b_flat_shading: bool,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub image: UPtr<crate::bindings::engine::UTexture2D>,
    pub opacity: f64,
    pub transparent_material_color: crate::bindings::core_u_object::FLinearColor,
    pub b_two_sided: bool,
    pub custom_material: TWeakObjectPtr<crate::bindings::engine::UMaterialInterface>,
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
    pub settings: UPtr<crate::bindings::modeling_operators::URecomputeUVsToolProperties>,
    pub polygroup_layer_properties: UPtr<
        crate::bindings::modeling_components::UPolygroupLayersProperties,
    >,
    pub material_settings: UPtr<UExistingMeshMaterialProperties>,
    pub b_create_uv_layout_view_on_setup: bool,
    pub uv_layout_view: UPtr<crate::bindings::modeling_components::UUVLayoutPreview>,
    pub recompute_u_vs_op_factory: UPtr<
        crate::bindings::modeling_operators::URecomputeUVsOpFactory,
    >,
    pub preview: UPtr<
        crate::bindings::modeling_components::UMeshOpPreviewWithBackgroundCompute,
    >,
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
    pub polygroup_layers_properties: UPtr<
        crate::bindings::modeling_components::UPolygroupLayersProperties,
    >,
    pub advanced_properties: UPtr<URemoveOccludedTrianglesAdvancedProperties>,
    pub previews: TArray<
        UPtr<crate::bindings::modeling_components::UMeshOpPreviewWithBackgroundCompute>,
    >,
    pub preview_copies: TArray<UPtr<crate::bindings::modeling_components::UPreviewMesh>>,
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
    pub position: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FQuat,
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
    pub edit_preview: UPtr<crate::bindings::modeling_components::UPolyEditPreviewMesh>,
    pub surface_path_mechanic: UPtr<
        crate::bindings::modeling_components::UCollectSurfacePathMechanic,
    >,
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
    pub extrude_height_mechanic: UPtr<
        crate::bindings::modeling_components::UPlaneDistanceFromHitMechanic,
    >,
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
    pub extrude_frame_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub extrude_frame_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
    pub single_direction_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub single_direction_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
    pub preview_geometry: UPtr<crate::bindings::modeling_components::UPreviewGeometry>,
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
    pub edit_preview: UPtr<crate::bindings::modeling_components::UPolyEditPreviewMesh>,
    pub curve_dist_mechanic: UPtr<
        crate::bindings::modeling_components::USpatialCurveDistanceMechanic,
    >,
    pub activity_context: UPtr<UPolyEditActivityContext>,
}
pub struct UPolyEditSetUVProperties {
    pub b_show_material: bool,
}
pub struct UPolyEditPlanarProjectionUVActivity {
    pub set_uv_properties: UPtr<UPolyEditSetUVProperties>,
    pub edit_preview: UPtr<crate::bindings::modeling_components::UPolyEditPreviewMesh>,
    pub surface_path_mechanic: UPtr<
        crate::bindings::modeling_components::UCollectSurfacePathMechanic,
    >,
    pub activity_context: UPtr<UPolyEditActivityContext>,
}
pub struct UUVLayoutToolBuilder {}
pub struct UUVLayoutTool {
    pub uv_channel_properties: UPtr<UMeshUVChannelProperties>,
    pub basic_properties: UPtr<crate::bindings::modeling_operators::UUVLayoutProperties>,
    pub material_settings: UPtr<UExistingMeshMaterialProperties>,
    pub previews: TArray<
        UPtr<crate::bindings::modeling_components::UMeshOpPreviewWithBackgroundCompute>,
    >,
    pub factories: TArray<
        UPtr<crate::bindings::modeling_operators::UUVLayoutOperatorFactory>,
    >,
    pub uv_layout_view: UPtr<crate::bindings::modeling_components::UUVLayoutPreview>,
}
pub struct UUVProjectionToolBuilder {}
pub struct UUVProjectionToolEditActions {}
pub struct UUVProjectionToolProperties {
    pub projection_type: crate::bindings::modeling_operators::EUVProjectionMethod,
    pub dimensions: crate::bindings::core_u_object::FVector,
    pub b_proportional_dimensions: bool,
    pub initialization: EUVProjectionToolInitializationMode,
    pub cylinder_split_angle: f32,
    pub exp_map_normal_blending: f32,
    pub exp_map_smoothing_steps: i32,
    pub exp_map_smoothing_alpha: f32,
    pub rotation: f32,
    pub scale: crate::bindings::core_u_object::FVector2D,
    pub translation: crate::bindings::core_u_object::FVector2D,
    pub saved_dimensions: crate::bindings::core_u_object::FVector,
    pub b_saved_proportional_dimensions: bool,
    pub saved_transform: crate::bindings::core_u_object::FTransform,
}
pub struct UUVProjectionOperatorFactory {
    pub tool: UPtr<UUVProjectionTool>,
}
pub struct UUVProjectionTool {
    pub uv_channel_properties: UPtr<UMeshUVChannelProperties>,
    pub basic_properties: UPtr<UUVProjectionToolProperties>,
    pub edit_actions: UPtr<UUVProjectionToolEditActions>,
    pub material_settings: UPtr<UExistingMeshMaterialProperties>,
    pub preview: UPtr<
        crate::bindings::modeling_components::UMeshOpPreviewWithBackgroundCompute,
    >,
    pub checker_material: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    pub transform_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
    pub transform_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub operator_factory: UPtr<UUVProjectionOperatorFactory>,
    pub edge_renderer: UPtr<crate::bindings::modeling_components::UPreviewGeometry>,
    pub click_to_set_plane_behavior: UPtr<
        crate::bindings::interactive_tools_framework::USingleClickInputBehavior,
    >,
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
    pub preview_compute: UPtr<
        crate::bindings::modeling_components::UMeshOpPreviewWithBackgroundCompute,
    >,
    pub mesh_elements_display: UPtr<
        crate::bindings::modeling_components::UMeshElementsVisualizer,
    >,
    pub operator_factory: UPtr<UWeldMeshEdgesOperatorFactory>,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBrushToolSizeType(pub u8);
impl EBrushToolSizeType {
    pub const ADAPTIVE: EBrushToolSizeType = EBrushToolSizeType(0);
    pub const WORLD: EBrushToolSizeType = EBrushToolSizeType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshVertexPaintBrushAreaType(pub u8);
impl EMeshVertexPaintBrushAreaType {
    pub const CONNECTED: EMeshVertexPaintBrushAreaType = EMeshVertexPaintBrushAreaType(
        0,
    );
    pub const VOLUMETRIC: EMeshVertexPaintBrushAreaType = EMeshVertexPaintBrushAreaType(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPlaneBrushSideMode(pub u8);
impl EPlaneBrushSideMode {
    pub const BOTH_SIDES: EPlaneBrushSideMode = EPlaneBrushSideMode(0);
    pub const PUSH_DOWN: EPlaneBrushSideMode = EPlaneBrushSideMode(1);
    pub const PULL_TOWARDS: EPlaneBrushSideMode = EPlaneBrushSideMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EVertexColorPaintBrushOpBlendMode(pub i32);
impl EVertexColorPaintBrushOpBlendMode {
    pub const LERP: EVertexColorPaintBrushOpBlendMode = EVertexColorPaintBrushOpBlendMode(
        0,
    );
    pub const MIX: EVertexColorPaintBrushOpBlendMode = EVertexColorPaintBrushOpBlendMode(
        1,
    );
    pub const MULTIPLY: EVertexColorPaintBrushOpBlendMode = EVertexColorPaintBrushOpBlendMode(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshBoundaryConstraint(pub u8);
impl EMeshBoundaryConstraint {
    pub const FIXED: EMeshBoundaryConstraint = EMeshBoundaryConstraint(7);
    pub const REFINE: EMeshBoundaryConstraint = EMeshBoundaryConstraint(5);
    pub const FREE: EMeshBoundaryConstraint = EMeshBoundaryConstraint(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroupBoundaryConstraint(pub u8);
impl EGroupBoundaryConstraint {
    pub const FIXED: EGroupBoundaryConstraint = EGroupBoundaryConstraint(7);
    pub const REFINE: EGroupBoundaryConstraint = EGroupBoundaryConstraint(5);
    pub const FREE: EGroupBoundaryConstraint = EGroupBoundaryConstraint(1);
    pub const IGNORE: EGroupBoundaryConstraint = EGroupBoundaryConstraint(0);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMaterialBoundaryConstraint(pub u8);
impl EMaterialBoundaryConstraint {
    pub const FIXED: EMaterialBoundaryConstraint = EMaterialBoundaryConstraint(7);
    pub const REFINE: EMaterialBoundaryConstraint = EMaterialBoundaryConstraint(5);
    pub const FREE: EMaterialBoundaryConstraint = EMaterialBoundaryConstraint(1);
    pub const IGNORE: EMaterialBoundaryConstraint = EMaterialBoundaryConstraint(0);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMakeMeshPolygroupMode(pub u8);
impl EMakeMeshPolygroupMode {
    pub const PER_SHAPE: EMakeMeshPolygroupMode = EMakeMeshPolygroupMode(0);
    pub const PER_FACE: EMakeMeshPolygroupMode = EMakeMeshPolygroupMode(1);
    pub const PER_QUAD: EMakeMeshPolygroupMode = EMakeMeshPolygroupMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMakeMeshPlacementType(pub u8);
impl EMakeMeshPlacementType {
    pub const GROUND_PLANE: EMakeMeshPlacementType = EMakeMeshPlacementType(0);
    pub const ON_SCENE: EMakeMeshPlacementType = EMakeMeshPlacementType(1);
    pub const AT_ORIGIN: EMakeMeshPlacementType = EMakeMeshPlacementType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMakeMeshPivotLocation(pub u8);
impl EMakeMeshPivotLocation {
    pub const BASE: EMakeMeshPivotLocation = EMakeMeshPivotLocation(0);
    pub const CENTERED: EMakeMeshPivotLocation = EMakeMeshPivotLocation(1);
    pub const TOP: EMakeMeshPivotLocation = EMakeMeshPivotLocation(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EProceduralRectType(pub i32);
impl EProceduralRectType {
    pub const RECTANGLE: EProceduralRectType = EProceduralRectType(0);
    pub const ROUNDED_RECTANGLE: EProceduralRectType = EProceduralRectType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EProceduralDiscType(pub i32);
impl EProceduralDiscType {
    pub const DISC: EProceduralDiscType = EProceduralDiscType(0);
    pub const PUNCTURED_DISC: EProceduralDiscType = EProceduralDiscType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EProceduralSphereType(pub i32);
impl EProceduralSphereType {
    pub const LAT_LONG: EProceduralSphereType = EProceduralSphereType(0);
    pub const BOX: EProceduralSphereType = EProceduralSphereType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EProceduralStairsType(pub i32);
impl EProceduralStairsType {
    pub const LINEAR: EProceduralStairsType = EProceduralStairsType(0);
    pub const FLOATING: EProceduralStairsType = EProceduralStairsType(1);
    pub const CURVED: EProceduralStairsType = EProceduralStairsType(2);
    pub const SPIRAL: EProceduralStairsType = EProceduralStairsType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EConvertToPolygonsMode(pub i32);
impl EConvertToPolygonsMode {
    pub const FACE_NORMAL_DEVIATION: EConvertToPolygonsMode = EConvertToPolygonsMode(0);
    pub const FIND_POLYGONS: EConvertToPolygonsMode = EConvertToPolygonsMode(1);
    pub const FROM_MATERIAL_I_DS: EConvertToPolygonsMode = EConvertToPolygonsMode(7);
    pub const FROM_UV_ISLANDS: EConvertToPolygonsMode = EConvertToPolygonsMode(2);
    pub const FROM_NORMAL_SEAMS: EConvertToPolygonsMode = EConvertToPolygonsMode(3);
    pub const FROM_CONNECTED_TRIS: EConvertToPolygonsMode = EConvertToPolygonsMode(4);
    pub const FROM_FURTHEST_POINT_SAMPLING: EConvertToPolygonsMode = EConvertToPolygonsMode(
        5,
    );
    pub const COPY_FROM_LAYER: EConvertToPolygonsMode = EConvertToPolygonsMode(6);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroupTopologyDeformationStrategy(pub u8);
impl EGroupTopologyDeformationStrategy {
    pub const LINEAR: EGroupTopologyDeformationStrategy = EGroupTopologyDeformationStrategy(
        0,
    );
    pub const LAPLACIAN: EGroupTopologyDeformationStrategy = EGroupTopologyDeformationStrategy(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EQuickTransformerMode(pub u8);
impl EQuickTransformerMode {
    pub const AXIS_TRANSLATION: EQuickTransformerMode = EQuickTransformerMode(0);
    pub const AXIS_ROTATION: EQuickTransformerMode = EQuickTransformerMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWeightScheme(pub i32);
impl EWeightScheme {
    pub const UNIFORM: EWeightScheme = EWeightScheme(0);
    pub const UMBRELLA: EWeightScheme = EWeightScheme(1);
    pub const VALENCE: EWeightScheme = EWeightScheme(2);
    pub const MEAN_VALUE: EWeightScheme = EWeightScheme(3);
    pub const COTANGENT: EWeightScheme = EWeightScheme(4);
    pub const CLAMPED_COTANGENT: EWeightScheme = EWeightScheme(5);
    pub const IDT_COTANGENT: EWeightScheme = EWeightScheme(6);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDisplaceMeshToolDisplaceType(pub u8);
impl EDisplaceMeshToolDisplaceType {
    pub const CONSTANT: EDisplaceMeshToolDisplaceType = EDisplaceMeshToolDisplaceType(0);
    pub const DISPLACEMENT_MAP: EDisplaceMeshToolDisplaceType = EDisplaceMeshToolDisplaceType(
        1,
    );
    pub const RANDOM_NOISE: EDisplaceMeshToolDisplaceType = EDisplaceMeshToolDisplaceType(
        2,
    );
    pub const PERLIN_NOISE: EDisplaceMeshToolDisplaceType = EDisplaceMeshToolDisplaceType(
        3,
    );
    pub const SINE_WAVE: EDisplaceMeshToolDisplaceType = EDisplaceMeshToolDisplaceType(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDisplaceMeshToolSubdivisionType(pub u8);
impl EDisplaceMeshToolSubdivisionType {
    pub const FLAT: EDisplaceMeshToolSubdivisionType = EDisplaceMeshToolSubdivisionType(
        0,
    );
    pub const PN_TRIANGLES: EDisplaceMeshToolSubdivisionType = EDisplaceMeshToolSubdivisionType(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDisplaceMeshToolTriangleSelectionType(pub u8);
impl EDisplaceMeshToolTriangleSelectionType {
    pub const NONE: EDisplaceMeshToolTriangleSelectionType = EDisplaceMeshToolTriangleSelectionType(
        0,
    );
    pub const MATERIAL: EDisplaceMeshToolTriangleSelectionType = EDisplaceMeshToolTriangleSelectionType(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDisplaceMeshToolChannelType(pub u8);
impl EDisplaceMeshToolChannelType {
    pub const RED: EDisplaceMeshToolChannelType = EDisplaceMeshToolChannelType(0);
    pub const GREEN: EDisplaceMeshToolChannelType = EDisplaceMeshToolChannelType(1);
    pub const BLUE: EDisplaceMeshToolChannelType = EDisplaceMeshToolChannelType(2);
    pub const ALPHA: EDisplaceMeshToolChannelType = EDisplaceMeshToolChannelType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERevolvePropertiesPolygroupMode(pub u8);
impl ERevolvePropertiesPolygroupMode {
    pub const PER_SHAPE: ERevolvePropertiesPolygroupMode = ERevolvePropertiesPolygroupMode(
        0,
    );
    pub const PER_FACE: ERevolvePropertiesPolygroupMode = ERevolvePropertiesPolygroupMode(
        1,
    );
    pub const PER_REVOLVE_STEP: ERevolvePropertiesPolygroupMode = ERevolvePropertiesPolygroupMode(
        2,
    );
    pub const PER_PATH_SEGMENT: ERevolvePropertiesPolygroupMode = ERevolvePropertiesPolygroupMode(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERevolvePropertiesQuadSplit(pub u8);
impl ERevolvePropertiesQuadSplit {
    pub const UNIFORM: ERevolvePropertiesQuadSplit = ERevolvePropertiesQuadSplit(0);
    pub const COMPACT: ERevolvePropertiesQuadSplit = ERevolvePropertiesQuadSplit(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERevolvePropertiesCapFillMode(pub u8);
impl ERevolvePropertiesCapFillMode {
    pub const NONE: ERevolvePropertiesCapFillMode = ERevolvePropertiesCapFillMode(0);
    pub const CENTER_FAN: ERevolvePropertiesCapFillMode = ERevolvePropertiesCapFillMode(
        1,
    );
    pub const DELAUNAY: ERevolvePropertiesCapFillMode = ERevolvePropertiesCapFillMode(2);
    pub const EAR_CLIPPING: ERevolvePropertiesCapFillMode = ERevolvePropertiesCapFillMode(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDrawPolygonDrawMode(pub u8);
impl EDrawPolygonDrawMode {
    pub const FREEHAND: EDrawPolygonDrawMode = EDrawPolygonDrawMode(0);
    pub const CIRCLE: EDrawPolygonDrawMode = EDrawPolygonDrawMode(1);
    pub const SQUARE: EDrawPolygonDrawMode = EDrawPolygonDrawMode(2);
    pub const RECTANGLE: EDrawPolygonDrawMode = EDrawPolygonDrawMode(3);
    pub const ROUNDED_RECTANGLE: EDrawPolygonDrawMode = EDrawPolygonDrawMode(4);
    pub const RING: EDrawPolygonDrawMode = EDrawPolygonDrawMode(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDrawPolygonExtrudeMode(pub u8);
impl EDrawPolygonExtrudeMode {
    pub const FLAT: EDrawPolygonExtrudeMode = EDrawPolygonExtrudeMode(0);
    pub const FIXED: EDrawPolygonExtrudeMode = EDrawPolygonExtrudeMode(1);
    pub const INTERACTIVE: EDrawPolygonExtrudeMode = EDrawPolygonExtrudeMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDynamicMeshSculptBrushType(pub u8);
impl EDynamicMeshSculptBrushType {
    pub const MOVE: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(0);
    pub const PULL_KELVIN: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(1);
    pub const PULL_SHARP_KELVIN: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(
        2,
    );
    pub const SMOOTH: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(3);
    pub const OFFSET: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(4);
    pub const SCULPT_VIEW: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(5);
    pub const SCULPT_MAX: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(6);
    pub const INFLATE: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(7);
    pub const SCALE_KELVIN: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(8);
    pub const PINCH: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(9);
    pub const TWIST_KELVIN: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(
        10,
    );
    pub const FLATTEN: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(11);
    pub const PLANE: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(12);
    pub const PLANE_VIEW_ALIGNED: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(
        13,
    );
    pub const FIXED_PLANE: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(14);
    pub const RESAMPLE: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(15);
    pub const LAST_VALUE: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(16);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELocalFrameMode(pub i32);
impl ELocalFrameMode {
    pub const FROM_OBJECT: ELocalFrameMode = ELocalFrameMode(0);
    pub const FROM_GEOMETRY: ELocalFrameMode = ELocalFrameMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBrushActionMode(pub i32);
impl EBrushActionMode {
    pub const PAINT: EBrushActionMode = EBrushActionMode(0);
    pub const FLOOD_FILL: EBrushActionMode = EBrushActionMode(1);
    pub const ERASE: EBrushActionMode = EBrushActionMode(2);
    pub const SMOOTH: EBrushActionMode = EBrushActionMode(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshAttributePaintMaterialMode(pub i32);
impl EMeshAttributePaintMaterialMode {
    pub const SHADED: EMeshAttributePaintMaterialMode = EMeshAttributePaintMaterialMode(
        0,
    );
    pub const COLOR_ONLY: EMeshAttributePaintMaterialMode = EMeshAttributePaintMaterialMode(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshGroupPaintBrushType(pub u8);
impl EMeshGroupPaintBrushType {
    pub const PAINT: EMeshGroupPaintBrushType = EMeshGroupPaintBrushType(0);
    pub const ERASE: EMeshGroupPaintBrushType = EMeshGroupPaintBrushType(1);
    pub const LAST_VALUE: EMeshGroupPaintBrushType = EMeshGroupPaintBrushType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshGroupPaintInteractionType(pub u8);
impl EMeshGroupPaintInteractionType {
    pub const BRUSH: EMeshGroupPaintInteractionType = EMeshGroupPaintInteractionType(0);
    pub const FILL: EMeshGroupPaintInteractionType = EMeshGroupPaintInteractionType(1);
    pub const GROUP_FILL: EMeshGroupPaintInteractionType = EMeshGroupPaintInteractionType(
        2,
    );
    pub const POLY_LASSO: EMeshGroupPaintInteractionType = EMeshGroupPaintInteractionType(
        3,
    );
    pub const LAST_VALUE: EMeshGroupPaintInteractionType = EMeshGroupPaintInteractionType(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshGroupPaintBrushAreaType(pub u8);
impl EMeshGroupPaintBrushAreaType {
    pub const CONNECTED: EMeshGroupPaintBrushAreaType = EMeshGroupPaintBrushAreaType(0);
    pub const VOLUMETRIC: EMeshGroupPaintBrushAreaType = EMeshGroupPaintBrushAreaType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshGroupPaintVisibilityType(pub u8);
impl EMeshGroupPaintVisibilityType {
    pub const NONE: EMeshGroupPaintVisibilityType = EMeshGroupPaintVisibilityType(0);
    pub const FRONT_FACING: EMeshGroupPaintVisibilityType = EMeshGroupPaintVisibilityType(
        1,
    );
    pub const UNOCCLUDED: EMeshGroupPaintVisibilityType = EMeshGroupPaintVisibilityType(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENonlinearOperationType(pub i8);
impl ENonlinearOperationType {
    pub const BEND: ENonlinearOperationType = ENonlinearOperationType(0);
    pub const FLARE: ENonlinearOperationType = ENonlinearOperationType(1);
    pub const TWIST: ENonlinearOperationType = ENonlinearOperationType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFlareProfileType(pub i8);
impl EFlareProfileType {
    pub const SIN_MODE: EFlareProfileType = EFlareProfileType(0);
    pub const SIN_SQUARED_MODE: EFlareProfileType = EFlareProfileType(1);
    pub const TRIANGLE_MODE: EFlareProfileType = EFlareProfileType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshVertexPaintBrushType(pub u8);
impl EMeshVertexPaintBrushType {
    pub const PAINT: EMeshVertexPaintBrushType = EMeshVertexPaintBrushType(0);
    pub const ERASE: EMeshVertexPaintBrushType = EMeshVertexPaintBrushType(1);
    pub const SOFTEN: EMeshVertexPaintBrushType = EMeshVertexPaintBrushType(2);
    pub const SMOOTH: EMeshVertexPaintBrushType = EMeshVertexPaintBrushType(3);
    pub const LAST_VALUE: EMeshVertexPaintBrushType = EMeshVertexPaintBrushType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshVertexPaintInteractionType(pub u8);
impl EMeshVertexPaintInteractionType {
    pub const BRUSH: EMeshVertexPaintInteractionType = EMeshVertexPaintInteractionType(
        0,
    );
    pub const TRI_FILL: EMeshVertexPaintInteractionType = EMeshVertexPaintInteractionType(
        1,
    );
    pub const FILL: EMeshVertexPaintInteractionType = EMeshVertexPaintInteractionType(2);
    pub const GROUP_FILL: EMeshVertexPaintInteractionType = EMeshVertexPaintInteractionType(
        3,
    );
    pub const POLY_LASSO: EMeshVertexPaintInteractionType = EMeshVertexPaintInteractionType(
        4,
    );
    pub const LAST_VALUE: EMeshVertexPaintInteractionType = EMeshVertexPaintInteractionType(
        5,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshVertexPaintColorBlendMode(pub u8);
impl EMeshVertexPaintColorBlendMode {
    pub const LERP: EMeshVertexPaintColorBlendMode = EMeshVertexPaintColorBlendMode(0);
    pub const MIX: EMeshVertexPaintColorBlendMode = EMeshVertexPaintColorBlendMode(1);
    pub const MULTIPLY: EMeshVertexPaintColorBlendMode = EMeshVertexPaintColorBlendMode(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshVertexPaintSecondaryActionType(pub u8);
impl EMeshVertexPaintSecondaryActionType {
    pub const ERASE: EMeshVertexPaintSecondaryActionType = EMeshVertexPaintSecondaryActionType(
        0,
    );
    pub const SOFTEN: EMeshVertexPaintSecondaryActionType = EMeshVertexPaintSecondaryActionType(
        1,
    );
    pub const SMOOTH: EMeshVertexPaintSecondaryActionType = EMeshVertexPaintSecondaryActionType(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshVertexPaintVisibilityType(pub u8);
impl EMeshVertexPaintVisibilityType {
    pub const NONE: EMeshVertexPaintVisibilityType = EMeshVertexPaintVisibilityType(0);
    pub const FRONT_FACING: EMeshVertexPaintVisibilityType = EMeshVertexPaintVisibilityType(
        1,
    );
    pub const UNOCCLUDED: EMeshVertexPaintVisibilityType = EMeshVertexPaintVisibilityType(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshVertexPaintMaterialMode(pub u8);
impl EMeshVertexPaintMaterialMode {
    pub const LIT_VERTEX_COLOR: EMeshVertexPaintMaterialMode = EMeshVertexPaintMaterialMode(
        0,
    );
    pub const UNLIT_VERTEX_COLOR: EMeshVertexPaintMaterialMode = EMeshVertexPaintMaterialMode(
        1,
    );
    pub const ORIGINAL_MATERIAL: EMeshVertexPaintMaterialMode = EMeshVertexPaintMaterialMode(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshVertexPaintToolUtilityOperations(pub i32);
impl EMeshVertexPaintToolUtilityOperations {
    pub const BLEND_ALL_SEAMS: EMeshVertexPaintToolUtilityOperations = EMeshVertexPaintToolUtilityOperations(
        0,
    );
    pub const FILL_CHANNELS: EMeshVertexPaintToolUtilityOperations = EMeshVertexPaintToolUtilityOperations(
        1,
    );
    pub const INVERT_CHANNELS: EMeshVertexPaintToolUtilityOperations = EMeshVertexPaintToolUtilityOperations(
        2,
    );
    pub const COPY_CHANNEL_TO_CHANNEL: EMeshVertexPaintToolUtilityOperations = EMeshVertexPaintToolUtilityOperations(
        3,
    );
    pub const SWAP_CHANNELS: EMeshVertexPaintToolUtilityOperations = EMeshVertexPaintToolUtilityOperations(
        4,
    );
    pub const COPY_FROM_WEIGHT_MAP: EMeshVertexPaintToolUtilityOperations = EMeshVertexPaintToolUtilityOperations(
        5,
    );
    pub const COPY_TO_OTHER_LO_DS: EMeshVertexPaintToolUtilityOperations = EMeshVertexPaintToolUtilityOperations(
        6,
    );
    pub const COPY_TO_SINGLE_LOD: EMeshVertexPaintToolUtilityOperations = EMeshVertexPaintToolUtilityOperations(
        7,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshVertexPaintColorChannel(pub u8);
impl EMeshVertexPaintColorChannel {
    pub const RED: EMeshVertexPaintColorChannel = EMeshVertexPaintColorChannel(0);
    pub const GREEN: EMeshVertexPaintColorChannel = EMeshVertexPaintColorChannel(1);
    pub const BLUE: EMeshVertexPaintColorChannel = EMeshVertexPaintColorChannel(2);
    pub const ALPHA: EMeshVertexPaintColorChannel = EMeshVertexPaintColorChannel(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshVertexSculptBrushType(pub u8);
impl EMeshVertexSculptBrushType {
    pub const MOVE: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(0);
    pub const PULL_KELVIN: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(1);
    pub const PULL_SHARP_KELVIN: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(
        2,
    );
    pub const SMOOTH: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(3);
    pub const SMOOTH_FILL: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(4);
    pub const OFFSET: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(5);
    pub const SCULPT_VIEW: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(6);
    pub const SCULPT_MAX: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(7);
    pub const INFLATE: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(8);
    pub const INFLATE_STROKE: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(9);
    pub const INFLATE_MAX: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(10);
    pub const SCALE_KELVIN: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(11);
    pub const PINCH: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(12);
    pub const TWIST_KELVIN: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(13);
    pub const FLATTEN: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(14);
    pub const PLANE: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(15);
    pub const PLANE_VIEW_ALIGNED: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(
        16,
    );
    pub const FIXED_PLANE: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(17);
    pub const ERASE_SCULPT_LAYER: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(
        18,
    );
    pub const LAST_VALUE: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(19);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshSculptFalloffType(pub u8);
impl EMeshSculptFalloffType {
    pub const SMOOTH: EMeshSculptFalloffType = EMeshSculptFalloffType(0);
    pub const LINEAR: EMeshSculptFalloffType = EMeshSculptFalloffType(1);
    pub const INVERSE: EMeshSculptFalloffType = EMeshSculptFalloffType(2);
    pub const ROUND: EMeshSculptFalloffType = EMeshSculptFalloffType(3);
    pub const BOX_SMOOTH: EMeshSculptFalloffType = EMeshSculptFalloffType(4);
    pub const BOX_LINEAR: EMeshSculptFalloffType = EMeshSculptFalloffType(5);
    pub const BOX_INVERSE: EMeshSculptFalloffType = EMeshSculptFalloffType(6);
    pub const BOX_ROUND: EMeshSculptFalloffType = EMeshSculptFalloffType(7);
    pub const LAST_VALUE: EMeshSculptFalloffType = EMeshSculptFalloffType(8);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshVertexSculptBrushFilterType(pub u8);
impl EMeshVertexSculptBrushFilterType {
    pub const NONE: EMeshVertexSculptBrushFilterType = EMeshVertexSculptBrushFilterType(
        0,
    );
    pub const COMPONENT: EMeshVertexSculptBrushFilterType = EMeshVertexSculptBrushFilterType(
        1,
    );
    pub const POLY_GROUP: EMeshVertexSculptBrushFilterType = EMeshVertexSculptBrushFilterType(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EOffsetMeshToolOffsetType(pub u8);
impl EOffsetMeshToolOffsetType {
    pub const ITERATIVE: EOffsetMeshToolOffsetType = EOffsetMeshToolOffsetType(0);
    pub const IMPLICIT: EOffsetMeshToolOffsetType = EOffsetMeshToolOffsetType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESetMeshMaterialMode(pub u8);
impl ESetMeshMaterialMode {
    pub const ORIGINAL: ESetMeshMaterialMode = ESetMeshMaterialMode(0);
    pub const CHECKERBOARD: ESetMeshMaterialMode = ESetMeshMaterialMode(1);
    pub const OVERRIDE: ESetMeshMaterialMode = ESetMeshMaterialMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshEditingMaterialModes(pub i32);
impl EMeshEditingMaterialModes {
    pub const EXISTING_MATERIAL: EMeshEditingMaterialModes = EMeshEditingMaterialModes(
        0,
    );
    pub const DIFFUSE: EMeshEditingMaterialModes = EMeshEditingMaterialModes(1);
    pub const GREY: EMeshEditingMaterialModes = EMeshEditingMaterialModes(2);
    pub const SOFT: EMeshEditingMaterialModes = EMeshEditingMaterialModes(3);
    pub const TRANSPARENT: EMeshEditingMaterialModes = EMeshEditingMaterialModes(4);
    pub const TANGENT_NORMAL: EMeshEditingMaterialModes = EMeshEditingMaterialModes(5);
    pub const VERTEX_COLOR: EMeshEditingMaterialModes = EMeshEditingMaterialModes(6);
    pub const CUSTOM_IMAGE: EMeshEditingMaterialModes = EMeshEditingMaterialModes(7);
    pub const CUSTOM: EMeshEditingMaterialModes = EMeshEditingMaterialModes(8);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EOcclusionCalculationUIMode(pub u8);
impl EOcclusionCalculationUIMode {
    pub const GENERALIZED_WINDING_NUMBER: EOcclusionCalculationUIMode = EOcclusionCalculationUIMode(
        0,
    );
    pub const RAYCAST_OCCLUSION_SAMPLES: EOcclusionCalculationUIMode = EOcclusionCalculationUIMode(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EOcclusionTriangleSamplingUIMode(pub u8);
impl EOcclusionTriangleSamplingUIMode {
    pub const VERTICES: EOcclusionTriangleSamplingUIMode = EOcclusionTriangleSamplingUIMode(
        0,
    );
    pub const VERTICES_AND_CENTROIDS: EOcclusionTriangleSamplingUIMode = EOcclusionTriangleSamplingUIMode(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EOccludedAction(pub u8);
impl EOccludedAction {
    pub const REMOVE: EOccludedAction = EOccludedAction(0);
    pub const SET_NEW_GROUP: EOccludedAction = EOccludedAction(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshSculptStrokeType(pub u8);
impl EMeshSculptStrokeType {
    pub const SPACING: EMeshSculptStrokeType = EMeshSculptStrokeType(0);
    pub const AIRBRUSH: EMeshSculptStrokeType = EMeshSculptStrokeType(1);
    pub const DOTS: EMeshSculptStrokeType = EMeshSculptStrokeType(2);
    pub const LAST_VALUE: EMeshSculptStrokeType = EMeshSculptStrokeType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESmoothMeshToolSmoothType(pub u8);
impl ESmoothMeshToolSmoothType {
    pub const ITERATIVE: ESmoothMeshToolSmoothType = ESmoothMeshToolSmoothType(0);
    pub const IMPLICIT: ESmoothMeshToolSmoothType = ESmoothMeshToolSmoothType(1);
    pub const DIFFUSION: ESmoothMeshToolSmoothType = ESmoothMeshToolSmoothType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPolyEditCutPlaneOrientation(pub i32);
impl EPolyEditCutPlaneOrientation {
    pub const FACE_NORMALS: EPolyEditCutPlaneOrientation = EPolyEditCutPlaneOrientation(
        0,
    );
    pub const VIEW_DIRECTION: EPolyEditCutPlaneOrientation = EPolyEditCutPlaneOrientation(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPolyEditExtrudeDistanceMode(pub i32);
impl EPolyEditExtrudeDistanceMode {
    pub const CLICK_IN_VIEWPORT: EPolyEditExtrudeDistanceMode = EPolyEditExtrudeDistanceMode(
        0,
    );
    pub const FIXED: EPolyEditExtrudeDistanceMode = EPolyEditExtrudeDistanceMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPolyEditExtrudeModeOptions(pub i32);
impl EPolyEditExtrudeModeOptions {
    pub const SINGLE_DIRECTION: EPolyEditExtrudeModeOptions = EPolyEditExtrudeModeOptions(
        3,
    );
    pub const SELECTED_TRIANGLE_NORMALS: EPolyEditExtrudeModeOptions = EPolyEditExtrudeModeOptions(
        0,
    );
    pub const SELECTED_TRIANGLE_NORMALS_EVEN: EPolyEditExtrudeModeOptions = EPolyEditExtrudeModeOptions(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPolyEditExtrudeDirection(pub i32);
impl EPolyEditExtrudeDirection {
    pub const SELECTION_NORMAL: EPolyEditExtrudeDirection = EPolyEditExtrudeDirection(0);
    pub const WORLD_X: EPolyEditExtrudeDirection = EPolyEditExtrudeDirection(1);
    pub const WORLD_Y: EPolyEditExtrudeDirection = EPolyEditExtrudeDirection(2);
    pub const WORLD_Z: EPolyEditExtrudeDirection = EPolyEditExtrudeDirection(3);
    pub const LOCAL_X: EPolyEditExtrudeDirection = EPolyEditExtrudeDirection(4);
    pub const LOCAL_Y: EPolyEditExtrudeDirection = EPolyEditExtrudeDirection(5);
    pub const LOCAL_Z: EPolyEditExtrudeDirection = EPolyEditExtrudeDirection(6);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPolyEditOffsetModeOptions(pub i32);
impl EPolyEditOffsetModeOptions {
    pub const VERTEX_NORMALS: EPolyEditOffsetModeOptions = EPolyEditOffsetModeOptions(2);
    pub const SELECTED_TRIANGLE_NORMALS: EPolyEditOffsetModeOptions = EPolyEditOffsetModeOptions(
        0,
    );
    pub const SELECTED_TRIANGLE_NORMALS_EVEN: EPolyEditOffsetModeOptions = EPolyEditOffsetModeOptions(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPolyEditPushPullModeOptions(pub i32);
impl EPolyEditPushPullModeOptions {
    pub const SELECTED_TRIANGLE_NORMALS: EPolyEditPushPullModeOptions = EPolyEditPushPullModeOptions(
        0,
    );
    pub const SELECTED_TRIANGLE_NORMALS_EVEN: EPolyEditPushPullModeOptions = EPolyEditPushPullModeOptions(
        1,
    );
    pub const SINGLE_DIRECTION: EPolyEditPushPullModeOptions = EPolyEditPushPullModeOptions(
        3,
    );
    pub const VERTEX_NORMALS: EPolyEditPushPullModeOptions = EPolyEditPushPullModeOptions(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPolyEditExtrudeEdgeDirectionMode(pub i32);
impl EPolyEditExtrudeEdgeDirectionMode {
    pub const LOCAL_EXTRUDE_FRAMES: EPolyEditExtrudeEdgeDirectionMode = EPolyEditExtrudeEdgeDirectionMode(
        0,
    );
    pub const SINGLE_DIRECTION: EPolyEditExtrudeEdgeDirectionMode = EPolyEditExtrudeEdgeDirectionMode(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPolyEditExtrudeEdgeDistanceMode(pub i32);
impl EPolyEditExtrudeEdgeDistanceMode {
    pub const FIXED: EPolyEditExtrudeEdgeDistanceMode = EPolyEditExtrudeEdgeDistanceMode(
        0,
    );
    pub const GIZMO: EPolyEditExtrudeEdgeDistanceMode = EPolyEditExtrudeEdgeDistanceMode(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroupEdgeInsertionMode(pub i32);
impl EGroupEdgeInsertionMode {
    pub const RETRIANGULATE: EGroupEdgeInsertionMode = EGroupEdgeInsertionMode(0);
    pub const PLANE_CUT: EGroupEdgeInsertionMode = EGroupEdgeInsertionMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EEdgeLoopPositioningMode(pub i32);
impl EEdgeLoopPositioningMode {
    pub const EVEN: EEdgeLoopPositioningMode = EEdgeLoopPositioningMode(0);
    pub const PROPORTION_OFFSET: EEdgeLoopPositioningMode = EEdgeLoopPositioningMode(1);
    pub const DISTANCE_OFFSET: EEdgeLoopPositioningMode = EEdgeLoopPositioningMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EEdgeLoopInsertionMode(pub i32);
impl EEdgeLoopInsertionMode {
    pub const RETRIANGULATE: EEdgeLoopInsertionMode = EEdgeLoopInsertionMode(0);
    pub const PLANE_CUT: EEdgeLoopInsertionMode = EEdgeLoopInsertionMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EUVProjectionToolInitializationMode(pub i32);
impl EUVProjectionToolInitializationMode {
    pub const DEFAULT: EUVProjectionToolInitializationMode = EUVProjectionToolInitializationMode(
        0,
    );
    pub const USE_PREVIOUS: EUVProjectionToolInitializationMode = EUVProjectionToolInitializationMode(
        1,
    );
    pub const AUTO_FIT: EUVProjectionToolInitializationMode = EUVProjectionToolInitializationMode(
        2,
    );
    pub const AUTO_FIT_ALIGN: EUVProjectionToolInitializationMode = EUVProjectionToolInitializationMode(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWeldMeshEdgesAttributeUIMode(pub u8);
impl EWeldMeshEdgesAttributeUIMode {
    pub const NONE: EWeldMeshEdgesAttributeUIMode = EWeldMeshEdgesAttributeUIMode(0);
    pub const ON_WELDED_MESH_EDGES_ONLY: EWeldMeshEdgesAttributeUIMode = EWeldMeshEdgesAttributeUIMode(
        1,
    );
    pub const ON_FULL_MESH: EWeldMeshEdgesAttributeUIMode = EWeldMeshEdgesAttributeUIMode(
        2,
    );
}
