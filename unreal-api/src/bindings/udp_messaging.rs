#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FUdpMockMessage {
    pub data: TArray<u8>,
}
pub struct UUdpMessagingSettings {
    pub enabled_by_default: bool,
    pub enable_transport: bool,
    pub b_auto_repair: bool,
    pub max_send_rate: f32,
    pub auto_repair_attempt_limit: u32,
    pub work_queue_size: u16,
    pub reliable_queue_priority: f32,
    pub b_stop_service_when_app_deactivates: bool,
    pub unicast_endpoint: FString,
    pub multicast_endpoint: FString,
    pub message_format: EUdpMessageFormat,
    pub multicast_time_to_live: u8,
    pub static_endpoints: TArray<FString>,
    pub excluded_endpoints: TArray<FString>,
    pub b_share_known_nodes_with_active_connections: bool,
    pub max_concurrent_deserialization_tasks: u32,
    pub connection_timeout_period: f32,
    pub enable_tunnel: bool,
    pub tunnel_unicast_endpoint: FString,
    pub tunnel_multicast_endpoint: FString,
    pub remote_tunnel_endpoints: TArray<FString>,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EUdpMessageFormat(pub u8);
impl EUdpMessageFormat {
    pub const NONE: EUdpMessageFormat = EUdpMessageFormat(0);
    pub const JSON: EUdpMessageFormat = EUdpMessageFormat(1);
    pub const TAGGED_PROPERTY: EUdpMessageFormat = EUdpMessageFormat(2);
    pub const CBOR_PLATFORM_ENDIANNESS: EUdpMessageFormat = EUdpMessageFormat(3);
    pub const CBOR_STANDARD_ENDIANNESS: EUdpMessageFormat = EUdpMessageFormat(4);
}
