#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FConstantQResults {
    pub time_seconds: f32,
    pub spectrum_values: TArray<f32>,
}
#[repr(C, align(4))]
pub struct FLoudnessResults {
    pub loudness: f32,
    pub normalized_loudness: f32,
    pub perceptual_energy: f32,
    pub time_seconds: f32,
}
#[repr(C, align(4))]
pub struct FMeterResults {
    pub time_seconds: f32,
    pub meter_value: f32,
    pub peak_value: f32,
    pub num_samples_clipping: i32,
    pub clipping_value: f32,
}
#[repr(C, align(8))]
pub struct FSynesthesiaSpectrumResults {
    pub time_seconds: f32,
    pub spectrum_values: TArray<f32>,
}
pub struct UAudioSynesthesiaSettings {}
pub struct UAudioSynesthesiaNRTSettings {}
pub struct UAudioSynesthesiaNRT {}
pub struct UConstantQSettings {
    pub starting_frequency_hz: f32,
    pub num_bands: i32,
    pub num_bands_per_octave: f32,
    pub analysis_period_in_seconds: f32,
    pub b_downmix_to_mono: bool,
    pub fft_size: EConstantQFFTSizeEnum,
    pub window_type: crate::bindings::engine::EFFTWindowType,
    pub spectrum_type: crate::bindings::engine::EAudioSpectrumType,
    pub band_width_stretch: f32,
    pub cqt_normalization: EConstantQNormalizationEnum,
    pub noise_floor_db: f32,
}
pub struct UConstantQAnalyzer {
    pub settings: UPtr<UConstantQSettings>,
    pub on_constant_q_results: FConstantQAnalyzer_OnConstantQResults,
    pub on_latest_constant_q_results: FConstantQAnalyzer_OnLatestConstantQResults,
}
pub struct UConstantQNRTSettings {
    pub starting_frequency: f32,
    pub num_bands: i32,
    pub num_bands_per_octave: f32,
    pub analysis_period: f32,
    pub b_downmix_to_mono: bool,
    pub fft_size: EConstantQFFTSizeEnum,
    pub window_type: crate::bindings::engine::EFFTWindowType,
    pub spectrum_type: crate::bindings::engine::EAudioSpectrumType,
    pub band_width_stretch: f32,
    pub cqt_normalization: EConstantQNormalizationEnum,
    pub noise_floor_db: f32,
}
pub struct UConstantQNRT {
    pub settings: UPtr<UConstantQNRTSettings>,
}
pub struct ULKFSSettings {
    pub analysis_period: f32,
    pub analysis_window_duration: f32,
    pub short_term_loudness_duration: f32,
    pub integrated_loudness_analysis_period: f32,
    pub integrated_loudness_duration: f32,
}
pub struct ULKFSAnalyzer {
    pub settings: UPtr<ULKFSSettings>,
    pub on_overall_lkfs_results: FLKFSAnalyzer_OnOverallLKFSResults,
    pub on_per_channel_lkfs_results: FLKFSAnalyzer_OnPerChannelLKFSResults,
    pub on_latest_overall_lkfs_results: FLKFSAnalyzer_OnLatestOverallLKFSResults,
    pub on_latest_per_channel_lkfs_results: FLKFSAnalyzer_OnLatestPerChannelLKFSResults,
}
pub struct ULKFSNRTSettings {
    pub analysis_period: f32,
    pub analysis_window_duration: f32,
    pub short_term_loudness_duration: f32,
}
pub struct ULKFSNRT {
    pub settings: UPtr<ULKFSNRTSettings>,
}
pub struct ULoudnessSettings {
    pub analysis_period: f32,
    pub minimum_frequency: f32,
    pub maximum_frequency: f32,
    pub curve_type: ELoudnessCurveTypeEnum,
    pub noise_floor_db: f32,
    pub expected_max_loudness: f32,
}
pub struct ULoudnessAnalyzer {
    pub settings: UPtr<ULoudnessSettings>,
    pub on_overall_loudness_results: FLoudnessAnalyzer_OnOverallLoudnessResults,
    pub on_per_channel_loudness_results: FLoudnessAnalyzer_OnPerChannelLoudnessResults,
    pub on_latest_overall_loudness_results: FLoudnessAnalyzer_OnLatestOverallLoudnessResults,
    pub on_latest_per_channel_loudness_results: FLoudnessAnalyzer_OnLatestPerChannelLoudnessResults,
}
pub struct ULoudnessNRTSettings {
    pub analysis_period: f32,
    pub minimum_frequency: f32,
    pub maximum_frequency: f32,
    pub curve_type: ELoudnessNRTCurveTypeEnum,
    pub noise_floor_db: f32,
}
pub struct ULoudnessNRT {
    pub settings: UPtr<ULoudnessNRTSettings>,
}
pub struct UMeterSettings {
    pub analysis_period: f32,
    pub peak_mode: EMeterPeakType,
    pub meter_attack_time: i32,
    pub meter_release_time: i32,
    pub peak_hold_time: i32,
    pub clipping_threshold: f32,
}
pub struct UMeterAnalyzer {
    pub settings: UPtr<UMeterSettings>,
    pub on_overall_meter_results: FMeterAnalyzer_OnOverallMeterResults,
    pub on_per_channel_meter_results: FMeterAnalyzer_OnPerChannelMeterResults,
    pub on_latest_overall_meter_results: FMeterAnalyzer_OnLatestOverallMeterResults,
    pub on_latest_per_channel_meter_results: FMeterAnalyzer_OnLatestPerChannelMeterResults,
}
pub struct UOnsetNRTSettings {
    pub b_downmix_to_mono: bool,
    pub granularity_in_seconds: f32,
    pub sensitivity: f32,
    pub minimum_frequency: f32,
    pub maximum_frequency: f32,
}
pub struct UOnsetNRT {
    pub settings: UPtr<UOnsetNRTSettings>,
}
pub struct USynesthesiaSpectrumAnalysisSettings {
    pub analysis_period: f32,
    pub fft_size: crate::bindings::engine::EFFTSize,
    pub spectrum_type: crate::bindings::engine::EAudioSpectrumType,
    pub window_type: crate::bindings::engine::EFFTWindowType,
    pub b_downmix_to_mono: bool,
}
pub struct USynesthesiaSpectrumAnalyzer {
    pub settings: UPtr<USynesthesiaSpectrumAnalysisSettings>,
    pub on_spectrum_results: FSynesthesiaSpectrumAnalyzer_OnSpectrumResults,
    pub on_latest_spectrum_results: FSynesthesiaSpectrumAnalyzer_OnLatestSpectrumResults,
}
pub struct FConstantQAnalyzer_OnConstantQResults;
pub struct FConstantQAnalyzer_OnLatestConstantQResults;
pub struct FLKFSAnalyzer_OnOverallLKFSResults;
pub struct FLKFSAnalyzer_OnPerChannelLKFSResults;
pub struct FLKFSAnalyzer_OnLatestOverallLKFSResults;
pub struct FLKFSAnalyzer_OnLatestPerChannelLKFSResults;
pub struct FLoudnessAnalyzer_OnOverallLoudnessResults;
pub struct FLoudnessAnalyzer_OnPerChannelLoudnessResults;
pub struct FLoudnessAnalyzer_OnLatestOverallLoudnessResults;
pub struct FLoudnessAnalyzer_OnLatestPerChannelLoudnessResults;
pub struct FMeterAnalyzer_OnOverallMeterResults;
pub struct FMeterAnalyzer_OnPerChannelMeterResults;
pub struct FMeterAnalyzer_OnLatestOverallMeterResults;
pub struct FMeterAnalyzer_OnLatestPerChannelMeterResults;
pub struct FSynesthesiaSpectrumAnalyzer_OnSpectrumResults;
pub struct FSynesthesiaSpectrumAnalyzer_OnLatestSpectrumResults;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EConstantQFFTSizeEnum(pub u8);
impl EConstantQFFTSizeEnum {
    pub const MIN: EConstantQFFTSizeEnum = EConstantQFFTSizeEnum(0);
    pub const XX_SMALL: EConstantQFFTSizeEnum = EConstantQFFTSizeEnum(1);
    pub const X_SMALL: EConstantQFFTSizeEnum = EConstantQFFTSizeEnum(2);
    pub const SMALL: EConstantQFFTSizeEnum = EConstantQFFTSizeEnum(3);
    pub const MEDIUM: EConstantQFFTSizeEnum = EConstantQFFTSizeEnum(4);
    pub const LARGE: EConstantQFFTSizeEnum = EConstantQFFTSizeEnum(5);
    pub const X_LARGE: EConstantQFFTSizeEnum = EConstantQFFTSizeEnum(6);
    pub const XX_LARGE: EConstantQFFTSizeEnum = EConstantQFFTSizeEnum(7);
    pub const MAX: EConstantQFFTSizeEnum = EConstantQFFTSizeEnum(8);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EConstantQNormalizationEnum(pub u8);
impl EConstantQNormalizationEnum {
    pub const EQUAL_EUCLIDEAN_NORM: EConstantQNormalizationEnum = EConstantQNormalizationEnum(
        0,
    );
    pub const EQUAL_ENERGY: EConstantQNormalizationEnum = EConstantQNormalizationEnum(1);
    pub const EQUAL_AMPLITUDE: EConstantQNormalizationEnum = EConstantQNormalizationEnum(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELoudnessCurveTypeEnum(pub u8);
impl ELoudnessCurveTypeEnum {
    pub const A: ELoudnessCurveTypeEnum = ELoudnessCurveTypeEnum(0);
    pub const B: ELoudnessCurveTypeEnum = ELoudnessCurveTypeEnum(1);
    pub const C: ELoudnessCurveTypeEnum = ELoudnessCurveTypeEnum(2);
    pub const D: ELoudnessCurveTypeEnum = ELoudnessCurveTypeEnum(3);
    pub const K: ELoudnessCurveTypeEnum = ELoudnessCurveTypeEnum(4);
    pub const NONE: ELoudnessCurveTypeEnum = ELoudnessCurveTypeEnum(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELoudnessNRTCurveTypeEnum(pub u8);
impl ELoudnessNRTCurveTypeEnum {
    pub const A: ELoudnessNRTCurveTypeEnum = ELoudnessNRTCurveTypeEnum(0);
    pub const B: ELoudnessNRTCurveTypeEnum = ELoudnessNRTCurveTypeEnum(1);
    pub const C: ELoudnessNRTCurveTypeEnum = ELoudnessNRTCurveTypeEnum(2);
    pub const D: ELoudnessNRTCurveTypeEnum = ELoudnessNRTCurveTypeEnum(3);
    pub const K: ELoudnessNRTCurveTypeEnum = ELoudnessNRTCurveTypeEnum(4);
    pub const NONE: ELoudnessNRTCurveTypeEnum = ELoudnessNRTCurveTypeEnum(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMeterPeakType(pub u8);
impl EMeterPeakType {
    pub const MEAN_SQUARED: EMeterPeakType = EMeterPeakType(0);
    pub const ROOT_MEAN_SQUARED: EMeterPeakType = EMeterPeakType(1);
    pub const PEAK: EMeterPeakType = EMeterPeakType(2);
    pub const COUNT: EMeterPeakType = EMeterPeakType(3);
}
