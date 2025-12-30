#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FMetaSoundOutput {}
#[repr(C, align(8))]
pub struct FMetaSoundAssetDirectory {
    pub directory: FDirectoryPath,
}
#[repr(C, align(8))]
pub struct FMetaSoundAsyncAssetDependencies {
    pub meta_sound: UPtr<UObject>,
}
#[repr(C, align(4))]
pub struct FMetaSoundBuilderNodeInputHandle {}
#[repr(C, align(4))]
pub struct FMetaSoundBuilderNodeOutputHandle {}
#[repr(C, align(4))]
pub struct FMetaSoundNodeHandle {
    pub node_id: FGuid,
}
#[repr(C, align(8))]
pub struct FMetaSoundBuilderOptions {
    pub name: FName,
    pub b_force_unique_class_name: bool,
    pub b_add_to_registry: bool,
    pub existing_meta_sound: TScriptInterface<IMetaSoundDocumentInterface>,
}
#[repr(C, align(8))]
pub struct FDefaultMetaSoundAssetAutoUpdateSettings {
    pub meta_sound: FSoftObjectPath,
}
#[repr(C, align(8))]
pub struct FMetaSoundPageSettings {
    pub unique_id: FGuid,
    pub name: FName,
    pub can_target: FPerPlatformBool,
    pub b_is_default_page: bool,
    pub exclude_from_cook: FPerPlatformBool,
}
#[repr(C, align(8))]
pub struct FMetaSoundQualitySettings {
    pub unique_id: FGuid,
    pub name: FName,
    pub sample_rate: FPerPlatformInt,
    pub block_rate: FPerPlatformFloat,
}
pub struct UMetasoundEditorGraphBase {}
pub struct UMetaSoundPatch {
    pub root_meta_sound_document: FMetasoundFrontendDocument,
    pub referenced_asset_class_keys: TSet<FString>,
    pub referenced_asset_class_objects: TSet<UPtr<UObject>>,
    pub reference_asset_class_cache: TSet<FSoftObjectPath>,
    pub graph: UPtr<UMetasoundEditorGraphBase>,
    pub editor_graph: UPtr<UMetasoundEditorGraphBase>,
    pub asset_class_id: FGuid,
    pub registry_input_types: FString,
    pub registry_output_types: FString,
    pub registry_version_major: i32,
    pub registry_version_minor: i32,
    pub b_is_preset: bool,
}
pub struct UMetaSoundAssetSubsystem {}
pub struct UMetaSoundBuilderBase {
    pub builder: FMetaSoundFrontendDocumentBuilder,
    pub class_name: FMetasoundFrontendClassName,
    pub b_is_attached: bool,
}
pub struct UMetaSoundPatchBuilder {}
pub struct UMetaSoundSourceBuilder {}
pub struct UMetaSoundBuilderSubsystem {
    pub named_builders: TMap<FName, UPtr<UMetaSoundBuilderBase>>,
}
pub struct UMetasoundFrontendLiteralBlueprintAccess {}
pub struct UMetasoundGeneratorHandle {}
pub struct UMetaSoundCacheSubsystem {}
pub struct UMetasoundOutputBlueprintAccess {}
pub struct UMetaSoundOutputSubsystem {}
pub struct UMetaSoundQualityHelper {}
pub struct UMetaSoundSettings {
    pub b_auto_update_enabled: bool,
    pub auto_update_denylist: TArray<FMetasoundFrontendClassName>,
    pub auto_update_asset_denylist: TArray<FDefaultMetaSoundAssetAutoUpdateSettings>,
    pub b_auto_update_log_warning_on_dropped_connection: bool,
    pub directories_to_register: TArray<FDirectoryPath>,
    pub deny_list_cache_change_id: i32,
    pub target_page_name: FName,
    pub default_page_settings: FMetaSoundPageSettings,
    pub page_settings: TArray<FMetaSoundPageSettings>,
    pub quality_settings: TArray<FMetaSoundQualitySettings>,
}
pub struct UMetaSoundSource {
    pub root_metasound_document: FMetasoundFrontendDocument,
    pub referenced_asset_class_keys: TSet<FString>,
    pub referenced_asset_class_objects: TSet<UPtr<UObject>>,
    pub reference_asset_class_cache: TSet<FSoftObjectPath>,
    pub graph: UPtr<UMetasoundEditorGraphBase>,
    pub editor_graph: UPtr<UMetasoundEditorGraphBase>,
    pub output_format: EMetaSoundOutputAudioFormat,
    pub quality_setting: FName,
    pub quality_setting_guid: FGuid,
    pub block_rate_override: FPerPlatformFloat,
    pub sample_rate_override: FPerPlatformInt,
    pub asset_class_id: FGuid,
    pub registry_input_types: FString,
    pub registry_output_types: FString,
    pub registry_version_major: i32,
    pub registry_version_minor: i32,
    pub b_is_preset: bool,
}
