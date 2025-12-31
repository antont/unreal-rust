#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FGroomCacheImportSettings {
    pub b_import_groom_cache: bool,
    pub import_type: EGroomCacheImportType,
    pub frame_start: i32,
    pub frame_end: i32,
    pub b_skip_empty_frames: bool,
    pub b_import_groom_asset: bool,
    pub groom_asset: crate::bindings::core_u_object::FSoftObjectPath,
    pub b_override_conversion_settings: bool,
    pub conversion_settings: FGroomConversionSettings,
}
#[repr(C, align(8))]
pub struct FGroomConversionSettings {
    pub rotation: crate::bindings::core_u_object::FVector,
    pub scale: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(4))]
pub struct FHairGroupDesc {
    pub hair_length: f32,
    pub hair_width: f32,
    pub hair_width_override: bool,
    pub hair_root_scale: f32,
    pub hair_root_scale_override: bool,
    pub hair_tip_scale: f32,
    pub hair_tip_scale_override: bool,
    pub hair_shadow_density: f32,
    pub hair_shadow_density_override: bool,
    pub hair_raytracing_radius_scale: f32,
    pub hair_raytracing_radius_scale_override: bool,
    pub b_use_hair_raytracing_geometry: bool,
    pub b_use_hair_raytracing_geometry_override: bool,
    pub lod_bias: f32,
    pub b_use_stable_rasterization: bool,
    pub b_use_stable_rasterization_override: bool,
    pub b_scatter_scene_lighting: bool,
    pub b_scatter_scene_lighting_override: bool,
    pub b_support_voxelization: bool,
    pub b_support_voxelization_override: bool,
    pub hair_length_scale: f32,
    pub hair_length_scale_override: bool,
}
#[repr(C, align(4))]
pub struct FHairGroupLODInfo {
    pub num_points: i32,
    pub num_curves: i32,
}
#[repr(C, align(8))]
pub struct FHairGroupInfo {
    pub group_index: i32,
    pub group_id: i32,
    pub group_name: FName,
    pub num_curves: i32,
    pub num_guides: i32,
    pub num_curve_vertices: i32,
    pub num_guide_vertices: i32,
    pub max_curve_length: f32,
    pub flags: u32,
    pub lod_infos: TArray<FHairGroupLODInfo>,
}
#[repr(C, align(8))]
pub struct FHairGroupsMaterial {
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub slot_name: FName,
}
#[repr(C, align(8))]
pub struct FHairGroupInfoWithVisibility {
    pub b_is_visible: bool,
}
#[repr(C, align(4))]
pub struct FHairGroupCardsInfo {
    pub num_cards: i32,
    pub num_card_vertices: i32,
}
#[repr(C, align(8))]
pub struct FHairGroupCardsTextures {
    pub layout: EHairTextureLayout,
    pub textures: TArray<UPtr<crate::bindings::engine::UTexture2D>>,
    pub depth_texture_deprecated: UPtr<crate::bindings::engine::UTexture2D>,
    pub coverage_texture_deprecated: UPtr<crate::bindings::engine::UTexture2D>,
    pub tangent_texture_deprecated: UPtr<crate::bindings::engine::UTexture2D>,
    pub attribute_texture_deprecated: UPtr<crate::bindings::engine::UTexture2D>,
    pub auxilary_data_texture_deprecated: UPtr<crate::bindings::engine::UTexture2D>,
    pub material_texture_deprecated: UPtr<crate::bindings::engine::UTexture2D>,
}
#[repr(C, align(8))]
pub struct FHairGroupsCardsSourceDescription {
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub material_slot_name: FName,
    pub source_type_deprecated: EHairCardsSourceType,
    pub procedural_mesh_deprecated: UPtr<crate::bindings::engine::UStaticMesh>,
    pub b_invert_uv: bool,
    pub guide_type: EHairCardsGuideType,
    pub imported_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub textures: FHairGroupCardsTextures,
    pub group_index: i32,
    pub lod_index: i32,
    pub generation_settings: UPtr<UHairCardGenerationSettings>,
    pub cards_info: FHairGroupCardsInfo,
    pub imported_mesh_key: FString,
}
#[repr(C, align(8))]
pub struct FGroomDataflowSettings {
    pub skeletal_meshes: TArray<UPtr<crate::bindings::engine::USkeletalMesh>>,
    pub mesh_lo_ds: TArray<i32>,
    pub preview_binding_asset: TSoftObjectPtr<UGroomBindingAsset>,
    pub preview_animation_asset: TSoftObjectPtr<
        crate::bindings::engine::UAnimationAsset,
    >,
}
#[repr(C, align(4))]
pub struct FHairLODSettings {
    pub curve_decimation: f32,
    pub vertex_decimation: f32,
    pub angular_threshold: f32,
    pub screen_size: f32,
    pub thickness_scale: f32,
    pub b_visible: bool,
    pub geometry_type: EGroomGeometryType,
    pub binding_type: EGroomBindingType,
    pub simulation: EGroomOverrideType,
    pub global_interpolation: EGroomOverrideType,
}
#[repr(C, align(4))]
pub struct FHairDecimationSettings {
    pub curve_decimation: f32,
    pub vertex_decimation: f32,
}
#[repr(C, align(4))]
pub struct FHairInterpolationSettings {
    pub guide_type: EGroomGuideType,
    pub b_override_guides_deprecated: bool,
    pub hair_to_guide_density: f32,
    pub rigged_guide_num_curves: i32,
    pub rigged_guide_num_points: i32,
    pub interpolation_quality: EHairInterpolationQuality,
    pub interpolation_distance: EHairInterpolationWeight,
    pub b_randomize_guide: bool,
    pub b_use_unique_guide: bool,
}
#[repr(C, align(4))]
pub struct FHairDeformationSettings {
    pub b_enable_rigging_deprecated: bool,
    pub num_curves_deprecated: i32,
    pub num_points_deprecated: i32,
}
#[repr(C, align(4))]
pub struct FHairGroupsInterpolation {
    pub decimation_settings: FHairDecimationSettings,
    pub interpolation_settings: FHairInterpolationSettings,
    pub rigging_settings: FHairDeformationSettings,
    pub group_name: FName,
}
#[repr(C, align(8))]
pub struct FHairGroupsLOD {
    pub auto_lod_bias: f32,
    pub lo_ds: TArray<FHairLODSettings>,
}
#[repr(C, align(8))]
pub struct FHairGroupsMeshesSourceDescription {
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub material_slot_name: FName,
    pub imported_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub textures: FHairGroupCardsTextures,
    pub group_index: i32,
    pub lod_index: i32,
    pub imported_mesh_key: FString,
}
#[repr(C, align(8))]
pub struct FHairSolverSettings {
    pub b_enable_deformation: bool,
    pub enable_simulation: bool,
    pub niagara_solver: EGroomNiagaraSolvers,
    pub custom_system: TSoftObjectPtr<crate::bindings::niagara::UNiagaraSystem>,
    pub gravity_preloading: f32,
    pub sub_steps: i32,
    pub iteration_count: i32,
    pub b_force_visible: bool,
}
#[repr(C, align(8))]
pub struct FHairExternalForces {
    pub gravity_vector: crate::bindings::core_u_object::FVector,
    pub air_drag: f32,
    pub air_velocity: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FHairBendConstraint {
    pub solve_bend: bool,
    pub project_bend: bool,
    pub bend_damping: f32,
    pub bend_stiffness: f32,
    pub bend_scale: crate::bindings::engine::FRuntimeFloatCurve,
}
#[repr(C, align(8))]
pub struct FHairStretchConstraint {
    pub solve_stretch: bool,
    pub project_stretch: bool,
    pub stretch_damping: f32,
    pub stretch_stiffness: f32,
    pub stretch_scale: crate::bindings::engine::FRuntimeFloatCurve,
}
#[repr(C, align(8))]
pub struct FHairCollisionConstraint {
    pub solve_collision: bool,
    pub project_collision: bool,
    pub static_friction: f32,
    pub kinetic_friction: f32,
    pub strands_viscosity: f32,
    pub grid_dimension: crate::bindings::core_u_object::FIntVector,
    pub collision_radius: f32,
    pub radius_scale: crate::bindings::engine::FRuntimeFloatCurve,
}
#[repr(C, align(8))]
pub struct FHairMaterialConstraints {
    pub bend_constraint: FHairBendConstraint,
    pub stretch_constraint: FHairStretchConstraint,
    pub collision_constraint: FHairCollisionConstraint,
}
#[repr(C, align(8))]
pub struct FHairStrandsParameters {
    pub strands_size: EGroomStrandsSize,
    pub strands_density: f32,
    pub strands_smoothing: f32,
    pub strands_thickness: f32,
    pub thickness_scale: crate::bindings::engine::FRuntimeFloatCurve,
}
#[repr(C, align(8))]
pub struct FHairGroupsPhysics {
    pub solver_settings: FHairSolverSettings,
    pub external_forces: FHairExternalForces,
    pub material_constraints: FHairMaterialConstraints,
    pub strands_parameters: FHairStrandsParameters,
    pub group_name: FName,
}
#[repr(C, align(1))]
pub struct FHairSimulationSolver {
    pub b_enable_simulation: bool,
}
#[repr(C, align(8))]
pub struct FHairSimulationForces {
    pub gravity_vector: crate::bindings::core_u_object::FVector,
    pub air_drag: f32,
    pub air_velocity: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(4))]
pub struct FHairSimulationConstraints {
    pub bend_damping: f32,
    pub bend_stiffness: f32,
    pub stretch_damping: f32,
    pub stretch_stiffness: f32,
    pub static_friction: f32,
    pub kinetic_friction: f32,
    pub strands_viscosity: f32,
    pub collision_radius: f32,
}
#[repr(C, align(8))]
pub struct FHairSimulationSetup {
    pub b_reset_simulation: bool,
    pub b_debug_simulation: bool,
    pub b_local_simulation: bool,
    pub linear_velocity_scale: f32,
    pub angular_velocity_scale: f32,
    pub local_bone: FString,
    pub teleport_distance: f32,
}
#[repr(C, align(8))]
pub struct FHairSimulationSettings {
    pub b_override_settings: bool,
    pub simulation_setup: FHairSimulationSetup,
    pub solver_settings: FHairSimulationSolver,
    pub external_forces: FHairSimulationForces,
    pub material_constraints: FHairSimulationConstraints,
}
#[repr(C, align(4))]
pub struct FHairGeometrySettings {
    pub hair_width: f32,
    pub hair_width_override: bool,
    pub hair_root_scale: f32,
    pub hair_tip_scale: f32,
}
#[repr(C, align(4))]
pub struct FHairShadowSettings {
    pub hair_shadow_density: f32,
    pub hair_raytracing_radius_scale: f32,
    pub b_use_hair_raytracing_geometry: bool,
    pub b_voxelize: bool,
}
#[repr(C, align(1))]
pub struct FHairAdvancedRenderingSettings {
    pub b_use_stable_rasterization: bool,
    pub b_scatter_scene_lighting: bool,
}
#[repr(C, align(8))]
pub struct FHairGroupsRendering {
    pub material_slot_name: FName,
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub geometry_settings: FHairGeometrySettings,
    pub shadow_settings: FHairShadowSettings,
    pub advanced_settings: FHairAdvancedRenderingSettings,
    pub group_name: FName,
}
#[repr(C, align(4))]
pub struct FGoomBindingGroupInfo {
    pub ren_root_count: i32,
    pub ren_lod_count: i32,
    pub sim_root_count: i32,
    pub sim_lod_count: i32,
}
#[repr(C, align(4))]
pub struct FGroomAnimationInfo {
    pub num_frames: u32,
    pub seconds_per_frame: f32,
    pub duration: f32,
    pub start_time: f32,
    pub end_time: f32,
    pub start_frame: i32,
    pub end_frame: i32,
    pub attributes: EGroomCacheAttributes,
}
#[repr(C, align(4))]
pub struct FGroomCacheInfo {
    pub version: i32,
    pub ty: EGroomCacheType,
    pub animation_info: FGroomAnimationInfo,
}
#[repr(C, align(8))]
pub struct FFollicleMaskOptions {
    pub groom: UPtr<UGroomAsset>,
    pub channel: EFollicleMaskChannel,
}
#[repr(C, align(4))]
pub struct FGroomHairGroupPreview {
    pub group_index: i32,
    pub group_name: FName,
    pub group_id: i32,
    pub curve_count: i32,
    pub guide_count: i32,
    pub attributes: u32,
    pub attribute_flags: u32,
    pub flags: u32,
    pub interpolation_settings: FHairGroupsInterpolation,
}
#[repr(C, align(4))]
pub struct FGroomBuildSettings {
    pub b_override_guides: bool,
    pub hair_to_guide_density: f32,
    pub interpolation_quality: EGroomInterpolationQuality,
    pub interpolation_distance: EGroomInterpolationWeight,
    pub b_randomize_guide: bool,
    pub b_use_unique_guide: bool,
}
#[repr(C, align(8))]
pub struct FMovieSceneGroomCacheParams {
    pub groom_cache: UPtr<UGroomCache>,
    pub first_loop_start_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub start_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub end_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub play_rate: f32,
    pub flags_24: u8,
}
#[repr(C, align(8))]
pub struct FMovieSceneGroomCacheSectionTemplateParameters {
    pub section_start_time: crate::bindings::core_u_object::FFrameNumber,
    pub section_end_time: crate::bindings::core_u_object::FFrameNumber,
}
#[repr(C, align(8))]
pub struct FMovieSceneGroomCacheSectionTemplate {
    pub params: FMovieSceneGroomCacheSectionTemplateParameters,
}
pub struct UGroomCacheImportOptions {
    pub import_settings: FGroomCacheImportSettings,
}
pub struct UGroomCacheImportData {
    pub settings: FGroomCacheImportSettings,
}
pub struct AGroomActor {
    pub groom_component: UPtr<UGroomComponent>,
    pub sprite_component: UPtr<crate::bindings::engine::UBillboardComponent>,
}
pub struct UGroomAsset {
    pub lod_mode: EGroomLODMode,
    pub auto_lod_bias: f32,
    pub dataflow_settings: FGroomDataflowSettings,
    pub hair_groups_info: TArray<FHairGroupInfoWithVisibility>,
    pub hair_groups_rendering: TArray<FHairGroupsRendering>,
    pub hair_groups_physics: TArray<FHairGroupsPhysics>,
    pub hair_groups_interpolation: TArray<FHairGroupsInterpolation>,
    pub hair_groups_lod: TArray<FHairGroupsLOD>,
    pub hair_groups_cards: TArray<FHairGroupsCardsSourceDescription>,
    pub hair_groups_meshes: TArray<FHairGroupsMeshesSourceDescription>,
    pub hair_groups_materials: TArray<FHairGroupsMaterial>,
    pub enable_global_interpolation: bool,
    pub enable_simulation_cache: bool,
    pub hair_interpolation_type: EGroomInterpolationType,
    pub rigged_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub deformed_group_sections: TArray<i32>,
    pub min_lod: crate::bindings::core_u_object::FPerPlatformInt,
    pub disable_below_min_lod_stripping: crate::bindings::core_u_object::FPerPlatformBool,
    pub effective_lod_bias: TArray<f32>,
    pub thumbnail_info: UPtr<crate::bindings::engine::UThumbnailInfo>,
    pub asset_import_data: UPtr<crate::bindings::engine::UAssetImportData>,
    pub asset_user_data: TArray<UPtr<crate::bindings::engine::UAssetUserData>>,
    pub hair_description_type: EHairDescriptionType,
}
pub struct UHairCardGenerationSettings {}
pub struct UDataflowGroomContent {
    pub binding_asset: UPtr<UGroomBindingAsset>,
}
pub struct UGroomAssetImportData {
    pub import_options: UPtr<UGroomImportOptions>,
    pub hair_strands_textures_options: UPtr<UGroomCreateStrandsTexturesOptions>,
}
pub struct UGroomBindingAsset {
    pub groom_binding_type: EGroomBindingMeshType,
    pub groom: UPtr<UGroomAsset>,
    pub source_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub source_mesh_requested_lod: i32,
    pub source_mesh_used_lod: i32,
    pub target_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub target_mesh_requested_min_lod: i32,
    pub target_mesh_used_min_lod: i32,
    pub source_geometry_cache: UPtr<crate::bindings::geometry_cache::UGeometryCache>,
    pub target_geometry_cache: UPtr<crate::bindings::geometry_cache::UGeometryCache>,
    pub num_interpolation_points: i32,
    pub matching_section: i32,
    pub target_binding_attribute: FName,
    pub group_infos: TArray<FGoomBindingGroupInfo>,
    pub thumbnail_info: UPtr<crate::bindings::engine::UThumbnailInfo>,
}
pub struct UGroomBindingAssetList {
    pub bindings: TArray<UPtr<UGroomBindingAsset>>,
}
pub struct UGroomBlueprintLibrary {}
pub struct UGroomCache {
    pub asset_import_data: UPtr<crate::bindings::engine::UAssetImportData>,
    pub asset_user_data: TArray<UPtr<crate::bindings::engine::UAssetUserData>>,
    pub groom_cache_info: FGroomCacheInfo,
}
pub struct UGroomComponent {
    pub groom_asset: UPtr<UGroomAsset>,
    pub groom_cache: UPtr<UGroomCache>,
    pub niagara_components: TArray<UPtr<crate::bindings::niagara::UNiagaraComponent>>,
    pub source_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub binding_asset: UPtr<UGroomBindingAsset>,
    pub physics_asset: UPtr<crate::bindings::engine::UPhysicsAsset>,
    pub simulation_settings: FHairSimulationSettings,
    pub mesh_deformer: UPtr<crate::bindings::engine::UMeshDeformer>,
    pub mesh_deformer_instance: UPtr<crate::bindings::engine::UMeshDeformerInstance>,
    pub mesh_deformer_instance_settings: UPtr<
        crate::bindings::engine::UMeshDeformerInstanceSettings,
    >,
    pub strands_debug_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub strands_default_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub cards_default_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub meshes_default_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub angular_springs_system: UPtr<crate::bindings::niagara::UNiagaraSystem>,
    pub cosserat_rods_system: UPtr<crate::bindings::niagara::UNiagaraSystem>,
    pub attachment_name: FString,
    pub groom_groups_desc: TArray<FHairGroupDesc>,
    pub b_use_cards: bool,
    pub b_running: bool,
    pub b_looping: bool,
    pub b_manual_tick: bool,
    pub elapsed_time: f32,
    pub groom_asset_being_loaded: UPtr<UGroomAsset>,
    pub binding_asset_being_loaded: UPtr<UGroomBindingAsset>,
}
pub struct UGroomCreateBindingOptions {
    pub groom_asset: TWeakObjectPtr<UGroomAsset>,
    pub groom_binding_type: EGroomBindingMeshType,
    pub source_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub target_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub source_geometry_cache: UPtr<crate::bindings::geometry_cache::UGeometryCache>,
    pub target_geometry_cache: UPtr<crate::bindings::geometry_cache::UGeometryCache>,
    pub num_interpolation_points: i32,
    pub matching_section: i32,
    pub target_binding_attribute: FName,
}
pub struct UGroomCreateFollicleMaskOptions {
    pub resolution: i32,
    pub root_radius: i32,
    pub grooms: TArray<FFollicleMaskOptions>,
}
pub struct UGroomCreateStrandsTexturesOptions {
    pub layout: EHairTextureLayout,
    pub resolution: i32,
    pub trace_type: EStrandsTexturesTraceType,
    pub trace_distance: f32,
    pub mesh_type: EStrandsTexturesMeshType,
    pub static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub lod_index: i32,
    pub section_index: i32,
    pub uv_channel_index: i32,
    pub group_index: TArray<i32>,
    pub dilation: i32,
    pub generated_textures: TArray<UPtr<crate::bindings::engine::UTexture2D>>,
}
pub struct UGroomImportOptions {
    pub conversion_settings: FGroomConversionSettings,
    pub interpolation_settings: TArray<FHairGroupsInterpolation>,
}
pub struct UGroomHairGroupsPreview {
    pub groups: TArray<FGroomHairGroupPreview>,
}
pub struct UGroomHairGroupsMapping {
    pub old_group_names: TArray<FName>,
    pub new_group_names: TArray<FName>,
    pub old_to_new_group_index_mapping: TArray<i32>,
    pub new_to_old_group_index_mapping: TArray<i32>,
}
pub struct UGroomPluginSettings {
    pub groom_cache_look_ahead_buffer: f32,
}
pub struct UMovieSceneGroomCacheSection {
    pub params: FMovieSceneGroomCacheParams,
}
pub struct UMovieSceneGroomCacheTrack {
    pub animation_sections: TArray<
        UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    >,
}
pub struct UNiagaraDataInterfaceHairStrands {
    pub default_source: UPtr<UGroomAsset>,
    pub source_actor: UPtr<crate::bindings::engine::AActor>,
}
pub struct UNiagaraDataInterfaceVelocityGrid {
    pub grid_size: crate::bindings::core_u_object::FIntVector,
}
pub struct UNiagaraDataInterfacePressureGrid {}
pub struct FBuild_CompletionDelegate;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroomCacheImportType(pub u8);
impl EGroomCacheImportType {
    pub const NONE: EGroomCacheImportType = EGroomCacheImportType(0);
    pub const STRANDS: EGroomCacheImportType = EGroomCacheImportType(1);
    pub const GUIDES: EGroomCacheImportType = EGroomCacheImportType(2);
    pub const ALL: EGroomCacheImportType = EGroomCacheImportType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EHairTextureLayout(pub u8);
impl EHairTextureLayout {
    pub const LAYOUT0: EHairTextureLayout = EHairTextureLayout(0);
    pub const LAYOUT1: EHairTextureLayout = EHairTextureLayout(1);
    pub const LAYOUT2: EHairTextureLayout = EHairTextureLayout(2);
    pub const LAYOUT3: EHairTextureLayout = EHairTextureLayout(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EHairCardsSourceType(pub u8);
impl EHairCardsSourceType {
    pub const PROCEDURAL: EHairCardsSourceType = EHairCardsSourceType(0);
    pub const IMPORTED: EHairCardsSourceType = EHairCardsSourceType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EHairCardsGuideType(pub u8);
impl EHairCardsGuideType {
    pub const GENERATED: EHairCardsGuideType = EHairCardsGuideType(0);
    pub const GUIDE_BASED: EHairCardsGuideType = EHairCardsGuideType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroomGeometryType(pub u8);
impl EGroomGeometryType {
    pub const STRANDS: EGroomGeometryType = EGroomGeometryType(0);
    pub const CARDS: EGroomGeometryType = EGroomGeometryType(1);
    pub const MESHES: EGroomGeometryType = EGroomGeometryType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroomBindingType(pub u8);
impl EGroomBindingType {
    pub const NONE_BINDING: EGroomBindingType = EGroomBindingType(0);
    pub const RIGID: EGroomBindingType = EGroomBindingType(1);
    pub const SKINNING: EGroomBindingType = EGroomBindingType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroomOverrideType(pub u8);
impl EGroomOverrideType {
    pub const AUTO: EGroomOverrideType = EGroomOverrideType(0);
    pub const ENABLE: EGroomOverrideType = EGroomOverrideType(1);
    pub const DISABLE: EGroomOverrideType = EGroomOverrideType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroomGuideType(pub u8);
impl EGroomGuideType {
    pub const IMPORTED: EGroomGuideType = EGroomGuideType(0);
    pub const GENERATED: EGroomGuideType = EGroomGuideType(1);
    pub const RIGGED: EGroomGuideType = EGroomGuideType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EHairInterpolationQuality(pub u8);
impl EHairInterpolationQuality {
    pub const LOW: EHairInterpolationQuality = EHairInterpolationQuality(0);
    pub const MEDIUM: EHairInterpolationQuality = EHairInterpolationQuality(1);
    pub const HIGH: EHairInterpolationQuality = EHairInterpolationQuality(2);
    pub const UNKNOWN: EHairInterpolationQuality = EHairInterpolationQuality(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EHairInterpolationWeight(pub u8);
impl EHairInterpolationWeight {
    pub const PARAMETRIC: EHairInterpolationWeight = EHairInterpolationWeight(0);
    pub const ROOT: EHairInterpolationWeight = EHairInterpolationWeight(1);
    pub const INDEX: EHairInterpolationWeight = EHairInterpolationWeight(2);
    pub const DISTANCE: EHairInterpolationWeight = EHairInterpolationWeight(3);
    pub const UNKNOWN: EHairInterpolationWeight = EHairInterpolationWeight(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroomNiagaraSolvers(pub u8);
impl EGroomNiagaraSolvers {
    pub const NONE: EGroomNiagaraSolvers = EGroomNiagaraSolvers(0);
    pub const COSSERAT_RODS: EGroomNiagaraSolvers = EGroomNiagaraSolvers(2);
    pub const ANGULAR_SPRINGS: EGroomNiagaraSolvers = EGroomNiagaraSolvers(4);
    pub const CUSTOM_SOLVER: EGroomNiagaraSolvers = EGroomNiagaraSolvers(8);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroomStrandsSize(pub u8);
impl EGroomStrandsSize {
    pub const NONE: EGroomStrandsSize = EGroomStrandsSize(0);
    pub const SIZE2: EGroomStrandsSize = EGroomStrandsSize(2);
    pub const SIZE4: EGroomStrandsSize = EGroomStrandsSize(4);
    pub const SIZE8: EGroomStrandsSize = EGroomStrandsSize(8);
    pub const SIZE16: EGroomStrandsSize = EGroomStrandsSize(16);
    pub const SIZE32: EGroomStrandsSize = EGroomStrandsSize(32);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroomCacheAttributes(pub u8);
impl EGroomCacheAttributes {
    pub const NONE: EGroomCacheAttributes = EGroomCacheAttributes(0);
    pub const POSITION: EGroomCacheAttributes = EGroomCacheAttributes(1);
    pub const WIDTH: EGroomCacheAttributes = EGroomCacheAttributes(2);
    pub const COLOR: EGroomCacheAttributes = EGroomCacheAttributes(4);
    pub const POSITION_WIDTH: EGroomCacheAttributes = EGroomCacheAttributes(3);
    pub const POSITION_COLOR: EGroomCacheAttributes = EGroomCacheAttributes(5);
    pub const WIDTH_COLOR: EGroomCacheAttributes = EGroomCacheAttributes(6);
    pub const POSITION_WIDTH_COLOR: EGroomCacheAttributes = EGroomCacheAttributes(7);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroomCacheType(pub u8);
impl EGroomCacheType {
    pub const NONE: EGroomCacheType = EGroomCacheType(0);
    pub const STRANDS: EGroomCacheType = EGroomCacheType(1);
    pub const GUIDES: EGroomCacheType = EGroomCacheType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFollicleMaskChannel(pub u8);
impl EFollicleMaskChannel {
    pub const R: EFollicleMaskChannel = EFollicleMaskChannel(0);
    pub const G: EFollicleMaskChannel = EFollicleMaskChannel(1);
    pub const B: EFollicleMaskChannel = EFollicleMaskChannel(2);
    pub const A: EFollicleMaskChannel = EFollicleMaskChannel(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroomInterpolationQuality(pub u8);
impl EGroomInterpolationQuality {
    pub const LOW: EGroomInterpolationQuality = EGroomInterpolationQuality(0);
    pub const MEDIUM: EGroomInterpolationQuality = EGroomInterpolationQuality(1);
    pub const HIGH: EGroomInterpolationQuality = EGroomInterpolationQuality(2);
    pub const UNKNOWN: EGroomInterpolationQuality = EGroomInterpolationQuality(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroomInterpolationWeight(pub u8);
impl EGroomInterpolationWeight {
    pub const PARAMETRIC: EGroomInterpolationWeight = EGroomInterpolationWeight(0);
    pub const ROOT: EGroomInterpolationWeight = EGroomInterpolationWeight(1);
    pub const INDEX: EGroomInterpolationWeight = EGroomInterpolationWeight(2);
    pub const UNKNOWN: EGroomInterpolationWeight = EGroomInterpolationWeight(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroomInterpolationType(pub u8);
impl EGroomInterpolationType {
    pub const NONE: EGroomInterpolationType = EGroomInterpolationType(0);
    pub const RIGID_TRANSFORM: EGroomInterpolationType = EGroomInterpolationType(2);
    pub const OFFSET_TRANSFORM: EGroomInterpolationType = EGroomInterpolationType(4);
    pub const SMOOTH_TRANSFORM: EGroomInterpolationType = EGroomInterpolationType(8);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroomBindingAssetBuildResult(pub u8);
impl EGroomBindingAssetBuildResult {
    pub const SUCCEEDED: EGroomBindingAssetBuildResult = EGroomBindingAssetBuildResult(
        0,
    );
    pub const FAILED: EGroomBindingAssetBuildResult = EGroomBindingAssetBuildResult(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroomBindingMeshType(pub u8);
impl EGroomBindingMeshType {
    pub const SKELETAL_MESH: EGroomBindingMeshType = EGroomBindingMeshType(0);
    pub const GEOMETRY_CACHE: EGroomBindingMeshType = EGroomBindingMeshType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroomLODMode(pub u8);
impl EGroomLODMode {
    pub const DEFAULT: EGroomLODMode = EGroomLODMode(0);
    pub const MANUAL: EGroomLODMode = EGroomLODMode(1);
    pub const AUTO: EGroomLODMode = EGroomLODMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EHairDescriptionType(pub u8);
impl EHairDescriptionType {
    pub const SOURCE: EHairDescriptionType = EHairDescriptionType(0);
    pub const EDIT: EHairDescriptionType = EHairDescriptionType(1);
    pub const COUNT: EHairDescriptionType = EHairDescriptionType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStrandsTexturesTraceType(pub u8);
impl EStrandsTexturesTraceType {
    pub const TRACE_INSIDE: EStrandsTexturesTraceType = EStrandsTexturesTraceType(0);
    pub const TRACE_OUSIDE: EStrandsTexturesTraceType = EStrandsTexturesTraceType(1);
    pub const TRACE_BIDIRECTIONAL: EStrandsTexturesTraceType = EStrandsTexturesTraceType(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStrandsTexturesMeshType(pub u8);
impl EStrandsTexturesMeshType {
    pub const STATIC: EStrandsTexturesMeshType = EStrandsTexturesMeshType(0);
    pub const SKELETAL: EStrandsTexturesMeshType = EStrandsTexturesMeshType(1);
}
