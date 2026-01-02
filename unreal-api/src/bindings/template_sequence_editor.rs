#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAssetDefinition_TemplateSequence {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_TemplateSequence {}
#[repr(C, align(8))]
pub struct UAssetDefinition_CameraAnimationSequence {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_CameraAnimationSequence {}
#[repr(C, align(8))]
pub struct UCameraAnimationSequenceFactoryNew {
    __padding_end: [u8; 136],
}
impl UCameraAnimationSequenceFactoryNew {}
#[repr(C, align(8))]
pub struct UTemplateSequenceFactoryNew {
    __padding_end: [u8; 144],
}
impl UTemplateSequenceFactoryNew {}
#[repr(C, align(8))]
pub struct UTemplateSequenceEditorSettings {
    __padding_end: [u8; 56],
}
impl UTemplateSequenceEditorSettings {}
#[repr(C, align(8))]
pub struct UTemplateSequenceCameraPreviewSystem {
    __padding_end: [u8; 80],
}
impl UTemplateSequenceCameraPreviewSystem {}
