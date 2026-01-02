#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UNetAnalyticsAggregatorConfig {
    __padding_end: [u8; 64],
}
impl UNetAnalyticsAggregatorConfig {}
#[repr(C, align(8))]
pub struct UStatePerObjectConfig {
    __padding_end: [u8; 128],
}
impl UStatePerObjectConfig {}
#[repr(C, align(8))]
pub struct UEscalationManagerConfig {
    __padding_end: [u8; 160],
}
impl UEscalationManagerConfig {}
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
#[repr(transparent)]
pub struct EReplicationSystem(pub u8);
impl EReplicationSystem {
    pub const DEFAULT: EReplicationSystem = EReplicationSystem(0);
    pub const GENERIC: EReplicationSystem = EReplicationSystem(1);
    pub const IRIS: EReplicationSystem = EReplicationSystem(2);
}
