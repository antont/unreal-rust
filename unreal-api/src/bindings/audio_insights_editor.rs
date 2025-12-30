#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UAudioInsightsEditorSettings {
    pub b_world_filter_defaults_to_first_client: bool,
    pub spectrogram_settings: FSpectrogramRackUnitSettings,
    pub spectrum_analyzer_settings: FSpectrumAnalyzerRackUnitSettings,
    pub loudness_meter_settings: FLoudnessMeterRackUnitSettings,
    pub event_log_settings: FAudioEventLogSettings,
    pub sound_dashboard_settings: FSoundDashboardSettings,
}
