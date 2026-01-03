#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FRigVMGraphVariableDescription {
    pub name: FName,
    pub cpp_type: FString,
    pub cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
    #[doc(hidden)]
    __padding_56: [u8; 16],
    pub default_value: FString,
    pub category: FText,
    pub tooltip: FText,
    pub b_exposed_on_spawn: bool,
    pub b_expose_to_cinematics: bool,
    pub b_public: bool,
    pub b_private: bool,
    __padding_end: [u8; 4],
}
impl FRigVMGraphVariableDescription {}
#[repr(C, align(8))]
pub struct FRigVMParserASTSettings {
    #[doc(hidden)]
    __padding_3: [u8; 3],
    pub b_setup_traits: bool,
    __padding_end: [u8; 52],
}
impl FRigVMParserASTSettings {}
#[repr(C, align(8))]
pub struct FRigVMCompileSettings {
    pub surpress_info_messages: bool,
    pub surpress_warnings: bool,
    pub surpress_errors: bool,
    pub enable_pin_watches: bool,
    pub ast_settings: FRigVMParserASTSettings,
    #[doc(hidden)]
    __padding_66: [u8; 2],
    pub b_warn_about_deprecated_nodes: bool,
    pub b_warn_about_duplicate_events: bool,
    __padding_end: [u8; 4],
}
impl FRigVMCompileSettings {}
#[repr(C, align(8))]
pub struct FRigVMGraphParameterDescription {
    pub name: FName,
    pub b_is_input: bool,
    pub cpp_type: FString,
    pub cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub default_value: FString,
}
impl FRigVMGraphParameterDescription {}
#[repr(C, align(8))]
pub struct FRigVMFunctionReferenceArray {
    __padding_end: [u8; 16],
}
impl FRigVMFunctionReferenceArray {}
#[repr(C, align(8))]
pub struct FRigStructScope {
    __padding_end: [u8; 16],
}
impl FRigStructScope {}
#[repr(C, align(8))]
pub struct URigVMEdGraph {
    __padding_end: [u8; 456],
}
impl URigVMEdGraph {}
#[repr(C, align(8))]
pub struct URigVMEdGraphNode {
    __padding_end: [u8; 1152],
}
impl URigVMEdGraphNode {}
#[repr(C, align(8))]
pub struct URigVMEdGraphSchema {
    __padding_end: [u8; 168],
}
impl URigVMEdGraphSchema {}
pub struct IRigVMAssetInterface {}
#[repr(C, align(8))]
pub struct URigVMAssetInterface {
    __padding_end: [u8; 48],
}
impl URigVMAssetInterface {}
#[repr(C, align(8))]
pub struct URigVMBlueprint {
    #[doc(hidden)]
    __padding_2992: [u8; 2992],
    pub vm_compile_settings: FRigVMCompileSettings,
    __padding_end: [u8; 744],
}
impl URigVMBlueprint {}
#[repr(C, align(8))]
pub struct URigVMCompiler {
    __padding_end: [u8; 312],
}
impl URigVMCompiler {}
#[repr(C, align(8))]
pub struct URigVMNode {
    __padding_end: [u8; 544],
}
impl URigVMNode {}
#[repr(C, align(8))]
pub struct URigVMTemplateNode {
    __padding_end: [u8; 632],
}
impl URigVMTemplateNode {}
#[repr(C, align(8))]
pub struct URigVMLibraryNode {
    __padding_end: [u8; 632],
}
impl URigVMLibraryNode {}
#[repr(C, align(8))]
pub struct URigVMCollapseNode {
    __padding_end: [u8; 688],
}
impl URigVMCollapseNode {}
#[repr(C, align(8))]
pub struct URigVMAggregateNode {
    __padding_end: [u8; 704],
}
impl URigVMAggregateNode {}
#[repr(C, align(8))]
pub struct UDEPRECATED_RigVMArrayNode {
    __padding_end: [u8; 640],
}
impl UDEPRECATED_RigVMArrayNode {}
#[repr(C, align(8))]
pub struct UDEPRECATED_RigVMBranchNode {
    __padding_end: [u8; 544],
}
impl UDEPRECATED_RigVMBranchNode {}
#[repr(C, align(8))]
pub struct URigVMCommentNode {
    __padding_end: [u8; 568],
}
impl URigVMCommentNode {}
#[repr(C, align(8))]
pub struct URigVMDispatchNode {
    __padding_end: [u8; 720],
}
impl URigVMDispatchNode {}
#[repr(C, align(8))]
pub struct URigVMEnumNode {
    __padding_end: [u8; 544],
}
impl URigVMEnumNode {}
#[repr(C, align(8))]
pub struct URigVMFunctionInterfaceNode {
    __padding_end: [u8; 632],
}
impl URigVMFunctionInterfaceNode {}
#[repr(C, align(8))]
pub struct URigVMFunctionEntryNode {
    __padding_end: [u8; 632],
}
impl URigVMFunctionEntryNode {}
#[repr(C, align(8))]
pub struct URigVMFunctionReferenceNode {
    __padding_end: [u8; 1288],
}
impl URigVMFunctionReferenceNode {}
#[repr(C, align(8))]
pub struct URigVMFunctionReturnNode {
    __padding_end: [u8; 632],
}
impl URigVMFunctionReturnNode {}
#[repr(C, align(8))]
pub struct UDEPRECATED_RigVMIfNode {
    __padding_end: [u8; 632],
}
impl UDEPRECATED_RigVMIfNode {}
#[repr(C, align(8))]
pub struct URigVMInvokeEntryNode {
    __padding_end: [u8; 544],
}
impl URigVMInvokeEntryNode {}
#[repr(C, align(8))]
pub struct URigVMParameterNode {
    __padding_end: [u8; 544],
}
impl URigVMParameterNode {}
#[repr(C, align(8))]
pub struct URigVMRerouteNode {
    __padding_end: [u8; 544],
}
impl URigVMRerouteNode {}
#[repr(C, align(8))]
pub struct UDEPRECATED_RigVMSelectNode {
    __padding_end: [u8; 632],
}
impl UDEPRECATED_RigVMSelectNode {}
#[repr(C, align(8))]
pub struct URigVMUnitNode {
    __padding_end: [u8; 680],
}
impl URigVMUnitNode {}
#[repr(C, align(8))]
pub struct URigVMVariableNode {
    __padding_end: [u8; 544],
}
impl URigVMVariableNode {}
#[repr(C, align(8))]
pub struct URigVMBuildData {
    __padding_end: [u8; 216],
}
impl URigVMBuildData {}
pub struct IRigVMClientHost {}
#[repr(C, align(8))]
pub struct URigVMClientHost {
    __padding_end: [u8; 48],
}
impl URigVMClientHost {}
pub struct IRigVMEditorSideObject {}
#[repr(C, align(8))]
pub struct URigVMEditorSideObject {
    __padding_end: [u8; 48],
}
impl URigVMEditorSideObject {}
pub struct IRigVMClientExternalModelHost {}
#[repr(C, align(8))]
pub struct URigVMClientExternalModelHost {
    __padding_end: [u8; 48],
}
impl URigVMClientExternalModelHost {}
#[repr(C, align(8))]
pub struct URigVMActionStack {
    __padding_end: [u8; 152],
}
impl URigVMActionStack {}
pub struct IRigVMExternalDependencyManager {}
#[repr(C, align(8))]
pub struct URigVMExternalDependencyManager {
    __padding_end: [u8; 48],
}
impl URigVMExternalDependencyManager {}
#[repr(C, align(8))]
pub struct URigVMGraph {
    __padding_end: [u8; 296],
}
impl URigVMGraph {}
#[repr(C, align(8))]
pub struct URigVMFunctionLibrary {
    __padding_end: [u8; 576],
}
impl URigVMFunctionLibrary {}
#[repr(C, align(8))]
pub struct URigVMLink {
    __padding_end: [u8; 96],
}
impl URigVMLink {}
#[repr(C, align(8))]
pub struct URigVMInjectionInfo {
    __padding_end: [u8; 88],
}
impl URigVMInjectionInfo {}
#[repr(C, align(8))]
pub struct URigVMPin {
    __padding_end: [u8; 544],
}
impl URigVMPin {}
#[repr(C, align(8))]
pub struct URigVMSchema {
    __padding_end: [u8; 80],
}
impl URigVMSchema {}
#[repr(C, align(8))]
pub struct URigVMUserWorkflowRegistry {
    __padding_end: [u8; 72],
}
impl URigVMUserWorkflowRegistry {}
#[repr(C, align(8))]
pub struct URigVMController {
    #[doc(hidden)]
    __padding_296: [u8; 296],
    pub modified_event_dynamic: FRigVMController_ModifiedEventDynamic,
    __padding_end: [u8; 408],
}
impl URigVMController {}
#[repr(C, align(8))]
pub struct URigVMControllerSettings {
    __padding_end: [u8; 136],
}
impl URigVMControllerSettings {}
#[repr(C, align(8))]
pub struct FRegisterProvider_InProvider {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FRigVMController_ModifiedEventDynamic {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ERigVMTagDisplayMode(pub u8);
impl ERigVMTagDisplayMode {
    pub const NONE: ERigVMTagDisplayMode = ERigVMTagDisplayMode(0);
    pub const ALL: ERigVMTagDisplayMode = ERigVMTagDisplayMode(1);
    pub const DEPRECATION_ONLY: ERigVMTagDisplayMode = ERigVMTagDisplayMode(2);
    pub const LAST: ERigVMTagDisplayMode = ERigVMTagDisplayMode(2);
}
#[repr(transparent)]
pub struct ERigVMPinDefaultValueType(pub u8);
impl ERigVMPinDefaultValueType {
    pub const AUTO_DETECT: ERigVMPinDefaultValueType = ERigVMPinDefaultValueType(0);
    pub const UNSET: ERigVMPinDefaultValueType = ERigVMPinDefaultValueType(1);
    pub const OVERRIDE: ERigVMPinDefaultValueType = ERigVMPinDefaultValueType(2);
    pub const KEEP_VALUE_TYPE: ERigVMPinDefaultValueType = ERigVMPinDefaultValueType(3);
}
#[repr(transparent)]
pub struct ERigVMNodeColorType(pub u8);
impl ERigVMNodeColorType {
    pub const FROM_METADATA: ERigVMNodeColorType = ERigVMNodeColorType(0);
    pub const USER_DEFINED: ERigVMNodeColorType = ERigVMNodeColorType(1);
}
#[repr(transparent)]
pub struct ERigVMGraphNotifType(pub u8);
impl ERigVMGraphNotifType {
    pub const GRAPH_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(0);
    pub const NODE_ADDED: ERigVMGraphNotifType = ERigVMGraphNotifType(1);
    pub const NODE_REMOVED: ERigVMGraphNotifType = ERigVMGraphNotifType(2);
    pub const NODE_SELECTED: ERigVMGraphNotifType = ERigVMGraphNotifType(3);
    pub const NODE_DESELECTED: ERigVMGraphNotifType = ERigVMGraphNotifType(4);
    pub const NODE_SELECTION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(5);
    pub const NODE_POSITION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(6);
    pub const NODE_SIZE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(7);
    pub const NODE_TITLE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(8);
    pub const NODE_COLOR_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(9);
    pub const PIN_ADDED: ERigVMGraphNotifType = ERigVMGraphNotifType(10);
    pub const PIN_REMOVED: ERigVMGraphNotifType = ERigVMGraphNotifType(11);
    pub const PIN_RENAMED: ERigVMGraphNotifType = ERigVMGraphNotifType(12);
    pub const PIN_EXPANSION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(13);
    pub const PIN_WATCHED_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(14);
    pub const PIN_ARRAY_SIZE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(15);
    pub const PIN_DEFAULT_VALUE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(16);
    pub const PIN_DIRECTION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(17);
    pub const PIN_TYPE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(18);
    pub const PIN_INDEX_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(19);
    pub const LINK_ADDED: ERigVMGraphNotifType = ERigVMGraphNotifType(20);
    pub const LINK_REMOVED: ERigVMGraphNotifType = ERigVMGraphNotifType(21);
    pub const COMMENT_TEXT_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(22);
    pub const VARIABLE_ADDED: ERigVMGraphNotifType = ERigVMGraphNotifType(23);
    pub const VARIABLE_REMOVED: ERigVMGraphNotifType = ERigVMGraphNotifType(24);
    pub const VARIABLE_RENAMED: ERigVMGraphNotifType = ERigVMGraphNotifType(25);
    pub const INTERACTION_BRACKET_OPENED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        26,
    );
    pub const INTERACTION_BRACKET_CLOSED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        27,
    );
    pub const INTERACTION_BRACKET_CANCELED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        28,
    );
    pub const PIN_BOUND_VARIABLE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        29,
    );
    pub const NODE_RENAMED: ERigVMGraphNotifType = ERigVMGraphNotifType(30);
    pub const FUNCTION_RENAMED: ERigVMGraphNotifType = ERigVMGraphNotifType(31);
    pub const NODE_REFERENCE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(32);
    pub const NODE_CATEGORY_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(33);
    pub const NODE_KEYWORDS_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(34);
    pub const NODE_DESCRIPTION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(35);
    pub const VARIABLE_REMAPPING_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        36,
    );
    pub const LIBRARY_TEMPLATE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(37);
    pub const FUNCTION_ACCESS_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(38);
    pub const VARIANT_TAGS_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(39);
    pub const PIN_DISPLAY_NAME_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(40);
    pub const PIN_CATEGORY_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(41);
    pub const PIN_CATEGORIES_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(42);
    pub const PIN_CATEGORY_EXPANSION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        43,
    );
    pub const FUNCTION_VARIANT_GUID_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        44,
    );
    pub const NODE_EARLY_EXIT_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(45);
    pub const LOCAL_VARIABLE_DEFAULT_VALUE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        46,
    );
    pub const LOCAL_VARIABLE_ADDED: ERigVMGraphNotifType = ERigVMGraphNotifType(47);
    pub const LOCAL_VARIABLE_REMOVED: ERigVMGraphNotifType = ERigVMGraphNotifType(48);
    pub const LOCAL_VARIABLE_RENAMED: ERigVMGraphNotifType = ERigVMGraphNotifType(49);
    pub const LOCAL_VARIABLE_TYPE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        50,
    );
    pub const INVALID: ERigVMGraphNotifType = ERigVMGraphNotifType(51);
}
