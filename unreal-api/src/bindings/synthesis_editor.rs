#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct USynthesisEditorSettings {
    __padding_end: [u8; 104],
}
impl USynthesisEditorSettings {}
#[repr(C, align(8))]
pub struct UAudioImpulseResponseFactory {
    __padding_end: [u8; 184],
}
impl UAudioImpulseResponseFactory {}
#[repr(C, align(8))]
pub struct UModularSynthPresetBankFactory {
    __padding_end: [u8; 136],
}
impl UModularSynthPresetBankFactory {}
#[repr(C, align(8))]
pub struct UMonoWaveTableSynthPresetFactory {
    __padding_end: [u8; 136],
}
impl UMonoWaveTableSynthPresetFactory {}
