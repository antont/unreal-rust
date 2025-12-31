#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FInterchangePipelineContextParams {
    pub context_type: EInterchangePipelineContext,
    pub import_object_type: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub reimport_asset: UPtr<crate::bindings::core_u_object::UObject>,
    pub base_node_container: UPtr<UInterchangeBaseNodeContainer>,
}
#[repr(C, align(1))]
pub struct FInterchangePipelinePropertyStatePerContext {
    pub b_visible: bool,
}
#[repr(C, align(1))]
pub struct FInterchangePipelinePropertyStates {
    pub b_locked: bool,
    pub b_pre_dialog_reset: bool,
    pub basic_layout_states: FInterchangePipelinePropertyStatePerContext,
    pub import_states: FInterchangePipelinePropertyStatePerContext,
    pub reimport_states: FInterchangePipelinePropertyStatePerContext,
}
#[repr(C, align(8))]
pub struct FInterchangeUserDefinedAttributeInfo {
    pub name: FString,
}
pub struct UInterchangeFactoryBase {
    pub results: UPtr<UInterchangeResultsContainer>,
}
pub struct UInterchangeWriterBase {}
pub struct UInterchangePipelineBase {
    pub destination_name: FString,
    pub content_import_path: FString,
    pub original_pipeline_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub b_allow_property_states_edition: bool,
    pub b_is_reimport_context: bool,
    pub b_is_show_essentials: bool,
    pub b_from_reimport_or_override: bool,
    pub results: UPtr<UInterchangeResultsContainer>,
    pub properties_states: TMap<FName, FInterchangePipelinePropertyStates>,
    pub cache_properties_states: TMap<FName, FInterchangePipelinePropertyStates>,
    pub cache_context_param: FInterchangePipelineContextParams,
}
pub struct UInterchangeResult {
    pub source_asset_name: FString,
    pub destination_asset_name: FString,
    pub asset_friendly_name: FString,
    pub asset_type: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub interchange_key: FString,
}
pub struct UInterchangeResultSuccess {}
pub struct UInterchangeResultWarning {}
pub struct UInterchangeResultError {}
pub struct UInterchangeResultWarning_Generic {
    pub text: FText,
}
pub struct UInterchangeResultError_Generic {
    pub text: FText,
}
pub struct UInterchangeResultError_ReimportFail {}
pub struct UInterchangeResultDisplay_Generic {
    pub text: FText,
}
pub struct UInterchangeResultsContainer {
    pub results: TArray<UPtr<UInterchangeResult>>,
}
pub struct UInterchangeSourceData {
    pub filename: FString,
    pub context_objects_by_tag: TMap<
        FString,
        UPtr<crate::bindings::core_u_object::UObject>,
    >,
}
pub struct UInterchangeTranslatorSettings {}
pub struct UInterchangeTranslatorBase {
    pub results: UPtr<UInterchangeResultsContainer>,
    pub source_data: UPtr<UInterchangeSourceData>,
}
pub struct UInterchangeBaseNode {
    pub user_interface_context: EInterchangeNodeUserInterfaceContext,
}
pub struct UInterchangeBaseNodeContainer {
    pub nodes: TMap<FString, UPtr<UInterchangeBaseNode>>,
}
pub struct UInterchangeFactoryBaseNode {
    pub attributes_applied_through_delegates_key_set: TSet<FString>,
}
pub struct UInterchangeSourceNode {}
pub struct UInterchangeUserDefinedAttributesAPI {}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInterchangeTranslatorType(pub u8);
impl EInterchangeTranslatorType {
    pub const INVALID: EInterchangeTranslatorType = EInterchangeTranslatorType(0);
    pub const ASSETS: EInterchangeTranslatorType = EInterchangeTranslatorType(2);
    pub const ACTORS: EInterchangeTranslatorType = EInterchangeTranslatorType(4);
    pub const SCENES: EInterchangeTranslatorType = EInterchangeTranslatorType(6);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
