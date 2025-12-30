#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FFabAssetMetadata {
    pub asset_id: FString,
    pub asset_name: FString,
    pub asset_type: FString,
    pub listing_type: FString,
    pub asset_namespace: FString,
    pub distribution_point_base_urls: TArray<FString>,
    pub is_quixel: bool,
}
#[repr(C, align(8))]
pub struct FEosConstantsGameDev {
    pub product_id: FString,
    pub sandbox_id: FString,
    pub deployment_id: FString,
    pub client_credentials_id: FString,
    pub client_credentials_secret: FString,
    pub game_name: FString,
    pub encryption_key: FString,
    pub product_version: FString,
}
#[repr(C, align(8))]
pub struct FEosConstantsProd {
    pub product_id: FString,
    pub sandbox_id: FString,
    pub deployment_id: FString,
    pub client_credentials_id: FString,
    pub client_credentials_secret: FString,
    pub game_name: FString,
    pub encryption_key: FString,
    pub product_version: FString,
}
#[repr(C, align(8))]
pub struct FFabAnalyticsEventValue {
    pub platform: FString,
    pub api_version: FFabApiVersion,
}
#[repr(C, align(8))]
pub struct FFabApiVersion {
    pub ue: FString,
    pub api: FString,
    pub pluginversion: FString,
    pub platform: FString,
}
#[repr(C, align(8))]
pub struct FFabAnalyticsPayload {
    pub interaction_type: FString,
    pub event_category: FString,
    pub event_action: FString,
    pub event_label: FString,
    pub event_type: FString,
    pub event_value: FFabAnalyticsEventValue,
}
#[repr(C, align(8))]
pub struct FFabFrontendSettings {
    pub preferredformat: FString,
    pub preferredquality: FString,
}
#[repr(C, align(8))]
pub struct FMegascanMaterialPair {
    pub standard_material: TSoftObjectPtr<UMaterialInterface>,
    pub vt_material: TSoftObjectPtr<UMaterialInterface>,
}
#[repr(C, align(1))]
pub struct FFabDistributionMethodTag {}
#[repr(C, align(4))]
pub struct FFabObjectNameColumn {
    pub name: FName,
}
#[repr(C, align(8))]
pub struct FFabObjectColumn {
    pub description: FString,
    pub asset_id: FGuid,
    pub asset_namespace: FGuid,
    pub listing_type: FName,
    pub seller: FString,
    pub source: FName,
    pub url_string: FString,
}
#[repr(C, align(8))]
pub struct FSemanticTags {
    pub asset_type: FString,
}
#[repr(C, align(8))]
pub struct FAssetMetaDataJson {
    pub id: FString,
    pub categories: TArray<FString>,
    pub semantic_tags: FSemanticTags,
    pub displacement_bias_tier1: f32,
    pub displacement_scale_tier1: f32,
}
pub struct UFabLocalAssets {
    pub paths_listing_id: TMap<FString, FString>,
}
pub struct UEosConstants {
    pub game_dev: FEosConstantsGameDev,
    pub prod: FEosConstantsProd,
}
pub struct UFabBrowserApi {}
pub struct UFabSettings {
    pub environment: EFabEnvironment,
    pub custom_url: FString,
    pub custom_auth_token: FString,
    pub b_enable_debug_options: bool,
    pub cache_directory_path: FDirectoryPath,
    pub cache_directory_size: FString,
    pub product_formats_section_sub_text: FString,
    pub preferred_default_format: EFabPreferredFormats,
    pub preferred_quality_tier: EFabPreferredQualityTier,
}
pub struct UFabPlaceholderSpawner {}
pub struct UFabStaticMeshPlaceholderSpawner {}
pub struct UFabSkeletalMeshPlaceholderSpawner {}
pub struct UFabDecalPlaceholderSpawner {}
pub struct UInterchangeInstancedFoliageTypeFactory {}
pub struct UMegascansMaterialParentSettings {
    pub material_parents: TMap<EMegascanMaterialType, FMegascanMaterialPair>,
}
pub struct UInterchangeMegascansPipeline {
    pub megascan_import_type: EMegascanImportType,
    pub megascans_material_parent_settings: UPtr<UMegascansMaterialParentSettings>,
    pub base_node_container: UPtr<UInterchangeBaseNodeContainer>,
}
pub struct UInterchangeInstancedFoliageTypeFactoryNode {}
pub struct UFabFactory {}
