#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FMetasoundEditorGraphMemberBreadcrumb {
    pub member_name: FName,
    pub description: FText,
}
#[repr(C, align(8))]
pub struct FMetasoundEditorGraphVertexBreadcrumb {
    pub access_type: crate::bindings::metasound_frontend::EMetasoundFrontendVertexAccessType,
    pub default_literals: TMap<
        crate::bindings::core_u_object::FGuid,
        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
    >,
    pub b_is_advanced_display: bool,
    pub sort_order_index: i32,
}
#[repr(C, align(8))]
pub struct FMetasoundEditorGraphVariableBreadcrumb {
    pub default_literal: crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
}
#[repr(C, align(4))]
pub struct FMetasoundEditorMemberPageDefault {
    pub page_name: FName,
    pub page_id: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(1))]
pub struct FMetasoundEditorGraphMemberDefaultBoolRef {
    pub value: bool,
}
#[repr(C, align(4))]
pub struct FMetasoundEditorMemberPageDefaultBool {
    pub value: FMetasoundEditorGraphMemberDefaultBoolRef,
}
#[repr(C, align(8))]
pub struct FMetasoundEditorMemberPageDefaultBoolArray {
    pub value: TArray<FMetasoundEditorGraphMemberDefaultBoolRef>,
}
#[repr(C, align(4))]
pub struct FMetasoundEditorGraphMemberDefaultIntRef {
    pub value: i32,
}
#[repr(C, align(4))]
pub struct FMetasoundEditorMemberPageDefaultInt {
    pub value: FMetasoundEditorGraphMemberDefaultIntRef,
}
#[repr(C, align(8))]
pub struct FMetasoundEditorMemberPageDefaultIntArray {
    pub value: TArray<FMetasoundEditorGraphMemberDefaultIntRef>,
}
#[repr(C, align(4))]
pub struct FMetasoundEditorMemberPageDefaultFloat {
    pub value: f32,
}
#[repr(C, align(8))]
pub struct FMetasoundEditorMemberPageDefaultFloatArray {
    pub value: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FMetasoundEditorMemberPageDefaultString {
    pub value: FString,
}
#[repr(C, align(8))]
pub struct FMetasoundEditorMemberPageDefaultStringArray {
    pub value: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FMetasoundEditorGraphMemberDefaultObjectRef {
    pub object: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FMetasoundEditorMemberPageDefaultObjectRef {
    pub value: FMetasoundEditorGraphMemberDefaultObjectRef,
}
#[repr(C, align(8))]
pub struct FMetasoundEditorMemberPageDefaultObjectArray {
    pub value: TArray<FMetasoundEditorGraphMemberDefaultObjectRef>,
}
#[repr(C, align(8))]
pub struct FMetasoundEditorGraphNodeBreadcrumb {
    pub class_name: crate::bindings::metasound_frontend::FMetasoundFrontendClassName,
    pub b_is_class_native: bool,
    pub node_configuration: crate::bindings::core_u_object::FInstancedStruct,
    pub template_params: TOptional<
        crate::bindings::metasound_frontend::FNodeTemplateGenerateInterfaceParams,
    >,
    pub version: crate::bindings::metasound_frontend::FMetasoundFrontendVersionNumber,
}
#[repr(C, align(8))]
pub struct FMetasoundEditorGraphMemberNodeBreadcrumb {
    pub member_name: FName,
    pub data_type: FName,
    pub default_literals: TMap<
        crate::bindings::core_u_object::FGuid,
        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
    >,
    pub vertex_metadata: crate::bindings::metasound_frontend::FMetasoundFrontendVertexMetadata,
    pub member_metadata_path: TOptional<crate::bindings::core_u_object::FSoftObjectPath>,
}
#[repr(C, align(8))]
pub struct FMetasoundEditorGraphVertexNodeBreadcrumb {
    pub access_type: crate::bindings::metasound_frontend::EMetasoundFrontendVertexAccessType,
}
#[repr(C, align(8))]
pub struct FMetasoundGraphSchemaAction {}
#[repr(C, align(8))]
pub struct FMetasoundGraphSchemaAction_NodeWithMultipleOutputs {}
#[repr(C, align(8))]
pub struct FMetasoundGraphSchemaAction_NewInput {
    pub node_id: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FMetasoundGraphSchemaAction_PromoteToInput {}
#[repr(C, align(8))]
pub struct FMetasoundGraphSchemaAction_NewOutput {
    pub node_id: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FMetasoundGraphSchemaAction_PromoteToOutput {}
#[repr(C, align(8))]
pub struct FMetasoundGraphSchemaAction_NewVariableNode {
    pub variable_id: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FMetasoundGraphSchemaAction_NewVariableAccessorNode {}
#[repr(C, align(8))]
pub struct FMetasoundGraphSchemaAction_NewVariableDeferredAccessorNode {}
#[repr(C, align(8))]
pub struct FMetasoundGraphSchemaAction_NewVariableMutatorNode {}
#[repr(C, align(8))]
pub struct FMetasoundGraphSchemaAction_PromoteToVariable_AccessorNode {}
#[repr(C, align(8))]
pub struct FMetasoundGraphSchemaAction_PromoteToVariable_MutatorNode {}
#[repr(C, align(8))]
pub struct FMetasoundGraphSchemaAction_PromoteToVariable_DeferredAccessorNode {}
#[repr(C, align(8))]
pub struct FMetasoundGraphSchemaAction_NewNode {}
#[repr(C, align(8))]
pub struct FMetasoundGraphSchemaAction_NewFromSelected {}
#[repr(C, align(8))]
pub struct FMetasoundGraphSchemaAction_NewAudioAnalyzer {}
#[repr(C, align(8))]
pub struct FMetasoundGraphSchemaAction_NewReroute {}
#[repr(C, align(8))]
pub struct FMetasoundGraphSchemaAction_NewComment {}
#[repr(C, align(8))]
pub struct FMetasoundGraphSchemaAction_Paste {}
#[repr(C, align(4))]
pub struct FMetasoundAnalyzerAnimationSettings {
    pub b_animate_connections: bool,
    pub envelope_wire_thickness: f32,
    pub envelope_speed: f32,
    pub envelope_direction: EMetasoundActiveAnalyzerEnvelopeDirection,
    pub numeric_wire_thickness: f32,
    pub wire_scalar_min: f32,
    pub wire_scalar_max: f32,
}
pub struct UMetaSoundPresetWidgetInterface {}
pub struct IMetaSoundPresetWidgetInterface {}
pub struct UAssetDefinition_MetaSoundPatch {}
pub struct UAssetDefinition_MetaSoundSource {}
pub struct UMetasoundEditorViewBase {}
pub struct UMetasoundInterfacesView {}
pub struct UMetasoundPagesView {}
pub struct UMetaSoundEditorBuilderListener {
    pub on_document_display_name_changed_delegate: FMetaSoundEditorBuilderListener_OnDocumentDisplayNameChangedDelegate,
    pub on_document_description_changed_delegate: FMetaSoundEditorBuilderListener_OnDocumentDescriptionChangedDelegate,
    pub on_document_author_changed_delegate: FMetaSoundEditorBuilderListener_OnDocumentAuthorChangedDelegate,
    pub on_document_keywords_changed_delegate: FMetaSoundEditorBuilderListener_OnDocumentKeywordsChangedDelegate,
    pub on_document_category_hierarchy_changed_delegate: FMetaSoundEditorBuilderListener_OnDocumentCategoryHierarchyChangedDelegate,
    pub on_document_is_deprecated_changed_delegate: FMetaSoundEditorBuilderListener_OnDocumentIsDeprecatedChangedDelegate,
    pub on_graph_input_added_delegate: FMetaSoundEditorBuilderListener_OnGraphInputAddedDelegate,
    pub on_removing_graph_input_delegate: FMetaSoundEditorBuilderListener_OnRemovingGraphInputDelegate,
    pub on_graph_input_name_changed_delegate: FMetaSoundEditorBuilderListener_OnGraphInputNameChangedDelegate,
    pub on_graph_input_display_name_changed_delegate: FMetaSoundEditorBuilderListener_OnGraphInputDisplayNameChangedDelegate,
    pub on_graph_input_data_type_changed_delegate: FMetaSoundEditorBuilderListener_OnGraphInputDataTypeChangedDelegate,
    pub on_graph_input_description_changed_delegate: FMetaSoundEditorBuilderListener_OnGraphInputDescriptionChangedDelegate,
    pub on_graph_input_sort_order_index_changed_delegate: FMetaSoundEditorBuilderListener_OnGraphInputSortOrderIndexChangedDelegate,
    pub on_graph_input_is_constructor_pin_changed_delegate: FMetaSoundEditorBuilderListener_OnGraphInputIsConstructorPinChangedDelegate,
    pub on_graph_input_is_advanced_display_changed_delegate: FMetaSoundEditorBuilderListener_OnGraphInputIsAdvancedDisplayChangedDelegate,
    pub on_graph_input_default_changed_delegate: FMetaSoundEditorBuilderListener_OnGraphInputDefaultChangedDelegate,
    pub on_graph_input_inherits_default_changed_delegate: FMetaSoundEditorBuilderListener_OnGraphInputInheritsDefaultChangedDelegate,
    pub on_graph_output_added_delegate: FMetaSoundEditorBuilderListener_OnGraphOutputAddedDelegate,
    pub on_removing_graph_output_delegate: FMetaSoundEditorBuilderListener_OnRemovingGraphOutputDelegate,
    pub on_graph_output_name_changed_delegate: FMetaSoundEditorBuilderListener_OnGraphOutputNameChangedDelegate,
    pub on_graph_output_display_name_changed_delegate: FMetaSoundEditorBuilderListener_OnGraphOutputDisplayNameChangedDelegate,
    pub on_graph_output_data_type_changed_delegate: FMetaSoundEditorBuilderListener_OnGraphOutputDataTypeChangedDelegate,
    pub on_graph_output_description_changed_delegate: FMetaSoundEditorBuilderListener_OnGraphOutputDescriptionChangedDelegate,
    pub on_graph_output_sort_order_index_changed_delegate: FMetaSoundEditorBuilderListener_OnGraphOutputSortOrderIndexChangedDelegate,
    pub on_graph_output_is_constructor_pin_changed_delegate: FMetaSoundEditorBuilderListener_OnGraphOutputIsConstructorPinChangedDelegate,
    pub on_graph_output_is_advanced_display_changed_delegate: FMetaSoundEditorBuilderListener_OnGraphOutputIsAdvancedDisplayChangedDelegate,
}
pub struct UMetasoundEditorGraphMemberDefaultLiteral {}
pub struct UMetasoundEditorGraphMember {
    pub literal: UPtr<UMetasoundEditorGraphMemberDefaultLiteral>,
    pub type_name: FName,
}
pub struct UMetasoundEditorGraphVertex {
    pub node_id: crate::bindings::core_u_object::FGuid,
    pub class_name: crate::bindings::metasound_frontend::FMetasoundFrontendClassName,
    pub breadcrumb: FMetasoundEditorGraphVertexBreadcrumb,
}
pub struct UMetasoundEditorGraphInput {}
pub struct UMetasoundEditorGraphOutput {}
pub struct UMetasoundEditorGraphVariable {
    pub variable_id: crate::bindings::core_u_object::FGuid,
    pub breadcrumb: FMetasoundEditorGraphVariableBreadcrumb,
}
pub struct UMetasoundEditorGraph {
    pub inputs: TArray<UPtr<UMetasoundEditorGraphInput>>,
    pub outputs: TArray<UPtr<UMetasoundEditorGraphOutput>>,
    pub variables: TArray<UPtr<UMetasoundEditorGraphVariable>>,
}
pub struct UMetasoundEditorGraphCommentNode {
    pub comment_id: crate::bindings::core_u_object::FGuid,
}
pub struct UMetasoundEditorGraphNode {}
pub struct UMetasoundEditorGraphMemberNode {}
pub struct UMetasoundEditorGraphInputNode {
    pub input: UPtr<UMetasoundEditorGraphInput>,
    pub node_id: crate::bindings::core_u_object::FGuid,
    pub breadcrumb: FMetasoundEditorGraphVertexNodeBreadcrumb,
}
pub struct UMetasoundEditorGraphMemberDefaultBool {
    pub defaults: TArray<FMetasoundEditorMemberPageDefaultBool>,
    pub default: FMetasoundEditorGraphMemberDefaultBoolRef,
    pub widget_type: EMetasoundBoolMemberDefaultWidget,
}
pub struct UMetasoundEditorGraphMemberDefaultBoolArray {
    pub defaults: TArray<FMetasoundEditorMemberPageDefaultBoolArray>,
    pub default: TArray<FMetasoundEditorGraphMemberDefaultBoolRef>,
}
pub struct UMetasoundEditorGraphMemberDefaultInt {
    pub defaults: TArray<FMetasoundEditorMemberPageDefaultInt>,
    pub default: FMetasoundEditorGraphMemberDefaultIntRef,
}
pub struct UMetasoundEditorGraphMemberDefaultIntArray {
    pub defaults: TArray<FMetasoundEditorMemberPageDefaultIntArray>,
    pub default: TArray<FMetasoundEditorGraphMemberDefaultIntRef>,
}
pub struct UMetasoundEditorGraphMemberDefaultFloat {
    pub default: f32,
    pub defaults: TArray<FMetasoundEditorMemberPageDefaultFloat>,
    pub clamp_default: bool,
    pub range: crate::bindings::core_u_object::FVector2D,
    pub widget_type: EMetasoundMemberDefaultWidget,
    pub widget_orientation: crate::bindings::slate_core::EOrientation,
    pub widget_value_type_deprecated: EMetasoundMemberDefaultWidgetValueType,
    pub widget_unit_value_type: crate::bindings::audio_widgets::EAudioUnitsValueType,
    pub volume_widget_use_linear_output: bool,
    pub volume_widget_decibel_range: crate::bindings::core_u_object::FVector2D,
}
pub struct UMetasoundEditorGraphMemberDefaultFloatArray {
    pub defaults: TArray<FMetasoundEditorMemberPageDefaultFloatArray>,
    pub default: TArray<f32>,
}
pub struct UMetasoundEditorGraphMemberDefaultString {
    pub default: FString,
    pub defaults: TArray<FMetasoundEditorMemberPageDefaultString>,
}
pub struct UMetasoundEditorGraphMemberDefaultStringArray {
    pub defaults: TArray<FMetasoundEditorMemberPageDefaultStringArray>,
    pub default: TArray<FString>,
}
pub struct UMetasoundEditorGraphMemberDefaultObject {
    pub default: FMetasoundEditorGraphMemberDefaultObjectRef,
    pub defaults: TArray<FMetasoundEditorMemberPageDefaultObjectRef>,
}
pub struct UMetasoundEditorGraphMemberDefaultObjectArray {
    pub defaults: TArray<FMetasoundEditorMemberPageDefaultObjectArray>,
    pub default: TArray<FMetasoundEditorGraphMemberDefaultObjectRef>,
}
pub struct UMetasoundEditorGraphOutputNode {
    pub output: UPtr<UMetasoundEditorGraphOutput>,
    pub breadcrumb: FMetasoundEditorGraphVertexNodeBreadcrumb,
}
pub struct UMetasoundEditorGraphExternalNode {
    pub breadcrumb: FMetasoundEditorGraphNodeBreadcrumb,
    pub class_name: crate::bindings::metasound_frontend::FMetasoundFrontendClassName,
    pub node_id: crate::bindings::core_u_object::FGuid,
    pub b_is_class_native: bool,
}
pub struct UMetasoundEditorGraphVariableNode {
    pub class_type: crate::bindings::metasound_frontend::EMetasoundFrontendClassType,
    pub class_name: crate::bindings::metasound_frontend::FMetasoundFrontendClassName,
    pub node_id: crate::bindings::core_u_object::FGuid,
    pub variable: UPtr<UMetasoundEditorGraphVariable>,
    pub breadcrumb: FMetasoundEditorGraphMemberNodeBreadcrumb,
}
pub struct UMetasoundEditorGraphSchema {}
pub struct UMetasoundEditorSettings {
    pub b_pin_meta_sound_patch_in_asset_menu: bool,
    pub b_pin_meta_sound_source_in_asset_menu: bool,
    pub b_apply_audition_settings_in_pie: bool,
    pub default_author: FString,
    pub audition_page_mode: EAuditionPageMode,
    pub audition_platform: FName,
    pub audition_page: FName,
    pub default_pin_type_color: crate::bindings::core_u_object::FLinearColor,
    pub audio_pin_type_color: crate::bindings::core_u_object::FLinearColor,
    pub boolean_pin_type_color: crate::bindings::core_u_object::FLinearColor,
    pub float_pin_type_color: crate::bindings::core_u_object::FLinearColor,
    pub int_pin_type_color: crate::bindings::core_u_object::FLinearColor,
    pub object_pin_type_color: crate::bindings::core_u_object::FLinearColor,
    pub string_pin_type_color: crate::bindings::core_u_object::FLinearColor,
    pub time_pin_type_color: crate::bindings::core_u_object::FLinearColor,
    pub trigger_pin_type_color: crate::bindings::core_u_object::FLinearColor,
    pub wave_table_pin_type_color: crate::bindings::core_u_object::FLinearColor,
    pub native_node_title_color: crate::bindings::core_u_object::FLinearColor,
    pub asset_reference_node_title_color: crate::bindings::core_u_object::FLinearColor,
    pub input_node_title_color: crate::bindings::core_u_object::FLinearColor,
    pub output_node_title_color: crate::bindings::core_u_object::FLinearColor,
    pub variable_node_title_color: crate::bindings::core_u_object::FLinearColor,
    pub spectrogram_settings: crate::bindings::audio_widgets::FSpectrogramRackUnitSettings,
    pub spectrum_analyzer_settings: crate::bindings::audio_widgets::FSpectrumAnalyzerRackUnitSettings,
    pub loudness_meter_settings: crate::bindings::audio_widgets::FLoudnessMeterRackUnitSettings,
    pub default_input_widget_type: EMetasoundMemberDefaultWidget,
    pub analyzer_animation_settings: FMetasoundAnalyzerAnimationSettings,
    pub detail_view: EMetasoundActiveDetailView,
    pub b_use_audio_material_widgets: bool,
    pub knob_style_override: crate::bindings::core_u_object::FSoftObjectPath,
    pub slider_style_override: crate::bindings::core_u_object::FSoftObjectPath,
    pub button_style_override: crate::bindings::core_u_object::FSoftObjectPath,
    pub meter_style_override: crate::bindings::core_u_object::FSoftObjectPath,
}
pub struct UMetaSoundEditorSubsystem {}
pub struct UMetaSoundBaseFactory {
    pub referenced_meta_sound_object: UPtr<crate::bindings::core_u_object::UObject>,
}
pub struct UMetaSoundFactory {}
pub struct UMetaSoundSourceFactory {}
pub struct FMetaSoundEditorBuilderListener_OnDocumentDisplayNameChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnDocumentDescriptionChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnDocumentAuthorChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnDocumentKeywordsChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnDocumentCategoryHierarchyChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnDocumentIsDeprecatedChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnGraphInputAddedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnRemovingGraphInputDelegate;
pub struct FMetaSoundEditorBuilderListener_OnGraphInputNameChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnGraphInputDisplayNameChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnGraphInputDataTypeChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnGraphInputDescriptionChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnGraphInputSortOrderIndexChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnGraphInputIsConstructorPinChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnGraphInputIsAdvancedDisplayChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnGraphInputDefaultChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnGraphInputInheritsDefaultChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputAddedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnRemovingGraphOutputDelegate;
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputNameChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputDisplayNameChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputDataTypeChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputDescriptionChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputSortOrderIndexChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputIsConstructorPinChangedDelegate;
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputIsAdvancedDisplayChangedDelegate;
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMetasoundMemberDefaultWidget(pub u8);
impl EMetasoundMemberDefaultWidget {
    pub const NONE: EMetasoundMemberDefaultWidget = EMetasoundMemberDefaultWidget(0);
    pub const SLIDER: EMetasoundMemberDefaultWidget = EMetasoundMemberDefaultWidget(1);
    pub const RADIAL_SLIDER: EMetasoundMemberDefaultWidget = EMetasoundMemberDefaultWidget(
        2,
    );
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAuditionPageMode(pub u8);
impl EAuditionPageMode {
    pub const FOCUSED: EAuditionPageMode = EAuditionPageMode(0);
    pub const USER: EAuditionPageMode = EAuditionPageMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMetasoundActiveDetailView(pub u8);
impl EMetasoundActiveDetailView {
    pub const METASOUND: EMetasoundActiveDetailView = EMetasoundActiveDetailView(0);
    pub const GENERAL: EMetasoundActiveDetailView = EMetasoundActiveDetailView(1);
}
