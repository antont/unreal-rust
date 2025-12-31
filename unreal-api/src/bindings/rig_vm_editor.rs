#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FRigVMBlueprintLoadLogEntry {
    pub severity: ERigVMBlueprintLoadLogSeverity,
    pub subject: UPtr<crate::bindings::core_u_object::UObject>,
    pub message: FString,
}
#[repr(C, align(8))]
pub struct FRigVMBlueprintCompiledData {
    pub intermediate_graphs: TArray<UPtr<crate::bindings::engine::UEdGraph>>,
}
#[repr(C, align(8))]
pub struct FRigVMActionMenuItem {}
#[repr(C, align(8))]
pub struct FRigVMEditorGraphMenuContext {
    pub graph: UPtr<crate::bindings::rig_vm_developer::URigVMGraph>,
    pub node: UPtr<crate::bindings::rig_vm_developer::URigVMNode>,
    pub pin: UPtr<crate::bindings::rig_vm_developer::URigVMPin>,
}
pub struct URigVMEdGraphNodeBlueprintSpawner {}
pub struct URigVMBlueprintCompilerExtension {}
pub struct URigVMDetailsViewWrapperObject {
    pub subject_ptr: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
}
pub struct URigVMEditorMenuContext {}
pub struct URigVMEditorBlueprintLibrary {}
pub struct FGetAssetsWithFilter_ForBlueprint_InAssetDataFilter;
pub struct FLoadAssetsWithAssetDataAndBlueprintFilters_ForBlueprint_InAssetDataFilter;
pub struct FLoadAssetsWithAssetDataAndBlueprintFilters_ForBlueprint_InBlueprintFilter;
pub struct FLoadAssetsWithAssetDataAndNodeFilters_ForBlueprint_InAssetDataFilter;
pub struct FLoadAssetsWithAssetDataAndNodeFilters_ForBlueprint_InNodeFilter;
pub struct FLoadAssetsWithAssetDataFilter_ForBlueprint_InAssetDataFilter;
pub struct FLoadAssetsWithBlueprintFilter_ForBlueprint_InBlueprintFilter;
pub struct FLoadAssetsWithNodeFilter_ForBlueprint_InNodeFilter;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigVMBlueprintLoadLogSeverity(pub u8);
impl ERigVMBlueprintLoadLogSeverity {
    pub const DISPLAY: ERigVMBlueprintLoadLogSeverity = ERigVMBlueprintLoadLogSeverity(
        0,
    );
    pub const WARNING: ERigVMBlueprintLoadLogSeverity = ERigVMBlueprintLoadLogSeverity(
        1,
    );
    pub const ERROR: ERigVMBlueprintLoadLogSeverity = ERigVMBlueprintLoadLogSeverity(2);
}
