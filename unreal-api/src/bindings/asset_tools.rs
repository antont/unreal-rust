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
    pub old_object_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub new_object_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub b_only_fix_soft_references: bool,
    pub b_also_rename_localized_variants: bool,
}
#[repr(C, align(8))]
pub struct FAdvancedCopyMap {
    pub class_to_copy: crate::bindings::core_u_object::FSoftClassPath,
    pub advanced_copy_customization: crate::bindings::core_u_object::FSoftClassPath,
}
#[repr(C, align(8))]
pub struct FAdvancedCopyParams {}
#[repr(C, align(8))]
pub struct FMigrationOptions {
    pub b_prompt: bool,
    pub b_ignore_dependencies: bool,
    pub asset_conflict: EAssetMigrationConflict,
    pub orphan_folder: FString,
}
pub struct UAssetToolsSettings {
    pub advanced_copy_customizations: TArray<FAdvancedCopyMap>,
}
pub struct UAssetTools {}
pub struct IAssetTools {}
pub struct UAssetToolsHelpers {}
pub struct UAdvancedCopyCustomization {}
pub struct UAssetDefinition_AssetTypeActionsProxy {}
pub struct UAssetToolsImpl {}
pub struct FBeginAdvancedCopyPackages_OnCopyComplete;
pub struct FCreateAssetAsync_OnComplete;
pub struct FCreateAssetAsync_OnCancelled;
pub struct FCreateAssetWithDialogAsync_OnComplete;
pub struct FCreateAssetWithDialogAsync_OnCancelled;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAssetMigrationConflict(pub u8);
impl EAssetMigrationConflict {
    pub const SKIP: EAssetMigrationConflict = EAssetMigrationConflict(0);
    pub const OVERWRITE: EAssetMigrationConflict = EAssetMigrationConflict(1);
    pub const CANCEL: EAssetMigrationConflict = EAssetMigrationConflict(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAssetRenameResult(pub u8);
impl EAssetRenameResult {
    pub const FAILURE: EAssetRenameResult = EAssetRenameResult(0);
    pub const SUCCESS: EAssetRenameResult = EAssetRenameResult(1);
    pub const PENDING: EAssetRenameResult = EAssetRenameResult(2);
}
#[allow(non_camel_case_types)]
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
