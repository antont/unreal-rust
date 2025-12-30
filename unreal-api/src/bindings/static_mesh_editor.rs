#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FStaticMeshReductionSettings {
    pub percent_triangles: f32,
    pub screen_size: f32,
}
#[repr(C, align(8))]
pub struct FStaticMeshReductionOptions {
    pub b_auto_compute_lod_screen_size: bool,
    pub reduction_settings: TArray<FStaticMeshReductionSettings>,
}
#[repr(C, align(8))]
pub struct FJoinStaticMeshActorsOptions {
    pub b_destroy_source_actors: bool,
    pub new_actor_label: FString,
    pub b_rename_components_from_source: bool,
}
#[repr(C, align(8))]
pub struct FMergeStaticMeshActorsOptions {
    pub b_spawn_merged_actor: bool,
    pub base_package_name: FString,
    pub mesh_merging_settings: FMeshMergingSettings,
}
#[repr(C, align(8))]
pub struct FCreateProxyMeshActorOptions {
    pub b_spawn_merged_actor: bool,
    pub base_package_name: FString,
    pub mesh_proxy_settings: FMeshProxySettings,
}
pub struct UStaticMeshEditorUISubsystem {}
pub struct UStaticMeshEditorSubsystem {}
