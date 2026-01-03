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
pub struct IDataflowGeometryCachable {}
#[repr(C, align(8))]
pub struct UDataflowGeometryCachable {
    __padding_end: [u8; 48],
}
impl UDataflowGeometryCachable {}
pub struct IDataflowSimulationInterface {}
#[repr(C, align(8))]
pub struct UDataflowSimulationInterface {
    __padding_end: [u8; 48],
}
impl UDataflowSimulationInterface {}
#[repr(C, align(8))]
pub struct UDataflowSimulationManager {
    __padding_end: [u8; 184],
}
impl UDataflowSimulationManager {}
pub struct IDataflowSimulationActor {}
#[repr(C, align(8))]
pub struct UDataflowSimulationActor {
    __padding_end: [u8; 48],
}
impl UDataflowSimulationActor {}
pub struct IDataflowCollisionObjectInterface {}
#[repr(C, align(8))]
pub struct UDataflowCollisionObjectInterface {
    __padding_end: [u8; 48],
}
impl UDataflowCollisionObjectInterface {}
pub struct IDataflowConstraintObjectInterface {}
#[repr(C, align(8))]
pub struct UDataflowConstraintObjectInterface {
    __padding_end: [u8; 48],
}
impl UDataflowConstraintObjectInterface {}
pub struct IDataflowPhysicsObjectInterface {}
#[repr(C, align(8))]
pub struct UDataflowPhysicsObjectInterface {
    __padding_end: [u8; 48],
}
impl UDataflowPhysicsObjectInterface {}
pub struct IDataflowPhysicsSolverInterface {}
#[repr(C, align(8))]
pub struct UDataflowPhysicsSolverInterface {
    __padding_end: [u8; 48],
}
impl UDataflowPhysicsSolverInterface {}
