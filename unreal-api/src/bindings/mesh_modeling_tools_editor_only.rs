#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UToolMeshSelector {
    __padding_end: [u8; 112],
}
impl UToolMeshSelector {}
#[repr(C, align(8))]
pub struct UAttributeEditorToolBuilder {
    __padding_end: [u8; 48],
}
impl UAttributeEditorToolBuilder {}
#[repr(C, align(8))]
pub struct UAttributeEditorAttribProperties {
    __padding_end: [u8; 280],
}
impl UAttributeEditorAttribProperties {}
#[repr(C, align(8))]
pub struct UAttributeEditorActionPropertySet {
    __padding_end: [u8; 192],
}
impl UAttributeEditorActionPropertySet {}
#[repr(C, align(8))]
pub struct UAttributeEditorNormalsActions {
    __padding_end: [u8; 192],
}
impl UAttributeEditorNormalsActions {}
#[repr(C, align(8))]
pub struct UAttributeEditorUVActions {
    __padding_end: [u8; 224],
}
impl UAttributeEditorUVActions {}
#[repr(C, align(8))]
pub struct UAttributeEditorLightmapUVActions {
    __padding_end: [u8; 208],
}
impl UAttributeEditorLightmapUVActions {}
#[repr(C, align(8))]
pub struct UAttributeEditorNewAttributeActions {
    __padding_end: [u8; 216],
}
impl UAttributeEditorNewAttributeActions {}
#[repr(C, align(8))]
pub struct UAttributeEditorModifyAttributeActions {
    __padding_end: [u8; 224],
}
impl UAttributeEditorModifyAttributeActions {}
#[repr(C, align(8))]
pub struct UAttributeEditorCopyAttributeActions {
    __padding_end: [u8; 224],
}
impl UAttributeEditorCopyAttributeActions {}
#[repr(C, align(8))]
pub struct UAttributeEditorTool {
    __padding_end: [u8; 304],
}
impl UAttributeEditorTool {}
#[repr(C, align(8))]
pub struct UParameterizeMeshToolBuilder {
    __padding_end: [u8; 48],
}
impl UParameterizeMeshToolBuilder {}
#[repr(C, align(8))]
pub struct UParameterizeMeshTool {
    __padding_end: [u8; 336],
}
impl UParameterizeMeshTool {}
#[repr(C, align(8))]
pub struct UPolygonOnMeshToolBuilder {
    __padding_end: [u8; 48],
}
impl UPolygonOnMeshToolBuilder {}
#[repr(C, align(8))]
pub struct UPolygonOnMeshToolProperties {
    __padding_end: [u8; 224],
}
impl UPolygonOnMeshToolProperties {}
#[repr(C, align(8))]
pub struct UPolygonOnMeshToolActionPropertySet {
    __padding_end: [u8; 192],
}
impl UPolygonOnMeshToolActionPropertySet {}
#[repr(C, align(16))]
pub struct UPolygonOnMeshTool {
    __padding_end: [u8; 656],
}
impl UPolygonOnMeshTool {}
#[repr(C, align(8))]
pub struct USimplifyMeshToolBuilder {
    __padding_end: [u8; 48],
}
impl USimplifyMeshToolBuilder {}
#[repr(C, align(8))]
pub struct USimplifyMeshToolProperties {
    __padding_end: [u8; 240],
}
impl USimplifyMeshToolProperties {}
#[repr(C, align(8))]
pub struct USimplifyMeshTool {
    __padding_end: [u8; 312],
}
impl USimplifyMeshTool {}
#[repr(C, align(8))]
pub struct URefSkeletonPoser {
    __padding_end: [u8; 488],
}
impl URefSkeletonPoser {}
pub struct ISkeletalMeshEditingInterface {}
#[repr(C, align(8))]
pub struct USkeletalMeshEditingInterface {
    __padding_end: [u8; 48],
}
impl USkeletalMeshEditingInterface {}
#[repr(C, align(8))]
pub struct USkeletalMeshGizmoContextObjectBase {
    __padding_end: [u8; 48],
}
impl USkeletalMeshGizmoContextObjectBase {}
#[repr(C, align(8))]
pub struct USkeletalMeshGizmoWrapperBase {
    __padding_end: [u8; 56],
}
impl USkeletalMeshGizmoWrapperBase {}
#[repr(C, align(8))]
pub struct USkeletalMeshEditorContextObjectBase {
    __padding_end: [u8; 48],
}
impl USkeletalMeshEditorContextObjectBase {}
#[repr(C, align(8))]
pub struct USkeletonEditingToolBuilder {
    __padding_end: [u8; 48],
}
impl USkeletonEditingToolBuilder {}
#[repr(C, align(16))]
pub struct USkeletonEditingTool {
    __padding_end: [u8; 720],
}
impl USkeletonEditingTool {}
#[repr(C, align(16))]
pub struct USkeletonEditingProperties {
    __padding_end: [u8; 336],
}
impl USkeletonEditingProperties {}
#[repr(C, align(16))]
pub struct UProjectionProperties {
    __padding_end: [u8; 352],
}
impl UProjectionProperties {}
#[repr(C, align(8))]
pub struct UMirroringProperties {
    __padding_end: [u8; 240],
}
impl UMirroringProperties {}
#[repr(C, align(8))]
pub struct UOrientingProperties {
    __padding_end: [u8; 240],
}
impl UOrientingProperties {}
#[repr(C, align(16))]
pub struct USkeletonTransformProxy {
    __padding_end: [u8; 512],
}
impl USkeletonTransformProxy {}
#[repr(C, align(8))]
pub struct USkinWeightsBindingToolBuilder {
    __padding_end: [u8; 48],
}
impl USkinWeightsBindingToolBuilder {}
#[repr(C, align(8))]
pub struct USkinWeightsBindingToolProperties {
    __padding_end: [u8; 216],
}
impl USkinWeightsBindingToolProperties {}
#[repr(C, align(16))]
pub struct USkinWeightsBindingTool {
    __padding_end: [u8; 656],
}
impl USkinWeightsBindingTool {}
#[repr(C, align(8))]
pub struct USkinWeightsPaintToolBuilder {
    __padding_end: [u8; 56],
}
impl USkinWeightsPaintToolBuilder {}
#[repr(C, align(16))]
pub struct USkinWeightsPaintToolProperties {
    __padding_end: [u8; 608],
}
impl USkinWeightsPaintToolProperties {}
#[repr(C, align(8))]
pub struct UDEPRECATED_WeightToolMeshSelector {
    __padding_end: [u8; 112],
}
impl UDEPRECATED_WeightToolMeshSelector {}
#[repr(C, align(8))]
pub struct UWeightToolTransferManager {
    __padding_end: [u8; 88],
}
impl UWeightToolTransferManager {}
#[repr(C, align(8))]
pub struct UWeightToolSelectionIsolator {
    __padding_end: [u8; 1864],
}
impl UWeightToolSelectionIsolator {}
#[repr(C, align(8))]
pub struct USkinWeightsPaintTool {
    __padding_end: [u8; 3416],
}
impl USkinWeightsPaintTool {}
#[repr(transparent)]
pub struct EWeightBrushFalloffMode(pub u8);
impl EWeightBrushFalloffMode {
    pub const SURFACE: EWeightBrushFalloffMode = EWeightBrushFalloffMode(0);
    pub const VOLUME: EWeightBrushFalloffMode = EWeightBrushFalloffMode(1);
}
#[repr(transparent)]
pub struct EComponentSelectionMode(pub u8);
impl EComponentSelectionMode {
    pub const VERTICES: EComponentSelectionMode = EComponentSelectionMode(0);
    pub const EDGES: EComponentSelectionMode = EComponentSelectionMode(1);
    pub const FACES: EComponentSelectionMode = EComponentSelectionMode(2);
}
#[repr(transparent)]
pub struct EAttributeEditorElementType(pub u8);
impl EAttributeEditorElementType {
    pub const VERTEX: EAttributeEditorElementType = EAttributeEditorElementType(0);
    pub const VERTEX_INSTANCE: EAttributeEditorElementType = EAttributeEditorElementType(
        1,
    );
    pub const TRIANGLE: EAttributeEditorElementType = EAttributeEditorElementType(2);
    pub const POLYGON: EAttributeEditorElementType = EAttributeEditorElementType(3);
    pub const EDGE: EAttributeEditorElementType = EAttributeEditorElementType(4);
    pub const POLYGON_GROUP: EAttributeEditorElementType = EAttributeEditorElementType(
        5,
    );
}
#[repr(transparent)]
pub struct EAttributeEditorAttribType(pub u8);
impl EAttributeEditorAttribType {
    pub const INT32: EAttributeEditorAttribType = EAttributeEditorAttribType(0);
    pub const BOOLEAN: EAttributeEditorAttribType = EAttributeEditorAttribType(1);
    pub const FLOAT: EAttributeEditorAttribType = EAttributeEditorAttribType(2);
    pub const VECTOR2: EAttributeEditorAttribType = EAttributeEditorAttribType(3);
    pub const VECTOR3: EAttributeEditorAttribType = EAttributeEditorAttribType(4);
    pub const VECTOR4: EAttributeEditorAttribType = EAttributeEditorAttribType(5);
    pub const STRING: EAttributeEditorAttribType = EAttributeEditorAttribType(6);
    pub const UNKNOWN: EAttributeEditorAttribType = EAttributeEditorAttribType(7);
}
#[repr(transparent)]
pub struct EPolygonType(pub i32);
impl EPolygonType {
    pub const CIRCLE: EPolygonType = EPolygonType(0);
    pub const SQUARE: EPolygonType = EPolygonType(1);
    pub const RECTANGLE: EPolygonType = EPolygonType(2);
    pub const ROUND_RECT: EPolygonType = EPolygonType(3);
    pub const CUSTOM: EPolygonType = EPolygonType(4);
}
#[repr(transparent)]
pub struct EEditingOperation(pub u8);
impl EEditingOperation {
    pub const SELECT: EEditingOperation = EEditingOperation(0);
    pub const CREATE: EEditingOperation = EEditingOperation(1);
    pub const REMOVE: EEditingOperation = EEditingOperation(2);
    pub const TRANSFORM: EEditingOperation = EEditingOperation(3);
    pub const PARENT: EEditingOperation = EEditingOperation(4);
    pub const RENAME: EEditingOperation = EEditingOperation(5);
    pub const MIRROR: EEditingOperation = EEditingOperation(6);
}
#[repr(transparent)]
pub struct EProjectionType(pub u8);
impl EProjectionType {
    pub const CAMERA_PLANE: EProjectionType = EProjectionType(0);
    pub const ON_MESH: EProjectionType = EProjectionType(1);
    pub const WITHIN_MESH: EProjectionType = EProjectionType(2);
}
#[repr(transparent)]
pub struct ESkinWeightsBindType(pub u8);
impl ESkinWeightsBindType {
    pub const DIRECT_DISTANCE: ESkinWeightsBindType = ESkinWeightsBindType(0);
    pub const GEODESIC_VOXEL: ESkinWeightsBindType = ESkinWeightsBindType(1);
}
#[repr(transparent)]
pub struct EWeightEditMode(pub u8);
impl EWeightEditMode {
    pub const BRUSH: EWeightEditMode = EWeightEditMode(0);
    pub const MESH: EWeightEditMode = EWeightEditMode(1);
    pub const BONES: EWeightEditMode = EWeightEditMode(2);
}
#[repr(transparent)]
pub struct EWeightEditOperation(pub u8);
impl EWeightEditOperation {
    pub const ADD: EWeightEditOperation = EWeightEditOperation(0);
    pub const REPLACE: EWeightEditOperation = EWeightEditOperation(1);
    pub const MULTIPLY: EWeightEditOperation = EWeightEditOperation(2);
    pub const RELAX: EWeightEditOperation = EWeightEditOperation(3);
    pub const RELATIVE_SCALE: EWeightEditOperation = EWeightEditOperation(4);
}
#[repr(transparent)]
pub struct EWeightColorMode(pub u8);
impl EWeightColorMode {
    pub const GREYSCALE: EWeightColorMode = EWeightColorMode(0);
    pub const RAMP: EWeightColorMode = EWeightColorMode(1);
    pub const BONE_COLORS: EWeightColorMode = EWeightColorMode(2);
    pub const FULL_MATERIAL: EWeightColorMode = EWeightColorMode(3);
}
#[repr(transparent)]
pub struct EMirrorDirection(pub u8);
impl EMirrorDirection {
    pub const POSITIVE_TO_NEGATIVE: EMirrorDirection = EMirrorDirection(0);
    pub const NEGATIVE_TO_POSITIVE: EMirrorDirection = EMirrorDirection(1);
}
#[repr(transparent)]
pub struct EMeshTransferOption(pub u8);
impl EMeshTransferOption {
    pub const SOURCE: EMeshTransferOption = EMeshTransferOption(0);
    pub const TARGET: EMeshTransferOption = EMeshTransferOption(1);
}
