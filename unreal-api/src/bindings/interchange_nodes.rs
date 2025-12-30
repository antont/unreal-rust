#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FInterchangeAnimationPayLoadKey {
    pub unique_id: FString,
    pub ty: EInterchangeAnimationPayLoadType,
}
#[repr(C, align(8))]
pub struct FInterchangeGroomPayloadKey {
    pub unique_id: FString,
    pub ty: EInterchangeGroomPayLoadType,
    pub frame_number: i32,
}
#[repr(C, align(8))]
pub struct FInterchangeMeshPayLoadKey {
    pub unique_id: FString,
    pub ty: EInterchangeMeshPayLoadType,
    pub frame_number: i32,
}
pub struct UInterchangePhysicalCameraNode {}
pub struct UInterchangeStandardCameraNode {}
pub struct UInterchangeShaderNode {}
pub struct UInterchangeDecalMaterialNode {}
pub struct UInterchangeDecalNode {}
pub struct UInterchangeTextureNode {}
pub struct UInterchangeTexture2DArrayNode {}
pub struct UInterchangeTextureCubeArrayNode {}
pub struct UInterchangeTextureCubeNode {}
pub struct UInterchangeTextureLightProfileNode {}
pub struct UInterchangeVolumeTextureNode {}
pub struct UInterchangeAnimationTrackSetNode {}
pub struct UInterchangeAnimationTrackBaseNode {}
pub struct UInterchangeAnimationTrackSetInstanceNode {}
pub struct UInterchangeAnimationTrackNode {}
pub struct UInterchangeTransformAnimationTrackNode {}
pub struct UInterchangeSkeletalAnimationTrackNode {}
pub struct UInterchangeAudioSoundWaveNode {}
pub struct UInterchangeGroomNode {}
pub struct UInterchangeBaseLightNode {}
pub struct UInterchangeLightNode {}
pub struct UInterchangePointLightNode {}
pub struct UInterchangeSpotLightNode {}
pub struct UInterchangeRectLightNode {}
pub struct UInterchangeDirectionalLightNode {}
pub struct UInterchangeSkyLightNode {}
pub struct UInterchangeMaterialInstanceNode {}
pub struct UInterchangeMaterialReferenceNode {}
pub struct UInterchangeMeshLODContainerNode {}
pub struct UInterchangeMeshNode {}
pub struct UInterchangeGeometryCacheNode {}
pub struct UInterchangeSceneComponentNode {}
pub struct UInterchangeInstancedStaticMeshComponentNode {}
pub struct UInterchangeSceneNode {}
pub struct UInterchangeShaderPortsAPI {}
pub struct UInterchangeFunctionCallShaderNode {}
pub struct UInterchangeShaderGraphNode {}
pub struct UInterchangeSpecularProfileNode {}
pub struct UInterchangeTexture2DNode {}
pub struct UInterchangeTextureBlurNode {}
pub struct UInterchangeVariantSetNode {}
pub struct UInterchangeSceneVariantSetsNode {}
pub struct UInterchangeVolumeNode {}
pub struct UInterchangeVolumeGridNode {}
