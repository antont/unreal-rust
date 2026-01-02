#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FInterchangeFilePickerParameters {
    __padding_end: [u8; 64],
}
impl FInterchangeFilePickerParameters {}
#[repr(C, align(8))]
pub struct FInterchangeStackInfo {
    pub stack_name: FName,
    pub pipelines: TArray<
        UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>,
    >,
}
impl FInterchangeStackInfo {}
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
    #[doc(hidden)]
    __padding_120: [u8; 24],
    pub on_assets_import_done: FImportAssetParameters_OnAssetsImportDone,
    #[doc(hidden)]
    __padding_176: [u8; 24],
    pub on_scene_object_done: FImportAssetParameters_OnSceneObjectDone,
    #[doc(hidden)]
    __padding_232: [u8; 24],
    pub on_scene_import_done: FImportAssetParameters_OnSceneImportDone,
    __padding_end: [u8; 32],
}
impl FImportAssetParameters {}
#[repr(C, align(8))]
pub struct FInterchangeTranslatorPipelines {
    pub translator: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub pipelines: TArray<crate::bindings::core_u_object::FSoftObjectPath>,
}
impl FInterchangeTranslatorPipelines {}
#[repr(C, align(8))]
pub struct FInterchangePipelineStack {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub per_translator_pipelines: TArray<FInterchangeTranslatorPipelines>,
}
impl FInterchangePipelineStack {}
#[repr(C, align(8))]
pub struct FInterchangePerTranslatorDialogOverride {
    pub translator: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub b_show_import_dialog: bool,
    pub b_show_reimport_dialog: bool,
    __padding_end: [u8; 6],
}
impl FInterchangePerTranslatorDialogOverride {}
#[repr(C, align(8))]
pub struct FInterchangeDialogOverride {
    pub b_show_import_dialog: bool,
    pub b_show_reimport_dialog: bool,
    pub per_translator_import_dialog_override: TArray<
        FInterchangePerTranslatorDialogOverride,
    >,
}
impl FInterchangeDialogOverride {}
#[repr(C, align(8))]
pub struct FInterchangeImportSettings {
    pub pipeline_stacks: TMap<FName, FInterchangePipelineStack>,
    pub default_pipeline_stack: FName,
    pub import_dialog_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub b_show_import_dialog: bool,
    pub b_show_reimport_dialog: bool,
    __padding_end: [u8; 6],
}
impl FInterchangeImportSettings {}
#[repr(C, align(8))]
pub struct FInterchangeSceneImportSettings {
    #[doc(hidden)]
    __padding_152: [u8; 152],
    pub per_translator_dialog_override: TArray<FInterchangePerTranslatorDialogOverride>,
}
impl FInterchangeSceneImportSettings {}
#[repr(C, align(8))]
pub struct FInterchangeContentImportSettings {
    #[doc(hidden)]
    __padding_152: [u8; 152],
    pub default_pipeline_stack_override: TMap<
        crate::bindings::interchange_core::EInterchangeTranslatorAssetType,
        FName,
    >,
    pub show_import_dialog_override: TMap<
        crate::bindings::interchange_core::EInterchangeTranslatorAssetType,
        FInterchangeDialogOverride,
    >,
}
impl FInterchangeContentImportSettings {}
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
impl FInterchangeGroup {}
#[repr(C, align(8))]
pub struct UInterchangeBlueprintPipelineBase {
    __padding_end: [u8; 1432],
}
impl UInterchangeBlueprintPipelineBase {}
#[repr(C, align(8))]
pub struct UInterchangeEditorUtilitiesBase {
    __padding_end: [u8; 48],
}
impl UInterchangeEditorUtilitiesBase {}
#[repr(C, align(8))]
pub struct UInterchangeFilePickerBase {
    __padding_end: [u8; 48],
}
impl UInterchangeFilePickerBase {}
#[repr(C, align(8))]
pub struct UInterchangePipelineConfigurationBase {
    __padding_end: [u8; 48],
}
impl UInterchangePipelineConfigurationBase {}
#[repr(C, align(8))]
pub struct UInterchangeAssetImportData {
    __padding_end: [u8; 312],
}
impl UInterchangeAssetImportData {}
#[repr(C, align(8))]
pub struct UInterchangeAssetImportDataConverterBase {
    __padding_end: [u8; 48],
}
impl UInterchangeAssetImportDataConverterBase {}
#[repr(C, align(8))]
pub struct UInterchangePipelineStackOverride {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub override_pipelines: TArray<crate::bindings::core_u_object::FSoftObjectPath>,
}
impl UInterchangePipelineStackOverride {}
#[repr(C, align(16))]
pub struct UInterchangeManager {
    __padding_end: [u8; 1072],
}
impl UInterchangeManager {}
#[repr(C, align(8))]
pub struct UInterchangeMeshUtilities {
    __padding_end: [u8; 48],
}
impl UInterchangeMeshUtilities {}
#[repr(C, align(8))]
pub struct UInterchangeProjectSettings {
    #[doc(hidden)]
    __padding_104: [u8; 104],
    pub content_import_settings: FInterchangeContentImportSettings,
    pub scene_import_settings: FInterchangeSceneImportSettings,
    pub file_picker_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub b_static_mesh_use_smooth_edges_if_smoothing_information_is_missing: bool,
    pub generic_pipeline_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub converter_default_pipeline: crate::bindings::core_u_object::FSoftObjectPath,
    pub interchange_groups: TArray<FInterchangeGroup>,
}
impl UInterchangeProjectSettings {}
#[repr(C, align(8))]
pub struct UInterchangeProjectSettingsScript {
    __padding_end: [u8; 48],
}
impl UInterchangeProjectSettingsScript {}
#[repr(C, align(8))]
pub struct UInterchangeEditorSettings {
    #[doc(hidden)]
    __padding_104: [u8; 104],
    pub b_show_import_dialog_at_reimport: bool,
    __padding_end: [u8; 31],
}
impl UInterchangeEditorSettings {}
#[repr(C, align(8))]
pub struct UInterchangePythonPipelineBase {
    __padding_end: [u8; 344],
}
impl UInterchangePythonPipelineBase {}
#[repr(C, align(8))]
pub struct UInterchangePythonPipelineAsset {
    __padding_end: [u8; 120],
}
impl UInterchangePythonPipelineAsset {}
#[repr(C, align(8))]
pub struct UInterchangeSceneImportAsset {
    __padding_end: [u8; 216],
}
impl UInterchangeSceneImportAsset {}
#[repr(C, align(8))]
pub struct FImportAssetParameters_OnAssetDone {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FImportAssetParameters_OnAssetsImportDone {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FImportAssetParameters_OnSceneObjectDone {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FImportAssetParameters_OnSceneImportDone {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FImportAsset_OnAssetDone {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FImportScene_OnAssetDone {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FReimportAsset_OnAssetDone {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FScriptedImportAssetAsync_OnAssetDone {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FScriptedImportSceneAsync_OnAssetDone {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FScriptedReimportAssetAsync_OnAssetDone {
    _opague: [u8; 32],
}
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
