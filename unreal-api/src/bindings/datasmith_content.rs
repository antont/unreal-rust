#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FDatasmithImportInfo {
    pub source_uri: FString,
    pub source_hash: FString,
}
#[repr(C, align(4))]
pub struct FDatasmithAssetImportOptions {
    pub package_path: FName,
}
#[repr(C, align(1))]
pub struct FDatasmithStaticMeshImportOptions {
    pub min_lightmap_resolution: EDatasmithImportLightmapMin,
    pub max_lightmap_resolution: EDatasmithImportLightmapMax,
    pub b_generate_lightmap_u_vs: bool,
    pub b_remove_degenerates: bool,
}
#[repr(C, align(1))]
pub struct FDatasmithReimportOptions {
    pub b_update_actors: bool,
    pub b_respawn_deleted_actors: bool,
}
#[repr(C, align(4))]
pub struct FDatasmithImportBaseOptions {
    pub scene_handling: EDatasmithImportScene,
    pub b_include_geometry: bool,
    pub b_include_material: bool,
    pub b_include_light: bool,
    pub b_include_camera: bool,
    pub b_include_animation: bool,
    pub asset_options: FDatasmithAssetImportOptions,
    pub static_mesh_options: FDatasmithStaticMeshImportOptions,
}
#[repr(C, align(8))]
pub struct FDatasmithTessellationOptions {
    pub chord_tolerance: f32,
    pub max_edge_length: f32,
    pub normal_tolerance: f32,
    pub stitching_technique: EDatasmithCADStitchingTechnique,
    pub geometric_tolerance: f64,
    pub stitching_tolerance: f64,
}
#[repr(C, align(8))]
pub struct FDatasmithRetessellationOptions {
    pub retessellation_rule: EDatasmithCADRetessellationRule,
}
#[repr(C, align(8))]
pub struct FDatasmithCameraLookatTrackingSettingsTemplate {
    pub flags_0: u8,
    pub actor_to_track: TSoftObjectPtr<AActor>,
}
#[repr(C, align(4))]
pub struct FDatasmithCameraFilmbackSettingsTemplate {
    pub sensor_width: f32,
    pub sensor_height: f32,
}
#[repr(C, align(4))]
pub struct FDatasmithCameraLensSettingsTemplate {
    pub max_f_stop: f32,
}
#[repr(C, align(4))]
pub struct FDatasmithCameraFocusSettingsTemplate {
    pub focus_method: ECameraFocusMethod,
    pub manual_focus_distance: f32,
}
#[repr(C, align(16))]
pub struct FDatasmithPostProcessSettingsTemplate {
    pub flags_0: u8,
    pub flags_4: u8,
    pub white_temp: f32,
    pub vignette_intensity: f32,
    pub color_saturation: FVector4,
    pub auto_exposure_method: EAutoExposureMethod,
    pub camera_iso: f32,
    pub camera_shutter_speed: f32,
    pub depth_of_field_fstop: f32,
}
#[repr(C, align(8))]
pub struct FDatasmithStaticParameterSetTemplate {
    pub static_switch_parameters: TMap<FName, bool>,
}
#[repr(C, align(4))]
pub struct FDatasmithMeshBuildSettingsTemplate {
    pub flags_0: u8,
    pub min_lightmap_resolution: i32,
    pub src_lightmap_index: i32,
    pub dst_lightmap_index: i32,
}
#[repr(C, align(8))]
pub struct FDatasmithStaticMaterialTemplate {
    pub material_slot_name: FName,
    pub material_interface: UPtr<UMaterialInterface>,
}
#[repr(C, align(4))]
pub struct FDatasmithMeshSectionInfoTemplate {
    pub material_index: i32,
}
#[repr(C, align(8))]
pub struct FDatasmithMeshSectionInfoMapTemplate {
    pub map: TMap<u32, FDatasmithMeshSectionInfoTemplate>,
}
pub struct UDatasmithAdditionalData {}
pub struct ADatasmithAreaLightActor {
    pub mobility: EComponentMobility,
    pub light_type: EDatasmithAreaLightActorType,
    pub light_shape: EDatasmithAreaLightActorShape,
    pub dimensions: FVector2D,
    pub intensity: f32,
    pub intensity_units: ELightUnits,
    pub color: FLinearColor,
    pub temperature: f32,
    pub ies_texture: UPtr<UTextureLightProfile>,
    pub b_use_ies_brightness: bool,
    pub ies_brightness_scale: f32,
    pub rotation: FRotator,
    pub source_radius: f32,
    pub source_length: f32,
    pub attenuation_radius: f32,
    pub spotlight_inner_angle: f32,
    pub spotlight_outer_angle: f32,
}
pub struct UDatasmithAssetImportData {
    pub asset_import_options: FDatasmithAssetImportOptions,
    pub additional_data: TArray<UPtr<UDatasmithAdditionalData>>,
    pub datasmith_import_info: FDatasmithImportInfo,
}
pub struct UDatasmithStaticMeshImportData {
    pub import_options: FDatasmithStaticMeshImportOptions,
}
pub struct UDatasmithStaticMeshCADImportData {
    pub tessellation_options: FDatasmithTessellationOptions,
    pub model_unit: f64,
    pub model_tolerance: f64,
    pub resource_path: FString,
    pub resource_filename: FString,
    pub auxiliary_filenames: TArray<FString>,
}
pub struct UDatasmithSceneImportData {
    pub base_options: FDatasmithImportBaseOptions,
    pub datasmith_import_info: FDatasmithImportInfo,
}
pub struct UDatasmithTranslatedSceneImportData {
    pub additional_options: TArray<UPtr<UDatasmithOptionsBase>>,
}
pub struct UDatasmithCADImportSceneData {
    pub tessellation_options: FDatasmithTessellationOptions,
}
pub struct UDatasmithMDLSceneImportData {}
pub struct UDatasmithGLTFSceneImportData {
    pub generator: FString,
    pub version: f32,
    pub author: FString,
    pub license: FString,
    pub source: FString,
}
pub struct UDatasmithStaticMeshGLTFImportData {
    pub source_mesh_name: FString,
}
pub struct UDatasmithFBXSceneImportData {
    pub b_generate_lightmap_u_vs: bool,
    pub textures_dir: FString,
    pub intermediate_serialization: u8,
    pub b_colorize_materials: bool,
}
pub struct UDatasmithDeltaGenAssetImportData {}
pub struct UDatasmithDeltaGenSceneImportData {
    pub b_merge_nodes: bool,
    pub b_optimize_duplicated_nodes: bool,
    pub b_remove_invisible_nodes: bool,
    pub b_simplify_node_hierarchy: bool,
    pub b_import_var: bool,
    pub var_path: FString,
    pub b_import_pos: bool,
    pub pos_path: FString,
    pub b_import_tml: bool,
    pub tml_path: FString,
}
pub struct UDatasmithVREDAssetImportData {}
pub struct UDatasmithVREDSceneImportData {
    pub b_merge_nodes: bool,
    pub b_optimize_duplicated_nodes: bool,
    pub b_import_mats: bool,
    pub mats_path: FString,
    pub b_import_var: bool,
    pub b_clean_var: bool,
    pub var_path: FString,
    pub b_import_light_info: bool,
    pub light_info_path: FString,
    pub b_import_clip_info: bool,
    pub clip_info_path: FString,
}
pub struct UDatasmithAssetUserData {
    pub meta_data: TMap<FName, FString>,
    pub object_templates: TMap<
        TSubclassOf<UDatasmithObjectTemplate>,
        UPtr<UDatasmithObjectTemplate>,
    >,
}
pub struct UDatasmithContentBlueprintLibrary {}
pub struct UDatasmithCustomActionBase {}
pub struct ADatasmithImportedSequencesActor {
    pub imported_sequences: TArray<UPtr<ULevelSequence>>,
}
pub struct UDatasmithOptionsBase {}
pub struct UDatasmithCommonTessellationOptions {
    pub options: FDatasmithTessellationOptions,
}
pub struct UDatasmithImportOptions {
    pub search_package_policy: EDatasmithImportSearchPackagePolicy,
    pub material_conflict_policy: EDatasmithImportAssetConflictPolicy,
    pub texture_conflict_policy: EDatasmithImportAssetConflictPolicy,
    pub static_mesh_actor_import_policy: EDatasmithImportActorPolicy,
    pub light_import_policy: EDatasmithImportActorPolicy,
    pub camera_import_policy: EDatasmithImportActorPolicy,
    pub other_actor_import_policy: EDatasmithImportActorPolicy,
    pub material_quality: EDatasmithImportMaterialQuality,
    pub base_options: FDatasmithImportBaseOptions,
    pub reimport_options: FDatasmithReimportOptions,
    pub file_name: FString,
    pub file_path: FString,
    pub source_uri: FString,
}
pub struct UDatasmithScene {
    pub asset_import_data: UPtr<UDatasmithSceneImportData>,
    pub bulk_data_version: i32,
    pub static_meshes: TMap<FName, TSoftObjectPtr<UStaticMesh>>,
    pub clothes: TMap<FName, TSoftObjectPtr<UObject>>,
    pub textures: TMap<FName, TSoftObjectPtr<UTexture>>,
    pub material_functions: TMap<FName, TSoftObjectPtr<UMaterialFunction>>,
    pub materials: TMap<FName, TSoftObjectPtr<UMaterialInterface>>,
    pub level_sequences: TMap<FName, TSoftObjectPtr<ULevelSequence>>,
    pub level_variant_sets: TMap<FName, TSoftObjectPtr<ULevelVariantSets>>,
    pub asset_user_data: TArray<UPtr<UAssetUserData>>,
}
pub struct ADatasmithSceneActor {
    pub scene: UPtr<UDatasmithScene>,
    pub related_actors: TMap<FName, TSoftObjectPtr<AActor>>,
}
pub struct UDatasmithObjectTemplate {}
pub struct UDatasmithActorTemplate {
    pub layers: TSet<FName>,
    pub tags: TSet<FName>,
}
pub struct UDatasmithAreaLightActorTemplate {
    pub light_type: EDatasmithAreaLightActorType,
    pub light_shape: EDatasmithAreaLightActorShape,
    pub dimensions: FVector2D,
    pub color: FLinearColor,
    pub intensity: f32,
    pub intensity_units: ELightUnits,
    pub temperature: f32,
    pub ies_texture: TSoftObjectPtr<UTextureLightProfile>,
    pub b_use_ies_brightness: bool,
    pub ies_brightness_scale: f32,
    pub rotation: FRotator,
    pub source_radius: f32,
    pub source_length: f32,
    pub attenuation_radius: f32,
}
pub struct UDatasmithCineCameraActorTemplate {
    pub lookat_tracking_settings: FDatasmithCameraLookatTrackingSettingsTemplate,
}
pub struct UDatasmithCineCameraComponentTemplate {
    pub filmback_settings: FDatasmithCameraFilmbackSettingsTemplate,
    pub lens_settings: FDatasmithCameraLensSettingsTemplate,
    pub focus_settings: FDatasmithCameraFocusSettingsTemplate,
    pub current_focal_length: f32,
    pub current_aperture: f32,
    pub post_process_settings: FDatasmithPostProcessSettingsTemplate,
}
pub struct UDatasmithDecalComponentTemplate {
    pub sort_order: i32,
    pub decal_size: FVector,
    pub material: UPtr<UMaterialInterface>,
}
pub struct UDatasmithLandscapeTemplate {
    pub landscape_material: UPtr<UMaterialInterface>,
    pub static_lighting_lod: i32,
}
pub struct UDatasmithLightComponentTemplate {
    pub flags_56: u8,
    pub flags_60: u8,
    pub intensity: f32,
    pub temperature: f32,
    pub ies_brightness_scale: f32,
    pub light_color: FLinearColor,
    pub light_function_material: UPtr<UMaterialInterface>,
    pub ies_texture: UPtr<UTextureLightProfile>,
}
pub struct UDatasmithMaterialInstanceTemplate {
    pub parent_material: TSoftObjectPtr<UMaterialInterface>,
    pub scalar_parameter_values: TMap<FName, f32>,
    pub vector_parameter_values: TMap<FName, FLinearColor>,
    pub texture_parameter_values: TMap<FName, TSoftObjectPtr<UTexture>>,
    pub static_parameters: FDatasmithStaticParameterSetTemplate,
}
pub struct UDatasmithPointLightComponentTemplate {
    pub intensity_units: ELightUnits,
    pub source_radius: f32,
    pub source_length: f32,
    pub attenuation_radius: f32,
}
pub struct UDatasmithPostProcessVolumeTemplate {
    pub settings: FDatasmithPostProcessSettingsTemplate,
    pub flags_128: u8,
}
pub struct UDatasmithSceneComponentTemplate {
    pub relative_transform: FTransform,
    pub mobility: EComponentMobility,
    pub attach_parent: TSoftObjectPtr<USceneComponent>,
    pub b_visible: bool,
    pub b_cast_shadow: bool,
    pub tags: TSet<FName>,
}
pub struct UDatasmithSkyLightComponentTemplate {
    pub source_type: ESkyLightSourceType,
    pub cubemap_resolution: i32,
    pub cubemap: UPtr<UTextureCube>,
}
pub struct UDatasmithSpotLightComponentTemplate {
    pub inner_cone_angle: f32,
    pub outer_cone_angle: f32,
}
pub struct UDatasmithStaticMeshComponentTemplate {
    pub static_mesh: UPtr<UStaticMesh>,
    pub override_materials: TArray<UPtr<UMaterialInterface>>,
}
pub struct UDatasmithStaticMeshTemplate {
    pub section_info_map: FDatasmithMeshSectionInfoMapTemplate,
    pub light_map_coordinate_index: i32,
    pub light_map_resolution: i32,
    pub build_settings: TArray<FDatasmithMeshBuildSettingsTemplate>,
    pub static_materials: TArray<FDatasmithStaticMaterialTemplate>,
}
