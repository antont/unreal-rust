#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use super::*;
pub use super::super::core_data::*;
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
    pub is_valid: bool,
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
    pub is_valid: bool,
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
    pub is_looped: bool,
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
    pub is_looped: bool,
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
    pub is_looped: bool,
    pub loop_key_offset: f32,
}
#[repr(C, align(8))]
pub struct FInterpCurveTwoVectors {
    pub points: TArray<FInterpCurvePointTwoVectors>,
    pub is_looped: bool,
    pub loop_key_offset: f32,
}
#[repr(C, align(8))]
pub struct FInterpCurveVector {
    pub points: TArray<FInterpCurvePointVector>,
    pub is_looped: bool,
    pub loop_key_offset: f32,
}
#[repr(C, align(8))]
pub struct FInterpCurveVector2D {
    pub points: TArray<FInterpCurvePointVector2D>,
    pub is_looped: bool,
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
    pub is_minimal_patch: bool,
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
    pub drop_frame_format: bool,
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
    pub recursive_paths: bool,
    pub recursive_classes: bool,
    pub include_only_on_disk_assets: bool,
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
    pub was_added: bool,
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
    pub is_instance_member: bool,
    pub is_fallible: bool,
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
