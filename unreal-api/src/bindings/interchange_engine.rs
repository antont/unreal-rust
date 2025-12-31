#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FInterchangeFilePickerParameters {
    pub b_allow_multiple_files: bool,
    pub title: FText,
    pub default_path: FString,
    pub b_show_all_factories_extension: bool,
    pub extra_formats: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FInterchangeStackInfo {
    pub stack_name: FName,
    pub pipelines: TArray<
        UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>,
    >,
}
#[repr(C, align(8))]
pub struct FImportAssetParameters {
    pub reimport_asset: UPtr<crate::bindings::core_u_object::UObject>,
    pub reimport_source_index: i32,
    pub b_is_automated: bool,
    pub b_follow_redirectors: bool,
    pub override_pipelines: TArray<crate::bindings::core_u_object::FSoftObjectPath>,
    pub import_level: UPtr<crate::bindings::engine::ULevel>,
    pub destination_name: FString,
    pub b_replace_existing: bool,
    pub b_force_show_dialog: bool,
    pub on_asset_done: FImportAssetParameters_OnAssetDone,
    pub on_assets_import_done: FImportAssetParameters_OnAssetsImportDone,
    pub on_scene_object_done: FImportAssetParameters_OnSceneObjectDone,
    pub on_scene_import_done: FImportAssetParameters_OnSceneImportDone,
}
#[repr(C, align(8))]
pub struct FInterchangeTranslatorPipelines {
    pub translator: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub pipelines: TArray<crate::bindings::core_u_object::FSoftObjectPath>,
}
#[repr(C, align(8))]
pub struct FInterchangePipelineStack {
    pub pipelines: TArray<crate::bindings::core_u_object::FSoftObjectPath>,
    pub per_translator_pipelines: TArray<FInterchangeTranslatorPipelines>,
}
#[repr(C, align(8))]
pub struct FInterchangePerTranslatorDialogOverride {
    pub translator: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub b_show_import_dialog: bool,
    pub b_show_reimport_dialog: bool,
}
#[repr(C, align(8))]
pub struct FInterchangeDialogOverride {
    pub b_show_import_dialog: bool,
    pub b_show_reimport_dialog: bool,
    pub per_translator_import_dialog_override: TArray<
        FInterchangePerTranslatorDialogOverride,
    >,
}
#[repr(C, align(8))]
pub struct FInterchangeImportSettings {
    pub pipeline_stacks: TMap<FName, FInterchangePipelineStack>,
    pub default_pipeline_stack: FName,
    pub import_dialog_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub b_show_import_dialog: bool,
    pub b_show_reimport_dialog: bool,
}
#[repr(C, align(8))]
pub struct FInterchangeSceneImportSettings {
    pub per_translator_dialog_override: TArray<FInterchangePerTranslatorDialogOverride>,
}
#[repr(C, align(8))]
pub struct FInterchangeContentImportSettings {
    pub default_pipeline_stack_override: TMap<
        crate::bindings::interchange_core::EInterchangeTranslatorAssetType,
        FName,
    >,
    pub show_import_dialog_override: TMap<
        crate::bindings::interchange_core::EInterchangeTranslatorAssetType,
        FInterchangeDialogOverride,
    >,
}
#[repr(C, align(8))]
pub struct FInterchangeGroup {
    pub display_name: FName,
    pub unique_id: crate::bindings::core_u_object::FGuid,
    pub default_pipeline_stack: FName,
    pub default_pipeline_stack_override: TMap<
        crate::bindings::interchange_core::EInterchangeTranslatorAssetType,
        FName,
    >,
    pub b_show_import_dialog: bool,
    pub b_show_reimport_dialog: bool,
    pub show_import_dialog_override: TMap<
        crate::bindings::interchange_core::EInterchangeTranslatorAssetType,
        FInterchangeDialogOverride,
    >,
}
#[repr(C, align(8))]
pub struct FPropertyData {}
pub struct UInterchangeBlueprintPipelineBase {}
pub struct UInterchangeEditorUtilitiesBase {}
pub struct UInterchangeFilePickerBase {}
pub struct UInterchangePipelineConfigurationBase {}
pub struct UInterchangeAssetImportData {
    pub scene_import_asset: crate::bindings::core_u_object::FSoftObjectPath,
    pub node_unique_id: FString,
    pub node_container_deprecated: UPtr<
        crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
    >,
    pub pipelines_deprecated: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub transient_node_container: UPtr<
        crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
    >,
    pub transient_pipelines: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub transient_translator_settings: UPtr<
        crate::bindings::interchange_core::UInterchangeTranslatorSettings,
    >,
}
pub struct UInterchangeAssetImportDataConverterBase {}
pub struct UInterchangePipelineStackOverride {
    pub override_pipelines: TArray<crate::bindings::core_u_object::FSoftObjectPath>,
}
pub struct UInterchangeManager {
    pub registered_translators_class: TSet<
        TSubclassOf<crate::bindings::core_u_object::UObject>,
    >,
    pub registered_factory_classes: TMap<
        TSubclassOf<crate::bindings::core_u_object::UObject>,
        TSubclassOf<crate::bindings::core_u_object::UObject>,
    >,
    pub registered_writers: TMap<
        TSubclassOf<crate::bindings::core_u_object::UObject>,
        UPtr<crate::bindings::interchange_core::UInterchangeWriterBase>,
    >,
    pub registered_converters: TMap<
        TSubclassOf<crate::bindings::core_u_object::UObject>,
        UPtr<UInterchangeAssetImportDataConverterBase>,
    >,
}
pub struct UInterchangeMeshUtilities {}
pub struct UInterchangeProjectSettings {
    pub content_import_settings: FInterchangeContentImportSettings,
    pub scene_import_settings: FInterchangeSceneImportSettings,
    pub file_picker_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub b_static_mesh_use_smooth_edges_if_smoothing_information_is_missing: bool,
    pub generic_pipeline_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub converter_default_pipeline: crate::bindings::core_u_object::FSoftObjectPath,
    pub interchange_groups: TArray<FInterchangeGroup>,
}
pub struct UInterchangeProjectSettingsScript {}
pub struct UInterchangeEditorSettings {
    pub b_show_import_dialog_at_reimport: bool,
    pub used_group_name: FName,
    pub used_group_uid: crate::bindings::core_u_object::FGuid,
}
pub struct UInterchangePythonPipelineBase {}
pub struct UInterchangePythonPipelineAsset {
    pub python_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub generated_pipeline: UPtr<UInterchangePythonPipelineBase>,
    pub json_default_properties: FString,
}
pub struct UInterchangeSceneImportAsset {
    pub asset_import_data: UPtr<UInterchangeAssetImportData>,
    pub asset_user_data: TArray<UPtr<crate::bindings::engine::UAssetUserData>>,
}
pub struct FImportAssetParameters_OnAssetDone;
pub struct FImportAssetParameters_OnAssetsImportDone;
pub struct FImportAssetParameters_OnSceneObjectDone;
pub struct FImportAssetParameters_OnSceneImportDone;
pub struct FImportAsset_OnAssetDone;
pub struct FImportScene_OnAssetDone;
pub struct FReimportAsset_OnAssetDone;
pub struct FScriptedImportAssetAsync_OnAssetDone;
pub struct FScriptedImportSceneAsync_OnAssetDone;
pub struct FScriptedReimportAssetAsync_OnAssetDone;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInterchangePipelineConfigurationDialogResult(pub u8);
impl EInterchangePipelineConfigurationDialogResult {
    pub const CANCEL: EInterchangePipelineConfigurationDialogResult = EInterchangePipelineConfigurationDialogResult(
        0,
    );
    pub const IMPORT: EInterchangePipelineConfigurationDialogResult = EInterchangePipelineConfigurationDialogResult(
        1,
    );
    pub const IMPORT_ALL: EInterchangePipelineConfigurationDialogResult = EInterchangePipelineConfigurationDialogResult(
        2,
    );
    pub const SAVE_CONFIG: EInterchangePipelineConfigurationDialogResult = EInterchangePipelineConfigurationDialogResult(
        3,
    );
}
