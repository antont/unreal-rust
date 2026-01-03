#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FGrassVariety {
    pub grass_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub override_materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub grass_density: crate::bindings::core_u_object::FPerPlatformFloat,
    pub grass_density_quality: crate::bindings::engine::FPerQualityLevelFloat,
    pub b_use_grid: bool,
    pub placement_jitter: f32,
    pub start_cull_distance: crate::bindings::core_u_object::FPerPlatformInt,
    pub start_cull_distance_quality: crate::bindings::engine::FPerQualityLevelInt,
    pub end_cull_distance: crate::bindings::core_u_object::FPerPlatformInt,
    pub end_cull_distance_quality: crate::bindings::engine::FPerQualityLevelInt,
    pub min_lod: i32,
    pub allowed_density_range: crate::bindings::core_u_object::FFloatInterval,
    pub scaling: EGrassScaling,
    pub scale_x: crate::bindings::core_u_object::FFloatInterval,
    pub scale_y: crate::bindings::core_u_object::FFloatInterval,
    pub scale_z: crate::bindings::core_u_object::FFloatInterval,
    pub b_weight_attenuates_max_scale: bool,
    pub max_scale_weight_attenuation: f32,
    pub random_rotation: bool,
    pub align_to_surface: bool,
    pub b_align_to_triangle_normals: bool,
    pub b_use_landscape_lightmap: bool,
    pub lighting_channels: crate::bindings::engine::FLightingChannels,
    pub b_receives_decals: bool,
    pub b_affect_distance_field_lighting: bool,
    pub b_cast_dynamic_shadow: bool,
    pub b_cast_contact_shadow: bool,
    pub b_keep_instance_buffer_cpu_copy: bool,
    #[doc(hidden)]
    __padding_744: [u8; 6],
    pub shadow_cache_invalidation_behavior: crate::bindings::engine::EShadowCacheInvalidationBehavior,
    __padding_end: [u8; 7],
}
impl FGrassVariety {}
#[repr(C, align(16))]
pub struct FLandscapeBrushParameters {
    pub render_area_world_transform: crate::bindings::core_u_object::FTransform,
    pub render_area_size: crate::bindings::core_u_object::FIntPoint,
    pub combined_result: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
    pub layer_type: ELandscapeToolTargetType,
    pub weightmap_layer_name: FName,
}
impl FLandscapeBrushParameters {}
#[repr(C, align(16))]
pub struct UControlPointMeshComponent {
    __padding_end: [u8; 1904],
}
impl UControlPointMeshComponent {}
pub struct ILandscapeBrushRenderCallAdapter_GlobalMergeLegacySupport_DEPRECATED {}
#[repr(C, align(8))]
pub struct ULandscapeBrushRenderCallAdapter_GlobalMergeLegacySupport_DEPRECATED {
    __padding_end: [u8; 48],
}
impl ULandscapeBrushRenderCallAdapter_GlobalMergeLegacySupport_DEPRECATED {}
pub struct ILandscapeSplineInterface {}
#[repr(C, align(8))]
pub struct ULandscapeSplineInterface {
    __padding_end: [u8; 48],
}
impl ULandscapeSplineInterface {}
pub struct ILandscapeEditLayerRenderer {}
#[repr(C, align(8))]
pub struct ULandscapeEditLayerRenderer {
    __padding_end: [u8; 48],
}
impl ULandscapeEditLayerRenderer {}
#[repr(C, align(8))]
pub struct ALandscapeGizmoActor {
    __padding_end: [u8; 1176],
}
impl ALandscapeGizmoActor {}
#[repr(C, align(8))]
pub struct ALandscapeGizmoActiveActor {
    __padding_end: [u8; 1608],
}
impl ALandscapeGizmoActiveActor {}
#[repr(C, align(16))]
pub struct ULandscapeGizmoRenderComponent {
    __padding_end: [u8; 1504],
}
impl ULandscapeGizmoRenderComponent {}
#[repr(C, align(8))]
pub struct ULandscapeGrassType {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub grass_varieties: TArray<FGrassVariety>,
    pub flags_64: u8,
    __padding_end: [u8; 39],
}
impl ULandscapeGrassType {}
#[repr(C, align(8))]
pub struct ALandscapeProxy {
    #[doc(hidden)]
    __padding_1272: [u8; 1272],
    pub b_enable_nanite: bool,
    #[doc(hidden)]
    __padding_1872: [u8; 592],
    pub landscape_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    #[doc(hidden)]
    __padding_1928: [u8; 48],
    pub runtime_virtual_textures: TArray<
        UPtr<crate::bindings::engine::URuntimeVirtualTexture>,
    >,
    #[doc(hidden)]
    __padding_1945: [u8; 1],
    pub b_virtual_texture_render_with_quad: bool,
    pub b_virtual_texture_render_with_quad_hq: bool,
    pub virtual_texture_num_lods: i32,
    pub virtual_texture_lod_bias: i32,
    pub virtual_texture_render_pass_type: crate::bindings::engine::ERuntimeVirtualTextureMainPassType,
    #[doc(hidden)]
    __padding_2164: [u8; 207],
    pub flags_2164: u8,
    pub shadow_cache_invalidation_behavior: crate::bindings::engine::EShadowCacheInvalidationBehavior,
    pub flags_2166: u8,
    #[doc(hidden)]
    __padding_2168: [u8; 1],
    pub flags_2168: u8,
    #[doc(hidden)]
    __padding_2172: [u8; 3],
    pub flags_2172: u8,
    #[doc(hidden)]
    __padding_2176: [u8; 3],
    pub flags_2176: u8,
    #[doc(hidden)]
    __padding_2180: [u8; 3],
    pub flags_2180: u8,
    pub lighting_channels: crate::bindings::engine::FLightingChannels,
    #[doc(hidden)]
    __padding_2200: [u8; 18],
    pub flags_2200: u8,
    #[doc(hidden)]
    __padding_2204: [u8; 3],
    pub flags_2204: u8,
    #[doc(hidden)]
    __padding_2208: [u8; 3],
    pub custom_depth_stencil_write_mask: crate::bindings::engine::ERendererStencilMask,
    pub custom_depth_stencil_value: i32,
    pub ld_max_draw_distance: f32,
    #[doc(hidden)]
    __padding_2256: [u8; 32],
    pub body_instance: crate::bindings::engine::FBodyInstance,
    pub flags_2688: u8,
    __padding_end: [u8; 655],
}
impl ALandscapeProxy {}
#[repr(C, align(16))]
pub struct ULandscapeHeightfieldCollisionComponent {
    __padding_end: [u8; 2096],
}
impl ULandscapeHeightfieldCollisionComponent {}
#[repr(C, align(8))]
pub struct ULandscapeMaterialInstanceConstant {
    __padding_end: [u8; 1872],
}
impl ULandscapeMaterialInstanceConstant {}
#[repr(C, align(16))]
pub struct ULandscapeMeshCollisionComponent_DEPRECATED {
    __padding_end: [u8; 2208],
}
impl ULandscapeMeshCollisionComponent_DEPRECATED {}
#[repr(C, align(8))]
pub struct ALandscapeMeshProxyActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub landscape_mesh_proxy_component: UPtr<ULandscapeMeshProxyComponent>,
}
impl ALandscapeMeshProxyActor {}
#[repr(C, align(16))]
pub struct ULandscapeMeshProxyComponent {
    __padding_end: [u8; 2000],
}
impl ULandscapeMeshProxyComponent {}
#[repr(C, align(16))]
pub struct ULandscapeSplinesComponent {
    __padding_end: [u8; 1824],
}
impl ULandscapeSplinesComponent {}
#[repr(C, align(8))]
pub struct ULandscapeSplineControlPoint {
    #[doc(hidden)]
    __padding_280: [u8; 280],
    pub body_instance: crate::bindings::engine::FBodyInstance,
    __padding_end: [u8; 168],
}
impl ULandscapeSplineControlPoint {}
#[repr(C, align(8))]
pub struct ULandscapeSplineSegment {
    #[doc(hidden)]
    __padding_200: [u8; 200],
    pub body_instance: crate::bindings::engine::FBodyInstance,
    __padding_end: [u8; 152],
}
impl ULandscapeSplineSegment {}
#[repr(C, align(8))]
pub struct ALandscapeStreamingProxy {
    __padding_end: [u8; 3496],
}
impl ALandscapeStreamingProxy {}
#[repr(C, align(8))]
pub struct ULandscapeWeightmapUsage {
    __padding_end: [u8; 96],
}
impl ULandscapeWeightmapUsage {}
#[repr(C, align(8))]
pub struct UMaterialExpressionLandscapeGrassOutput {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionLandscapeGrassOutput {}
#[repr(C, align(8))]
pub struct AControlPointMeshActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub control_point_mesh_component: UPtr<UControlPointMeshComponent>,
}
impl AControlPointMeshActor {}
#[repr(C, align(8))]
pub struct ALandscape {
    __padding_end: [u8; 3768],
}
impl ALandscape {}
#[repr(C, align(16))]
pub struct ALandscapeBlueprintBrushBase {
    #[doc(hidden)]
    __padding_1152: [u8; 1152],
    pub update_on_property_change: bool,
    pub affect_heightmap: bool,
    pub affect_weightmap: bool,
    pub affect_visibility_layer: bool,
    pub affected_weightmap_layers: TArray<FName>,
    pub b_use_power_of_two_render_target: bool,
    __padding_end: [u8; 135],
}
impl ALandscapeBlueprintBrushBase {}
#[repr(C, align(8))]
pub struct ULandscapeLODStreamingProxy_DEPRECATED {
    __padding_end: [u8; 256],
}
impl ULandscapeLODStreamingProxy_DEPRECATED {}
#[repr(C, align(16))]
pub struct ULandscapeComponent {
    #[doc(hidden)]
    __padding_1504: [u8; 1504],
    pub section_base_x: i32,
    pub section_base_y: i32,
    #[doc(hidden)]
    __padding_1528: [u8; 16],
    pub override_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub override_hole_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    #[doc(hidden)]
    __padding_2172: [u8; 628],
    pub forced_lod: i32,
    pub lod_bias: i32,
    __padding_end: [u8; 364],
}
impl ULandscapeComponent {}
#[repr(C, align(8))]
pub struct ULandscapeHeightmapTextureEdgeFixup {
    __padding_end: [u8; 152],
}
impl ULandscapeHeightmapTextureEdgeFixup {}
#[repr(C, align(8))]
pub struct ULandscapeEditLayerBase {
    __padding_end: [u8; 216],
}
impl ULandscapeEditLayerBase {}
#[repr(C, align(8))]
pub struct ULandscapeEditLayerPersistent {
    __padding_end: [u8; 224],
}
impl ULandscapeEditLayerPersistent {}
#[repr(C, align(8))]
pub struct ULandscapeEditLayer {
    __padding_end: [u8; 224],
}
impl ULandscapeEditLayer {}
#[repr(C, align(8))]
pub struct ULandscapeEditLayerProcedural {
    __padding_end: [u8; 216],
}
impl ULandscapeEditLayerProcedural {}
#[repr(C, align(8))]
pub struct ULandscapeEditLayerSplines {
    __padding_end: [u8; 224],
}
impl ULandscapeEditLayerSplines {}
#[repr(C, align(8))]
pub struct ULandscapeDefaultEditLayerRenderer {
    __padding_end: [u8; 56],
}
impl ULandscapeDefaultEditLayerRenderer {}
#[repr(C, align(8))]
pub struct ULandscapeHeightmapNormalsEditLayerRenderer {
    __padding_end: [u8; 56],
}
impl ULandscapeHeightmapNormalsEditLayerRenderer {}
#[repr(C, align(8))]
pub struct ULandscapeWeightmapWeightBlendedLayersRenderer {
    __padding_end: [u8; 56],
}
impl ULandscapeWeightmapWeightBlendedLayersRenderer {}
#[repr(C, align(8))]
pub struct ULandscapeScratchRenderTarget {
    __padding_end: [u8; 128],
}
impl ULandscapeScratchRenderTarget {}
#[repr(C, align(8))]
pub struct ULandscapeEditResourcesSubsystem {
    __padding_end: [u8; 136],
}
impl ULandscapeEditResourcesSubsystem {}
#[repr(C, align(8))]
pub struct ULandscapeHLODBuilder {
    __padding_end: [u8; 72],
}
impl ULandscapeHLODBuilder {}
#[repr(C, align(8))]
pub struct ULandscapeInfo {
    __padding_end: [u8; 848],
}
impl ULandscapeInfo {}
#[repr(C, align(8))]
pub struct ULandscapeInfoMap {
    __padding_end: [u8; 136],
}
impl ULandscapeInfoMap {}
#[repr(C, align(8))]
pub struct ULandscapeLayerInfoObject {
    __padding_end: [u8; 184],
}
impl ULandscapeLayerInfoObject {}
#[repr(C, align(16))]
pub struct ULandscapeNaniteComponent {
    __padding_end: [u8; 1936],
}
impl ULandscapeNaniteComponent {}
#[repr(C, align(8))]
pub struct ULandscapeSettings {
    __padding_end: [u8; 264],
}
impl ULandscapeSettings {}
#[repr(C, align(8))]
pub struct ALandscapeSplineActor {
    __padding_end: [u8; 1168],
}
impl ALandscapeSplineActor {}
#[repr(C, align(8))]
pub struct ALandscapeSplineMeshesActor {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub static_mesh_components: TArray<
        UPtr<crate::bindings::engine::UStaticMeshComponent>,
    >,
    __padding_end: [u8; 16],
}
impl ALandscapeSplineMeshesActor {}
#[repr(C, align(8))]
pub struct ULandscapeSubsystem {
    __padding_end: [u8; 496],
}
impl ULandscapeSubsystem {}
#[repr(C, align(8))]
pub struct ULandscapeTextureHash {
    __padding_end: [u8; 168],
}
impl ULandscapeTextureHash {}
#[repr(C, align(8))]
pub struct ULandscapeTextureMipEdgeOverrideFactory {
    __padding_end: [u8; 64],
}
impl ULandscapeTextureMipEdgeOverrideFactory {}
#[repr(C, align(8))]
pub struct ULandscapeTextureStorageProviderFactory {
    __padding_end: [u8; 112],
}
impl ULandscapeTextureStorageProviderFactory {}
#[repr(C, align(8))]
pub struct UMaterialExpressionLandscapeLayerBlend {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionLandscapeLayerBlend {}
#[repr(C, align(8))]
pub struct UMaterialExpressionLandscapeLayerCoords {
    __padding_end: [u8; 224],
}
impl UMaterialExpressionLandscapeLayerCoords {}
#[repr(C, align(8))]
pub struct UMaterialExpressionLandscapeLayerSample {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionLandscapeLayerSample {}
#[repr(C, align(8))]
pub struct UMaterialExpressionLandscapeLayerSwitch {
    __padding_end: [u8; 312],
}
impl UMaterialExpressionLandscapeLayerSwitch {}
#[repr(C, align(8))]
pub struct UMaterialExpressionLandscapeLayerWeight {
    __padding_end: [u8; 336],
}
impl UMaterialExpressionLandscapeLayerWeight {}
#[repr(C, align(8))]
pub struct UMaterialExpressionLandscapePhysicalMaterialOutput {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionLandscapePhysicalMaterialOutput {}
#[repr(C, align(8))]
pub struct UMaterialExpressionLandscapeVisibilityMask {
    __padding_end: [u8; 200],
}
impl UMaterialExpressionLandscapeVisibilityMask {}
#[repr(transparent)]
pub struct EGrassScaling(pub u8);
impl EGrassScaling {
    pub const UNIFORM: EGrassScaling = EGrassScaling(0);
    pub const FREE: EGrassScaling = EGrassScaling(1);
    pub const LOCK_XY: EGrassScaling = EGrassScaling(2);
}
#[repr(transparent)]
pub struct LandscapeSplineMeshOrientation(pub u8);
impl LandscapeSplineMeshOrientation {
    pub const LSMO_X_UP: LandscapeSplineMeshOrientation = LandscapeSplineMeshOrientation(
        0,
    );
    pub const LSMO_Y_UP: LandscapeSplineMeshOrientation = LandscapeSplineMeshOrientation(
        1,
    );
}
#[repr(transparent)]
pub struct ELandscapeBlendMode(pub u8);
impl ELandscapeBlendMode {
    pub const LSBM_ADDITIVE_BLEND: ELandscapeBlendMode = ELandscapeBlendMode(0);
    pub const LSBM_ALPHA_BLEND: ELandscapeBlendMode = ELandscapeBlendMode(1);
}
#[repr(transparent)]
pub struct ELandscapeToolTargetType(pub u8);
impl ELandscapeToolTargetType {
    pub const HEIGHTMAP: ELandscapeToolTargetType = ELandscapeToolTargetType(0);
    pub const WEIGHTMAP: ELandscapeToolTargetType = ELandscapeToolTargetType(1);
    pub const VISIBILITY: ELandscapeToolTargetType = ELandscapeToolTargetType(2);
    pub const INVALID: ELandscapeToolTargetType = ELandscapeToolTargetType(3);
    pub const COUNT: ELandscapeToolTargetType = ELandscapeToolTargetType(3);
}
#[repr(transparent)]
pub struct ELandscapeLayerBlendType(pub u8);
impl ELandscapeLayerBlendType {
    pub const LB_WEIGHT_BLEND: ELandscapeLayerBlendType = ELandscapeLayerBlendType(0);
    pub const LB_ALPHA_BLEND: ELandscapeLayerBlendType = ELandscapeLayerBlendType(1);
    pub const LB_HEIGHT_BLEND: ELandscapeLayerBlendType = ELandscapeLayerBlendType(2);
}
#[repr(transparent)]
pub struct ELandscapeGizmoType(pub u8);
impl ELandscapeGizmoType {
    pub const LGT_NONE: ELandscapeGizmoType = ELandscapeGizmoType(0);
    pub const LGT_HEIGHT: ELandscapeGizmoType = ELandscapeGizmoType(1);
    pub const LGT_WEIGHT: ELandscapeGizmoType = ELandscapeGizmoType(2);
}
#[repr(transparent)]
pub struct ELandscapeGizmoSnapType(pub i32);
impl ELandscapeGizmoSnapType {
    pub const NONE: ELandscapeGizmoSnapType = ELandscapeGizmoSnapType(0);
    pub const COMPONENT: ELandscapeGizmoSnapType = ELandscapeGizmoSnapType(1);
    pub const TEXEL: ELandscapeGizmoSnapType = ELandscapeGizmoSnapType(2);
}
#[repr(transparent)]
pub struct ELandscapeLODFalloff(pub u8);
impl ELandscapeLODFalloff {
    pub const LINEAR: ELandscapeLODFalloff = ELandscapeLODFalloff(0);
    pub const SQUARE_ROOT: ELandscapeLODFalloff = ELandscapeLODFalloff(1);
}
#[repr(transparent)]
pub struct ELandscapeHLODTextureSizePolicy(pub u8);
impl ELandscapeHLODTextureSizePolicy {
    pub const AUTOMATIC_SIZE: ELandscapeHLODTextureSizePolicy = ELandscapeHLODTextureSizePolicy(
        0,
    );
    pub const SPECIFIC_SIZE: ELandscapeHLODTextureSizePolicy = ELandscapeHLODTextureSizePolicy(
        1,
    );
}
#[repr(transparent)]
pub struct ELandscapeHLODMeshSourceLODPolicy(pub u8);
impl ELandscapeHLODMeshSourceLODPolicy {
    pub const AUTOMATIC_LOD: ELandscapeHLODMeshSourceLODPolicy = ELandscapeHLODMeshSourceLODPolicy(
        0,
    );
    pub const SPECIFIC_LOD: ELandscapeHLODMeshSourceLODPolicy = ELandscapeHLODMeshSourceLODPolicy(
        1,
    );
    pub const LOWEST_DETAIL_LOD: ELandscapeHLODMeshSourceLODPolicy = ELandscapeHLODMeshSourceLODPolicy(
        2,
    );
}
#[repr(transparent)]
pub struct ELandscapeLayerDisplayMode(pub u8);
impl ELandscapeLayerDisplayMode {
    pub const DEFAULT: ELandscapeLayerDisplayMode = ELandscapeLayerDisplayMode(0);
    pub const ALPHABETICAL: ELandscapeLayerDisplayMode = ELandscapeLayerDisplayMode(1);
    pub const USER_SPECIFIC: ELandscapeLayerDisplayMode = ELandscapeLayerDisplayMode(2);
    pub const BY_BLEND_METHOD: ELandscapeLayerDisplayMode = ELandscapeLayerDisplayMode(
        3,
    );
}
#[repr(transparent)]
pub struct ESplineModulationColorMask(pub u8);
impl ESplineModulationColorMask {
    pub const RED: ESplineModulationColorMask = ESplineModulationColorMask(0);
    pub const GREEN: ESplineModulationColorMask = ESplineModulationColorMask(1);
    pub const BLUE: ESplineModulationColorMask = ESplineModulationColorMask(2);
    pub const ALPHA: ESplineModulationColorMask = ESplineModulationColorMask(3);
}
#[repr(transparent)]
pub struct ELandscapeTargetLayerBlendMethod(pub u8);
impl ELandscapeTargetLayerBlendMethod {
    pub const NONE: ELandscapeTargetLayerBlendMethod = ELandscapeTargetLayerBlendMethod(
        0,
    );
    pub const FINAL_WEIGHT_BLENDING: ELandscapeTargetLayerBlendMethod = ELandscapeTargetLayerBlendMethod(
        1,
    );
    pub const PREMULTIPLIED_ALPHA_BLENDING: ELandscapeTargetLayerBlendMethod = ELandscapeTargetLayerBlendMethod(
        2,
    );
    pub const COUNT: ELandscapeTargetLayerBlendMethod = ELandscapeTargetLayerBlendMethod(
        3,
    );
}
#[repr(transparent)]
pub struct ELandscapeDirtyingMode(pub u8);
impl ELandscapeDirtyingMode {
    pub const AUTO: ELandscapeDirtyingMode = ELandscapeDirtyingMode(0);
    pub const IN_LANDSCAPE_MODE_ONLY: ELandscapeDirtyingMode = ELandscapeDirtyingMode(1);
    pub const IN_LANDSCAPE_MODE_AND_USER_TRIGGERED_CHANGES: ELandscapeDirtyingMode = ELandscapeDirtyingMode(
        2,
    );
}
#[repr(transparent)]
pub struct ELandscapeTextureType(pub u8);
impl ELandscapeTextureType {
    pub const UNKNOWN: ELandscapeTextureType = ELandscapeTextureType(0);
    pub const HEIGHTMAP: ELandscapeTextureType = ELandscapeTextureType(1);
    pub const WEIGHTMAP: ELandscapeTextureType = ELandscapeTextureType(2);
}
#[repr(transparent)]
pub struct ELandscapeTextureUsage(pub u8);
impl ELandscapeTextureUsage {
    pub const UNKNOWN: ELandscapeTextureUsage = ELandscapeTextureUsage(0);
    pub const EDIT_LAYER_DATA: ELandscapeTextureUsage = ELandscapeTextureUsage(1);
    pub const FINAL_DATA: ELandscapeTextureUsage = ELandscapeTextureUsage(2);
}
#[repr(transparent)]
pub struct ETerrainCoordMappingType(pub u8);
impl ETerrainCoordMappingType {
    pub const TCMT_AUTO: ETerrainCoordMappingType = ETerrainCoordMappingType(0);
    pub const TCMT_XY: ETerrainCoordMappingType = ETerrainCoordMappingType(1);
    pub const TCMT_XZ: ETerrainCoordMappingType = ETerrainCoordMappingType(2);
    pub const TCMT_YZ: ETerrainCoordMappingType = ETerrainCoordMappingType(3);
}
#[repr(transparent)]
pub struct ELandscapeCustomizedCoordType(pub u8);
impl ELandscapeCustomizedCoordType {
    pub const LCCT_NONE: ELandscapeCustomizedCoordType = ELandscapeCustomizedCoordType(
        0,
    );
    pub const LCCT_CUSTOM_UV0: ELandscapeCustomizedCoordType = ELandscapeCustomizedCoordType(
        1,
    );
    pub const LCCT_CUSTOM_UV1: ELandscapeCustomizedCoordType = ELandscapeCustomizedCoordType(
        2,
    );
    pub const LCCT_CUSTOM_UV2: ELandscapeCustomizedCoordType = ELandscapeCustomizedCoordType(
        3,
    );
    pub const LCCT_WEIGHT_MAP_UV: ELandscapeCustomizedCoordType = ELandscapeCustomizedCoordType(
        4,
    );
}
#[repr(transparent)]
pub struct ELandscapeImportAlphamapType(pub u8);
impl ELandscapeImportAlphamapType {
    pub const ADDITIVE: ELandscapeImportAlphamapType = ELandscapeImportAlphamapType(0);
    pub const LAYERED: ELandscapeImportAlphamapType = ELandscapeImportAlphamapType(1);
}
#[repr(transparent)]
pub struct ELandscapeLayerPaintingRestriction(pub u8);
impl ELandscapeLayerPaintingRestriction {
    pub const NONE: ELandscapeLayerPaintingRestriction = ELandscapeLayerPaintingRestriction(
        0,
    );
    pub const USE_MAX_LAYERS: ELandscapeLayerPaintingRestriction = ELandscapeLayerPaintingRestriction(
        1,
    );
    pub const EXISTING_ONLY: ELandscapeLayerPaintingRestriction = ELandscapeLayerPaintingRestriction(
        2,
    );
    pub const USE_COMPONENT_ALLOW_LIST: ELandscapeLayerPaintingRestriction = ELandscapeLayerPaintingRestriction(
        3,
    );
}
