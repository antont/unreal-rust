#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UInterchangeCardsPipeline {
    pub factory_node_classes_to_disabled: TArray<TSubclassOf<UObject>>,
}
pub struct UInterchangeEditorPipelineBase {}
pub struct UInterchangeEditorBlueprintPipelineBase {}
pub struct UInterchangeGraphInspectorPipeline {}
pub struct UInterchangePipelineConfigurationGeneric {}
pub struct UInterchangeBlueprintPipelineBaseFactory {
    pub blueprint_type: EBlueprintType,
    pub parent_class: TSubclassOf<UInterchangePipelineBase>,
}
pub struct UInterchangeEditorBlueprintPipelineBaseFactory {
    pub blueprint_type: EBlueprintType,
    pub parent_class: TSubclassOf<UInterchangeEditorPipelineBase>,
}
pub struct UInterchangePipelineBaseFactory {}
pub struct UInterchangePythonPipelineAssetFactory {}
