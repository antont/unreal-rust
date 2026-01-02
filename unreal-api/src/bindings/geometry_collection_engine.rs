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
    __padding_end: [u8; 4],
}
impl FChaosBreakingEventData {}
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
impl FChaosCollisionEventData {}
#[repr(C, align(8))]
pub struct FChaosRemovalEventData {
    pub location: crate::bindings::core_u_object::FVector,
    pub mass: f32,
    pub particle_index: i32,
}
impl FChaosRemovalEventData {}
#[repr(C, align(8))]
pub struct FChaosTrailingEventData {
    pub location: crate::bindings::core_u_object::FVector,
    pub velocity: crate::bindings::core_u_object::FVector,
    pub angular_velocity: crate::bindings::core_u_object::FVector,
    pub mass: f32,
    pub particle_index: i32,
}
impl FChaosTrailingEventData {}
#[repr(C, align(4))]
pub struct FGeometryCollectionDamagePropagationData {
    pub b_enabled: bool,
    pub break_damage_propagation_factor: f32,
    pub shock_damage_propagation_factor: f32,
}
impl FGeometryCollectionDamagePropagationData {}
#[repr(C, align(4))]
pub struct FChaosBreakingEventRequestSettings {
    pub max_number_of_results: i32,
    pub min_radius: f32,
    pub min_speed: f32,
    pub min_mass: f32,
    pub max_distance: f32,
    pub sort_method: EChaosBreakingSortMethod,
    __padding_end: [u8; 3],
}
impl FChaosBreakingEventRequestSettings {}
#[repr(C, align(4))]
pub struct FChaosCollisionEventRequestSettings {
    pub max_number_results: i32,
    pub min_mass: f32,
    pub min_speed: f32,
    pub min_impulse: f32,
    pub max_distance: f32,
    pub sort_method: EChaosCollisionSortMethod,
    __padding_end: [u8; 3],
}
impl FChaosCollisionEventRequestSettings {}
#[repr(C, align(4))]
pub struct FChaosRemovalEventRequestSettings {
    pub max_number_of_results: i32,
    pub min_mass: f32,
    pub max_distance: f32,
    pub sort_method: EChaosRemovalSortMethod,
    __padding_end: [u8; 3],
}
impl FChaosRemovalEventRequestSettings {}
#[repr(C, align(4))]
pub struct FChaosTrailingEventRequestSettings {
    pub max_number_of_results: i32,
    pub min_mass: f32,
    pub min_speed: f32,
    pub min_angular_speed: f32,
    pub max_distance: f32,
    pub sort_method: EChaosTrailingSortMethod,
    __padding_end: [u8; 3],
}
impl FChaosTrailingEventRequestSettings {}
#[repr(C, align(16))]
pub struct FGeometryCollectionSource {
    pub source_geometry_object: crate::bindings::core_u_object::FSoftObjectPath,
    pub local_transform: crate::bindings::core_u_object::FTransform,
    pub source_material: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub instance_custom_data: TArray<f32>,
    pub b_add_internal_materials: bool,
    pub b_split_components: bool,
    pub b_set_internal_from_material_index: bool,
    __padding_end: [u8; 13],
}
impl FGeometryCollectionSource {}
#[repr(C, align(8))]
pub struct FGeometryCollectionAutoInstanceMesh {
    #[doc(hidden)]
    __padding_40: [u8; 40],
    pub mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    __padding_end: [u8; 24],
}
impl FGeometryCollectionAutoInstanceMesh {}
#[repr(C, align(8))]
pub struct FGeometryCollectionEmbeddedExemplar {
    __padding_end: [u8; 56],
}
impl FGeometryCollectionEmbeddedExemplar {}
#[repr(C, align(8))]
pub struct FGeometryCollectionProxyMeshMaterials {
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
}
impl FGeometryCollectionProxyMeshMaterials {}
#[repr(C, align(8))]
pub struct FGeometryCollectionProxyMeshData {
    pub proxy_meshes: TArray<UPtr<crate::bindings::engine::UStaticMesh>>,
    pub mesh_transforms: TArray<crate::bindings::core_u_object::FTransform3f>,
    pub mesh_override_materials: TArray<FGeometryCollectionProxyMeshMaterials>,
}
impl FGeometryCollectionProxyMeshData {}
pub struct UGeometryCollectionExternalRenderInterface {}
pub struct IGeometryCollectionExternalRenderInterface {}
pub struct UGeometryCollectionCustomDataInterface {}
pub struct IGeometryCollectionCustomDataInterface {}
#[repr(C, align(16))]
pub struct UChaosDestructionListener {
    #[doc(hidden)]
    __padding_656: [u8; 656],
    pub flags_656: u8,
    pub collision_event_request_settings: FChaosCollisionEventRequestSettings,
    pub breaking_event_request_settings: FChaosBreakingEventRequestSettings,
    pub trailing_event_request_settings: FChaosTrailingEventRequestSettings,
    pub removal_event_request_settings: FChaosRemovalEventRequestSettings,
    pub chaos_solver_actors: TSet<
        UPtr<crate::bindings::chaos_solver_engine::AChaosSolverActor>,
    >,
    pub geometry_collection_actors: TSet<UPtr<AGeometryCollectionActor>>,
    __padding_end: [u8; 464],
}
impl UChaosDestructionListener {}
#[repr(C, align(8))]
pub struct AGeometryCollectionActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub geometry_collection_component: UPtr<UGeometryCollectionComponent>,
    __padding_end: [u8; 8],
}
impl AGeometryCollectionActor {}
#[repr(C, align(8))]
pub struct UGeometryCollectionBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UGeometryCollectionBlueprintLibrary {}
#[repr(C, align(8))]
pub struct UGeometryCollectionCache {
    __padding_end: [u8; 88],
}
impl UGeometryCollectionCache {}
#[repr(C, align(16))]
pub struct UGeometryCollectionComponent {
    #[doc(hidden)]
    __padding_1616: [u8; 1616],
    pub chaos_solver_actor: UPtr<
        crate::bindings::chaos_solver_engine::AChaosSolverActor,
    >,
    pub rest_collection: UPtr<UGeometryCollection>,
    pub initialization_fields: TArray<
        UPtr<crate::bindings::field_system_engine::AFieldSystemActor>,
    >,
    #[doc(hidden)]
    __padding_1650: [u8; 2],
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
    #[doc(hidden)]
    __padding_1720: [u8; 4],
    pub collision_group: i32,
    pub collision_sample_fraction: f32,
    #[doc(hidden)]
    __padding_1744: [u8; 16],
    pub initial_velocity_type: crate::bindings::chaos::EInitialVelocityTypeEnum,
    pub initial_linear_velocity: crate::bindings::core_u_object::FVector,
    pub initial_angular_velocity: crate::bindings::core_u_object::FVector,
    #[doc(hidden)]
    __padding_2152: [u8; 352],
    pub desired_cache_time: f32,
    pub cache_playback: bool,
    #[doc(hidden)]
    __padding_2184: [u8; 27],
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
    #[doc(hidden)]
    __padding_2197: [u8; 1],
    pub b_show_bone_colors: bool,
    pub b_update_component_transform_to_root_bone: bool,
    pub b_use_root_proxy_for_navigation: bool,
    pub b_update_navigation_in_tick: bool,
    pub run_time_data_collection_guid: crate::bindings::core_u_object::FGuid,
    #[doc(hidden)]
    __padding_2221: [u8; 1],
    pub b_enable_replication: bool,
    pub b_enable_abandon_after_level: bool,
    pub abandoned_collision_profile_name: FName,
    #[doc(hidden)]
    __padding_2248: [u8; 8],
    pub custom_renderer_type: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub b_override_custom_renderer: bool,
    #[doc(hidden)]
    __padding_2259: [u8; 2],
    pub b_use_static_mesh_collision_for_traces: bool,
    #[doc(hidden)]
    __padding_2280: [u8; 16],
    pub collision_profile_per_level: TArray<FName>,
    #[doc(hidden)]
    __padding_2312: [u8; 16],
    pub replication_abandon_after_level: i32,
    pub replication_max_position_and_velocity_correction_level: i32,
    __padding_end: [u8; 672],
}
impl UGeometryCollectionComponent {}
#[repr(C, align(8))]
pub struct AGeometryCollectionDebugDrawActor {
    __padding_end: [u8; 1328],
}
impl AGeometryCollectionDebugDrawActor {}
#[repr(C, align(8))]
pub struct UGeometryCollectionDebugDrawComponent {
    __padding_end: [u8; 264],
}
impl UGeometryCollectionDebugDrawComponent {}
#[repr(C, align(8))]
pub struct AGeometryCollectionISMPoolActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub ism_pool_comp: UPtr<UGeometryCollectionISMPoolComponent>,
    pub ism_pool_debug_draw_comp: UPtr<UGeometryCollectionISMPoolDebugDrawComponent>,
}
impl AGeometryCollectionISMPoolActor {}
#[repr(C, align(16))]
pub struct UGeometryCollectionISMPoolComponent {
    __padding_end: [u8; 960],
}
impl UGeometryCollectionISMPoolComponent {}
#[repr(C, align(16))]
pub struct UGeometryCollectionISMPoolRenderer {
    __padding_end: [u8; 240],
}
impl UGeometryCollectionISMPoolRenderer {}
#[repr(C, align(8))]
pub struct UGeometryCollectionISMPoolSubSystem {
    __padding_end: [u8; 144],
}
impl UGeometryCollectionISMPoolSubSystem {}
#[repr(C, align(8))]
pub struct UGeometryCollection {
    #[doc(hidden)]
    __padding_132: [u8; 132],
    pub damage_model: crate::bindings::chaos::EDamageModelTypeEnum,
    #[doc(hidden)]
    __padding_153: [u8; 20],
    pub b_use_material_damage_modifiers: bool,
    #[doc(hidden)]
    __padding_176: [u8; 16],
    pub geometry_source: TArray<FGeometryCollectionSource>,
    #[doc(hidden)]
    __padding_225: [u8; 33],
    pub b_strip_on_cook: bool,
    pub b_strip_render_data_on_cook: bool,
    pub custom_renderer_type: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub root_proxy_data: FGeometryCollectionProxyMeshData,
    #[doc(hidden)]
    __padding_304: [u8; 16],
    pub enable_nanite: bool,
    #[doc(hidden)]
    __padding_312: [u8; 7],
    pub b_convert_vertex_colors_to_srgb: bool,
    #[doc(hidden)]
    __padding_376: [u8; 56],
    pub physics_material: UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    pub b_density_from_physics_material: bool,
    #[doc(hidden)]
    __padding_392: [u8; 7],
    pub b_mass_as_density: bool,
    pub mass: f32,
    pub minimum_mass_clamp: f32,
    pub b_import_collision_from_source: bool,
    pub b_optimize_convexes: bool,
    #[doc(hidden)]
    __padding_416: [u8; 10],
    pub b_scale_on_removal: bool,
    pub b_remove_on_max_sleep: bool,
    pub b_automatic_crumble_partial_clusters: bool,
    pub maximum_sleep_time: crate::bindings::core_u_object::FVector2D,
    pub removal_duration: crate::bindings::core_u_object::FVector2D,
    pub b_slow_moving_as_sleeping: bool,
    pub slow_moving_velocity_threshold: f32,
    #[doc(hidden)]
    __padding_504: [u8; 40],
    pub asset_import_data: UPtr<crate::bindings::engine::UAssetImportData>,
    #[doc(hidden)]
    __padding_520: [u8; 8],
    pub dataflow_asset: UPtr<crate::bindings::dataflow_engine::UDataflow>,
    #[doc(hidden)]
    __padding_544: [u8; 16],
    pub overrides: TMap<FString, FString>,
    __padding_end: [u8; 264],
}
impl UGeometryCollection {}
#[repr(C, align(16))]
pub struct AGeometryCollectionRenderLevelSetActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub target_volume_texture: UPtr<crate::bindings::engine::UVolumeTexture>,
    pub ray_march_material: UPtr<crate::bindings::engine::UMaterial>,
    pub surface_tolerance: f32,
    pub isovalue: f32,
    pub enabled: bool,
    pub render_volume_bounding_box: bool,
    __padding_end: [u8; 214],
}
impl AGeometryCollectionRenderLevelSetActor {}
#[repr(C, align(8))]
pub struct UGeometryCollectionRootProxyRenderer {
    __padding_end: [u8; 80],
}
impl UGeometryCollectionRootProxyRenderer {}
#[repr(C, align(16))]
pub struct UGeometryCollectionISMPoolDebugDrawComponent {
    __padding_end: [u8; 1744],
}
impl UGeometryCollectionISMPoolDebugDrawComponent {}
#[repr(transparent)]
pub struct FChaosDestructionListener_OnCollisionEvents {
    _opague: u8,
}
#[repr(transparent)]
pub struct FChaosDestructionListener_OnBreakingEvents {
    _opague: u8,
}
#[repr(transparent)]
pub struct FChaosDestructionListener_OnTrailingEvents {
    _opague: u8,
}
#[repr(transparent)]
pub struct FChaosDestructionListener_OnRemovalEvents {
    _opague: u8,
}
#[repr(transparent)]
pub struct FGeometryCollectionComponent_NotifyGeometryCollectionPhysicsStateChange {
    _opague: u8,
}
#[repr(transparent)]
pub struct FGeometryCollectionComponent_NotifyGeometryCollectionPhysicsLoadingStateChange {
    _opague: u8,
}
#[repr(transparent)]
pub struct FGeometryCollectionComponent_OnChaosBreakEvent {
    _opague: u8,
}
#[repr(transparent)]
pub struct FGeometryCollectionComponent_OnChaosRemovalEvent {
    _opague: u8,
}
#[repr(transparent)]
pub struct FGeometryCollectionComponent_OnChaosCrumblingEvent {
    _opague: u8,
}
#[repr(transparent)]
pub struct FGeometryCollectionComponent_OnChaosPhysicsCollision {
    _opague: u8,
}
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
