#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAssetEditorUISubsystem {
    __padding_end: [u8; 56],
}
impl UAssetEditorUISubsystem {}
pub struct UAssetEditorContextInterface {}
pub struct IAssetEditorContextInterface {}
pub struct UTypedElementDetailsInterface {}
pub struct ITypedElementDetailsInterface {}
#[repr(C, align(8))]
pub struct UTypedElementViewportInteraction {
    __padding_end: [u8; 2096],
}
impl UTypedElementViewportInteraction {}
pub struct UAssetFactoryInterface {}
pub struct IAssetFactoryInterface {}
#[repr(C, align(8))]
pub struct UEditorElementSubsystem {
    __padding_end: [u8; 56],
}
impl UEditorElementSubsystem {}
#[repr(C, align(8))]
pub struct UPlacementSubsystem {
    __padding_end: [u8; 112],
}
impl UPlacementSubsystem {}
