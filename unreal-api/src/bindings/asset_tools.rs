#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FAssetRenameData {
    pub asset: TWeakObjectPtr<UObject>,
    pub new_package_path: FString,
    pub new_name: FString,
    pub old_object_path: FSoftObjectPath,
    pub new_object_path: FSoftObjectPath,
    pub b_only_fix_soft_references: bool,
    pub b_also_rename_localized_variants: bool,
}
#[repr(C, align(8))]
pub struct FAdvancedCopyMap {
    pub class_to_copy: FSoftClassPath,
    pub advanced_copy_customization: FSoftClassPath,
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
