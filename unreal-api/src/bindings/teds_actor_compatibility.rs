#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FActorComponentTypeTag {}
#[repr(C, align(1))]
pub struct FTedsActorMobilityColumn {
    pub mobility: crate::bindings::engine::EComponentMobility,
}
pub struct UActorLevelDataStorageFactory {}
pub struct UActorIconOverrideDataStorageFactory {}
pub struct UActorLabelDataStorageFactory {}
pub struct UActorMobilityDataStorageFactory {}
pub struct UActorParentDataStorageFactory {}
pub struct UActorViewportDataStorageFactory {}
pub struct UActorVisibilityDataStorageFactory {}
pub struct UTedsActorCompatibilityFactory {}
