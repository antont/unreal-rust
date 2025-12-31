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
#[repr(C, align(8))]
pub struct FDataflowSimulationProperty {}
#[repr(C, align(8))]
pub struct FDataflowSimulationNode {}
#[repr(C, align(8))]
pub struct FDataflowInvalidNode {}
#[repr(C, align(8))]
pub struct FDataflowExecutionNode {}
#[repr(C, align(4))]
pub struct FDataflowSimulationTime {
    pub delta_time: f32,
    pub current_time: f32,
    pub time_offset: f32,
}
#[repr(C, align(8))]
pub struct FGetSimulationTimeDataflowNode {
    pub simulation_time: FDataflowSimulationTime,
}
#[repr(C, align(8))]
pub struct FGetPhysicsSolversDataflowNode {
    pub physics_solvers: TArray<FDataflowSimulationProperty>,
    pub simulation_groups: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FAdvancePhysicsSolversDataflowNode {
    pub simulation_time: FDataflowSimulationTime,
    pub physics_solvers: TArray<FDataflowSimulationProperty>,
}
#[repr(C, align(8))]
pub struct FFilterSimulationProxiesDataflowNode {
    pub simulation_proxies: TArray<FDataflowSimulationProperty>,
    pub filtered_proxies: TArray<FDataflowSimulationProperty>,
    pub simulation_groups: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FSimulationProxiesTerminalDataflowNode {
    pub simulation_proxies: TArray<FDataflowSimulationProperty>,
}
#[repr(C, align(8))]
pub struct FDataflowSimulationProxy {}
#[repr(C, align(8))]
pub struct FDataflowCollisionObjectProxy {}
#[repr(C, align(8))]
pub struct FDataflowConstraintObjectProxy {}
#[repr(C, align(8))]
pub struct FDataflowPhysicsObjectProxy {}
#[repr(C, align(8))]
pub struct FDataflowPhysicsSolverProxy {}
pub struct UDataflowGeometryCachable {}
pub struct IDataflowGeometryCachable {}
pub struct UDataflowSimulationInterface {}
pub struct IDataflowSimulationInterface {}
pub struct UDataflowSimulationManager {}
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
