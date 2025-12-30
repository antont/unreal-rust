#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FMaterialUsage {
    pub instance_id: FString,
    pub material_slot: FString,
}
#[repr(C, align(8))]
pub struct FMeshInfo {
    pub mesh_id: FString,
    pub name: FString,
    pub path: FString,
    pub number_of_lods: i8,
    pub material_usage: TArray<FMaterialUsage>,
}
#[repr(C, align(8))]
pub struct FFoliageTypeInfo {
    pub path: FString,
}
#[repr(C, align(8))]
pub struct FMaterialParams {
    pub param_name: FString,
    pub used_texture_id: FString,
}
#[repr(C, align(8))]
pub struct FMaterialInstanceInfo {
    pub instance_id: FString,
    pub instance_name: FString,
    pub instance_path: FString,
    pub instance_master: FString,
    pub ty: FString,
    pub params: TArray<FMaterialParams>,
}
#[repr(C, align(8))]
pub struct FMasterMaterialInfo {
    pub master_id: FString,
    pub master_material_name: FString,
    pub path: FString,
}
#[repr(C, align(8))]
pub struct FChannelPackedInfo {
    pub channel: FString,
    pub packed_type: FString,
}
#[repr(C, align(8))]
pub struct FTextureUsage {
    pub mat_id: FString,
    pub mat_params: FString,
}
#[repr(C, align(8))]
pub struct FTexturesList {
    pub texture_id: FString,
    pub ty: FString,
    pub resolution: FString,
    pub name: FString,
    pub path: FString,
    pub is_channel_packed: bool,
    pub channel_pack_info: TArray<FChannelPackedInfo>,
    pub plugged_in: TArray<FTextureUsage>,
}
#[repr(C, align(8))]
pub struct FUAssetMeta {
    pub asset_id: FString,
    pub asset_name: FString,
    pub asset_type: FString,
    pub asset_sub_type: FString,
    pub asset_tier: i8,
    pub asset_root_path: FString,
    pub mesh_list: TArray<FMeshInfo>,
    pub foliage_asset_paths: TArray<FString>,
    pub material_instances: TArray<FMaterialInstanceInfo>,
    pub texture_sets: TArray<FTexturesList>,
    pub master_materials: TArray<FMasterMaterialInfo>,
}
#[repr(C, align(8))]
pub struct FAssetInfo {
    pub path: FString,
    pub version: FString,
}
#[repr(C, align(8))]
pub struct FVersionData {
    pub assets: TArray<FAssetInfo>,
}
#[repr(C, align(8))]
pub struct FAssetImportTime {
    pub path: FString,
    pub time: FString,
}
#[repr(C, align(8))]
pub struct FImportTimeData {
    pub assets: TArray<FAssetImportTime>,
}
pub struct UMegascansSettings {
    pub b_create_foliage: bool,
    pub b_apply_to_selection: bool,
}
pub struct UMaterialBlendSettings {
    pub blended_material_name: FString,
    pub blended_material_path: FDirectoryPath,
}
pub struct UMaterialAssetSettings {
    pub master_material3d: FString,
    pub master_material_surface: FString,
    pub master_material_plant: FString,
}
pub struct UMaterialPresetsSettings {
    pub master_material3d: TLazyObjectPtr<UMaterial>,
    pub master_material_surface: TLazyObjectPtr<UMaterial>,
    pub master_material_plant: TLazyObjectPtr<UMaterial>,
}
pub struct UVersionInfoHandler {
    pub common_version_data: FVersionData,
}
