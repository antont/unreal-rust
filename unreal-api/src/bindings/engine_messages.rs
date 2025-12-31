#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FEngineServicePing {}
#[repr(C, align(8))]
pub struct FEngineServicePong {
    pub current_level: FString,
    pub engine_version: i32,
    pub has_begun_play: bool,
    pub instance_id: crate::bindings::core_u_object::FGuid,
    pub instance_type: FString,
    pub session_id: crate::bindings::core_u_object::FGuid,
    pub world_time_seconds: f32,
}
#[repr(C, align(8))]
pub struct FEngineServiceAuthDeny {
    pub user_name: FString,
    pub user_to_deny: FString,
}
#[repr(C, align(8))]
pub struct FEngineServiceAuthGrant {
    pub user_name: FString,
    pub user_to_grant: FString,
}
#[repr(C, align(8))]
pub struct FEngineServiceExecuteCommand {
    pub command: FString,
    pub user_name: FString,
}
#[repr(C, align(8))]
pub struct FEngineServiceTerminate {
    pub user_name: FString,
}
#[repr(C, align(8))]
pub struct FEngineServiceNotification {
    pub text: FString,
    pub time_seconds: f64,
}
#[repr(C, align(1))]
pub struct FTraceControlStatusPing {}
#[repr(C, align(8))]
pub struct FTraceControlStatus {
    pub endpoint: FString,
    pub session_guid: crate::bindings::core_u_object::FGuid,
    pub trace_guid: crate::bindings::core_u_object::FGuid,
    pub bytes_sent: u64,
    pub bytes_traced: u64,
    pub memory_used: u64,
    pub cache_allocated: u32,
    pub cache_used: u32,
    pub cache_waste: u32,
    pub b_are_stat_named_events_enabled: bool,
    pub b_is_paused: bool,
    pub b_is_tracing: bool,
    pub status_timestamp: crate::bindings::core_u_object::FDateTime,
    pub trace_system_status: u8,
}
#[repr(C, align(1))]
pub struct FTraceControlSettingsPing {}
#[repr(C, align(8))]
pub struct FTraceChannelPreset {
    pub name: FString,
    pub channel_list: FString,
    pub b_is_read_only: bool,
}
#[repr(C, align(8))]
pub struct FTraceControlSettings {
    pub b_use_worker_thread: bool,
    pub b_use_important_cache: bool,
    pub tail_size_bytes: u32,
    pub channel_presets: TArray<FTraceChannelPreset>,
}
#[repr(C, align(4))]
pub struct FTraceControlChannelsPing {
    pub known_channel_count: u32,
}
#[repr(C, align(8))]
pub struct FTraceControlChannelsDesc {
    pub channels: TArray<FString>,
    pub ids: TArray<u32>,
    pub descriptions: TArray<FString>,
    pub read_only_ids: TArray<u32>,
}
#[repr(C, align(8))]
pub struct FTraceControlChannelsStatus {
    pub enabled_ids: TArray<u32>,
}
#[repr(C, align(8))]
pub struct FTraceControlChannelsSet {
    pub channel_ids_to_enable: TArray<u32>,
    pub channel_ids_to_disable: TArray<u32>,
}
#[repr(C, align(8))]
pub struct FTraceControlChannelsSetError {
    pub errors: TMap<u32, FString>,
}
#[repr(C, align(4))]
pub struct FTraceControlDiscoveryPing {
    pub session_id: crate::bindings::core_u_object::FGuid,
    pub instance_id: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FTraceControlDiscovery {
    pub session_id: crate::bindings::core_u_object::FGuid,
    pub instance_id: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(1))]
pub struct FTraceControlStop {}
#[repr(C, align(8))]
pub struct FTraceControlStartCommon {
    pub channels: FString,
    pub b_exclude_tail: bool,
}
#[repr(C, align(8))]
pub struct FTraceControlSend {
    pub host: FString,
}
#[repr(C, align(8))]
pub struct FTraceControlFile {
    pub file: FString,
    pub b_truncate_file: bool,
}
#[repr(C, align(1))]
pub struct FTraceControlPause {}
#[repr(C, align(1))]
pub struct FTraceControlResume {}
#[repr(C, align(8))]
pub struct FTraceControlSnapshotSend {
    pub host: FString,
}
#[repr(C, align(8))]
pub struct FTraceControlSnapshotFile {
    pub file: FString,
}
#[repr(C, align(8))]
pub struct FTraceControlBookmark {
    pub label: FString,
}
#[repr(C, align(8))]
pub struct FTraceControlScreenshot {
    pub name: FString,
    pub b_show_ui: bool,
}
#[repr(C, align(1))]
pub struct FTraceControlSetStatNamedEvents {
    pub b_enabled: bool,
}
