#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FPluginReferenceMetadata {
    pub name: FString,
    pub b_optional: bool,
    pub b_enabled: bool,
    pub b_activate: bool,
}
#[repr(C, align(8))]
pub struct FPluginDisallowedMetadata {
    pub name: FString,
    pub comment: FString,
}
#[repr(C, align(8))]
pub struct FPluginPaths_External {
    pub additional_plugin_directories: TArray<
        crate::bindings::core_u_object::FDirectoryPath,
    >,
    pub user_plugin_directories: TArray<crate::bindings::core_u_object::FDirectoryPath>,
    pub command_line_directories: TArray<crate::bindings::core_u_object::FDirectoryPath>,
    pub environment_directories: TArray<crate::bindings::core_u_object::FDirectoryPath>,
}
pub struct UNewPluginDescriptorData {
    pub created_by: FString,
    pub created_by_url: FString,
    pub description: FString,
    pub b_is_beta_version: bool,
}
pub struct UPluginMetadataObject {
    pub version: i32,
    pub version_name: FString,
    pub friendly_name: FString,
    pub description: FString,
    pub category: FString,
    pub created_by: FString,
    pub created_by_url: FString,
    pub docs_url: FString,
    pub marketplace_url: FString,
    pub support_url: FString,
    pub editor_custom_virtual_path: FString,
    pub b_can_contain_content: bool,
    pub b_is_beta_version: bool,
    pub b_is_enabled_by_default: bool,
    pub b_explicitly_loaded: bool,
    pub b_is_sealed: bool,
    pub b_no_code: bool,
    pub plugins: TArray<FPluginReferenceMetadata>,
    pub disallowed_plugins: TArray<FPluginDisallowedMetadata>,
}
