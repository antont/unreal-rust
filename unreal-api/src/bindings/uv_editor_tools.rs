#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UUVEditorMechanicAdapterTool {
    __padding_end: [u8; 208],
}
impl UUVEditorMechanicAdapterTool {}
#[repr(C, align(8))]
pub struct UUVToolContextObject {
    __padding_end: [u8; 48],
}
impl UUVToolContextObject {}
#[repr(C, align(8))]
pub struct UUVToolAssetInputsContext {
    __padding_end: [u8; 80],
}
impl UUVToolAssetInputsContext {}
#[repr(C, align(8))]
pub struct UUVToolViewportButtonsAPI {
    __padding_end: [u8; 176],
}
impl UUVToolViewportButtonsAPI {}
pub struct IVUnwrapDynamicMesh {}
#[repr(C, align(8))]
pub struct UUVUnwrapDynamicMesh {
    __padding_end: [u8; 48],
}
impl UUVUnwrapDynamicMesh {}
#[repr(C, align(8))]
pub struct UUVToolAction {
    __padding_end: [u8; 72],
}
impl UUVToolAction {}
#[repr(C, align(8))]
pub struct UUVMakeIslandAction {
    __padding_end: [u8; 72],
}
impl UUVMakeIslandAction {}
#[repr(C, align(8))]
pub struct UUVSeamSewAction {
    __padding_end: [u8; 72],
}
impl UUVSeamSewAction {}
#[repr(C, align(8))]
pub struct UUVSplitAction {
    __padding_end: [u8; 72],
}
impl UUVSplitAction {}
#[repr(C, align(8))]
pub struct UUVToolEmitChangeAPI {
    __padding_end: [u8; 56],
}
impl UUVToolEmitChangeAPI {}
#[repr(C, align(16))]
pub struct UUVToolLivePreviewAPI {
    __padding_end: [u8; 288],
}
impl UUVToolLivePreviewAPI {}
#[repr(C, align(8))]
pub struct UUVTool2DViewportAPI {
    __padding_end: [u8; 168],
}
impl UUVTool2DViewportAPI {}
#[repr(C, align(16))]
pub struct UUVToolAssetAndChannelAPI {
    __padding_end: [u8; 192],
}
impl UUVToolAssetAndChannelAPI {}
#[repr(C, align(8))]
pub struct UUVToolAABBTreeStorage {
    __padding_end: [u8; 128],
}
impl UUVToolAABBTreeStorage {}
#[repr(C, align(8))]
pub struct UUVEditorToolPropertiesAPI {
    __padding_end: [u8; 56],
}
impl UUVEditorToolPropertiesAPI {}
#[repr(C, align(16))]
pub struct UBasicLineSetComponentBase {
    __padding_end: [u8; 1664],
}
impl UBasicLineSetComponentBase {}
#[repr(C, align(16))]
pub struct UBasic2DLineSetComponent {
    __padding_end: [u8; 1680],
}
impl UBasic2DLineSetComponent {}
#[repr(C, align(16))]
pub struct UBasic3DLineSetComponent {
    __padding_end: [u8; 1680],
}
impl UBasic3DLineSetComponent {}
#[repr(C, align(16))]
pub struct UBasicPointSetComponentBase {
    __padding_end: [u8; 1664],
}
impl UBasicPointSetComponentBase {}
#[repr(C, align(16))]
pub struct UBasic2DPointSetComponent {
    __padding_end: [u8; 1680],
}
impl UBasic2DPointSetComponent {}
#[repr(C, align(16))]
pub struct UBasic3DPointSetComponent {
    __padding_end: [u8; 1680],
}
impl UBasic3DPointSetComponent {}
#[repr(C, align(16))]
pub struct UBasicTriangleSetComponentBase {
    __padding_end: [u8; 1664],
}
impl UBasicTriangleSetComponentBase {}
#[repr(C, align(16))]
pub struct UBasic2DTriangleSetComponent {
    __padding_end: [u8; 1696],
}
impl UBasic2DTriangleSetComponent {}
#[repr(C, align(16))]
pub struct UBasic3DTriangleSetComponent {
    __padding_end: [u8; 1696],
}
impl UBasic3DTriangleSetComponent {}
#[repr(C, align(8))]
pub struct UUVEditorUVTransformPropertiesBase {
    __padding_end: [u8; 184],
}
impl UUVEditorUVTransformPropertiesBase {}
#[repr(C, align(8))]
pub struct UUVEditorUVTransformProperties {
    __padding_end: [u8; 280],
}
impl UUVEditorUVTransformProperties {}
#[repr(C, align(8))]
pub struct UUVEditorUVQuickTransformProperties {
    __padding_end: [u8; 280],
}
impl UUVEditorUVQuickTransformProperties {}
#[repr(C, align(8))]
pub struct UUVEditorUVAlignProperties {
    __padding_end: [u8; 216],
}
impl UUVEditorUVAlignProperties {}
#[repr(C, align(8))]
pub struct UUVEditorUVDistributeProperties {
    __padding_end: [u8; 200],
}
impl UUVEditorUVDistributeProperties {}
#[repr(C, align(16))]
pub struct UUVEditorUVTransformOperatorFactory {
    __padding_end: [u8; 400],
}
impl UUVEditorUVTransformOperatorFactory {}
#[repr(C, align(16))]
pub struct UUVEditorMeshSelectionMechanic {
    __padding_end: [u8; 496],
}
impl UUVEditorMeshSelectionMechanic {}
#[repr(C, align(8))]
pub struct UUVToolSelectionAPI {
    __padding_end: [u8; 288],
}
impl UUVToolSelectionAPI {}
pub struct IVToolSupportsSelection {}
#[repr(C, align(8))]
pub struct UUVToolSupportsSelection {
    __padding_end: [u8; 48],
}
impl UUVToolSupportsSelection {}
#[repr(C, align(8))]
pub struct UUVToolSelectionHighlightMechanic {
    __padding_end: [u8; 184],
}
impl UUVToolSelectionHighlightMechanic {}
#[repr(C, align(16))]
pub struct UUVEditorToolMeshInput {
    __padding_end: [u8; 256],
}
impl UUVEditorToolMeshInput {}
#[repr(C, align(8))]
pub struct UUVEditorBrushSelectToolProperties {
    __padding_end: [u8; 200],
}
impl UUVEditorBrushSelectToolProperties {}
#[repr(C, align(8))]
pub struct UUVEditorBrushSelectTool {
    __padding_end: [u8; 392],
}
impl UUVEditorBrushSelectTool {}
#[repr(C, align(8))]
pub struct UUVEditorChannelEditToolBuilder {
    __padding_end: [u8; 56],
}
impl UUVEditorChannelEditToolBuilder {}
#[repr(C, align(8))]
pub struct UUVEditorChannelEditSettings {
    __padding_end: [u8; 192],
}
impl UUVEditorChannelEditSettings {}
#[repr(C, align(8))]
pub struct UUVEditorChannelEditTargetProperties {
    __padding_end: [u8; 288],
}
impl UUVEditorChannelEditTargetProperties {}
#[repr(C, align(8))]
pub struct UUVEditorChannelEditAddProperties {
    __padding_end: [u8; 184],
}
impl UUVEditorChannelEditAddProperties {}
#[repr(C, align(8))]
pub struct UUVEditorChannelEditCopyProperties {
    __padding_end: [u8; 184],
}
impl UUVEditorChannelEditCopyProperties {}
#[repr(C, align(8))]
pub struct UUVEditorChannelEditDeleteProperties {
    __padding_end: [u8; 184],
}
impl UUVEditorChannelEditDeleteProperties {}
#[repr(C, align(8))]
pub struct UUVEditorChannelEditToolActionPropertySet {
    __padding_end: [u8; 192],
}
impl UUVEditorChannelEditToolActionPropertySet {}
#[repr(C, align(8))]
pub struct UUVEditorChannelEditTool {
    __padding_end: [u8; 432],
}
impl UUVEditorChannelEditTool {}
#[repr(C, align(8))]
pub struct UUVEditorLayoutToolBuilder {
    __padding_end: [u8; 56],
}
impl UUVEditorLayoutToolBuilder {}
#[repr(C, align(8))]
pub struct UUVEditorLayoutTool {
    __padding_end: [u8; 392],
}
impl UUVEditorLayoutTool {}
#[repr(C, align(8))]
pub struct UUVEditorRecomputeUVsToolBuilder {
    __padding_end: [u8; 56],
}
impl UUVEditorRecomputeUVsToolBuilder {}
#[repr(C, align(8))]
pub struct UUVEditorRecomputeUVsTool {
    __padding_end: [u8; 408],
}
impl UUVEditorRecomputeUVsTool {}
#[repr(C, align(8))]
pub struct UUVEditorSeamToolProperties {
    __padding_end: [u8; 200],
}
impl UUVEditorSeamToolProperties {}
#[repr(C, align(8))]
pub struct UUVEditorSeamToolBuilder {
    __padding_end: [u8; 56],
}
impl UUVEditorSeamToolBuilder {}
#[repr(C, align(16))]
pub struct UUVEditorSeamTool {
    __padding_end: [u8; 688],
}
impl UUVEditorSeamTool {}
#[repr(C, align(8))]
pub struct UUVEditorTexelDensityToolBuilder {
    __padding_end: [u8; 56],
}
impl UUVEditorTexelDensityToolBuilder {}
#[repr(C, align(8))]
pub struct UUVEditorTexelDensityActionSettings {
    __padding_end: [u8; 192],
}
impl UUVEditorTexelDensityActionSettings {}
#[repr(C, align(8))]
pub struct UUVEditorTexelDensityToolSettings {
    __padding_end: [u8; 216],
}
impl UUVEditorTexelDensityToolSettings {}
#[repr(C, align(16))]
pub struct UUVEditorTexelDensityTool {
    __padding_end: [u8; 672],
}
impl UUVEditorTexelDensityTool {}
pub struct IVEditorGenericBuildableTool {}
#[repr(C, align(8))]
pub struct UUVEditorGenericBuildableTool {
    __padding_end: [u8; 48],
}
impl UUVEditorGenericBuildableTool {}
#[repr(C, align(8))]
pub struct UGenericUVEditorToolBuilder {
    __padding_end: [u8; 64],
}
impl UGenericUVEditorToolBuilder {}
#[repr(C, align(8))]
pub struct UUVEditorTransformToolDisplayProperties {
    __padding_end: [u8; 192],
}
impl UUVEditorTransformToolDisplayProperties {}
#[repr(C, align(8))]
pub struct UUVEditorBaseTransformToolBuilder {
    __padding_end: [u8; 56],
}
impl UUVEditorBaseTransformToolBuilder {}
#[repr(C, align(8))]
pub struct UUVEditorTransformToolBuilder {
    __padding_end: [u8; 56],
}
impl UUVEditorTransformToolBuilder {}
#[repr(C, align(8))]
pub struct UUVEditorAlignToolBuilder {
    __padding_end: [u8; 56],
}
impl UUVEditorAlignToolBuilder {}
#[repr(C, align(8))]
pub struct UUVEditorDistributeToolBuilder {
    __padding_end: [u8; 56],
}
impl UUVEditorDistributeToolBuilder {}
#[repr(C, align(8))]
pub struct UUVEditorTransformTool {
    __padding_end: [u8; 416],
}
impl UUVEditorTransformTool {}
#[repr(C, align(8))]
pub struct UUVEditorUVSnapshotToolBuilder {
    __padding_end: [u8; 56],
}
impl UUVEditorUVSnapshotToolBuilder {}
#[repr(C, align(8))]
pub struct UUVEditorUVSnapshotTool {
    __padding_end: [u8; 288],
}
impl UUVEditorUVSnapshotTool {}
#[repr(C, align(8))]
pub struct UUVEditorBakeUVShellProperties {
    __padding_end: [u8; 304],
}
impl UUVEditorBakeUVShellProperties {}
#[repr(C, align(8))]
pub struct UUVSelectToolBuilder {
    __padding_end: [u8; 56],
}
impl UUVSelectToolBuilder {}
#[repr(C, align(16))]
pub struct UUVSelectTool {
    __padding_end: [u8; 512],
}
impl UUVSelectTool {}
#[repr(transparent)]
pub struct EUVEditorTranslationMode(pub i32);
impl EUVEditorTranslationMode {
    pub const RELATIVE: EUVEditorTranslationMode = EUVEditorTranslationMode(0);
    pub const ABSOLUTE: EUVEditorTranslationMode = EUVEditorTranslationMode(1);
}
#[repr(transparent)]
pub struct EUVEditorPivotType(pub i32);
impl EUVEditorPivotType {
    pub const BOUNDING_BOX_CENTER: EUVEditorPivotType = EUVEditorPivotType(0);
    pub const ORIGIN: EUVEditorPivotType = EUVEditorPivotType(1);
    pub const INDIVIDUAL_BOUNDING_BOX_CENTER: EUVEditorPivotType = EUVEditorPivotType(2);
    pub const MANUAL: EUVEditorPivotType = EUVEditorPivotType(3);
}
#[repr(transparent)]
pub struct EUVEditorAlignAnchor(pub i32);
impl EUVEditorAlignAnchor {
    pub const BOUNDING_BOX: EUVEditorAlignAnchor = EUVEditorAlignAnchor(0);
    pub const UDIM_TILE: EUVEditorAlignAnchor = EUVEditorAlignAnchor(1);
    pub const MANUAL: EUVEditorAlignAnchor = EUVEditorAlignAnchor(2);
}
#[repr(transparent)]
pub struct EUVEditorAlignDirection(pub i32);
impl EUVEditorAlignDirection {
    pub const NONE: EUVEditorAlignDirection = EUVEditorAlignDirection(0);
    pub const TOP: EUVEditorAlignDirection = EUVEditorAlignDirection(1);
    pub const BOTTOM: EUVEditorAlignDirection = EUVEditorAlignDirection(2);
    pub const LEFT: EUVEditorAlignDirection = EUVEditorAlignDirection(3);
    pub const RIGHT: EUVEditorAlignDirection = EUVEditorAlignDirection(4);
    pub const CENTER_VERTICALLY: EUVEditorAlignDirection = EUVEditorAlignDirection(5);
    pub const CENTER_HORIZONTALLY: EUVEditorAlignDirection = EUVEditorAlignDirection(6);
}
#[repr(transparent)]
pub struct EUVEditorAlignDistributeGroupingMode(pub u8);
impl EUVEditorAlignDistributeGroupingMode {
    pub const INDIVIDUAL_BOUNDING_BOXES: EUVEditorAlignDistributeGroupingMode = EUVEditorAlignDistributeGroupingMode(
        0,
    );
    pub const ENCLOSING_BOUNDING_BOX: EUVEditorAlignDistributeGroupingMode = EUVEditorAlignDistributeGroupingMode(
        1,
    );
    pub const INDIVIDUAL_VERTICES: EUVEditorAlignDistributeGroupingMode = EUVEditorAlignDistributeGroupingMode(
        2,
    );
}
#[repr(transparent)]
pub struct EUVEditorDistributeMode(pub u16);
impl EUVEditorDistributeMode {
    pub const NONE: EUVEditorDistributeMode = EUVEditorDistributeMode(0);
    pub const VERTICAL_SPACE: EUVEditorDistributeMode = EUVEditorDistributeMode(1);
    pub const HORIZONTAL_SPACE: EUVEditorDistributeMode = EUVEditorDistributeMode(2);
    pub const TOP_EDGES: EUVEditorDistributeMode = EUVEditorDistributeMode(3);
    pub const BOTTOM_EDGES: EUVEditorDistributeMode = EUVEditorDistributeMode(4);
    pub const LEFT_EDGES: EUVEditorDistributeMode = EUVEditorDistributeMode(5);
    pub const RIGHT_EDGES: EUVEditorDistributeMode = EUVEditorDistributeMode(6);
    pub const CENTERS_VERTICALLY: EUVEditorDistributeMode = EUVEditorDistributeMode(7);
    pub const CENTERS_HORIZONTALLY: EUVEditorDistributeMode = EUVEditorDistributeMode(8);
    pub const MINIMALLY_REMOVE_OVERLAP: EUVEditorDistributeMode = EUVEditorDistributeMode(
        9,
    );
}
#[repr(transparent)]
pub struct EChannelEditToolAction(pub i32);
impl EChannelEditToolAction {
    pub const NO_ACTION: EChannelEditToolAction = EChannelEditToolAction(0);
    pub const ADD: EChannelEditToolAction = EChannelEditToolAction(1);
    pub const COPY: EChannelEditToolAction = EChannelEditToolAction(2);
    pub const DELETE: EChannelEditToolAction = EChannelEditToolAction(3);
}
#[repr(transparent)]
pub struct EUVEditorSeamMode(pub u8);
impl EUVEditorSeamMode {
    pub const CUT: EUVEditorSeamMode = EUVEditorSeamMode(0);
    pub const JOIN: EUVEditorSeamMode = EUVEditorSeamMode(1);
}
