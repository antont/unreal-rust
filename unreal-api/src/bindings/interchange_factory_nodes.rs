#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UInterchangeAudioSoundWaveFactoryNode {}
pub struct UInterchangeActorFactoryNode {}
pub struct UInterchangePhysicalCameraFactoryNode {}
pub struct UInterchangeStandardCameraFactoryNode {}
pub struct UInterchangePhysicsAssetFactoryNode {}
pub struct UInterchangeSkeletonFactoryNode {}
pub struct UInterchangeTextureFactoryNode {}
pub struct UInterchangeTexture2DArrayFactoryNode {}
pub struct UInterchangeTexture2DFactoryNode {}
pub struct UInterchangeTextureCubeArrayFactoryNode {}
pub struct UInterchangeTextureCubeFactoryNode {}
pub struct UInterchangeTextureLightProfileFactoryNode {}
pub struct UInterchangeVolumeTextureFactoryNode {}
pub struct UInterchangeAnimSequenceFactoryNode {}
pub struct UInterchangeCommonPipelineDataFactoryNode {}
pub struct UInterchangeDecalActorFactoryNode {}
pub struct UInterchangeBaseMaterialFactoryNode {}
pub struct UInterchangeDecalMaterialFactoryNode {}
pub struct UInterchangeMeshFactoryNode {}
pub struct UInterchangeGeometryCacheFactoryNode {}
pub struct UInterchangeGroomCacheFactoryNode {}
pub struct UInterchangeGroomFactoryNode {}
pub struct UInterchangeHeterogeneousVolumeActorFactoryNode {}
pub struct UInterchangeLevelFactoryNode {}
pub struct UInterchangeLevelInstanceActorFactoryNode {}
pub struct UInterchangeLevelSequenceFactoryNode {}
pub struct UInterchangeBaseLightFactoryNode {}
pub struct UInterchangeDirectionalLightFactoryNode {}
pub struct UInterchangeLightFactoryNode {}
pub struct UInterchangeRectLightFactoryNode {}
pub struct UInterchangePointLightFactoryNode {}
pub struct UInterchangeSpotLightFactoryNode {}
pub struct UInterchangeSkyLightFactoryNode {}
pub struct UInterchangeMaterialFactoryNode {}
pub struct UInterchangeMaterialExpressionFactoryNode {}
pub struct UInterchangeMaterialInstanceFactoryNode {}
pub struct UInterchangeMaterialReferenceFactoryNode {}
pub struct UInterchangeMaterialFunctionCallExpressionFactoryNode {}
pub struct UInterchangeMaterialFunctionFactoryNode {}
pub struct UInterchangeMeshActorFactoryNode {}
pub struct UInterchangeSceneComponentFactoryNode {}
pub struct UInterchangeInstancedStaticMeshComponentFactoryNode {}
pub struct UInterchangeSceneImportAssetFactoryNode {}
pub struct UInterchangeSceneVariantSetsFactoryNode {}
pub struct UInterchangeSkeletalMeshFactoryNode {}
pub struct UInterchangeSkeletalMeshLodDataNode {}
pub struct UInterchangeSparseVolumeTextureFactoryNode {}
pub struct UInterchangeSpecularProfileFactoryNode {}
pub struct UInterchangeStaticMeshFactoryNode {}
pub struct UInterchangeStaticMeshLodDataNode {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInterchangeGroomCacheImportType(pub u8);
impl EInterchangeGroomCacheImportType {
    pub const NONE: EInterchangeGroomCacheImportType = EInterchangeGroomCacheImportType(
        0,
    );
    pub const STRANDS: EInterchangeGroomCacheImportType = EInterchangeGroomCacheImportType(
        1,
    );
    pub const GUIDES: EInterchangeGroomCacheImportType = EInterchangeGroomCacheImportType(
        2,
    );
    pub const ALL: EInterchangeGroomCacheImportType = EInterchangeGroomCacheImportType(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInterchangeSkeletalMeshContentType(pub u8);
impl EInterchangeSkeletalMeshContentType {
    pub const ALL: EInterchangeSkeletalMeshContentType = EInterchangeSkeletalMeshContentType(
        0,
    );
    pub const GEOMETRY: EInterchangeSkeletalMeshContentType = EInterchangeSkeletalMeshContentType(
        1,
    );
    pub const SKINNING_WEIGHTS: EInterchangeSkeletalMeshContentType = EInterchangeSkeletalMeshContentType(
        2,
    );
    pub const MAX: EInterchangeSkeletalMeshContentType = EInterchangeSkeletalMeshContentType(
        3,
    );
}
