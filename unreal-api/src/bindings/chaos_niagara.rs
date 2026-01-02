#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceChaosDestruction {
    __padding_end: [u8; 936],
}
impl UNiagaraDataInterfaceChaosDestruction {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceGeometryCollection {
    __padding_end: [u8; 352],
}
impl UNiagaraDataInterfaceGeometryCollection {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfacePhysicsField {
    __padding_end: [u8; 152],
}
impl UNiagaraDataInterfacePhysicsField {}
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
