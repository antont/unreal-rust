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
#[repr(C, align(8))]
pub struct FRelayConnectionInfo {
    pub address: FString,
    pub port: u16,
    pub certificate_authority: TArray<u8>,
}
#[repr(C, align(16))]
pub struct FBreakEventCallbackWrapper {}
#[repr(C, align(16))]
pub struct FRemovalEventCallbackWrapper {}
#[repr(C, align(16))]
pub struct FCrumblingEventCallbackWrapper {}
#[repr(C, align(8))]
pub struct FChaosHandlerSet {
    pub chaos_handlers: TSet<UPtr<crate::bindings::core_u_object::UObject>>,
}
#[repr(C, align(8))]
pub struct FChaosDebugSubstepControl {
    pub b_pause: bool,
    pub b_substep: bool,
    pub b_step: bool,
}
#[repr(C, align(8))]
pub struct FDataflowRigidSolverProxy {}
#[repr(C, align(4))]
pub struct FChaosVDSessionPing {
    pub controller_instance_id: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FChaosVDSessionPong {
    pub instance_id: crate::bindings::core_u_object::FGuid,
    pub session_id: crate::bindings::core_u_object::FGuid,
    pub session_name: FString,
    pub build_target_type: u8,
}
#[repr(C, align(1))]
pub struct FChaosVDStopRecordingCommandMessage {}
#[repr(C, align(8))]
pub struct FChaosVDRecordingStatusMessage {
    pub instance_id: crate::bindings::core_u_object::FGuid,
    pub b_is_recording: bool,
    pub elapsed_time: f32,
}
#[repr(C, align(8))]
pub struct FChaosVDTraceConnectionDetailsMessage {
    pub instance_id: crate::bindings::core_u_object::FGuid,
    pub trace_details: crate::bindings::chaos_vd_runtime::FChaosVDTraceDetails,
}
#[repr(C, align(8))]
pub struct FChaosVDRelayTraceDataMessage {
    pub instance_id: crate::bindings::core_u_object::FGuid,
    pub data_buffer: TArray<u8>,
}
#[repr(C, align(8))]
pub struct FChaosVDRelayTraceStatusMessage {
    pub instance_id: crate::bindings::core_u_object::FGuid,
    pub queued_data_bytes_num: i64,
    pub throttling_state: ERelayThrottlingState,
}
#[repr(C, align(8))]
pub struct FChaosVDDataChannelState {
    pub channel_name: FString,
    pub b_is_enabled: bool,
    pub b_can_change_channel_state: bool,
}
#[repr(C, align(8))]
pub struct FChaosVDChannelStateChangeCommandMessage {
    pub new_state: FChaosVDDataChannelState,
}
#[repr(C, align(8))]
pub struct FChaosVDChannelStateChangeResponseMessage {
    pub instance_id: crate::bindings::core_u_object::FGuid,
    pub new_state: FChaosVDDataChannelState,
}
#[repr(C, align(1))]
pub struct FChaosVDFullSessionInfoRequestMessage {}
#[repr(C, align(8))]
pub struct FChaosVDFullSessionInfoResponseMessage {
    pub instance_id: crate::bindings::core_u_object::FGuid,
    pub data_channels_states: TArray<FChaosVDDataChannelState>,
    pub b_is_recording: bool,
}
pub struct UChaosDebugDrawComponent {}
pub struct UChaosDebugDrawSubsystem {}
pub struct UChaosEventListenerComponent {}
pub struct UChaosGameplayEventDispatcher {
    pub collision_event_registrations: TMap<
        UPtr<crate::bindings::engine::UPrimitiveComponent>,
        FChaosHandlerSet,
    >,
    pub break_event_registrations: TMap<
        UPtr<crate::bindings::engine::UPrimitiveComponent>,
        FBreakEventCallbackWrapper,
    >,
    pub removal_event_registrations: TMap<
        UPtr<crate::bindings::engine::UPrimitiveComponent>,
        FRemovalEventCallbackWrapper,
    >,
    pub crumbling_event_registrations: TMap<
        UPtr<crate::bindings::engine::UPrimitiveComponent>,
        FCrumblingEventCallbackWrapper,
    >,
}
pub struct UChaosNotifyHandlerInterface {}
pub struct IChaosNotifyHandlerInterface {}
pub struct UChaosSolverEngineBlueprintLibrary {}
pub struct UChaosSolver {}
pub struct AChaosSolverActor {
    pub properties: crate::bindings::chaos::FChaosSolverConfiguration,
    pub time_step_multiplier_deprecated: f32,
    pub collision_iterations_deprecated: i32,
    pub push_out_iterations_deprecated: i32,
    pub push_out_pair_iterations_deprecated: i32,
    pub cluster_connection_factor_deprecated: f32,
    pub cluster_union_connection_type_deprecated: EClusterConnectionTypeEnum,
    pub do_generate_collision_data_deprecated: bool,
    pub collision_filter_settings_deprecated: crate::bindings::chaos::FSolverCollisionFilterSettings,
    pub do_generate_breaking_data_deprecated: bool,
    pub breaking_filter_settings_deprecated: crate::bindings::chaos::FSolverBreakingFilterSettings,
    pub do_generate_trailing_data_deprecated: bool,
    pub trailing_filter_settings_deprecated: crate::bindings::chaos::FSolverTrailingFilterSettings,
    pub mass_scale_deprecated: f32,
    pub b_has_floor: bool,
    pub floor_height: f32,
    pub chaos_debug_substep_control: FChaosDebugSubstepControl,
    pub sprite_component: UPtr<crate::bindings::engine::UBillboardComponent>,
    pub simulation_asset: crate::bindings::dataflow_simulation::FDataflowSimulationAsset,
    pub gameplay_event_dispatcher_component: UPtr<UChaosGameplayEventDispatcher>,
}
pub struct UChaosSolverSettings {
    pub default_chaos_solver_actor_class: crate::bindings::core_u_object::FSoftClassPath,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERelayThrottlingState(pub u8);
impl ERelayThrottlingState {
    pub const INACTIVE: ERelayThrottlingState = ERelayThrottlingState(0);
    pub const ACTIVE: ERelayThrottlingState = ERelayThrottlingState(1);
}
#[allow(non_camel_case_types)]
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
