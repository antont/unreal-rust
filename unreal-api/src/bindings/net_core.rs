#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
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
