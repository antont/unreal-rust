#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UInterchangeCardsPipeline {
    __padding_end: [u8; 360],
}
impl UInterchangeCardsPipeline {}
#[repr(C, align(8))]
pub struct UInterchangeEditorPipelineBase {
    __padding_end: [u8; 344],
}
impl UInterchangeEditorPipelineBase {}
#[repr(C, align(8))]
pub struct UInterchangeEditorBlueprintPipelineBase {
    __padding_end: [u8; 1432],
}
impl UInterchangeEditorBlueprintPipelineBase {}
#[repr(C, align(8))]
pub struct UInterchangeGraphInspectorPipeline {
    __padding_end: [u8; 344],
}
impl UInterchangeGraphInspectorPipeline {}
#[repr(C, align(8))]
pub struct UInterchangePipelineConfigurationGeneric {
    __padding_end: [u8; 48],
}
impl UInterchangePipelineConfigurationGeneric {}
#[repr(C, align(8))]
pub struct UInterchangeBlueprintPipelineBaseFactory {
    __padding_end: [u8; 152],
}
impl UInterchangeBlueprintPipelineBaseFactory {}
#[repr(C, align(8))]
pub struct UInterchangeEditorBlueprintPipelineBaseFactory {
    __padding_end: [u8; 152],
}
impl UInterchangeEditorBlueprintPipelineBaseFactory {}
#[repr(C, align(8))]
pub struct UInterchangePipelineBaseFactory {
    __padding_end: [u8; 144],
}
impl UInterchangePipelineBaseFactory {}
#[repr(C, align(8))]
pub struct UInterchangePythonPipelineAssetFactory {
    __padding_end: [u8; 144],
}
impl UInterchangePythonPipelineAssetFactory {}
