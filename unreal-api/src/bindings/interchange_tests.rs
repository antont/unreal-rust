#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FInterchangeTestScreenshotParameters {
    pub b_auto_focus: bool,
    pub camera_location: FVector,
    pub camera_rotation: FRotator,
    pub focus_actor_name: FString,
    pub focus_actor_class: TSubclassOf<AActor>,
    pub comparison_tolerance: EComparisonTolerance,
    pub view_mode: EViewModeIndex,
    pub wireframe_opacity: f32,
}
#[repr(C, align(8))]
pub struct FInterchangeTestFunctionResult {
    pub infos: TArray<FString>,
    pub warnings: TArray<FString>,
    pub errors: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FInterchangeTestFunction {
    pub asset_class: TSubclassOf<UObject>,
    pub optional_asset_name: FString,
    pub check_function: UPtr<UFunction>,
    pub parameters: TMap<FName, FString>,
}
#[repr(C, align(8))]
pub struct FInterchangeTestPlanPipelineSettings {
    pub custom_pipelines: TArray<UPtr<UInterchangePipelineBase>>,
    pub parent_test_step: UPtr<UInterchangeImportTestStepBase>,
}
pub struct UInterchangeImportTestSettings {
    pub import_tests_path: FString,
    pub import_files: TArray<FString>,
}
pub struct UImportTestFunctionsBase {}
pub struct UActorImportTestFunctions {}
pub struct UAnimationImportTestFunctions {}
pub struct UAssetImportTestFunctions {}
pub struct UInterchangeResultImportTestFunctions {}
pub struct ULevelSequenceImportTestFunctions {}
pub struct ULevelVariantSetsImportTestFunctions {}
pub struct ULightImportTestFunctions {}
pub struct UMaterialImportTestFunctions {}
pub struct UMaterialXTestFunctions {}
pub struct UPointLightImportTestFunctions {}
pub struct USkeletalMeshImportTestFunctions {}
pub struct USpotLightImportTestFunctions {}
pub struct UStaticMeshImportTestFunctions {}
pub struct UTextureImportTestFunctions {}
pub struct UInterchangeImportTestPlan {
    pub description: FString,
    pub steps_deprecated: TArray<UPtr<UInterchangeImportTestStepBase>>,
    pub world_path: FSoftObjectPath,
    pub import_step: UPtr<UInterchangeImportTestStepImport>,
    pub reimport_stack: TArray<UPtr<UInterchangeImportTestStepReimport>>,
}
pub struct UInterchangeImportTestStepBase {
    pub tests: TArray<FInterchangeTestFunction>,
}
pub struct UInterchangeImportTestStepImport {
    pub source_file: FFilePath,
    pub b_use_override_pipeline_stack: bool,
    pub pipeline_stack: TArray<UPtr<UInterchangePipelineBase>>,
    pub pipeline_settings: FInterchangeTestPlanPipelineSettings,
    pub b_empty_destination_folder_prior_to_import: bool,
    pub b_import_into_level: bool,
    pub b_take_screenshot: bool,
    pub screenshot_parameters: FInterchangeTestScreenshotParameters,
    pub last_source_file_extension: FString,
}
pub struct UInterchangeImportTestStepReimport {
    pub source_file_to_reimport: FFilePath,
    pub b_use_override_pipeline_stack: bool,
    pub pipeline_stack: TArray<UPtr<UInterchangePipelineBase>>,
    pub pipeline_settings: FInterchangeTestPlanPipelineSettings,
    pub b_import_into_level: bool,
    pub asset_type_to_reimport: TSubclassOf<UObject>,
    pub asset_name_to_reimport: FString,
    pub b_take_screenshot: bool,
    pub screenshot_parameters: FInterchangeTestScreenshotParameters,
    pub last_source_file_extension: FString,
}
pub struct UInterchangeTestsBlueprintFunctionLibrary {}
