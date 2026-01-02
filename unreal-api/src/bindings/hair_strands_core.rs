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
impl FGroomCacheImportSettings {}
#[repr(C, align(8))]
pub struct FGroomConversionSettings {
    pub rotation: crate::bindings::core_u_object::FVector,
    pub scale: crate::bindings::core_u_object::FVector,
}
impl FGroomConversionSettings {}
#[repr(C, align(4))]
pub struct FHairGroupDesc {
    #[doc(hidden)]
    __padding_4: [u8; 4],
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
    #[doc(hidden)]
    __padding_56: [u8; 4],
    pub hair_length_scale: f32,
    pub hair_length_scale_override: bool,
    __padding_end: [u8; 3],
}
impl FHairGroupDesc {}
#[repr(C, align(4))]
pub struct FHairGroupLODInfo {
    __padding_end: [u8; 8],
}
impl FHairGroupLODInfo {}
#[repr(C, align(8))]
pub struct FHairGroupInfo {
    __padding_end: [u8; 64],
}
impl FHairGroupInfo {}
#[repr(C, align(8))]
pub struct FHairGroupsMaterial {
    __padding_end: [u8; 24],
}
impl FHairGroupsMaterial {}
#[repr(C, align(8))]
pub struct FHairGroupInfoWithVisibility {
    __padding_end: [u8; 72],
}
impl FHairGroupInfoWithVisibility {}
#[repr(C, align(4))]
pub struct FHairGroupCardsInfo {
    __padding_end: [u8; 8],
}
impl FHairGroupCardsInfo {}
#[repr(C, align(8))]
pub struct FHairGroupCardsTextures {
    __padding_end: [u8; 80],
}
impl FHairGroupCardsTextures {}
#[repr(C, align(8))]
pub struct FHairGroupsCardsSourceDescription {
    __padding_end: [u8; 168],
}
impl FHairGroupsCardsSourceDescription {}
#[repr(C, align(4))]
pub struct FHairLODSettings {
    __padding_end: [u8; 28],
}
impl FHairLODSettings {}
#[repr(C, align(4))]
pub struct FHairDecimationSettings {
    __padding_end: [u8; 8],
}
impl FHairDecimationSettings {}
#[repr(C, align(4))]
pub struct FHairInterpolationSettings {
    pub guide_type: EGroomGuideType,
    pub hair_to_guide_density: f32,
    pub rigged_guide_num_curves: i32,
    pub rigged_guide_num_points: i32,
    pub interpolation_quality: EHairInterpolationQuality,
    pub interpolation_distance: EHairInterpolationWeight,
    pub b_randomize_guide: bool,
    pub b_use_unique_guide: bool,
}
impl FHairInterpolationSettings {}
#[repr(C, align(4))]
pub struct FHairDeformationSettings {
    __padding_end: [u8; 12],
}
impl FHairDeformationSettings {}
#[repr(C, align(4))]
pub struct FHairGroupsInterpolation {
    __padding_end: [u8; 52],
}
impl FHairGroupsInterpolation {}
#[repr(C, align(8))]
pub struct FHairGroupsLOD {
    __padding_end: [u8; 24],
}
impl FHairGroupsLOD {}
#[repr(C, align(8))]
pub struct FHairGroupsMeshesSourceDescription {
    __padding_end: [u8; 136],
}
impl FHairGroupsMeshesSourceDescription {}
#[repr(C, align(8))]
pub struct FHairSolverSettings {
    __padding_end: [u8; 72],
}
impl FHairSolverSettings {}
#[repr(C, align(8))]
pub struct FHairExternalForces {
    __padding_end: [u8; 56],
}
impl FHairExternalForces {}
#[repr(C, align(8))]
pub struct FHairBendConstraint {
    __padding_end: [u8; 152],
}
impl FHairBendConstraint {}
#[repr(C, align(8))]
pub struct FHairStretchConstraint {
    __padding_end: [u8; 152],
}
impl FHairStretchConstraint {}
#[repr(C, align(8))]
pub struct FHairCollisionConstraint {
    __padding_end: [u8; 168],
}
impl FHairCollisionConstraint {}
#[repr(C, align(8))]
pub struct FHairMaterialConstraints {
    __padding_end: [u8; 472],
}
impl FHairMaterialConstraints {}
#[repr(C, align(8))]
pub struct FHairStrandsParameters {
    __padding_end: [u8; 152],
}
impl FHairStrandsParameters {}
#[repr(C, align(8))]
pub struct FHairGroupsPhysics {
    __padding_end: [u8; 768],
}
impl FHairGroupsPhysics {}
#[repr(C, align(1))]
pub struct FHairSimulationSolver {
    pub b_enable_simulation: bool,
}
impl FHairSimulationSolver {}
#[repr(C, align(8))]
pub struct FHairSimulationForces {
    pub gravity_vector: crate::bindings::core_u_object::FVector,
    pub air_drag: f32,
    pub air_velocity: crate::bindings::core_u_object::FVector,
}
impl FHairSimulationForces {}
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
impl FHairSimulationConstraints {}
#[repr(C, align(8))]
pub struct FHairSimulationSetup {
    pub b_reset_simulation: bool,
    pub b_debug_simulation: bool,
    pub b_local_simulation: bool,
    pub linear_velocity_scale: f32,
    pub angular_velocity_scale: f32,
    pub local_bone: FString,
    pub teleport_distance: f32,
    __padding_end: [u8; 4],
}
impl FHairSimulationSetup {}
#[repr(C, align(8))]
pub struct FHairSimulationSettings {
    pub b_override_settings: bool,
    pub simulation_setup: FHairSimulationSetup,
    pub solver_settings: FHairSimulationSolver,
    pub external_forces: FHairSimulationForces,
    pub material_constraints: FHairSimulationConstraints,
}
impl FHairSimulationSettings {}
#[repr(C, align(4))]
pub struct FHairGeometrySettings {
    pub hair_width: f32,
    pub hair_width_override: bool,
    pub hair_root_scale: f32,
    pub hair_tip_scale: f32,
}
impl FHairGeometrySettings {}
#[repr(C, align(4))]
pub struct FHairShadowSettings {
    pub hair_shadow_density: f32,
    pub hair_raytracing_radius_scale: f32,
    pub b_use_hair_raytracing_geometry: bool,
    pub b_voxelize: bool,
    __padding_end: [u8; 2],
}
impl FHairShadowSettings {}
#[repr(C, align(1))]
pub struct FHairAdvancedRenderingSettings {
    pub b_use_stable_rasterization: bool,
    pub b_scatter_scene_lighting: bool,
}
impl FHairAdvancedRenderingSettings {}
#[repr(C, align(8))]
pub struct FHairGroupsRendering {
    __padding_end: [u8; 72],
}
impl FHairGroupsRendering {}
#[repr(C, align(4))]
pub struct FGoomBindingGroupInfo {
    __padding_end: [u8; 16],
}
impl FGoomBindingGroupInfo {}
#[repr(C, align(8))]
pub struct FFollicleMaskOptions {
    pub groom: UPtr<UGroomAsset>,
    pub channel: EFollicleMaskChannel,
    __padding_end: [u8; 7],
}
impl FFollicleMaskOptions {}
#[repr(C, align(4))]
pub struct FGroomHairGroupPreview {
    pub group_index: i32,
    pub group_name: FName,
    pub group_id: i32,
    pub curve_count: i32,
    pub guide_count: i32,
    #[doc(hidden)]
    __padding_40: [u8; 12],
    pub interpolation_settings: FHairGroupsInterpolation,
}
impl FGroomHairGroupPreview {}
#[repr(C, align(4))]
pub struct FGroomBuildSettings {
    pub b_override_guides: bool,
    pub hair_to_guide_density: f32,
    pub interpolation_quality: EGroomInterpolationQuality,
    pub interpolation_distance: EGroomInterpolationWeight,
    pub b_randomize_guide: bool,
    pub b_use_unique_guide: bool,
}
impl FGroomBuildSettings {}
#[repr(C, align(8))]
pub struct UGroomCacheImportOptions {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub import_settings: FGroomCacheImportSettings,
}
impl UGroomCacheImportOptions {}
#[repr(C, align(8))]
pub struct UGroomCacheImportData {
    __padding_end: [u8; 208],
}
impl UGroomCacheImportData {}
#[repr(C, align(8))]
pub struct AGroomActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub groom_component: UPtr<UGroomComponent>,
    __padding_end: [u8; 8],
}
impl AGroomActor {}
#[repr(C, align(16))]
pub struct UGroomAsset {
    #[doc(hidden)]
    __padding_352: [u8; 352],
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
    __padding_end: [u8; 672],
}
impl UGroomAsset {}
#[repr(C, align(8))]
pub struct UHairCardGenerationSettings {
    __padding_end: [u8; 48],
}
impl UHairCardGenerationSettings {}
#[repr(C, align(8))]
pub struct UDataflowGroomContent {
    __padding_end: [u8; 240],
}
impl UDataflowGroomContent {}
#[repr(C, align(8))]
pub struct UGroomAssetImportData {
    __padding_end: [u8; 112],
}
impl UGroomAssetImportData {}
#[repr(C, align(16))]
pub struct UGroomBindingAsset {
    #[doc(hidden)]
    __padding_56: [u8; 56],
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
    __padding_end: [u8; 176],
}
impl UGroomBindingAsset {}
#[repr(C, align(8))]
pub struct UGroomBindingAssetList {
    __padding_end: [u8; 64],
}
impl UGroomBindingAssetList {}
#[repr(C, align(8))]
pub struct UGroomBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UGroomBlueprintLibrary {}
#[repr(C, align(8))]
pub struct UGroomCache {
    __padding_end: [u8; 160],
}
impl UGroomCache {}
#[repr(C, align(16))]
pub struct UGroomComponent {
    #[doc(hidden)]
    __padding_1592: [u8; 1592],
    pub groom_asset: UPtr<UGroomAsset>,
    pub groom_cache: UPtr<UGroomCache>,
    #[doc(hidden)]
    __padding_1632: [u8; 24],
    pub binding_asset: UPtr<UGroomBindingAsset>,
    pub physics_asset: UPtr<crate::bindings::engine::UPhysicsAsset>,
    #[doc(hidden)]
    __padding_1664: [u8; 16],
    pub simulation_settings: FHairSimulationSettings,
    pub mesh_deformer: UPtr<crate::bindings::engine::UMeshDeformer>,
    pub mesh_deformer_instance: UPtr<crate::bindings::engine::UMeshDeformerInstance>,
    pub mesh_deformer_instance_settings: UPtr<
        crate::bindings::engine::UMeshDeformerInstanceSettings,
    >,
    #[doc(hidden)]
    __padding_1880: [u8; 48],
    pub attachment_name: FString,
    #[doc(hidden)]
    __padding_2032: [u8; 136],
    pub groom_groups_desc: TArray<FHairGroupDesc>,
    pub b_use_cards: bool,
    __padding_end: [u8; 143],
}
impl UGroomComponent {}
#[repr(C, align(8))]
pub struct UGroomCreateBindingOptions {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub groom_binding_type: EGroomBindingMeshType,
    pub source_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub target_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub source_geometry_cache: UPtr<crate::bindings::geometry_cache::UGeometryCache>,
    pub target_geometry_cache: UPtr<crate::bindings::geometry_cache::UGeometryCache>,
    pub num_interpolation_points: i32,
    pub matching_section: i32,
    pub target_binding_attribute: FName,
    __padding_end: [u8; 4],
}
impl UGroomCreateBindingOptions {}
#[repr(C, align(8))]
pub struct UGroomCreateFollicleMaskOptions {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub resolution: i32,
    pub root_radius: i32,
    pub grooms: TArray<FFollicleMaskOptions>,
}
impl UGroomCreateFollicleMaskOptions {}
#[repr(C, align(8))]
pub struct UGroomCreateStrandsTexturesOptions {
    #[doc(hidden)]
    __padding_48: [u8; 48],
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
impl UGroomCreateStrandsTexturesOptions {}
#[repr(C, align(8))]
pub struct UGroomImportOptions {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub conversion_settings: FGroomConversionSettings,
    pub interpolation_settings: TArray<FHairGroupsInterpolation>,
}
impl UGroomImportOptions {}
#[repr(C, align(8))]
pub struct UGroomHairGroupsPreview {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub groups: TArray<FGroomHairGroupPreview>,
}
impl UGroomHairGroupsPreview {}
#[repr(C, align(8))]
pub struct UGroomHairGroupsMapping {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub old_group_names: TArray<FName>,
    pub new_group_names: TArray<FName>,
    pub old_to_new_group_index_mapping: TArray<i32>,
    pub new_to_old_group_index_mapping: TArray<i32>,
    __padding_end: [u8; 16],
}
impl UGroomHairGroupsMapping {}
#[repr(C, align(8))]
pub struct UGroomPluginSettings {
    __padding_end: [u8; 56],
}
impl UGroomPluginSettings {}
#[repr(C, align(8))]
pub struct UMovieSceneGroomCacheSection {
    __padding_end: [u8; 400],
}
impl UMovieSceneGroomCacheSection {}
#[repr(C, align(8))]
pub struct UMovieSceneGroomCacheTrack {
    __padding_end: [u8; 408],
}
impl UMovieSceneGroomCacheTrack {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceHairStrands {
    __padding_end: [u8; 184],
}
impl UNiagaraDataInterfaceHairStrands {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceVelocityGrid {
    __padding_end: [u8; 168],
}
impl UNiagaraDataInterfaceVelocityGrid {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfacePressureGrid {
    __padding_end: [u8; 168],
}
impl UNiagaraDataInterfacePressureGrid {}
#[repr(transparent)]
pub struct FBuild_CompletionDelegate {
    _opague: u8,
}
#[repr(transparent)]
pub struct EGroomCacheImportType(pub u8);
impl EGroomCacheImportType {
    pub const NONE: EGroomCacheImportType = EGroomCacheImportType(0);
    pub const STRANDS: EGroomCacheImportType = EGroomCacheImportType(1);
    pub const GUIDES: EGroomCacheImportType = EGroomCacheImportType(2);
    pub const ALL: EGroomCacheImportType = EGroomCacheImportType(3);
}
#[repr(transparent)]
pub struct EHairTextureLayout(pub u8);
impl EHairTextureLayout {
    pub const LAYOUT0: EHairTextureLayout = EHairTextureLayout(0);
    pub const LAYOUT1: EHairTextureLayout = EHairTextureLayout(1);
    pub const LAYOUT2: EHairTextureLayout = EHairTextureLayout(2);
    pub const LAYOUT3: EHairTextureLayout = EHairTextureLayout(3);
}
#[repr(transparent)]
pub struct EHairCardsSourceType(pub u8);
impl EHairCardsSourceType {
    pub const PROCEDURAL: EHairCardsSourceType = EHairCardsSourceType(0);
    pub const IMPORTED: EHairCardsSourceType = EHairCardsSourceType(1);
}
#[repr(transparent)]
pub struct EHairCardsGuideType(pub u8);
impl EHairCardsGuideType {
    pub const GENERATED: EHairCardsGuideType = EHairCardsGuideType(0);
    pub const GUIDE_BASED: EHairCardsGuideType = EHairCardsGuideType(1);
}
#[repr(transparent)]
pub struct EGroomGeometryType(pub u8);
impl EGroomGeometryType {
    pub const STRANDS: EGroomGeometryType = EGroomGeometryType(0);
    pub const CARDS: EGroomGeometryType = EGroomGeometryType(1);
    pub const MESHES: EGroomGeometryType = EGroomGeometryType(2);
}
#[repr(transparent)]
pub struct EGroomBindingType(pub u8);
impl EGroomBindingType {
    pub const NONE_BINDING: EGroomBindingType = EGroomBindingType(0);
    pub const RIGID: EGroomBindingType = EGroomBindingType(1);
    pub const SKINNING: EGroomBindingType = EGroomBindingType(2);
}
#[repr(transparent)]
pub struct EGroomOverrideType(pub u8);
impl EGroomOverrideType {
    pub const AUTO: EGroomOverrideType = EGroomOverrideType(0);
    pub const ENABLE: EGroomOverrideType = EGroomOverrideType(1);
    pub const DISABLE: EGroomOverrideType = EGroomOverrideType(2);
}
#[repr(transparent)]
pub struct EGroomGuideType(pub u8);
impl EGroomGuideType {
    pub const IMPORTED: EGroomGuideType = EGroomGuideType(0);
    pub const GENERATED: EGroomGuideType = EGroomGuideType(1);
    pub const RIGGED: EGroomGuideType = EGroomGuideType(2);
}
#[repr(transparent)]
pub struct EHairInterpolationQuality(pub u8);
impl EHairInterpolationQuality {
    pub const LOW: EHairInterpolationQuality = EHairInterpolationQuality(0);
    pub const MEDIUM: EHairInterpolationQuality = EHairInterpolationQuality(1);
    pub const HIGH: EHairInterpolationQuality = EHairInterpolationQuality(2);
    pub const UNKNOWN: EHairInterpolationQuality = EHairInterpolationQuality(3);
}
#[repr(transparent)]
pub struct EHairInterpolationWeight(pub u8);
impl EHairInterpolationWeight {
    pub const PARAMETRIC: EHairInterpolationWeight = EHairInterpolationWeight(0);
    pub const ROOT: EHairInterpolationWeight = EHairInterpolationWeight(1);
    pub const INDEX: EHairInterpolationWeight = EHairInterpolationWeight(2);
    pub const DISTANCE: EHairInterpolationWeight = EHairInterpolationWeight(3);
    pub const UNKNOWN: EHairInterpolationWeight = EHairInterpolationWeight(4);
}
#[repr(transparent)]
pub struct EGroomNiagaraSolvers(pub u8);
impl EGroomNiagaraSolvers {
    pub const NONE: EGroomNiagaraSolvers = EGroomNiagaraSolvers(0);
    pub const COSSERAT_RODS: EGroomNiagaraSolvers = EGroomNiagaraSolvers(2);
    pub const ANGULAR_SPRINGS: EGroomNiagaraSolvers = EGroomNiagaraSolvers(4);
    pub const CUSTOM_SOLVER: EGroomNiagaraSolvers = EGroomNiagaraSolvers(8);
}
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
#[repr(transparent)]
pub struct EGroomCacheType(pub u8);
impl EGroomCacheType {
    pub const NONE: EGroomCacheType = EGroomCacheType(0);
    pub const STRANDS: EGroomCacheType = EGroomCacheType(1);
    pub const GUIDES: EGroomCacheType = EGroomCacheType(2);
}
#[repr(transparent)]
pub struct EFollicleMaskChannel(pub u8);
impl EFollicleMaskChannel {
    pub const R: EFollicleMaskChannel = EFollicleMaskChannel(0);
    pub const G: EFollicleMaskChannel = EFollicleMaskChannel(1);
    pub const B: EFollicleMaskChannel = EFollicleMaskChannel(2);
    pub const A: EFollicleMaskChannel = EFollicleMaskChannel(3);
}
#[repr(transparent)]
pub struct EGroomInterpolationQuality(pub u8);
impl EGroomInterpolationQuality {
    pub const LOW: EGroomInterpolationQuality = EGroomInterpolationQuality(0);
    pub const MEDIUM: EGroomInterpolationQuality = EGroomInterpolationQuality(1);
    pub const HIGH: EGroomInterpolationQuality = EGroomInterpolationQuality(2);
    pub const UNKNOWN: EGroomInterpolationQuality = EGroomInterpolationQuality(3);
}
#[repr(transparent)]
pub struct EGroomInterpolationWeight(pub u8);
impl EGroomInterpolationWeight {
    pub const PARAMETRIC: EGroomInterpolationWeight = EGroomInterpolationWeight(0);
    pub const ROOT: EGroomInterpolationWeight = EGroomInterpolationWeight(1);
    pub const INDEX: EGroomInterpolationWeight = EGroomInterpolationWeight(2);
    pub const UNKNOWN: EGroomInterpolationWeight = EGroomInterpolationWeight(3);
}
#[repr(transparent)]
pub struct EGroomInterpolationType(pub u8);
impl EGroomInterpolationType {
    pub const NONE: EGroomInterpolationType = EGroomInterpolationType(0);
    pub const RIGID_TRANSFORM: EGroomInterpolationType = EGroomInterpolationType(2);
    pub const OFFSET_TRANSFORM: EGroomInterpolationType = EGroomInterpolationType(4);
    pub const SMOOTH_TRANSFORM: EGroomInterpolationType = EGroomInterpolationType(8);
}
#[repr(transparent)]
pub struct EGroomBindingAssetBuildResult(pub u8);
impl EGroomBindingAssetBuildResult {
    pub const SUCCEEDED: EGroomBindingAssetBuildResult = EGroomBindingAssetBuildResult(
        0,
    );
    pub const FAILED: EGroomBindingAssetBuildResult = EGroomBindingAssetBuildResult(1);
}
#[repr(transparent)]
pub struct EGroomBindingMeshType(pub u8);
impl EGroomBindingMeshType {
    pub const SKELETAL_MESH: EGroomBindingMeshType = EGroomBindingMeshType(0);
    pub const GEOMETRY_CACHE: EGroomBindingMeshType = EGroomBindingMeshType(1);
}
#[repr(transparent)]
pub struct EGroomLODMode(pub u8);
impl EGroomLODMode {
    pub const DEFAULT: EGroomLODMode = EGroomLODMode(0);
    pub const MANUAL: EGroomLODMode = EGroomLODMode(1);
    pub const AUTO: EGroomLODMode = EGroomLODMode(2);
}
#[repr(transparent)]
pub struct EHairDescriptionType(pub u8);
impl EHairDescriptionType {
    pub const SOURCE: EHairDescriptionType = EHairDescriptionType(0);
    pub const EDIT: EHairDescriptionType = EHairDescriptionType(1);
    pub const COUNT: EHairDescriptionType = EHairDescriptionType(2);
}
#[repr(transparent)]
pub struct EStrandsTexturesTraceType(pub u8);
impl EStrandsTexturesTraceType {
    pub const TRACE_INSIDE: EStrandsTexturesTraceType = EStrandsTexturesTraceType(0);
    pub const TRACE_OUSIDE: EStrandsTexturesTraceType = EStrandsTexturesTraceType(1);
    pub const TRACE_BIDIRECTIONAL: EStrandsTexturesTraceType = EStrandsTexturesTraceType(
        2,
    );
}
#[repr(transparent)]
pub struct EStrandsTexturesMeshType(pub u8);
impl EStrandsTexturesMeshType {
    pub const STATIC: EStrandsTexturesMeshType = EStrandsTexturesMeshType(0);
    pub const SKELETAL: EStrandsTexturesMeshType = EStrandsTexturesMeshType(1);
}
