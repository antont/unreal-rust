#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FCustomFieldData {
    pub key: FString,
    pub value: FString,
}
#[repr(C, align(1))]
pub struct FSHAHashData {
    pub hash: u8,
}
#[repr(C, align(8))]
pub struct FChunkInfoData {
    pub guid: crate::bindings::core_u_object::FGuid,
    pub hash: u64,
    pub sha_hash: FSHAHashData,
    pub file_size: i64,
    pub group_number: u8,
}
#[repr(C, align(4))]
pub struct FChunkPartData {
    pub guid: crate::bindings::core_u_object::FGuid,
    pub offset: u32,
    pub size: u32,
}
#[repr(C, align(8))]
pub struct FFileManifestData {
    pub filename: FString,
    pub file_hash: FSHAHashData,
    pub file_chunk_parts: TArray<FChunkPartData>,
    pub install_tags: TArray<FString>,
    pub b_is_unix_executable: bool,
    pub symlink_target: FString,
    pub b_is_read_only: bool,
    pub b_is_compressed: bool,
}
pub struct UBuildPatchManifest {
    pub manifest_file_version: u8,
    pub b_is_file_data: bool,
    pub app_id: u32,
    pub app_name: FString,
    pub build_version: FString,
    pub launch_exe: FString,
    pub launch_command: FString,
    pub prereq_ids: TSet<FString>,
    pub prereq_name: FString,
    pub prereq_path: FString,
    pub prereq_args: FString,
    pub file_manifest_list: TArray<FFileManifestData>,
    pub chunk_list: TArray<FChunkInfoData>,
    pub custom_fields: TArray<FCustomFieldData>,
}
