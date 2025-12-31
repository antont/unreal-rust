#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FChaosDestructionEvent {
    pub position: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub velocity: crate::bindings::core_u_object::FVector,
    pub angular_velocity: crate::bindings::core_u_object::FVector,
    pub extent_min: f32,
    pub extent_max: f32,
    pub particle_id: i32,
    pub time: f32,
    pub ty: i32,
}
pub struct UNiagaraDataInterfaceChaosDestruction {
    pub chaos_solver_actor_set: TSet<
        UPtr<crate::bindings::chaos_solver_engine::AChaosSolverActor>,
    >,
    pub data_source_type: EDataSourceTypeEnum,
    pub data_process_frequency: i32,
    pub max_number_of_data_entries_to_spawn: i32,
    pub do_spawn: bool,
    pub spawn_multiplier_min_max: crate::bindings::core_u_object::FVector2D,
    pub spawn_chance: f32,
    pub impulse_to_spawn_min_max: crate::bindings::core_u_object::FVector2D,
    pub speed_to_spawn_min_max: crate::bindings::core_u_object::FVector2D,
    pub mass_to_spawn_min_max: crate::bindings::core_u_object::FVector2D,
    pub extent_min_to_spawn_min_max: crate::bindings::core_u_object::FVector2D,
    pub extent_max_to_spawn_min_max: crate::bindings::core_u_object::FVector2D,
    pub volume_to_spawn_min_max: crate::bindings::core_u_object::FVector2D,
    pub solver_time_to_spawn_min_max: crate::bindings::core_u_object::FVector2D,
    pub surface_type_to_spawn: i32,
    pub location_filtering_mode: ELocationFilteringModeEnum,
    pub location_x_to_spawn: ELocationXToSpawnEnum,
    pub location_x_to_spawn_min_max: crate::bindings::core_u_object::FVector2D,
    pub location_y_to_spawn: ELocationYToSpawnEnum,
    pub location_y_to_spawn_min_max: crate::bindings::core_u_object::FVector2D,
    pub location_z_to_spawn: ELocationZToSpawnEnum,
    pub location_z_to_spawn_min_max: crate::bindings::core_u_object::FVector2D,
    pub trail_min_speed_to_spawn: f32,
    pub data_sorting_type: EDataSortTypeEnum,
    pub b_get_external_collision_data: bool,
    pub do_spatial_hash: bool,
    pub spatial_hash_volume_min: crate::bindings::core_u_object::FVector,
    pub spatial_hash_volume_max: crate::bindings::core_u_object::FVector,
    pub spatial_hash_volume_cell_size: crate::bindings::core_u_object::FVector,
    pub max_data_per_cell: i32,
    pub b_apply_materials_filter: bool,
    pub chaos_breaking_material_set: TSet<
        UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    >,
    pub b_get_external_breaking_data: bool,
    pub b_get_external_trailing_data: bool,
    pub random_position_magnitude_min_max: crate::bindings::core_u_object::FVector2D,
    pub inherited_velocity_multiplier: f32,
    pub random_velocity_generation_type: ERandomVelocityGenerationTypeEnum,
    pub random_velocity_magnitude_min_max: crate::bindings::core_u_object::FVector2D,
    pub spread_angle_max: f32,
    pub velocity_offset_min: crate::bindings::core_u_object::FVector,
    pub velocity_offset_max: crate::bindings::core_u_object::FVector,
    pub final_velocity_magnitude_min_max: crate::bindings::core_u_object::FVector2D,
    pub max_latency: f32,
    pub debug_type: EDebugTypeEnum,
    pub last_spawned_point_id: i32,
    pub last_spawn_time: f32,
    pub solver_time: f32,
    pub time_stamp_of_last_processed_data: f32,
}
pub struct UNiagaraDataInterfaceGeometryCollection {
    pub source_mode: ENDIGeometryCollection_SourceMode,
    pub preview_collection: TSoftObjectPtr<
        crate::bindings::geometry_collection_engine::UGeometryCollection,
    >,
    pub default_geometry_collection: UPtr<
        crate::bindings::geometry_collection_engine::UGeometryCollection,
    >,
    pub geometry_collection_actor: TSoftObjectPtr<
        crate::bindings::geometry_collection_engine::AGeometryCollectionActor,
    >,
    pub source_component: UPtr<
        crate::bindings::geometry_collection_engine::UGeometryCollectionComponent,
    >,
    pub geometry_collection_user_parameter: crate::bindings::niagara::FNiagaraUserParameterBinding,
    pub b_include_intermediate_bones: bool,
}
pub struct UNiagaraDataInterfacePhysicsField {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataSourceTypeEnum(pub u8);
impl EDataSourceTypeEnum {
    pub const CHAOS_NIAGARA_DATA_SOURCE_TYPE_COLLISION: EDataSourceTypeEnum = EDataSourceTypeEnum(
        0,
    );
    pub const CHAOS_NIAGARA_DATA_SOURCE_TYPE_BREAKING: EDataSourceTypeEnum = EDataSourceTypeEnum(
        1,
    );
    pub const CHAOS_NIAGARA_DATA_SOURCE_TYPE_TRAILING: EDataSourceTypeEnum = EDataSourceTypeEnum(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELocationFilteringModeEnum(pub u8);
impl ELocationFilteringModeEnum {
    pub const CHAOS_NIAGARA_LOCATION_FILTERING_MODE_INCLUSIVE: ELocationFilteringModeEnum = ELocationFilteringModeEnum(
        0,
    );
    pub const CHAOS_NIAGARA_LOCATION_FILTERING_MODE_EXCLUSIVE: ELocationFilteringModeEnum = ELocationFilteringModeEnum(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELocationXToSpawnEnum(pub u8);
impl ELocationXToSpawnEnum {
    pub const CHAOS_NIAGARA_LOCATION_X_TO_SPAWN_NONE: ELocationXToSpawnEnum = ELocationXToSpawnEnum(
        0,
    );
    pub const CHAOS_NIAGARA_LOCATION_X_TO_SPAWN_MIN: ELocationXToSpawnEnum = ELocationXToSpawnEnum(
        1,
    );
    pub const CHAOS_NIAGARA_LOCATION_X_TO_SPAWN_MIN_MAX: ELocationXToSpawnEnum = ELocationXToSpawnEnum(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELocationYToSpawnEnum(pub u8);
impl ELocationYToSpawnEnum {
    pub const CHAOS_NIAGARA_LOCATION_Y_TO_SPAWN_NONE: ELocationYToSpawnEnum = ELocationYToSpawnEnum(
        0,
    );
    pub const CHAOS_NIAGARA_LOCATION_Y_TO_SPAWN_MIN: ELocationYToSpawnEnum = ELocationYToSpawnEnum(
        1,
    );
    pub const CHAOS_NIAGARA_LOCATION_Y_TO_SPAWN_MIN_MAX: ELocationYToSpawnEnum = ELocationYToSpawnEnum(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELocationZToSpawnEnum(pub u8);
impl ELocationZToSpawnEnum {
    pub const CHAOS_NIAGARA_LOCATION_Z_TO_SPAWN_NONE: ELocationZToSpawnEnum = ELocationZToSpawnEnum(
        0,
    );
    pub const CHAOS_NIAGARA_LOCATION_Z_TO_SPAWN_MIN: ELocationZToSpawnEnum = ELocationZToSpawnEnum(
        1,
    );
    pub const CHAOS_NIAGARA_LOCATION_Z_TO_SPAWN_MIN_MAX: ELocationZToSpawnEnum = ELocationZToSpawnEnum(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataSortTypeEnum(pub u8);
impl EDataSortTypeEnum {
    pub const CHAOS_NIAGARA_DATA_SORT_TYPE_NO_SORTING: EDataSortTypeEnum = EDataSortTypeEnum(
        0,
    );
    pub const CHAOS_NIAGARA_DATA_SORT_TYPE_RANDOM_SHUFFLE: EDataSortTypeEnum = EDataSortTypeEnum(
        1,
    );
    pub const CHAOS_NIAGARA_DATA_SORT_TYPE_SORT_BY_MASS_MAX_TO_MIN: EDataSortTypeEnum = EDataSortTypeEnum(
        2,
    );
    pub const CHAOS_NIAGARA_DATA_SORT_TYPE_SORT_BY_MASS_MIN_TO_MAX: EDataSortTypeEnum = EDataSortTypeEnum(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERandomVelocityGenerationTypeEnum(pub u8);
impl ERandomVelocityGenerationTypeEnum {
    pub const CHAOS_NIAGARA_RANDOM_VELOCITY_GENERATION_TYPE_RANDOM_DISTRIBUTION: ERandomVelocityGenerationTypeEnum = ERandomVelocityGenerationTypeEnum(
        0,
    );
    pub const CHAOS_NIAGARA_RANDOM_VELOCITY_GENERATION_TYPE_RANDOM_DISTRIBUTION_WITH_STREAMERS: ERandomVelocityGenerationTypeEnum = ERandomVelocityGenerationTypeEnum(
        1,
    );
    pub const CHAOS_NIAGARA_RANDOM_VELOCITY_GENERATION_TYPE_COLLISION_NORMAL_BASED: ERandomVelocityGenerationTypeEnum = ERandomVelocityGenerationTypeEnum(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDebugTypeEnum(pub u8);
impl EDebugTypeEnum {
    pub const CHAOS_NIAGARA_DEBUG_TYPE_NO_DEBUG: EDebugTypeEnum = EDebugTypeEnum(0);
    pub const CHAOS_NIAGARA_DEBUG_TYPE_COLOR_BY_SOLVER: EDebugTypeEnum = EDebugTypeEnum(
        1,
    );
    pub const CHAOS_NIAGARA_DEBUG_TYPE_COLOR_BY_PARTICLE_INDEX: EDebugTypeEnum = EDebugTypeEnum(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENDIGeometryCollection_SourceMode(pub u8);
impl ENDIGeometryCollection_SourceMode {
    pub const DEFAULT: ENDIGeometryCollection_SourceMode = ENDIGeometryCollection_SourceMode(
        0,
    );
    pub const SOURCE: ENDIGeometryCollection_SourceMode = ENDIGeometryCollection_SourceMode(
        1,
    );
    pub const ATTACH_PARENT: ENDIGeometryCollection_SourceMode = ENDIGeometryCollection_SourceMode(
        2,
    );
    pub const DEFAULT_COLLECTION_ONLY: ENDIGeometryCollection_SourceMode = ENDIGeometryCollection_SourceMode(
        3,
    );
    pub const PARAMETER_BINDING: ENDIGeometryCollection_SourceMode = ENDIGeometryCollection_SourceMode(
        4,
    );
}
