#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FValidateAssetsDetails {}
#[repr(C, align(4))]
pub struct FValidatorStatistics {
    pub assets_validated: i32,
    pub assets_added_for_validation: i32,
}
#[repr(C, align(8))]
pub struct FValidateAssetsResults {
    pub num_requested: i32,
    pub num_external_objects: i32,
    pub num_checked: i32,
    pub num_valid: i32,
    pub num_invalid: i32,
    pub num_skipped: i32,
    pub num_warnings: i32,
    pub num_unable_to_validate: i32,
    pub b_asset_limit_reached: bool,
    pub validator_statistics: TMap<FTopLevelAssetPath, FValidatorStatistics>,
    pub assets_details: TMap<FString, FValidateAssetsDetails>,
}
#[repr(C, align(8))]
pub struct FValidateAssetsSettings {
    pub b_skip_excluded_directories: bool,
    pub b_show_if_no_failures: bool,
    pub b_collect_per_asset_details: bool,
    pub validation_usecase: EDataValidationUsecase,
    pub b_load_assets_for_validation: bool,
    pub b_unload_assets_loaded_for_validation: bool,
    pub b_load_external_objects_for_validation: bool,
    pub b_capture_asset_load_logs: bool,
    pub b_capture_logs_during_validation: bool,
    pub b_capture_warnings_during_validation_as_errors: bool,
    pub max_assets_to_validate: i32,
    pub b_validate_referencers_of_deleted_assets: bool,
}
#[repr(C, align(4))]
pub struct FMaterialEditorValidationShaderPlatform {
    pub name: FName,
}
#[repr(C, align(4))]
pub struct FMaterialEditorValidationPlatform {
    pub shader_platform: FMaterialEditorValidationShaderPlatform,
    pub feature_level: EMaterialEditorValidationFeatureLevel,
    pub material_quality_level: EMaterialEditorValidationQualityLevel,
}
pub struct UDataValidationSettings {
    pub flags_104: u8,
    pub b_enable_material_validation: bool,
    pub b_material_validation_allow_compiling_shaders: bool,
    pub b_material_validation_show_warnings_when_not_compiling_shaders: bool,
    pub material_validation_platforms: TArray<FMaterialEditorValidationPlatform>,
}
pub struct UDataValidationChangelist {}
pub struct UDataValidationCommandlet {}
pub struct ADataValidationTestActor {
    pub b_pass_validation: bool,
    pub sprite_component: UPtr<UBillboardComponent>,
}
pub struct UEditorValidatorBase {
    pub b_is_enabled: bool,
    pub b_is_config_disabled: bool,
    pub b_only_print_custom_message: bool,
    pub current_object_being_validated: UPtr<UObject>,
}
pub struct UDirtyFilesChangelistValidator {}
pub struct UEditorValidatorSubsystem {
    pub excluded_directories: TArray<FDirectoryPath>,
    pub b_validate_on_save: bool,
    pub validators: TMap<FTopLevelAssetPath, UPtr<UEditorValidatorBase>>,
    pub b_allow_blueprint_validators: bool,
}
pub struct UEditorValidator_Localization {}
pub struct UEditorValidator_Material {}
pub struct UValidationMaterial {}
pub struct UPackageFileValidator {
    pub b_validate_payload_hashes: bool,
}
pub struct UWorldPartitionChangelistValidator {}
