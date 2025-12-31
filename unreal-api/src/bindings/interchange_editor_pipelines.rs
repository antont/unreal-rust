#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UInterchangeCardsPipeline {
    pub factory_node_classes_to_disabled: TArray<
        TSubclassOf<crate::bindings::core_u_object::UObject>,
    >,
}
pub struct UInterchangeEditorPipelineBase {}
pub struct UInterchangeEditorBlueprintPipelineBase {}
pub struct UInterchangeGraphInspectorPipeline {}
pub struct UInterchangePipelineConfigurationGeneric {}
pub struct UInterchangeBlueprintPipelineBaseFactory {
    pub blueprint_type: crate::bindings::engine::EBlueprintType,
    pub parent_class: TSubclassOf<
        crate::bindings::interchange_core::UInterchangePipelineBase,
    >,
}
pub struct UInterchangeEditorBlueprintPipelineBaseFactory {
    pub blueprint_type: crate::bindings::engine::EBlueprintType,
    pub parent_class: TSubclassOf<UInterchangeEditorPipelineBase>,
}
pub struct UInterchangePipelineBaseFactory {}
pub struct UInterchangePythonPipelineAssetFactory {}
