#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FChaosPhysicsCollisionInfo {
    pub component: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub other_component: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub location: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub accumulated_impulse: crate::bindings::core_u_object::FVector,
    pub velocity: crate::bindings::core_u_object::FVector,
    pub other_velocity: crate::bindings::core_u_object::FVector,
    pub angular_velocity: crate::bindings::core_u_object::FVector,
    pub other_angular_velocity: crate::bindings::core_u_object::FVector,
    pub mass: f32,
    pub other_mass: f32,
}
impl FChaosPhysicsCollisionInfo {}
#[repr(C, align(8))]
pub struct FDataflowRigidSolverProxy {
    __padding_end: [u8; 144],
}
impl FDataflowRigidSolverProxy {}
#[repr(C, align(8))]
pub struct UChaosDebugDrawComponent {
    __padding_end: [u8; 264],
}
impl UChaosDebugDrawComponent {}
#[repr(C, align(8))]
pub struct UChaosDebugDrawSubsystem {
    __padding_end: [u8; 136],
}
impl UChaosDebugDrawSubsystem {}
#[repr(C, align(8))]
pub struct UChaosEventListenerComponent {
    __padding_end: [u8; 248],
}
impl UChaosEventListenerComponent {}
#[repr(C, align(8))]
pub struct UChaosGameplayEventDispatcher {
    __padding_end: [u8; 776],
}
impl UChaosGameplayEventDispatcher {}
pub struct IChaosNotifyHandlerInterface {}
#[repr(C, align(8))]
pub struct UChaosNotifyHandlerInterface {
    __padding_end: [u8; 48],
}
impl UChaosNotifyHandlerInterface {}
#[repr(C, align(8))]
pub struct UChaosSolverEngineBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UChaosSolverEngineBlueprintLibrary {}
#[repr(C, align(8))]
pub struct UChaosSolver {
    __padding_end: [u8; 48],
}
impl UChaosSolver {}
#[repr(C, align(8))]
pub struct AChaosSolverActor {
    __padding_end: [u8; 1672],
}
impl AChaosSolverActor {}
#[repr(C, align(8))]
pub struct UChaosSolverSettings {
    __padding_end: [u8; 152],
}
impl UChaosSolverSettings {}
#[repr(transparent)]
pub struct ERelayThrottlingState(pub u8);
impl ERelayThrottlingState {
    pub const INACTIVE: ERelayThrottlingState = ERelayThrottlingState(0);
    pub const ACTIVE: ERelayThrottlingState = ERelayThrottlingState(1);
}
#[repr(transparent)]
pub struct EClusterConnectionTypeEnum(pub u8);
impl EClusterConnectionTypeEnum {
    pub const CHAOS_POINT_IMPLICIT: EClusterConnectionTypeEnum = EClusterConnectionTypeEnum(
        0,
    );
    pub const CHAOS_DELAUNAY_TRIANGULATION: EClusterConnectionTypeEnum = EClusterConnectionTypeEnum(
        1,
    );
    pub const CHAOS_MINIMAL_SPANNING_SUBSET_DELAUNAY_TRIANGULATION: EClusterConnectionTypeEnum = EClusterConnectionTypeEnum(
        2,
    );
    pub const CHAOS_POINT_IMPLICIT_AUGMENTED_WITH_MINIMAL_DELAUNAY: EClusterConnectionTypeEnum = EClusterConnectionTypeEnum(
        3,
    );
    pub const CHAOS_BOUNDS_OVERLAP_FILTERED_DELAUNAY_TRIANGULATION: EClusterConnectionTypeEnum = EClusterConnectionTypeEnum(
        4,
    );
    pub const CHAOS_NONE: EClusterConnectionTypeEnum = EClusterConnectionTypeEnum(5);
}
