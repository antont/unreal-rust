#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UStateTreeEditorContext {
    __padding_end: [u8; 64],
}
impl UStateTreeEditorContext {}
#[repr(C, align(8))]
pub struct UAssetDefinition_StateTree {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_StateTree {}
#[repr(C, align(8))]
pub struct UStateTreeCompileAllCommandlet {
    __padding_end: [u8; 144],
}
impl UStateTreeCompileAllCommandlet {}
#[repr(C, align(8))]
pub struct UStateTreePropertyRefSchema {
    __padding_end: [u8; 152],
}
impl UStateTreePropertyRefSchema {}
#[repr(C, align(8))]
pub struct UK2Node_StateTreeBlueprintPropertyRef {
    __padding_end: [u8; 192],
}
impl UK2Node_StateTreeBlueprintPropertyRef {}
#[repr(C, align(8))]
pub struct UK2Node_StateTreeNodeGetPropertyDescription {
    __padding_end: [u8; 248],
}
impl UK2Node_StateTreeNodeGetPropertyDescription {}
#[repr(C, align(8))]
pub struct UK2Node_MakeStateTreeReference {
    __padding_end: [u8; 224],
}
impl UK2Node_MakeStateTreeReference {}
#[repr(C, align(8))]
pub struct UStateTreeEditingSubsystem {
    __padding_end: [u8; 152],
}
impl UStateTreeEditingSubsystem {}
#[repr(C, align(16))]
pub struct UStateTreeEditorData {
    __padding_end: [u8; 576],
}
impl UStateTreeEditorData {}
#[repr(C, align(16))]
pub struct UQAStateTreeEditorData {
    __padding_end: [u8; 576],
}
impl UQAStateTreeEditorData {}
#[repr(C, align(8))]
pub struct UStateTreeEditorDataExtension {
    __padding_end: [u8; 48],
}
impl UStateTreeEditorDataExtension {}
#[repr(C, align(8))]
pub struct UStateTreeEditorMode {
    __padding_end: [u8; 376],
}
impl UStateTreeEditorMode {}
pub struct IStateTreeEditorPropertyBindingsOwner {}
#[repr(C, align(8))]
pub struct UStateTreeEditorPropertyBindingsOwner {
    __padding_end: [u8; 48],
}
impl UStateTreeEditorPropertyBindingsOwner {}
#[repr(C, align(8))]
pub struct UStateTreeEditorSchema {
    __padding_end: [u8; 48],
}
impl UStateTreeEditorSchema {}
#[repr(C, align(8))]
pub struct UStateTreeEditorSettings {
    __padding_end: [u8; 112],
}
impl UStateTreeEditorSettings {}
#[repr(C, align(8))]
pub struct UStateTreeEditorUISubsystem {
    __padding_end: [u8; 56],
}
impl UStateTreeEditorUISubsystem {}
#[repr(C, align(8))]
pub struct UStateTreeEditorUserSettings {
    __padding_end: [u8; 152],
}
impl UStateTreeEditorUserSettings {}
#[repr(C, align(8))]
pub struct UStateTreeFactory {
    __padding_end: [u8; 144],
}
impl UStateTreeFactory {}
#[repr(C, align(8))]
pub struct UStateTreeState {
    __padding_end: [u8; 528],
}
impl UStateTreeState {}
#[repr(C, align(8))]
pub struct UStateTreeClipboardBindings {
    __padding_end: [u8; 64],
}
impl UStateTreeClipboardBindings {}
#[repr(transparent)]
pub struct EStateTreeSaveOnCompile(pub u8);
impl EStateTreeSaveOnCompile {
    pub const NEVER: EStateTreeSaveOnCompile = EStateTreeSaveOnCompile(0);
    pub const SUCCESS_ONLY: EStateTreeSaveOnCompile = EStateTreeSaveOnCompile(1);
    pub const ALWAYS: EStateTreeSaveOnCompile = EStateTreeSaveOnCompile(2);
}
#[repr(transparent)]
pub struct EStateTreeEditorUserSettingsNodeType(pub u8);
impl EStateTreeEditorUserSettingsNodeType {
    pub const CONDITION: EStateTreeEditorUserSettingsNodeType = EStateTreeEditorUserSettingsNodeType(
        1,
    );
    pub const TASK: EStateTreeEditorUserSettingsNodeType = EStateTreeEditorUserSettingsNodeType(
        2,
    );
    pub const TRANSITION: EStateTreeEditorUserSettingsNodeType = EStateTreeEditorUserSettingsNodeType(
        4,
    );
    pub const FLAG: EStateTreeEditorUserSettingsNodeType = EStateTreeEditorUserSettingsNodeType(
        8,
    );
    pub const ALL: EStateTreeEditorUserSettingsNodeType = EStateTreeEditorUserSettingsNodeType(
        15,
    );
}
