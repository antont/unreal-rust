pub use super::enum_definition::*;
struct AActor {}
trait IActor {}
#[repr(C, align(4))]
pub struct FGuid {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
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
#[repr(C, align(1))]
pub struct FFallbackStruct {}
#[repr(C, align(4))]
pub struct FInt32Point {
    pub x: i32,
    pub y: i32,
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
#[repr(C, align(4))]
pub struct FInterpCurvePointFloat {
    pub in_val: f32,
    pub out_val: f32,
    pub arrive_tangent: f32,
    pub leave_tangent: f32,
    pub interp_mode: UNKOWN,
}
#[repr(C, align(4))]
pub struct FInterpCurvePointLinearColor {
    pub in_val: f32,
    pub out_val: FLinearColor,
    pub arrive_tangent: FLinearColor,
    pub leave_tangent: FLinearColor,
    pub interp_mode: UNKOWN,
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
    pub interp_mode: UNKOWN,
}
#[repr(C, align(16))]
pub struct FQuat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
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
    pub interp_mode: UNKOWN,
}
#[repr(C, align(8))]
pub struct FInterpCurvePointVector2D {
    pub in_val: f32,
    pub out_val: FVector2D,
    pub arrive_tangent: FVector2D,
    pub leave_tangent: FVector2D,
    pub interp_mode: UNKOWN,
}
#[repr(C, align(4))]
pub struct FIntPoint {
    pub x: i32,
    pub y: i32,
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
#[repr(C, align(8))]
pub struct FHitResult {
    pub face_index: i32,
    pub time: f32,
    pub distance: f32,
    pub location: FVector_NetQuantize,
    pub impact_point: FVector_NetQuantize,
    pub normal: FVector_NetQuantizeNormal,
    pub impact_normal: FVector_NetQuantizeNormal,
    pub trace_start: FVector_NetQuantize,
    pub trace_end: FVector_NetQuantize,
    pub penetration_depth: f32,
    pub my_item: i32,
    pub item: i32,
    pub element_index: u8,
    pub blocking_hit: bool,
    pub start_penetrating: bool,
    pub phys_material: UNKOWN,
    pub hit_object_handle: FActorInstanceHandle,
    pub component: UNKOWN,
    pub bone_name: FName,
    pub my_bone_name: FName,
}
#[repr(C, align(8))]
pub struct FVector_NetQuantize {}
#[repr(C, align(8))]
pub struct FVector_NetQuantizeNormal {}
#[repr(C, align(4))]
pub struct FWalkableSlopeOverride {
    pub walkable_slope_behavior: UNKOWN,
    pub walkable_slope_angle: f32,
}
#[repr(C, align(4))]
pub struct FGeometry {}
#[repr(C, align(4))]
pub struct FMargin {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
#[repr(C, align(4))]
pub struct FDeprecateSlateVector2D {}
#[repr(C, align(1))]
pub struct FPropertyBagMissingStruct {}
#[repr(C, align(1))]
pub struct FEditorDataStorageTag {}
#[repr(C, align(1))]
pub struct FEditorDataStorageUiDecoratorWidgetFactoryTag {}
#[repr(C, align(1))]
pub struct FEditorDataStorageColumn {}
#[repr(C, align(4))]
pub struct FTypedElementUObjectIdColumn {
    pub id: u32,
    pub serial_number: i32,
}
#[repr(C, align(8))]
pub struct FTypedElementExternalObjectColumn {}
#[repr(C, align(1))]
pub struct FTypedElementClassDefaultObjectTag {}
#[repr(C, align(1))]
pub struct FTypedElementActorTag {}
#[repr(C, align(1))]
pub struct FEditorDataStorageWorldTag {}
#[repr(C, align(1))]
pub struct FEditorDataStorageLevelTag {}
#[repr(C, align(1))]
pub struct FTypedElementPropertyBagPlaceholderTag {}
#[repr(C, align(1))]
pub struct FTypedElementLoosePropertyTag {}
#[repr(C, align(1))]
pub struct FFolderTag {}
#[repr(C, align(8))]
pub struct FTableRowParentColumn {}
#[repr(C, align(8))]
pub struct FTypedElementLabelHashColumn {
    pub label_hash: u64,
}
#[repr(C, align(1))]
pub struct FTypedElementSyncBackToWorldTag {}
#[repr(C, align(1))]
pub struct FTypedElementSyncFromWorldTag {}
#[repr(C, align(1))]
pub struct FTypedElementSyncFromWorldInteractiveTag {}
#[repr(C, align(8))]
pub struct FTypedElementRowReferenceColumn {}
#[repr(C, align(1))]
pub struct FFavoriteTag {}
#[repr(C, align(1))]
pub struct FObjectOverrideColumn {}
#[repr(C, align(8))]
pub struct FTypedElementPackageReference {}
#[repr(C, align(1))]
pub struct FTypedElementPackageUpdatedTag {}
#[repr(C, align(8))]
pub struct FTypedElementPivotOffset {
    pub offset: FVector,
}
#[repr(C, align(1))]
pub struct FSCCInChangelistTag {}
#[repr(C, align(1))]
pub struct FSCCStagedTag {}
#[repr(C, align(1))]
pub struct FSCCLockedTag {}
#[repr(C, align(1))]
pub struct FSCCExternallyEditedTag {}
#[repr(C, align(1))]
pub struct FSCCNotCurrentTag {}
#[repr(C, align(1))]
pub struct FTypedElementSlateWidgetReferenceDeletesRowTag {}
#[repr(C, align(1))]
pub struct FIsInEditingModeTag {}
#[repr(C, align(8))]
pub struct FWidgetPurposeReferenceColumn {}
#[repr(C, align(1))]
pub struct FTestColumnA {}
#[repr(C, align(1))]
pub struct FTestColumnB {}
#[repr(C, align(1))]
pub struct FTestColumnC {}
#[repr(C, align(1))]
pub struct FTestColumnD {}
#[repr(C, align(1))]
pub struct FTestColumnE {}
#[repr(C, align(1))]
pub struct FTestColumnF {}
#[repr(C, align(1))]
pub struct FTestColumnG {}
#[repr(C, align(1))]
pub struct FTestColumnDynamic {}
#[repr(C, align(1))]
pub struct FTestTagColumnA {}
#[repr(C, align(1))]
pub struct FTestTagColumnB {}
#[repr(C, align(1))]
pub struct FTestTagColumnC {}
#[repr(C, align(1))]
pub struct FTestTagColumnD {}
#[repr(C, align(1))]
pub struct FTEDSProcessorTests_PrimaryTag {}
#[repr(C, align(1))]
pub struct FTEDSProcessorTests_SecondaryTag {}
#[repr(C, align(1))]
pub struct FTEDSProcessorTests_Linked {}
#[repr(C, align(1))]
pub struct FHideRowFromUITag {}
#[repr(C, align(4))]
pub struct FTypedElementU32IntValueCacheColumn {
    pub value: u32,
}
#[repr(C, align(4))]
pub struct FTypedElementI32IntValueCacheColumn {
    pub value: i32,
}
#[repr(C, align(8))]
pub struct FTypedElementU64IntValueCacheColumn {
    pub value: u64,
}
#[repr(C, align(8))]
pub struct FTypedElementI64IntValueCacheColumn {
    pub value: i64,
}
#[repr(C, align(4))]
pub struct FTypedElementFloatValueCacheColumn {
    pub value: f32,
}
#[repr(C, align(8))]
pub struct FWidgetFactoryColumn {}
#[repr(C, align(8))]
pub struct FTest_PingPongPrePhys {
    pub value: u64,
}
#[repr(C, align(8))]
pub struct FTest_PingPongDurPhys {
    pub value: u64,
}
#[repr(C, align(8))]
pub struct FTest_PingPongPostPhys {
    pub value: u64,
}
#[repr(C, align(1))]
pub struct FUniversalObjectLocatorEmptyPayload {}
#[repr(C, align(1))]
pub struct FEngineServicePing {}
#[repr(C, align(1))]
pub struct FTraceControlStatusPing {}
#[repr(C, align(1))]
pub struct FTraceControlSettingsPing {}
#[repr(C, align(1))]
pub struct FTraceControlStop {}
#[repr(C, align(1))]
pub struct FTraceControlPause {}
#[repr(C, align(1))]
pub struct FTraceControlResume {}
#[repr(C, align(1))]
pub struct FSwarmPingMessage {}
#[repr(C, align(4))]
pub struct FHeaderWidgetSizeColumn {
    pub width: f32,
}
#[repr(C, align(1))]
pub struct FDataflowAnyType {}
#[repr(C, align(1))]
pub struct FDataflowAllTypes {}
#[repr(C, align(1))]
pub struct FDataflowArrayTypes {}
#[repr(C, align(1))]
pub struct FDataflowFreezeActions {}
#[repr(C, align(4))]
pub struct FTransformGizmoHandlePlacement {}
#[repr(C, align(8))]
pub struct FGizmoHandle {}
#[repr(C, align(1))]
pub struct FGameplayTagCreationWidgetHelper {}
#[repr(C, align(8))]
pub struct FDataflowSimulationProperty {}
#[repr(C, align(1))]
pub struct FChaosVDStopRecordingCommandMessage {}
#[repr(C, align(1))]
pub struct FChaosVDFullSessionInfoRequestMessage {}
#[repr(C, align(1))]
pub struct FGeometryCollectionDebugDrawWarningMessage {}
#[repr(C, align(4))]
pub struct FRichCurveKey {
    pub interp_mode: UNKOWN,
    pub tangent_mode: UNKOWN,
    pub tangent_weight_mode: UNKOWN,
    pub time: f32,
    pub value: f32,
    pub arrive_tangent: f32,
    pub arrive_tangent_weight: f32,
    pub leave_tangent: f32,
    pub leave_tangent_weight: f32,
}
#[repr(C, align(1))]
pub struct FWidgetEventField {}
#[repr(C, align(1))]
pub struct FSubobjectDataSubsystemContextDataBase {}
#[repr(C, align(8))]
pub struct FAnimPhysSimSpaceSettings {
    pub sim_space_angular_alpha: f32,
    pub max_angular_velocity: f32,
    pub max_angular_acceleration: f32,
    pub external_angular_velocity: FVector,
}
#[repr(C, align(8))]
pub struct FSimSpaceSettings {
    pub world_alpha: f32,
    pub velocity_scale_z: f32,
    pub damping_alpha: f32,
    pub max_linear_velocity: f32,
    pub max_angular_velocity: f32,
    pub max_linear_acceleration: f32,
    pub max_angular_acceleration: f32,
    pub external_linear_drag: f32,
    pub external_linear_drag_v: FVector,
    pub external_linear_velocity: FVector,
    pub external_angular_velocity: FVector,
}
#[repr(C, align(4))]
pub struct FMovieSceneFloatValue {
    pub value: f32,
    pub tangent: FMovieSceneTangentData,
    pub interp_mode: UNKOWN,
    pub tangent_mode: UNKOWN,
    pub padding_byte: u8,
}
#[repr(C, align(4))]
pub struct FMovieSceneTangentData {
    pub arrive_tangent: f32,
    pub leave_tangent: f32,
    pub arrive_tangent_weight: f32,
    pub leave_tangent_weight: f32,
    pub tangent_weight_mode: UNKOWN,
}
#[repr(C, align(1))]
pub struct FMovieSceneEmptyStruct {}
#[repr(C, align(8))]
pub struct FMovieSceneDoubleValue {
    pub value: f64,
    pub tangent: FMovieSceneTangentData,
    pub interp_mode: UNKOWN,
    pub tangent_mode: UNKOWN,
    pub padding_byte: u8,
}
#[repr(C, align(8))]
pub struct FVector_NetQuantize100 {}
#[repr(C, align(1))]
pub struct FEmptyPayload {}
#[repr(C, align(1))]
pub struct FBookmark2DJumpToSettings {}
#[repr(C, align(1))]
pub struct FBookmarkBaseJumpToSettings {}
#[repr(C, align(4))]
pub struct FSimpleCurveKey {
    pub time: f32,
    pub value: f32,
}
#[repr(C, align(8))]
pub struct FVector_NetQuantize10 {}
#[repr(C, align(8))]
pub struct FOverlapInfo {
    pub from_sweep: bool,
    pub overlap_info: FHitResult,
}
#[repr(C, align(1))]
pub struct FAnimBlueprintMutableData {}
#[repr(C, align(1))]
pub struct FAnimBlueprintConstantData {}
#[repr(C, align(1))]
pub struct FDummySpacerCameraTypes {}
#[repr(C, align(4))]
pub struct FNameCurveKey {
    pub time: f32,
    pub value: FName,
}
#[repr(C, align(8))]
pub struct FOverlapResult {
    pub overlap_object_handle: FActorInstanceHandle,
    pub component: UNKOWN,
    pub blocking_hit: bool,
}
#[repr(C, align(8))]
pub struct FStreamableTextureInstance {}
#[repr(C, align(16))]
pub struct FParticleSysParam {
    pub name: FName,
    pub param_type: UNKOWN,
    pub scalar: f32,
    pub scalar_low: f32,
    pub vector: FVector,
    pub vector_low: FVector,
    pub color: FColor,
    pub actor: AActor,
    pub material: UMaterialInterface,
}
#[repr(C, align(1))]
pub struct FNiagaraRequestSimpleClientInfoMessage {}
#[repr(C, align(1))]
pub struct FNiagaraWildcard {}
#[repr(C, align(1))]
pub struct FNiagaraNumeric {}
#[repr(C, align(1))]
pub struct FNiagaraParameterMap {}
#[repr(C, align(4))]
pub struct FGeometryCacheMeshBatchInfo {}
#[repr(C, align(1))]
pub struct FMassFragment {}
#[repr(C, align(1))]
pub struct FMassTag {}
#[repr(C, align(1))]
pub struct FMassChunkFragment {}
#[repr(C, align(1))]
pub struct FMassSharedFragment {}
#[repr(C, align(1))]
pub struct FMassConstSharedFragment {}
#[repr(C, align(1))]
pub struct FMassRelation {}
#[repr(C, align(1))]
pub struct FMassRelationMappingFragment {}
#[repr(C, align(1))]
pub struct FProcessorAuxDataBase {}
#[repr(C, align(1))]
pub struct FMassChildOfRelation {}
#[repr(C, align(1))]
pub struct FTestFragment_Tag {}
#[repr(C, align(1))]
pub struct FTestTag_A {}
#[repr(C, align(1))]
pub struct FTestTag_B {}
#[repr(C, align(1))]
pub struct FTestTag_C {}
#[repr(C, align(1))]
pub struct FTestTag_D {}
#[repr(C, align(1))]
pub struct FFarmJustBecameReadyToHarvestTag {}
#[repr(C, align(1))]
pub struct FFarmReadyToHarvestTag {}
#[repr(C, align(1))]
pub struct FHierarchyElementChangedPayload {}
#[repr(C, align(1))]
pub struct FTaggedAssetBrowserConfigurationDataBase {}
#[repr(C, align(1))]
pub struct FTaggedAssetBrowserConfigurationData_Extension {}
#[repr(C, align(1))]
pub struct FNiagaraParametersChangedData {}
#[repr(C, align(1))]
pub struct FRigHierarchyRef {}
#[repr(C, align(4))]
pub struct FMetasoundCommentNodeIntVector {}
#[repr(C, align(4))]
pub struct FStateTreeCompareIntConditionInstanceData {
    pub left: i32,
    pub right: i32,
}
#[repr(C, align(8))]
pub struct FStateTreeCompareFloatConditionInstanceData {
    pub left: f64,
    pub right: f64,
}
#[repr(C, align(1))]
pub struct FStateTreeCompareBoolConditionInstanceData {
    pub left: bool,
    pub right: bool,
}
#[repr(C, align(8))]
pub struct FStateTreeCompareEnumConditionInstanceData {
    pub left: FStateTreeAnyEnum,
    pub right: FStateTreeAnyEnum,
}
#[repr(C, align(8))]
pub struct FStateTreeCompareDistanceConditionInstanceData {
    pub source: FVector,
    pub target: FVector,
    pub distance: f64,
}
#[repr(C, align(4))]
pub struct FStateTreeCompareNameConditionInstanceData {
    pub left: FName,
    pub right: FName,
}
#[repr(C, align(4))]
pub struct FStateTreeRandomConditionInstanceData {
    pub threshold: f32,
}
#[repr(C, align(1))]
pub struct FStateItemCustomData {}
#[repr(C, align(1))]
pub struct FStateTreeTransitionDelegateListener {}
#[repr(C, align(1))]
pub struct FTestTask_StopTreeInstanceData {}
#[repr(C, align(4))]
pub struct FStateTreeTestConditionInstanceData {
    pub count: i32,
}
#[repr(C, align(4))]
pub struct FTestPropertyFunction_InstanceData {
    pub input: i32,
    pub result: i32,
}
#[repr(C, align(1))]
pub struct FStateTreeTestBooleanConditionInstanceData {
    pub success: bool,
}
#[repr(C, align(1))]
pub struct FTestTask_ListenerOnNode_InstanceData {}
#[repr(C, align(4))]
pub struct FBoneIndexControlAttributeMapping {}
#[repr(C, align(8))]
pub struct FCameraFramingZone {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
#[repr(C, align(8))]
pub struct FTedsChildAlertColumn {}
#[repr(C, align(1))]
pub struct FTedsAlertChainTag {}
#[repr(C, align(1))]
pub struct FTedsUnsortedAlertChainTag {}
#[repr(C, align(1))]
pub struct FAlertWidgetTag {}
#[repr(C, align(1))]
pub struct FAlertHeaderWidgetTag {}
#[repr(C, align(1))]
pub struct FAlertHeaderActiveWidgetTag {}
#[repr(C, align(1))]
pub struct FActorComponentTypeTag {}
#[repr(C, align(1))]
pub struct FFolderExpandedTag {}
#[repr(C, align(1))]
pub struct FDataflowConstructionObjectTag {}
#[repr(C, align(1))]
pub struct FDataflowSimulationObjectTag {}
#[repr(C, align(1))]
pub struct FDataflowSceneObjectTag {}
#[repr(C, align(1))]
pub struct FDataflowSceneStructTag {}
#[repr(C, align(1))]
pub struct FChaosVDSelectionContext {}
#[repr(C, align(1))]
pub struct FChaosVDObjectDataTag {}
#[repr(C, align(1))]
pub struct FTypedElementFromCVDWorldTag {}
#[repr(C, align(1))]
pub struct FChaosVDActiveObjectTag {}
#[repr(C, align(1))]
pub struct FImgMediaSourceCustomizationSequenceProxy {}
#[repr(C, align(1))]
pub struct FEditorDataHierarchyParentTag_Template {}
#[repr(C, align(1))]
pub struct FEditorDataHierarchyChildTag_Template {}
#[repr(C, align(1))]
pub struct FTestDynamicTag {}
#[repr(C, align(1))]
pub struct FTedsSharedColumn {}
#[repr(C, align(1))]
pub struct FTypedElementMementoTag {}
#[repr(C, align(1))]
pub struct FExportedTextWidgetTag {}
#[repr(C, align(1))]
pub struct FFolderTypeColumn_Experimental {}
#[repr(C, align(1))]
pub struct FAssetTag {}
#[repr(C, align(1))]
pub struct FPrivateAssetTag {}
#[repr(C, align(1))]
pub struct FEpicInternalAssetTag {}
#[repr(C, align(1))]
pub struct FPublicAssetTag {}
#[repr(C, align(1))]
pub struct FUpdatedPathTag {}
#[repr(C, align(1))]
pub struct FUpdatedAssetDataTag {}
#[repr(C, align(1))]
pub struct FThumbnailSizeColumn_Experimental {}
#[repr(C, align(1))]
pub struct FTextOverflowPolicyColumn_Experimental {}
#[repr(C, align(1))]
pub struct FSettingsContainerTag {}
#[repr(C, align(1))]
pub struct FSettingsCategoryTag {}
#[repr(C, align(1))]
pub struct FSettingsSectionTag {}
#[repr(C, align(1))]
pub struct FSettingsInactiveSectionTag {}
#[repr(C, align(1))]
pub struct FDataStorageTypeInfoTag {}
#[repr(C, align(1))]
pub struct FDataStorageClassTypeInfoTag {}
#[repr(C, align(1))]
pub struct FDataStorageStructTypeInfoTag {}
#[repr(C, align(1))]
pub struct FDataStorageTypeInfoInterfaceTag {}
#[repr(C, align(1))]
pub struct FDataStorageVerseTypeInfoTag {}
#[repr(C, align(1))]
pub struct FDataStorageVerseTypeInfoAccessLevel {}
#[repr(C, align(1))]
pub struct FDataStorageTypeInfoRequiresHierarchyUpdateTag {}
#[repr(C, align(8))]
pub struct FEntity {
    pub id: u64,
}
#[repr(C, align(1))]
pub struct FFabDistributionMethodTag {}
#[repr(C, align(1))]
pub struct FTestStructSimpleBase {}
#[repr(C, align(1))]
pub struct FSessionServiceLogSubscribe {}
#[repr(C, align(1))]
pub struct FSessionServiceLogUnsubscribe {}
