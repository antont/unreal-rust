#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FValidateAssetsDetails {
    __padding_end: [u8; 96],
}
impl FValidateAssetsDetails {}
#[repr(C, align(4))]
pub struct FValidatorStatistics {
    pub assets_validated: i32,
    pub assets_added_for_validation: i32,
}
impl FValidatorStatistics {}
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
    pub validator_statistics: TMap<
        crate::bindings::core_u_object::FTopLevelAssetPath,
        FValidatorStatistics,
    >,
    pub assets_details: TMap<FString, FValidateAssetsDetails>,
}
impl FValidateAssetsResults {}
#[repr(C, align(8))]
pub struct FValidateAssetsSettings {
    pub b_skip_excluded_directories: bool,
    pub b_show_if_no_failures: bool,
    pub b_collect_per_asset_details: bool,
    pub validation_usecase: crate::bindings::core_u_object::EDataValidationUsecase,
    pub b_load_assets_for_validation: bool,
    pub b_unload_assets_loaded_for_validation: bool,
    pub b_load_external_objects_for_validation: bool,
    pub b_capture_asset_load_logs: bool,
    pub b_capture_logs_during_validation: bool,
    pub b_capture_warnings_during_validation_as_errors: bool,
    pub max_assets_to_validate: i32,
    pub b_validate_referencers_of_deleted_assets: bool,
    __padding_end: [u8; 47],
}
impl FValidateAssetsSettings {}
#[repr(C, align(8))]
pub struct UDataValidationSettings {
    __padding_end: [u8; 128],
}
impl UDataValidationSettings {}
#[repr(C, align(8))]
pub struct UDataValidationChangelist {
    __padding_end: [u8; 144],
}
impl UDataValidationChangelist {}
#[repr(C, align(8))]
pub struct UDataValidationCommandlet {
    __padding_end: [u8; 136],
}
impl UDataValidationCommandlet {}
#[repr(C, align(8))]
pub struct ADataValidationTestActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub b_pass_validation: bool,
    __padding_end: [u8; 15],
}
impl ADataValidationTestActor {}
#[repr(C, align(8))]
pub struct UEditorValidatorBase {
    __padding_end: [u8; 120],
}
impl UEditorValidatorBase {}
#[repr(C, align(8))]
pub struct UDirtyFilesChangelistValidator {
    __padding_end: [u8; 120],
}
impl UDirtyFilesChangelistValidator {}
#[repr(C, align(8))]
pub struct UEditorValidatorSubsystem {
    __padding_end: [u8; 424],
}
impl UEditorValidatorSubsystem {}
#[repr(C, align(8))]
pub struct UEditorValidator_Localization {
    __padding_end: [u8; 200],
}
impl UEditorValidator_Localization {}
#[repr(C, align(8))]
pub struct UEditorValidator_Material {
    __padding_end: [u8; 136],
}
impl UEditorValidator_Material {}
#[repr(C, align(8))]
pub struct UValidationMaterial {
    __padding_end: [u8; 3400],
}
impl UValidationMaterial {}
#[repr(C, align(8))]
pub struct UPackageFileValidator {
    __padding_end: [u8; 128],
}
impl UPackageFileValidator {}
#[repr(C, align(8))]
pub struct UWorldPartitionChangelistValidator {
    __padding_end: [u8; 408],
}
impl UWorldPartitionChangelistValidator {}
#[repr(transparent)]
pub struct EMaterialEditorValidationFeatureLevel(pub i32);
impl EMaterialEditorValidationFeatureLevel {
    pub const CURRENT_MAX_FEATURE_LEVEL: EMaterialEditorValidationFeatureLevel = EMaterialEditorValidationFeatureLevel(
        6,
    );
    pub const ES3_1: EMaterialEditorValidationFeatureLevel = EMaterialEditorValidationFeatureLevel(
        1,
    );
    pub const SM5: EMaterialEditorValidationFeatureLevel = EMaterialEditorValidationFeatureLevel(
        3,
    );
    pub const SM6: EMaterialEditorValidationFeatureLevel = EMaterialEditorValidationFeatureLevel(
        4,
    );
}
#[repr(transparent)]
pub struct EMaterialEditorValidationQualityLevel(pub u8);
impl EMaterialEditorValidationQualityLevel {
    pub const LOW: EMaterialEditorValidationQualityLevel = EMaterialEditorValidationQualityLevel(
        0,
    );
    pub const MEDIUM: EMaterialEditorValidationQualityLevel = EMaterialEditorValidationQualityLevel(
        2,
    );
    pub const HIGH: EMaterialEditorValidationQualityLevel = EMaterialEditorValidationQualityLevel(
        1,
    );
    pub const EPIC: EMaterialEditorValidationQualityLevel = EMaterialEditorValidationQualityLevel(
        3,
    );
}
