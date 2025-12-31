#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UAudioAnalyzerAssetBase {}
pub struct UAudioAnalyzerSettings {}
pub struct UAudioAnalyzer {
    pub audio_bus: UPtr<crate::bindings::engine::UAudioBus>,
    pub audio_analyzer_subsystem: UPtr<UAudioAnalyzerSubsystem>,
}
pub struct UAudioAnalyzerNRTSettings {}
pub struct UAudioAnalyzerNRT {
    pub sound: UPtr<crate::bindings::engine::USoundWave>,
    pub duration_in_seconds: f32,
    pub on_analysis_complete: FAudioAnalyzerNRT_OnAnalysisComplete,
}
pub struct UAudioAnalyzerSubsystem {
    pub audio_analyzers: TArray<UPtr<UAudioAnalyzer>>,
}
pub struct FAudioAnalyzerNRT_OnAnalysisComplete;
