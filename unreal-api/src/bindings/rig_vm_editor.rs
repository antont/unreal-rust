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
impl FRigVMBlueprintLoadLogEntry {}
#[repr(C, align(8))]
pub struct FRigVMEditorGraphMenuContext {
    pub graph: UPtr<crate::bindings::rig_vm_developer::URigVMGraph>,
    pub node: UPtr<crate::bindings::rig_vm_developer::URigVMNode>,
    pub pin: UPtr<crate::bindings::rig_vm_developer::URigVMPin>,
}
impl FRigVMEditorGraphMenuContext {}
#[repr(C, align(8))]
pub struct URigVMEdGraphNodeBlueprintSpawner {
    __padding_end: [u8; 288],
}
impl URigVMEdGraphNodeBlueprintSpawner {}
#[repr(C, align(8))]
pub struct URigVMBlueprintCompilerExtension {
    __padding_end: [u8; 48],
}
impl URigVMBlueprintCompilerExtension {}
#[repr(C, align(8))]
pub struct URigVMDetailsViewWrapperObject {
    __padding_end: [u8; 96],
}
impl URigVMDetailsViewWrapperObject {}
#[repr(C, align(8))]
pub struct URigVMEditorMenuContext {
    __padding_end: [u8; 88],
}
impl URigVMEditorMenuContext {}
#[repr(C, align(8))]
pub struct URigVMEditorBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl URigVMEditorBlueprintLibrary {}
#[repr(transparent)]
pub struct FGetAssetsWithFilter_ForBlueprint_InAssetDataFilter {
    _opague: u8,
}
#[repr(transparent)]
pub struct FLoadAssetsWithAssetDataAndBlueprintFilters_ForBlueprint_InAssetDataFilter {
    _opague: u8,
}
#[repr(transparent)]
pub struct FLoadAssetsWithAssetDataAndBlueprintFilters_ForBlueprint_InBlueprintFilter {
    _opague: u8,
}
#[repr(transparent)]
pub struct FLoadAssetsWithAssetDataAndNodeFilters_ForBlueprint_InAssetDataFilter {
    _opague: u8,
}
#[repr(transparent)]
pub struct FLoadAssetsWithAssetDataAndNodeFilters_ForBlueprint_InNodeFilter {
    _opague: u8,
}
#[repr(transparent)]
pub struct FLoadAssetsWithAssetDataFilter_ForBlueprint_InAssetDataFilter {
    _opague: u8,
}
#[repr(transparent)]
pub struct FLoadAssetsWithBlueprintFilter_ForBlueprint_InBlueprintFilter {
    _opague: u8,
}
#[repr(transparent)]
pub struct FLoadAssetsWithNodeFilter_ForBlueprint_InNodeFilter {
    _opague: u8,
}
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
