#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FDataflowSimulationAsset {
    pub dataflow_asset: UPtr<crate::bindings::dataflow_engine::UDataflow>,
    pub simulation_groups: TSet<FString>,
}
impl FDataflowSimulationAsset {}
#[repr(C, align(8))]
pub struct FDataflowSimulationProperty {
    __padding_end: [u8; 8],
}
impl FDataflowSimulationProperty {}
pub struct UDataflowGeometryCachable {}
pub struct IDataflowGeometryCachable {}
pub struct UDataflowSimulationInterface {}
pub struct IDataflowSimulationInterface {}
#[repr(C, align(8))]
pub struct UDataflowSimulationManager {
    __padding_end: [u8; 184],
}
impl UDataflowSimulationManager {}
pub struct UDataflowSimulationActor {}
pub struct IDataflowSimulationActor {}
pub struct UDataflowCollisionObjectInterface {}
pub struct IDataflowCollisionObjectInterface {}
pub struct UDataflowConstraintObjectInterface {}
pub struct IDataflowConstraintObjectInterface {}
pub struct UDataflowPhysicsObjectInterface {}
pub struct IDataflowPhysicsObjectInterface {}
pub struct UDataflowPhysicsSolverInterface {}
pub struct IDataflowPhysicsSolverInterface {}
