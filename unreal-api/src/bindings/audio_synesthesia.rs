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
impl FConstantQResults {}
#[repr(C, align(4))]
pub struct FLoudnessResults {
    pub loudness: f32,
    pub normalized_loudness: f32,
    pub perceptual_energy: f32,
    pub time_seconds: f32,
}
impl FLoudnessResults {}
#[repr(C, align(4))]
pub struct FMeterResults {
    pub time_seconds: f32,
    pub meter_value: f32,
    pub peak_value: f32,
    pub num_samples_clipping: i32,
    pub clipping_value: f32,
}
impl FMeterResults {}
#[repr(C, align(8))]
pub struct FSynesthesiaSpectrumResults {
    pub time_seconds: f32,
    pub spectrum_values: TArray<f32>,
}
impl FSynesthesiaSpectrumResults {}
#[repr(C, align(8))]
pub struct UAudioSynesthesiaSettings {
    __padding_end: [u8; 48],
}
impl UAudioSynesthesiaSettings {}
#[repr(C, align(8))]
pub struct UAudioSynesthesiaNRTSettings {
    __padding_end: [u8; 80],
}
impl UAudioSynesthesiaNRTSettings {}
#[repr(C, align(8))]
pub struct UAudioSynesthesiaNRT {
    __padding_end: [u8; 232],
}
impl UAudioSynesthesiaNRT {}
#[repr(C, align(8))]
pub struct UConstantQSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
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
impl UConstantQSettings {}
#[repr(C, align(8))]
pub struct UConstantQAnalyzer {
    #[doc(hidden)]
    __padding_168: [u8; 168],
    pub settings: UPtr<UConstantQSettings>,
    __padding_end: [u8; 112],
}
impl UConstantQAnalyzer {}
#[repr(C, align(8))]
pub struct UConstantQNRTSettings {
    #[doc(hidden)]
    __padding_80: [u8; 80],
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
impl UConstantQNRTSettings {}
#[repr(C, align(8))]
pub struct UConstantQNRT {
    #[doc(hidden)]
    __padding_232: [u8; 232],
    pub settings: UPtr<UConstantQNRTSettings>,
}
impl UConstantQNRT {}
#[repr(C, align(8))]
pub struct ULKFSSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub analysis_period: f32,
    pub analysis_window_duration: f32,
    pub short_term_loudness_duration: f32,
    pub integrated_loudness_analysis_period: f32,
    pub integrated_loudness_duration: f32,
    __padding_end: [u8; 4],
}
impl ULKFSSettings {}
#[repr(C, align(8))]
pub struct ULKFSAnalyzer {
    #[doc(hidden)]
    __padding_168: [u8; 168],
    pub settings: UPtr<ULKFSSettings>,
    __padding_end: [u8; 224],
}
impl ULKFSAnalyzer {}
#[repr(C, align(8))]
pub struct ULKFSNRTSettings {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub analysis_period: f32,
    pub analysis_window_duration: f32,
    pub short_term_loudness_duration: f32,
    __padding_end: [u8; 4],
}
impl ULKFSNRTSettings {}
#[repr(C, align(8))]
pub struct ULKFSNRT {
    #[doc(hidden)]
    __padding_232: [u8; 232],
    pub settings: UPtr<ULKFSNRTSettings>,
}
impl ULKFSNRT {}
#[repr(C, align(8))]
pub struct ULoudnessSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub analysis_period: f32,
    pub minimum_frequency: f32,
    pub maximum_frequency: f32,
    pub curve_type: ELoudnessCurveTypeEnum,
    pub noise_floor_db: f32,
    pub expected_max_loudness: f32,
}
impl ULoudnessSettings {}
#[repr(C, align(8))]
pub struct ULoudnessAnalyzer {
    #[doc(hidden)]
    __padding_168: [u8; 168],
    pub settings: UPtr<ULoudnessSettings>,
    __padding_end: [u8; 96],
}
impl ULoudnessAnalyzer {}
#[repr(C, align(8))]
pub struct ULoudnessNRTSettings {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub analysis_period: f32,
    pub minimum_frequency: f32,
    pub maximum_frequency: f32,
    pub curve_type: ELoudnessNRTCurveTypeEnum,
    pub noise_floor_db: f32,
    __padding_end: [u8; 4],
}
impl ULoudnessNRTSettings {}
#[repr(C, align(8))]
pub struct ULoudnessNRT {
    #[doc(hidden)]
    __padding_232: [u8; 232],
    pub settings: UPtr<ULoudnessNRTSettings>,
}
impl ULoudnessNRT {}
#[repr(C, align(8))]
pub struct UMeterSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub analysis_period: f32,
    pub peak_mode: EMeterPeakType,
    pub meter_attack_time: i32,
    pub meter_release_time: i32,
    pub peak_hold_time: i32,
    pub clipping_threshold: f32,
}
impl UMeterSettings {}
#[repr(C, align(8))]
pub struct UMeterAnalyzer {
    #[doc(hidden)]
    __padding_168: [u8; 168],
    pub settings: UPtr<UMeterSettings>,
    __padding_end: [u8; 224],
}
impl UMeterAnalyzer {}
#[repr(C, align(8))]
pub struct UOnsetNRTSettings {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub b_downmix_to_mono: bool,
    pub granularity_in_seconds: f32,
    pub sensitivity: f32,
    pub minimum_frequency: f32,
    pub maximum_frequency: f32,
    __padding_end: [u8; 4],
}
impl UOnsetNRTSettings {}
#[repr(C, align(8))]
pub struct UOnsetNRT {
    #[doc(hidden)]
    __padding_232: [u8; 232],
    pub settings: UPtr<UOnsetNRTSettings>,
}
impl UOnsetNRT {}
#[repr(C, align(8))]
pub struct USynesthesiaSpectrumAnalysisSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub analysis_period: f32,
    pub fft_size: crate::bindings::engine::EFFTSize,
    pub spectrum_type: crate::bindings::engine::EAudioSpectrumType,
    pub window_type: crate::bindings::engine::EFFTWindowType,
    pub b_downmix_to_mono: bool,
}
impl USynesthesiaSpectrumAnalysisSettings {}
#[repr(C, align(8))]
pub struct USynesthesiaSpectrumAnalyzer {
    #[doc(hidden)]
    __padding_168: [u8; 168],
    pub settings: UPtr<USynesthesiaSpectrumAnalysisSettings>,
    __padding_end: [u8; 112],
}
impl USynesthesiaSpectrumAnalyzer {}
#[repr(transparent)]
pub struct FConstantQAnalyzer_OnConstantQResults {
    _opague: u8,
}
#[repr(transparent)]
pub struct FConstantQAnalyzer_OnLatestConstantQResults {
    _opague: u8,
}
#[repr(transparent)]
pub struct FLKFSAnalyzer_OnOverallLKFSResults {
    _opague: u8,
}
#[repr(transparent)]
pub struct FLKFSAnalyzer_OnPerChannelLKFSResults {
    _opague: u8,
}
#[repr(transparent)]
pub struct FLKFSAnalyzer_OnLatestOverallLKFSResults {
    _opague: u8,
}
#[repr(transparent)]
pub struct FLKFSAnalyzer_OnLatestPerChannelLKFSResults {
    _opague: u8,
}
#[repr(transparent)]
pub struct FLoudnessAnalyzer_OnOverallLoudnessResults {
    _opague: u8,
}
#[repr(transparent)]
pub struct FLoudnessAnalyzer_OnPerChannelLoudnessResults {
    _opague: u8,
}
#[repr(transparent)]
pub struct FLoudnessAnalyzer_OnLatestOverallLoudnessResults {
    _opague: u8,
}
#[repr(transparent)]
pub struct FLoudnessAnalyzer_OnLatestPerChannelLoudnessResults {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMeterAnalyzer_OnOverallMeterResults {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMeterAnalyzer_OnPerChannelMeterResults {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMeterAnalyzer_OnLatestOverallMeterResults {
    _opague: u8,
}
#[repr(transparent)]
pub struct FMeterAnalyzer_OnLatestPerChannelMeterResults {
    _opague: u8,
}
#[repr(transparent)]
pub struct FSynesthesiaSpectrumAnalyzer_OnSpectrumResults {
    _opague: u8,
}
#[repr(transparent)]
pub struct FSynesthesiaSpectrumAnalyzer_OnLatestSpectrumResults {
    _opague: u8,
}
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
#[repr(transparent)]
pub struct EMeterPeakType(pub u8);
impl EMeterPeakType {
    pub const MEAN_SQUARED: EMeterPeakType = EMeterPeakType(0);
    pub const ROOT_MEAN_SQUARED: EMeterPeakType = EMeterPeakType(1);
    pub const PEAK: EMeterPeakType = EMeterPeakType(2);
    pub const COUNT: EMeterPeakType = EMeterPeakType(3);
}
