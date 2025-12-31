#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UAudioInsightsEditorSettings {
    pub b_world_filter_defaults_to_first_client: bool,
    pub spectrogram_settings: crate::bindings::audio_widgets::FSpectrogramRackUnitSettings,
    pub spectrum_analyzer_settings: crate::bindings::audio_widgets::FSpectrumAnalyzerRackUnitSettings,
    pub loudness_meter_settings: crate::bindings::audio_widgets::FLoudnessMeterRackUnitSettings,
    pub event_log_settings: crate::bindings::audio_insights::FAudioEventLogSettings,
    pub sound_dashboard_settings: crate::bindings::audio_insights::FSoundDashboardSettings,
}
