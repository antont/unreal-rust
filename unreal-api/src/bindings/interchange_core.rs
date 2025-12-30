#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FInterchangePipelineContextParams {
    pub context_type: EInterchangePipelineContext,
    pub import_object_type: TSubclassOf<UObject>,
    pub reimport_asset: UPtr<UObject>,
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
    pub original_pipeline_path: FSoftObjectPath,
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
    pub asset_type: TSubclassOf<UObject>,
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
    pub context_objects_by_tag: TMap<FString, UPtr<UObject>>,
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
