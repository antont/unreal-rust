#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FChaosVDRecentFile {
    pub file_name: FString,
    pub last_open_time: crate::bindings::core_u_object::FDateTime,
}
#[repr(C, align(4))]
pub struct FChaosVDFrameIndexTestData {
    pub frame_number: i32,
    pub stage_number: i32,
}
#[repr(C, align(8))]
pub struct FChaosVDSceneCompositionTestData {
    pub objects_count_by_type: TMap<FName, i32>,
}
#[repr(C, align(8))]
pub struct FChaosVDPlaybackEngineSnapshot {
    pub installed_extensions_names: TArray<FName>,
    pub frame_index_data_by_track_id: TMap<i32, FChaosVDFrameIndexTestData>,
    pub timeline_sync_mode: EChaosVDSyncTimelinesMode,
    pub scene_composition: FChaosVDSceneCompositionTestData,
}
#[repr(C, align(8))]
pub struct FChaosVDBaseSceneObject {}
#[repr(C, align(16))]
pub struct FChaosVDImplicitObjectBasicView {
    pub implicit_object_type: FName,
    pub shape_instance_index: i32,
    pub b_is_root_object: bool,
    pub relative_transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FChaosVDMeshDataInstanceState {
    pub collision_data: crate::bindings::chaos_vd_runtime::FChaosVDShapeCollisionData,
    pub implicit_object_info: FChaosVDImplicitObjectBasicView,
    pub current_world_transform: crate::bindings::core_u_object::FTransform,
    pub mesh_component_type: EChaosVDMeshComponent,
    pub mesh_component: UPtr<crate::bindings::engine::UMeshComponent>,
    pub mesh_instance_index: i32,
    pub current_geometry_color: crate::bindings::core_u_object::FLinearColor,
    pub owning_particle_id: i32,
    pub owning_solver_id: i32,
    pub b_is_visible: bool,
    pub b_is_selected: bool,
    pub b_show_cvd_debug_data: bool,
}
#[repr(C, align(16))]
pub struct FChaosVDSceneParticle {}
#[repr(C, align(1))]
pub struct FChaosVDSelectionContext {}
#[repr(C, align(8))]
pub struct FChaosVDSelectionMultipleView {}
#[repr(C, align(4))]
pub struct FChaosDebugDrawColorsByShapeType {
    pub simple_type_color: crate::bindings::core_u_object::FColor,
    pub convex_color: crate::bindings::core_u_object::FColor,
    pub height_field_color: crate::bindings::core_u_object::FColor,
    pub triangle_mesh_color: crate::bindings::core_u_object::FColor,
    pub level_set_color: crate::bindings::core_u_object::FColor,
}
#[repr(C, align(4))]
pub struct FChaosDebugDrawColorsByClientServer {
    pub server_base_color: crate::bindings::core_u_object::FColor,
    pub client_base_color: crate::bindings::core_u_object::FColor,
}
#[repr(C, align(4))]
pub struct FChaosDebugDrawColorsByState {
    pub dynamic_color: crate::bindings::core_u_object::FColor,
    pub sleeping_color: crate::bindings::core_u_object::FColor,
    pub kinematic_color: crate::bindings::core_u_object::FColor,
    pub static_color: crate::bindings::core_u_object::FColor,
}
#[repr(C, align(4))]
pub struct FChaosParticleDataDebugDrawColors {
    pub velocity_color: crate::bindings::core_u_object::FLinearColor,
    pub angular_velocity_color: crate::bindings::core_u_object::FLinearColor,
    pub acceleration_color: crate::bindings::core_u_object::FLinearColor,
    pub angular_acceleration_color: crate::bindings::core_u_object::FLinearColor,
    pub linear_impulse_color: crate::bindings::core_u_object::FLinearColor,
    pub angular_impulse_color: crate::bindings::core_u_object::FLinearColor,
    pub center_of_mass_color: crate::bindings::core_u_object::FLinearColor,
    pub connectivity_data_color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FChaosVDTableRowParentColumn {}
#[repr(C, align(1))]
pub struct FChaosVDObjectDataTag {}
#[repr(C, align(1))]
pub struct FTypedElementFromCVDWorldTag {}
#[repr(C, align(1))]
pub struct FChaosVDActiveObjectTag {}
#[repr(C, align(4))]
pub struct FChaosVDSceneQuerySelectionContext {}
#[repr(C, align(8))]
pub struct FChaosVDCollisionDataSelectionContext {}
pub struct UChaosVDCharacterGroundConstraintDataProviderInterface {}
pub struct IChaosVDCharacterGroundConstraintDataProviderInterface {}
pub struct UChaosVDSettingsObjectBase {}
pub struct UChaosVDMiscSettings {
    pub recent_files: TArray<FChaosVDRecentFile>,
    pub max_recent_files_num: i32,
    pub data_channel_enabled_state: TMap<FString, bool>,
}
pub struct UChaosVDPooledObject {}
pub struct IChaosVDPooledObject {}
pub struct UChaosVDSelectableObject {}
pub struct IChaosVDSelectableObject {}
pub struct UChaosVDSkySphereInterface {}
pub struct IChaosVDSkySphereInterface {}
pub struct UChaosVDSolverTrackSettings {
    pub sync_mode: EChaosVDSyncTimelinesMode,
}
pub struct AChaosVDDataContainerBaseActor {}
pub struct AChaosVDGameFrameInfoActor {}
pub struct AChaosVDGeometryContainer {}
pub struct AChaosVDSolverInfoActor {
    pub simulation_transform: crate::bindings::core_u_object::FTransform,
    pub solver_name: FName,
    pub collision_data_component: UPtr<UChaosVDSolverCollisionDataComponent>,
    pub particle_data_component: UPtr<UChaosVDParticleDataComponent>,
    pub joints_data_component: UPtr<UChaosVDSolverJointConstraintDataComponent>,
    pub character_ground_constraint_data_component: UPtr<
        UChaosVDSolverCharacterGroundConstraintDataComponent,
    >,
    pub scene_query_data_component: UPtr<UChaosVDSceneQueryDataComponent>,
    pub gt_data_re_route_component: UPtr<UChaosVDAdditionalGTDataRouterComponent>,
}
pub struct UChaosVDEditorMode {}
pub struct UChaosVDGeometryComponent {}
pub struct IChaosVDGeometryComponent {}
pub struct UChaosVDSolverDataComponent {}
pub struct UChaosVDAdditionalGTDataRouterComponent {}
pub struct UChaosVDConstraintDataComponent {}
pub struct UChaosVDInstancedStaticMeshComponent {}
pub struct UChaosVDParticleDataComponent {}
pub struct UChaosVDSceneQueryDataComponent {}
pub struct UChaosVDSolverCharacterGroundConstraintDataComponent {}
pub struct UChaosVDSolverCollisionDataComponent {}
pub struct UChaosVDSolverJointConstraintDataComponent {}
pub struct UChaosVDStaticMeshComponent {}
pub struct UChaosVDVisualizationSettingsObjectBase {}
pub struct UChaosVDCharacterConstraintsVisualizationSettings {
    pub b_show_debug_text: bool,
    pub b_auto_select_constraint_from_selected_particle: bool,
    pub depth_priority: crate::bindings::engine::ESceneDepthPriorityGroup,
    pub force_scale: f32,
    pub torque_scale: f32,
    pub general_scale: f32,
    pub base_line_thickness: f32,
    pub normal_force_color: crate::bindings::core_u_object::FColor,
    pub radial_force_color: crate::bindings::core_u_object::FColor,
    pub torque_color: crate::bindings::core_u_object::FColor,
    pub global_character_ground_constraint_data_visualization_flags: u32,
}
pub struct UChaosVDCollisionDataVisualizationSettings {
    pub b_show_debug_text: bool,
    pub depth_priority: crate::bindings::engine::ESceneDepthPriorityGroup,
    pub contact_circle_radius: f32,
    pub contact_normal_scale: f32,
    pub collision_data_visualization_flags: u32,
}
pub struct UChaosVDSettingsObjectsOuter {}
pub struct UChaosVDCoreSettings {
    pub query_only_meshes_material: TSoftObjectPtr<crate::bindings::engine::UMaterial>,
    pub sim_only_meshes_material: TSoftObjectPtr<crate::bindings::engine::UMaterial>,
    pub instanced_meshes_material: TSoftObjectPtr<crate::bindings::engine::UMaterial>,
    pub instanced_meshes_query_only_material: TSoftObjectPtr<
        crate::bindings::engine::UMaterial,
    >,
    pub sky_sphere_actor_class: crate::bindings::core_u_object::FSoftClassPath,
    pub ambient_cube_map_texture: TSoftObjectPtr<crate::bindings::engine::UTextureCube>,
    pub box_mesh: TSoftObjectPtr<crate::bindings::engine::UStaticMesh>,
    pub sphere_mesh: TSoftObjectPtr<crate::bindings::engine::UStaticMesh>,
}
pub struct UChaosVDGeneralSettings {
    pub b_trim_empty_frames: bool,
    pub max_game_thread_frames_to_queue_num: i32,
    pub b_streaming_system_enabled: bool,
    pub streaming_box_extent_size: f32,
    pub b_process_pending_operations_queue_in_worker_thread: bool,
    pub b_update_scene_outliner_during_playback: bool,
    pub max_connection_retries: i32,
    pub data_transport_mode_override: crate::bindings::chaos_vd_runtime::EChaosVDTransportMode,
    pub b_save_memory_traces_to_disk: bool,
}
pub struct UChaosVDJointConstraintsVisualizationSettings {
    pub b_show_debug_text: bool,
    pub depth_priority: crate::bindings::engine::ESceneDepthPriorityGroup,
    pub linear_impulse_scale: f32,
    pub angular_impulse_scale: f32,
    pub general_scale: f32,
    pub base_line_thickness: f32,
    pub center_of_mass_size: f32,
    pub constraint_axis_length: f32,
    pub global_joints_data_visualization_flags: u32,
}
pub struct UChaosVDParticleVisualizationColorSettings {
    pub particle_color_mode: EChaosVDParticleDebugColorMode,
    pub colors_by_shape_type: FChaosDebugDrawColorsByShapeType,
    pub colors_by_particle_state: FChaosDebugDrawColorsByState,
    pub colors_by_client_server: FChaosDebugDrawColorsByClientServer,
}
pub struct UChaosVDParticleVisualizationDebugDrawSettings {
    pub b_show_debug_text: bool,
    pub depth_priority: crate::bindings::engine::ESceneDepthPriorityGroup,
    pub velocity_scale: f32,
    pub angular_velocity_scale: f32,
    pub acceleration_scale: f32,
    pub angular_acceleration_scale: f32,
    pub linear_impulse_scale: f32,
    pub angular_impulse_scale: f32,
    pub center_of_mass_radius: f32,
    pub b_draw_tri_mesh_bvh: bool,
    pub tri_mesh_bvh_draw_level: i32,
    pub color_settings: FChaosParticleDataDebugDrawColors,
    pub particle_data_visualization_flags: u32,
}
pub struct UChaosVDParticleVisualizationSettings {
    pub geometry_visibility_flags: u32,
}
pub struct UChaosVDSceneQueriesVisualizationSettings {
    pub b_show_text: bool,
    pub depth_priority: crate::bindings::engine::ESceneDepthPriorityGroup,
    pub current_visualization_mode: EChaosVDSQFrameVisualizationMode,
    pub global_scene_queries_visualization_flags: u32,
}
pub struct UChaosVDCustomIconDataStorageFactory {}
pub struct UChaosVDDataStorageVisibilityQueries {}
pub struct UChaosVDParentDataStorageFactory {}
pub struct UChaosVDParticleEditorDataFactory {}
pub struct UChaosVDSelectionInterface {}
pub struct UChaosVDLogBrowserToolbarMenuContext {}
pub struct UChaosVDMainToolbarMenuContext {}
pub struct UChaosVDRecordingToolbarMenuContext {}
pub struct UChaosVDSceneQueryBrowserToolbarMenuContext {}
pub struct UChaosVDSolverTracksToolbarMenuContext {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EChaosVDSyncTimelinesMode(pub u8);
impl EChaosVDSyncTimelinesMode {
    pub const NONE: EChaosVDSyncTimelinesMode = EChaosVDSyncTimelinesMode(0);
    pub const RECORDED_TIMESTAMP: EChaosVDSyncTimelinesMode = EChaosVDSyncTimelinesMode(
        1,
    );
    pub const NETWORK_TICK: EChaosVDSyncTimelinesMode = EChaosVDSyncTimelinesMode(2);
    pub const MANUAL: EChaosVDSyncTimelinesMode = EChaosVDSyncTimelinesMode(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EChaosVDMeshComponent(pub i32);
impl EChaosVDMeshComponent {
    pub const INVALID: EChaosVDMeshComponent = EChaosVDMeshComponent(0);
    pub const STATIC: EChaosVDMeshComponent = EChaosVDMeshComponent(1);
    pub const INSTANCED_STATIC: EChaosVDMeshComponent = EChaosVDMeshComponent(2);
    pub const DYNAMIC: EChaosVDMeshComponent = EChaosVDMeshComponent(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EChaosVDParticleDebugColorMode(pub i32);
impl EChaosVDParticleDebugColorMode {
    pub const NONE: EChaosVDParticleDebugColorMode = EChaosVDParticleDebugColorMode(0);
    pub const STATE: EChaosVDParticleDebugColorMode = EChaosVDParticleDebugColorMode(1);
    pub const SHAPE_TYPE: EChaosVDParticleDebugColorMode = EChaosVDParticleDebugColorMode(
        2,
    );
    pub const CLIENT_SERVER: EChaosVDParticleDebugColorMode = EChaosVDParticleDebugColorMode(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EChaosVDSQFrameVisualizationMode(pub u8);
impl EChaosVDSQFrameVisualizationMode {
    pub const ALL_ENABLED_QUERIES: EChaosVDSQFrameVisualizationMode = EChaosVDSQFrameVisualizationMode(
        0,
    );
    pub const PER_SOLVER_RECORDING_ORDER: EChaosVDSQFrameVisualizationMode = EChaosVDSQFrameVisualizationMode(
        1,
    );
}
