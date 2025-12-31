#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FISMEditorTarget {
    pub transform_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub transform_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
}
pub struct UPivotActorTransformProperties {
    pub position: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FQuat,
}
pub struct UAddPivotActorToolBuilder {}
pub struct UAddPivotActorTool {
    pub drag_alignment_mechanic: UPtr<
        crate::bindings::modeling_components::UDragAlignmentMechanic,
    >,
    pub transform_gizmo: UPtr<
        crate::bindings::interactive_tools_framework::UCombinedTransformGizmo,
    >,
    pub transform_proxy: UPtr<
        crate::bindings::interactive_tools_framework::UTransformProxy,
    >,
    pub transform_properties: UPtr<UPivotActorTransformProperties>,
}
pub struct UBakeRenderCaptureResults {
    pub base_color_map: UPtr<crate::bindings::engine::UTexture2D>,
    pub normal_map: UPtr<crate::bindings::engine::UTexture2D>,
    pub packed_mrs_map: UPtr<crate::bindings::engine::UTexture2D>,
    pub metallic_map: UPtr<crate::bindings::engine::UTexture2D>,
    pub roughness_map: UPtr<crate::bindings::engine::UTexture2D>,
    pub specular_map: UPtr<crate::bindings::engine::UTexture2D>,
    pub emissive_map: UPtr<crate::bindings::engine::UTexture2D>,
    pub opacity_map: UPtr<crate::bindings::engine::UTexture2D>,
    pub subsurface_color_map: UPtr<crate::bindings::engine::UTexture2D>,
    pub device_depth_map: UPtr<crate::bindings::engine::UTexture2D>,
}
pub struct UBakeRenderCaptureToolBuilder {}
pub struct URenderCaptureProperties {
    pub resolution: crate::bindings::modeling_components::EBakeTextureResolution,
    pub b_base_color_map: bool,
    pub b_normal_map: bool,
    pub b_packed_mrs_map: bool,
    pub b_metallic_map: bool,
    pub b_roughness_map: bool,
    pub b_specular_map: bool,
    pub b_emissive_map: bool,
    pub b_opacity_map: bool,
    pub b_subsurface_color_map: bool,
    pub b_anti_aliasing: bool,
    pub b_device_depth_map: bool,
    pub capture_field_of_view: f32,
    pub near_plane_dist: f32,
}
pub struct UBakeRenderCaptureToolProperties {
    pub map_preview: FString,
    pub samples_per_pixel: crate::bindings::modeling_components::EBakeTextureSamplesPerPixel,
    pub texture_size: crate::bindings::modeling_components::EBakeTextureResolution,
    pub valid_sample_depth_threshold: f32,
    pub b_enable_map_preview: bool,
    pub map_preview_names_list: TArray<FString>,
}
pub struct UBakeRenderCaptureInputToolProperties {
    pub target_static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub target_uv_layer: FString,
    pub target_uv_layer_names_list: TArray<FString>,
}
pub struct UBakeRenderCaptureVisualizationProperties {
    pub b_preview_as_material: bool,
    pub brightness: f32,
    pub ss_brightness: f32,
    pub emissive_scale: f32,
}
pub struct UBakeRenderCaptureTool {
    pub actors: TArray<UPtr<crate::bindings::engine::AActor>>,
    pub settings: UPtr<UBakeRenderCaptureToolProperties>,
    pub render_capture_properties: UPtr<URenderCaptureProperties>,
    pub input_mesh_settings: UPtr<UBakeRenderCaptureInputToolProperties>,
    pub visualization_props: UPtr<UBakeRenderCaptureVisualizationProperties>,
    pub result_settings: UPtr<UBakeRenderCaptureResults>,
    pub empty_normal_map: UPtr<crate::bindings::engine::UTexture2D>,
    pub empty_color_map_black: UPtr<crate::bindings::engine::UTexture2D>,
    pub empty_color_map_white: UPtr<crate::bindings::engine::UTexture2D>,
    pub empty_emissive_map: UPtr<crate::bindings::engine::UTexture2D>,
    pub empty_opacity_map: UPtr<crate::bindings::engine::UTexture2D>,
    pub empty_subsurface_color_map: UPtr<crate::bindings::engine::UTexture2D>,
    pub empty_packed_mrs_map: UPtr<crate::bindings::engine::UTexture2D>,
    pub empty_roughness_map: UPtr<crate::bindings::engine::UTexture2D>,
    pub empty_metallic_map: UPtr<crate::bindings::engine::UTexture2D>,
    pub empty_specular_map: UPtr<crate::bindings::engine::UTexture2D>,
    pub working_preview_material: UPtr<
        crate::bindings::engine::UMaterialInstanceDynamic,
    >,
    pub error_preview_material: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    pub preview_material_rc: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    pub preview_material_packed_rc: UPtr<
        crate::bindings::engine::UMaterialInstanceDynamic,
    >,
    pub preview_material_rc_subsurface: UPtr<
        crate::bindings::engine::UMaterialInstanceDynamic,
    >,
    pub preview_material_packed_rc_subsurface: UPtr<
        crate::bindings::engine::UMaterialInstanceDynamic,
    >,
    pub preview_mesh: UPtr<crate::bindings::modeling_components::UPreviewMesh>,
}
pub struct UBspConversionToolBuilder {}
pub struct UBspConversionToolProperties {
    pub conversion_mode: EBspConversionMode,
    pub b_include_volumes: bool,
    pub b_remove_converted_volumes: bool,
    pub b_explicit_subtractive_brush_selection: bool,
    pub b_remove_converted_subtractive_brushes: bool,
    pub b_cache_brushes: bool,
    pub b_show_preview: bool,
}
pub struct UBspConversionToolActionPropertySet {}
pub struct UBspConversionTool {
    pub settings: UPtr<UBspConversionToolProperties>,
    pub tool_actions: UPtr<UBspConversionToolActionPropertySet>,
    pub preview_mesh: UPtr<crate::bindings::modeling_components::UPreviewMesh>,
}
pub struct UDrawSplineToolProperties {
    pub b_loop: bool,
    pub output_mode: EDrawSplineOutputMode,
    pub target_actor: TWeakObjectPtr<crate::bindings::engine::AActor>,
    pub blueprint_to_create: TWeakObjectPtr<crate::bindings::engine::UBlueprint>,
    pub existing_spline_index_to_replace: i32,
    pub draw_mode: EDrawSplineDrawMode,
    pub min_point_spacing: f64,
    pub click_offset: f64,
    pub offset_method: ESplineOffsetMethod,
    pub offset_direction: crate::bindings::core_u_object::FVector,
    pub frame_visualization_width: f64,
    pub up_vector_mode: EDrawSplineUpVectorMode,
    pub b_preview_using_actor_copy: bool,
    pub b_hit_world: bool,
    pub b_hit_custom_plane: bool,
    pub b_hit_ground_planes: bool,
    pub b_rerun_construction_script_on_drag: bool,
}
pub struct UDrawSplineTool {
    pub settings: UPtr<UDrawSplineToolProperties>,
    pub click_or_drag_behavior: UPtr<
        crate::bindings::interactive_tools_framework::USingleClickOrDragInputBehavior,
    >,
    pub plane_mechanic: UPtr<
        crate::bindings::modeling_components::UConstructionPlaneMechanic,
    >,
    pub preview_actor: UPtr<crate::bindings::engine::AActor>,
    pub previous_target_actor: UPtr<crate::bindings::engine::AActor>,
}
pub struct UDrawSplineToolBuilder {}
pub struct UEditMeshMaterialsToolBuilder {}
pub struct UEditMeshMaterialsToolProperties {
    pub active_material: FString,
    pub material_names_list: TArray<FString>,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
}
pub struct UEditMeshMaterialsEditActions {}
pub struct UEditMeshMaterialsTool {
    pub material_props: UPtr<UEditMeshMaterialsToolProperties>,
}
pub struct UHarvestInstancesToolBuilder {}
pub struct UHarvestInstancesToolSettings {
    pub b_draw_bounds: bool,
}
pub struct UHarvestInstancesTool_OutputSettings {
    pub component_type: EHarvestInstancesToolOutputType,
    pub b_single_actor: bool,
    pub actor_name: FString,
    pub b_include_single_instances: bool,
    pub b_delete_inputs: bool,
    pub b_have_single_instance_set: bool,
    pub b_have_loner_instances: bool,
    pub b_can_delete_inputs: bool,
}
pub struct UHarvestInstancesTool {
    pub settings: UPtr<UHarvestInstancesToolSettings>,
    pub output_settings: UPtr<UHarvestInstancesTool_OutputSettings>,
}
pub struct UISMEditorToolBuilder {}
pub struct UISMEditorToolProperties {
    pub transform_mode: EISMEditorTransformMode,
    pub b_set_pivot_mode: bool,
    pub b_show_selectable: bool,
    pub b_show_selected: bool,
    pub b_hide_when_dragging: bool,
    pub selected_instances: TArray<FString>,
}
pub struct UISMEditorToolActionPropertySetBase {}
pub struct UISMEditorToolActionPropertySet {}
pub struct UISMEditorToolReplacePropertySet {
    pub replace_with: UPtr<crate::bindings::engine::UStaticMesh>,
}
pub struct UISMEditorTool {
    pub click_or_drag_behavior: UPtr<
        crate::bindings::interactive_tools_framework::USingleClickOrDragInputBehavior,
    >,
    pub rectangle_marquee_mechanic: UPtr<
        crate::bindings::modeling_components::URectangleMarqueeMechanic,
    >,
    pub transform_props: UPtr<UISMEditorToolProperties>,
    pub tool_actions: UPtr<UISMEditorToolActionPropertySet>,
    pub replace_action: UPtr<UISMEditorToolReplacePropertySet>,
    pub target_components: TArray<
        UPtr<crate::bindings::engine::UInstancedStaticMeshComponent>,
    >,
    pub active_gizmos: TArray<FISMEditorTarget>,
    pub preview_geometry: UPtr<crate::bindings::modeling_components::UPreviewGeometry>,
}
pub struct UMergeMeshesToolProperties {
    pub voxel_count: i32,
    pub mesh_adaptivity: f32,
    pub offset_distance: f32,
    pub b_auto_simplify: bool,
}
pub struct UMergeMeshesTool {
    pub merge_props: UPtr<UMergeMeshesToolProperties>,
}
pub struct UMergeMeshesToolBuilder {}
pub struct UMeshTangentsToolBuilder {}
pub struct UMeshTangentsToolProperties {
    pub calculation_method: crate::bindings::modeling_operators_editor_only::EMeshTangentsType,
    pub b_show_tangents: bool,
    pub b_show_normals: bool,
    pub line_length: f32,
    pub line_thickness: f32,
    pub b_show_degenerates: bool,
    pub b_compare_with_mikkt: bool,
    pub compare_with_mikkt_threshold: f32,
}
pub struct UMeshTangentsTool {
    pub settings: UPtr<UMeshTangentsToolProperties>,
    pub uv_channel_properties: UPtr<
        crate::bindings::mesh_modeling_tools::UMeshUVChannelProperties,
    >,
    pub default_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub preview_mesh: UPtr<crate::bindings::modeling_components::UPreviewMesh>,
    pub preview_geometry: UPtr<crate::bindings::modeling_components::UPreviewGeometry>,
    pub geometry_selection_viz_properties: UPtr<
        crate::bindings::modeling_components::UGeometrySelectionVisualizationProperties,
    >,
    pub geometry_selection_viz: UPtr<
        crate::bindings::modeling_components::UPreviewGeometry,
    >,
}
pub struct UMeshToVolumeToolBuilder {}
pub struct UMeshToVolumeToolProperties {
    pub conversion_mode: EMeshToVolumeMode,
    pub b_preserve_group_boundaries: bool,
    pub b_auto_simplify: bool,
    pub simplify_max_triangles: i32,
    pub new_volume_type: TSubclassOf<crate::bindings::engine::AVolume>,
    pub target_volume: TLazyObjectPtr<crate::bindings::engine::AVolume>,
}
pub struct UMeshToVolumeTool {
    pub settings: UPtr<UMeshToVolumeToolProperties>,
    pub handle_sources_properties: UPtr<
        crate::bindings::modeling_components::UOnAcceptHandleSourcesPropertiesSingle,
    >,
    pub preview_mesh: UPtr<crate::bindings::modeling_components::UPreviewMesh>,
    pub volume_edges_set: UPtr<crate::bindings::modeling_components::ULineSetComponent>,
}
pub struct UShapeSprayToolBuilder {}
pub struct UShapeSprayToolProperties {
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub b_random_color: bool,
    pub drop_speed: f32,
    pub object_size: f32,
    pub num_splats: i32,
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
}
pub struct UShapeSprayTool {
    pub settings: UPtr<UShapeSprayToolProperties>,
    pub accum_mesh_component: UPtr<
        crate::bindings::geometry_framework::UDynamicMeshComponent,
    >,
}
pub struct USubdividePolyToolBuilder {}
pub struct USubdividePolyToolProperties {
    pub subdivision_level: i32,
    pub subdivision_scheme: crate::bindings::modeling_components_editor_only::ESubdivisionScheme,
    pub boundary_scheme: crate::bindings::modeling_components_editor_only::ESubdivisionBoundaryScheme,
    pub normal_computation_method: crate::bindings::modeling_components_editor_only::ESubdivisionOutputNormals,
    pub uv_computation_method: crate::bindings::modeling_components_editor_only::ESubdivisionOutputUVs,
    pub b_new_poly_groups: bool,
    pub b_render_groups: bool,
    pub b_render_cage: bool,
    pub b_add_extra_corners: bool,
    pub extra_corner_angle_threshold_degrees: f64,
    pub b_overridden_subdivision_scheme: bool,
}
pub struct USubdividePolyTool {
    pub preview_mesh: UPtr<crate::bindings::modeling_components::UPreviewMesh>,
    pub properties: UPtr<USubdividePolyToolProperties>,
    pub preview_geometry: UPtr<crate::bindings::modeling_components::UPreviewGeometry>,
}
pub struct UVoxelCSGMeshesToolProperties {
    pub operation: EVoxelCSGOperation,
    pub voxel_count: i32,
    pub mesh_adaptivity: f32,
    pub offset_distance: f32,
    pub b_auto_simplify: bool,
}
pub struct UVoxelCSGMeshesTool {
    pub csg_props: UPtr<UVoxelCSGMeshesToolProperties>,
}
pub struct UVoxelCSGMeshesToolBuilder {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBspConversionMode(pub u8);
impl EBspConversionMode {
    pub const CONVERT_FIRST: EBspConversionMode = EBspConversionMode(0);
    pub const COMBINE_FIRST: EBspConversionMode = EBspConversionMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDrawSplineOutputMode(pub u8);
impl EDrawSplineOutputMode {
    pub const EMPTY_ACTOR: EDrawSplineOutputMode = EDrawSplineOutputMode(0);
    pub const EXISTING_ACTOR: EDrawSplineOutputMode = EDrawSplineOutputMode(1);
    pub const CREATE_BLUEPRINT: EDrawSplineOutputMode = EDrawSplineOutputMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDrawSplineDrawMode(pub u8);
impl EDrawSplineDrawMode {
    pub const TANGENT_DRAG: EDrawSplineDrawMode = EDrawSplineDrawMode(0);
    pub const CLICK_AUTO_TANGENT: EDrawSplineDrawMode = EDrawSplineDrawMode(1);
    pub const FREE_DRAW: EDrawSplineDrawMode = EDrawSplineDrawMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESplineOffsetMethod(pub u8);
impl ESplineOffsetMethod {
    pub const HIT_NORMAL: ESplineOffsetMethod = ESplineOffsetMethod(0);
    pub const CUSTOM: ESplineOffsetMethod = ESplineOffsetMethod(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDrawSplineUpVectorMode(pub u8);
impl EDrawSplineUpVectorMode {
    pub const ALIGN_TO_PREVIOUS: EDrawSplineUpVectorMode = EDrawSplineUpVectorMode(0);
    pub const USE_HIT_NORMAL: EDrawSplineUpVectorMode = EDrawSplineUpVectorMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EHarvestInstancesToolOutputType(pub i32);
impl EHarvestInstancesToolOutputType {
    pub const HISMC: EHarvestInstancesToolOutputType = EHarvestInstancesToolOutputType(
        0,
    );
    pub const ISMC: EHarvestInstancesToolOutputType = EHarvestInstancesToolOutputType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EISMEditorTransformMode(pub u8);
impl EISMEditorTransformMode {
    pub const SHARED_GIZMO: EISMEditorTransformMode = EISMEditorTransformMode(0);
    pub const SHARED_GIZMO_LOCAL: EISMEditorTransformMode = EISMEditorTransformMode(1);
    pub const PER_OBJECT_GIZMO: EISMEditorTransformMode = EISMEditorTransformMode(2);
    pub const LAST_VALUE: EISMEditorTransformMode = EISMEditorTransformMode(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeshToVolumeMode(pub i32);
impl EMeshToVolumeMode {
    pub const TRIANGULATE_POLYGONS: EMeshToVolumeMode = EMeshToVolumeMode(0);
    pub const MINIMAL_POLYGONS: EMeshToVolumeMode = EMeshToVolumeMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EVoxelCSGOperation(pub u8);
impl EVoxelCSGOperation {
    pub const DIFFERENCE_AB: EVoxelCSGOperation = EVoxelCSGOperation(0);
    pub const DIFFERENCE_BA: EVoxelCSGOperation = EVoxelCSGOperation(1);
    pub const INTERSECT: EVoxelCSGOperation = EVoxelCSGOperation(2);
    pub const UNION: EVoxelCSGOperation = EVoxelCSGOperation(3);
}
