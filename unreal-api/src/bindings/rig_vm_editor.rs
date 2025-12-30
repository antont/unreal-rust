#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FRigVMBlueprintLoadLogEntry {
    pub severity: ERigVMBlueprintLoadLogSeverity,
    pub subject: UPtr<UObject>,
    pub message: FString,
}
#[repr(C, align(8))]
pub struct FRigVMBlueprintCompiledData {
    pub intermediate_graphs: TArray<UPtr<UEdGraph>>,
}
#[repr(C, align(8))]
pub struct FRigVMActionMenuItem {}
#[repr(C, align(8))]
pub struct FRigVMEditorGraphMenuContext {
    pub graph: UPtr<URigVMGraph>,
    pub node: UPtr<URigVMNode>,
    pub pin: UPtr<URigVMPin>,
}
pub struct URigVMEdGraphNodeBlueprintSpawner {}
pub struct URigVMBlueprintCompilerExtension {}
pub struct URigVMDetailsViewWrapperObject {
    pub subject_ptr: TWeakObjectPtr<UObject>,
}
pub struct URigVMEditorMenuContext {}
pub struct URigVMEditorBlueprintLibrary {}
