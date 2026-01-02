#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FInterchangeAnimationPayLoadKey {
    pub unique_id: FString,
    pub ty: EInterchangeAnimationPayLoadType,
    __padding_end: [u8; 7],
}
impl FInterchangeAnimationPayLoadKey {}
#[repr(C, align(8))]
pub struct FInterchangeGroomPayloadKey {
    pub unique_id: FString,
    pub ty: EInterchangeGroomPayLoadType,
    pub frame_number: i32,
}
impl FInterchangeGroomPayloadKey {}
#[repr(C, align(8))]
pub struct FInterchangeMeshPayLoadKey {
    pub unique_id: FString,
    pub ty: EInterchangeMeshPayLoadType,
    pub frame_number: i32,
}
impl FInterchangeMeshPayLoadKey {}
#[repr(C, align(8))]
pub struct UInterchangePhysicalCameraNode {
    __padding_end: [u8; 176],
}
impl UInterchangePhysicalCameraNode {}
#[repr(C, align(8))]
pub struct UInterchangeStandardCameraNode {
    __padding_end: [u8; 208],
}
impl UInterchangeStandardCameraNode {}
#[repr(C, align(8))]
pub struct UInterchangeShaderNode {
    __padding_end: [u8; 128],
}
impl UInterchangeShaderNode {}
#[repr(C, align(8))]
pub struct UInterchangeDecalMaterialNode {
    __padding_end: [u8; 160],
}
impl UInterchangeDecalMaterialNode {}
#[repr(C, align(8))]
pub struct UInterchangeDecalNode {
    __padding_end: [u8; 160],
}
impl UInterchangeDecalNode {}
#[repr(C, align(8))]
pub struct UInterchangeTextureNode {
    __padding_end: [u8; 176],
}
impl UInterchangeTextureNode {}
#[repr(C, align(8))]
pub struct UInterchangeTexture2DArrayNode {
    __padding_end: [u8; 176],
}
impl UInterchangeTexture2DArrayNode {}
#[repr(C, align(8))]
pub struct UInterchangeTextureCubeArrayNode {
    __padding_end: [u8; 176],
}
impl UInterchangeTextureCubeArrayNode {}
#[repr(C, align(8))]
pub struct UInterchangeTextureCubeNode {
    __padding_end: [u8; 176],
}
impl UInterchangeTextureCubeNode {}
#[repr(C, align(8))]
pub struct UInterchangeTextureLightProfileNode {
    __padding_end: [u8; 176],
}
impl UInterchangeTextureLightProfileNode {}
#[repr(C, align(8))]
pub struct UInterchangeVolumeTextureNode {
    __padding_end: [u8; 176],
}
impl UInterchangeVolumeTextureNode {}
#[repr(C, align(8))]
pub struct UInterchangeAnimationTrackSetNode {
    __padding_end: [u8; 160],
}
impl UInterchangeAnimationTrackSetNode {}
#[repr(C, align(8))]
pub struct UInterchangeAnimationTrackBaseNode {
    __padding_end: [u8; 128],
}
impl UInterchangeAnimationTrackBaseNode {}
#[repr(C, align(8))]
pub struct UInterchangeAnimationTrackSetInstanceNode {
    __padding_end: [u8; 192],
}
impl UInterchangeAnimationTrackSetInstanceNode {}
#[repr(C, align(8))]
pub struct UInterchangeAnimationTrackNode {
    __padding_end: [u8; 224],
}
impl UInterchangeAnimationTrackNode {}
#[repr(C, align(8))]
pub struct UInterchangeTransformAnimationTrackNode {
    __padding_end: [u8; 240],
}
impl UInterchangeTransformAnimationTrackNode {}
#[repr(C, align(8))]
pub struct UInterchangeSkeletalAnimationTrackNode {
    __padding_end: [u8; 704],
}
impl UInterchangeSkeletalAnimationTrackNode {}
#[repr(C, align(8))]
pub struct UInterchangeAudioSoundWaveNode {
    __padding_end: [u8; 112],
}
impl UInterchangeAudioSoundWaveNode {}
#[repr(C, align(8))]
pub struct UInterchangeGroomNode {
    __padding_end: [u8; 192],
}
impl UInterchangeGroomNode {}
#[repr(C, align(8))]
pub struct UInterchangeBaseLightNode {
    __padding_end: [u8; 176],
}
impl UInterchangeBaseLightNode {}
#[repr(C, align(8))]
pub struct UInterchangeLightNode {
    __padding_end: [u8; 272],
}
impl UInterchangeLightNode {}
#[repr(C, align(8))]
pub struct UInterchangePointLightNode {
    __padding_end: [u8; 304],
}
impl UInterchangePointLightNode {}
#[repr(C, align(8))]
pub struct UInterchangeSpotLightNode {
    __padding_end: [u8; 336],
}
impl UInterchangeSpotLightNode {}
#[repr(C, align(8))]
pub struct UInterchangeRectLightNode {
    __padding_end: [u8; 304],
}
impl UInterchangeRectLightNode {}
#[repr(C, align(8))]
pub struct UInterchangeDirectionalLightNode {
    __padding_end: [u8; 176],
}
impl UInterchangeDirectionalLightNode {}
#[repr(C, align(8))]
pub struct UInterchangeSkyLightNode {
    __padding_end: [u8; 208],
}
impl UInterchangeSkyLightNode {}
#[repr(C, align(8))]
pub struct UInterchangeMaterialInstanceNode {
    __padding_end: [u8; 144],
}
impl UInterchangeMaterialInstanceNode {}
#[repr(C, align(8))]
pub struct UInterchangeMaterialReferenceNode {
    __padding_end: [u8; 128],
}
impl UInterchangeMaterialReferenceNode {}
#[repr(C, align(8))]
pub struct UInterchangeMeshLODContainerNode {
    __padding_end: [u8; 144],
}
impl UInterchangeMeshLODContainerNode {}
#[repr(C, align(8))]
pub struct UInterchangeMeshNode {
    __padding_end: [u8; 552],
}
impl UInterchangeMeshNode {}
#[repr(C, align(8))]
pub struct UInterchangeGeometryCacheNode {
    __padding_end: [u8; 616],
}
impl UInterchangeGeometryCacheNode {}
#[repr(C, align(16))]
pub struct UInterchangeSceneComponentNode {
    __padding_end: [u8; 304],
}
impl UInterchangeSceneComponentNode {}
#[repr(C, align(16))]
pub struct UInterchangeInstancedStaticMeshComponentNode {
    __padding_end: [u8; 352],
}
impl UInterchangeInstancedStaticMeshComponentNode {}
#[repr(C, align(16))]
pub struct UInterchangeSceneNode {
    __padding_end: [u8; 1248],
}
impl UInterchangeSceneNode {}
#[repr(C, align(8))]
pub struct UInterchangeShaderPortsAPI {
    __padding_end: [u8; 48],
}
impl UInterchangeShaderPortsAPI {}
#[repr(C, align(8))]
pub struct UInterchangeFunctionCallShaderNode {
    __padding_end: [u8; 144],
}
impl UInterchangeFunctionCallShaderNode {}
#[repr(C, align(8))]
pub struct UInterchangeShaderGraphNode {
    __padding_end: [u8; 240],
}
impl UInterchangeShaderGraphNode {}
#[repr(C, align(8))]
pub struct UInterchangeSpecularProfileNode {
    __padding_end: [u8; 144],
}
impl UInterchangeSpecularProfileNode {}
#[repr(C, align(8))]
pub struct UInterchangeTexture2DNode {
    __padding_end: [u8; 344],
}
impl UInterchangeTexture2DNode {}
#[repr(C, align(8))]
pub struct UInterchangeTextureBlurNode {
    __padding_end: [u8; 344],
}
impl UInterchangeTextureBlurNode {}
#[repr(C, align(8))]
pub struct UInterchangeVariantSetNode {
    __padding_end: [u8; 176],
}
impl UInterchangeVariantSetNode {}
#[repr(C, align(8))]
pub struct UInterchangeSceneVariantSetsNode {
    __padding_end: [u8; 144],
}
impl UInterchangeSceneVariantSetsNode {}
#[repr(C, align(8))]
pub struct UInterchangeVolumeNode {
    __padding_end: [u8; 208],
}
impl UInterchangeVolumeNode {}
#[repr(C, align(8))]
pub struct UInterchangeVolumeGridNode {
    __padding_end: [u8; 208],
}
impl UInterchangeVolumeGridNode {}
#[repr(transparent)]
pub struct EInterchangeAnimationPayLoadType(pub u8);
impl EInterchangeAnimationPayLoadType {
    pub const NONE: EInterchangeAnimationPayLoadType = EInterchangeAnimationPayLoadType(
        0,
    );
    pub const CURVE: EInterchangeAnimationPayLoadType = EInterchangeAnimationPayLoadType(
        1,
    );
    pub const MORPHTARGETCURVE: EInterchangeAnimationPayLoadType = EInterchangeAnimationPayLoadType(
        2,
    );
    pub const STEPCURVE: EInterchangeAnimationPayLoadType = EInterchangeAnimationPayLoadType(
        3,
    );
    pub const BAKED: EInterchangeAnimationPayLoadType = EInterchangeAnimationPayLoadType(
        4,
    );
    pub const MORPHTARGETCURVEWEIGHTINSTANCE: EInterchangeAnimationPayLoadType = EInterchangeAnimationPayLoadType(
        5,
    );
    pub const GEOMETRY_CACHE_TRANSFORM: EInterchangeAnimationPayLoadType = EInterchangeAnimationPayLoadType(
        6,
    );
}
#[repr(transparent)]
pub struct EInterchangeGroomPayLoadType(pub u8);
impl EInterchangeGroomPayLoadType {
    pub const STATIC: EInterchangeGroomPayLoadType = EInterchangeGroomPayLoadType(0);
    pub const ANIMATED: EInterchangeGroomPayLoadType = EInterchangeGroomPayLoadType(1);
}
#[repr(transparent)]
pub struct EInterchangeMeshPayLoadType(pub u8);
impl EInterchangeMeshPayLoadType {
    pub const NONE: EInterchangeMeshPayLoadType = EInterchangeMeshPayLoadType(0);
    pub const STATIC: EInterchangeMeshPayLoadType = EInterchangeMeshPayLoadType(1);
    pub const SKELETAL: EInterchangeMeshPayLoadType = EInterchangeMeshPayLoadType(2);
    pub const MORPHTARGET: EInterchangeMeshPayLoadType = EInterchangeMeshPayLoadType(3);
    pub const ANIMATED: EInterchangeMeshPayLoadType = EInterchangeMeshPayLoadType(4);
}
#[repr(transparent)]
pub struct EInterchangeCameraProjectionType(pub u8);
impl EInterchangeCameraProjectionType {
    pub const PERSPECTIVE: EInterchangeCameraProjectionType = EInterchangeCameraProjectionType(
        0,
    );
    pub const ORTHOGRAPHIC: EInterchangeCameraProjectionType = EInterchangeCameraProjectionType(
        1,
    );
}
#[repr(transparent)]
pub struct EInterchangeTextureColorSpace(pub u8);
impl EInterchangeTextureColorSpace {
    pub const TCS_NONE: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(0);
    pub const TCS_S_RGB: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        1,
    );
    pub const TCS_REC2020: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        2,
    );
    pub const TCS_ACESAP0: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        3,
    );
    pub const TCS_ACESAP1: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        4,
    );
    pub const TCS_P3DCI: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        5,
    );
    pub const TCS_P3D65: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        6,
    );
    pub const TCS_RED_WIDE_GAMUT: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        7,
    );
    pub const TCS_SONY_S_GAMUT3: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        8,
    );
    pub const TCS_SONY_S_GAMUT3_CINE: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        9,
    );
    pub const TCS_ALEXA_WIDE_GAMUT: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        10,
    );
    pub const TCS_CANON_CINEMA_GAMUT: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        11,
    );
    pub const TCS_GO_PRO_PROTUNE_NATIVE: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        12,
    );
    pub const TCS_PANASONIC_V_GAMUT: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        13,
    );
    pub const TCS_CUSTOM: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        99,
    );
}
#[repr(transparent)]
pub struct EInterchangeTextureFilterMode(pub u8);
impl EInterchangeTextureFilterMode {
    pub const NEAREST: EInterchangeTextureFilterMode = EInterchangeTextureFilterMode(0);
    pub const BILINEAR: EInterchangeTextureFilterMode = EInterchangeTextureFilterMode(1);
    pub const TRILINEAR: EInterchangeTextureFilterMode = EInterchangeTextureFilterMode(
        2,
    );
    pub const DEFAULT: EInterchangeTextureFilterMode = EInterchangeTextureFilterMode(3);
}
#[repr(transparent)]
pub struct EInterchangePropertyTracks(pub i32);
impl EInterchangePropertyTracks {
    pub const AFFECT_DISTANCE_FIELD_LIGHTING: EInterchangePropertyTracks = EInterchangePropertyTracks(
        0,
    );
    pub const AFFECT_DYNAMIC_INDIRECT_LIGHTING: EInterchangePropertyTracks = EInterchangePropertyTracks(
        1,
    );
    pub const AFFECT_INDIRECT_LIGHTING_WHILE_HIDDEN: EInterchangePropertyTracks = EInterchangePropertyTracks(
        2,
    );
    pub const AUTO_ACTIVATE: EInterchangePropertyTracks = EInterchangePropertyTracks(3);
    pub const BODY_INSTANCE_ANGULAR_DAMPING: EInterchangePropertyTracks = EInterchangePropertyTracks(
        4,
    );
    pub const BODY_INSTANCEB_ENABLE_GRAVITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        5,
    );
    pub const BODY_INSTANCEB_NOTIFY_RIGID_BODY_COLLISION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        6,
    );
    pub const BODY_INSTANCEB_SIMULATE_PHYSICS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        7,
    );
    pub const BODY_INSTANCEB_UPDATE_KINEMATIC_FROM_SIMULATION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        8,
    );
    pub const BODY_INSTANCEB_USE_CCD: EInterchangePropertyTracks = EInterchangePropertyTracks(
        9,
    );
    pub const BODY_INSTANCE_LINEAR_DAMPING: EInterchangePropertyTracks = EInterchangePropertyTracks(
        10,
    );
    pub const BODY_INSTANCE_MASS_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        11,
    );
    pub const BOUNDS_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(12);
    pub const CAST_CONTACT_SHADOW: EInterchangePropertyTracks = EInterchangePropertyTracks(
        13,
    );
    pub const CAST_HIDDEN_SHADOW: EInterchangePropertyTracks = EInterchangePropertyTracks(
        14,
    );
    pub const CAST_INSET_SHADOW: EInterchangePropertyTracks = EInterchangePropertyTracks(
        15,
    );
    pub const CAST_SHADOW: EInterchangePropertyTracks = EInterchangePropertyTracks(16);
    pub const CUSTOM_DEPTH_STENCIL_VALUE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        17,
    );
    pub const CUSTOM_DEPTH_STENCIL_WRITE_MASK: EInterchangePropertyTracks = EInterchangePropertyTracks(
        18,
    );
    pub const DEFAULT_UP_VECTOR: EInterchangePropertyTracks = EInterchangePropertyTracks(
        19,
    );
    pub const DRAW_DEBUG: EInterchangePropertyTracks = EInterchangePropertyTracks(20);
    pub const EMISSIVE_LIGHT_SOURCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        21,
    );
    pub const EXCLUDE_FROM_LIGHT_ATTACHMENT_GROUP: EInterchangePropertyTracks = EInterchangePropertyTracks(
        22,
    );
    pub const HIDDEN_IN_GAME: EInterchangePropertyTracks = EInterchangePropertyTracks(
        23,
    );
    pub const HIDDEN_IN_SCENE_CAPTURE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        24,
    );
    pub const HOLDOUT: EInterchangePropertyTracks = EInterchangePropertyTracks(25);
    pub const LIGHT_ATTACHMENTS_AS_GROUP: EInterchangePropertyTracks = EInterchangePropertyTracks(
        26,
    );
    pub const MOBILITY: EInterchangePropertyTracks = EInterchangePropertyTracks(27);
    pub const ONLY_OWNER_SEE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        28,
    );
    pub const OWNER_NO_SEE: EInterchangePropertyTracks = EInterchangePropertyTracks(29);
    pub const RECEIVES_DECALS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        30,
    );
    pub const RENDER_CUSTOM_DEPTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        31,
    );
    pub const RENDER_IN_DEPTH_PASS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        32,
    );
    pub const RENDER_IN_MAIN_PASS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        33,
    );
    pub const SINGLE_SAMPLE_SHADOW_FROM_STATIONARY_LIGHTS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        34,
    );
    pub const TRANSLUCENCY_SORT_DISTANCE_OFFSET: EInterchangePropertyTracks = EInterchangePropertyTracks(
        35,
    );
    pub const VISIBLE_IN_RAY_TRACING: EInterchangePropertyTracks = EInterchangePropertyTracks(
        36,
    );
    pub const VISIBLE_IN_SCENE_CAPTURE_ONLY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        37,
    );
    pub const ACTOR_HIDDEN_IN_GAME: EInterchangePropertyTracks = EInterchangePropertyTracks(
        38,
    );
    pub const LIGHT_AFFECT_GLOBAL_ILLUMINATION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        39,
    );
    pub const LIGHT_AFFECT_REFLECTION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        40,
    );
    pub const LIGHT_AFFECT_TRANSLUCENT_LIGHTING: EInterchangePropertyTracks = EInterchangePropertyTracks(
        41,
    );
    pub const LIGHT_ATMOSPHERE_SUN_DISK_COLOR_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        42,
    );
    pub const LIGHT_ATMOSPHERE_SUN_LIGHT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        43,
    );
    pub const LIGHT_ATMOSPHERE_SUN_LIGHT_INDEX: EInterchangePropertyTracks = EInterchangePropertyTracks(
        44,
    );
    pub const LIGHT_ATTENUATION_RADIUS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        45,
    );
    pub const LIGHT_BARN_DOOR_ANGLE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        46,
    );
    pub const LIGHT_BARN_DOOR_LENGTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        47,
    );
    pub const LIGHT_BLOOM_MAX_BRIGHTNESS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        48,
    );
    pub const LIGHT_BLOOM_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        49,
    );
    pub const LIGHT_BLOOM_THRESHOLD: EInterchangePropertyTracks = EInterchangePropertyTracks(
        50,
    );
    pub const LIGHT_BLOOM_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        51,
    );
    pub const LIGHT_CASCADE_DISTRIBUTION_EXPONENT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        52,
    );
    pub const LIGHT_CASCADE_TRANSITION_FRACTION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        53,
    );
    pub const LIGHT_CAST_DEEP_SHADOW: EInterchangePropertyTracks = EInterchangePropertyTracks(
        54,
    );
    pub const LIGHT_CAST_SHADOWS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        55,
    );
    pub const LIGHT_CAST_VOLUMETRIC_SHADOW: EInterchangePropertyTracks = EInterchangePropertyTracks(
        56,
    );
    pub const LIGHT_CLOUD_AMBIENT_OCCLUSION_STRENGTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        57,
    );
    pub const LIGHT_CLOUD_SCATTERED_LUMINANCE_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        58,
    );
    pub const LIGHT_CLOUD_SHADOW_ON_ATMOSPHERE_STRENGTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        59,
    );
    pub const LIGHT_CLOUD_SHADOW_ON_SURFACE_STRENGTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        60,
    );
    pub const LIGHT_CLOUD_SHADOW_STRENGTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        61,
    );
    pub const LIGHT_COLOR: EInterchangePropertyTracks = EInterchangePropertyTracks(62);
    pub const LIGHT_DYNAMIC_SHADOW_CASCADES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        63,
    );
    pub const LIGHT_DYNAMIC_SHADOW_DISTANCE_MOVABLE_LIGHT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        64,
    );
    pub const LIGHT_DYNAMIC_SHADOW_DISTANCE_STATIONARY_LIGHT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        65,
    );
    pub const LIGHT_ENABLE_LIGHT_SHAFT_BLOOM: EInterchangePropertyTracks = EInterchangePropertyTracks(
        66,
    );
    pub const LIGHT_ENABLE_LIGHT_SHAFT_OCCLUSION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        67,
    );
    pub const LIGHT_FALLOFF_EXPONENT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        68,
    );
    pub const LIGHT_FORCE_CACHED_SHADOWS_FOR_MOVABLE_PRIMITIVES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        69,
    );
    pub const LIGHT_FORWARD_SHADING_PRIORITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        70,
    );
    pub const LIGHT_FUNCTION_FADE_DISTANCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        71,
    );
    pub const LIGHT_FUNCTION_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        72,
    );
    pub const LIGHT_IES_BRIGHTNESS_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        73,
    );
    pub const LIGHT_INDIRECT_LIGHTING_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        74,
    );
    pub const LIGHT_INNER_CONE_ANGLE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        75,
    );
    pub const LIGHT_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        76,
    );
    pub const LIGHT_INTENSITY_UNITS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        77,
    );
    pub const LIGHT_INVERSE_EXPOSURE_BLEND: EInterchangePropertyTracks = EInterchangePropertyTracks(
        78,
    );
    pub const LIGHT_LOWER_HEMISPHERE_COLOR: EInterchangePropertyTracks = EInterchangePropertyTracks(
        79,
    );
    pub const LIGHTMASS_SETTINGS_LIGHT_SOURCE_ANGLE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        80,
    );
    pub const LIGHT_MIN_OCCLUSION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        81,
    );
    pub const LIGHT_MODULATED_SHADOW_COLOR: EInterchangePropertyTracks = EInterchangePropertyTracks(
        82,
    );
    pub const LIGHT_OCCLUSION_DEPTH_RANGE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        83,
    );
    pub const LIGHT_OCCLUSION_EXPONENT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        84,
    );
    pub const LIGHT_OCCLUSION_MASK_DARKNESS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        85,
    );
    pub const LIGHT_OCCLUSION_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        86,
    );
    pub const LIGHT_OUTER_CONE_ANGLE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        87,
    );
    pub const LIGHT_SAMPLES_PER_PIXEL: EInterchangePropertyTracks = EInterchangePropertyTracks(
        88,
    );
    pub const LIGHT_SHADOW_AMOUNT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        89,
    );
    pub const LIGHT_SHADOW_BIAS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        90,
    );
    pub const LIGHT_SHADOW_CASCADE_BIAS_DISTRIBUTION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        91,
    );
    pub const LIGHT_SHADOW_DISTANCE_FADEOUT_FRACTION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        92,
    );
    pub const LIGHT_SHADOW_SLOPE_BIAS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        93,
    );
    pub const LIGHT_SHADOW_SOURCE_ANGLE_FACTOR: EInterchangePropertyTracks = EInterchangePropertyTracks(
        94,
    );
    pub const LIGHT_SHAFT_OVERRIDE_DIRECTION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        95,
    );
    pub const LIGHT_SOFT_SOURCE_RADIUS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        96,
    );
    pub const LIGHT_SOURCE_ANGLE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        97,
    );
    pub const LIGHT_SOURCE_CUBEMAP_ANGLE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        98,
    );
    pub const LIGHT_SOURCE_HEIGHT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        99,
    );
    pub const LIGHT_SOURCE_LENGTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        100,
    );
    pub const LIGHT_SOURCE_RADIUS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        101,
    );
    pub const LIGHT_SOURCE_SOFT_ANGLE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        102,
    );
    pub const LIGHT_SOURCE_WIDTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        103,
    );
    pub const LIGHT_SPECULAR_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        104,
    );
    pub const LIGHT_DIFFUSE_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        105,
    );
    pub const LIGHT_TEMPERATURE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        106,
    );
    pub const LIGHT_TRANSMISSION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        107,
    );
    pub const LIGHT_USE_IES_BRIGHTNESS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        108,
    );
    pub const LIGHT_USE_INVERSE_SQUARED_FALLOFF: EInterchangePropertyTracks = EInterchangePropertyTracks(
        109,
    );
    pub const LIGHT_USE_TEMPERATURE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        110,
    );
    pub const LIGHT_VOLUMETRIC_SCATTERING_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        111,
    );
    pub const CAMERA_ASPECT_RATIO: EInterchangePropertyTracks = EInterchangePropertyTracks(
        112,
    );
    pub const CAMERA_ASPECT_RATIO_AXIS_CONSTRAINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        113,
    );
    pub const CAMERA_AUTO_CALCULATE_ORTHO_PLANES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        114,
    );
    pub const CAMERA_AUTO_PLANE_SHIFT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        115,
    );
    pub const CAMERA_CONSTRAIN_ASPECT_RATIO: EInterchangePropertyTracks = EInterchangePropertyTracks(
        116,
    );
    pub const CAMERA_CURRENT_APERTURE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        117,
    );
    pub const CAMERA_CURRENT_FOCAL_LENGTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        118,
    );
    pub const CAMERA_CUSTOM_NEAR_CLIPPING_PLANE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        119,
    );
    pub const CAMERA_FIELD_OF_VIEW: EInterchangePropertyTracks = EInterchangePropertyTracks(
        120,
    );
    pub const CAMERA_FILMBACK_SENSOR_ASPECT_RATIO: EInterchangePropertyTracks = EInterchangePropertyTracks(
        121,
    );
    pub const CAMERA_FILMBACK_SENSOR_HEIGHT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        122,
    );
    pub const CAMERA_FILMBACK_SENSOR_WIDTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        123,
    );
    pub const CAMERA_FOCUS_SETTINGS_FOCUS_OFFSET: EInterchangePropertyTracks = EInterchangePropertyTracks(
        124,
    );
    pub const CAMERA_FOCUS_SETTINGS_MANUAL_FOCUS_DISTANCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        125,
    );
    pub const CAMERA_FOCUS_SETTINGS_TRACKING_FOCUS_SETTINGS_RELATIVE_OFFSET: EInterchangePropertyTracks = EInterchangePropertyTracks(
        126,
    );
    pub const CAMERA_ORTHO_FAR_CLIP_PLANE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        127,
    );
    pub const CAMERA_ORTHO_NEAR_CLIP_PLANE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        128,
    );
    pub const CAMERA_ORTHO_WIDTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        129,
    );
    pub const CAMERA_OVERRIDE_ASPECT_RATIO_AXIS_CONSTRAINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        130,
    );
    pub const CAMERA_POST_PROCESS_BLEND_WEIGHT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        131,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_CUBEMAP_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        132,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_CUBEMAP_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        133,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_BIAS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        134,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_FADE_DISTANCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        135,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_FADE_RADIUS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        136,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        137,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_MIP_BLEND: EInterchangePropertyTracks = EInterchangePropertyTracks(
        138,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_MIP_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        139,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_MIP_THRESHOLD: EInterchangePropertyTracks = EInterchangePropertyTracks(
        140,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_POWER: EInterchangePropertyTracks = EInterchangePropertyTracks(
        141,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_QUALITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        142,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_RADIUS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        143,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_STATIC_FRACTION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        144,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_TEMPORAL_BLEND_WEIGHT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        145,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AUTO_EXPOSURE_BIAS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        146,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AUTO_EXPOSURE_HIGH_PERCENT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        147,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AUTO_EXPOSURE_LOW_PERCENT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        148,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AUTO_EXPOSURE_MAX_BRIGHTNESS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        149,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AUTO_EXPOSURE_MIN_BRIGHTNESS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        150,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AUTO_EXPOSURE_SPEED_DOWN: EInterchangePropertyTracks = EInterchangePropertyTracks(
        151,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AUTO_EXPOSURE_SPEED_UP: EInterchangePropertyTracks = EInterchangePropertyTracks(
        152,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM1_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        153,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM1_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        154,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM2_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        155,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM2_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        156,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM3_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        157,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM3_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        158,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM4_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        159,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM4_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        160,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM5_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        161,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM5_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        162,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM6_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        163,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM6_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        164,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_CONVOLUTION_BUFFER_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        165,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_CONVOLUTION_CENTER_UV: EInterchangePropertyTracks = EInterchangePropertyTracks(
        166,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_CONVOLUTION_PRE_FILTER_MAX: EInterchangePropertyTracks = EInterchangePropertyTracks(
        167,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_CONVOLUTION_PRE_FILTER_MIN: EInterchangePropertyTracks = EInterchangePropertyTracks(
        168,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_CONVOLUTION_PRE_FILTER_MULT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        169,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_CONVOLUTION_SCATTER_DISPERSION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        170,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_CONVOLUTION_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        171,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_DIRT_MASK_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        172,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_DIRT_MASK_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        173,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        174,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_SIZE_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        175,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_THRESHOLD: EInterchangePropertyTracks = EInterchangePropertyTracks(
        176,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLUE_CORRECTION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        177,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_CHROMATIC_ABERRATION_START_OFFSET: EInterchangePropertyTracks = EInterchangePropertyTracks(
        178,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_CONTRAST: EInterchangePropertyTracks = EInterchangePropertyTracks(
        179,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_CONTRAST_HIGHLIGHTS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        180,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_CONTRAST_MIDTONES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        181,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_CONTRAST_SHADOWS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        182,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_CORRECTION_HIGHLIGHTS_MAX: EInterchangePropertyTracks = EInterchangePropertyTracks(
        183,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_CORRECTION_HIGHLIGHTS_MIN: EInterchangePropertyTracks = EInterchangePropertyTracks(
        184,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_CORRECTION_SHADOWS_MAX: EInterchangePropertyTracks = EInterchangePropertyTracks(
        185,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_GAIN: EInterchangePropertyTracks = EInterchangePropertyTracks(
        186,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_GAIN_HIGHLIGHTS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        187,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_GAIN_MIDTONES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        188,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_GAIN_SHADOWS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        189,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_GAMMA: EInterchangePropertyTracks = EInterchangePropertyTracks(
        190,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_GAMMA_HIGHLIGHTS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        191,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_GAMMA_MIDTONES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        192,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_GAMMA_SHADOWS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        193,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_GRADING_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        194,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_OFFSET: EInterchangePropertyTracks = EInterchangePropertyTracks(
        195,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_OFFSET_HIGHLIGHTS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        196,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_OFFSET_MIDTONES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        197,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_OFFSET_SHADOWS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        198,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_SATURATION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        199,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_SATURATION_HIGHLIGHTS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        200,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_SATURATION_MIDTONES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        201,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_SATURATION_SHADOWS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        202,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_BLADE_COUNT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        203,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_DEPTH_BLUR_AMOUNT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        204,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_DEPTH_BLUR_RADIUS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        205,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_FAR_BLUR_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        206,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_FAR_TRANSITION_REGION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        207,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_FOCAL_DISTANCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        208,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_FOCAL_REGION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        209,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_FSTOP: EInterchangePropertyTracks = EInterchangePropertyTracks(
        210,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_MIN_FSTOP: EInterchangePropertyTracks = EInterchangePropertyTracks(
        211,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_NEAR_BLUR_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        212,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_NEAR_TRANSITION_REGION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        213,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_OCCLUSION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        214,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        215,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_SKY_FOCUS_DISTANCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        216,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_USE_HAIR_DEPTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        217,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_VIGNETTE_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        218,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DYNAMIC_GLOBAL_ILLUMINATION_METHOD: EInterchangePropertyTracks = EInterchangePropertyTracks(
        219,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_EXPAND_GAMUT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        220,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_BLACK_CLIP: EInterchangePropertyTracks = EInterchangePropertyTracks(
        221,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_GRAIN_HIGHLIGHTS_MAX: EInterchangePropertyTracks = EInterchangePropertyTracks(
        222,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_GRAIN_HIGHLIGHTS_MIN: EInterchangePropertyTracks = EInterchangePropertyTracks(
        223,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_GRAIN_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        224,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_GRAIN_INTENSITY_HIGHLIGHTS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        225,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_GRAIN_INTENSITY_MIDTONES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        226,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_GRAIN_INTENSITY_SHADOWS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        227,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_GRAIN_SHADOWS_MAX: EInterchangePropertyTracks = EInterchangePropertyTracks(
        228,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_GRAIN_TEXEL_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        229,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_SHOULDER: EInterchangePropertyTracks = EInterchangePropertyTracks(
        230,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_SLOPE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        231,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_TOE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        232,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_WHITE_CLIP: EInterchangePropertyTracks = EInterchangePropertyTracks(
        233,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_HISTOGRAM_LOG_MAX: EInterchangePropertyTracks = EInterchangePropertyTracks(
        234,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_HISTOGRAM_LOG_MIN: EInterchangePropertyTracks = EInterchangePropertyTracks(
        235,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_INDIRECT_LIGHTING_COLOR: EInterchangePropertyTracks = EInterchangePropertyTracks(
        236,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_INDIRECT_LIGHTING_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        237,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LENS_FLARE_BOKEH_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        238,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LENS_FLARE_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        239,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LENS_FLARE_THRESHOLD: EInterchangePropertyTracks = EInterchangePropertyTracks(
        240,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LENS_FLARE_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        241,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LOCAL_EXPOSURE_BLURRED_LUMINANCE_BLEND: EInterchangePropertyTracks = EInterchangePropertyTracks(
        242,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LOCAL_EXPOSURE_BLURRED_LUMINANCE_KERNEL_SIZE_PERCENT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        243,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LOCAL_EXPOSURE_DETAIL_STRENGTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        244,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LOCAL_EXPOSURE_HIGHLIGHT_CONTRAST_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        245,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LOCAL_EXPOSURE_HIGHLIGHT_THRESHOLD: EInterchangePropertyTracks = EInterchangePropertyTracks(
        246,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LOCAL_EXPOSURE_MIDDLE_GREY_BIAS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        247,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LOCAL_EXPOSURE_SHADOW_CONTRAST_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        248,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LOCAL_EXPOSURE_SHADOW_THRESHOLD: EInterchangePropertyTracks = EInterchangePropertyTracks(
        249,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_DIFFUSE_COLOR_BOOST: EInterchangePropertyTracks = EInterchangePropertyTracks(
        250,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_FINAL_GATHER_LIGHTING_UPDATE_SPEED: EInterchangePropertyTracks = EInterchangePropertyTracks(
        251,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_FINAL_GATHER_QUALITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        252,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_FINAL_GATHER_SCREEN_TRACES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        253,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_FRONT_LAYER_TRANSLUCENCY_REFLECTIONS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        254,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_FULL_SKYLIGHT_LEAKING_DISTANCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        255,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_MAX_REFLECTION_BOUNCES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        256,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_MAX_REFRACTION_BOUNCES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        257,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_MAX_ROUGHNESS_TO_TRACE_REFLECTIONS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        258,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_MAX_TRACE_DISTANCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        259,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_RAY_LIGHTING_MODE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        260,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_REFLECTION_QUALITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        261,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_REFLECTIONS_SCREEN_TRACES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        262,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_SCENE_DETAIL: EInterchangePropertyTracks = EInterchangePropertyTracks(
        263,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_SCENE_LIGHTING_QUALITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        264,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_SCENE_LIGHTING_UPDATE_SPEED: EInterchangePropertyTracks = EInterchangePropertyTracks(
        265,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_SCENE_VIEW_DISTANCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        266,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_SKYLIGHT_LEAKING: EInterchangePropertyTracks = EInterchangePropertyTracks(
        267,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_SKYLIGHT_LEAKING_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        268,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_MOTION_BLUR_AMOUNT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        269,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_MOTION_BLUR_MAX: EInterchangePropertyTracks = EInterchangePropertyTracks(
        270,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_MOTION_BLUR_PER_OBJECT_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        271,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_PATH_TRACING_MAX_BOUNCES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        272,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_PATH_TRACING_MAX_PATH_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        273,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_RAY_TRACING_AO: EInterchangePropertyTracks = EInterchangePropertyTracks(
        274,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_RAY_TRACING_AO_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        275,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_RAY_TRACING_AO_RADIUS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        276,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_RAY_TRACING_AO_SAMPLES_PER_PIXEL: EInterchangePropertyTracks = EInterchangePropertyTracks(
        277,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_RAY_TRACING_TRANSLUCENCY_MAX_ROUGHNESS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        278,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_RAY_TRACING_TRANSLUCENCY_REFRACTION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        279,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_RAY_TRACING_TRANSLUCENCY_REFRACTION_RAYS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        280,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_RAY_TRACING_TRANSLUCENCY_SAMPLES_PER_PIXEL: EInterchangePropertyTracks = EInterchangePropertyTracks(
        281,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_RAY_TRACING_TRANSLUCENCY_SHADOWS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        282,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_REFLECTION_METHOD: EInterchangePropertyTracks = EInterchangePropertyTracks(
        283,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_SCENE_COLOR_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        284,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_SCENE_FRINGE_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        285,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_SCREEN_SPACE_REFLECTION_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        286,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_SCREEN_SPACE_REFLECTION_MAX_ROUGHNESS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        287,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_SCREEN_SPACE_REFLECTION_QUALITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        288,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_SHARPEN: EInterchangePropertyTracks = EInterchangePropertyTracks(
        289,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_TEMPERATURE_TYPE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        290,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_TONE_CURVE_AMOUNT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        291,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_TRANSLUCENCY_TYPE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        292,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_VIGNETTE_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        293,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_WHITE_TEMP: EInterchangePropertyTracks = EInterchangePropertyTracks(
        294,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_WHITE_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        295,
    );
    pub const CAMERA_PROJECTION_MODE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        296,
    );
    pub const CAMERA_UPDATE_ORTHO_PLANES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        297,
    );
    pub const CAMERA_USE_CAMERA_HEIGHT_AS_VIEW_TARGET: EInterchangePropertyTracks = EInterchangePropertyTracks(
        298,
    );
    pub const CAMERA_USE_FIELD_OF_VIEW_FOR_LOD: EInterchangePropertyTracks = EInterchangePropertyTracks(
        299,
    );
    pub const MESH_OVERLAY_MATERIAL_MAX_DRAW_DISTANCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        300,
    );
    pub const SKINNED_MESH_CAPSULE_INDIRECT_SHADOW_MIN_VISIBILITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        301,
    );
    pub const SKINNED_MESH_CAST_CAPSULE_DIRECT_SHADOW: EInterchangePropertyTracks = EInterchangePropertyTracks(
        302,
    );
    pub const SKINNED_MESH_CAST_CAPSULE_INDIRECT_SHADOW: EInterchangePropertyTracks = EInterchangePropertyTracks(
        303,
    );
    pub const SKINNED_MESH_RENDER_STATIC: EInterchangePropertyTracks = EInterchangePropertyTracks(
        304,
    );
    pub const SKINNED_MESH_VISIBILITY_BASED_ANIM_TICK_OPTION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        305,
    );
    pub const SKELETAL_MESH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        306,
    );
    pub const SKELETAL_MESH_ALLOW_CLOTH_ACTORS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        307,
    );
    pub const SKELETAL_MESH_ANIMATION_MODE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        308,
    );
    pub const SKELETAL_MESH_CLOTH_BLEND_WEIGHT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        309,
    );
    pub const SKELETAL_MESH_CLOTH_MAX_DISTANCE_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        310,
    );
    pub const STATIC_MESH_DISTANCE_FIELD_SELF_SHADOW_BIAS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        311,
    );
    pub const STATIC_MESH_EVALUATE_WORLD_POSITION_OFFSET: EInterchangePropertyTracks = EInterchangePropertyTracks(
        312,
    );
    pub const STATIC_MESH_EVALUATE_WORLD_POSITION_OFFSET_IN_RAY_TRACING: EInterchangePropertyTracks = EInterchangePropertyTracks(
        313,
    );
    pub const STATIC_MESH_FORCED_LOD_MODEL: EInterchangePropertyTracks = EInterchangePropertyTracks(
        314,
    );
    pub const STATIC_MESH_REVERSE_CULLING: EInterchangePropertyTracks = EInterchangePropertyTracks(
        315,
    );
    pub const STATIC_MESH_WORLD_POSITION_OFFSET_DISABLE_DISTANCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        316,
    );
    pub const STATIC_MESH: EInterchangePropertyTracks = EInterchangePropertyTracks(317);
    pub const HETEROGENEOUS_VOLUME_FRAME: EInterchangePropertyTracks = EInterchangePropertyTracks(
        318,
    );
    pub const NONE: EInterchangePropertyTracks = EInterchangePropertyTracks(-1);
    pub const VISIBILITY: EInterchangePropertyTracks = EInterchangePropertyTracks(38);
}
#[repr(transparent)]
pub struct EInterchangeGroomCacheAttributes(pub u8);
impl EInterchangeGroomCacheAttributes {
    pub const NONE: EInterchangeGroomCacheAttributes = EInterchangeGroomCacheAttributes(
        0,
    );
    pub const POSITION: EInterchangeGroomCacheAttributes = EInterchangeGroomCacheAttributes(
        1,
    );
    pub const WIDTH: EInterchangeGroomCacheAttributes = EInterchangeGroomCacheAttributes(
        2,
    );
    pub const COLOR: EInterchangeGroomCacheAttributes = EInterchangeGroomCacheAttributes(
        4,
    );
    pub const POSITION_WIDTH: EInterchangeGroomCacheAttributes = EInterchangeGroomCacheAttributes(
        3,
    );
    pub const POSITION_COLOR: EInterchangeGroomCacheAttributes = EInterchangeGroomCacheAttributes(
        5,
    );
    pub const WIDTH_COLOR: EInterchangeGroomCacheAttributes = EInterchangeGroomCacheAttributes(
        6,
    );
    pub const POSITION_WIDTH_COLOR: EInterchangeGroomCacheAttributes = EInterchangeGroomCacheAttributes(
        7,
    );
}
#[repr(transparent)]
pub struct EInterchangeLightUnits(pub u8);
impl EInterchangeLightUnits {
    pub const UNITLESS: EInterchangeLightUnits = EInterchangeLightUnits(0);
    pub const CANDELAS: EInterchangeLightUnits = EInterchangeLightUnits(1);
    pub const LUMENS: EInterchangeLightUnits = EInterchangeLightUnits(2);
    pub const EV: EInterchangeLightUnits = EInterchangeLightUnits(3);
}
#[repr(transparent)]
pub struct EInterchangeSkyLightSourceType(pub u8);
impl EInterchangeSkyLightSourceType {
    pub const CAPTURED_SCENE: EInterchangeSkyLightSourceType = EInterchangeSkyLightSourceType(
        0,
    );
    pub const SPECIFIED_CUBEMAP: EInterchangeSkyLightSourceType = EInterchangeSkyLightSourceType(
        1,
    );
}
#[repr(transparent)]
pub struct EInterchangeMeshCollision(pub u8);
impl EInterchangeMeshCollision {
    pub const BOX: EInterchangeMeshCollision = EInterchangeMeshCollision(0);
    pub const SPHERE: EInterchangeMeshCollision = EInterchangeMeshCollision(1);
    pub const CAPSULE: EInterchangeMeshCollision = EInterchangeMeshCollision(2);
    pub const CONVEX10_DOP_X: EInterchangeMeshCollision = EInterchangeMeshCollision(3);
    pub const CONVEX10_DOP_Y: EInterchangeMeshCollision = EInterchangeMeshCollision(4);
    pub const CONVEX10_DOP_Z: EInterchangeMeshCollision = EInterchangeMeshCollision(5);
    pub const CONVEX18_DOP: EInterchangeMeshCollision = EInterchangeMeshCollision(6);
    pub const CONVEX26_DOP: EInterchangeMeshCollision = EInterchangeMeshCollision(7);
    pub const NONE: EInterchangeMeshCollision = EInterchangeMeshCollision(255);
}
#[repr(transparent)]
pub struct EInterchangeTextureWrapMode(pub u8);
impl EInterchangeTextureWrapMode {
    pub const WRAP: EInterchangeTextureWrapMode = EInterchangeTextureWrapMode(0);
    pub const CLAMP: EInterchangeTextureWrapMode = EInterchangeTextureWrapMode(1);
    pub const MIRROR: EInterchangeTextureWrapMode = EInterchangeTextureWrapMode(2);
}
#[repr(transparent)]
pub struct EVolumeGridElementType(pub u8);
impl EVolumeGridElementType {
    pub const UNKNOWN: EVolumeGridElementType = EVolumeGridElementType(0);
    pub const HALF: EVolumeGridElementType = EVolumeGridElementType(1);
    pub const FLOAT: EVolumeGridElementType = EVolumeGridElementType(2);
    pub const DOUBLE: EVolumeGridElementType = EVolumeGridElementType(3);
}
#[repr(transparent)]
pub struct EInterchangeMotionVectorsHandling(pub u8);
impl EInterchangeMotionVectorsHandling {
    pub const NO_MOTION_VECTORS: EInterchangeMotionVectorsHandling = EInterchangeMotionVectorsHandling(
        0,
    );
    pub const IMPORT_VELOCITIES_AS_MOTION_VECTORS: EInterchangeMotionVectorsHandling = EInterchangeMotionVectorsHandling(
        1,
    );
    pub const CALCULATE_MOTION_VECTORS_DURING_IMPORT: EInterchangeMotionVectorsHandling = EInterchangeMotionVectorsHandling(
        2,
    );
}
