#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FNetSerializerConfig {}
#[repr(C, align(4))]
pub struct FObjectScopeHysteresisProfile {
    pub filter_profile_name: FName,
    pub hysteresis_frame_count: u8,
}
#[repr(C, align(8))]
pub struct FDataStreamDefinition {
    pub data_stream_name: FName,
    pub class_name: FName,
    pub class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub default_send_status: EDataStreamSendStatus,
    pub b_auto_create: bool,
    pub b_dynamic_create: bool,
}
#[repr(C, align(8))]
pub struct FIrisFastArraySerializer {
    pub change_mask_storage: u32,
}
#[repr(C, align(4))]
pub struct FReplicationStateDescriptorClassPushModelConfig {
    pub class_name: FName,
}
#[repr(C, align(4))]
pub struct FSupportsStructNetSerializerConfig {
    pub struct_name: FName,
    pub b_can_use_struct_net_serializer: bool,
}
#[repr(C, align(4))]
pub struct FNetObjectFilterDefinition {
    pub filter_name: FName,
    pub class_name: FName,
    pub config_class_name: FName,
}
#[repr(C, align(4))]
pub struct FNetObjectGridFilterProfile {
    pub filter_profile_name: FName,
    pub frame_count_before_culling: u16,
}
#[repr(C, align(4))]
pub struct FNetBlobHandlerDefinition {
    pub class_name: FName,
}
#[repr(C, align(8))]
pub struct FNetTokenStoreTypeIdPair {
    pub store_type_name: FString,
    pub type_id: u32,
}
#[repr(C, align(4))]
pub struct FObjectReplicationBridgePollConfig {
    pub class_name: FName,
    pub poll_frequency: f32,
    pub b_include_subclasses: bool,
}
#[repr(C, align(4))]
pub struct FObjectReplicationBridgeFilterConfig {
    pub class_name: FName,
    pub dynamic_filter_name: FName,
    pub filter_profile: FName,
    pub b_force_enable_on_all_instances: bool,
}
#[repr(C, align(4))]
pub struct FObjectReplicationBridgePrioritizerConfig {
    pub class_name: FName,
    pub prioritizer_name: FName,
    pub b_force_enable_on_all_instances: bool,
}
#[repr(C, align(4))]
pub struct FObjectReplicationBridgeDeltaCompressionConfig {
    pub class_name: FName,
    pub b_enable_delta_compression: bool,
}
#[repr(C, align(4))]
pub struct FObjectReplicatedBridgeCriticalClassConfig {
    pub class_name: FName,
    pub b_disconnect_on_protocol_mismatch: bool,
}
#[repr(C, align(4))]
pub struct FObjectReplicationBridgeAsyncLoadingClassConfig {
    pub class_name: FName,
    pub iris_async_loading_priority: EIrisAsyncLoadingPriority,
}
#[repr(C, align(4))]
pub struct FObjectReplicationBridgeTypeStatsConfig {
    pub class_name: FName,
    pub type_stats_name: FName,
    pub b_include_in_minimal_csv_stats: bool,
}
#[repr(C, align(8))]
pub struct FNetObjectPrioritizerDefinition {
    pub prioritizer_name: FName,
    pub class_name: FName,
    pub class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub config_class_name: FName,
    pub config_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FDateTimeNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FEnumInt8NetSerializerConfig {
    pub lower_bound: i8,
    pub upper_bound: i8,
    pub bit_count: u8,
}
#[repr(C, align(8))]
pub struct FEnumInt16NetSerializerConfig {
    pub lower_bound: i16,
    pub upper_bound: i16,
    pub bit_count: u8,
}
#[repr(C, align(8))]
pub struct FEnumInt32NetSerializerConfig {
    pub lower_bound: i32,
    pub upper_bound: i32,
    pub bit_count: u8,
}
#[repr(C, align(8))]
pub struct FEnumInt64NetSerializerConfig {
    pub lower_bound: i64,
    pub upper_bound: i64,
    pub bit_count: u8,
}
#[repr(C, align(8))]
pub struct FEnumUint8NetSerializerConfig {
    pub lower_bound: u8,
    pub upper_bound: u8,
    pub bit_count: u8,
}
#[repr(C, align(8))]
pub struct FEnumUint16NetSerializerConfig {
    pub lower_bound: u16,
    pub upper_bound: u16,
    pub bit_count: u8,
}
#[repr(C, align(8))]
pub struct FEnumUint32NetSerializerConfig {
    pub lower_bound: u32,
    pub upper_bound: u32,
    pub bit_count: u8,
}
#[repr(C, align(8))]
pub struct FEnumUint64NetSerializerConfig {
    pub lower_bound: u64,
    pub upper_bound: u64,
    pub bit_count: u8,
}
#[repr(C, align(8))]
pub struct FFloatNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FDoubleNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FGuidNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FInstancedStructNetSerializerConfig {
    pub supported_types: TArray<
        TSoftObjectPtr<crate::bindings::core_u_object::UScriptStruct>,
    >,
}
#[repr(C, align(8))]
pub struct FBitfieldNetSerializerConfig {
    pub bit_mask: u8,
}
#[repr(C, align(8))]
pub struct FArrayPropertyNetSerializerConfig {
    pub max_element_count: u16,
    pub element_count_bit_count: u16,
    pub property: TFieldPath<FArrayProperty>,
}
#[repr(C, align(8))]
pub struct FLastResortPropertyNetSerializerConfig {
    pub property: TFieldPath<FProperty>,
    pub b_exclude_from_default_state_hash: bool,
    pub max_quantized_size_bits: u32,
}
#[repr(C, align(8))]
pub struct FNetRoleNetSerializerConfig {
    pub relative_internal_offset_to_other_role: i32,
    pub relative_external_offset_to_other_role: i32,
    pub lower_bound: u8,
    pub upper_bound: u8,
    pub bit_count: u8,
    pub autonomous_proxy_value: u8,
    pub simulated_proxy_value: u8,
}
#[repr(C, align(8))]
pub struct FFieldPathNetSerializerConfig {
    pub property: TFieldPath<FProperty>,
}
#[repr(C, align(8))]
pub struct FFieldPathNetSerializerSerializationHelper {
    pub owner: TWeakObjectPtr<crate::bindings::core_u_object::UStruct>,
    pub property_path: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FIntNetSerializerConfig {
    pub bit_count: u8,
}
#[repr(C, align(8))]
pub struct FInt8RangeNetSerializerConfig {
    pub lower_bound: i8,
    pub upper_bound: i8,
    pub bit_count: u8,
}
#[repr(C, align(8))]
pub struct FInt16RangeNetSerializerConfig {
    pub lower_bound: i16,
    pub upper_bound: i16,
    pub bit_count: u8,
}
#[repr(C, align(8))]
pub struct FInt32RangeNetSerializerConfig {
    pub lower_bound: i32,
    pub upper_bound: i32,
    pub bit_count: u8,
}
#[repr(C, align(8))]
pub struct FInt64RangeNetSerializerConfig {
    pub lower_bound: i64,
    pub upper_bound: i64,
    pub bit_count: u8,
}
#[repr(C, align(8))]
pub struct FBoolNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FStructNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FNopNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FObjectNetSerializerConfig {
    pub property_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FObjectPtrNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FWeakObjectNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FScriptInterfaceNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FPackedInt64NetSerializerConfig {}
#[repr(C, align(8))]
pub struct FPackedUint64NetSerializerConfig {}
#[repr(C, align(8))]
pub struct FPackedInt32NetSerializerConfig {}
#[repr(C, align(8))]
pub struct FPackedUint32NetSerializerConfig {}
#[repr(C, align(8))]
pub struct FVectorNetQuantizeNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FVectorNetQuantize10NetSerializerConfig {}
#[repr(C, align(8))]
pub struct FVectorNetQuantize100NetSerializerConfig {}
#[repr(C, align(8))]
pub struct FVectorNetQuantizeNormalNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FPolymorphicStructNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FPolymorphicArrayStructNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FUnitQuatNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FUnitQuat4fNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FUnitQuat4dNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FRemoteObjectIdNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FRemoteObjectReferenceNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FRemoteServerIdNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FRotatorNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FRotatorAsByteNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FRotatorAsShortNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FRotator3fNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FRotator3dNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FSoftObjectNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FSoftObjectPathNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FSoftClassPathNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FNameNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FNameAsNetTokenNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FStringNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FUintNetSerializerConfig {
    pub bit_count: u8,
}
#[repr(C, align(8))]
pub struct FUint8RangeNetSerializerConfig {
    pub lower_bound: u8,
    pub upper_bound: u8,
    pub bit_count: u8,
}
#[repr(C, align(8))]
pub struct FUint16RangeNetSerializerConfig {
    pub lower_bound: u16,
    pub upper_bound: u16,
    pub bit_count: u8,
}
#[repr(C, align(8))]
pub struct FUint32RangeNetSerializerConfig {
    pub lower_bound: u32,
    pub upper_bound: u32,
    pub bit_count: u8,
}
#[repr(C, align(8))]
pub struct FUint64RangeNetSerializerConfig {
    pub lower_bound: u64,
    pub upper_bound: u64,
    pub bit_count: u8,
}
#[repr(C, align(8))]
pub struct FVectorNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FVector3fNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FVector3dNetSerializerConfig {}
pub struct UReplicationFilteringConfig {
    pub b_enable_object_scope_hysteresis: bool,
    pub default_hysteresis_frame_count: u8,
    pub hysteresis_update_connection_throttling: u8,
    pub hysteresis_profiles: TArray<FObjectScopeHysteresisProfile>,
}
pub struct UDataStream {}
pub struct UDataStreamDefinitions {
    pub data_stream_definitions: TArray<FDataStreamDefinition>,
}
pub struct UDataStreamManager {}
pub struct UReplicationStateDescriptorConfig {
    pub supports_struct_net_serializer_list: TArray<FSupportsStructNetSerializerConfig>,
    pub ensure_fully_push_model_class_names: TArray<
        FReplicationStateDescriptorClassPushModelConfig,
    >,
    pub b_ensure_all_classes_are_fully_push_model: bool,
}
pub struct UChunkedDataStream {
    pub package_map: UPtr<UIrisObjectReferencePackageMap>,
}
pub struct UNetObjectFilterConfig {}
pub struct UAlwaysRelevantNetObjectFilterConfig {}
pub struct UNetObjectFilter {}
pub struct UAlwaysRelevantNetObjectFilter {}
pub struct UFilterOutNetObjectFilterConfig {}
pub struct UFilterOutNetObjectFilter {}
pub struct UNetObjectFilterDefinitions {
    pub net_object_filter_definitions: TArray<FNetObjectFilterDefinition>,
}
pub struct UNetObjectGridFilterConfig {
    pub view_pos_relevancy_frame_count: u32,
    pub default_frame_count_before_culling: u16,
    pub cell_size_x: f32,
    pub cell_size_y: f32,
    pub default_cull_distance: f32,
    pub b_use_exact_cull_distance: bool,
    pub filter_profiles: TArray<FNetObjectGridFilterProfile>,
}
pub struct UNetObjectGridFilter {}
pub struct UNetObjectGridWorldLocFilter {}
pub struct UNetBlobHandler {}
pub struct UNetBlobHandlerDefinitions {
    pub net_blob_handler_definitions: TArray<FNetBlobHandlerDefinition>,
}
pub struct UNetObjectBlobHandler {}
pub struct UNetRPCHandler {}
pub struct USequentialPartialNetBlobHandlerConfig {
    pub max_part_bit_count: u32,
    pub max_part_count: u32,
}
pub struct UPartialNetObjectAttachmentHandlerConfig {
    pub bit_count_split_threshold: u32,
    pub client_unreliable_bit_count_split_threshold: u32,
    pub server_unreliable_bit_count_split_threshold: u32,
}
pub struct USequentialPartialNetBlobHandler {}
pub struct UPartialNetObjectAttachmentHandler {}
pub struct UNetObjectFactory {}
pub struct UNetTokenDataStream {}
pub struct UNetTokenTypeIdConfig {
    pub reserved_type_ids: TArray<FNetTokenStoreTypeIdPair>,
}
pub struct UReplicationBridge {}
pub struct UObjectReplicationBridge {
    pub net_object_factories: TArray<UPtr<UNetObjectFactory>>,
}
pub struct UObjectReplicationBridgeConfig {
    pub poll_configs: TArray<FObjectReplicationBridgePollConfig>,
    pub filter_configs: TArray<FObjectReplicationBridgeFilterConfig>,
    pub prioritizer_configs: TArray<FObjectReplicationBridgePrioritizerConfig>,
    pub delta_compression_configs: TArray<
        FObjectReplicationBridgeDeltaCompressionConfig,
    >,
    pub critical_class_configs: TArray<FObjectReplicatedBridgeCriticalClassConfig>,
    pub b_all_classes_critical: bool,
    pub type_stats_configs: TArray<FObjectReplicationBridgeTypeStatsConfig>,
    pub default_spatial_filter_name: FName,
    pub async_loading_class_configs: TArray<
        FObjectReplicationBridgeAsyncLoadingClassConfig,
    >,
    pub required_net_driver_channel_class_name: FName,
    pub critical_actor_classes: TArray<FName>,
}
pub struct UNetObjectPrioritizerConfig {}
pub struct UFieldOfViewNetObjectPrioritizerConfig {
    pub inner_sphere_radius: f32,
    pub inner_sphere_priority: f32,
    pub outer_sphere_radius: f32,
    pub outer_sphere_priority: f32,
    pub cone_field_of_view_degrees: f32,
    pub inner_cone_length: f32,
    pub cone_length: f32,
    pub min_cone_priority: f32,
    pub max_cone_priority: f32,
    pub line_of_sight_width: f32,
    pub line_of_sight_priority: f32,
    pub outside_priority: f32,
}
pub struct UNetObjectPrioritizer {}
pub struct ULocationBasedNetObjectPrioritizer {}
pub struct UFieldOfViewNetObjectPrioritizer {}
pub struct UNetObjectConnectionFilterConfig {
    pub max_object_count: u16,
}
pub struct UNetObjectConnectionFilter {}
pub struct UNetObjectCountLimiterConfig {
    pub mode: ENetObjectCountLimiterMode,
    pub max_object_count: u32,
    pub priority: f32,
    pub owning_connection_priority: f32,
    pub b_enable_owned_objects_fast_lane: bool,
}
pub struct UNetObjectCountLimiter {}
pub struct UNetObjectPrioritizerDefinitions {
    pub net_object_prioritizer_definitions: TArray<FNetObjectPrioritizerDefinition>,
}
pub struct USphereNetObjectPrioritizerConfig {
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub inner_priority: f32,
    pub outer_priority: f32,
    pub outside_priority: f32,
}
pub struct USphereNetObjectPrioritizer {}
pub struct USphereWithOwnerBoostNetObjectPrioritizerConfig {
    pub owner_priority_boost: f32,
}
pub struct USphereWithOwnerBoostNetObjectPrioritizer {}
pub struct UReplicationDataStream {}
pub struct UReplicationSystem {
    pub replication_bridge: UPtr<UObjectReplicationBridge>,
}
pub struct UWorldLocationsConfig {
    pub min_pos: crate::bindings::core_u_object::FVector,
    pub max_pos: crate::bindings::core_u_object::FVector,
    pub max_net_cull_distance: f32,
}
pub struct UIrisObjectReferencePackageMap {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataStreamSendStatus(pub u8);
impl EDataStreamSendStatus {
    pub const PAUSE: EDataStreamSendStatus = EDataStreamSendStatus(0);
    pub const SEND: EDataStreamSendStatus = EDataStreamSendStatus(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EIrisAsyncLoadingPriority(pub u8);
impl EIrisAsyncLoadingPriority {
    pub const DEFAULT: EIrisAsyncLoadingPriority = EIrisAsyncLoadingPriority(0);
    pub const HIGH: EIrisAsyncLoadingPriority = EIrisAsyncLoadingPriority(1);
    pub const VERY_HIGH: EIrisAsyncLoadingPriority = EIrisAsyncLoadingPriority(2);
    pub const MAX: EIrisAsyncLoadingPriority = EIrisAsyncLoadingPriority(2);
    pub const INVALID: EIrisAsyncLoadingPriority = EIrisAsyncLoadingPriority(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENetObjectCountLimiterMode(pub u32);
impl ENetObjectCountLimiterMode {
    pub const ROUND_ROBIN: ENetObjectCountLimiterMode = ENetObjectCountLimiterMode(0);
    pub const FILL: ENetObjectCountLimiterMode = ENetObjectCountLimiterMode(1);
}
