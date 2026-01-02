#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UFabLocalAssets {
    __padding_end: [u8; 208],
}
impl UFabLocalAssets {}
#[repr(C, align(8))]
pub struct UEosConstants {
    __padding_end: [u8; 312],
}
impl UEosConstants {}
#[repr(C, align(8))]
pub struct UFabBrowserApi {
    __padding_end: [u8; 96],
}
impl UFabBrowserApi {}
#[repr(C, align(8))]
pub struct UFabSettings {
    __padding_end: [u8; 152],
}
impl UFabSettings {}
#[repr(C, align(8))]
pub struct UFabPlaceholderSpawner {
    __padding_end: [u8; 168],
}
impl UFabPlaceholderSpawner {}
#[repr(C, align(8))]
pub struct UFabStaticMeshPlaceholderSpawner {
    __padding_end: [u8; 168],
}
impl UFabStaticMeshPlaceholderSpawner {}
#[repr(C, align(8))]
pub struct UFabSkeletalMeshPlaceholderSpawner {
    __padding_end: [u8; 168],
}
impl UFabSkeletalMeshPlaceholderSpawner {}
#[repr(C, align(8))]
pub struct UFabDecalPlaceholderSpawner {
    __padding_end: [u8; 168],
}
impl UFabDecalPlaceholderSpawner {}
#[repr(C, align(8))]
pub struct UInterchangeInstancedFoliageTypeFactory {
    __padding_end: [u8; 56],
}
impl UInterchangeInstancedFoliageTypeFactory {}
#[repr(C, align(8))]
pub struct UMegascansMaterialParentSettings {
    __padding_end: [u8; 184],
}
impl UMegascansMaterialParentSettings {}
#[repr(C, align(8))]
pub struct UInterchangeMegascansPipeline {
    #[doc(hidden)]
    __padding_344: [u8; 344],
    pub megascan_import_type: EMegascanImportType,
    pub megascans_material_parent_settings: UPtr<UMegascansMaterialParentSettings>,
    __padding_end: [u8; 80],
}
impl UInterchangeMegascansPipeline {}
#[repr(C, align(8))]
pub struct UInterchangeInstancedFoliageTypeFactoryNode {
    __padding_end: [u8; 624],
}
impl UInterchangeInstancedFoliageTypeFactoryNode {}
#[repr(C, align(8))]
pub struct UFabFactory {
    __padding_end: [u8; 56],
}
impl UFabFactory {}
#[repr(transparent)]
pub struct EFabEnvironment(pub u8);
impl EFabEnvironment {
    pub const PROD: EFabEnvironment = EFabEnvironment(0);
    pub const GAMEDEV: EFabEnvironment = EFabEnvironment(1);
    pub const TEST: EFabEnvironment = EFabEnvironment(2);
    pub const CUSTOM_URL: EFabEnvironment = EFabEnvironment(3);
}
#[repr(transparent)]
pub struct EFabPreferredFormats(pub u8);
impl EFabPreferredFormats {
    pub const GLTF: EFabPreferredFormats = EFabPreferredFormats(0);
    pub const FBX: EFabPreferredFormats = EFabPreferredFormats(1);
}
#[repr(transparent)]
pub struct EFabPreferredQualityTier(pub u8);
impl EFabPreferredQualityTier {
    pub const LOW: EFabPreferredQualityTier = EFabPreferredQualityTier(0);
    pub const MEDIUM: EFabPreferredQualityTier = EFabPreferredQualityTier(1);
    pub const HIGH: EFabPreferredQualityTier = EFabPreferredQualityTier(2);
    pub const RAW: EFabPreferredQualityTier = EFabPreferredQualityTier(3);
}
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
#[repr(transparent)]
pub struct EMegascanImportType(pub i32);
impl EMegascanImportType {
    pub const MODEL3_D: EMegascanImportType = EMegascanImportType(0);
    pub const SURFACE: EMegascanImportType = EMegascanImportType(1);
    pub const DECAL: EMegascanImportType = EMegascanImportType(2);
    pub const IMPERFECTION: EMegascanImportType = EMegascanImportType(3);
    pub const PLANT: EMegascanImportType = EMegascanImportType(4);
}
