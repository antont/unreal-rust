#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAudioAnalyzerAssetBase {
    __padding_end: [u8; 48],
}
impl UAudioAnalyzerAssetBase {}
#[repr(C, align(8))]
pub struct UAudioAnalyzerSettings {
    __padding_end: [u8; 48],
}
impl UAudioAnalyzerSettings {}
#[repr(C, align(8))]
pub struct UAudioAnalyzer {
    __padding_end: [u8; 168],
}
impl UAudioAnalyzer {}
#[repr(C, align(8))]
pub struct UAudioAnalyzerNRTSettings {
    __padding_end: [u8; 80],
}
impl UAudioAnalyzerNRTSettings {}
#[repr(C, align(8))]
pub struct UAudioAnalyzerNRT {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub sound: UPtr<crate::bindings::engine::USoundWave>,
    pub duration_in_seconds: f32,
    __padding_end: [u8; 172],
}
impl UAudioAnalyzerNRT {}
#[repr(C, align(8))]
pub struct UAudioAnalyzerSubsystem {
    __padding_end: [u8; 88],
}
impl UAudioAnalyzerSubsystem {}
#[repr(transparent)]
pub struct FAudioAnalyzerNRT_OnAnalysisComplete {
    _opague: u8,
}
