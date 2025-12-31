#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FNetAnalyticsDataConfig {
    pub data_name: FName,
    pub b_enabled: bool,
}
#[repr(C, align(8))]
pub struct FStateStruct {
    pub state_name: FString,
}
#[repr(C, align(8))]
pub struct FEscalationState {
    pub b_log_escalate: bool,
    pub b_dormant: bool,
    pub cooloff_time: i16,
    pub auto_escalate_time: i16,
    pub highest_time_period: i8,
    pub all_time_periods: TArray<i8>,
}
#[repr(C, align(8))]
pub struct FNetFaultState {
    pub b_close_connection: bool,
    pub escalate_quota_faults_per_period: i16,
    pub escalate_quota_fault_percent_per_period: i8,
    pub descalate_quota_faults_per_period: i16,
    pub descalate_quota_fault_percent_per_period: i8,
    pub escalate_quota_time_period: i8,
}
#[repr(C, align(4))]
pub struct FFastArraySerializerItem {
    pub replication_id: i32,
    pub replication_key: i32,
    pub most_recent_array_replication_key: i32,
}
#[repr(C, align(8))]
pub struct FFastArraySerializer {
    pub id_counter: i32,
    pub array_replication_key: i32,
    pub delta_flags: EFastArraySerializerDeltaFlags,
}
pub struct UNetAnalyticsAggregatorConfig {
    pub net_analytics_data: TArray<FNetAnalyticsDataConfig>,
}
pub struct UStatePerObjectConfig {
    pub per_object_config_section: FString,
    pub b_enabled: bool,
}
pub struct UEscalationManagerConfig {
    pub escalation_severity: TArray<FString>,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFastArraySerializerDeltaFlags(pub u8);
impl EFastArraySerializerDeltaFlags {
    pub const NONE: EFastArraySerializerDeltaFlags = EFastArraySerializerDeltaFlags(0);
    pub const HAS_BEEN_SERIALIZED: EFastArraySerializerDeltaFlags = EFastArraySerializerDeltaFlags(
        1,
    );
    pub const HAS_DELTA_BEEN_REQUESTED: EFastArraySerializerDeltaFlags = EFastArraySerializerDeltaFlags(
        2,
    );
    pub const IS_USING_DELTA_SERIALIZATION: EFastArraySerializerDeltaFlags = EFastArraySerializerDeltaFlags(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENetworkFailure(pub u8);
impl ENetworkFailure {
    pub const NET_DRIVER_ALREADY_EXISTS: ENetworkFailure = ENetworkFailure(0);
    pub const NET_DRIVER_CREATE_FAILURE: ENetworkFailure = ENetworkFailure(1);
    pub const NET_DRIVER_LISTEN_FAILURE: ENetworkFailure = ENetworkFailure(2);
    pub const CONNECTION_LOST: ENetworkFailure = ENetworkFailure(3);
    pub const CONNECTION_TIMEOUT: ENetworkFailure = ENetworkFailure(4);
    pub const FAILURE_RECEIVED: ENetworkFailure = ENetworkFailure(5);
    pub const OUTDATED_CLIENT: ENetworkFailure = ENetworkFailure(6);
    pub const OUTDATED_SERVER: ENetworkFailure = ENetworkFailure(7);
    pub const PENDING_CONNECTION_FAILURE: ENetworkFailure = ENetworkFailure(8);
    pub const NET_GUID_MISMATCH: ENetworkFailure = ENetworkFailure(9);
    pub const NET_CHECKSUM_MISMATCH: ENetworkFailure = ENetworkFailure(10);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EReplicationSystem(pub u8);
impl EReplicationSystem {
    pub const DEFAULT: EReplicationSystem = EReplicationSystem(0);
    pub const GENERIC: EReplicationSystem = EReplicationSystem(1);
    pub const IRIS: EReplicationSystem = EReplicationSystem(2);
}
