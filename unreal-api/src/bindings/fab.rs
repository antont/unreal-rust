#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
    pub standard_material: TSoftObjectPtr<crate::bindings::engine::UMaterialInterface>,
    pub vt_material: TSoftObjectPtr<crate::bindings::engine::UMaterialInterface>,
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
    pub asset_id: crate::bindings::core_u_object::FGuid,
    pub asset_namespace: crate::bindings::core_u_object::FGuid,
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
    pub cache_directory_path: crate::bindings::core_u_object::FDirectoryPath,
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
    pub base_node_container: UPtr<
        crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
    >,
}
pub struct UInterchangeInstancedFoliageTypeFactoryNode {}
pub struct UFabFactory {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFabEnvironment(pub u8);
impl EFabEnvironment {
    pub const PROD: EFabEnvironment = EFabEnvironment(0);
    pub const GAMEDEV: EFabEnvironment = EFabEnvironment(1);
    pub const TEST: EFabEnvironment = EFabEnvironment(2);
    pub const CUSTOM_URL: EFabEnvironment = EFabEnvironment(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFabPreferredFormats(pub u8);
impl EFabPreferredFormats {
    pub const GLTF: EFabPreferredFormats = EFabPreferredFormats(0);
    pub const FBX: EFabPreferredFormats = EFabPreferredFormats(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFabPreferredQualityTier(pub u8);
impl EFabPreferredQualityTier {
    pub const LOW: EFabPreferredQualityTier = EFabPreferredQualityTier(0);
    pub const MEDIUM: EFabPreferredQualityTier = EFabPreferredQualityTier(1);
    pub const HIGH: EFabPreferredQualityTier = EFabPreferredQualityTier(2);
    pub const RAW: EFabPreferredQualityTier = EFabPreferredQualityTier(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMegascanMaterialType(pub i32);
impl EMegascanMaterialType {
    pub const INVALID: EMegascanMaterialType = EMegascanMaterialType(0);
    pub const BASE: EMegascanMaterialType = EMegascanMaterialType(1);
    pub const BASE_MASKED: EMegascanMaterialType = EMegascanMaterialType(2);
    pub const BASE_FUZZ: EMegascanMaterialType = EMegascanMaterialType(3);
    pub const BASE_TRANSMISSION: EMegascanMaterialType = EMegascanMaterialType(4);
    pub const GLASS: EMegascanMaterialType = EMegascanMaterialType(5);
    pub const SURFACE: EMegascanMaterialType = EMegascanMaterialType(6);
    pub const SURFACE_MASKED: EMegascanMaterialType = EMegascanMaterialType(7);
    pub const SURFACE_FUZZ: EMegascanMaterialType = EMegascanMaterialType(8);
    pub const SURFACE_TRANSMISSION: EMegascanMaterialType = EMegascanMaterialType(9);
    pub const FABRIC: EMegascanMaterialType = EMegascanMaterialType(10);
    pub const FABRIC_MASKED: EMegascanMaterialType = EMegascanMaterialType(11);
    pub const DECAL: EMegascanMaterialType = EMegascanMaterialType(12);
    pub const PLANT: EMegascanMaterialType = EMegascanMaterialType(13);
    pub const PLANT_BILLBOARD: EMegascanMaterialType = EMegascanMaterialType(14);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMegascanImportType(pub i32);
impl EMegascanImportType {
    pub const MODEL3_D: EMegascanImportType = EMegascanImportType(0);
    pub const SURFACE: EMegascanImportType = EMegascanImportType(1);
    pub const DECAL: EMegascanImportType = EMegascanImportType(2);
    pub const IMPERFECTION: EMegascanImportType = EMegascanImportType(3);
    pub const PLANT: EMegascanImportType = EMegascanImportType(4);
}
