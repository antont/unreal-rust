#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAutomationEvent {
    pub ty: EAutomationEventType,
    pub message: FString,
    pub context: FString,
    pub artifact: FGuid,
}
#[repr(C, align(4))]
pub struct FGuid {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
}
#[repr(C, align(8))]
pub struct FAutomationExecutionEntry {
    pub event: FAutomationEvent,
    pub filename: FString,
    pub line_number: i32,
    pub timestamp: FDateTime,
}
#[repr(C, align(8))]
pub struct FDateTime {
    pub ticks: i64,
}
#[repr(C, align(8))]
pub struct FBox {
    pub min: FVector,
    pub max: FVector,
    pub is_valid: bool,
}
#[repr(C, align(8))]
pub struct FVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[repr(C, align(8))]
pub struct FBox2D {
    pub min: FVector2D,
    pub max: FVector2D,
    pub b_is_valid: bool,
}
#[repr(C, align(8))]
pub struct FVector2D {
    pub x: f64,
    pub y: f64,
}
#[repr(C, align(4))]
pub struct FBox2f {
    pub min: FVector2f,
    pub max: FVector2f,
    pub b_is_valid: bool,
}
#[repr(C, align(4))]
pub struct FVector2f {
    pub x: f32,
    pub y: f32,
}
#[repr(C, align(8))]
pub struct FBox3d {
    pub min: FVector3d,
    pub max: FVector3d,
    pub is_valid: bool,
}
#[repr(C, align(8))]
pub struct FVector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[repr(C, align(4))]
pub struct FBox3f {
    pub min: FVector3f,
    pub max: FVector3f,
    pub is_valid: bool,
}
#[repr(C, align(4))]
pub struct FVector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C, align(8))]
pub struct FBoxSphereBounds {
    pub origin: FVector,
    pub box_extent: FVector,
    pub sphere_radius: f64,
}
#[repr(C, align(8))]
pub struct FBoxSphereBounds3d {
    pub origin: FVector3d,
    pub box_extent: FVector3d,
    pub sphere_radius: f64,
}
#[repr(C, align(4))]
pub struct FBoxSphereBounds3f {
    pub origin: FVector3f,
    pub box_extent: FVector3f,
    pub sphere_radius: f32,
}
#[repr(C, align(4))]
pub struct FColor {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}
#[repr(C, align(8))]
pub struct FDoubleRange {
    pub lower_bound: FDoubleRangeBound,
    pub upper_bound: FDoubleRangeBound,
}
#[repr(C, align(8))]
pub struct FDoubleRangeBound {
    pub ty: ERangeBoundTypes,
    pub value: f64,
}
#[repr(C, align(1))]
pub struct FFallbackStruct {}
#[repr(C, align(4))]
pub struct FFloatInterval {
    pub min: f32,
    pub max: f32,
}
#[repr(C, align(4))]
pub struct FFloatRange {
    pub lower_bound: FFloatRangeBound,
    pub upper_bound: FFloatRangeBound,
}
#[repr(C, align(4))]
pub struct FFloatRangeBound {
    pub ty: ERangeBoundTypes,
    pub value: f32,
}
#[repr(C, align(4))]
pub struct FFrameNumber {
    pub value: i32,
}
#[repr(C, align(4))]
pub struct FFrameNumberRange {
    pub lower_bound: FFrameNumberRangeBound,
    pub upper_bound: FFrameNumberRangeBound,
}
#[repr(C, align(4))]
pub struct FFrameNumberRangeBound {
    pub ty: ERangeBoundTypes,
    pub value: FFrameNumber,
}
#[repr(C, align(4))]
pub struct FFrameRate {
    pub numerator: i32,
    pub denominator: i32,
}
#[repr(C, align(4))]
pub struct FFrameTime {
    pub frame_number: FFrameNumber,
    pub sub_frame: f32,
}
#[repr(C, align(4))]
pub struct FInputDeviceId {
    pub internal_id: i32,
}
#[repr(C, align(4))]
pub struct FInt32Interval {
    pub min: i32,
    pub max: i32,
}
#[repr(C, align(4))]
pub struct FInt32Point {
    pub x: i32,
    pub y: i32,
}
#[repr(C, align(4))]
pub struct FInt32Range {
    pub lower_bound: FInt32RangeBound,
    pub upper_bound: FInt32RangeBound,
}
#[repr(C, align(4))]
pub struct FInt32RangeBound {
    pub ty: ERangeBoundTypes,
    pub value: i32,
}
#[repr(C, align(4))]
pub struct FInt32Rect {
    pub min: FInt32Point,
    pub max: FInt32Point,
}
#[repr(C, align(4))]
pub struct FInt32Vector {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
#[repr(C, align(4))]
pub struct FInt32Vector2 {
    pub x: i32,
    pub y: i32,
}
#[repr(C, align(4))]
pub struct FInt32Vector4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}
#[repr(C, align(8))]
pub struct FInt64Point {
    pub x: i64,
    pub y: i64,
}
#[repr(C, align(8))]
pub struct FInt64Rect {
    pub min: FInt64Point,
    pub max: FInt64Point,
}
#[repr(C, align(8))]
pub struct FInt64Vector {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}
#[repr(C, align(8))]
pub struct FInt64Vector2 {
    pub x: i64,
    pub y: i64,
}
#[repr(C, align(8))]
pub struct FInt64Vector4 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub w: i64,
}
#[repr(C, align(8))]
pub struct FInterpCurveFloat {
    pub points: TArray<FInterpCurvePointFloat>,
    pub b_is_looped: bool,
    pub loop_key_offset: f32,
}
#[repr(C, align(4))]
pub struct FInterpCurvePointFloat {
    pub in_val: f32,
    pub out_val: f32,
    pub arrive_tangent: f32,
    pub leave_tangent: f32,
    pub interp_mode: EInterpCurveMode,
}
#[repr(C, align(8))]
pub struct FInterpCurveLinearColor {
    pub points: TArray<FInterpCurvePointLinearColor>,
    pub b_is_looped: bool,
    pub loop_key_offset: f32,
}
#[repr(C, align(4))]
pub struct FInterpCurvePointLinearColor {
    pub in_val: f32,
    pub out_val: FLinearColor,
    pub arrive_tangent: FLinearColor,
    pub leave_tangent: FLinearColor,
    pub interp_mode: EInterpCurveMode,
}
#[repr(C, align(4))]
pub struct FLinearColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[repr(C, align(16))]
pub struct FInterpCurvePointQuat {
    pub in_val: f32,
    pub out_val: FQuat,
    pub arrive_tangent: FQuat,
    pub leave_tangent: FQuat,
    pub interp_mode: EInterpCurveMode,
}
#[repr(C, align(16))]
pub struct FQuat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
#[repr(C, align(8))]
pub struct FInterpCurvePointTwoVectors {
    pub in_val: f32,
    pub out_val: FTwoVectors,
    pub arrive_tangent: FTwoVectors,
    pub leave_tangent: FTwoVectors,
    pub interp_mode: EInterpCurveMode,
}
#[repr(C, align(8))]
pub struct FTwoVectors {
    pub v1: FVector,
    pub v2: FVector,
}
#[repr(C, align(8))]
pub struct FInterpCurvePointVector {
    pub in_val: f32,
    pub out_val: FVector,
    pub arrive_tangent: FVector,
    pub leave_tangent: FVector,
    pub interp_mode: EInterpCurveMode,
}
#[repr(C, align(8))]
pub struct FInterpCurvePointVector2D {
    pub in_val: f32,
    pub out_val: FVector2D,
    pub arrive_tangent: FVector2D,
    pub leave_tangent: FVector2D,
    pub interp_mode: EInterpCurveMode,
}
#[repr(C, align(8))]
pub struct FInterpCurveQuat {
    pub points: TArray<FInterpCurvePointQuat>,
    pub b_is_looped: bool,
    pub loop_key_offset: f32,
}
#[repr(C, align(8))]
pub struct FInterpCurveTwoVectors {
    pub points: TArray<FInterpCurvePointTwoVectors>,
    pub b_is_looped: bool,
    pub loop_key_offset: f32,
}
#[repr(C, align(8))]
pub struct FInterpCurveVector {
    pub points: TArray<FInterpCurvePointVector>,
    pub b_is_looped: bool,
    pub loop_key_offset: f32,
}
#[repr(C, align(8))]
pub struct FInterpCurveVector2D {
    pub points: TArray<FInterpCurvePointVector2D>,
    pub b_is_looped: bool,
    pub loop_key_offset: f32,
}
#[repr(C, align(4))]
pub struct FIntPoint {
    pub x: i32,
    pub y: i32,
}
#[repr(C, align(4))]
pub struct FIntRect {
    pub min: FIntPoint,
    pub max: FIntPoint,
}
#[repr(C, align(4))]
pub struct FIntVector {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
#[repr(C, align(4))]
pub struct FIntVector2 {
    pub x: i32,
    pub y: i32,
}
#[repr(C, align(4))]
pub struct FIntVector4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}
#[repr(C, align(16))]
pub struct FMatrix {
    pub x_plane: FPlane,
    pub y_plane: FPlane,
    pub z_plane: FPlane,
    pub w_plane: FPlane,
}
#[repr(C, align(16))]
pub struct FPlane {
    pub w: f64,
}
#[repr(C, align(16))]
pub struct FMatrix44d {
    pub x_plane: FPlane4d,
    pub y_plane: FPlane4d,
    pub z_plane: FPlane4d,
    pub w_plane: FPlane4d,
}
#[repr(C, align(16))]
pub struct FPlane4d {
    pub w: f64,
}
#[repr(C, align(16))]
pub struct FMatrix44f {
    pub x_plane: FPlane4f,
    pub y_plane: FPlane4f,
    pub z_plane: FPlane4f,
    pub w_plane: FPlane4f,
}
#[repr(C, align(16))]
pub struct FPlane4f {
    pub w: f32,
}
#[repr(C, align(4))]
pub struct FMusicalTime {
    pub bar: i32,
    pub tick_in_bar: i32,
    pub ticks_per_bar: i32,
    pub ticks_per_beat: i32,
}
#[repr(C, align(8))]
pub struct FOrientedBox {
    pub center: FVector,
    pub axis_x: FVector,
    pub axis_y: FVector,
    pub axis_z: FVector,
    pub extent_x: f64,
    pub extent_y: f64,
    pub extent_z: f64,
}
#[repr(C, align(1))]
pub struct FPackedNormal {
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub w: u8,
}
#[repr(C, align(8))]
pub struct FPackedRemoteObjectPathName {
    pub remote_ids: TArray<u16>,
    pub names: TArray<u16>,
}
#[repr(C, align(4))]
pub struct FPackedRGB10A2N {
    pub packed: i32,
}
#[repr(C, align(4))]
pub struct FPackedRGBA16N {
    pub xy: i32,
    pub zw: i32,
}
#[repr(C, align(4))]
pub struct FPlatformInputDeviceState {
    pub owning_platform_user: FPlatformUserId,
    pub connection_state: EInputDeviceConnectionState,
}
#[repr(C, align(4))]
pub struct FPlatformUserId {
    pub internal_id: i32,
}
#[repr(C, align(8))]
pub struct FPolyglotTextData {
    pub category: ELocalizedTextSourceCategory,
    pub native_culture: FString,
    pub namespace: FString,
    pub key: FString,
    pub native_string: FString,
    pub localized_strings: TMap<FString, FString>,
    pub b_is_minimal_patch: bool,
    pub cached_text: FText,
}
#[repr(C, align(4))]
pub struct FQualifiedFrameTime {
    pub time: FFrameTime,
    pub rate: FFrameRate,
}
#[repr(C, align(16))]
pub struct FQuat4d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
#[repr(C, align(16))]
pub struct FQuat4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[repr(C, align(4))]
pub struct FRandomStream {
    pub initial_seed: i32,
    pub seed: i32,
}
#[repr(C, align(8))]
pub struct FRay {
    pub origin: FVector,
    pub direction: FVector,
}
#[repr(C, align(8))]
pub struct FRay3d {
    pub origin: FVector3d,
    pub direction: FVector3d,
}
#[repr(C, align(4))]
pub struct FRay3f {
    pub origin: FVector3f,
    pub direction: FVector3f,
}
#[repr(C, align(8))]
pub struct FRemoteObjectBytes {
    pub bytes: TArray<u8>,
}
#[repr(C, align(8))]
pub struct FRemoteObjectData {
    pub tables: FRemoteObjectTables,
    pub path_names: TArray<FPackedRemoteObjectPathName>,
    pub bytes: TArray<FRemoteObjectBytes>,
}
#[repr(C, align(8))]
pub struct FRemoteObjectTables {
    pub names: TArray<FName>,
    pub remote_ids: TArray<FRemoteObjectId>,
}
#[repr(C, align(8))]
pub struct FRemoteObjectId {
    pub id: u64,
}
#[repr(C, align(8))]
pub struct FRemoteObjectPathName {}
#[repr(C, align(8))]
pub struct FRemoteObjectReference {
    pub object_id: FRemoteObjectId,
    pub server_id: FRemoteServerId,
}
#[repr(C, align(4))]
pub struct FRemoteServerId {
    pub id: u32,
}
#[repr(C, align(4))]
pub struct FRemoteTransactionId {
    pub id: u32,
}
#[repr(C, align(8))]
pub struct FRemoteWorkPriority {
    pub packed_data: u64,
}
#[repr(C, align(8))]
pub struct FRotator {
    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64,
}
#[repr(C, align(8))]
pub struct FRotator3d {
    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64,
}
#[repr(C, align(4))]
pub struct FRotator3f {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}
#[repr(C, align(8))]
pub struct FSphere {
    pub center: FVector,
    pub w: f64,
}
#[repr(C, align(8))]
pub struct FSphere3d {
    pub center: FVector3d,
    pub w: f64,
}
#[repr(C, align(4))]
pub struct FSphere3f {
    pub center: FVector3f,
    pub w: f32,
}
#[repr(C, align(4))]
pub struct FTimecode {
    pub hours: i32,
    pub minutes: i32,
    pub seconds: i32,
    pub frames: i32,
    pub subframe: f32,
    pub b_drop_frame_format: bool,
}
#[repr(C, align(8))]
pub struct FTimespan {
    pub ticks: i64,
}
#[repr(C, align(16))]
pub struct FTransform {
    pub rotation: FQuat,
    pub translation: FVector,
    pub scale3_d: FVector,
}
#[repr(C, align(16))]
pub struct FTransform3d {
    pub rotation: FQuat4d,
    pub translation: FVector3d,
    pub scale3_d: FVector3d,
}
#[repr(C, align(16))]
pub struct FTransform3f {
    pub rotation: FQuat4f,
    pub translation: FVector3f,
    pub scale3_d: FVector3f,
}
#[repr(C, align(4))]
pub struct FUint32Point {
    pub x: i32,
    pub y: i32,
}
#[repr(C, align(4))]
pub struct FUint32Rect {
    pub min: FUint32Point,
    pub max: FUint32Point,
}
#[repr(C, align(4))]
pub struct FUint32Vector {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
#[repr(C, align(4))]
pub struct FUint32Vector2 {
    pub x: u32,
    pub y: u32,
}
#[repr(C, align(4))]
pub struct FUint32Vector4 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
}
#[repr(C, align(8))]
pub struct FUint64Point {
    pub x: i64,
    pub y: i64,
}
#[repr(C, align(8))]
pub struct FUint64Rect {
    pub min: FUint64Point,
    pub max: FUint64Point,
}
#[repr(C, align(8))]
pub struct FUint64Vector {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}
#[repr(C, align(8))]
pub struct FUint64Vector2 {
    pub x: u64,
    pub y: u64,
}
#[repr(C, align(8))]
pub struct FUint64Vector4 {
    pub x: u64,
    pub y: u64,
    pub z: u64,
    pub w: u64,
}
#[repr(C, align(4))]
pub struct FUintPoint {
    pub x: i32,
    pub y: i32,
}
#[repr(C, align(4))]
pub struct FUintRect {
    pub min: FUintPoint,
    pub max: FUintPoint,
}
#[repr(C, align(4))]
pub struct FUintVector {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
#[repr(C, align(4))]
pub struct FUintVector2 {
    pub x: u32,
    pub y: u32,
}
#[repr(C, align(4))]
pub struct FUintVector4 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
}
#[repr(C, align(16))]
pub struct FVector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
#[repr(C, align(16))]
pub struct FVector4d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
#[repr(C, align(16))]
pub struct FVector4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[repr(C, align(4))]
pub struct FTopLevelAssetPath {
    pub package_name: FName,
    pub asset_name: FName,
}
#[repr(C, align(8))]
pub struct FSoftObjectPath {
    pub asset_path: FTopLevelAssetPath,
    pub sub_path_string: FUtf8String,
}
#[repr(C, align(8))]
pub struct FARFilter {
    pub package_names: TArray<FName>,
    pub package_paths: TArray<FName>,
    pub object_paths: TArray<FName>,
    pub soft_object_paths: TArray<FSoftObjectPath>,
    pub class_names: TArray<FName>,
    pub class_paths: TArray<FTopLevelAssetPath>,
    pub recursive_classes_exclusion_set: TSet<FName>,
    pub recursive_class_paths_exclusion_set: TSet<FTopLevelAssetPath>,
    pub b_recursive_paths: bool,
    pub b_recursive_classes: bool,
    pub b_include_only_on_disk_assets: bool,
}
#[repr(C, align(8))]
pub struct FSoftClassPath {}
#[repr(C, align(4))]
pub struct FPrimaryAssetId {
    pub primary_asset_type: FPrimaryAssetType,
    pub primary_asset_name: FName,
}
#[repr(C, align(4))]
pub struct FPrimaryAssetType {
    pub name: FName,
}
#[repr(C, align(8))]
pub struct FAssetData {
    pub object_path: FName,
    pub package_name: FName,
    pub package_path: FName,
    pub asset_name: FName,
    pub asset_class: FName,
    pub asset_class_path: FTopLevelAssetPath,
}
#[repr(C, align(8))]
pub struct FInstancedStructBaseStructQueryParams {}
#[repr(C, align(8))]
pub struct FPropertyTextFName {}
#[repr(C, align(8))]
pub struct FPropertyTextString {}
#[repr(C, align(8))]
pub struct FTestUndeclaredScriptStructObjectReferencesTest {
    pub strong_object_pointer: UPtr<UObject>,
    pub soft_object_pointer: TSoftObjectPtr<UObject>,
    pub soft_object_path: FSoftObjectPath,
    pub weak_object_pointer: TWeakObjectPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FTestUninitializedScriptStructMembersTest {
    pub uninitialized_object_reference: UPtr<UObject>,
    pub initialized_object_reference: UPtr<UObject>,
    pub unused_value: f32,
}
#[repr(C, align(8))]
pub struct FVerseRational {
    pub numerator: i64,
    pub denominator: i64,
}
#[repr(C, align(8))]
pub struct FVerseIntConstraints {
    pub clamp_min: TOptional<i64>,
    pub clamp_max: TOptional<i64>,
}
#[repr(C, align(8))]
pub struct FVerseDoubleConstraints {
    pub clamp_min: f64,
    pub clamp_max: f64,
}
#[repr(C, align(8))]
pub struct FAssetBundleEntry {
    pub bundle_name: FName,
    pub bundle_assets: TArray<FSoftObjectPath>,
    pub asset_paths: TArray<FTopLevelAssetPath>,
}
#[repr(C, align(8))]
pub struct FAssetBundleData {
    pub bundles: TArray<FAssetBundleEntry>,
}
#[repr(C, align(8))]
pub struct FInstancedStruct {}
#[repr(C, align(8))]
pub struct FInstancedStructContainer {}
#[repr(C, align(1))]
pub struct FPropertyBagContainerTypes {}
#[repr(C, align(8))]
pub struct FPropertyBagPropertyDescMetaData {
    pub key: FName,
    pub value: FString,
}
#[repr(C, align(8))]
pub struct FPropertyBagPropertyDesc {
    pub value_type_object: UPtr<UObject>,
    pub id: FGuid,
    pub name: FName,
    pub value_type: EPropertyBagPropertyType,
    pub container_types: FPropertyBagContainerTypes,
    pub property_flags: u64,
    pub meta_data: TArray<FPropertyBagPropertyDescMetaData>,
    pub meta_class: TSubclassOf<UObject>,
}
#[repr(C, align(8))]
pub struct FInstancedPropertyBag {
    pub value: FInstancedStruct,
}
#[repr(C, align(1))]
pub struct FPropertyBagMissingStruct {}
#[repr(C, align(8))]
pub struct FSharedStruct {}
#[repr(C, align(8))]
pub struct FConstSharedStruct {}
#[repr(C, align(8))]
pub struct FObjectCookedMetaDataStore {
    pub object_meta_data: TMap<FName, FString>,
}
#[repr(C, align(8))]
pub struct FFieldCookedMetaDataKey {
    pub field_path: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FFieldCookedMetaDataValue {
    pub meta_data: TMap<FName, FString>,
}
#[repr(C, align(8))]
pub struct FFieldCookedMetaDataStore {
    pub field_meta_data: TMap<FName, FString>,
    pub sub_field_meta_data: TMap<FFieldCookedMetaDataKey, FFieldCookedMetaDataValue>,
}
#[repr(C, align(8))]
pub struct FStructCookedMetaDataStore {
    pub object_meta_data: FObjectCookedMetaDataStore,
    pub properties_meta_data: TMap<FName, FFieldCookedMetaDataStore>,
}
#[repr(C, align(8))]
pub struct FOverriddenPropertyPath {
    pub path: TArray<FName>,
    pub cached_hash: u32,
}
#[repr(C, align(8))]
pub struct FOverriddenPropertyNodeID {
    pub path: FOverriddenPropertyPath,
    pub object: TWeakObjectPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FOverriddenPropertyNode {
    pub node_id: FOverriddenPropertyNodeID,
    pub operation: EOverriddenPropertyOperation,
}
#[repr(C, align(8))]
pub struct FOverriddenPropertySet {
    pub owner: UPtr<UObject>,
    pub b_was_added: bool,
    pub root_node: FOverriddenPropertyNode,
}
#[repr(C, align(8))]
pub struct FPerPlatformInt {
    pub default: i32,
    pub per_platform: TMap<FName, i32>,
}
#[repr(C, align(8))]
pub struct FFreezablePerPlatformInt {}
#[repr(C, align(8))]
pub struct FPerPlatformFloat {
    pub default: f32,
    pub per_platform: TMap<FName, f32>,
}
#[repr(C, align(8))]
pub struct FPerPlatformBool {
    pub default: bool,
    pub per_platform: TMap<FName, bool>,
}
#[repr(C, align(8))]
pub struct FPerPlatformFrameRate {
    pub default: FFrameRate,
    pub per_platform: TMap<FName, FFrameRate>,
}
#[repr(C, align(8))]
pub struct FFilePath {
    pub file_path: FString,
}
#[repr(C, align(8))]
pub struct FDirectoryPath {
    pub path: FString,
}
#[repr(C, align(8))]
pub struct FTemplateString {
    pub template: FString,
    pub resolved: FText,
}
#[repr(C, align(8))]
pub struct FProfileLocus {}
#[repr(C, align(8))]
pub struct FSolarisProfilingData {}
#[repr(C, align(8))]
pub struct FVersePersistentVar {
    pub path: FString,
    pub property: TFieldPath<FMapProperty>,
}
#[repr(C, align(8))]
pub struct FVerseSessionVar {
    pub property: TFieldPath<FMapProperty>,
}
#[repr(C, align(8))]
pub struct FVerseClassVarAccessor {
    pub func: UPtr<UFunction>,
    pub b_is_instance_member: bool,
    pub b_is_fallible: bool,
}
#[repr(C, align(8))]
pub struct FVerseClassVarAccessors {
    pub getters: TMap<i32, FVerseClassVarAccessor>,
    pub setters: TMap<i32, FVerseClassVarAccessor>,
}
#[repr(C, align(4))]
pub struct FTestPropertyPathFunctionsStructKey {
    pub unused: i32,
    pub key: i32,
}
#[repr(C, align(8))]
pub struct FTestPropertyPathFunctionsStruct {
    pub unused: i32,
    pub int32: i32,
    pub int32_static_array: i32,
    pub int32_array: TArray<i32>,
    pub int32_set: TSet<i32>,
    pub int32_map: TMap<i32, i32>,
    pub int32_optional: TOptional<i32>,
}
#[repr(C, align(4))]
pub struct FTestInstanceDataObjectPoint {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}
#[repr(C, align(4))]
pub struct FTestInstanceDataObjectPointAlternate {
    pub u: i32,
    pub v: i32,
    pub w: i32,
}
#[repr(C, align(4))]
pub struct FTestInstanceDataObjectStruct {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
    pub bird: ETestInstanceDataObjectBird,
    pub grain: ETestInstanceDataObjectGrain,
    pub fruit: ETestInstanceDataObjectFruit,
    pub direction: ETestInstanceDataObjectDirection,
    pub full_flags: ETestInstanceDataObjectFullFlags,
    pub grain_from_enum_class: ETestInstanceDataObjectGrain,
    pub fruit_from_namespace: ETestInstanceDataObjectFruit,
    pub grain_type_change: ETestInstanceDataObjectGrain,
    pub fruit_type_change: ETestInstanceDataObjectFruit,
    pub grain_type_and_property_change: ETestInstanceDataObjectGrain,
    pub fruit_type_and_property_change: ETestInstanceDataObjectFruit,
    pub point: FTestInstanceDataObjectPoint,
}
#[repr(C, align(8))]
pub struct FTestInstanceDataObjectStructAlternate {
    pub b: f32,
    pub c: i64,
    pub d: i32,
    pub e: i32,
    pub bird: ETestInstanceDataObjectBird,
    pub grain: ETestInstanceDataObjectGrainAlternate,
    pub fruit: ETestInstanceDataObjectFruitAlternate,
    pub direction: ETestInstanceDataObjectDirectionAlternate,
    pub grain_from_enum_class: ETestInstanceDataObjectGrainAlternateEnumClass,
    pub fruit_from_namespace: ETestInstanceDataObjectFruitAlternateNamespace,
    pub grain_type_change: ETestInstanceDataObjectGrainAlternate,
    pub fruit_type_change: ETestInstanceDataObjectFruitAlternate,
    pub grain_type_and_property_change: ETestInstanceDataObjectGrainAlternateEnumClass,
    pub fruit_type_and_property_change: ETestInstanceDataObjectFruitAlternateNamespace,
    pub deleted_grain: ETestInstanceDataObjectGrainAlternate,
    pub deleted_fruit: ETestInstanceDataObjectFruitAlternate,
    pub deleted_direction: ETestInstanceDataObjectDirectionAlternate,
    pub point: FTestInstanceDataObjectPointAlternate,
}
#[repr(C, align(8))]
pub struct FSubobjectInstancingTestStructType {
    pub inner_object: UPtr<USubobjectInstancingTestObject>,
    pub inner_object_from_type: UPtr<USubobjectInstancingDefaultToInstancedTestObject>,
    pub inner_object_array: TArray<UPtr<USubobjectInstancingTestObject>>,
}
pub struct UObject {}
pub struct UGCBarrier {}
pub struct UGCObjectReferencer {}
pub struct UObjectPtrTestClass {}
pub struct UObjectPtrAbstractTestClass {}
pub struct UObjectPtrAbstractDerivedTestClass {}
pub struct UObjectPtrTestClassWithRef {
    pub object_ptr: UPtr<UObjectPtrTestClass>,
    pub object_ptr_non_nullable: UPtr<UObjectPtrTestClass>,
    pub object_ptr_abstract_non_nullable: UPtr<UObjectPtrAbstractTestClass>,
    pub array_obj_ptr: TArray<UPtr<UObjectPtrTestClass>>,
}
pub struct UObjectWithClassProperty {
    pub class_ptr: TSubclassOf<UObject>,
    pub sub_class: TSubclassOf<UObjectPtrTestClass>,
    pub class_raw: TSubclassOf<UObject>,
}
pub struct UObjectWithRawProperty {
    pub object_ptr: UPtr<UObjectPtrTestClass>,
    pub object_ptr_non_nullable: UPtr<UObjectPtrTestClass>,
}
pub struct UObjectPtrDerrivedTestClass {}
pub struct UObjectPtrNotLazyTestClass {}
pub struct UObjectPtrStressTestClass {}
pub struct UMiddleClass {}
pub struct UDerrivedClass {}
pub struct UOptionalPropertyTestObject {
    pub optional_string: TOptional<FString>,
    pub optional_text: TOptional<FText>,
    pub optional_name: TOptional<FName>,
    pub optional_int: TOptional<i32>,
}
pub struct UPropertyWrapper {}
pub struct UMulticastDelegatePropertyWrapper {}
pub struct UMulticastInlineDelegatePropertyWrapper {}
pub struct UField {
    pub next: UPtr<UField>,
}
pub struct UProperty {}
pub struct UNumericProperty {}
pub struct UByteProperty {
    pub enum_: UPtr<UEnum>,
}
pub struct UInt8Property {}
pub struct UInt16Property {}
pub struct UIntProperty {}
pub struct UInt64Property {}
pub struct UUInt16Property {}
pub struct UUInt32Property {}
pub struct UUInt64Property {}
pub struct UFloatProperty {}
pub struct UDoubleProperty {}
pub struct UBoolProperty {}
pub struct UObjectPropertyBase {
    pub property_class: TSubclassOf<UObject>,
}
pub struct UObjectProperty {}
pub struct UWeakObjectProperty {}
pub struct ULazyObjectProperty {}
pub struct USoftObjectProperty {}
pub struct UClassProperty {
    pub meta_class: TSubclassOf<UObject>,
}
pub struct USoftClassProperty {
    pub meta_class: TSubclassOf<UObject>,
}
pub struct UInterfaceProperty {
    pub interface_class: TSubclassOf<UObject>,
}
pub struct UNameProperty {}
pub struct UStrProperty {}
pub struct UArrayProperty {
    pub inner: UPtr<UProperty>,
}
pub struct UMapProperty {
    pub key_prop: UPtr<UProperty>,
    pub value_prop: UPtr<UProperty>,
}
pub struct USetProperty {
    pub element_prop: UPtr<UProperty>,
}
pub struct UStructProperty {
    pub _struct: UPtr<UScriptStruct>,
}
pub struct UDelegateProperty {
    pub signature_function: UPtr<UFunction>,
}
pub struct UMulticastDelegateProperty {
    pub signature_function: UPtr<UFunction>,
}
pub struct UMulticastInlineDelegateProperty {}
pub struct UMulticastSparseDelegateProperty {}
pub struct UEnumProperty {
    pub underlying_prop: UPtr<UNumericProperty>,
    pub enum_: UPtr<UEnum>,
}
pub struct UTextProperty {}
pub struct UInterface {}
pub struct UEditorPathObjectInterface {}
pub struct IEditorPathObjectInterface {}
pub struct UTextBuffer {}
pub struct UPropertyBagMissingObject {}
pub struct UStruct {
    pub super_struct: UPtr<UStruct>,
    pub children: UPtr<UField>,
    pub script_and_property_object_references: TArray<UPtr<UObject>>,
    pub property_wrappers: TArray<UPtr<UPropertyWrapper>>,
}
pub struct UScriptStruct {}
pub struct UPropertyBag {
    pub property_descs: TArray<FPropertyBagPropertyDesc>,
}
pub struct UUserDefinedStruct {
    pub primary_struct: TWeakObjectPtr<UUserDefinedStruct>,
    pub error_message: FString,
    pub editor_data: UPtr<UObject>,
    pub status: EUserDefinedStructureStatus,
    pub guid: FGuid,
    pub cached_cooked_meta_data_ptr: UPtr<UStructCookedMetaData>,
}
pub struct UUserDefinedStructEditorDataBase {}
pub struct UFunction {}
pub struct UDelegateFunction {}
pub struct USparseDelegateFunction {}
pub struct UEnum {}
pub struct UPackage {}
pub struct UClass {
    pub class_within: TSubclassOf<UObject>,
    pub class_generated_by: UPtr<UObject>,
    pub net_fields: TArray<UPtr<UField>>,
    pub class_default_object: UPtr<UObject>,
}
pub struct UEnumCookedMetaData {
    pub enum_meta_data: FObjectCookedMetaDataStore,
}
pub struct UStructCookedMetaData {
    pub struct_meta_data: FStructCookedMetaDataStore,
}
pub struct UClassCookedMetaData {
    pub class_meta_data: FStructCookedMetaDataStore,
    pub functions_meta_data: TMap<FName, FStructCookedMetaDataStore>,
}
pub struct UPackageMap {}
pub struct UObjectReachabilityStressData {
    pub children: TArray<UPtr<UObjectReachabilityStressData>>,
}
pub struct UInstanceDataObjectClass {}
pub struct UInstanceDataObjectStruct {}
pub struct ULinkerPlaceholderClass {}
pub struct ULinkerPlaceholderExportObject {}
pub struct ULinkerPlaceholderFunction {}
pub struct UDEPRECATED_MetaData {}
pub struct UObjectRedirector {}
pub struct UVerseClass {
    pub sol_class_flags: u32,
    pub task_classes: TArray<TSubclassOf<UObject>>,
    pub init_instance_function: UPtr<UFunction>,
    pub persistent_vars: TArray<FVersePersistentVar>,
    pub session_vars: TArray<FVerseSessionVar>,
    pub var_accessors: TMap<FName, FVerseClassVarAccessors>,
    pub constructor_effects: EVerseEffectSet,
    pub mangled_package_verse_path: FName,
    pub package_relative_verse_path: FString,
    pub display_name_to_ue_name_function_map: TMap<FName, FName>,
    pub direct_interfaces: TArray<TSubclassOf<UObject>>,
    pub properties_written_by_init_cdo: TArray<TFieldPath<FProperty>>,
    pub function_mangled_names: TMap<FName, FName>,
    pub predicts_function_names: TArray<FName>,
    pub predicts_var_names: TMap<FAnsiString, FName>,
    pub predicts_coerced_functions: TMap<FName, FName>,
    pub int_property_constraints: TMap<TFieldPath<FInt64Property>, FVerseIntConstraints>,
    pub double_property_constraints: TMap<
        TFieldPath<FDoubleProperty>,
        FVerseDoubleConstraints,
    >,
    pub cached_cooked_meta_data_ptr: UPtr<UClassCookedMetaData>,
}
pub struct UVerseEnum {
    pub verse_enum_flags: EVerseEnumFlags,
    pub qualified_name: FUtf8String,
    pub cached_cooked_meta_data_ptr: UPtr<UEnumCookedMetaData>,
}
pub struct UVerseFunction {}
pub struct UVerseStruct {
    pub verse_class_flags: u32,
    pub qualified_name: FUtf8String,
    pub init_function: UPtr<UFunction>,
    pub module_class: TSubclassOf<UObject>,
    pub guid: FGuid,
    pub factory_function: UPtr<UFunction>,
    pub override_factory_function: UPtr<UFunction>,
    pub constructor_effects: EVerseEffectSet,
    pub int_property_constraints: TMap<TFieldPath<FInt64Property>, FVerseIntConstraints>,
    pub double_property_constraints: TMap<
        TFieldPath<FDoubleProperty>,
        FVerseDoubleConstraints,
    >,
    pub cached_cooked_meta_data_ptr: UPtr<UStructCookedMetaData>,
}
pub struct UTestPropertyPathFunctionsClass {
    pub struct_static_array: FTestPropertyPathFunctionsStruct,
    pub struct_array: TArray<FTestPropertyPathFunctionsStruct>,
    pub struct_set: TSet<FTestPropertyPathFunctionsStructKey>,
    pub struct_map: TMap<
        FTestPropertyPathFunctionsStructKey,
        FTestPropertyPathFunctionsStruct,
    >,
    pub struct_optional: TOptional<FTestPropertyPathFunctionsStruct>,
}
pub struct UTestInstanceDataObjectClass {
    pub a: i32,
    pub b: f32,
    pub c: i64,
    pub d: i32,
    pub e: i32,
    pub _struct: FTestInstanceDataObjectStruct,
}
pub struct USubobjectInstancingTestObject {
    pub test_value: i32,
}
pub struct USubobjectInstancingTestDerivedObject {}
pub struct USubobjectInstancingTestDirectlyNestedObject {
    pub self_ref: UPtr<UObject>,
    pub owner_object: UPtr<UObject>,
    pub inner_object: UPtr<USubobjectInstancingTestObject>,
}
pub struct USubobjectInstancingTestIndirectlyNestedObject {
    pub inner_object: UPtr<USubobjectInstancingTestDirectlyNestedObject>,
}
pub struct USubobjectInstancingDefaultToInstancedTestObject {}
pub struct USubobjectInstancingTestOuterObject {
    pub self_ref: UPtr<UObject>,
    pub null_object: UPtr<USubobjectInstancingTestObject>,
    pub inner_object: UPtr<USubobjectInstancingTestObject>,
    pub shared_object: UPtr<USubobjectInstancingTestObject>,
    pub external_object: UPtr<USubobjectInstancingTestObject>,
    pub internal_object: UPtr<USubobjectInstancingTestObject>,
    pub transient_inner_object: UPtr<USubobjectInstancingTestObject>,
    pub edit_time_inner_object: UPtr<USubobjectInstancingTestObject>,
    pub local_only_inner_object: UPtr<USubobjectInstancingTestObject>,
    pub inner_object_from_type: UPtr<USubobjectInstancingDefaultToInstancedTestObject>,
    pub inner_object_using_new: UPtr<USubobjectInstancingTestObject>,
    pub inner_object_post_init: UPtr<USubobjectInstancingTestObject>,
    pub inner_object_array: TArray<UPtr<USubobjectInstancingTestObject>>,
    pub inner_object_from_type_array: TArray<
        UPtr<USubobjectInstancingDefaultToInstancedTestObject>,
    >,
    pub inner_object_set: TSet<UPtr<USubobjectInstancingTestObject>>,
    pub inner_object_from_type_set: TSet<
        UPtr<USubobjectInstancingDefaultToInstancedTestObject>,
    >,
    pub inner_object_map: TMap<
        UPtr<USubobjectInstancingTestObject>,
        UPtr<USubobjectInstancingTestObject>,
    >,
    pub inner_object_from_type_map: TMap<
        UPtr<USubobjectInstancingDefaultToInstancedTestObject>,
        UPtr<USubobjectInstancingDefaultToInstancedTestObject>,
    >,
    pub struct_with_inner_objects: FSubobjectInstancingTestStructType,
    pub optional_inner_object: TOptional<UPtr<USubobjectInstancingTestObject>>,
    pub optional_inner_object_from_type: TOptional<
        UPtr<USubobjectInstancingDefaultToInstancedTestObject>,
    >,
    pub optional_inner_object_array: TOptional<
        TArray<UPtr<USubobjectInstancingTestObject>>,
    >,
    pub inner_object_with_directly_nested_object: UPtr<
        USubobjectInstancingTestDirectlyNestedObject,
    >,
    pub inner_object_with_indirectly_nested_object: UPtr<
        USubobjectInstancingTestIndirectlyNestedObject,
    >,
}
pub struct USubobjectInstancingTestDerivedOuterObjectWithTypeOverride {}
pub struct USubobjectInstancingTestDerivedOuterObjectWithDoNotCreateOverride {}
pub struct UDynamicSubobjectInstancingTestClass {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAutomationEventType(pub u8);
impl EAutomationEventType {
    pub const INFO: EAutomationEventType = EAutomationEventType(0);
    pub const WARNING: EAutomationEventType = EAutomationEventType(1);
    pub const ERROR: EAutomationEventType = EAutomationEventType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERangeBoundTypes(pub u8);
impl ERangeBoundTypes {
    pub const EXCLUSIVE: ERangeBoundTypes = ERangeBoundTypes(0);
    pub const INCLUSIVE: ERangeBoundTypes = ERangeBoundTypes(1);
    pub const OPEN: ERangeBoundTypes = ERangeBoundTypes(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInterpCurveMode(pub u8);
impl EInterpCurveMode {
    pub const CIM_LINEAR: EInterpCurveMode = EInterpCurveMode(0);
    pub const CIM_CURVE_AUTO: EInterpCurveMode = EInterpCurveMode(1);
    pub const CIM_CONSTANT: EInterpCurveMode = EInterpCurveMode(2);
    pub const CIM_CURVE_USER: EInterpCurveMode = EInterpCurveMode(3);
    pub const CIM_CURVE_BREAK: EInterpCurveMode = EInterpCurveMode(4);
    pub const CIM_CURVE_AUTO_CLAMPED: EInterpCurveMode = EInterpCurveMode(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInputDeviceConnectionState(pub u8);
impl EInputDeviceConnectionState {
    pub const INVALID: EInputDeviceConnectionState = EInputDeviceConnectionState(0);
    pub const UNKNOWN: EInputDeviceConnectionState = EInputDeviceConnectionState(1);
    pub const DISCONNECTED: EInputDeviceConnectionState = EInputDeviceConnectionState(2);
    pub const CONNECTED: EInputDeviceConnectionState = EInputDeviceConnectionState(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELocalizedTextSourceCategory(pub u8);
impl ELocalizedTextSourceCategory {
    pub const GAME: ELocalizedTextSourceCategory = ELocalizedTextSourceCategory(0);
    pub const ENGINE: ELocalizedTextSourceCategory = ELocalizedTextSourceCategory(1);
    pub const EDITOR: ELocalizedTextSourceCategory = ELocalizedTextSourceCategory(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPropertyBagPropertyType(pub u8);
impl EPropertyBagPropertyType {
    pub const NONE: EPropertyBagPropertyType = EPropertyBagPropertyType(0);
    pub const BOOL: EPropertyBagPropertyType = EPropertyBagPropertyType(1);
    pub const BYTE: EPropertyBagPropertyType = EPropertyBagPropertyType(2);
    pub const INT32: EPropertyBagPropertyType = EPropertyBagPropertyType(3);
    pub const INT64: EPropertyBagPropertyType = EPropertyBagPropertyType(4);
    pub const FLOAT: EPropertyBagPropertyType = EPropertyBagPropertyType(5);
    pub const DOUBLE: EPropertyBagPropertyType = EPropertyBagPropertyType(6);
    pub const NAME: EPropertyBagPropertyType = EPropertyBagPropertyType(7);
    pub const STRING: EPropertyBagPropertyType = EPropertyBagPropertyType(8);
    pub const TEXT: EPropertyBagPropertyType = EPropertyBagPropertyType(9);
    pub const ENUM: EPropertyBagPropertyType = EPropertyBagPropertyType(10);
    pub const STRUCT: EPropertyBagPropertyType = EPropertyBagPropertyType(11);
    pub const OBJECT: EPropertyBagPropertyType = EPropertyBagPropertyType(12);
    pub const SOFT_OBJECT: EPropertyBagPropertyType = EPropertyBagPropertyType(13);
    pub const CLASS: EPropertyBagPropertyType = EPropertyBagPropertyType(14);
    pub const SOFT_CLASS: EPropertyBagPropertyType = EPropertyBagPropertyType(15);
    pub const U_INT32: EPropertyBagPropertyType = EPropertyBagPropertyType(16);
    pub const U_INT64: EPropertyBagPropertyType = EPropertyBagPropertyType(17);
    pub const COUNT: EPropertyBagPropertyType = EPropertyBagPropertyType(18);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EOverriddenPropertyOperation(pub u8);
impl EOverriddenPropertyOperation {
    pub const NONE: EOverriddenPropertyOperation = EOverriddenPropertyOperation(0);
    pub const MODIFIED: EOverriddenPropertyOperation = EOverriddenPropertyOperation(1);
    pub const REPLACE: EOverriddenPropertyOperation = EOverriddenPropertyOperation(2);
    pub const ADD: EOverriddenPropertyOperation = EOverriddenPropertyOperation(3);
    pub const REMOVE: EOverriddenPropertyOperation = EOverriddenPropertyOperation(4);
    pub const SUB_OBJECTS_SHADOWING: EOverriddenPropertyOperation = EOverriddenPropertyOperation(
        5,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETestInstanceDataObjectBird(pub u8);
impl ETestInstanceDataObjectBird {
    pub const TIDOB_NONE: ETestInstanceDataObjectBird = ETestInstanceDataObjectBird(0);
    pub const TIDOB_CARDINAL: ETestInstanceDataObjectBird = ETestInstanceDataObjectBird(
        1,
    );
    pub const TIDOB_CROW: ETestInstanceDataObjectBird = ETestInstanceDataObjectBird(2);
    pub const TIDOB_EAGLE: ETestInstanceDataObjectBird = ETestInstanceDataObjectBird(3);
    pub const TIDOB_HAWK: ETestInstanceDataObjectBird = ETestInstanceDataObjectBird(4);
    pub const TIDOB_OWL: ETestInstanceDataObjectBird = ETestInstanceDataObjectBird(5);
    pub const TIDOB_RAVEN: ETestInstanceDataObjectBird = ETestInstanceDataObjectBird(6);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETestInstanceDataObjectGrain(pub u8);
impl ETestInstanceDataObjectGrain {
    pub const NONE: ETestInstanceDataObjectGrain = ETestInstanceDataObjectGrain(0);
    pub const BARLEY: ETestInstanceDataObjectGrain = ETestInstanceDataObjectGrain(1);
    pub const CORN: ETestInstanceDataObjectGrain = ETestInstanceDataObjectGrain(2);
    pub const QUINOA: ETestInstanceDataObjectGrain = ETestInstanceDataObjectGrain(3);
    pub const RICE: ETestInstanceDataObjectGrain = ETestInstanceDataObjectGrain(4);
    pub const WHEAT: ETestInstanceDataObjectGrain = ETestInstanceDataObjectGrain(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETestInstanceDataObjectFruit(pub u8);
impl ETestInstanceDataObjectFruit {
    pub const NONE: ETestInstanceDataObjectFruit = ETestInstanceDataObjectFruit(0);
    pub const APPLE: ETestInstanceDataObjectFruit = ETestInstanceDataObjectFruit(1);
    pub const BANANA: ETestInstanceDataObjectFruit = ETestInstanceDataObjectFruit(2);
    pub const LEMON: ETestInstanceDataObjectFruit = ETestInstanceDataObjectFruit(3);
    pub const ORANGE: ETestInstanceDataObjectFruit = ETestInstanceDataObjectFruit(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETestInstanceDataObjectDirection(pub u16);
impl ETestInstanceDataObjectDirection {
    pub const NONE: ETestInstanceDataObjectDirection = ETestInstanceDataObjectDirection(
        0,
    );
    pub const NORTH: ETestInstanceDataObjectDirection = ETestInstanceDataObjectDirection(
        1,
    );
    pub const EAST: ETestInstanceDataObjectDirection = ETestInstanceDataObjectDirection(
        2,
    );
    pub const SOUTH: ETestInstanceDataObjectDirection = ETestInstanceDataObjectDirection(
        4,
    );
    pub const WEST: ETestInstanceDataObjectDirection = ETestInstanceDataObjectDirection(
        8,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETestInstanceDataObjectFullFlags(pub u8);
impl ETestInstanceDataObjectFullFlags {
    pub const NONE: ETestInstanceDataObjectFullFlags = ETestInstanceDataObjectFullFlags(
        0,
    );
    pub const FLAG0: ETestInstanceDataObjectFullFlags = ETestInstanceDataObjectFullFlags(
        1,
    );
    pub const FLAG1: ETestInstanceDataObjectFullFlags = ETestInstanceDataObjectFullFlags(
        2,
    );
    pub const FLAG2: ETestInstanceDataObjectFullFlags = ETestInstanceDataObjectFullFlags(
        4,
    );
    pub const FLAG4: ETestInstanceDataObjectFullFlags = ETestInstanceDataObjectFullFlags(
        16,
    );
    pub const FLAG5: ETestInstanceDataObjectFullFlags = ETestInstanceDataObjectFullFlags(
        32,
    );
    pub const FLAG6: ETestInstanceDataObjectFullFlags = ETestInstanceDataObjectFullFlags(
        64,
    );
    pub const FLAG7: ETestInstanceDataObjectFullFlags = ETestInstanceDataObjectFullFlags(
        128,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETestInstanceDataObjectGrainAlternate(pub u8);
impl ETestInstanceDataObjectGrainAlternate {
    pub const NONE: ETestInstanceDataObjectGrainAlternate = ETestInstanceDataObjectGrainAlternate(
        0,
    );
    pub const CORN: ETestInstanceDataObjectGrainAlternate = ETestInstanceDataObjectGrainAlternate(
        1,
    );
    pub const RICE: ETestInstanceDataObjectGrainAlternate = ETestInstanceDataObjectGrainAlternate(
        2,
    );
    pub const RYE: ETestInstanceDataObjectGrainAlternate = ETestInstanceDataObjectGrainAlternate(
        3,
    );
    pub const WHEAT: ETestInstanceDataObjectGrainAlternate = ETestInstanceDataObjectGrainAlternate(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETestInstanceDataObjectFruitAlternate(pub u8);
impl ETestInstanceDataObjectFruitAlternate {
    pub const NONE: ETestInstanceDataObjectFruitAlternate = ETestInstanceDataObjectFruitAlternate(
        0,
    );
    pub const APPLE: ETestInstanceDataObjectFruitAlternate = ETestInstanceDataObjectFruitAlternate(
        1,
    );
    pub const CHERRY: ETestInstanceDataObjectFruitAlternate = ETestInstanceDataObjectFruitAlternate(
        2,
    );
    pub const ORANGE: ETestInstanceDataObjectFruitAlternate = ETestInstanceDataObjectFruitAlternate(
        3,
    );
    pub const PEAR: ETestInstanceDataObjectFruitAlternate = ETestInstanceDataObjectFruitAlternate(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETestInstanceDataObjectDirectionAlternate(pub u16);
impl ETestInstanceDataObjectDirectionAlternate {
    pub const NONE: ETestInstanceDataObjectDirectionAlternate = ETestInstanceDataObjectDirectionAlternate(
        0,
    );
    pub const UP: ETestInstanceDataObjectDirectionAlternate = ETestInstanceDataObjectDirectionAlternate(
        1,
    );
    pub const DOWN: ETestInstanceDataObjectDirectionAlternate = ETestInstanceDataObjectDirectionAlternate(
        2,
    );
    pub const NORTH: ETestInstanceDataObjectDirectionAlternate = ETestInstanceDataObjectDirectionAlternate(
        4,
    );
    pub const EAST: ETestInstanceDataObjectDirectionAlternate = ETestInstanceDataObjectDirectionAlternate(
        8,
    );
    pub const SOUTH: ETestInstanceDataObjectDirectionAlternate = ETestInstanceDataObjectDirectionAlternate(
        16,
    );
    pub const WEST: ETestInstanceDataObjectDirectionAlternate = ETestInstanceDataObjectDirectionAlternate(
        32,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETestInstanceDataObjectGrainAlternateEnumClass(pub u8);
impl ETestInstanceDataObjectGrainAlternateEnumClass {
    pub const NONE: ETestInstanceDataObjectGrainAlternateEnumClass = ETestInstanceDataObjectGrainAlternateEnumClass(
        0,
    );
    pub const CORN: ETestInstanceDataObjectGrainAlternateEnumClass = ETestInstanceDataObjectGrainAlternateEnumClass(
        1,
    );
    pub const RICE: ETestInstanceDataObjectGrainAlternateEnumClass = ETestInstanceDataObjectGrainAlternateEnumClass(
        2,
    );
    pub const RYE: ETestInstanceDataObjectGrainAlternateEnumClass = ETestInstanceDataObjectGrainAlternateEnumClass(
        3,
    );
    pub const WHEAT: ETestInstanceDataObjectGrainAlternateEnumClass = ETestInstanceDataObjectGrainAlternateEnumClass(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETestInstanceDataObjectFruitAlternateNamespace(pub u8);
impl ETestInstanceDataObjectFruitAlternateNamespace {
    pub const NONE: ETestInstanceDataObjectFruitAlternateNamespace = ETestInstanceDataObjectFruitAlternateNamespace(
        0,
    );
    pub const APPLE: ETestInstanceDataObjectFruitAlternateNamespace = ETestInstanceDataObjectFruitAlternateNamespace(
        1,
    );
    pub const CHERRY: ETestInstanceDataObjectFruitAlternateNamespace = ETestInstanceDataObjectFruitAlternateNamespace(
        2,
    );
    pub const ORANGE: ETestInstanceDataObjectFruitAlternateNamespace = ETestInstanceDataObjectFruitAlternateNamespace(
        3,
    );
    pub const PEAR: ETestInstanceDataObjectFruitAlternateNamespace = ETestInstanceDataObjectFruitAlternateNamespace(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAxis(pub u8);
impl EAxis {
    pub const NONE: EAxis = EAxis(0);
    pub const X: EAxis = EAxis(1);
    pub const Y: EAxis = EAxis(2);
    pub const Z: EAxis = EAxis(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPixelFormat(pub u8);
impl EPixelFormat {
    pub const PF_UNKNOWN: EPixelFormat = EPixelFormat(0);
    pub const PF_A32B32G32R32F: EPixelFormat = EPixelFormat(1);
    pub const PF_B8G8R8A8: EPixelFormat = EPixelFormat(2);
    pub const PF_G8: EPixelFormat = EPixelFormat(3);
    pub const PF_G16: EPixelFormat = EPixelFormat(4);
    pub const PF_DXT1: EPixelFormat = EPixelFormat(5);
    pub const PF_DXT3: EPixelFormat = EPixelFormat(6);
    pub const PF_DXT5: EPixelFormat = EPixelFormat(7);
    pub const PF_UYVY: EPixelFormat = EPixelFormat(8);
    pub const PF_FLOAT_RGB: EPixelFormat = EPixelFormat(9);
    pub const PF_FLOAT_RGBA: EPixelFormat = EPixelFormat(10);
    pub const PF_DEPTH_STENCIL: EPixelFormat = EPixelFormat(11);
    pub const PF_SHADOW_DEPTH: EPixelFormat = EPixelFormat(12);
    pub const PF_R32_FLOAT: EPixelFormat = EPixelFormat(13);
    pub const PF_G16R16: EPixelFormat = EPixelFormat(14);
    pub const PF_G16R16F: EPixelFormat = EPixelFormat(15);
    pub const PF_G16R16F_FILTER: EPixelFormat = EPixelFormat(16);
    pub const PF_G32R32F: EPixelFormat = EPixelFormat(17);
    pub const PF_A2B10G10R10: EPixelFormat = EPixelFormat(18);
    pub const PF_A16B16G16R16: EPixelFormat = EPixelFormat(19);
    pub const PF_D24: EPixelFormat = EPixelFormat(20);
    pub const PF_R16F: EPixelFormat = EPixelFormat(21);
    pub const PF_R16F_FILTER: EPixelFormat = EPixelFormat(22);
    pub const PF_BC5: EPixelFormat = EPixelFormat(23);
    pub const PF_V8U8: EPixelFormat = EPixelFormat(24);
    pub const PF_A1: EPixelFormat = EPixelFormat(25);
    pub const PF_FLOAT_R11G11B10: EPixelFormat = EPixelFormat(26);
    pub const PF_A8: EPixelFormat = EPixelFormat(27);
    pub const PF_R32_UINT: EPixelFormat = EPixelFormat(28);
    pub const PF_R32_SINT: EPixelFormat = EPixelFormat(29);
    pub const PF_PVRTC2: EPixelFormat = EPixelFormat(30);
    pub const PF_PVRTC4: EPixelFormat = EPixelFormat(31);
    pub const PF_R16_UINT: EPixelFormat = EPixelFormat(32);
    pub const PF_R16_SINT: EPixelFormat = EPixelFormat(33);
    pub const PF_R16G16B16A16_UINT: EPixelFormat = EPixelFormat(34);
    pub const PF_R16G16B16A16_SINT: EPixelFormat = EPixelFormat(35);
    pub const PF_R5G6B5_UNORM: EPixelFormat = EPixelFormat(36);
    pub const PF_R8G8B8A8: EPixelFormat = EPixelFormat(37);
    pub const PF_A8R8G8B8: EPixelFormat = EPixelFormat(38);
    pub const PF_BC4: EPixelFormat = EPixelFormat(39);
    pub const PF_R8G8: EPixelFormat = EPixelFormat(40);
    pub const PF_ATC_RGB: EPixelFormat = EPixelFormat(41);
    pub const PF_ATC_RGBA_E: EPixelFormat = EPixelFormat(42);
    pub const PF_ATC_RGBA_I: EPixelFormat = EPixelFormat(43);
    pub const PF_X24_G8: EPixelFormat = EPixelFormat(44);
    pub const PF_ETC1: EPixelFormat = EPixelFormat(45);
    pub const PF_ETC2_RGB: EPixelFormat = EPixelFormat(46);
    pub const PF_ETC2_RGBA: EPixelFormat = EPixelFormat(47);
    pub const PF_R32G32B32A32_UINT: EPixelFormat = EPixelFormat(48);
    pub const PF_R16G16_UINT: EPixelFormat = EPixelFormat(49);
    pub const PF_ASTC_4X4: EPixelFormat = EPixelFormat(50);
    pub const PF_ASTC_6X6: EPixelFormat = EPixelFormat(51);
    pub const PF_ASTC_8X8: EPixelFormat = EPixelFormat(52);
    pub const PF_ASTC_10X10: EPixelFormat = EPixelFormat(53);
    pub const PF_ASTC_12X12: EPixelFormat = EPixelFormat(54);
    pub const PF_BC6H: EPixelFormat = EPixelFormat(55);
    pub const PF_BC7: EPixelFormat = EPixelFormat(56);
    pub const PF_R8_UINT: EPixelFormat = EPixelFormat(57);
    pub const PF_L8: EPixelFormat = EPixelFormat(58);
    pub const PF_XGXR8: EPixelFormat = EPixelFormat(59);
    pub const PF_R8G8B8A8_UINT: EPixelFormat = EPixelFormat(60);
    pub const PF_R8G8B8A8_SNORM: EPixelFormat = EPixelFormat(61);
    pub const PF_R16G16B16A16_UNORM: EPixelFormat = EPixelFormat(62);
    pub const PF_R16G16B16A16_SNORM: EPixelFormat = EPixelFormat(63);
    pub const PF_PLATFORM_HDR_0: EPixelFormat = EPixelFormat(64);
    pub const PF_PLATFORM_HDR_1: EPixelFormat = EPixelFormat(65);
    pub const PF_PLATFORM_HDR_2: EPixelFormat = EPixelFormat(66);
    pub const PF_NV12: EPixelFormat = EPixelFormat(67);
    pub const PF_R32G32_UINT: EPixelFormat = EPixelFormat(68);
    pub const PF_ETC2_R11_EAC: EPixelFormat = EPixelFormat(69);
    pub const PF_ETC2_RG11_EAC: EPixelFormat = EPixelFormat(70);
    pub const PF_R8: EPixelFormat = EPixelFormat(71);
    pub const PF_B5G5R5A1_UNORM: EPixelFormat = EPixelFormat(72);
    pub const PF_ASTC_4X4_HDR: EPixelFormat = EPixelFormat(73);
    pub const PF_ASTC_6X6_HDR: EPixelFormat = EPixelFormat(74);
    pub const PF_ASTC_8X8_HDR: EPixelFormat = EPixelFormat(75);
    pub const PF_ASTC_10X10_HDR: EPixelFormat = EPixelFormat(76);
    pub const PF_ASTC_12X12_HDR: EPixelFormat = EPixelFormat(77);
    pub const PF_G16R16_SNORM: EPixelFormat = EPixelFormat(78);
    pub const PF_R8G8_UINT: EPixelFormat = EPixelFormat(79);
    pub const PF_R32G32B32_UINT: EPixelFormat = EPixelFormat(80);
    pub const PF_R32G32B32_SINT: EPixelFormat = EPixelFormat(81);
    pub const PF_R32G32B32F: EPixelFormat = EPixelFormat(82);
    pub const PF_R8_SINT: EPixelFormat = EPixelFormat(83);
    pub const PF_R64_UINT: EPixelFormat = EPixelFormat(84);
    pub const PF_R9G9B9EXP5: EPixelFormat = EPixelFormat(85);
    pub const PF_P010: EPixelFormat = EPixelFormat(86);
    pub const PF_ASTC_4X4_NORM_RG: EPixelFormat = EPixelFormat(87);
    pub const PF_ASTC_6X6_NORM_RG: EPixelFormat = EPixelFormat(88);
    pub const PF_ASTC_8X8_NORM_RG: EPixelFormat = EPixelFormat(89);
    pub const PF_ASTC_10X10_NORM_RG: EPixelFormat = EPixelFormat(90);
    pub const PF_ASTC_12X12_NORM_RG: EPixelFormat = EPixelFormat(91);
    pub const PF_R16G16_SINT: EPixelFormat = EPixelFormat(92);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELifetimeCondition(pub u8);
impl ELifetimeCondition {
    pub const COND_NONE: ELifetimeCondition = ELifetimeCondition(0);
    pub const COND_INITIAL_ONLY: ELifetimeCondition = ELifetimeCondition(1);
    pub const COND_OWNER_ONLY: ELifetimeCondition = ELifetimeCondition(2);
    pub const COND_SKIP_OWNER: ELifetimeCondition = ELifetimeCondition(3);
    pub const COND_SIMULATED_ONLY: ELifetimeCondition = ELifetimeCondition(4);
    pub const COND_AUTONOMOUS_ONLY: ELifetimeCondition = ELifetimeCondition(5);
    pub const COND_SIMULATED_OR_PHYSICS: ELifetimeCondition = ELifetimeCondition(6);
    pub const COND_INITIAL_OR_OWNER: ELifetimeCondition = ELifetimeCondition(7);
    pub const COND_CUSTOM: ELifetimeCondition = ELifetimeCondition(8);
    pub const COND_REPLAY_OR_OWNER: ELifetimeCondition = ELifetimeCondition(9);
    pub const COND_REPLAY_ONLY: ELifetimeCondition = ELifetimeCondition(10);
    pub const COND_SIMULATED_ONLY_NO_REPLAY: ELifetimeCondition = ELifetimeCondition(11);
    pub const COND_SIMULATED_OR_PHYSICS_NO_REPLAY: ELifetimeCondition = ELifetimeCondition(
        12,
    );
    pub const COND_SKIP_REPLAY: ELifetimeCondition = ELifetimeCondition(13);
    pub const COND_DYNAMIC: ELifetimeCondition = ELifetimeCondition(14);
    pub const COND_NEVER: ELifetimeCondition = ELifetimeCondition(15);
    pub const COND_NET_GROUP: ELifetimeCondition = ELifetimeCondition(16);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInputDeviceTriggerMask(pub u8);
impl EInputDeviceTriggerMask {
    pub const NONE: EInputDeviceTriggerMask = EInputDeviceTriggerMask(0);
    pub const LEFT: EInputDeviceTriggerMask = EInputDeviceTriggerMask(1);
    pub const RIGHT: EInputDeviceTriggerMask = EInputDeviceTriggerMask(2);
    pub const ALL: EInputDeviceTriggerMask = EInputDeviceTriggerMask(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EUnit(pub u8);
impl EUnit {
    pub const MICROMETERS: EUnit = EUnit(0);
    pub const MILLIMETERS: EUnit = EUnit(1);
    pub const CENTIMETERS: EUnit = EUnit(2);
    pub const METERS: EUnit = EUnit(3);
    pub const KILOMETERS: EUnit = EUnit(4);
    pub const INCHES: EUnit = EUnit(5);
    pub const FEET: EUnit = EUnit(6);
    pub const YARDS: EUnit = EUnit(7);
    pub const MILES: EUnit = EUnit(8);
    pub const LIGHTYEARS: EUnit = EUnit(9);
    pub const DEGREES: EUnit = EUnit(10);
    pub const RADIANS: EUnit = EUnit(11);
    pub const CENTIMETERS_PER_SECOND: EUnit = EUnit(12);
    pub const METERS_PER_SECOND: EUnit = EUnit(13);
    pub const KILOMETERS_PER_HOUR: EUnit = EUnit(14);
    pub const MILES_PER_HOUR: EUnit = EUnit(15);
    pub const DEGREES_PER_SECOND: EUnit = EUnit(16);
    pub const RADIANS_PER_SECOND: EUnit = EUnit(17);
    pub const CENTIMETERS_PER_SECOND_SQUARED: EUnit = EUnit(18);
    pub const METERS_PER_SECOND_SQUARED: EUnit = EUnit(19);
    pub const CELSIUS: EUnit = EUnit(20);
    pub const FARENHEIT: EUnit = EUnit(21);
    pub const KELVIN: EUnit = EUnit(22);
    pub const MICROGRAMS: EUnit = EUnit(23);
    pub const MILLIGRAMS: EUnit = EUnit(24);
    pub const GRAMS: EUnit = EUnit(25);
    pub const KILOGRAMS: EUnit = EUnit(26);
    pub const METRIC_TONS: EUnit = EUnit(27);
    pub const OUNCES: EUnit = EUnit(28);
    pub const POUNDS: EUnit = EUnit(29);
    pub const STONES: EUnit = EUnit(30);
    pub const GRAMS_PER_CUBIC_CENTIMETER: EUnit = EUnit(31);
    pub const GRAMS_PER_CUBIC_METER: EUnit = EUnit(32);
    pub const KILOGRAMS_PER_CUBIC_CENTIMETER: EUnit = EUnit(33);
    pub const KILOGRAMS_PER_CUBIC_METER: EUnit = EUnit(34);
    pub const NEWTONS: EUnit = EUnit(35);
    pub const POUNDS_FORCE: EUnit = EUnit(36);
    pub const KILOGRAMS_FORCE: EUnit = EUnit(37);
    pub const KILOGRAM_CENTIMETERS_PER_SECOND_SQUARED: EUnit = EUnit(38);
    pub const NEWTON_METERS: EUnit = EUnit(39);
    pub const KILOGRAM_CENTIMETERS_SQUARED_PER_SECOND_SQUARED: EUnit = EUnit(40);
    pub const NEWTON_SECONDS: EUnit = EUnit(41);
    pub const KILOGRAM_CENTIMETERS: EUnit = EUnit(42);
    pub const KILOGRAM_METERS: EUnit = EUnit(43);
    pub const HERTZ: EUnit = EUnit(44);
    pub const KILOHERTZ: EUnit = EUnit(45);
    pub const MEGAHERTZ: EUnit = EUnit(46);
    pub const GIGAHERTZ: EUnit = EUnit(47);
    pub const REVOLUTIONS_PER_MINUTE: EUnit = EUnit(48);
    pub const BYTES: EUnit = EUnit(49);
    pub const KILOBYTES: EUnit = EUnit(50);
    pub const MEGABYTES: EUnit = EUnit(51);
    pub const GIGABYTES: EUnit = EUnit(52);
    pub const TERABYTES: EUnit = EUnit(53);
    pub const LUMENS: EUnit = EUnit(54);
    pub const CANDELA: EUnit = EUnit(55);
    pub const LUX: EUnit = EUnit(56);
    pub const CANDELA_PER_METER2: EUnit = EUnit(57);
    pub const EXPOSURE_VALUE: EUnit = EUnit(59);
    pub const NANOSECONDS: EUnit = EUnit(60);
    pub const MICROSECONDS: EUnit = EUnit(61);
    pub const MILLISECONDS: EUnit = EUnit(62);
    pub const SECONDS: EUnit = EUnit(63);
    pub const MINUTES: EUnit = EUnit(64);
    pub const HOURS: EUnit = EUnit(65);
    pub const DAYS: EUnit = EUnit(66);
    pub const MONTHS: EUnit = EUnit(67);
    pub const YEARS: EUnit = EUnit(68);
    pub const PIXELS_PER_INCH: EUnit = EUnit(69);
    pub const PERCENTAGE: EUnit = EUnit(70);
    pub const MULTIPLIER: EUnit = EUnit(71);
    pub const PASCALS: EUnit = EUnit(72);
    pub const KILO_PASCALS: EUnit = EUnit(73);
    pub const MEGA_PASCALS: EUnit = EUnit(74);
    pub const GIGA_PASCALS: EUnit = EUnit(75);
    pub const UNSPECIFIED: EUnit = EUnit(76);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataValidationUsecase(pub u8);
impl EDataValidationUsecase {
    pub const NONE: EDataValidationUsecase = EDataValidationUsecase(0);
    pub const MANUAL: EDataValidationUsecase = EDataValidationUsecase(1);
    pub const COMMANDLET: EDataValidationUsecase = EDataValidationUsecase(2);
    pub const SAVE: EDataValidationUsecase = EDataValidationUsecase(3);
    pub const PRE_SUBMIT: EDataValidationUsecase = EDataValidationUsecase(4);
    pub const SCRIPT: EDataValidationUsecase = EDataValidationUsecase(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPropertyAccessChangeNotifyMode(pub u8);
impl EPropertyAccessChangeNotifyMode {
    pub const DEFAULT: EPropertyAccessChangeNotifyMode = EPropertyAccessChangeNotifyMode(
        0,
    );
    pub const NEVER: EPropertyAccessChangeNotifyMode = EPropertyAccessChangeNotifyMode(
        1,
    );
    pub const ALWAYS: EPropertyAccessChangeNotifyMode = EPropertyAccessChangeNotifyMode(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMouseCursor(pub u8);
impl EMouseCursor {
    pub const NONE: EMouseCursor = EMouseCursor(0);
    pub const DEFAULT: EMouseCursor = EMouseCursor(1);
    pub const TEXT_EDIT_BEAM: EMouseCursor = EMouseCursor(2);
    pub const RESIZE_LEFT_RIGHT: EMouseCursor = EMouseCursor(3);
    pub const RESIZE_UP_DOWN: EMouseCursor = EMouseCursor(4);
    pub const RESIZE_SOUTH_EAST: EMouseCursor = EMouseCursor(5);
    pub const RESIZE_SOUTH_WEST: EMouseCursor = EMouseCursor(6);
    pub const CARDINAL_CROSS: EMouseCursor = EMouseCursor(7);
    pub const CROSSHAIRS: EMouseCursor = EMouseCursor(8);
    pub const HAND: EMouseCursor = EMouseCursor(9);
    pub const GRAB_HAND: EMouseCursor = EMouseCursor(10);
    pub const GRAB_HAND_CLOSED: EMouseCursor = EMouseCursor(11);
    pub const SLASHED_CIRCLE: EMouseCursor = EMouseCursor(12);
    pub const EYE_DROPPER: EMouseCursor = EMouseCursor(13);
    pub const CUSTOM: EMouseCursor = EMouseCursor(14);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESearchCase(pub u8);
impl ESearchCase {
    pub const CASE_SENSITIVE: ESearchCase = ESearchCase(0);
    pub const IGNORE_CASE: ESearchCase = ESearchCase(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESearchDir(pub u8);
impl ESearchDir {
    pub const FROM_START: ESearchDir = ESearchDir(0);
    pub const FROM_END: ESearchDir = ESearchDir(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataValidationResult(pub u8);
impl EDataValidationResult {
    pub const INVALID: EDataValidationResult = EDataValidationResult(0);
    pub const VALID: EDataValidationResult = EDataValidationResult(1);
    pub const NOT_VALIDATED: EDataValidationResult = EDataValidationResult(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAppMsgType(pub u8);
impl EAppMsgType {
    pub const OK: EAppMsgType = EAppMsgType(0);
    pub const YES_NO: EAppMsgType = EAppMsgType(1);
    pub const OK_CANCEL: EAppMsgType = EAppMsgType(2);
    pub const YES_NO_CANCEL: EAppMsgType = EAppMsgType(3);
    pub const CANCEL_RETRY_CONTINUE: EAppMsgType = EAppMsgType(4);
    pub const YES_NO_YES_ALL_NO_ALL: EAppMsgType = EAppMsgType(5);
    pub const YES_NO_YES_ALL_NO_ALL_CANCEL: EAppMsgType = EAppMsgType(6);
    pub const YES_NO_YES_ALL: EAppMsgType = EAppMsgType(7);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAppReturnType(pub u8);
impl EAppReturnType {
    pub const NO: EAppReturnType = EAppReturnType(0);
    pub const YES: EAppReturnType = EAppReturnType(1);
    pub const YES_ALL: EAppReturnType = EAppReturnType(2);
    pub const NO_ALL: EAppReturnType = EAppReturnType(3);
    pub const CANCEL: EAppReturnType = EAppReturnType(4);
    pub const OK: EAppReturnType = EAppReturnType(5);
    pub const RETRY: EAppReturnType = EAppReturnType(6);
    pub const CONTINUE: EAppReturnType = EAppReturnType(7);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAppMsgCategory(pub u8);
impl EAppMsgCategory {
    pub const WARNING: EAppMsgCategory = EAppMsgCategory(0);
    pub const ERROR: EAppMsgCategory = EAppMsgCategory(1);
    pub const SUCCESS: EAppMsgCategory = EAppMsgCategory(2);
    pub const INFO: EAppMsgCategory = EAppMsgCategory(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EUserDefinedStructureStatus(pub u8);
impl EUserDefinedStructureStatus {
    pub const UDSS_UP_TO_DATE: EUserDefinedStructureStatus = EUserDefinedStructureStatus(
        0,
    );
    pub const UDSS_DIRTY: EUserDefinedStructureStatus = EUserDefinedStructureStatus(1);
    pub const UDSS_ERROR: EUserDefinedStructureStatus = EUserDefinedStructureStatus(2);
    pub const UDSS_DUPLICATE: EUserDefinedStructureStatus = EUserDefinedStructureStatus(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EVerseEffectSet(pub u8);
impl EVerseEffectSet {
    pub const NONE: EVerseEffectSet = EVerseEffectSet(0);
    pub const SUSPENDS: EVerseEffectSet = EVerseEffectSet(1);
    pub const DECIDES: EVerseEffectSet = EVerseEffectSet(2);
    pub const DIVERGES: EVerseEffectSet = EVerseEffectSet(4);
    pub const READS: EVerseEffectSet = EVerseEffectSet(8);
    pub const WRITES: EVerseEffectSet = EVerseEffectSet(16);
    pub const ALLOCATES: EVerseEffectSet = EVerseEffectSet(32);
    pub const NO_ROLLBACK: EVerseEffectSet = EVerseEffectSet(64);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EVerseEnumFlags(pub u32);
impl EVerseEnumFlags {
    pub const NONE: EVerseEnumFlags = EVerseEnumFlags(0);
    pub const NATIVE_BOUND: EVerseEnumFlags = EVerseEnumFlags(1);
    pub const UHT_NATIVE: EVerseEnumFlags = EVerseEnumFlags(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAxisList(pub u8);
impl EAxisList {
    pub const NONE: EAxisList = EAxisList(0);
    pub const X: EAxisList = EAxisList(1);
    pub const Y: EAxisList = EAxisList(2);
    pub const Z: EAxisList = EAxisList(4);
    pub const SCREEN: EAxisList = EAxisList(8);
    pub const XY: EAxisList = EAxisList(3);
    pub const XZ: EAxisList = EAxisList(5);
    pub const YZ: EAxisList = EAxisList(6);
    pub const XYZ: EAxisList = EAxisList(7);
    pub const ALL: EAxisList = EAxisList(15);
    pub const Z_ROTATION: EAxisList = EAxisList(6);
    pub const ROTATE2_D: EAxisList = EAxisList(8);
    pub const LEFT: EAxisList = EAxisList(16);
    pub const UP: EAxisList = EAxisList(32);
    pub const FORWARD: EAxisList = EAxisList(64);
    pub const LU: EAxisList = EAxisList(48);
    pub const LF: EAxisList = EAxisList(80);
    pub const UF: EAxisList = EAxisList(96);
    pub const LEFT_UP_FORWARD: EAxisList = EAxisList(112);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInputDeviceMappingPolicy(pub i32);
impl EInputDeviceMappingPolicy {
    pub const INVALID: EInputDeviceMappingPolicy = EInputDeviceMappingPolicy(-1);
    pub const USE_MANAGED_PLATFORM_LOGIN: EInputDeviceMappingPolicy = EInputDeviceMappingPolicy(
        0,
    );
    pub const PRIMARY_USER_SHARES_KEYBOARD_AND_FIRST_GAMEPAD: EInputDeviceMappingPolicy = EInputDeviceMappingPolicy(
        1,
    );
    pub const CREATE_UNIQUE_PLATFORM_USER_FOR_EACH_DEVICE: EInputDeviceMappingPolicy = EInputDeviceMappingPolicy(
        2,
    );
    pub const MAP_ALL_DEVICES_TO_PRIMARY_USER: EInputDeviceMappingPolicy = EInputDeviceMappingPolicy(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELogTimes(pub u8);
impl ELogTimes {
    pub const NONE: ELogTimes = ELogTimes(0);
    pub const UTC: ELogTimes = ELogTimes(1);
    pub const SINCE_G_START_TIME: ELogTimes = ELogTimes(2);
    pub const LOCAL: ELogTimes = ELogTimes(3);
}
