#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FMetaHumanAssetReportItem {
    pub message: FText,
    pub project_item: UPtr<crate::bindings::core_u_object::UObject>,
    pub source_item: FString,
}
impl FMetaHumanAssetReportItem {}
#[repr(C, align(1))]
pub struct FMetaHumanImportOptions {
    pub b_verbose: bool,
    pub b_force_update: bool,
}
impl FMetaHumanImportOptions {}
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
    pub platforms_included: TArray<
        crate::bindings::meta_human_sdk_runtime::EMetaHumanQualityLevel,
    >,
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
    pub card_mesh_texture_resolution: crate::bindings::core_u_object::FIntVector2,
    pub volume_mesh_count: i32,
    pub volume_mesh_vertices: i32,
    pub volume_mesh_texture_resolution: crate::bindings::core_u_object::FIntVector2,
    pub num_materials: i32,
    pub engine_version: FString,
}
impl FMetaHumanAggregateDetails {}
#[repr(C, align(8))]
pub struct FMetaHumanAssetDescription {
    pub name: FName,
    pub asset_data: crate::bindings::core_u_object::FAssetData,
    pub dependent_packages: TArray<FName>,
    pub asset_type: EMetaHumanAssetType,
    pub details: FMetaHumanAggregateDetails,
    pub total_size: i32,
    pub verification_report: UPtr<UMetaHumanAssetReport>,
}
impl FMetaHumanAssetDescription {}
#[repr(C, align(8))]
pub struct FMetaHumanMultiArchiveDescription {
    pub contained_archives: TArray<FString>,
}
impl FMetaHumanMultiArchiveDescription {}
#[repr(C, align(8))]
pub struct FMetaHumanArchiveEntry {
    pub path: FString,
    pub version: FString,
}
impl FMetaHumanArchiveEntry {}
#[repr(C, align(8))]
pub struct FMetaHumanArchiveContents {
    pub files: TArray<FMetaHumanArchiveEntry>,
}
impl FMetaHumanArchiveContents {}
#[repr(C, align(1))]
pub struct FMetaHumanVerificationOptions {
    pub b_verbose: bool,
    pub b_treat_warnings_as_errors: bool,
    pub b_verify_packaging_rules: bool,
}
impl FMetaHumanVerificationOptions {}
#[repr(C, align(8))]
pub struct UMetaHumanPackageFactory {
    __padding_end: [u8; 136],
}
impl UMetaHumanPackageFactory {}
#[repr(C, align(8))]
pub struct UMetaHumanAssetReport {
    __padding_end: [u8; 120],
}
impl UMetaHumanAssetReport {}
#[repr(C, align(8))]
pub struct UMetaHumanAssetManager {
    __padding_end: [u8; 48],
}
impl UMetaHumanAssetManager {}
#[repr(C, align(8))]
pub struct UMetaHumanVerificationRuleBase {
    __padding_end: [u8; 48],
}
impl UMetaHumanVerificationRuleBase {}
#[repr(C, align(8))]
pub struct UMetaHumanVerificationRuleCollection {
    __padding_end: [u8; 64],
}
impl UMetaHumanVerificationRuleCollection {}
#[repr(C, align(8))]
pub struct UVerifyMetaHumanCharacter {
    __padding_end: [u8; 48],
}
impl UVerifyMetaHumanCharacter {}
#[repr(C, align(8))]
pub struct UVerifyMetaHumanGroom {
    __padding_end: [u8; 48],
}
impl UVerifyMetaHumanGroom {}
#[repr(C, align(8))]
pub struct UVerifyMetaHumanOutfitClothing {
    __padding_end: [u8; 48],
}
impl UVerifyMetaHumanOutfitClothing {}
#[repr(C, align(8))]
pub struct UVerifyMetaHumanPackageSource {
    __padding_end: [u8; 48],
}
impl UVerifyMetaHumanPackageSource {}
#[repr(C, align(8))]
pub struct UVerifyMetaHumanSkeletalClothing {
    __padding_end: [u8; 48],
}
impl UVerifyMetaHumanSkeletalClothing {}
#[repr(C, align(8))]
pub struct UVerifyObjectValid {
    __padding_end: [u8; 48],
}
impl UVerifyObjectValid {}
#[repr(C, align(8))]
pub struct UMetaHumanCharacterTypesVerificationExtensionBase {
    __padding_end: [u8; 48],
}
impl UMetaHumanCharacterTypesVerificationExtensionBase {}
#[repr(C, align(8))]
pub struct UMetaHumanCloudServicesSettings {
    __padding_end: [u8; 264],
}
impl UMetaHumanCloudServicesSettings {}
#[repr(C, align(8))]
pub struct UMetaHumanSDKSettings {
    __padding_end: [u8; 176],
}
impl UMetaHumanSDKSettings {}
#[repr(transparent)]
pub struct EMetaHumanAssetType(pub u8);
impl EMetaHumanAssetType {
    pub const CHARACTER: EMetaHumanAssetType = EMetaHumanAssetType(0);
    pub const CHARACTER_ASSEMBLY: EMetaHumanAssetType = EMetaHumanAssetType(1);
    pub const SKELETAL_CLOTHING: EMetaHumanAssetType = EMetaHumanAssetType(2);
    pub const OUTFIT_CLOTHING: EMetaHumanAssetType = EMetaHumanAssetType(3);
    pub const GROOM: EMetaHumanAssetType = EMetaHumanAssetType(4);
}
#[repr(transparent)]
pub struct EMetaHumanOperationResult(pub u8);
impl EMetaHumanOperationResult {
    pub const SUCCESS: EMetaHumanOperationResult = EMetaHumanOperationResult(0);
    pub const FAILURE: EMetaHumanOperationResult = EMetaHumanOperationResult(1);
}
#[repr(transparent)]
pub struct EMetaHumanCloudServiceEnvironment(pub i32);
impl EMetaHumanCloudServiceEnvironment {
    pub const PRODUCTION: EMetaHumanCloudServiceEnvironment = EMetaHumanCloudServiceEnvironment(
        0,
    );
    pub const GAME_DEV: EMetaHumanCloudServiceEnvironment = EMetaHumanCloudServiceEnvironment(
        1,
    );
}
