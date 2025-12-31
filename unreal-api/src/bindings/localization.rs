#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FGatherTextSearchDirectory {
    pub path_root: ELocalizationGatherPathRoot,
    pub path: FString,
}
#[repr(C, align(8))]
pub struct FGatherTextIncludePath {
    pub path_root: ELocalizationGatherPathRoot,
    pub pattern: FString,
}
#[repr(C, align(8))]
pub struct FGatherTextExcludePath {
    pub path_root: ELocalizationGatherPathRoot,
    pub pattern: FString,
}
#[repr(C, align(8))]
pub struct FGatherTextFileExtension {
    pub pattern: FString,
}
#[repr(C, align(8))]
pub struct FGatherTextFromTextFilesConfiguration {
    pub is_enabled: bool,
    pub search_directories: TArray<FGatherTextSearchDirectory>,
    pub exclude_path_wildcards: TArray<FGatherTextExcludePath>,
    pub file_extensions: TArray<FGatherTextFileExtension>,
    pub should_gather_from_editor_only_data: bool,
}
#[repr(C, align(8))]
pub struct FGatherTextFromPackagesConfiguration {
    pub is_enabled: bool,
    pub include_path_wildcards: TArray<FGatherTextIncludePath>,
    pub exclude_path_wildcards: TArray<FGatherTextExcludePath>,
    pub file_extensions: TArray<FGatherTextFileExtension>,
    pub collections: TArray<FName>,
    pub world_collections: TArray<FName>,
    pub exclude_classes: TArray<crate::bindings::core_u_object::FSoftClassPath>,
    pub should_exclude_derived_classes: bool,
    pub should_gather_from_editor_only_data: bool,
    pub skip_gather_cache: bool,
}
#[repr(C, align(8))]
pub struct FMetaDataTextKeyPattern {
    pub pattern: FString,
}
#[repr(C, align(8))]
pub struct FMetaDataKeyName {
    pub name: FString,
}
#[repr(C, align(8))]
pub struct FMetaDataKeyGatherSpecification {
    pub meta_data_key: FMetaDataKeyName,
    pub text_namespace: FString,
    pub text_key_pattern: FMetaDataTextKeyPattern,
}
#[repr(C, align(8))]
pub struct FGatherTextFromMetaDataConfiguration {
    pub is_enabled: bool,
    pub include_path_wildcards: TArray<FGatherTextIncludePath>,
    pub exclude_path_wildcards: TArray<FGatherTextExcludePath>,
    pub key_specifications: TArray<FMetaDataKeyGatherSpecification>,
    pub field_types_to_include: TArray<FString>,
    pub field_types_to_exclude: TArray<FString>,
    pub field_owner_types_to_include: TArray<FString>,
    pub field_owner_types_to_exclude: TArray<FString>,
    pub should_gather_from_editor_only_data: bool,
}
#[repr(C, align(1))]
pub struct FLocalizationExportingSettings {
    pub collapse_mode: ELocalizedTextCollapseMode,
    pub po_format: EPortableObjectFormat,
    pub should_persist_comments_on_export: bool,
    pub should_add_source_locations_as_comments: bool,
}
#[repr(C, align(1))]
pub struct FLocalizationCompilationSettings {
    pub skip_source_check: bool,
    pub validate_format_patterns: bool,
    pub validate_safe_whitespace: bool,
    pub validate_rich_text_tags: bool,
}
#[repr(C, align(8))]
pub struct FLocalizationImportDialogueSettings {
    pub raw_audio_path: crate::bindings::core_u_object::FDirectoryPath,
    pub imported_dialogue_folder: FString,
    pub b_import_native_as_source: bool,
}
#[repr(C, align(8))]
pub struct FCultureStatistics {
    pub culture_name: FString,
    pub word_count: u32,
}
#[repr(C, align(8))]
pub struct FLocalizationTargetSettings {
    pub name: FString,
    pub guid: crate::bindings::core_u_object::FGuid,
    pub conflict_status: ELocalizationTargetConflictStatus,
    pub target_dependencies: TArray<crate::bindings::core_u_object::FGuid>,
    pub additional_manifest_dependencies: TArray<
        crate::bindings::core_u_object::FFilePath,
    >,
    pub required_module_names: TArray<FString>,
    pub gather_from_text_files: FGatherTextFromTextFilesConfiguration,
    pub gather_from_packages: FGatherTextFromPackagesConfiguration,
    pub gather_from_meta_data: FGatherTextFromMetaDataConfiguration,
    pub export_settings: FLocalizationExportingSettings,
    pub compile_settings: FLocalizationCompilationSettings,
    pub import_dialogue_settings: FLocalizationImportDialogueSettings,
    pub native_culture_index: i32,
    pub supported_cultures_statistics: TArray<FCultureStatistics>,
}
#[repr(C, align(8))]
pub struct FUserGeneratedContentLocalizationDescriptor {
    pub native_culture: FString,
    pub cultures_to_generate: TArray<FString>,
    pub po_format: EPortableObjectFormat,
}
pub struct ULocalizationSettings {
    pub engine_target_set: UPtr<ULocalizationTargetSet>,
    pub engine_targets_settings: TArray<FLocalizationTargetSettings>,
    pub game_target_set: UPtr<ULocalizationTargetSet>,
    pub game_targets_settings: TArray<FLocalizationTargetSettings>,
}
pub struct ULocalizationTargetSet {
    pub target_objects: TArray<UPtr<ULocalizationTarget>>,
}
pub struct ULocalizationTarget {
    pub settings: FLocalizationTargetSettings,
}
pub struct UUserGeneratedContentLocalizationSettings {
    pub cultures_to_disable: TArray<FString>,
    pub b_compile_dlc_localization_during_cook: bool,
    pub b_validate_dlc_localization_during_cook: bool,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELocalizationGatherPathRoot(pub u8);
impl ELocalizationGatherPathRoot {
    pub const AUTO: ELocalizationGatherPathRoot = ELocalizationGatherPathRoot(0);
    pub const ENGINE: ELocalizationGatherPathRoot = ELocalizationGatherPathRoot(1);
    pub const PROJECT: ELocalizationGatherPathRoot = ELocalizationGatherPathRoot(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELocalizedTextCollapseMode(pub u8);
impl ELocalizedTextCollapseMode {
    pub const IDENTICAL_TEXT_ID_AND_SOURCE: ELocalizedTextCollapseMode = ELocalizedTextCollapseMode(
        0,
    );
    pub const IDENTICAL_PACKAGE_ID_TEXT_ID_AND_SOURCE: ELocalizedTextCollapseMode = ELocalizedTextCollapseMode(
        1,
    );
    pub const IDENTICAL_NAMESPACE_AND_SOURCE: ELocalizedTextCollapseMode = ELocalizedTextCollapseMode(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPortableObjectFormat(pub u8);
impl EPortableObjectFormat {
    pub const UNREAL: EPortableObjectFormat = EPortableObjectFormat(0);
    pub const CROWDIN: EPortableObjectFormat = EPortableObjectFormat(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELocalizationTargetConflictStatus(pub u8);
impl ELocalizationTargetConflictStatus {
    pub const UNKNOWN: ELocalizationTargetConflictStatus = ELocalizationTargetConflictStatus(
        0,
    );
    pub const CONFLICTS_PRESENT: ELocalizationTargetConflictStatus = ELocalizationTargetConflictStatus(
        1,
    );
    pub const CLEAR: ELocalizationTargetConflictStatus = ELocalizationTargetConflictStatus(
        2,
    );
}
