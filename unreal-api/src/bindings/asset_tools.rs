#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAssetRenameData {
    pub asset: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
    pub new_package_path: FString,
    pub new_name: FString,
    __padding_end: [u8; 88],
}
impl FAssetRenameData {}
#[repr(C, align(8))]
pub struct FMigrationOptions {
    pub b_prompt: bool,
    pub b_ignore_dependencies: bool,
    pub asset_conflict: EAssetMigrationConflict,
    pub orphan_folder: FString,
}
impl FMigrationOptions {}
#[repr(C, align(8))]
pub struct UAssetToolsSettings {
    __padding_end: [u8; 120],
}
impl UAssetToolsSettings {}
pub struct UAssetTools {}
pub struct IAssetTools {}
#[repr(C, align(8))]
pub struct UAssetToolsHelpers {
    __padding_end: [u8; 48],
}
impl UAssetToolsHelpers {}
#[repr(C, align(8))]
pub struct UAdvancedCopyCustomization {
    __padding_end: [u8; 424],
}
impl UAdvancedCopyCustomization {}
#[repr(C, align(8))]
pub struct UAssetDefinition_AssetTypeActionsProxy {
    __padding_end: [u8; 112],
}
impl UAssetDefinition_AssetTypeActionsProxy {}
#[repr(C, align(8))]
pub struct UAssetToolsImpl {
    __padding_end: [u8; 1064],
}
impl UAssetToolsImpl {}
#[repr(C, align(8))]
pub struct FBeginAdvancedCopyPackages_OnCopyComplete {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCreateAssetAsync_OnComplete {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCreateAssetAsync_OnCancelled {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCreateAssetWithDialogAsync_OnComplete {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCreateAssetWithDialogAsync_OnCancelled {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct EAssetMigrationConflict(pub u8);
impl EAssetMigrationConflict {
    pub const SKIP: EAssetMigrationConflict = EAssetMigrationConflict(0);
    pub const OVERWRITE: EAssetMigrationConflict = EAssetMigrationConflict(1);
    pub const CANCEL: EAssetMigrationConflict = EAssetMigrationConflict(2);
}
#[repr(transparent)]
pub struct EAssetRenameResult(pub u8);
impl EAssetRenameResult {
    pub const FAILURE: EAssetRenameResult = EAssetRenameResult(0);
    pub const SUCCESS: EAssetRenameResult = EAssetRenameResult(1);
    pub const PENDING: EAssetRenameResult = EAssetRenameResult(2);
}
#[repr(transparent)]
pub struct EAssetTypeActivationOpenedMethod(pub u8);
impl EAssetTypeActivationOpenedMethod {
    pub const EDIT: EAssetTypeActivationOpenedMethod = EAssetTypeActivationOpenedMethod(
        0,
    );
    pub const VIEW: EAssetTypeActivationOpenedMethod = EAssetTypeActivationOpenedMethod(
        1,
    );
}
