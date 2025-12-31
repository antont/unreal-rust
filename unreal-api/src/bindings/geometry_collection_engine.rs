#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FChaosBreakingEventData {
    pub location: crate::bindings::core_u_object::FVector,
    pub velocity: crate::bindings::core_u_object::FVector,
    pub mass: f32,
}
#[repr(C, align(8))]
pub struct FChaosCollisionEventData {
    pub location: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub velocity1: crate::bindings::core_u_object::FVector,
    pub velocity2: crate::bindings::core_u_object::FVector,
    pub mass1: f32,
    pub mass2: f32,
    pub impulse: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FChaosRemovalEventData {
    pub location: crate::bindings::core_u_object::FVector,
    pub mass: f32,
    pub particle_index: i32,
}
#[repr(C, align(8))]
pub struct FChaosTrailingEventData {
    pub location: crate::bindings::core_u_object::FVector,
    pub velocity: crate::bindings::core_u_object::FVector,
    pub angular_velocity: crate::bindings::core_u_object::FVector,
    pub mass: f32,
    pub particle_index: i32,
}
#[repr(C, align(4))]
pub struct FGeometryCollectionDamagePropagationData {
    pub b_enabled: bool,
    pub break_damage_propagation_factor: f32,
    pub shock_damage_propagation_factor: f32,
}
#[repr(C, align(8))]
pub struct FGeometryCollectionRepDynamicData {}
#[repr(C, align(8))]
pub struct FGeometryCollectionRepStateData {}
#[repr(C, align(8))]
pub struct FGeometryCollectionRepData {}
#[repr(C, align(8))]
pub struct FGeomComponentCacheParameters {
    pub cache_mode: crate::bindings::chaos::EGeometryCollectionCacheType,
    pub target_cache: UPtr<UGeometryCollectionCache>,
    pub reverse_cache_begin_time: f32,
    pub save_collision_data: bool,
    pub do_generate_collision_data: bool,
    pub collision_data_size_max: i32,
    pub do_collision_data_spatial_hash: bool,
    pub collision_data_spatial_hash_radius: f32,
    pub max_collision_per_cell: i32,
    pub save_breaking_data: bool,
    pub do_generate_breaking_data: bool,
    pub breaking_data_size_max: i32,
    pub do_breaking_data_spatial_hash: bool,
    pub breaking_data_spatial_hash_radius: f32,
    pub max_breaking_per_cell: i32,
    pub save_trailing_data: bool,
    pub do_generate_trailing_data: bool,
    pub trailing_data_size_max: i32,
    pub trailing_min_speed_threshold: f32,
    pub trailing_min_volume_threshold: f32,
}
#[repr(C, align(4))]
pub struct FChaosBreakingEventRequestSettings {
    pub max_number_of_results: i32,
    pub min_radius: f32,
    pub min_speed: f32,
    pub min_mass: f32,
    pub max_distance: f32,
    pub sort_method: EChaosBreakingSortMethod,
}
#[repr(C, align(4))]
pub struct FChaosCollisionEventRequestSettings {
    pub max_number_results: i32,
    pub min_mass: f32,
    pub min_speed: f32,
    pub min_impulse: f32,
    pub max_distance: f32,
    pub sort_method: EChaosCollisionSortMethod,
}
#[repr(C, align(4))]
pub struct FChaosRemovalEventRequestSettings {
    pub max_number_of_results: i32,
    pub min_mass: f32,
    pub max_distance: f32,
    pub sort_method: EChaosRemovalSortMethod,
}
#[repr(C, align(4))]
pub struct FChaosTrailingEventRequestSettings {
    pub max_number_of_results: i32,
    pub min_mass: f32,
    pub min_speed: f32,
    pub min_angular_speed: f32,
    pub max_distance: f32,
    pub sort_method: EChaosTrailingSortMethod,
}
#[repr(C, align(1))]
pub struct FGeometryCollectionDebugDrawWarningMessage {}
#[repr(C, align(8))]
pub struct FGeometryCollectionDebugDrawActorSelectedRigidBody {
    pub id: i32,
    pub solver: UPtr<crate::bindings::chaos_solver_engine::AChaosSolverActor>,
    pub geometry_collection: UPtr<AGeometryCollectionActor>,
}
#[repr(C, align(16))]
pub struct FGeometryCollectionSource {
    pub source_geometry_object: crate::bindings::core_u_object::FSoftObjectPath,
    pub local_transform: crate::bindings::core_u_object::FTransform,
    pub source_material: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub instance_custom_data: TArray<f32>,
    pub b_add_internal_materials: bool,
    pub b_split_components: bool,
    pub b_set_internal_from_material_index: bool,
}
#[repr(C, align(8))]
pub struct FGeometryCollectionAutoInstanceMesh {
    pub static_mesh_deprecated: crate::bindings::core_u_object::FSoftObjectPath,
    pub mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub num_instances: i32,
    pub custom_data: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FGeometryCollectionEmbeddedExemplar {
    pub static_mesh_exemplar: crate::bindings::core_u_object::FSoftObjectPath,
    pub start_cull_distance: f32,
    pub end_cull_distance: f32,
    pub instance_count: i32,
}
#[repr(C, align(4))]
pub struct FGeometryCollectionLevelSetData {
    pub min_level_set_resolution: i32,
    pub max_level_set_resolution: i32,
    pub min_cluster_level_set_resolution: i32,
    pub max_cluster_level_set_resolution: i32,
}
#[repr(C, align(4))]
pub struct FGeometryCollectionCollisionParticleData {
    pub collision_particles_fraction: f32,
    pub maximum_collision_particles: i32,
}
#[repr(C, align(4))]
pub struct FGeometryCollectionCollisionTypeData {
    pub collision_type: crate::bindings::chaos::ECollisionTypeEnum,
    pub implicit_type: crate::bindings::chaos::EImplicitTypeEnum,
    pub level_set: FGeometryCollectionLevelSetData,
    pub collision_particles: FGeometryCollectionCollisionParticleData,
    pub collision_object_reduction_percentage: f32,
    pub collision_margin_fraction: f32,
}
#[repr(C, align(8))]
pub struct FGeometryCollectionSizeSpecificData {
    pub max_size: f32,
    pub collision_shapes: TArray<FGeometryCollectionCollisionTypeData>,
    pub collision_type_deprecated: crate::bindings::chaos::ECollisionTypeEnum,
    pub implicit_type_deprecated: crate::bindings::chaos::EImplicitTypeEnum,
    pub min_level_set_resolution_deprecated: i32,
    pub max_level_set_resolution_deprecated: i32,
    pub min_cluster_level_set_resolution_deprecated: i32,
    pub max_cluster_level_set_resolution_deprecated: i32,
    pub collision_object_reduction_percentage_deprecated: i32,
    pub collision_particles_fraction_deprecated: f32,
    pub maximum_collision_particles_deprecated: i32,
    pub damage_threshold: i32,
}
#[repr(C, align(8))]
pub struct FGeometryCollectionProxyMeshMaterials {
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
}
#[repr(C, align(8))]
pub struct FGeometryCollectionProxyMeshData {
    pub proxy_meshes: TArray<UPtr<crate::bindings::engine::UStaticMesh>>,
    pub mesh_transforms: TArray<crate::bindings::core_u_object::FTransform3f>,
    pub mesh_override_materials: TArray<FGeometryCollectionProxyMeshMaterials>,
}
#[repr(C, align(8))]
pub struct FGeometryCollectionRenderResourceSizeInfo {
    pub mesh_resources_size: u64,
    pub nanite_resources_size: u64,
}
pub struct UGeometryCollectionExternalRenderInterface {}
pub struct IGeometryCollectionExternalRenderInterface {}
pub struct UGeometryCollectionCustomDataInterface {}
pub struct IGeometryCollectionCustomDataInterface {}
pub struct UChaosDestructionListener {
    pub flags_656: u8,
    pub collision_event_request_settings: FChaosCollisionEventRequestSettings,
    pub breaking_event_request_settings: FChaosBreakingEventRequestSettings,
    pub trailing_event_request_settings: FChaosTrailingEventRequestSettings,
    pub removal_event_request_settings: FChaosRemovalEventRequestSettings,
    pub chaos_solver_actors: TSet<
        UPtr<crate::bindings::chaos_solver_engine::AChaosSolverActor>,
    >,
    pub geometry_collection_actors: TSet<UPtr<AGeometryCollectionActor>>,
    pub on_collision_events: FChaosDestructionListener_OnCollisionEvents,
    pub on_breaking_events: FChaosDestructionListener_OnBreakingEvents,
    pub on_trailing_events: FChaosDestructionListener_OnTrailingEvents,
    pub on_removal_events: FChaosDestructionListener_OnRemovalEvents,
}
pub struct AGeometryCollectionActor {
    pub geometry_collection_component: UPtr<UGeometryCollectionComponent>,
    pub geometry_collection_debug_draw_component_deprecated: UPtr<
        UGeometryCollectionDebugDrawComponent,
    >,
}
pub struct UGeometryCollectionBlueprintLibrary {}
pub struct UGeometryCollectionCache {
    pub recorded_data: crate::bindings::chaos::FRecordedTransformTrack,
    pub supported_collection: UPtr<UGeometryCollection>,
    pub compatible_collection_state: crate::bindings::core_u_object::FGuid,
}
pub struct UGeometryCollectionComponent {
    pub chaos_solver_actor: UPtr<
        crate::bindings::chaos_solver_engine::AChaosSolverActor,
    >,
    pub rest_collection: UPtr<UGeometryCollection>,
    pub initialization_fields: TArray<
        UPtr<crate::bindings::field_system_engine::AFieldSystemActor>,
    >,
    pub simulating_deprecated: bool,
    pub object_type: crate::bindings::chaos::EObjectStateTypeEnum,
    pub gravity_group_index: i32,
    pub one_way_interaction_level: i32,
    pub b_density_from_physics_material: bool,
    pub b_force_motion_blur: bool,
    pub enable_clustering: bool,
    pub cluster_group_index: i32,
    pub max_cluster_level: i32,
    pub max_simulated_level: i32,
    pub damage_model: crate::bindings::chaos::EDamageModelTypeEnum,
    pub damage_threshold: TArray<f32>,
    pub b_use_size_specific_damage_threshold: bool,
    pub b_use_material_damage_modifiers: bool,
    pub damage_propagation_data: FGeometryCollectionDamagePropagationData,
    pub b_enable_damage_from_collision: bool,
    pub b_allow_removal_on_sleep: bool,
    pub b_allow_removal_on_break: bool,
    pub b_force_update_active_transforms: bool,
    pub cluster_connection_type_deprecated: crate::bindings::chaos_solver_engine::EClusterConnectionTypeEnum,
    pub collision_group: i32,
    pub collision_sample_fraction: f32,
    pub linear_ether_drag_deprecated: f32,
    pub physical_material_deprecated: UPtr<
        crate::bindings::physics_core::UChaosPhysicalMaterial,
    >,
    pub initial_velocity_type: crate::bindings::chaos::EInitialVelocityTypeEnum,
    pub initial_linear_velocity: crate::bindings::core_u_object::FVector,
    pub initial_angular_velocity: crate::bindings::core_u_object::FVector,
    pub physical_material_override_deprecated: UPtr<
        crate::bindings::physics_core::UPhysicalMaterial,
    >,
    pub cache_parameters: FGeomComponentCacheParameters,
    pub rest_transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub notify_geometry_collection_physics_state_change: FGeometryCollectionComponent_NotifyGeometryCollectionPhysicsStateChange,
    pub notify_geometry_collection_physics_loading_state_change: FGeometryCollectionComponent_NotifyGeometryCollectionPhysicsLoadingStateChange,
    pub on_chaos_break_event: FGeometryCollectionComponent_OnChaosBreakEvent,
    pub on_chaos_removal_event: FGeometryCollectionComponent_OnChaosRemovalEvent,
    pub on_chaos_crumbling_event: FGeometryCollectionComponent_OnChaosCrumblingEvent,
    pub desired_cache_time: f32,
    pub cache_playback: bool,
    pub on_chaos_physics_collision: FGeometryCollectionComponent_OnChaosPhysicsCollision,
    pub b_notify_breaks: bool,
    pub b_notify_collisions: bool,
    pub b_notify_trailing: bool,
    pub b_notify_removals: bool,
    pub b_notify_crumblings: bool,
    pub b_crumbling_event_includes_children: bool,
    pub b_notify_global_breaks: bool,
    pub b_notify_global_collisions: bool,
    pub b_notify_global_removals: bool,
    pub b_notify_global_crumblings: bool,
    pub b_global_crumbling_event_includes_children: bool,
    pub b_store_velocities: bool,
    pub b_is_currently_navigation_relevant: bool,
    pub b_show_bone_colors: bool,
    pub b_update_component_transform_to_root_bone: bool,
    pub b_use_root_proxy_for_navigation: bool,
    pub b_update_navigation_in_tick: bool,
    pub b_enable_run_time_data_collection: bool,
    pub run_time_data_collection_guid: crate::bindings::core_u_object::FGuid,
    pub b_enable_replication: bool,
    pub b_enable_abandon_after_level: bool,
    pub abandoned_collision_profile_name: FName,
    pub ism_pool_deprecated: UPtr<crate::bindings::ism_pool::AISMPoolActor>,
    pub custom_renderer_type: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub b_override_custom_renderer: bool,
    pub b_auto_assign_ism_pool_deprecated: bool,
    pub b_use_static_mesh_collision_for_traces: bool,
    pub replication_abandon_cluster_level_deprecated: i32,
    pub custom_renderer: TScriptInterface<IGeometryCollectionExternalRenderInterface>,
    pub collision_profile_per_level: TArray<FName>,
    pub replication_abandon_after_level: i32,
    pub replication_max_position_and_velocity_correction_level: i32,
    pub rep_data: FGeometryCollectionRepData,
    pub rep_state_data: FGeometryCollectionRepStateData,
    pub rep_dynamic_data: FGeometryCollectionRepDynamicData,
    pub selected_bones: TArray<i32>,
    pub highlighted_bones: TArray<i32>,
    pub dummy_body_setup: UPtr<crate::bindings::engine::UBodySetup>,
    pub editor_actor: UPtr<crate::bindings::engine::AActor>,
    pub event_dispatcher: UPtr<
        crate::bindings::chaos_solver_engine::UChaosGameplayEventDispatcher,
    >,
    pub embedded_geometry_components: TArray<
        UPtr<crate::bindings::engine::UInstancedStaticMeshComponent>,
    >,
    pub angular_ether_drag_deprecated: f32,
}
pub struct AGeometryCollectionDebugDrawActor {
    pub warning_message_deprecated: FGeometryCollectionDebugDrawWarningMessage,
    pub selected_rigid_body_deprecated: FGeometryCollectionDebugDrawActorSelectedRigidBody,
    pub b_debug_draw_whole_collection: bool,
    pub b_debug_draw_hierarchy: bool,
    pub b_debug_draw_clustering: bool,
    pub hide_geometry: EGeometryCollectionDebugDrawActorHideGeometry,
    pub b_show_rigid_body_id: bool,
    pub b_show_rigid_body_collision: bool,
    pub b_collision_at_origin: bool,
    pub b_show_rigid_body_transform: bool,
    pub b_show_rigid_body_inertia: bool,
    pub b_show_rigid_body_velocity: bool,
    pub b_show_rigid_body_force: bool,
    pub b_show_rigid_body_infos: bool,
    pub b_show_transform_index: bool,
    pub b_show_transform: bool,
    pub b_show_parent: bool,
    pub b_show_level: bool,
    pub b_show_connectivity_edges: bool,
    pub b_show_geometry_index: bool,
    pub b_show_geometry_transform: bool,
    pub b_show_bounding_box: bool,
    pub b_show_faces: bool,
    pub b_show_face_indices: bool,
    pub b_show_face_normals: bool,
    pub b_show_single_face: bool,
    pub single_face_index: i32,
    pub b_show_vertices: bool,
    pub b_show_vertex_indices: bool,
    pub b_show_vertex_normals: bool,
    pub b_use_active_visualization: bool,
    pub point_thickness: f32,
    pub line_thickness: f32,
    pub b_text_shadow: bool,
    pub text_scale: f32,
    pub normal_scale: f32,
    pub axis_scale: f32,
    pub arrow_scale: f32,
    pub rigid_body_id_color: crate::bindings::core_u_object::FColor,
    pub rigid_body_transform_scale: f32,
    pub rigid_body_collision_color: crate::bindings::core_u_object::FColor,
    pub rigid_body_inertia_color: crate::bindings::core_u_object::FColor,
    pub rigid_body_velocity_color: crate::bindings::core_u_object::FColor,
    pub rigid_body_force_color: crate::bindings::core_u_object::FColor,
    pub rigid_body_info_color: crate::bindings::core_u_object::FColor,
    pub transform_index_color: crate::bindings::core_u_object::FColor,
    pub transform_scale: f32,
    pub level_color: crate::bindings::core_u_object::FColor,
    pub parent_color: crate::bindings::core_u_object::FColor,
    pub connectivity_edge_thickness: f32,
    pub geometry_index_color: crate::bindings::core_u_object::FColor,
    pub geometry_transform_scale: f32,
    pub bounding_box_color: crate::bindings::core_u_object::FColor,
    pub face_color: crate::bindings::core_u_object::FColor,
    pub face_index_color: crate::bindings::core_u_object::FColor,
    pub face_normal_color: crate::bindings::core_u_object::FColor,
    pub single_face_color: crate::bindings::core_u_object::FColor,
    pub vertex_color: crate::bindings::core_u_object::FColor,
    pub vertex_index_color: crate::bindings::core_u_object::FColor,
    pub vertex_normal_color: crate::bindings::core_u_object::FColor,
    pub sprite_component: UPtr<crate::bindings::engine::UBillboardComponent>,
}
pub struct UGeometryCollectionDebugDrawComponent {
    pub geometry_collection_debug_draw_actor_deprecated: UPtr<
        AGeometryCollectionDebugDrawActor,
    >,
    pub geometry_collection_render_level_set_actor: UPtr<
        AGeometryCollectionRenderLevelSetActor,
    >,
}
pub struct AGeometryCollectionISMPoolActor {
    pub ism_pool_comp: UPtr<UGeometryCollectionISMPoolComponent>,
    pub ism_pool_debug_draw_comp: UPtr<UGeometryCollectionISMPoolDebugDrawComponent>,
}
pub struct UGeometryCollectionISMPoolComponent {}
pub struct UGeometryCollectionISMPoolRenderer {
    pub cached_ism_pool_component: UPtr<crate::bindings::ism_pool::UISMPoolComponent>,
    pub local_ism_pool_component: UPtr<crate::bindings::ism_pool::UISMPoolComponent>,
}
pub struct UGeometryCollectionISMPoolSubSystem {}
pub struct UGeometryCollection {
    pub enable_clustering: bool,
    pub cluster_group_index: i32,
    pub max_cluster_level: i32,
    pub damage_model: crate::bindings::chaos::EDamageModelTypeEnum,
    pub damage_threshold: TArray<f32>,
    pub b_use_size_specific_damage_threshold: bool,
    pub b_use_material_damage_modifiers: bool,
    pub per_cluster_only_damage_threshold: bool,
    pub damage_propagation_data: FGeometryCollectionDamagePropagationData,
    pub cluster_connection_type: crate::bindings::chaos_solver_engine::EClusterConnectionTypeEnum,
    pub connection_graph_bounds_filtering_margin: f32,
    pub geometry_source: TArray<FGeometryCollectionSource>,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub embedded_geometry_exemplar: TArray<FGeometryCollectionEmbeddedExemplar>,
    pub b_use_full_precision_u_vs: bool,
    pub b_strip_on_cook: bool,
    pub b_strip_render_data_on_cook: bool,
    pub custom_renderer_type: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub root_proxy_data: FGeometryCollectionProxyMeshData,
    pub auto_instance_meshes: TArray<FGeometryCollectionAutoInstanceMesh>,
    pub enable_nanite: bool,
    pub b_enable_nanite_fallback: bool,
    pub nanite_minimum_residency_in_kb: u32,
    pub b_convert_vertex_colors_to_srgb: bool,
    pub collision_type_deprecated: crate::bindings::chaos::ECollisionTypeEnum,
    pub implicit_type_deprecated: crate::bindings::chaos::EImplicitTypeEnum,
    pub min_level_set_resolution_deprecated: i32,
    pub max_level_set_resolution_deprecated: i32,
    pub min_cluster_level_set_resolution_deprecated: i32,
    pub max_cluster_level_set_resolution_deprecated: i32,
    pub collision_object_reduction_percentage_deprecated: f32,
    pub root_proxy_deprecated: crate::bindings::core_u_object::FSoftObjectPath,
    pub physics_material: UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    pub b_density_from_physics_material: bool,
    pub cached_density_from_physics_material_in_g_cm3: f32,
    pub b_mass_as_density: bool,
    pub mass: f32,
    pub minimum_mass_clamp: f32,
    pub b_import_collision_from_source: bool,
    pub b_optimize_convexes: bool,
    pub collision_particles_fraction_deprecated: f32,
    pub maximum_collision_particles_deprecated: i32,
    pub b_scale_on_removal: bool,
    pub b_remove_on_max_sleep: bool,
    pub b_automatic_crumble_partial_clusters: bool,
    pub maximum_sleep_time: crate::bindings::core_u_object::FVector2D,
    pub removal_duration: crate::bindings::core_u_object::FVector2D,
    pub b_slow_moving_as_sleeping: bool,
    pub slow_moving_velocity_threshold: f32,
    pub size_specific_data: TArray<FGeometryCollectionSizeSpecificData>,
    pub enable_remove_pieces_on_fracture_deprecated: bool,
    pub remove_on_fracture_materials_deprecated: TArray<
        UPtr<crate::bindings::engine::UMaterialInterface>,
    >,
    pub asset_import_data: UPtr<crate::bindings::engine::UAssetImportData>,
    pub thumbnail_info: UPtr<crate::bindings::engine::UThumbnailInfo>,
    pub dataflow_asset: UPtr<crate::bindings::dataflow_engine::UDataflow>,
    pub dataflow_terminal_deprecated: FString,
    pub overrides: TMap<FString, FString>,
    pub dataflow_instance: crate::bindings::dataflow_engine::FDataflowInstance,
    pub persistent_guid: crate::bindings::core_u_object::FGuid,
    pub state_guid: crate::bindings::core_u_object::FGuid,
    pub root_index: i32,
    pub breadth_first_transform_indices: TArray<i32>,
    pub auto_instance_transform_remap_indices: TArray<i32>,
    pub bone_selected_material_index: i32,
    pub asset_user_data: TArray<UPtr<crate::bindings::engine::UAssetUserData>>,
}
pub struct AGeometryCollectionRenderLevelSetActor {
    pub target_volume_texture: UPtr<crate::bindings::engine::UVolumeTexture>,
    pub ray_march_material: UPtr<crate::bindings::engine::UMaterial>,
    pub surface_tolerance: f32,
    pub isovalue: f32,
    pub enabled: bool,
    pub render_volume_bounding_box: bool,
}
pub struct UGeometryCollectionRootProxyRenderer {
    pub static_mesh_components: TArray<
        UPtr<crate::bindings::engine::UStaticMeshComponent>,
    >,
}
pub struct UGeometryCollectionISMPoolDebugDrawComponent {
    pub b_show_global_stats: bool,
    pub b_show_stats: bool,
    pub b_show_bounds: bool,
    pub selected_component: UPtr<crate::bindings::engine::UInstancedStaticMeshComponent>,
}
pub struct FChaosDestructionListener_OnCollisionEvents;
pub struct FChaosDestructionListener_OnBreakingEvents;
pub struct FChaosDestructionListener_OnTrailingEvents;
pub struct FChaosDestructionListener_OnRemovalEvents;
pub struct FGeometryCollectionComponent_NotifyGeometryCollectionPhysicsStateChange;
pub struct FGeometryCollectionComponent_NotifyGeometryCollectionPhysicsLoadingStateChange;
pub struct FGeometryCollectionComponent_OnChaosBreakEvent;
pub struct FGeometryCollectionComponent_OnChaosRemovalEvent;
pub struct FGeometryCollectionComponent_OnChaosCrumblingEvent;
pub struct FGeometryCollectionComponent_OnChaosPhysicsCollision;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EChaosBreakingSortMethod(pub u8);
impl EChaosBreakingSortMethod {
    pub const SORT_NONE: EChaosBreakingSortMethod = EChaosBreakingSortMethod(0);
    pub const SORT_BY_HIGHEST_MASS: EChaosBreakingSortMethod = EChaosBreakingSortMethod(
        1,
    );
    pub const SORT_BY_HIGHEST_SPEED: EChaosBreakingSortMethod = EChaosBreakingSortMethod(
        2,
    );
    pub const SORT_BY_NEAREST_FIRST: EChaosBreakingSortMethod = EChaosBreakingSortMethod(
        3,
    );
    pub const COUNT: EChaosBreakingSortMethod = EChaosBreakingSortMethod(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EChaosCollisionSortMethod(pub u8);
impl EChaosCollisionSortMethod {
    pub const SORT_NONE: EChaosCollisionSortMethod = EChaosCollisionSortMethod(0);
    pub const SORT_BY_HIGHEST_MASS: EChaosCollisionSortMethod = EChaosCollisionSortMethod(
        1,
    );
    pub const SORT_BY_HIGHEST_SPEED: EChaosCollisionSortMethod = EChaosCollisionSortMethod(
        2,
    );
    pub const SORT_BY_HIGHEST_IMPULSE: EChaosCollisionSortMethod = EChaosCollisionSortMethod(
        3,
    );
    pub const SORT_BY_NEAREST_FIRST: EChaosCollisionSortMethod = EChaosCollisionSortMethod(
        4,
    );
    pub const COUNT: EChaosCollisionSortMethod = EChaosCollisionSortMethod(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EChaosRemovalSortMethod(pub u8);
impl EChaosRemovalSortMethod {
    pub const SORT_NONE: EChaosRemovalSortMethod = EChaosRemovalSortMethod(0);
    pub const SORT_BY_HIGHEST_MASS: EChaosRemovalSortMethod = EChaosRemovalSortMethod(1);
    pub const SORT_BY_NEAREST_FIRST: EChaosRemovalSortMethod = EChaosRemovalSortMethod(
        2,
    );
    pub const COUNT: EChaosRemovalSortMethod = EChaosRemovalSortMethod(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EChaosTrailingSortMethod(pub u8);
impl EChaosTrailingSortMethod {
    pub const SORT_NONE: EChaosTrailingSortMethod = EChaosTrailingSortMethod(0);
    pub const SORT_BY_HIGHEST_MASS: EChaosTrailingSortMethod = EChaosTrailingSortMethod(
        1,
    );
    pub const SORT_BY_HIGHEST_SPEED: EChaosTrailingSortMethod = EChaosTrailingSortMethod(
        2,
    );
    pub const SORT_BY_NEAREST_FIRST: EChaosTrailingSortMethod = EChaosTrailingSortMethod(
        3,
    );
    pub const COUNT: EChaosTrailingSortMethod = EChaosTrailingSortMethod(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGeometryCollectionDebugDrawActorHideGeometry(pub u8);
impl EGeometryCollectionDebugDrawActorHideGeometry {
    pub const HIDE_NONE: EGeometryCollectionDebugDrawActorHideGeometry = EGeometryCollectionDebugDrawActorHideGeometry(
        0,
    );
    pub const HIDE_WITH_COLLISION: EGeometryCollectionDebugDrawActorHideGeometry = EGeometryCollectionDebugDrawActorHideGeometry(
        1,
    );
    pub const HIDE_SELECTED: EGeometryCollectionDebugDrawActorHideGeometry = EGeometryCollectionDebugDrawActorHideGeometry(
        2,
    );
    pub const HIDE_WHOLE_COLLECTION: EGeometryCollectionDebugDrawActorHideGeometry = EGeometryCollectionDebugDrawActorHideGeometry(
        3,
    );
    pub const HIDE_ALL: EGeometryCollectionDebugDrawActorHideGeometry = EGeometryCollectionDebugDrawActorHideGeometry(
        4,
    );
}
