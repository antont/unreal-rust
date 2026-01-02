#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FPyTestStruct {
    pub bool: bool,
    pub int: i32,
    pub float: f32,
    pub enum_: EPyTestEnum,
    pub string: FString,
    pub name: FName,
    pub text: FText,
    pub field_path: TFieldPath<FProperty>,
    pub struct_field_path: TFieldPath<FStructProperty>,
    pub string_array: TArray<FString>,
    pub string_set: TSet<FString>,
    pub string_int_map: TMap<FString, i32>,
    #[doc(hidden)]
    __padding_342: [u8; 6],
    pub bool_mutable: bool,
    __padding_end: [u8; 1],
}
impl FPyTestStruct {}
#[repr(C, align(8))]
pub struct FPythonLogOutputEntry {
    pub ty: EPythonLogOutputType,
    pub output: FString,
}
impl FPythonLogOutputEntry {}
#[repr(C, align(8))]
pub struct FPyTestChildStruct {
    __padding_end: [u8; 344],
}
impl FPyTestChildStruct {}
#[repr(C, align(4))]
pub struct FPyTestClassSparseData {
    pub int_from_sparse_data: i32,
}
impl FPyTestClassSparseData {}
pub struct UPyTestInterface {}
pub struct IPyTestInterface {}
pub struct UPyTestChildInterface {}
pub struct IPyTestChildInterface {}
pub struct UPyTestOtherInterface {}
pub struct IPyTestOtherInterface {}
#[repr(C, align(8))]
pub struct UEditorPythonScriptingLibrary {
    __padding_end: [u8; 48],
}
impl UEditorPythonScriptingLibrary {}
#[repr(C, align(8))]
pub struct UK2Node_ExecutePythonScript {
    __padding_end: [u8; 368],
}
impl UK2Node_ExecutePythonScript {}
#[repr(C, align(8))]
pub struct UPyTestStructLibrary {
    __padding_end: [u8; 48],
}
impl UPyTestStructLibrary {}
#[repr(C, align(8))]
pub struct UPyTestObject {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub bool: bool,
    pub int: i32,
    pub float: f32,
    pub enum_: EPyTestEnum,
    pub string: FString,
    pub name: FName,
    pub text: FText,
    pub field_path: TFieldPath<FProperty>,
    pub struct_field_path: TFieldPath<FStructProperty>,
    pub string_array: TArray<FString>,
    pub string_set: TSet<FString>,
    pub string_int_map: TMap<FString, i32>,
    pub delegate: FPyTestObject_Delegate,
    #[doc(hidden)]
    __padding_456: [u8; 24],
    pub _struct: FPyTestStruct,
    pub struct_array: TArray<FPyTestStruct>,
    pub child_struct: FPyTestChildStruct,
    __padding_end: [u8; 8],
}
impl UPyTestObject {}
#[repr(C, align(8))]
pub struct UPyTestChildObject {
    __padding_end: [u8; 1168],
}
impl UPyTestChildObject {}
#[repr(C, align(8))]
pub struct UDEPRECATED_LegacyPyTestObject {
    __padding_end: [u8; 1168],
}
impl UDEPRECATED_LegacyPyTestObject {}
#[repr(C, align(8))]
pub struct UPyTestObjectLibrary {
    __padding_end: [u8; 48],
}
impl UPyTestObjectLibrary {}
#[repr(C, align(8))]
pub struct UPyTestStructDelegate {
    __padding_end: [u8; 72],
}
impl UPyTestStructDelegate {}
#[repr(C, align(8))]
pub struct UPyTestVectorDelegate {
    __padding_end: [u8; 72],
}
impl UPyTestVectorDelegate {}
#[repr(C, align(8))]
pub struct UPyTestTypeHint {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub bool_prop: bool,
    pub int_prop: i32,
    pub float_prop: f32,
    pub enum_prop: EPyTestEnum,
    pub string_prop: FString,
    pub name_prop: FName,
    pub text_prop: FText,
    pub field_path_prop: TFieldPath<FProperty>,
    pub struct_prop: FPyTestStruct,
    pub object_prop: UPtr<UPyTestObject>,
    pub str_array_prop: TArray<FString>,
    pub name_array_prop: TArray<FName>,
    pub text_array_prop: TArray<FText>,
    pub object_array_prop: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub set_prop: TSet<FString>,
    pub map_prop: TMap<i32, FString>,
    pub delegate_prop: FPyTestTypeHint_DelegateProp,
    #[doc(hidden)]
    __padding_792: [u8; 24],
    pub slate_tick_delegate: FPyTestTypeHint_SlateTickDelegate,
}
impl UPyTestTypeHint {}
#[repr(C, align(8))]
pub struct UPythonOnlineDocsCommandlet {
    __padding_end: [u8; 136],
}
impl UPythonOnlineDocsCommandlet {}
#[repr(C, align(8))]
pub struct UPythonScriptCommandlet {
    __padding_end: [u8; 136],
}
impl UPythonScriptCommandlet {}
#[repr(C, align(8))]
pub struct UPythonScriptLibrary {
    __padding_end: [u8; 48],
}
impl UPythonScriptLibrary {}
#[repr(C, align(8))]
pub struct UPythonScriptPluginSettings {
    __padding_end: [u8; 232],
}
impl UPythonScriptPluginSettings {}
#[repr(C, align(8))]
pub struct UPythonScriptPluginUserSettings {
    __padding_end: [u8; 112],
}
impl UPythonScriptPluginUserSettings {}
pub struct UPythonResourceOwner {}
pub struct IPythonResourceOwner {}
#[repr(C, align(8))]
pub struct UPythonObjectHandle {
    __padding_end: [u8; 64],
}
impl UPythonObjectHandle {}
#[repr(C, align(8))]
pub struct UPythonCallableForDelegate {
    __padding_end: [u8; 64],
}
impl UPythonCallableForDelegate {}
#[repr(C, align(8))]
pub struct UPythonGeneratedEnum {
    __padding_end: [u8; 216],
}
impl UPythonGeneratedEnum {}
#[repr(C, align(8))]
pub struct UPythonGeneratedClass {
    __padding_end: [u8; 1048],
}
impl UPythonGeneratedClass {}
#[repr(C, align(8))]
pub struct UPythonGeneratedStruct {
    __padding_end: [u8; 1040],
}
impl UPythonGeneratedStruct {}
#[repr(C, align(8))]
pub struct FFuncTakingPyTestDelegate_InDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCheckDelegateTypeHints_Param1 {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCheckDelegateTypeHints_ReturnValue {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FPyTestObject_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FPyTestObject_MulticastDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPyTestStructDelegate_OnNameCollisionTestDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPyTestVectorDelegate_OnNameCollisionTestDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPyTestTypeHint_DelegateProp {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FPyTestTypeHint_MulticastDelegateProp {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPyTestTypeHint_SlateTickDelegate {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct EPyTestEnum(pub u8);
impl EPyTestEnum {
    pub const ONE: EPyTestEnum = EPyTestEnum(0);
    pub const TWO: EPyTestEnum = EPyTestEnum(1);
}
#[repr(transparent)]
pub struct EPythonLogOutputType(pub u8);
impl EPythonLogOutputType {
    pub const INFO: EPythonLogOutputType = EPythonLogOutputType(0);
    pub const WARNING: EPythonLogOutputType = EPythonLogOutputType(1);
    pub const ERROR: EPythonLogOutputType = EPythonLogOutputType(2);
}
#[repr(transparent)]
pub struct EPythonCommandExecutionMode(pub u8);
impl EPythonCommandExecutionMode {
    pub const EXECUTE_FILE: EPythonCommandExecutionMode = EPythonCommandExecutionMode(0);
    pub const EXECUTE_STATEMENT: EPythonCommandExecutionMode = EPythonCommandExecutionMode(
        1,
    );
    pub const EVALUATE_STATEMENT: EPythonCommandExecutionMode = EPythonCommandExecutionMode(
        2,
    );
}
#[repr(transparent)]
pub struct EPythonFileExecutionScope(pub u8);
impl EPythonFileExecutionScope {
    pub const PRIVATE: EPythonFileExecutionScope = EPythonFileExecutionScope(0);
    pub const PUBLIC: EPythonFileExecutionScope = EPythonFileExecutionScope(1);
}
#[repr(transparent)]
pub struct EPythonEnabledOverrideState(pub u8);
impl EPythonEnabledOverrideState {
    pub const NONE: EPythonEnabledOverrideState = EPythonEnabledOverrideState(0);
    pub const ENABLE: EPythonEnabledOverrideState = EPythonEnabledOverrideState(1);
    pub const DISABLE: EPythonEnabledOverrideState = EPythonEnabledOverrideState(2);
}
#[repr(transparent)]
pub struct ETypeHintingMode(pub u8);
impl ETypeHintingMode {
    pub const OFF: ETypeHintingMode = ETypeHintingMode(0);
    pub const AUTO_COMPLETION: ETypeHintingMode = ETypeHintingMode(1);
    pub const TYPE_CHECKER: ETypeHintingMode = ETypeHintingMode(2);
}
