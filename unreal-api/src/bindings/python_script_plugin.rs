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
    pub legacy_int_deprecated: i32,
    pub bool_instance_only: bool,
    pub bool_defaults_only: bool,
    pub bool_mutable: bool,
}
#[repr(C, align(8))]
pub struct FPythonLogOutputEntry {
    pub ty: EPythonLogOutputType,
    pub output: FString,
}
#[repr(C, align(8))]
pub struct FPyTestChildStruct {}
#[repr(C, align(4))]
pub struct FPyTestClassSparseData {
    pub int_from_sparse_data: i32,
}
pub struct UPyTestInterface {}
pub struct IPyTestInterface {}
pub struct UPyTestChildInterface {}
pub struct IPyTestChildInterface {}
pub struct UPyTestOtherInterface {}
pub struct IPyTestOtherInterface {}
pub struct UEditorPythonScriptingLibrary {}
pub struct UK2Node_ExecutePythonScript {
    pub inputs: TArray<FName>,
    pub outputs: TArray<FName>,
}
pub struct UPyTestStructLibrary {}
pub struct UPyTestObject {
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
    pub multicast_delegate: FPyTestObject_MulticastDelegate,
    pub _struct: FPyTestStruct,
    pub struct_array: TArray<FPyTestStruct>,
    pub child_struct: FPyTestChildStruct,
    pub bool_instance_only: bool,
    pub bool_defaults_only: bool,
}
pub struct UPyTestChildObject {}
pub struct UDEPRECATED_LegacyPyTestObject {}
pub struct UPyTestObjectLibrary {}
pub struct UPyTestStructDelegate {
    pub on_name_collision_test_delegate: FPyTestStructDelegate_OnNameCollisionTestDelegate,
}
pub struct UPyTestVectorDelegate {
    pub on_name_collision_test_delegate: FPyTestVectorDelegate_OnNameCollisionTestDelegate,
}
pub struct UPyTestTypeHint {
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
    pub multicast_delegate_prop: FPyTestTypeHint_MulticastDelegateProp,
    pub slate_tick_delegate: FPyTestTypeHint_SlateTickDelegate,
}
pub struct UPythonOnlineDocsCommandlet {}
pub struct UPythonScriptCommandlet {}
pub struct UPythonScriptLibrary {}
pub struct UPythonScriptPluginSettings {
    pub startup_scripts: TArray<FString>,
    pub additional_paths: TArray<crate::bindings::core_u_object::FDirectoryPath>,
    pub b_isolate_interpreter_environment: bool,
    pub b_developer_mode: bool,
    pub b_run_pip_install_on_startup: bool,
    pub b_pip_strict_hash_check: bool,
    pub b_offline_only: bool,
    pub override_index_url: FString,
    pub extra_install_args: FString,
    pub b_remote_execution: bool,
    pub remote_execution_multicast_group_endpoint: FString,
    pub remote_execution_multicast_bind_address: FString,
    pub remote_execution_send_buffer_size_bytes: i32,
    pub remote_execution_receive_buffer_size_bytes: i32,
    pub remote_execution_multicast_ttl: u8,
}
pub struct UPythonScriptPluginUserSettings {
    pub enable_python_override: EPythonEnabledOverrideState,
    pub b_developer_mode: bool,
    pub type_hinting_mode: ETypeHintingMode,
    pub b_enable_content_browser_integration: bool,
}
pub struct UPythonResourceOwner {}
pub struct IPythonResourceOwner {}
pub struct UPythonObjectHandle {}
pub struct UPythonCallableForDelegate {}
pub struct UPythonGeneratedEnum {}
pub struct UPythonGeneratedClass {}
pub struct UPythonGeneratedStruct {}
pub struct FFuncTakingPyTestDelegate_InDelegate;
pub struct FCheckDelegateTypeHints_Param1;
pub struct FCheckDelegateTypeHints_ReturnValue;
pub struct FPyTestObject_Delegate;
pub struct FPyTestObject_MulticastDelegate;
pub struct FPyTestStructDelegate_OnNameCollisionTestDelegate;
pub struct FPyTestVectorDelegate_OnNameCollisionTestDelegate;
pub struct FPyTestTypeHint_DelegateProp;
pub struct FPyTestTypeHint_MulticastDelegateProp;
pub struct FPyTestTypeHint_SlateTickDelegate;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPyTestEnum(pub u8);
impl EPyTestEnum {
    pub const ONE: EPyTestEnum = EPyTestEnum(0);
    pub const TWO: EPyTestEnum = EPyTestEnum(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPythonLogOutputType(pub u8);
impl EPythonLogOutputType {
    pub const INFO: EPythonLogOutputType = EPythonLogOutputType(0);
    pub const WARNING: EPythonLogOutputType = EPythonLogOutputType(1);
    pub const ERROR: EPythonLogOutputType = EPythonLogOutputType(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPythonFileExecutionScope(pub u8);
impl EPythonFileExecutionScope {
    pub const PRIVATE: EPythonFileExecutionScope = EPythonFileExecutionScope(0);
    pub const PUBLIC: EPythonFileExecutionScope = EPythonFileExecutionScope(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPythonEnabledOverrideState(pub u8);
impl EPythonEnabledOverrideState {
    pub const NONE: EPythonEnabledOverrideState = EPythonEnabledOverrideState(0);
    pub const ENABLE: EPythonEnabledOverrideState = EPythonEnabledOverrideState(1);
    pub const DISABLE: EPythonEnabledOverrideState = EPythonEnabledOverrideState(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETypeHintingMode(pub u8);
impl ETypeHintingMode {
    pub const OFF: ETypeHintingMode = ETypeHintingMode(0);
    pub const AUTO_COMPLETION: ETypeHintingMode = ETypeHintingMode(1);
    pub const TYPE_CHECKER: ETypeHintingMode = ETypeHintingMode(2);
}
