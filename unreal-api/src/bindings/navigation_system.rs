#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FNavCollisionCylinder {
    pub offset: FVector,
    pub radius: f32,
    pub height: f32,
}
#[repr(C, align(8))]
pub struct FNavCollisionBox {
    pub offset: FVector,
    pub extent: FVector,
}
#[repr(C, align(8))]
pub struct FNavigationFilterArea {
    pub area_class: TSubclassOf<UNavArea>,
    pub travel_cost_override: f32,
    pub entering_cost_override: f32,
    pub flags_16: u8,
}
#[repr(C, align(4))]
pub struct FNavigationFilterFlags {
    pub flags_0: u8,
    pub flags_1: u8,
}
#[repr(C, align(8))]
pub struct FNavGraphEdge {}
#[repr(C, align(8))]
pub struct FNavGraphNode {
    pub owner: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FSupportedAreaData {
    pub area_class_name: FString,
    pub area_id: i32,
    pub area_class: TSubclassOf<UObject>,
}
#[repr(C, align(8))]
pub struct FNavLinkCustomInstanceData {
    pub custom_link_id: FNavLinkId,
    pub auxiliary_custom_link_id: FNavLinkAuxiliaryId,
}
#[repr(C, align(8))]
pub struct FNavLinkGenerationJumpConfig {
    pub b_enabled: bool,
    pub name: FName,
    pub jump_length: f32,
    pub jump_distance_from_edge: f32,
    pub jump_max_depth: f32,
    pub jump_height: f32,
    pub jump_ends_height_tolerance: f32,
    pub sampling_separation_factor: f32,
    pub filter_distance_threshold: f32,
    pub link_builder_flags: u16,
    pub area_class_deprecated: TSubclassOf<UNavAreaBase>,
    pub down_direction_area_class: TSubclassOf<UNavAreaBase>,
    pub up_direction_area_class: TSubclassOf<UNavAreaBase>,
    pub link_proxy_class: TSubclassOf<UBaseGeneratedNavLinksProxy>,
    pub link_proxy_id: FNavLinkId,
    pub link_proxy: UPtr<UBaseGeneratedNavLinksProxy>,
    pub b_link_proxy_registered: bool,
}
#[repr(C, align(8))]
pub struct FNavLinkGenerationJumpDownConfig {
    pub b_enabled: bool,
    pub jump_length: f32,
    pub jump_distance_from_edge: f32,
    pub jump_max_depth: f32,
    pub jump_height: f32,
    pub jump_ends_height_tolerance: f32,
    pub sampling_separation_factor: f32,
    pub filter_distance_threshold: f32,
    pub link_builder_flags: u16,
    pub area_class_deprecated: TSubclassOf<UNavAreaBase>,
    pub down_direction_area_class: TSubclassOf<UNavAreaBase>,
    pub up_direction_area_class: TSubclassOf<UNavAreaBase>,
    pub link_proxy_class: TSubclassOf<UBaseGeneratedNavLinksProxy>,
    pub link_proxy_id: FNavLinkId,
    pub link_proxy: UPtr<UBaseGeneratedNavLinksProxy>,
    pub b_link_proxy_registered: bool,
}
#[repr(C, align(4))]
pub struct FRecastNavMeshTileGenerationDebug {
    pub flags_0: u8,
    pub tile_coordinate: FIntVector,
    pub max_tile_coordinate: FIntVector,
    pub flags_28: u8,
    pub height_field_render_mode: EHeightFieldRenderMode,
    pub flags_36: u8,
    pub flags_37: u8,
    pub link_generation_debug_flags: u16,
    pub link_generation_selected_edge: i32,
    pub link_generation_selected_config: i32,
}
#[repr(C, align(4))]
pub struct FNavMeshResolutionParam {
    pub cell_size: f32,
    pub cell_height: f32,
    pub agent_max_step_height: f32,
}
pub struct UCrowdManagerBase {}
pub struct UBaseGeneratedNavLinksProxy {
    pub link_proxy_id: FNavLinkId,
    pub owner: UPtr<UObject>,
}
pub struct UNavigationQueryFilter {
    pub areas: TArray<FNavigationFilterArea>,
    pub include_flags: FNavigationFilterFlags,
    pub exclude_flags: FNavigationFilterFlags,
}
pub struct ANavigationGraphNode {}
pub struct UNavigationGraphNodeComponent {
    pub node: FNavGraphNode,
    pub next_node_component: UPtr<UNavigationGraphNodeComponent>,
    pub prev_node_component: UPtr<UNavigationGraphNodeComponent>,
}
pub struct UNavigationPathGenerator {}
pub struct INavigationPathGenerator {}
pub struct UNavLinkCustomInterface {}
pub struct INavLinkCustomInterface {}
pub struct UNavLinkHostInterface {}
pub struct INavLinkHostInterface {}
pub struct UNavLinkTrivial {}
pub struct UNavNodeInterface {}
pub struct INavNodeInterface {}
pub struct ANavigationData {
    pub rendering_comp: UPtr<UPrimitiveComponent>,
    pub nav_data_config: FNavDataConfig,
    pub flags_1304: u8,
    pub runtime_generation: ERuntimeGenerationType,
    pub observed_paths_tick_interval: f32,
    pub data_version: u32,
    pub supported_areas: TArray<FSupportedAreaData>,
}
pub struct AAbstractNavData {}
pub struct UNavArea {
    pub default_cost: f32,
    pub fixed_area_entering_cost: f32,
    pub draw_color: FColor,
    pub supported_agents: FNavAgentSelector,
    pub flags_72: u8,
    pub flags_73: u8,
}
pub struct UNavAreaMeta {}
pub struct UNavAreaMeta_SwitchByAgent {
    pub agent0_area: TSubclassOf<UNavArea>,
    pub agent1_area: TSubclassOf<UNavArea>,
    pub agent2_area: TSubclassOf<UNavArea>,
    pub agent3_area: TSubclassOf<UNavArea>,
    pub agent4_area: TSubclassOf<UNavArea>,
    pub agent5_area: TSubclassOf<UNavArea>,
    pub agent6_area: TSubclassOf<UNavArea>,
    pub agent7_area: TSubclassOf<UNavArea>,
    pub agent8_area: TSubclassOf<UNavArea>,
    pub agent9_area: TSubclassOf<UNavArea>,
    pub agent10_area: TSubclassOf<UNavArea>,
    pub agent11_area: TSubclassOf<UNavArea>,
    pub agent12_area: TSubclassOf<UNavArea>,
    pub agent13_area: TSubclassOf<UNavArea>,
    pub agent14_area: TSubclassOf<UNavArea>,
    pub agent15_area: TSubclassOf<UNavArea>,
}
pub struct UNavArea_Default {}
pub struct UNavArea_LowHeight {}
pub struct UNavArea_Null {}
pub struct UNavArea_Obstacle {}
pub struct UNavCollision {
    pub cylinder_collision: TArray<FNavCollisionCylinder>,
    pub box_collision: TArray<FNavCollisionBox>,
    pub area_class: TSubclassOf<UNavArea>,
    pub flags_272: u8,
}
pub struct URecastFilter_UseDefaultArea {}
pub struct ANavigationGraph {}
pub struct UNavigationInvokerComponent {
    pub tile_generation_radius: f32,
    pub tile_removal_radius: f32,
    pub supported_agents: FNavAgentSelector,
    pub priority: ENavigationInvokerPriority,
}
pub struct UNavigationObjectRepository {}
pub struct UNavigationPath {
    pub path_updated_notifier: FNavigationPath_PathUpdatedNotifier,
    pub path_points: TArray<FVector>,
    pub recalculate_on_invalidation: ENavigationOptionFlag,
}
pub struct UNavigationSystemV1 {
    pub main_nav_data: UPtr<ANavigationData>,
    pub abstract_nav_data: UPtr<ANavigationData>,
    pub default_agent_name: FName,
    pub crowd_manager_class: TSoftObjectPtr<UClass>,
    pub flags_128: u8,
    pub flags_129: u8,
    pub geometry_export_triangle_count_warning_threshold: i32,
    pub flags_136: u8,
    pub active_tiles_update_interval: f32,
    pub invokers_maximum_distance_from_seed: f64,
    pub data_gathering_mode: ENavDataGatheringModeConfig,
    pub dirty_area_warning_size_threshold: f32,
    pub gathering_nav_modifiers_warning_limit_time: f32,
    pub supported_agents: TArray<FNavDataConfig>,
    pub supported_agents_mask: FNavAgentSelector,
    pub build_bounds: FBox,
    pub nav_data_set: TArray<UPtr<ANavigationData>>,
    pub nav_data_registration_queue: TArray<UPtr<ANavigationData>>,
    pub on_nav_data_registered_event: FNavigationSystemV1_OnNavDataRegisteredEvent,
    pub on_navigation_generation_finished_delegate: FNavigationSystemV1_OnNavigationGenerationFinishedDelegate,
    pub operation_mode: FNavigationSystemRunMode,
    pub repository: UPtr<UNavigationObjectRepository>,
}
pub struct UNavigationSystemModuleConfig {
    pub flags_112: u8,
}
pub struct ANavigationTestingActor {
    pub capsule_component: UPtr<UCapsuleComponent>,
    pub ed_render_comp: UPtr<UNavTestRenderingComponent>,
    pub invoker_component: UPtr<UNavigationInvokerComponent>,
    pub flags_1176: u8,
    pub nav_agent_props: FNavAgentProperties,
    pub querying_extent: FVector,
    pub my_nav_data: UPtr<ANavigationData>,
    pub projected_location: FVector,
    pub flags_1304: u8,
    pub cost_limit_factor: f32,
    pub minimum_cost_limit: f32,
    pub flags_1316: u8,
    pub query_target_actor: UPtr<AActor>,
    pub flags_1328: u8,
    pub radius_used_to_validate_nav_data: f32,
    pub cost_display_mode: ENavCostDisplay,
    pub text_canvas_offset: FVector2D,
    pub flags_1360: u8,
    pub pathfinding_time: f32,
    pub path_cost: f64,
    pub pathfinding_steps: i32,
    pub other_actor: UPtr<ANavigationTestingActor>,
    pub filter_class: TSubclassOf<UNavigationQueryFilter>,
    pub show_step_index: i32,
    pub offset_from_corners_distance: f32,
}
pub struct UNavLinkComponent {
    pub links: TArray<FNavigationLink>,
}
pub struct UNavRelevantComponent {
    pub flags_304: u8,
    pub cached_nav_parent: UPtr<UObject>,
}
pub struct UNavLinkCustomComponent {
    pub nav_link_user_id: u32,
    pub custom_link_id: FNavLinkId,
    pub auxiliary_custom_link_id: FNavLinkAuxiliaryId,
    pub enabled_area_class: TSubclassOf<UNavArea>,
    pub disabled_area_class: TSubclassOf<UNavArea>,
    pub supported_agents: FNavAgentSelector,
    pub link_relative_start: FVector,
    pub link_relative_end: FVector,
    pub link_direction: ENavLinkDirection,
    pub flags_428: u8,
    pub obstacle_offset: FVector,
    pub obstacle_extent: FVector,
    pub obstacle_area_class: TSubclassOf<UNavArea>,
    pub broadcast_radius: f32,
    pub broadcast_interval: f32,
    pub broadcast_channel: ECollisionChannel,
}
pub struct UNavLinkRenderingComponent {}
pub struct ANavMeshBoundsVolume {
    pub supported_agents: FNavAgentSelector,
}
pub struct UNavMeshRenderingComponent {}
pub struct UNavTestRenderingComponent {}
pub struct ARecastNavMesh {
    pub flags_1760: u8,
    pub flags_1761: u8,
    pub flags_1762: u8,
    pub draw_offset: f32,
    pub tile_generation_debug: FRecastNavMeshTileGenerationDebug,
    pub flags_1820: u8,
    pub tile_pool_size: i32,
    pub tile_size_uu: f32,
    pub cell_size: f32,
    pub cell_height: f32,
    pub agent_max_step_height: f32,
    pub nav_mesh_resolution_params: FNavMeshResolutionParam,
    pub agent_radius: f32,
    pub agent_height: f32,
    pub agent_max_slope: f32,
    pub min_region_area: f32,
    pub merge_region_size: f32,
    pub max_vertical_merge_error: i32,
    pub max_simplification_error: f32,
    pub simplification_elevation_ratio: f32,
    pub max_simultaneous_tile_generation_jobs_count: i32,
    pub tile_number_hard_limit: i32,
    pub expected_max_layers_per_tile: i32,
    pub poly_ref_tile_bits: i32,
    pub poly_ref_nav_poly_bits: i32,
    pub poly_ref_salt_bits: i32,
    pub nav_mesh_origin_offset: FVector,
    pub default_draw_distance: f32,
    pub default_max_search_nodes: f32,
    pub default_max_hierarchical_search_nodes: f32,
    pub ledge_slope_filter_mode: ENavigationLedgeSlopeFilterMode,
    pub region_partitioning: ERecastPartitioning,
    pub layer_partitioning: ERecastPartitioning,
    pub region_chunk_splits: i32,
    pub layer_chunk_splits: i32,
    pub flags_1984: u8,
    pub flags_1985: u8,
    pub time_slice_filter_ledge_spans_max_y_process: i32,
    pub time_slice_long_duration_debug: f64,
    pub invoker_tile_priority_bump_distance_threshold_in_tile_units: u32,
    pub invoker_tile_priority_bump_increase: u8,
    pub flags_2008: u8,
    pub nav_link_jump_down_config: FNavLinkGenerationJumpDownConfig,
    pub nav_link_jump_configs: TArray<FNavLinkGenerationJumpConfig>,
    pub flags_2132: u8,
    pub tile_set_update_interval: f32,
    pub heuristic_scale: f32,
    pub vertical_deviation_from_ground_compensation: f32,
}
pub struct URecastNavMeshDataChunk {}
pub struct UNavModifierComponent {
    pub area_class: TSubclassOf<UNavArea>,
    pub area_class_to_replace: TSubclassOf<UNavArea>,
    pub failsafe_extent: FVector,
    pub nav_mesh_resolution: ENavigationDataResolution,
    pub flags_361: u8,
}
pub struct ANavModifierVolume {
    pub area_class: TSubclassOf<UNavArea>,
    pub area_class_to_replace: TSubclassOf<UNavArea>,
    pub b_mask_fill_collision_underneath_for_navmesh: bool,
    pub nav_mesh_resolution: ENavigationDataResolution,
}
pub struct ANavSystemConfigOverride {
    pub sprite_component: UPtr<UBillboardComponent>,
    pub navigation_system_config: UPtr<UNavigationSystemConfig>,
    pub override_policy: ENavSystemOverridePolicy,
    pub flags_1153: u8,
}
pub struct USplineNavModifierComponent {
    pub b_update_nav_data_on_spline_change: bool,
    pub attached_spline: FComponentReference,
    pub stroke_width: f64,
    pub stroke_height: f64,
    pub subdivision_lod: ESubdivisionLOD,
}
