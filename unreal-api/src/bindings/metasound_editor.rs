#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct IMetaSoundPresetWidgetInterface {}
#[repr(C, align(8))]
pub struct UMetaSoundPresetWidgetInterface {
    __padding_end: [u8; 48],
}
impl UMetaSoundPresetWidgetInterface {}
#[repr(C, align(8))]
pub struct UAssetDefinition_MetaSoundPatch {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_MetaSoundPatch {}
#[repr(C, align(8))]
pub struct UAssetDefinition_MetaSoundSource {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_MetaSoundSource {}
#[repr(C, align(8))]
pub struct UMetasoundEditorViewBase {
    __padding_end: [u8; 56],
}
impl UMetasoundEditorViewBase {}
#[repr(C, align(8))]
pub struct UMetasoundInterfacesView {
    __padding_end: [u8; 56],
}
impl UMetasoundInterfacesView {}
#[repr(C, align(8))]
pub struct UMetasoundPagesView {
    __padding_end: [u8; 56],
}
impl UMetasoundPagesView {}
#[repr(C, align(8))]
pub struct UMetaSoundEditorBuilderListener {
    __padding_end: [u8; 904],
}
impl UMetaSoundEditorBuilderListener {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphMemberDefaultLiteral {
    __padding_end: [u8; 64],
}
impl UMetasoundEditorGraphMemberDefaultLiteral {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphMember {
    __padding_end: [u8; 104],
}
impl UMetasoundEditorGraphMember {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphVertex {
    __padding_end: [u8; 288],
}
impl UMetasoundEditorGraphVertex {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphInput {
    __padding_end: [u8; 288],
}
impl UMetasoundEditorGraphInput {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphOutput {
    __padding_end: [u8; 288],
}
impl UMetasoundEditorGraphOutput {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphVariable {
    __padding_end: [u8; 240],
}
impl UMetasoundEditorGraphVariable {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraph {
    __padding_end: [u8; 248],
}
impl UMetasoundEditorGraph {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphCommentNode {
    __padding_end: [u8; 312],
}
impl UMetasoundEditorGraphCommentNode {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphNode {
    __padding_end: [u8; 256],
}
impl UMetasoundEditorGraphNode {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphMemberNode {
    __padding_end: [u8; 256],
}
impl UMetasoundEditorGraphMemberNode {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphInputNode {
    __padding_end: [u8; 616],
}
impl UMetasoundEditorGraphInputNode {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphMemberDefaultBool {
    __padding_end: [u8; 120],
}
impl UMetasoundEditorGraphMemberDefaultBool {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphMemberDefaultBoolArray {
    __padding_end: [u8; 96],
}
impl UMetasoundEditorGraphMemberDefaultBoolArray {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphMemberDefaultInt {
    __padding_end: [u8; 88],
}
impl UMetasoundEditorGraphMemberDefaultInt {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphMemberDefaultIntArray {
    __padding_end: [u8; 96],
}
impl UMetasoundEditorGraphMemberDefaultIntArray {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphMemberDefaultFloat {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub range: crate::bindings::core_u_object::FVector2D,
    __padding_end: [u8; 120],
}
impl UMetasoundEditorGraphMemberDefaultFloat {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphMemberDefaultFloatArray {
    __padding_end: [u8; 96],
}
impl UMetasoundEditorGraphMemberDefaultFloatArray {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphMemberDefaultString {
    __padding_end: [u8; 96],
}
impl UMetasoundEditorGraphMemberDefaultString {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphMemberDefaultStringArray {
    __padding_end: [u8; 96],
}
impl UMetasoundEditorGraphMemberDefaultStringArray {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphMemberDefaultObject {
    __padding_end: [u8; 88],
}
impl UMetasoundEditorGraphMemberDefaultObject {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphMemberDefaultObjectArray {
    __padding_end: [u8; 96],
}
impl UMetasoundEditorGraphMemberDefaultObjectArray {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphOutputNode {
    __padding_end: [u8; 600],
}
impl UMetasoundEditorGraphOutputNode {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphExternalNode {
    __padding_end: [u8; 416],
}
impl UMetasoundEditorGraphExternalNode {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphVariableNode {
    __padding_end: [u8; 648],
}
impl UMetasoundEditorGraphVariableNode {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphSchema {
    __padding_end: [u8; 48],
}
impl UMetasoundEditorGraphSchema {}
#[repr(C, align(8))]
pub struct UMetasoundEditorSettings {
    __padding_end: [u8; 640],
}
impl UMetasoundEditorSettings {}
#[repr(C, align(8))]
pub struct UMetaSoundEditorSubsystem {
    __padding_end: [u8; 72],
}
impl UMetaSoundEditorSubsystem {}
#[repr(C, align(8))]
pub struct UMetaSoundBaseFactory {
    __padding_end: [u8; 144],
}
impl UMetaSoundBaseFactory {}
#[repr(C, align(8))]
pub struct UMetaSoundFactory {
    __padding_end: [u8; 144],
}
impl UMetaSoundFactory {}
#[repr(C, align(8))]
pub struct UMetaSoundSourceFactory {
    __padding_end: [u8; 144],
}
impl UMetaSoundSourceFactory {}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnDocumentDisplayNameChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnDocumentDescriptionChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnDocumentAuthorChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnDocumentKeywordsChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnDocumentCategoryHierarchyChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnDocumentIsDeprecatedChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputAddedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnRemovingGraphInputDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputNameChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputDisplayNameChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputDataTypeChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputDescriptionChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputSortOrderIndexChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputIsConstructorPinChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputIsAdvancedDisplayChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputDefaultChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputInheritsDefaultChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputAddedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnRemovingGraphOutputDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputNameChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputDisplayNameChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputDataTypeChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputDescriptionChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputSortOrderIndexChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputIsConstructorPinChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputIsAdvancedDisplayChangedDelegate {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EMetasoundActiveAnalyzerEnvelopeDirection(pub u8);
impl EMetasoundActiveAnalyzerEnvelopeDirection {
    pub const FROM_SOURCE_OUTPUT: EMetasoundActiveAnalyzerEnvelopeDirection = EMetasoundActiveAnalyzerEnvelopeDirection(
        0,
    );
    pub const FROM_DESTINATION_INPUT: EMetasoundActiveAnalyzerEnvelopeDirection = EMetasoundActiveAnalyzerEnvelopeDirection(
        1,
    );
}
#[repr(transparent)]
pub struct EMetasoundBoolMemberDefaultWidget(pub u8);
impl EMetasoundBoolMemberDefaultWidget {
    pub const NONE: EMetasoundBoolMemberDefaultWidget = EMetasoundBoolMemberDefaultWidget(
        0,
    );
    pub const BUTTON: EMetasoundBoolMemberDefaultWidget = EMetasoundBoolMemberDefaultWidget(
        1,
    );
}
#[repr(transparent)]
pub struct EMetasoundMemberDefaultWidget(pub u8);
impl EMetasoundMemberDefaultWidget {
    pub const NONE: EMetasoundMemberDefaultWidget = EMetasoundMemberDefaultWidget(0);
    pub const SLIDER: EMetasoundMemberDefaultWidget = EMetasoundMemberDefaultWidget(1);
    pub const RADIAL_SLIDER: EMetasoundMemberDefaultWidget = EMetasoundMemberDefaultWidget(
        2,
    );
}
#[repr(transparent)]
pub struct EMetasoundMemberDefaultWidgetValueType(pub u8);
impl EMetasoundMemberDefaultWidgetValueType {
    pub const LINEAR: EMetasoundMemberDefaultWidgetValueType = EMetasoundMemberDefaultWidgetValueType(
        0,
    );
    pub const FREQUENCY: EMetasoundMemberDefaultWidgetValueType = EMetasoundMemberDefaultWidgetValueType(
        1,
    );
    pub const VOLUME: EMetasoundMemberDefaultWidgetValueType = EMetasoundMemberDefaultWidgetValueType(
        2,
    );
}
#[repr(transparent)]
pub struct EAuditionPageMode(pub u8);
impl EAuditionPageMode {
    pub const FOCUSED: EAuditionPageMode = EAuditionPageMode(0);
    pub const USER: EAuditionPageMode = EAuditionPageMode(1);
}
#[repr(transparent)]
pub struct EMetasoundActiveDetailView(pub u8);
impl EMetasoundActiveDetailView {
    pub const METASOUND: EMetasoundActiveDetailView = EMetasoundActiveDetailView(0);
    pub const GENERAL: EMetasoundActiveDetailView = EMetasoundActiveDetailView(1);
}
