#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FInterchangePipelinePropertyStatePerContext {
    pub b_visible: bool,
}
impl FInterchangePipelinePropertyStatePerContext {}
#[repr(C, align(1))]
pub struct FInterchangePipelinePropertyStates {
    pub b_locked: bool,
    pub b_pre_dialog_reset: bool,
    pub basic_layout_states: FInterchangePipelinePropertyStatePerContext,
    pub import_states: FInterchangePipelinePropertyStatePerContext,
    pub reimport_states: FInterchangePipelinePropertyStatePerContext,
}
impl FInterchangePipelinePropertyStates {}
#[repr(C, align(8))]
pub struct FInterchangeUserDefinedAttributeInfo {
    pub name: FString,
    __padding_end: [u8; 32],
}
impl FInterchangeUserDefinedAttributeInfo {}
#[repr(C, align(8))]
pub struct UInterchangeFactoryBase {
    __padding_end: [u8; 56],
}
impl UInterchangeFactoryBase {}
#[repr(C, align(8))]
pub struct UInterchangeWriterBase {
    __padding_end: [u8; 48],
}
impl UInterchangeWriterBase {}
#[repr(C, align(8))]
pub struct UInterchangePipelineBase {
    __padding_end: [u8; 344],
}
impl UInterchangePipelineBase {}
#[repr(C, align(8))]
pub struct UInterchangeResult {
    __padding_end: [u8; 120],
}
impl UInterchangeResult {}
#[repr(C, align(8))]
pub struct UInterchangeResultSuccess {
    __padding_end: [u8; 120],
}
impl UInterchangeResultSuccess {}
#[repr(C, align(8))]
pub struct UInterchangeResultWarning {
    __padding_end: [u8; 120],
}
impl UInterchangeResultWarning {}
#[repr(C, align(8))]
pub struct UInterchangeResultError {
    __padding_end: [u8; 120],
}
impl UInterchangeResultError {}
#[repr(C, align(8))]
pub struct UInterchangeResultWarning_Generic {
    __padding_end: [u8; 136],
}
impl UInterchangeResultWarning_Generic {}
#[repr(C, align(8))]
pub struct UInterchangeResultError_Generic {
    __padding_end: [u8; 136],
}
impl UInterchangeResultError_Generic {}
#[repr(C, align(8))]
pub struct UInterchangeResultError_ReimportFail {
    __padding_end: [u8; 120],
}
impl UInterchangeResultError_ReimportFail {}
#[repr(C, align(8))]
pub struct UInterchangeResultDisplay_Generic {
    __padding_end: [u8; 136],
}
impl UInterchangeResultDisplay_Generic {}
#[repr(C, align(8))]
pub struct UInterchangeResultsContainer {
    __padding_end: [u8; 104],
}
impl UInterchangeResultsContainer {}
#[repr(C, align(8))]
pub struct UInterchangeSourceData {
    __padding_end: [u8; 168],
}
impl UInterchangeSourceData {}
#[repr(C, align(8))]
pub struct UInterchangeTranslatorSettings {
    __padding_end: [u8; 48],
}
impl UInterchangeTranslatorSettings {}
#[repr(C, align(8))]
pub struct UInterchangeTranslatorBase {
    __padding_end: [u8; 80],
}
impl UInterchangeTranslatorBase {}
#[repr(C, align(8))]
pub struct UInterchangeBaseNode {
    __padding_end: [u8; 112],
}
impl UInterchangeBaseNode {}
#[repr(C, align(8))]
pub struct UInterchangeBaseNodeContainer {
    __padding_end: [u8; 208],
}
impl UInterchangeBaseNodeContainer {}
#[repr(C, align(8))]
pub struct UInterchangeFactoryBaseNode {
    __padding_end: [u8; 464],
}
impl UInterchangeFactoryBaseNode {}
#[repr(C, align(8))]
pub struct UInterchangeSourceNode {
    __padding_end: [u8; 472],
}
impl UInterchangeSourceNode {}
#[repr(C, align(8))]
pub struct UInterchangeUserDefinedAttributesAPI {
    __padding_end: [u8; 48],
}
impl UInterchangeUserDefinedAttributesAPI {}
#[repr(transparent)]
pub struct EInterchangePipelineContext(pub u8);
impl EInterchangePipelineContext {
    pub const NONE: EInterchangePipelineContext = EInterchangePipelineContext(0);
    pub const ASSET_IMPORT: EInterchangePipelineContext = EInterchangePipelineContext(1);
    pub const ASSET_REIMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        2,
    );
    pub const SCENE_IMPORT: EInterchangePipelineContext = EInterchangePipelineContext(3);
    pub const SCENE_REIMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        4,
    );
    pub const ASSET_CUSTOM_LOD_IMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        5,
    );
    pub const ASSET_CUSTOM_LOD_REIMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        6,
    );
    pub const ASSET_ALTERNATE_SKINNING_IMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        7,
    );
    pub const ASSET_ALTERNATE_SKINNING_REIMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        8,
    );
    pub const ASSET_CUSTOM_MORPH_TARGET_IMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        9,
    );
    pub const ASSET_CUSTOM_MORPH_TARGET_RE_IMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        10,
    );
}
#[repr(transparent)]
pub struct EInterchangeTranslatorAssetType(pub u8);
impl EInterchangeTranslatorAssetType {
    pub const NONE: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(0);
    pub const TEXTURES: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(
        1,
    );
    pub const MATERIALS: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(
        2,
    );
    pub const MESHES: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(
        4,
    );
    pub const ANIMATIONS: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(
        8,
    );
    pub const SOUNDS: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(
        16,
    );
    pub const GROOMS: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(
        32,
    );
}
#[repr(transparent)]
pub struct EInterchangeFactoryAssetType(pub u8);
impl EInterchangeFactoryAssetType {
    pub const NONE: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(0);
    pub const TEXTURES: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(1);
    pub const MATERIALS: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(2);
    pub const MESHES: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(3);
    pub const ANIMATIONS: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(4);
    pub const PHYSICS: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(5);
    pub const GROOMS: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(6);
    pub const SOUNDS: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(7);
}
#[repr(transparent)]
pub struct EInterchangeTranslatorType(pub u8);
impl EInterchangeTranslatorType {
    pub const INVALID: EInterchangeTranslatorType = EInterchangeTranslatorType(0);
    pub const ASSETS: EInterchangeTranslatorType = EInterchangeTranslatorType(2);
    pub const ACTORS: EInterchangeTranslatorType = EInterchangeTranslatorType(4);
    pub const SCENES: EInterchangeTranslatorType = EInterchangeTranslatorType(6);
}
#[repr(transparent)]
pub struct EInterchangeNodeContainerType(pub u8);
impl EInterchangeNodeContainerType {
    pub const NONE: EInterchangeNodeContainerType = EInterchangeNodeContainerType(0);
    pub const TRANSLATED_SCENE: EInterchangeNodeContainerType = EInterchangeNodeContainerType(
        1,
    );
    pub const TRANSLATED_ASSET: EInterchangeNodeContainerType = EInterchangeNodeContainerType(
        2,
    );
    pub const FACTORY_DATA: EInterchangeNodeContainerType = EInterchangeNodeContainerType(
        3,
    );
}
#[repr(transparent)]
pub struct EReimportStrategyFlags(pub u8);
impl EReimportStrategyFlags {
    pub const APPLY_NO_PROPERTIES: EReimportStrategyFlags = EReimportStrategyFlags(0);
    pub const APPLY_PIPELINE_PROPERTIES: EReimportStrategyFlags = EReimportStrategyFlags(
        1,
    );
    pub const APPLY_EDITOR_CHANGED_PROPERTIES: EReimportStrategyFlags = EReimportStrategyFlags(
        2,
    );
}
#[repr(transparent)]
pub struct EInterchangeNodeUserInterfaceContext(pub u8);
impl EInterchangeNodeUserInterfaceContext {
    pub const NONE: EInterchangeNodeUserInterfaceContext = EInterchangeNodeUserInterfaceContext(
        0,
    );
    pub const PREVIEW: EInterchangeNodeUserInterfaceContext = EInterchangeNodeUserInterfaceContext(
        1,
    );
}
