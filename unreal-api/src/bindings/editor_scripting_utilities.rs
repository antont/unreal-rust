#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FEditorDialogLibraryObjectDetailsViewOptions {
    pub b_show_object_name: bool,
    pub b_allow_search: bool,
    pub b_allow_resizing: bool,
    pub min_width: i32,
    pub min_height: i32,
    pub value_column_width_ratio: f32,
}
#[repr(C, align(8))]
pub struct FEditorScriptingJoinStaticMeshActorsOptions_Deprecated {
    pub b_destroy_source_actors: bool,
    pub new_actor_label: FString,
    pub b_rename_components_from_source: bool,
}
#[repr(C, align(8))]
pub struct FEditorScriptingMergeStaticMeshActorsOptions_Deprecated {
    pub b_spawn_merged_actor: bool,
    pub base_package_name: FString,
    pub mesh_merging_settings: FMeshMergingSettings,
}
#[repr(C, align(8))]
pub struct FEditorScriptingCreateProxyMeshActorOptions_Deprecated {
    pub b_spawn_merged_actor: bool,
    pub base_package_name: FString,
    pub mesh_proxy_settings: FMeshProxySettings,
}
#[repr(C, align(4))]
pub struct FEditorScriptingMeshReductionSettings_Deprecated {
    pub percent_triangles: f32,
    pub screen_size: f32,
}
#[repr(C, align(8))]
pub struct FEditorScriptingMeshReductionOptions_Deprecated {
    pub b_auto_compute_lod_screen_size: bool,
    pub reduction_settings: TArray<FEditorScriptingMeshReductionSettings_Deprecated>,
}
pub struct UEditorAssetLibrary {}
pub struct UEditorDialogLibrary {}
pub struct UEditorFilterLibrary {}
pub struct UEditorLevelLibrary {}
pub struct UDEPRECATED_EditorSkeletalMeshLibrary {}
pub struct UDEPRECATED_EditorStaticMeshLibrary {}
