#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FFractureModeCustomSectionColor {
    pub section_name: FString,
    pub color: FLinearColor,
}
#[repr(C, align(8))]
pub struct FFractureModeCustomToolColor {
    pub tool_name: FString,
    pub color: FLinearColor,
}
pub struct UFractureEditorMode {}
pub struct UFractureModeCustomizationSettings {
    pub tool_section_order: TArray<FString>,
    pub tool_favorites: TArray<FString>,
    pub section_colors: TArray<FFractureModeCustomSectionColor>,
    pub tool_colors: TArray<FFractureModeCustomToolColor>,
}
pub struct UFractureModeSettings {
    pub new_asset_location: EFractureModeNewAssetLocation,
    pub convex_can_exceed_fraction: f32,
    pub convex_simplification_distance_threshold: f32,
    pub convex_remove_overlaps: EConvexOverlapRemoval,
    pub convex_overlap_removal_shrink_percent: f32,
    pub convex_fraction_allow_remove: f64,
    pub proximity_method: EProximityMethod,
    pub proximity_distance_threshold: f32,
    pub b_proximity_use_as_connection_graph: bool,
    pub proximity_connection_contact_area_method: EConnectionContactMethod,
    pub proximity_contact_method: EProximityContactMethod,
    pub proximity_contact_threshold: f32,
}
pub struct UFractureSettings {
    pub explode_amount: f32,
    pub fracture_level: i32,
    pub b_hide_unselected: bool,
    pub selection_display_mode: EFractureSelectionDisplayMode,
    pub rest_collection: TWeakObjectPtr<UGeometryCollection>,
}
pub struct UFractureToolSettings {
    pub owner_tool: UPtr<UFractureModalTool>,
}
pub struct UFractureAutoClusterSettings {
    pub auto_cluster_mode_deprecated: EFractureAutoClusterMode,
    pub cluster_size_method: EClusterSizeMethod,
    pub site_count: u32,
    pub site_count_fraction: f32,
    pub site_size: f32,
    pub cluster_grid_width: i32,
    pub cluster_grid_depth: i32,
    pub cluster_grid_height: i32,
    pub b_show_grid_lines: bool,
    pub minimum_size: f32,
    pub drift_iterations: i32,
    pub b_prefer_convexity: bool,
    pub concavity_tolerance: f64,
    pub b_enforce_connectivity: bool,
    pub b_enforce_site_parameters: bool,
    pub b_avoid_isolated: bool,
}
pub struct UFractureActionTool {}
pub struct UFractureModalTool {
    pub visualized_collections: TArray<UPtr<UGeometryCollectionComponent>>,
}
pub struct UFractureToolAutoCluster {
    pub auto_cluster_settings: UPtr<UFractureAutoClusterSettings>,
}
pub struct UFractureBrickSettings {
    pub bond: EFractureBrickBond,
    pub brick_length: f32,
    pub brick_height: f32,
    pub brick_depth: f32,
}
pub struct UFractureInteractiveTool {}
pub struct UFractureToolCutterBase {
    pub cutter_settings: UPtr<UFractureCutterSettings>,
    pub collision_settings: UPtr<UFractureCollisionSettings>,
}
pub struct UFractureToolBrick {
    pub brick_settings: UPtr<UFractureBrickSettings>,
}
pub struct UFractureClusterCutterSettings {
    pub number_clusters_min: i32,
    pub number_clusters_max: i32,
    pub sites_per_cluster_min: i32,
    pub sites_per_cluster_max: i32,
    pub cluster_radius_fraction_min: f32,
    pub cluster_radius_fraction_max: f32,
    pub cluster_radius_offset: f32,
}
pub struct UFractureToolVoronoiCutterBase {
    pub voronoi_line_sets: TArray<UPtr<ULineSetComponent>>,
    pub voronoi_noise_previews: TArray<UPtr<UDynamicMeshComponent>>,
}
pub struct UFractureToolClusterCutter {
    pub cluster_settings: UPtr<UFractureClusterCutterSettings>,
}
pub struct UFractureToolFlattenAll {}
pub struct UFractureToolCluster {}
pub struct UFractureToolUncluster {}
pub struct UFractureToolMoveUp {}
pub struct UFractureToolClusterMerge {}
pub struct UFractureClusterMagnetSettings {
    pub iterations: u32,
}
pub struct UFractureToolClusterMagnet {
    pub cluster_magnet_settings: UPtr<UFractureClusterMagnetSettings>,
}
pub struct UFractureConvertSettings {
    pub b_prompt_for_base_name: bool,
    pub b_per_bone: bool,
    pub b_center_pivots: bool,
    pub b_place_in_world: bool,
    pub b_select_new_actors: bool,
}
pub struct UFractureReimportSettings {
    pub b_odd_materials_are_internal: bool,
}
pub struct UFractureToolConvert {
    pub convert_settings: UPtr<UFractureConvertSettings>,
    pub reimport_settings: UPtr<UFractureReimportSettings>,
}
pub struct UFractureConvexSettings {
    pub can_exceed_fraction: f64,
    pub simplification_distance_threshold: f64,
    pub remove_overlaps: EConvexOverlapRemoval,
    pub overlap_removal_shrink_percent: f64,
    pub fraction_allow_remove: f64,
    pub b_see_through_lines: bool,
    pub line_thickness: f32,
}
pub struct UFractureConvexActions {}
pub struct UFractureToolConvex {
    pub convex_settings: UPtr<UFractureConvexSettings>,
    pub convex_actions: UPtr<UFractureConvexActions>,
}
pub struct UFractureCustomVoronoiSettings {
    pub voronoi_pattern: EVoronoiPattern,
    pub normal_offset: f32,
    pub variability: f32,
    pub sites_to_add: i32,
    pub grid_x: i32,
    pub grid_y: i32,
    pub grid_z: i32,
    pub skip_fraction: f32,
    pub skip_mode: EDownsamplingMode,
    pub reference_mesh: TLazyObjectPtr<AStaticMeshActor>,
    pub b_start_at_actor: bool,
}
pub struct UFractureToolCustomVoronoi {
    pub custom_voronoi_settings: UPtr<UFractureCustomVoronoiSettings>,
    pub gizmo_settings: UPtr<UFractureTransformGizmoSettings>,
}
pub struct UFractureCutterSettings {
    pub internal_material: FString,
    pub random_seed: i32,
    pub chance_to_fracture: f32,
    pub b_group_fracture: bool,
    pub b_group_fracture_toggle_enabled: bool,
    pub b_split_islands: bool,
    pub grout: f32,
    pub b_grout_setting_enabled: bool,
    pub b_draw_sites: bool,
    pub b_draw_sites_toggle_enabled: bool,
    pub b_draw_diagram: bool,
    pub b_draw_noise_preview: bool,
    pub b_noise_preview_toggle_enabled: bool,
    pub fraction_preview_cells: f32,
    pub noise_preview_scale: f64,
    pub b_noise_preview_has_scale: bool,
    pub amplitude: f32,
    pub frequency: f32,
    pub persistence: f32,
    pub lacunarity: f32,
    pub octave_number: i32,
    pub point_spacing: f32,
    pub b_noise_settings_enabled: bool,
    pub active_material_names_list: TArray<FString>,
}
pub struct UFractureCollisionSettings {
    pub b_add_samples_for_collision: bool,
    pub point_spacing: f32,
}
pub struct UFractureTransformGizmoSettings {
    pub b_use_gizmo: bool,
    pub b_center_on_selection: bool,
    pub b_show_use_gizmo_option: bool,
    pub transform_gizmo: UPtr<UCombinedTransformGizmo>,
    pub transform_proxy: UPtr<UTransformProxy>,
    pub attached_cutter: UPtr<UFractureToolCutterBase>,
    pub used_tools_context: UPtr<UInteractiveToolsContext>,
}
pub struct UFractureToolDeleteBranch {}
pub struct UFractureToolMergeSelected {}
pub struct UFractureToolSplitSelected {}
pub struct UFractureToolHide {}
pub struct UFractureToolUnhide {}
pub struct UFractureValidateSettings {
    pub b_remove_unreferenced_geometry: bool,
    pub b_remove_clusters_of_one: bool,
    pub b_remove_dangling_clusters: bool,
}
pub struct UFractureToolValidate {
    pub validation_settings: UPtr<UFractureValidateSettings>,
}
pub struct UFractureToolAddEmbeddedGeometry {}
pub struct UFractureToolAutoEmbedGeometry {}
pub struct UFractureToolFlushEmbeddedGeometrySettings {
    pub b_extract_as_static_mesh_actors: bool,
}
pub struct UFractureToolFlushEmbeddedGeometry {
    pub flush_embedded_geometry_settings: UPtr<
        UFractureToolFlushEmbeddedGeometrySettings,
    >,
}
pub struct UFractureTinyGeoSettings {
    pub merge_type: EMergeType,
    pub b_on_fracture_level: bool,
    pub b_only_clusters: bool,
    pub b_only_same_parent: bool,
    pub b_fracture_level_is_all: bool,
    pub neighbor_selection: ENeighborSelectionMethod,
    pub b_only_to_connected: bool,
    pub use_bone_selection: EUseBoneSelection,
    pub selection_method: EGeometrySelectionMethod,
    pub min_volume_cube_root: f64,
    pub relative_volume: f64,
}
pub struct UFractureToolFixTinyGeo {
    pub tiny_geo_settings: UPtr<UFractureTinyGeoSettings>,
}
pub struct UFractureToolGenerateAsset {
    pub asset_path: FString,
    pub last_dataflow_asset: UPtr<UDataflow>,
    pub last_physical_material: UPtr<UPhysicalMaterial>,
}
pub struct UGeometryCollectionResetSettings {
    pub b_reset_materials: bool,
}
pub struct UFractureToolResetAsset {
    pub reset_settings: UPtr<UGeometryCollectionResetSettings>,
}
pub struct UFractureMaterialsSettings {
    pub editing_collection: FString,
    pub materials: TArray<UPtr<UMaterialInterface>>,
    pub b_only_selected_components: bool,
    pub assign_material: FString,
    pub to_faces: EMaterialAssignmentTargets,
    pub b_only_selected_bones: bool,
    pub b_have_target_collection: bool,
    pub assign_material_names_list: TArray<FString>,
}
pub struct UFractureToolMaterials {
    pub materials_settings: UPtr<UFractureMaterialsSettings>,
    pub active_selected_component: TWeakObjectPtr<UGeometryCollectionComponent>,
}
pub struct UFractureMeshCutSettings {
    pub cutting_actor: TLazyObjectPtr<AStaticMeshActor>,
    pub cut_distribution: EMeshCutDistribution,
    pub number_to_scatter: i32,
    pub grid_x: i32,
    pub grid_y: i32,
    pub grid_z: i32,
    pub variability: f32,
    pub min_scale_factor: f32,
    pub max_scale_factor: f32,
    pub b_random_orientation: bool,
    pub roll_range: f32,
    pub pitch_range: f32,
    pub yaw_range: f32,
}
pub struct UFractureToolMeshCut {
    pub mesh_cut_settings: UPtr<UFractureMeshCutSettings>,
}
pub struct UFracturePlaneCutSettings {
    pub number_planar_cuts: i32,
    pub b_can_cut_with_multiple_planes: bool,
}
pub struct UFractureToolPlaneCut {
    pub plane_cut_settings: UPtr<UFracturePlaneCutSettings>,
    pub gizmo_settings: UPtr<UFractureTransformGizmoSettings>,
    pub noise_preview: UPtr<UMeshOpPreviewWithBackgroundCompute>,
}
pub struct UFractureInitialDynamicStateSettings {
    pub initial_dynamic_state: EDynamicStateOverrideEnum,
}
pub struct UFractureToolSetInitialDynamicState {
    pub state_settings: UPtr<UFractureInitialDynamicStateSettings>,
}
pub struct UFractureRemoveOnBreakSettings {
    pub enabled: bool,
    pub post_break_timer: FVector2f,
    pub cluster_crumbling: bool,
    pub removal_timer: FVector2f,
}
pub struct UFractureToolSetRemoveOnBreak {
    pub remove_on_break_settings: UPtr<UFractureRemoveOnBreakSettings>,
}
pub struct UFractureProximitySettings {
    pub method: EProximityMethod,
    pub distance_threshold: f64,
    pub contact_method: EProximityContactMethod,
    pub contact_threshold: f64,
    pub b_use_as_connection_graph: bool,
    pub contact_area_method: EConnectionContactMethod,
    pub b_show_proximity: bool,
    pub b_only_show_for_selected: bool,
    pub line_thickness: f32,
    pub line_color: FColor,
    pub center_size: f32,
    pub center_color: FColor,
}
pub struct UFractureProximityActions {}
pub struct UFractureToolProximity {
    pub proximity_settings: UPtr<UFractureProximitySettings>,
    pub proximity_actions: UPtr<UFractureProximityActions>,
}
pub struct UFractureRadialSettings {
    pub center: FVector,
    pub normal: FVector,
    pub b_positioned_by_gizmo: bool,
    pub angular_steps: i32,
    pub angle_offset: f32,
    pub angular_noise: f32,
    pub radius: f32,
    pub radial_steps: i32,
    pub radial_step_exponent: f32,
    pub radial_min_step: f32,
    pub radial_noise: f32,
    pub radial_variability: f32,
    pub angular_variability: f32,
    pub axial_variability: f32,
}
pub struct UFractureToolRadial {
    pub radial_settings: UPtr<UFractureRadialSettings>,
    pub gizmo_settings: UPtr<UFractureTransformGizmoSettings>,
}
pub struct UFractureRecomputeNormalsSettings {
    pub b_show_normals: bool,
    pub b_show_tangents: bool,
    pub length: f32,
    pub b_only_tangents: bool,
    pub b_recompute_sharp_edges: bool,
    pub sharp_edge_angle_threshold: f32,
    pub b_only_internal_surfaces: bool,
}
pub struct UFractureToolRecomputeNormals {
    pub normals_settings: UPtr<UFractureRecomputeNormalsSettings>,
}
pub struct UFractureResampleSettings {
    pub b_only_show_added_points: bool,
}
pub struct UFractureToolResample {
    pub resample_settings: UPtr<UFractureResampleSettings>,
}
pub struct UFractureSelectionSettings {
    pub mouse_selection_method: EMouseSelectionMethod,
    pub volume_selection_method: EVolumeSelectionMethod,
    pub selection_operation: ESelectionOperation,
    pub min_volume: f64,
    pub max_volume: f64,
    pub min_volume_frac: f64,
    pub max_volume_frac: f64,
    pub keep_fraction: f64,
    pub random_seed: i32,
}
pub struct URectangleMarqueeManager {
    pub b_use_external_click_drag_behavior: bool,
    pub on_drag_rectangle_changed_deferred_threshold: f64,
    pub click_drag_behavior: UPtr<UClickDragInputBehavior>,
}
pub struct UFractureToolSelection {
    pub selection_behavior_set: UPtr<UInputBehaviorSet>,
    pub selection_behavior_source: UPtr<ULocalInputBehaviorSource>,
    pub rectangle_marquee_manager: UPtr<URectangleMarqueeManager>,
    pub used_tools_context: UPtr<UInteractiveToolsContext>,
    pub selection_settings: UPtr<UFractureSelectionSettings>,
}
pub struct UFractureToolSelectAll {}
pub struct UFractureToolSelectNone {}
pub struct UFractureToolSelectNeighbors {}
pub struct UFractureToolSelectParent {}
pub struct UFractureToolSelectChildren {}
pub struct UFractureToolSelectSiblings {}
pub struct UFractureToolSelectAllInLevel {}
pub struct UFractureToolSelectInvert {}
pub struct UFractureToolSelectLeaf {}
pub struct UFractureToolSelectCluster {}
pub struct UFractureSliceSettings {
    pub slices_x: i32,
    pub slices_y: i32,
    pub slices_z: i32,
    pub slice_angle_variation: f32,
    pub slice_offset_variation: f32,
}
pub struct UFractureToolSlice {
    pub slice_settings: UPtr<UFractureSliceSettings>,
}
pub struct UFractureUniformSettings {
    pub number_voronoi_sites_min: i32,
    pub number_voronoi_sites_max: i32,
}
pub struct UFractureToolUniform {
    pub uniform_settings: UPtr<UFractureUniformSettings>,
}
pub struct UFractureAutoUVSettings {
    pub uv_channel: FString,
    pub uv_channel_names_list: TArray<FString>,
    pub projection_scale: FVector,
    pub b_auto_fit_to_bounds: bool,
    pub b_uniform_projection_scale: bool,
    pub projection_uv_offset: FVector2D,
    pub b_center_at_pivot: bool,
    pub target_faces: ETargetFaces,
    pub material_i_ds: TArray<i32>,
    pub resolution: EAutoUVTextureResolution,
    pub gutter_size: i32,
    pub result: UPtr<UTexture2D>,
    pub b_prompt_to_save: bool,
    pub b_replace_existing: bool,
    pub bake_texture_type: ETextureType,
    pub b_dist_to_outer: bool,
    pub b_ambient_occlusion: bool,
    pub b_smoothed_curvature: bool,
    pub max_distance: f64,
    pub occlusion_rays: i32,
    pub occlusion_blur_radius: f64,
    pub curvature_blur_radius: f64,
    pub voxel_resolution: i32,
    pub smoothing_iterations: i32,
    pub thickness_factor: f64,
    pub max_curvature: f64,
}
pub struct UFractureToolAutoUV {
    pub auto_uv_settings: UPtr<UFractureAutoUVSettings>,
}
pub struct UHistogramSettings {
    pub inspected_attribute: EInspectedAttributeEnum,
    pub b_sorted: bool,
    pub b_show_clusters: bool,
    pub b_show_rigids: bool,
    pub b_show_embedded: bool,
}
pub struct UOutlinerSettings {
    pub color_by_level: bool,
    pub column_mode: EOutlinerColumnMode,
}
