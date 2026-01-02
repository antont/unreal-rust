#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UReplicationFilteringConfig {
    __padding_end: [u8; 72],
}
impl UReplicationFilteringConfig {}
#[repr(C, align(8))]
pub struct UDataStream {
    __padding_end: [u8; 88],
}
impl UDataStream {}
#[repr(C, align(8))]
pub struct UDataStreamDefinitions {
    __padding_end: [u8; 72],
}
impl UDataStreamDefinitions {}
#[repr(C, align(8))]
pub struct UDataStreamManager {
    __padding_end: [u8; 96],
}
impl UDataStreamManager {}
#[repr(C, align(8))]
pub struct UReplicationStateDescriptorConfig {
    __padding_end: [u8; 88],
}
impl UReplicationStateDescriptorConfig {}
#[repr(C, align(8))]
pub struct UChunkedDataStream {
    __padding_end: [u8; 112],
}
impl UChunkedDataStream {}
#[repr(C, align(8))]
pub struct UNetObjectFilterConfig {
    __padding_end: [u8; 48],
}
impl UNetObjectFilterConfig {}
#[repr(C, align(8))]
pub struct UAlwaysRelevantNetObjectFilterConfig {
    __padding_end: [u8; 48],
}
impl UAlwaysRelevantNetObjectFilterConfig {}
#[repr(C, align(8))]
pub struct UNetObjectFilter {
    __padding_end: [u8; 104],
}
impl UNetObjectFilter {}
#[repr(C, align(8))]
pub struct UAlwaysRelevantNetObjectFilter {
    __padding_end: [u8; 104],
}
impl UAlwaysRelevantNetObjectFilter {}
#[repr(C, align(8))]
pub struct UFilterOutNetObjectFilterConfig {
    __padding_end: [u8; 48],
}
impl UFilterOutNetObjectFilterConfig {}
#[repr(C, align(8))]
pub struct UFilterOutNetObjectFilter {
    __padding_end: [u8; 104],
}
impl UFilterOutNetObjectFilter {}
#[repr(C, align(8))]
pub struct UNetObjectFilterDefinitions {
    __padding_end: [u8; 64],
}
impl UNetObjectFilterDefinitions {}
#[repr(C, align(8))]
pub struct UNetObjectGridFilterConfig {
    __padding_end: [u8; 88],
}
impl UNetObjectGridFilterConfig {}
#[repr(C, align(8))]
pub struct UNetObjectGridFilter {
    __padding_end: [u8; 416],
}
impl UNetObjectGridFilter {}
#[repr(C, align(8))]
pub struct UNetObjectGridWorldLocFilter {
    __padding_end: [u8; 424],
}
impl UNetObjectGridWorldLocFilter {}
#[repr(C, align(8))]
pub struct UNetBlobHandler {
    __padding_end: [u8; 64],
}
impl UNetBlobHandler {}
#[repr(C, align(8))]
pub struct UNetBlobHandlerDefinitions {
    __padding_end: [u8; 64],
}
impl UNetBlobHandlerDefinitions {}
#[repr(C, align(8))]
pub struct UNetObjectBlobHandler {
    __padding_end: [u8; 64],
}
impl UNetObjectBlobHandler {}
#[repr(C, align(8))]
pub struct UNetRPCHandler {
    __padding_end: [u8; 72],
}
impl UNetRPCHandler {}
#[repr(C, align(8))]
pub struct USequentialPartialNetBlobHandlerConfig {
    __padding_end: [u8; 56],
}
impl USequentialPartialNetBlobHandlerConfig {}
#[repr(C, align(8))]
pub struct UPartialNetObjectAttachmentHandlerConfig {
    __padding_end: [u8; 72],
}
impl UPartialNetObjectAttachmentHandlerConfig {}
#[repr(C, align(8))]
pub struct USequentialPartialNetBlobHandler {
    __padding_end: [u8; 80],
}
impl USequentialPartialNetBlobHandler {}
#[repr(C, align(8))]
pub struct UPartialNetObjectAttachmentHandler {
    __padding_end: [u8; 344],
}
impl UPartialNetObjectAttachmentHandler {}
#[repr(C, align(8))]
pub struct UNetObjectFactory {
    __padding_end: [u8; 64],
}
impl UNetObjectFactory {}
#[repr(C, align(8))]
pub struct UNetTokenDataStream {
    __padding_end: [u8; 168],
}
impl UNetTokenDataStream {}
#[repr(C, align(8))]
pub struct UNetTokenTypeIdConfig {
    __padding_end: [u8; 64],
}
impl UNetTokenTypeIdConfig {}
#[repr(C, align(8))]
pub struct UReplicationBridge {
    __padding_end: [u8; 280],
}
impl UReplicationBridge {}
#[repr(C, align(16))]
pub struct UObjectReplicationBridge {
    __padding_end: [u8; 1712],
}
impl UObjectReplicationBridge {}
#[repr(C, align(8))]
pub struct UObjectReplicationBridgeConfig {
    __padding_end: [u8; 216],
}
impl UObjectReplicationBridgeConfig {}
#[repr(C, align(8))]
pub struct UNetObjectPrioritizerConfig {
    __padding_end: [u8; 48],
}
impl UNetObjectPrioritizerConfig {}
#[repr(C, align(8))]
pub struct UFieldOfViewNetObjectPrioritizerConfig {
    __padding_end: [u8; 96],
}
impl UFieldOfViewNetObjectPrioritizerConfig {}
#[repr(C, align(8))]
pub struct UNetObjectPrioritizer {
    __padding_end: [u8; 48],
}
impl UNetObjectPrioritizer {}
#[repr(C, align(8))]
pub struct ULocationBasedNetObjectPrioritizer {
    __padding_end: [u8; 104],
}
impl ULocationBasedNetObjectPrioritizer {}
#[repr(C, align(8))]
pub struct UFieldOfViewNetObjectPrioritizer {
    __padding_end: [u8; 112],
}
impl UFieldOfViewNetObjectPrioritizer {}
#[repr(C, align(8))]
pub struct UNetObjectConnectionFilterConfig {
    __padding_end: [u8; 56],
}
impl UNetObjectConnectionFilterConfig {}
#[repr(C, align(8))]
pub struct UNetObjectConnectionFilter {
    __padding_end: [u8; 176],
}
impl UNetObjectConnectionFilter {}
#[repr(C, align(8))]
pub struct UNetObjectCountLimiterConfig {
    __padding_end: [u8; 72],
}
impl UNetObjectCountLimiterConfig {}
#[repr(C, align(8))]
pub struct UNetObjectCountLimiter {
    __padding_end: [u8; 152],
}
impl UNetObjectCountLimiter {}
#[repr(C, align(8))]
pub struct UNetObjectPrioritizerDefinitions {
    __padding_end: [u8; 64],
}
impl UNetObjectPrioritizerDefinitions {}
#[repr(C, align(8))]
pub struct USphereNetObjectPrioritizerConfig {
    __padding_end: [u8; 72],
}
impl USphereNetObjectPrioritizerConfig {}
#[repr(C, align(8))]
pub struct USphereNetObjectPrioritizer {
    __padding_end: [u8; 112],
}
impl USphereNetObjectPrioritizer {}
#[repr(C, align(8))]
pub struct USphereWithOwnerBoostNetObjectPrioritizerConfig {
    __padding_end: [u8; 80],
}
impl USphereWithOwnerBoostNetObjectPrioritizerConfig {}
#[repr(C, align(8))]
pub struct USphereWithOwnerBoostNetObjectPrioritizer {
    __padding_end: [u8; 168],
}
impl USphereWithOwnerBoostNetObjectPrioritizer {}
#[repr(C, align(8))]
pub struct UReplicationDataStream {
    __padding_end: [u8; 104],
}
impl UReplicationDataStream {}
#[repr(C, align(8))]
pub struct UReplicationSystem {
    __padding_end: [u8; 96],
}
impl UReplicationSystem {}
#[repr(C, align(8))]
pub struct UWorldLocationsConfig {
    __padding_end: [u8; 104],
}
impl UWorldLocationsConfig {}
#[repr(C, align(8))]
pub struct UIrisObjectReferencePackageMap {
    __padding_end: [u8; 264],
}
impl UIrisObjectReferencePackageMap {}
#[repr(transparent)]
pub struct EDataStreamSendStatus(pub u8);
impl EDataStreamSendStatus {
    pub const PAUSE: EDataStreamSendStatus = EDataStreamSendStatus(0);
    pub const SEND: EDataStreamSendStatus = EDataStreamSendStatus(1);
}
#[repr(transparent)]
pub struct EIrisAsyncLoadingPriority(pub u8);
impl EIrisAsyncLoadingPriority {
    pub const DEFAULT: EIrisAsyncLoadingPriority = EIrisAsyncLoadingPriority(0);
    pub const HIGH: EIrisAsyncLoadingPriority = EIrisAsyncLoadingPriority(1);
    pub const VERY_HIGH: EIrisAsyncLoadingPriority = EIrisAsyncLoadingPriority(2);
    pub const MAX: EIrisAsyncLoadingPriority = EIrisAsyncLoadingPriority(2);
    pub const INVALID: EIrisAsyncLoadingPriority = EIrisAsyncLoadingPriority(3);
}
#[repr(transparent)]
pub struct ENetObjectCountLimiterMode(pub u32);
impl ENetObjectCountLimiterMode {
    pub const ROUND_ROBIN: ENetObjectCountLimiterMode = ENetObjectCountLimiterMode(0);
    pub const FILL: ENetObjectCountLimiterMode = ENetObjectCountLimiterMode(1);
}
