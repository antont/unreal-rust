#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FMetaSoundAssetKey {
    pub class_name: FMetasoundFrontendClassName,
    pub version: FMetasoundFrontendVersionNumber,
}
#[repr(C, align(4))]
pub struct FMetasoundFrontendVersionNumber {
    pub major: i32,
    pub minor: i32,
}
#[repr(C, align(4))]
pub struct FMetasoundFrontendClassName {
    pub namespace: FName,
    pub name: FName,
    pub variant: FName,
}
#[repr(C, align(8))]
pub struct FMetaSoundAssetTagCollections {
    pub asset_keys: TArray<FMetaSoundAssetKey>,
}
#[repr(C, align(8))]
pub struct FMetaSoundAssetTagClassCollections {
    pub defined_interfaces: TArray<FMetasoundFrontendInterfaceMetadata>,
    pub inherited_interfaces: TArray<FMetasoundFrontendVersion>,
}
#[repr(C, align(4))]
pub struct FMetasoundFrontendVersion {
    pub name: FName,
    pub number: FMetasoundFrontendVersionNumber,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendInterfaceMetadata {
    pub version: FMetasoundFrontendVersion,
    pub u_class_options: TArray<FMetasoundFrontendInterfaceUClassOptions>,
}
#[repr(C, align(4))]
pub struct FMetasoundFrontendInterfaceUClassOptions {
    pub class_path: FTopLevelAssetPath,
    pub b_is_modifiable: bool,
    pub b_is_default: bool,
}
#[repr(C, align(8))]
pub struct FMetaSoundDocumentInfo {
    pub document_version: FMetasoundFrontendVersionNumber,
    pub referenced_asset_keys: TArray<FMetaSoundAssetKey>,
    pub flags_24: u8,
}
#[repr(C, align(4))]
pub struct FMetasoundCommentNodeIntVector {}
#[repr(C, align(8))]
pub struct FMetaSoundFrontendGraphComment {
    pub color: FLinearColor,
    pub comment: FString,
    pub depth: i32,
    pub font_size: i32,
    pub position: FMetasoundCommentNodeIntVector,
    pub size: FMetasoundCommentNodeIntVector,
    pub move_mode: EMetaSoundFrontendGraphCommentMoveMode,
    pub flags_57: u8,
}
#[repr(C, align(4))]
pub struct FMetasoundFrontendVertex {
    pub name: FName,
    pub type_name: FName,
    pub vertex_id: FGuid,
}
#[repr(C, align(4))]
pub struct FMetasoundFrontendVertexHandle {
    pub node_id: FGuid,
    pub vertex_id: FGuid,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendVertexLiteral {
    pub vertex_id: FGuid,
    pub value: FMetasoundFrontendLiteral,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendLiteral {
    pub ty: EMetasoundFrontendLiteralType,
    pub as_num_default: i32,
    pub as_boolean: TArray<bool>,
    pub as_integer: TArray<i32>,
    pub as_float: TArray<f32>,
    pub as_string: TArray<FString>,
    pub as_u_object: TArray<UPtr<UObject>>,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendVariable {
    pub name: FName,
    pub display_name: FText,
    pub description: FText,
    pub type_name: FName,
    pub literal: FMetasoundFrontendLiteral,
    pub id: FGuid,
    pub variable_node_id: FGuid,
    pub mutator_node_id: FGuid,
    pub accessor_node_i_ds: TArray<FGuid>,
    pub deferred_accessor_node_i_ds: TArray<FGuid>,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendNodeInterface {
    pub inputs: TArray<FMetasoundFrontendVertex>,
    pub outputs: TArray<FMetasoundFrontendVertex>,
    pub environment: TArray<FMetasoundFrontendVertex>,
}
#[repr(C, align(8))]
pub struct FMetaSoundFrontendNodeConfiguration {}
#[repr(C, align(8))]
pub struct FMetasoundFrontendNodeStyleDisplay {
    pub visibility: EMetasoundFrontendNodeStyleDisplayVisibility,
    pub locations: TMap<FGuid, FVector2D>,
    pub comment: FString,
    pub b_comment_visible: bool,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendNodeStyle {
    pub display: FMetasoundFrontendNodeStyleDisplay,
    pub b_message_node_updated: bool,
    pub b_is_private: bool,
    pub b_unconnected_pins_hidden: bool,
}
#[repr(C, align(4))]
pub struct FMetasoundFrontendEdge {
    pub from_node_id: FGuid,
    pub from_vertex_id: FGuid,
    pub to_node_id: FGuid,
    pub to_vertex_id: FGuid,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendEdgeStyleLiteralColorPair {
    pub value: FMetasoundFrontendLiteral,
    pub color: FLinearColor,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendEdgeStyle {
    pub node_id: FGuid,
    pub output_name: FName,
    pub literal_color_pairs: TArray<FMetasoundFrontendEdgeStyleLiteralColorPair>,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendGraphStyle {
    pub b_is_graph_editable: bool,
    pub edge_styles: TArray<FMetasoundFrontendEdgeStyle>,
    pub comments: TMap<FGuid, FMetaSoundFrontendGraphComment>,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendVertexMetadata {
    pub display_name: FText,
    pub display_name_transient: FText,
    pub description: FText,
    pub description_transient: FText,
    pub sort_order_index: i32,
    pub b_is_advanced_display: bool,
    pub b_serialize_text: bool,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendClassVertex {
    pub node_id: FGuid,
    pub metadata: FMetasoundFrontendVertexMetadata,
    pub access_type: EMetasoundFrontendVertexAccessType,
}
#[repr(C, align(4))]
pub struct FMetasoundFrontendClassStyleDisplay {
    pub image_name: FName,
    pub b_show_name: bool,
    pub b_show_input_names: bool,
    pub b_show_output_names: bool,
    pub b_show_literals: bool,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendClassInputDefault {
    pub literal: FMetasoundFrontendLiteral,
    pub page_id: FGuid,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendClassInput {
    pub default_literal: FMetasoundFrontendLiteral,
    pub defaults: TArray<FMetasoundFrontendClassInputDefault>,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendClassVariable {
    pub default_literal: FMetasoundFrontendLiteral,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendClassOutput {}
#[repr(C, align(4))]
pub struct FMetasoundFrontendClassEnvironmentVariable {
    pub name: FName,
    pub type_name: FName,
    pub b_is_required: bool,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendInterfaceStyle {
    pub default_sort_order: TArray<i32>,
    pub required_members: TMap<FName, FText>,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendClassInterface {
    pub input_style: FMetasoundFrontendInterfaceStyle,
    pub output_style: FMetasoundFrontendInterfaceStyle,
    pub inputs: TArray<FMetasoundFrontendClassInput>,
    pub outputs: TArray<FMetasoundFrontendClassOutput>,
    pub environment: TArray<FMetasoundFrontendClassEnvironmentVariable>,
    pub change_id: FGuid,
}
#[repr(C, align(4))]
pub struct FMetasoundFrontendInterfaceVertexBinding {
    pub output_name: FName,
    pub input_name: FName,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendInterfaceBinding {
    pub output_interface_version: FMetasoundFrontendVersion,
    pub input_interface_version: FMetasoundFrontendVersion,
    pub binding_priority: i32,
    pub vertex_bindings: TArray<FMetasoundFrontendInterfaceVertexBinding>,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendInterface {
    pub metadata: FMetasoundFrontendInterfaceMetadata,
    pub version: FMetasoundFrontendVersion,
    pub u_class_options: TArray<FMetasoundFrontendInterfaceUClassOptions>,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendClassMetadata {
    pub class_name: FMetasoundFrontendClassName,
    pub version: FMetasoundFrontendVersionNumber,
    pub ty: EMetasoundFrontendClassType,
    pub display_name: FText,
    pub display_name_transient: FText,
    pub description: FText,
    pub description_transient: FText,
    pub prompt_if_missing_transient: FText,
    pub author: FString,
    pub keywords: TArray<FText>,
    pub keywords_transient: TArray<FText>,
    pub category_hierarchy: TArray<FText>,
    pub category_hierarchy_transient: TArray<FText>,
    pub b_is_deprecated: bool,
    pub access_flags: u16,
    pub b_auto_update_manages_interface: bool,
    pub b_serialize_text: bool,
    pub change_id: FGuid,
}
#[repr(C, align(4))]
pub struct FMetasoundFrontendClassStyle {
    pub display: FMetasoundFrontendClassStyleDisplay,
    pub change_id: FGuid,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendClass {
    pub id: FGuid,
    pub metadata: FMetasoundFrontendClassMetadata,
    pub interface: FMetasoundFrontendClassInterface,
    pub style: FMetasoundFrontendClassStyle,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendNode {
    pub id: FGuid,
    pub class_id: FGuid,
    pub name: FName,
    pub interface: FMetasoundFrontendNodeInterface,
    pub input_literals: TArray<FMetasoundFrontendVertexLiteral>,
    pub configuration: FInstancedStruct,
    pub class_interface_override: FInstancedStruct,
    pub style: FMetasoundFrontendNodeStyle,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendGraphClassPresetOptions {
    pub b_is_preset: bool,
    pub inputs_inheriting_default: TSet<FName>,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendGraph {
    pub nodes: TArray<FMetasoundFrontendNode>,
    pub edges: TArray<FMetasoundFrontendEdge>,
    pub variables: TArray<FMetasoundFrontendVariable>,
    pub style: FMetasoundFrontendGraphStyle,
    pub page_id: FGuid,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendGraphClass {
    pub graph: FMetasoundFrontendGraph,
    pub paged_graphs: TArray<FMetasoundFrontendGraph>,
    pub preset_options: FMetasoundFrontendGraphClassPresetOptions,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendDocumentMetadata {
    pub version: FMetasoundFrontendVersion,
    pub member_metadata: TMap<FGuid, UPtr<UMetaSoundFrontendMemberMetadata>>,
}
#[repr(C, align(8))]
pub struct FMetasoundFrontendDocument {
    pub metadata: FMetasoundFrontendDocumentMetadata,
    pub interfaces: TSet<FMetasoundFrontendVersion>,
    pub root_graph: FMetasoundFrontendGraphClass,
    pub subgraphs: TArray<FMetasoundFrontendGraphClass>,
    pub dependencies: TArray<FMetasoundFrontendClass>,
    pub archetype_version: FMetasoundFrontendVersion,
    pub interface_versions: TArray<FMetasoundFrontendVersion>,
    pub id_counter: u32,
}
#[repr(C, align(8))]
pub struct FMetaSoundFrontendDocumentBuilder {
    pub document_interface: TScriptInterface<IMetaSoundDocumentInterface>,
    pub build_page_id: FGuid,
}
#[repr(C, align(8))]
pub struct FNodeTemplateGenerateInterfaceParams {
    pub inputs_to_connect: TArray<FName>,
    pub outputs_to_connect: TArray<FName>,
}
#[repr(C, align(4))]
pub struct FMetaSoundClassVertexInfo {
    pub name: FName,
    pub type_name: FName,
    pub access_type: EMetasoundFrontendVertexAccessType,
}
#[repr(C, align(8))]
pub struct FMetaSoundClassSearchInfo {
    pub class_display_name: FText,
    pub class_description: FText,
    pub hierarchy: TArray<FText>,
    pub keywords: TArray<FText>,
}
#[repr(C, align(8))]
pub struct FMetaSoundClassVertexCollectionInfo {
    pub class_vertex_info: TArray<FMetaSoundClassVertexInfo>,
}
#[repr(C, align(8))]
pub struct FMetaSoundClassInterfaceInfo {
    pub defined_interfaces: TArray<FMetasoundFrontendInterfaceMetadata>,
    pub search_info: FMetaSoundClassSearchInfo,
    pub inputs: TArray<FMetaSoundClassVertexInfo>,
    pub outputs: TArray<FMetaSoundClassVertexInfo>,
    pub inherited_interfaces: TArray<FMetasoundFrontendVersion>,
}
#[repr(C, align(8))]
pub struct FMetaSoundFrontendTemplateNodeConfiguration {}
pub struct UMetaSoundDocumentInterface {}
pub struct IMetaSoundDocumentInterface {}
pub struct UMetaSoundFrontendMemberMetadata {
    pub member_id: FGuid,
}
pub struct UMetaSoundBuilderDocument {
    pub document: FMetasoundFrontendDocument,
    pub meta_sound_u_class: TSubclassOf<UObject>,
    pub builder_u_class: TSubclassOf<UObject>,
}
pub struct UMetasoundParameterPack {}
