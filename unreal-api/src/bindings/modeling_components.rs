#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FModelingToolsAxisFilter {
    __padding_end: [u8; 3],
}
impl FModelingToolsAxisFilter {}
#[repr(C, align(1))]
pub struct FModelingToolsColorChannelFilter {
    __padding_end: [u8; 4],
}
impl FModelingToolsColorChannelFilter {}
#[repr(C, align(16))]
pub struct FCreateMeshObjectParams {
    __padding_end: [u8; 1712],
}
impl FCreateMeshObjectParams {}
#[repr(C, align(8))]
pub struct FCreateMeshObjectResult {
    __padding_end: [u8; 32],
}
impl FCreateMeshObjectResult {}
#[repr(C, align(8))]
pub struct FCreateTextureObjectParams {
    __padding_end: [u8; 64],
}
impl FCreateTextureObjectParams {}
#[repr(C, align(8))]
pub struct FCreateTextureObjectResult {
    __padding_end: [u8; 16],
}
impl FCreateTextureObjectResult {}
#[repr(C, align(8))]
pub struct FCreateMaterialObjectParams {
    __padding_end: [u8; 40],
}
impl FCreateMaterialObjectParams {}
#[repr(C, align(8))]
pub struct FCreateMaterialObjectResult {
    __padding_end: [u8; 16],
}
impl FCreateMaterialObjectResult {}
#[repr(C, align(16))]
pub struct FCreateActorParams {
    __padding_end: [u8; 144],
}
impl FCreateActorParams {}
#[repr(C, align(8))]
pub struct FCreateActorResult {
    __padding_end: [u8; 16],
}
impl FCreateActorResult {}
#[repr(C, align(8))]
pub struct FCreateComponentParams {
    __padding_end: [u8; 40],
}
impl FCreateComponentParams {}
#[repr(C, align(8))]
pub struct FCreateComponentResult {
    __padding_end: [u8; 16],
}
impl FCreateComponentResult {}
pub struct UDynamicMeshProvider {}
pub struct IDynamicMeshProvider {}
pub struct UDynamicMeshCommitter {}
pub struct IDynamicMeshCommitter {}
pub struct UPersistentDynamicMeshSource {}
pub struct IPersistentDynamicMeshSource {}
#[repr(C, align(8))]
pub struct UInteractiveToolActivity {
    __padding_end: [u8; 56],
}
impl UInteractiveToolActivity {}
pub struct UToolActivityHost {}
pub struct IToolActivityHost {}
pub struct ULatticeStateStorage {}
pub struct ILatticeStateStorage {}
pub struct UMeshSculptLayersManager {}
pub struct IMeshSculptLayersManager {}
pub struct UModelingToolExternalDynamicMeshUpdateAPI {}
pub struct IModelingToolExternalDynamicMeshUpdateAPI {}
#[repr(C, align(8))]
pub struct UGeometrySelectionEditCommandArguments {
    __padding_end: [u8; 96],
}
impl UGeometrySelectionEditCommandArguments {}
#[repr(C, align(8))]
pub struct UGeometrySelectionEditCommandResult {
    __padding_end: [u8; 168],
}
impl UGeometrySelectionEditCommandResult {}
#[repr(C, align(8))]
pub struct UGeometrySelectionEditCommand {
    __padding_end: [u8; 48],
}
impl UGeometrySelectionEditCommand {}
#[repr(C, align(8))]
pub struct UVoxelProperties {
    __padding_end: [u8; 208],
}
impl UVoxelProperties {}
#[repr(C, align(8))]
pub struct UVolumetricBrushStampIndicatorBuilder {
    __padding_end: [u8; 48],
}
impl UVolumetricBrushStampIndicatorBuilder {}
#[repr(C, align(8))]
pub struct UVolumetricBrushStampIndicator {
    __padding_end: [u8; 232],
}
impl UVolumetricBrushStampIndicator {}
#[repr(C, align(8))]
pub struct UMultiSelectionMeshEditingToolBuilder {
    __padding_end: [u8; 48],
}
impl UMultiSelectionMeshEditingToolBuilder {}
#[repr(C, align(8))]
pub struct UBaseCreateFromSelectedToolBuilder {
    __padding_end: [u8; 48],
}
impl UBaseCreateFromSelectedToolBuilder {}
#[repr(C, align(8))]
pub struct UOnAcceptHandleSourcesPropertiesBase {
    __padding_end: [u8; 184],
}
impl UOnAcceptHandleSourcesPropertiesBase {}
#[repr(C, align(8))]
pub struct UOnAcceptHandleSourcesProperties {
    __padding_end: [u8; 192],
}
impl UOnAcceptHandleSourcesProperties {}
#[repr(C, align(8))]
pub struct UBaseCreateFromSelectedHandleSourceProperties {
    __padding_end: [u8; 232],
}
impl UBaseCreateFromSelectedHandleSourceProperties {}
#[repr(C, align(8))]
pub struct UBaseCreateFromSelectedCollisionProperties {
    __padding_end: [u8; 192],
}
impl UBaseCreateFromSelectedCollisionProperties {}
#[repr(C, align(8))]
pub struct UTransformInputsToolProperties {
    __padding_end: [u8; 192],
}
impl UTransformInputsToolProperties {}
#[repr(C, align(8))]
pub struct UMultiSelectionMeshEditingTool {
    __padding_end: [u8; 232],
}
impl UMultiSelectionMeshEditingTool {}
#[repr(C, align(8))]
pub struct UBaseCreateFromSelectedTool {
    __padding_end: [u8; 312],
}
impl UBaseCreateFromSelectedTool {}
#[repr(C, align(8))]
pub struct USingleTargetWithSelectionToolBuilder {
    __padding_end: [u8; 48],
}
impl USingleTargetWithSelectionToolBuilder {}
#[repr(C, align(8))]
pub struct UBaseMeshProcessingToolBuilder {
    __padding_end: [u8; 48],
}
impl UBaseMeshProcessingToolBuilder {}
#[repr(C, align(8))]
pub struct USingleTargetWithSelectionTool {
    __padding_end: [u8; 336],
}
impl USingleTargetWithSelectionTool {}
#[repr(C, align(16))]
pub struct UBaseMeshProcessingTool {
    __padding_end: [u8; 1184],
}
impl UBaseMeshProcessingTool {}
#[repr(C, align(8))]
pub struct UBaseVoxelTool {
    __padding_end: [u8; 336],
}
impl UBaseVoxelTool {}
#[repr(C, align(8))]
pub struct UMeshSurfacePointMeshEditingToolBuilder {
    __padding_end: [u8; 56],
}
impl UMeshSurfacePointMeshEditingToolBuilder {}
#[repr(C, align(8))]
pub struct UMultiTargetWithSelectionToolBuilder {
    __padding_end: [u8; 48],
}
impl UMultiTargetWithSelectionToolBuilder {}
#[repr(C, align(8))]
pub struct UMultiTargetWithSelectionTool {
    __padding_end: [u8; 280],
}
impl UMultiTargetWithSelectionTool {}
#[repr(C, align(8))]
pub struct USingleSelectionMeshEditingToolBuilder {
    __padding_end: [u8; 48],
}
impl USingleSelectionMeshEditingToolBuilder {}
#[repr(C, align(8))]
pub struct USingleSelectionMeshEditingTool {
    __padding_end: [u8; 224],
}
impl USingleSelectionMeshEditingTool {}
#[repr(C, align(8))]
pub struct UDynamicMeshReplacementChangeTarget {
    __padding_end: [u8; 104],
}
impl UDynamicMeshReplacementChangeTarget {}
#[repr(C, align(16))]
pub struct UOctreeDynamicMeshComponent {
    __padding_end: [u8; 1984],
}
impl UOctreeDynamicMeshComponent {}
#[repr(C, align(16))]
pub struct ULineSetComponent {
    __padding_end: [u8; 1712],
}
impl ULineSetComponent {}
#[repr(C, align(8))]
pub struct UMeshElementsVisualizerProperties {
    __padding_end: [u8; 232],
}
impl UMeshElementsVisualizerProperties {}
#[repr(C, align(8))]
pub struct UPreviewGeometry {
    __padding_end: [u8; 296],
}
impl UPreviewGeometry {}
#[repr(C, align(8))]
pub struct UMeshElementsVisualizer {
    __padding_end: [u8; 336],
}
impl UMeshElementsVisualizer {}
#[repr(C, align(16))]
pub struct UMeshWireframeComponent {
    __padding_end: [u8; 1744],
}
impl UMeshWireframeComponent {}
#[repr(C, align(16))]
pub struct UPointSetComponent {
    __padding_end: [u8; 1712],
}
impl UPointSetComponent {}
#[repr(C, align(16))]
pub struct UPreviewMesh {
    __padding_end: [u8; 304],
}
impl UPreviewMesh {}
#[repr(C, align(16))]
pub struct UPolyEditPreviewMesh {
    __padding_end: [u8; 1152],
}
impl UPolyEditPreviewMesh {}
#[repr(C, align(8))]
pub struct APreviewGeometryActor {
    __padding_end: [u8; 1144],
}
impl APreviewGeometryActor {}
#[repr(C, align(16))]
pub struct UTriangleSetComponent {
    __padding_end: [u8; 1840],
}
impl UTriangleSetComponent {}
#[repr(C, align(8))]
pub struct UUVLayoutPreviewProperties {
    __padding_end: [u8; 224],
}
impl UUVLayoutPreviewProperties {}
#[repr(C, align(16))]
pub struct UUVLayoutPreview {
    __padding_end: [u8; 384],
}
impl UUVLayoutPreview {}
#[repr(C, align(16))]
pub struct UCollectSurfacePathMechanic {
    __padding_end: [u8; 1472],
}
impl UCollectSurfacePathMechanic {}
#[repr(C, align(16))]
pub struct UCollisionPrimitivesMechanic {
    __padding_end: [u8; 1456],
}
impl UCollisionPrimitivesMechanic {}
#[repr(C, align(16))]
pub struct UConstructionPlaneMechanic {
    __padding_end: [u8; 256],
}
impl UConstructionPlaneMechanic {}
#[repr(C, align(16))]
pub struct UCurveControlPointsMechanic {
    __padding_end: [u8; 1664],
}
impl UCurveControlPointsMechanic {}
#[repr(C, align(16))]
pub struct UDragAlignmentMechanic {
    __padding_end: [u8; 640],
}
impl UDragAlignmentMechanic {}
#[repr(C, align(16))]
pub struct UDragAlignmentInteraction {
    __padding_end: [u8; 736],
}
impl UDragAlignmentInteraction {}
#[repr(C, align(16))]
pub struct ULatticeControlPointsMechanic {
    __padding_end: [u8; 1296],
}
impl ULatticeControlPointsMechanic {}
#[repr(C, align(16))]
pub struct UPlaneDistanceFromHitMechanic {
    __padding_end: [u8; 1312],
}
impl UPlaneDistanceFromHitMechanic {}
#[repr(C, align(16))]
pub struct UPolyLassoMarqueeMechanic {
    __padding_end: [u8; 512],
}
impl UPolyLassoMarqueeMechanic {}
#[repr(C, align(16))]
pub struct URectangleMarqueeMechanic {
    __padding_end: [u8; 576],
}
impl URectangleMarqueeMechanic {}
#[repr(C, align(16))]
pub struct URectangleMarqueeInteraction {
    __padding_end: [u8; 592],
}
impl URectangleMarqueeInteraction {}
#[repr(C, align(8))]
pub struct USpaceCurveDeformationMechanicPropertySet {
    __padding_end: [u8; 200],
}
impl USpaceCurveDeformationMechanicPropertySet {}
#[repr(C, align(16))]
pub struct USpaceCurveDeformationMechanic {
    __padding_end: [u8; 736],
}
impl USpaceCurveDeformationMechanic {}
#[repr(C, align(16))]
pub struct USpatialCurveDistanceMechanic {
    __padding_end: [u8; 1040],
}
impl USpatialCurveDistanceMechanic {}
#[repr(C, align(8))]
pub struct UMeshOpPreviewWithBackgroundCompute {
    __padding_end: [u8; 216],
}
impl UMeshOpPreviewWithBackgroundCompute {}
#[repr(C, align(8))]
pub struct UModelingComponentsSettings {
    __padding_end: [u8; 112],
}
impl UModelingComponentsSettings {}
#[repr(C, align(8))]
pub struct UModelingComponentsEditorSettings {
    __padding_end: [u8; 128],
}
impl UModelingComponentsEditorSettings {}
#[repr(C, align(8))]
pub struct UModelingObjectsCreationAPI {
    __padding_end: [u8; 48],
}
impl UModelingObjectsCreationAPI {}
#[repr(C, align(8))]
pub struct APreviewMeshActor {
    __padding_end: [u8; 1144],
}
impl APreviewMeshActor {}
#[repr(C, align(8))]
pub struct UCreateMeshObjectTypeProperties {
    __padding_end: [u8; 232],
}
impl UCreateMeshObjectTypeProperties {}
#[repr(C, align(8))]
pub struct UGeometrySelectionVisualizationProperties {
    __padding_end: [u8; 280],
}
impl UGeometrySelectionVisualizationProperties {}
#[repr(C, align(8))]
pub struct UOnAcceptHandleSourcesPropertiesSingle {
    __padding_end: [u8; 192],
}
impl UOnAcceptHandleSourcesPropertiesSingle {}
#[repr(C, align(8))]
pub struct UPolygroupLayersProperties {
    __padding_end: [u8; 216],
}
impl UPolygroupLayersProperties {}
#[repr(C, align(8))]
pub struct UWeightMapSetProperties {
    __padding_end: [u8; 224],
}
impl UWeightMapSetProperties {}
#[repr(C, align(16))]
pub struct UMeshTopologySelectionMechanic {
    __padding_end: [u8; 2880],
}
impl UMeshTopologySelectionMechanic {}
#[repr(C, align(16))]
pub struct UBoundarySelectionMechanic {
    __padding_end: [u8; 2880],
}
impl UBoundarySelectionMechanic {}
#[repr(C, align(8))]
pub struct UGeometrySelectionManager {
    __padding_end: [u8; 1240],
}
impl UGeometrySelectionManager {}
#[repr(C, align(8))]
pub struct UMeshTopologySelectionMechanicProperties {
    __padding_end: [u8; 208],
}
impl UMeshTopologySelectionMechanicProperties {}
#[repr(C, align(8))]
pub struct UDEPRECATED_PolygonSelectionMechanicProperties {
    __padding_end: [u8; 208],
}
impl UDEPRECATED_PolygonSelectionMechanicProperties {}
#[repr(C, align(16))]
pub struct UPolygonSelectionMechanic {
    __padding_end: [u8; 2880],
}
impl UPolygonSelectionMechanic {}
#[repr(C, align(8))]
pub struct UModelingSceneSnappingManager {
    __padding_end: [u8; 344],
}
impl UModelingSceneSnappingManager {}
pub struct UToolHostCustomizationAPI {}
pub struct IToolHostCustomizationAPI {}
#[repr(C, align(16))]
pub struct UMultiTransformer {
    __padding_end: [u8; 432],
}
impl UMultiTransformer {}
#[repr(transparent)]
pub struct ECreateObjectTypeHint(pub u8);
impl ECreateObjectTypeHint {
    pub const UNDEFINED: ECreateObjectTypeHint = ECreateObjectTypeHint(0);
    pub const STATIC_MESH: ECreateObjectTypeHint = ECreateObjectTypeHint(1);
    pub const VOLUME: ECreateObjectTypeHint = ECreateObjectTypeHint(2);
    pub const DYNAMIC_MESH_ACTOR: ECreateObjectTypeHint = ECreateObjectTypeHint(3);
}
#[repr(transparent)]
pub struct ECreateModelingObjectResult(pub u8);
impl ECreateModelingObjectResult {
    pub const OK: ECreateModelingObjectResult = ECreateModelingObjectResult(0);
    pub const CANCELLED: ECreateModelingObjectResult = ECreateModelingObjectResult(1);
    pub const FAILED_UNKNOWN: ECreateModelingObjectResult = ECreateModelingObjectResult(
        2,
    );
    pub const FAILED_NO_API_FOUND: ECreateModelingObjectResult = ECreateModelingObjectResult(
        3,
    );
    pub const FAILED_INVALID_WORLD: ECreateModelingObjectResult = ECreateModelingObjectResult(
        4,
    );
    pub const FAILED_INVALID_MESH: ECreateModelingObjectResult = ECreateModelingObjectResult(
        5,
    );
    pub const FAILED_INVALID_TEXTURE: ECreateModelingObjectResult = ECreateModelingObjectResult(
        6,
    );
    pub const FAILED_ASSET_CREATION_FAILED: ECreateModelingObjectResult = ECreateModelingObjectResult(
        7,
    );
    pub const FAILED_ACTOR_CREATION_FAILED: ECreateModelingObjectResult = ECreateModelingObjectResult(
        8,
    );
    pub const FAILED_INVALID_MATERIAL: ECreateModelingObjectResult = ECreateModelingObjectResult(
        9,
    );
    pub const FAILED_INVALID_ACTOR: ECreateModelingObjectResult = ECreateModelingObjectResult(
        10,
    );
}
#[repr(transparent)]
pub struct EHandleSourcesMethod(pub u8);
impl EHandleSourcesMethod {
    pub const DELETE_SOURCES: EHandleSourcesMethod = EHandleSourcesMethod(0);
    pub const HIDE_SOURCES: EHandleSourcesMethod = EHandleSourcesMethod(1);
    pub const KEEP_SOURCES: EHandleSourcesMethod = EHandleSourcesMethod(2);
    pub const KEEP_FIRST_SOURCE: EHandleSourcesMethod = EHandleSourcesMethod(3);
    pub const KEEP_LAST_SOURCE: EHandleSourcesMethod = EHandleSourcesMethod(4);
}
#[repr(transparent)]
pub struct EBaseCreateFromSelectedTargetType(pub i32);
impl EBaseCreateFromSelectedTargetType {
    pub const NEW_OBJECT: EBaseCreateFromSelectedTargetType = EBaseCreateFromSelectedTargetType(
        0,
    );
    pub const FIRST_INPUT_OBJECT: EBaseCreateFromSelectedTargetType = EBaseCreateFromSelectedTargetType(
        1,
    );
    pub const LAST_INPUT_OBJECT: EBaseCreateFromSelectedTargetType = EBaseCreateFromSelectedTargetType(
        2,
    );
}
#[repr(transparent)]
pub struct EUVLayoutPreviewSide(pub i32);
impl EUVLayoutPreviewSide {
    pub const LEFT: EUVLayoutPreviewSide = EUVLayoutPreviewSide(0);
    pub const RIGHT: EUVLayoutPreviewSide = EUVLayoutPreviewSide(1);
}
#[repr(transparent)]
pub struct ESpaceCurveControlPointTransformMode(pub i32);
impl ESpaceCurveControlPointTransformMode {
    pub const SHARED: ESpaceCurveControlPointTransformMode = ESpaceCurveControlPointTransformMode(
        0,
    );
    pub const PER_VERTEX: ESpaceCurveControlPointTransformMode = ESpaceCurveControlPointTransformMode(
        1,
    );
}
#[repr(transparent)]
pub struct ESpaceCurveControlPointOriginMode(pub i32);
impl ESpaceCurveControlPointOriginMode {
    pub const SHARED: ESpaceCurveControlPointOriginMode = ESpaceCurveControlPointOriginMode(
        0,
    );
    pub const FIRST: ESpaceCurveControlPointOriginMode = ESpaceCurveControlPointOriginMode(
        1,
    );
    pub const LAST: ESpaceCurveControlPointOriginMode = ESpaceCurveControlPointOriginMode(
        2,
    );
}
#[repr(transparent)]
pub struct ESpaceCurveControlPointFalloffType(pub i32);
impl ESpaceCurveControlPointFalloffType {
    pub const LINEAR: ESpaceCurveControlPointFalloffType = ESpaceCurveControlPointFalloffType(
        0,
    );
    pub const SMOOTH: ESpaceCurveControlPointFalloffType = ESpaceCurveControlPointFalloffType(
        1,
    );
}
#[repr(transparent)]
pub struct EModelingComponentsPlaneVisualizationMode(pub u8);
impl EModelingComponentsPlaneVisualizationMode {
    pub const SIMPLE_GRID: EModelingComponentsPlaneVisualizationMode = EModelingComponentsPlaneVisualizationMode(
        0,
    );
    pub const HIERARCHICAL_GRID: EModelingComponentsPlaneVisualizationMode = EModelingComponentsPlaneVisualizationMode(
        1,
    );
    pub const FIXED_SCREEN_AREA_GRID: EModelingComponentsPlaneVisualizationMode = EModelingComponentsPlaneVisualizationMode(
        2,
    );
}
#[repr(transparent)]
pub struct EGeometrySelectionElementType(pub u8);
impl EGeometrySelectionElementType {
    pub const VERTEX: EGeometrySelectionElementType = EGeometrySelectionElementType(1);
    pub const EDGE: EGeometrySelectionElementType = EGeometrySelectionElementType(2);
    pub const FACE: EGeometrySelectionElementType = EGeometrySelectionElementType(4);
}
#[repr(transparent)]
pub struct EGeometrySelectionTopologyType(pub u8);
impl EGeometrySelectionTopologyType {
    pub const TRIANGLE: EGeometrySelectionTopologyType = EGeometrySelectionTopologyType(
        1,
    );
    pub const POLYGROUP: EGeometrySelectionTopologyType = EGeometrySelectionTopologyType(
        2,
    );
}
#[repr(transparent)]
pub struct EMarqueeSelectionUpdateType(pub i32);
impl EMarqueeSelectionUpdateType {
    pub const ON_DRAG: EMarqueeSelectionUpdateType = EMarqueeSelectionUpdateType(0);
    pub const ON_TICK_AND_RELEASE: EMarqueeSelectionUpdateType = EMarqueeSelectionUpdateType(
        1,
    );
    pub const ON_RELEASE: EMarqueeSelectionUpdateType = EMarqueeSelectionUpdateType(2);
}
#[repr(transparent)]
pub struct ELatticeInterpolationType(pub u8);
impl ELatticeInterpolationType {
    pub const LINEAR: ELatticeInterpolationType = ELatticeInterpolationType(0);
    pub const CUBIC: ELatticeInterpolationType = ELatticeInterpolationType(1);
}
#[repr(transparent)]
pub struct EBakeTextureResolution(pub i32);
impl EBakeTextureResolution {
    pub const RESOLUTION16: EBakeTextureResolution = EBakeTextureResolution(16);
    pub const RESOLUTION32: EBakeTextureResolution = EBakeTextureResolution(32);
    pub const RESOLUTION64: EBakeTextureResolution = EBakeTextureResolution(64);
    pub const RESOLUTION128: EBakeTextureResolution = EBakeTextureResolution(128);
    pub const RESOLUTION256: EBakeTextureResolution = EBakeTextureResolution(256);
    pub const RESOLUTION512: EBakeTextureResolution = EBakeTextureResolution(512);
    pub const RESOLUTION1024: EBakeTextureResolution = EBakeTextureResolution(1024);
    pub const RESOLUTION2048: EBakeTextureResolution = EBakeTextureResolution(2048);
    pub const RESOLUTION4096: EBakeTextureResolution = EBakeTextureResolution(4096);
    pub const RESOLUTION8192: EBakeTextureResolution = EBakeTextureResolution(8192);
}
#[repr(transparent)]
pub struct EBakeTextureBitDepth(pub i32);
impl EBakeTextureBitDepth {
    pub const CHANNEL_BITS8: EBakeTextureBitDepth = EBakeTextureBitDepth(0);
    pub const CHANNEL_BITS16: EBakeTextureBitDepth = EBakeTextureBitDepth(1);
}
#[repr(transparent)]
pub struct EBakeTextureSamplesPerPixel(pub i32);
impl EBakeTextureSamplesPerPixel {
    pub const SAMPLE1: EBakeTextureSamplesPerPixel = EBakeTextureSamplesPerPixel(1);
    pub const SAMPLE4: EBakeTextureSamplesPerPixel = EBakeTextureSamplesPerPixel(4);
    pub const SAMPLE16: EBakeTextureSamplesPerPixel = EBakeTextureSamplesPerPixel(16);
    pub const SAMPLE64: EBakeTextureSamplesPerPixel = EBakeTextureSamplesPerPixel(64);
    pub const SAMPLE256: EBakeTextureSamplesPerPixel = EBakeTextureSamplesPerPixel(256);
}
