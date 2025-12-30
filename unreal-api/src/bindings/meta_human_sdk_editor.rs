#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FMetaHumanAssetReportItem {
    pub message: FText,
    pub project_item: UPtr<UObject>,
    pub source_item: FString,
}
#[repr(C, align(1))]
pub struct FMetaHumanImportOptions {
    pub b_verbose: bool,
    pub b_force_update: bool,
}
#[repr(C, align(8))]
pub struct FMetaHumanAggregateDetails {
    pub b_resizes_with_blendable_bodies: bool,
    pub b_has_clothing_mask: bool,
    pub included_lods: i32,
    pub lod0_vert_count: i32,
    pub num_unique_clothing_items: i32,
    pub b_contains_grooms: bool,
    pub b_contains_clothing: bool,
    pub b_is_editable_character: bool,
    pub platforms_included: TArray<EMetaHumanQualityLevel>,
    pub num_unique_characters: i32,
    pub num_virtual_textures: i32,
    pub num_substrate_materials: i32,
    pub num_unique_grooms: i32,
    pub b_physics: bool,
    pub strands_count: i32,
    pub strands_point_count: i32,
    pub b_has_lods: bool,
    pub card_mesh_count: i32,
    pub card_mesh_vertices: i32,
    pub card_mesh_texture_resolution: FIntVector2,
    pub volume_mesh_count: i32,
    pub volume_mesh_vertices: i32,
    pub volume_mesh_texture_resolution: FIntVector2,
    pub num_materials: i32,
    pub engine_version: FString,
}
#[repr(C, align(8))]
pub struct FMetaHumanAssetDescription {
    pub name: FName,
    pub asset_data: FAssetData,
    pub dependent_packages: TArray<FName>,
    pub asset_type: EMetaHumanAssetType,
    pub details: FMetaHumanAggregateDetails,
    pub total_size: i32,
    pub verification_report: UPtr<UMetaHumanAssetReport>,
}
#[repr(C, align(8))]
pub struct FMetaHumanMultiArchiveDescription {
    pub contained_archives: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FMetaHumanArchiveEntry {
    pub path: FString,
    pub version: FString,
}
#[repr(C, align(8))]
pub struct FMetaHumanArchiveContents {
    pub files: TArray<FMetaHumanArchiveEntry>,
}
#[repr(C, align(1))]
pub struct FMetaHumanVerificationOptions {
    pub b_verbose: bool,
    pub b_treat_warnings_as_errors: bool,
    pub b_verify_packaging_rules: bool,
}
#[repr(C, align(8))]
pub struct FEosConstantsPlatform {
    pub product_id: FString,
    pub sandbox_id: FString,
    pub deployment_id: FString,
    pub client_credentials_id: FString,
    pub client_credentials_secret: FString,
}
pub struct UMetaHumanPackageFactory {}
pub struct UMetaHumanAssetReport {
    pub subject: FString,
    pub infos: TArray<FMetaHumanAssetReportItem>,
    pub warnings: TArray<FMetaHumanAssetReportItem>,
    pub errors: TArray<FMetaHumanAssetReportItem>,
}
pub struct UMetaHumanAssetManager {}
pub struct UMetaHumanVerificationRuleBase {}
pub struct UMetaHumanVerificationRuleCollection {
    pub rules: TArray<UPtr<UMetaHumanVerificationRuleBase>>,
}
pub struct UVerifyMetaHumanCharacter {}
pub struct UVerifyMetaHumanGroom {}
pub struct UVerifyMetaHumanOutfitClothing {}
pub struct UVerifyMetaHumanPackageSource {}
pub struct UVerifyMetaHumanSkeletalClothing {}
pub struct UVerifyObjectValid {}
pub struct UMetaHumanCharacterTypesVerificationExtensionBase {}
pub struct UMetaHumanCloudServicesSettings {
    pub texture_synthesis_service_url: FString,
    pub autorig_service_url: FString,
    pub timeout: f32,
    pub long_poll_timeout: f32,
    pub auth_timeout: f32,
    pub auth_poll_interval: f32,
    pub retry_count: i32,
    pub service_environment: EMetaHumanCloudServiceEnvironment,
    pub prod_eos_constants: FEosConstantsPlatform,
    pub game_dev_eos_constants: FEosConstantsPlatform,
}
pub struct UMetaHumanSDKSettings {
    pub version_service_base_url: FString,
    pub cinematic_import_path: FDirectoryPath,
    pub optimized_import_path: FDirectoryPath,
    pub character_asset_packaging_path: FDirectoryPath,
    pub character_assembly_packaging_path: FDirectoryPath,
    pub skeletal_clothing_packaging_path: FDirectoryPath,
    pub outfit_packaging_path: FDirectoryPath,
    pub groom_packaging_path: FDirectoryPath,
}
