#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
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
    pub pipelines: TArray<UPtr<UInterchangePipelineBase>>,
}
#[repr(C, align(8))]
pub struct FImportAssetParameters {
    pub reimport_asset: UPtr<UObject>,
    pub reimport_source_index: i32,
    pub b_is_automated: bool,
    pub b_follow_redirectors: bool,
    pub override_pipelines: TArray<FSoftObjectPath>,
    pub import_level: UPtr<ULevel>,
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
    pub translator: TSoftObjectPtr<UClass>,
    pub pipelines: TArray<FSoftObjectPath>,
}
#[repr(C, align(8))]
pub struct FInterchangePipelineStack {
    pub pipelines: TArray<FSoftObjectPath>,
    pub per_translator_pipelines: TArray<FInterchangeTranslatorPipelines>,
}
#[repr(C, align(8))]
pub struct FInterchangePerTranslatorDialogOverride {
    pub translator: TSoftObjectPtr<UClass>,
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
    pub import_dialog_class: TSoftObjectPtr<UClass>,
    pub b_show_import_dialog: bool,
    pub b_show_reimport_dialog: bool,
}
#[repr(C, align(8))]
pub struct FInterchangeSceneImportSettings {
    pub per_translator_dialog_override: TArray<FInterchangePerTranslatorDialogOverride>,
}
#[repr(C, align(8))]
pub struct FInterchangeContentImportSettings {
    pub default_pipeline_stack_override: TMap<EInterchangeTranslatorAssetType, FName>,
    pub show_import_dialog_override: TMap<
        EInterchangeTranslatorAssetType,
        FInterchangeDialogOverride,
    >,
}
#[repr(C, align(8))]
pub struct FInterchangeGroup {
    pub display_name: FName,
    pub unique_id: FGuid,
    pub default_pipeline_stack: FName,
    pub default_pipeline_stack_override: TMap<EInterchangeTranslatorAssetType, FName>,
    pub b_show_import_dialog: bool,
    pub b_show_reimport_dialog: bool,
    pub show_import_dialog_override: TMap<
        EInterchangeTranslatorAssetType,
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
    pub scene_import_asset: FSoftObjectPath,
    pub node_unique_id: FString,
    pub node_container_deprecated: UPtr<UInterchangeBaseNodeContainer>,
    pub pipelines_deprecated: TArray<UPtr<UObject>>,
    pub transient_node_container: UPtr<UInterchangeBaseNodeContainer>,
    pub transient_pipelines: TArray<UPtr<UObject>>,
    pub transient_translator_settings: UPtr<UInterchangeTranslatorSettings>,
}
pub struct UInterchangeAssetImportDataConverterBase {}
pub struct UInterchangePipelineStackOverride {
    pub override_pipelines: TArray<FSoftObjectPath>,
}
pub struct UInterchangeManager {
    pub registered_translators_class: TSet<TSubclassOf<UObject>>,
    pub registered_factory_classes: TMap<TSubclassOf<UObject>, TSubclassOf<UObject>>,
    pub registered_writers: TMap<TSubclassOf<UObject>, UPtr<UInterchangeWriterBase>>,
    pub registered_converters: TMap<
        TSubclassOf<UObject>,
        UPtr<UInterchangeAssetImportDataConverterBase>,
    >,
}
pub struct UInterchangeMeshUtilities {}
pub struct UInterchangeProjectSettings {
    pub content_import_settings: FInterchangeContentImportSettings,
    pub scene_import_settings: FInterchangeSceneImportSettings,
    pub file_picker_class: TSoftObjectPtr<UClass>,
    pub b_static_mesh_use_smooth_edges_if_smoothing_information_is_missing: bool,
    pub generic_pipeline_class: TSoftObjectPtr<UClass>,
    pub converter_default_pipeline: FSoftObjectPath,
    pub interchange_groups: TArray<FInterchangeGroup>,
}
pub struct UInterchangeProjectSettingsScript {}
pub struct UInterchangeEditorSettings {
    pub b_show_import_dialog_at_reimport: bool,
    pub used_group_name: FName,
    pub used_group_uid: FGuid,
}
pub struct UInterchangePythonPipelineBase {}
pub struct UInterchangePythonPipelineAsset {
    pub python_class: TSoftObjectPtr<UClass>,
    pub generated_pipeline: UPtr<UInterchangePythonPipelineBase>,
    pub json_default_properties: FString,
}
pub struct UInterchangeSceneImportAsset {
    pub asset_import_data: UPtr<UInterchangeAssetImportData>,
    pub asset_user_data: TArray<UPtr<UAssetUserData>>,
}
