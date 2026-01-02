#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FSynth1PatchCable {
    pub depth: f32,
    pub destination: ESynth1PatchDestination,
    __padding_end: [u8; 3],
}
impl FSynth1PatchCable {}
#[repr(C, align(4))]
pub struct FPatchId {
    __padding_end: [u8; 4],
}
impl FPatchId {}
#[repr(C, align(8))]
pub struct FEpicSynth1Patch {
    pub patch_source: ESynth1PatchSource,
    pub patch_cables: TArray<FSynth1PatchCable>,
}
impl FEpicSynth1Patch {}
#[repr(C, align(8))]
pub struct FModularSynthPreset {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub flags_8: u8,
    #[doc(hidden)]
    __padding_12: [u8; 3],
    pub osc1_type: ESynth1OscType,
    pub osc1_gain: f32,
    pub osc1_octave: f32,
    pub osc1_semitones: f32,
    pub osc1_cents: f32,
    pub osc1_pulse_width: f32,
    pub osc2_type: ESynth1OscType,
    pub osc2_gain: f32,
    pub osc2_octave: f32,
    pub osc2_semitones: f32,
    pub osc2_cents: f32,
    pub osc2_pulse_width: f32,
    pub portamento: f32,
    pub flags_64: u8,
    pub spread: f32,
    pub pan: f32,
    pub lfo1_frequency: f32,
    pub lfo1_gain: f32,
    pub lfo1_type: ESynthLFOType,
    pub lfo1_mode: ESynthLFOMode,
    pub lfo1_patch_type: ESynthLFOPatchType,
    pub lfo2_frequency: f32,
    pub lfo2_gain: f32,
    pub lfo2_type: ESynthLFOType,
    pub lfo2_mode: ESynthLFOMode,
    pub lfo2_patch_type: ESynthLFOPatchType,
    pub gain_db: f32,
    pub attack_time: f32,
    pub decay_time: f32,
    pub sustain_gain: f32,
    pub release_time: f32,
    pub mod_env_patch_type: ESynthModEnvPatch,
    pub mod_env_bias_patch_type: ESynthModEnvBiasPatch,
    #[doc(hidden)]
    __padding_124: [u8; 2],
    pub flags_124: u8,
    pub modulation_envelope_depth: f32,
    pub modulation_envelope_attack_time: f32,
    pub modulation_envelope_decay_time: f32,
    pub modulation_envelope_sustain_gain: f32,
    pub modulation_envelope_release_time: f32,
    pub flags_148: u8,
    pub filter_frequency: f32,
    pub filter_q: f32,
    pub filter_type: ESynthFilterType,
    pub filter_algorithm: ESynthFilterAlgorithm,
    #[doc(hidden)]
    __padding_164: [u8; 2],
    pub flags_164: u8,
    #[doc(hidden)]
    __padding_168: [u8; 3],
    pub stereo_delay_mode: ESynthStereoDelayMode,
    pub stereo_delay_time: f32,
    pub stereo_delay_feedback: f32,
    pub stereo_delay_wetlevel: f32,
    pub stereo_delay_ratio: f32,
    pub flags_188: u8,
    pub chorus_depth: f32,
    pub chorus_feedback: f32,
    pub chorus_frequency: f32,
    pub patches: TArray<FEpicSynth1Patch>,
}
impl FModularSynthPreset {}
#[repr(C, align(8))]
pub struct FModularSynthPresetBankEntry {
    pub preset_name: FString,
    pub preset: FModularSynthPreset,
}
impl FModularSynthPresetBankEntry {}
#[repr(C, align(4))]
pub struct FSourceEffectBitCrusherBaseSettings {
    pub sample_rate: f32,
    pub bit_depth: f32,
}
impl FSourceEffectBitCrusherBaseSettings {}
#[repr(C, align(8))]
pub struct FSourceEffectBitCrusherSettings {
    pub sample_rate_modulation: crate::bindings::engine::FSoundModulationDestinationSettings,
    pub bit_modulation: crate::bindings::engine::FSoundModulationDestinationSettings,
}
impl FSourceEffectBitCrusherSettings {}
#[repr(C, align(4))]
pub struct FSourceEffectChorusBaseSettings {
    pub depth: f32,
    pub frequency: f32,
    pub feedback: f32,
    pub wet_level: f32,
    pub dry_level: f32,
    pub spread: f32,
}
impl FSourceEffectChorusBaseSettings {}
#[repr(C, align(8))]
pub struct FSourceEffectChorusSettings {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub depth_modulation: crate::bindings::engine::FSoundModulationDestinationSettings,
    pub frequency_modulation: crate::bindings::engine::FSoundModulationDestinationSettings,
    pub feedback_modulation: crate::bindings::engine::FSoundModulationDestinationSettings,
    pub wet_modulation: crate::bindings::engine::FSoundModulationDestinationSettings,
    pub dry_modulation: crate::bindings::engine::FSoundModulationDestinationSettings,
    pub spread_modulation: crate::bindings::engine::FSoundModulationDestinationSettings,
}
impl FSourceEffectChorusSettings {}
#[repr(C, align(4))]
pub struct FSourceEffectConvolutionReverbSettings {
    #[doc(hidden)]
    __padding_4: [u8; 4],
    pub wet_volume_db: f32,
    pub dry_volume_db: f32,
    pub b_bypass: bool,
    __padding_end: [u8; 3],
}
impl FSourceEffectConvolutionReverbSettings {}
#[repr(C, align(4))]
pub struct FSourceEffectDynamicsProcessorSettings {
    pub dynamics_processor_type: ESourceEffectDynamicsProcessorType,
    pub peak_mode: ESourceEffectDynamicsPeakMode,
    pub look_ahead_msec: f32,
    pub attack_time_msec: f32,
    pub release_time_msec: f32,
    pub threshold_db: f32,
    pub ratio: f32,
    pub knee_bandwidth_db: f32,
    pub input_gain_db: f32,
    pub output_gain_db: f32,
    pub flags_36: u8,
    #[doc(hidden)]
    __padding_40: [u8; 3],
    pub flags_40: u8,
    __padding_end: [u8; 3],
}
impl FSourceEffectDynamicsProcessorSettings {}
#[repr(C, align(4))]
pub struct FSourceEffectEnvelopeFollowerSettings {
    pub attack_time: f32,
    pub release_time: f32,
    pub peak_mode: EEnvelopeFollowerPeakMode,
    pub b_is_analog_mode: bool,
    __padding_end: [u8; 2],
}
impl FSourceEffectEnvelopeFollowerSettings {}
#[repr(C, align(4))]
pub struct FSourceEffectEQBand {
    pub frequency: f32,
    pub bandwidth: f32,
    pub gain_db: f32,
    pub flags_12: u8,
    __padding_end: [u8; 3],
}
impl FSourceEffectEQBand {}
#[repr(C, align(8))]
pub struct FSourceEffectEQSettings {
    pub eq_bands: TArray<FSourceEffectEQBand>,
}
impl FSourceEffectEQSettings {}
#[repr(C, align(8))]
pub struct FSourceEffectFilterAudioBusModulationSettings {
    pub audio_bus: UPtr<crate::bindings::engine::UAudioBus>,
    pub envelope_follower_attack_time_msec: i32,
    pub envelope_follower_release_time_msec: i32,
    pub envelope_gain_multiplier: f32,
    #[doc(hidden)]
    __padding_24: [u8; 4],
    pub min_frequency_modulation: f32,
    pub max_frequency_modulation: f32,
    pub min_resonance_modulation: f32,
    pub max_resonance_modulation: f32,
}
impl FSourceEffectFilterAudioBusModulationSettings {}
#[repr(C, align(8))]
pub struct FSourceEffectFilterSettings {
    pub filter_circuit: ESourceEffectFilterCircuit,
    pub filter_type: ESourceEffectFilterType,
    pub cutoff_frequency: f32,
    pub filter_q: f32,
    pub audio_bus_modulation: TArray<FSourceEffectFilterAudioBusModulationSettings>,
}
impl FSourceEffectFilterSettings {}
#[repr(C, align(4))]
pub struct FSourceEffectFoldbackDistortionSettings {
    pub input_gain_db: f32,
    pub threshold_db: f32,
    pub output_gain_db: f32,
}
impl FSourceEffectFoldbackDistortionSettings {}
#[repr(C, align(4))]
pub struct FSourceEffectMidSideSpreaderSettings {
    pub spread_amount: f32,
    pub input_mode: EStereoChannelMode,
    pub output_mode: EStereoChannelMode,
    pub b_equal_power: bool,
    __padding_end: [u8; 1],
}
impl FSourceEffectMidSideSpreaderSettings {}
#[repr(C, align(4))]
pub struct FSourceEffectIndividualFilterSettings {
    pub filter_circuit: ESourceEffectMotionFilterCircuit,
    pub filter_type: ESourceEffectMotionFilterType,
    pub cutoff_frequency: f32,
    pub filter_q: f32,
}
impl FSourceEffectIndividualFilterSettings {}
#[repr(C, align(8))]
pub struct FSourceEffectMotionFilterModulationSettings {
    pub modulation_source: ESourceEffectMotionFilterModSource,
    pub modulation_input_range: crate::bindings::core_u_object::FVector2D,
    pub modulation_output_minimum_range: crate::bindings::core_u_object::FVector2D,
    pub modulation_output_maximum_range: crate::bindings::core_u_object::FVector2D,
    pub update_ease_ms: f32,
    __padding_end: [u8; 4],
}
impl FSourceEffectMotionFilterModulationSettings {}
#[repr(C, align(8))]
pub struct FSourceEffectMotionFilterSettings {
    pub motion_filter_topology: ESourceEffectMotionFilterTopology,
    pub motion_filter_mix: f32,
    pub filter_a_settings: FSourceEffectIndividualFilterSettings,
    pub filter_b_settings: FSourceEffectIndividualFilterSettings,
    pub modulation_mappings: TMap<
        ESourceEffectMotionFilterModDestination,
        FSourceEffectMotionFilterModulationSettings,
    >,
    pub dry_volume_db: f32,
    __padding_end: [u8; 4],
}
impl FSourceEffectMotionFilterSettings {}
#[repr(C, align(4))]
pub struct FSourceEffectPannerSettings {
    pub spread: f32,
    pub pan: f32,
}
impl FSourceEffectPannerSettings {}
#[repr(C, align(4))]
pub struct FSourceEffectPhaserSettings {
    pub wet_level: f32,
    pub frequency: f32,
    pub feedback: f32,
    pub lfo_type: EPhaserLFOType,
    pub use_quadrature_phase: bool,
    __padding_end: [u8; 2],
}
impl FSourceEffectPhaserSettings {}
#[repr(C, align(8))]
pub struct FSourceEffectRingModulationSettings {
    pub modulator_type: ERingModulatorTypeSourceEffect,
    pub frequency: f32,
    pub depth: f32,
    pub dry_level: f32,
    pub wet_level: f32,
    pub audio_bus_modulator: UPtr<crate::bindings::engine::UAudioBus>,
}
impl FSourceEffectRingModulationSettings {}
#[repr(C, align(4))]
pub struct FSourceEffectSimpleDelaySettings {
    pub speed_of_sound: f32,
    pub delay_amount: f32,
    pub dry_amount: f32,
    pub wet_amount: f32,
    pub feedback: f32,
    pub flags_20: u8,
    __padding_end: [u8; 3],
}
impl FSourceEffectSimpleDelaySettings {}
#[repr(C, align(4))]
pub struct FSourceEffectStereoDelaySettings {
    pub delay_mode: EStereoDelaySourceEffect,
    pub delay_time_msec: f32,
    pub feedback: f32,
    pub delay_ratio: f32,
    pub wet_level: f32,
    pub dry_level: f32,
    pub b_filter_enabled: bool,
    pub filter_type: EStereoDelayFiltertype,
    pub filter_frequency: f32,
    pub filter_q: f32,
}
impl FSourceEffectStereoDelaySettings {}
#[repr(C, align(4))]
pub struct FSourceEffectWaveShaperSettings {
    pub amount: f32,
    pub output_gain_db: f32,
}
impl FSourceEffectWaveShaperSettings {}
#[repr(C, align(8))]
pub struct FSubmixEffectConvolutionReverbSettings {
    #[doc(hidden)]
    __padding_4: [u8; 4],
    pub wet_volume_db: f32,
    pub dry_volume_db: f32,
    pub b_bypass: bool,
    pub b_mix_input_channel_format_to_impulse_response_format: bool,
    pub b_mix_reverb_output_to_output_channel_format: bool,
    pub surround_rear_channel_bleed_db: f32,
    pub b_invert_rear_channel_bleed_phase: bool,
    pub b_surround_rear_channel_flip: bool,
    __padding_end: [u8; 26],
}
impl FSubmixEffectConvolutionReverbSettings {}
#[repr(C, align(4))]
pub struct FSubmixEffectDelaySettings {
    pub maximum_delay_length: f32,
    pub interpolation_time: f32,
    pub delay_length: f32,
}
impl FSubmixEffectDelaySettings {}
#[repr(C, align(4))]
pub struct FSubmixEffectFilterSettings {
    pub filter_type: ESubmixFilterType,
    pub filter_algorithm: ESubmixFilterAlgorithm,
    pub filter_frequency: f32,
    pub filter_q: f32,
}
impl FSubmixEffectFilterSettings {}
#[repr(C, align(4))]
pub struct FSubmixEffectFlexiverbSettings {
    pub pre_delay: f32,
    pub decay_time: f32,
    pub room_dampening: f32,
    pub complexity: i32,
}
impl FSubmixEffectFlexiverbSettings {}
#[repr(C, align(4))]
pub struct FDynamicsBandSettings {
    pub crossover_top_frequency: f32,
    pub attack_time_msec: f32,
    pub release_time_msec: f32,
    pub threshold_db: f32,
    pub ratio: f32,
    pub knee_bandwidth_db: f32,
    pub input_gain_db: f32,
    pub output_gain_db: f32,
}
impl FDynamicsBandSettings {}
#[repr(C, align(8))]
pub struct FSubmixEffectMultibandCompressorSettings {
    pub dynamics_processor_type: crate::bindings::audio_mixer::ESubmixEffectDynamicsProcessorType,
    pub peak_mode: crate::bindings::audio_mixer::ESubmixEffectDynamicsPeakMode,
    pub link_mode: crate::bindings::audio_mixer::ESubmixEffectDynamicsChannelLinkMode,
    pub look_ahead_msec: f32,
    pub b_analog_mode: bool,
    pub b_four_pole: bool,
    pub b_bypass: bool,
    pub key_source: crate::bindings::audio_mixer::ESubmixEffectDynamicsKeySource,
    pub external_audio_bus: UPtr<crate::bindings::engine::UAudioBus>,
    pub external_submix: UPtr<crate::bindings::engine::USoundSubmix>,
    pub key_gain_db: f32,
    pub b_key_audition: bool,
    pub bands: TArray<FDynamicsBandSettings>,
}
impl FSubmixEffectMultibandCompressorSettings {}
#[repr(C, align(4))]
pub struct FSubmixEffectStereoDelaySettings {
    pub delay_mode: EStereoDelaySourceEffect,
    pub delay_time_msec: f32,
    pub feedback: f32,
    pub delay_ratio: f32,
    pub wet_level: f32,
    pub dry_level: f32,
    pub b_filter_enabled: bool,
    pub filter_type: EStereoDelayFiltertype,
    pub filter_frequency: f32,
    pub filter_q: f32,
}
impl FSubmixEffectStereoDelaySettings {}
#[repr(C, align(4))]
pub struct FSubmixEffectStereoToQuadSettings {
    pub b_flip_channels: bool,
    pub rear_channel_gain: f32,
}
impl FSubmixEffectStereoToQuadSettings {}
#[repr(C, align(4))]
pub struct FTapDelayInfo {
    pub tap_line_mode: ETapLineMode,
    pub delay_length: f32,
    pub gain: f32,
    pub output_channel: i32,
    pub pan_in_degrees: f32,
    __padding_end: [u8; 4],
}
impl FTapDelayInfo {}
#[repr(C, align(8))]
pub struct FSubmixEffectTapDelaySettings {
    pub maximum_delay_length: f32,
    pub interpolation_time: f32,
    pub taps: TArray<FTapDelayInfo>,
}
impl FSubmixEffectTapDelaySettings {}
#[repr(C, align(16))]
pub struct FSynth2DSliderStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub normal_thumb_image: crate::bindings::slate_core::FSlateBrush,
    pub disabled_thumb_image: crate::bindings::slate_core::FSlateBrush,
    pub normal_bar_image: crate::bindings::slate_core::FSlateBrush,
    pub disabled_bar_image: crate::bindings::slate_core::FSlateBrush,
    pub background_image: crate::bindings::slate_core::FSlateBrush,
    pub bar_thickness: f32,
    __padding_end: [u8; 12],
}
impl FSynth2DSliderStyle {}
#[repr(C, align(16))]
pub struct FSynthKnobStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub large_knob: crate::bindings::slate_core::FSlateBrush,
    pub large_knob_overlay: crate::bindings::slate_core::FSlateBrush,
    pub medium_knob: crate::bindings::slate_core::FSlateBrush,
    pub medium_knob_overlay: crate::bindings::slate_core::FSlateBrush,
    pub min_value_angle: f32,
    pub max_value_angle: f32,
    pub knob_size: ESynthKnobSize,
    __padding_end: [u8; 7],
}
impl FSynthKnobStyle {}
#[repr(C, align(8))]
pub struct FSynthSlateStyle {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub size_type: ESynthSlateSizeType,
    pub color_style: ESynthSlateColorStyle,
    __padding_end: [u8; 6],
}
impl FSynthSlateStyle {}
#[repr(C, align(8))]
pub struct UAudioImpulseResponse {
    #[doc(hidden)]
    __padding_72: [u8; 72],
    pub normalization_volume_db: f32,
    pub b_true_stereo: bool,
    __padding_end: [u8; 59],
}
impl UAudioImpulseResponse {}
#[repr(C, align(8))]
pub struct UModularSynthPresetBank {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub presets: TArray<FModularSynthPresetBankEntry>,
}
impl UModularSynthPresetBank {}
#[repr(C, align(8))]
pub struct UModularSynthLibrary {
    __padding_end: [u8; 48],
}
impl UModularSynthLibrary {}
#[repr(C, align(16))]
pub struct UModularSynthComponent {
    #[doc(hidden)]
    __padding_2384: [u8; 2384],
    pub voice_count: i32,
    __padding_end: [u8; 1788],
}
impl UModularSynthComponent {}
#[repr(C, align(8))]
pub struct USourceEffectBitCrusherPreset {
    #[doc(hidden)]
    __padding_344: [u8; 344],
    pub settings: FSourceEffectBitCrusherSettings,
}
impl USourceEffectBitCrusherPreset {}
#[repr(C, align(8))]
pub struct USourceEffectChorusPreset {
    #[doc(hidden)]
    __padding_752: [u8; 752],
    pub settings: FSourceEffectChorusSettings,
}
impl USourceEffectChorusPreset {}
#[repr(C, align(8))]
pub struct USourceEffectConvolutionReverbPreset {
    #[doc(hidden)]
    __padding_112: [u8; 112],
    pub impulse_response: UPtr<UAudioImpulseResponse>,
    pub settings: FSourceEffectConvolutionReverbSettings,
    pub block_size: ESubmixEffectConvolutionReverbBlockSize,
    pub b_enable_hardware_acceleration: bool,
    __padding_end: [u8; 142],
}
impl USourceEffectConvolutionReverbPreset {}
#[repr(C, align(8))]
pub struct USourceEffectDynamicsProcessorPreset {
    #[doc(hidden)]
    __padding_196: [u8; 196],
    pub settings: FSourceEffectDynamicsProcessorSettings,
}
impl USourceEffectDynamicsProcessorPreset {}
#[repr(C, align(8))]
pub struct UEnvelopeFollowerListener {
    __padding_end: [u8; 280],
}
impl UEnvelopeFollowerListener {}
#[repr(C, align(8))]
pub struct USourceEffectEnvelopeFollowerPreset {
    #[doc(hidden)]
    __padding_164: [u8; 164],
    pub settings: FSourceEffectEnvelopeFollowerSettings,
}
impl USourceEffectEnvelopeFollowerPreset {}
#[repr(C, align(8))]
pub struct USourceEffectEQPreset {
    #[doc(hidden)]
    __padding_168: [u8; 168],
    pub settings: FSourceEffectEQSettings,
}
impl USourceEffectEQPreset {}
#[repr(C, align(8))]
pub struct USourceEffectFilterPreset {
    #[doc(hidden)]
    __padding_184: [u8; 184],
    pub settings: FSourceEffectFilterSettings,
}
impl USourceEffectFilterPreset {}
#[repr(C, align(8))]
pub struct USourceEffectFoldbackDistortionPreset {
    #[doc(hidden)]
    __padding_164: [u8; 164],
    pub settings: FSourceEffectFoldbackDistortionSettings,
}
impl USourceEffectFoldbackDistortionPreset {}
#[repr(C, align(8))]
pub struct USourceEffectMidSideSpreaderPreset {
    #[doc(hidden)]
    __padding_160: [u8; 160],
    pub settings: FSourceEffectMidSideSpreaderSettings,
}
impl USourceEffectMidSideSpreaderPreset {}
#[repr(C, align(8))]
pub struct USourceEffectMotionFilterPreset {
    #[doc(hidden)]
    __padding_272: [u8; 272],
    pub settings: FSourceEffectMotionFilterSettings,
}
impl USourceEffectMotionFilterPreset {}
#[repr(C, align(8))]
pub struct USourceEffectPannerPreset {
    #[doc(hidden)]
    __padding_160: [u8; 160],
    pub settings: FSourceEffectPannerSettings,
}
impl USourceEffectPannerPreset {}
#[repr(C, align(8))]
pub struct USourceEffectPhaserPreset {
    #[doc(hidden)]
    __padding_168: [u8; 168],
    pub settings: FSourceEffectPhaserSettings,
}
impl USourceEffectPhaserPreset {}
#[repr(C, align(8))]
pub struct USourceEffectRingModulationPreset {
    #[doc(hidden)]
    __padding_184: [u8; 184],
    pub settings: FSourceEffectRingModulationSettings,
}
impl USourceEffectRingModulationPreset {}
#[repr(C, align(8))]
pub struct USourceEffectSimpleDelayPreset {
    #[doc(hidden)]
    __padding_176: [u8; 176],
    pub settings: FSourceEffectSimpleDelaySettings,
}
impl USourceEffectSimpleDelayPreset {}
#[repr(C, align(8))]
pub struct USourceEffectStereoDelayPreset {
    #[doc(hidden)]
    __padding_188: [u8; 188],
    pub settings: FSourceEffectStereoDelaySettings,
}
impl USourceEffectStereoDelayPreset {}
#[repr(C, align(8))]
pub struct USourceEffectWaveShaperPreset {
    #[doc(hidden)]
    __padding_160: [u8; 160],
    pub settings: FSourceEffectWaveShaperSettings,
}
impl USourceEffectWaveShaperPreset {}
#[repr(C, align(8))]
pub struct USubmixEffectConvolutionReverbPreset {
    #[doc(hidden)]
    __padding_112: [u8; 112],
    pub impulse_response: UPtr<UAudioImpulseResponse>,
    pub settings: FSubmixEffectConvolutionReverbSettings,
    pub block_size: ESubmixEffectConvolutionReverbBlockSize,
    pub b_enable_hardware_acceleration: bool,
    __padding_end: [u8; 174],
}
impl USubmixEffectConvolutionReverbPreset {}
#[repr(C, align(8))]
pub struct USubmixEffectDelayStatics {
    __padding_end: [u8; 48],
}
impl USubmixEffectDelayStatics {}
#[repr(C, align(8))]
pub struct USubmixEffectDelayPreset {
    #[doc(hidden)]
    __padding_164: [u8; 164],
    pub settings: FSubmixEffectDelaySettings,
    __padding_end: [u8; 16],
}
impl USubmixEffectDelayPreset {}
#[repr(C, align(8))]
pub struct USubmixEffectFilterPreset {
    #[doc(hidden)]
    __padding_164: [u8; 164],
    pub settings: FSubmixEffectFilterSettings,
}
impl USubmixEffectFilterPreset {}
#[repr(C, align(8))]
pub struct USubmixEffectFlexiverbPreset {
    #[doc(hidden)]
    __padding_168: [u8; 168],
    pub settings: FSubmixEffectFlexiverbSettings,
}
impl USubmixEffectFlexiverbPreset {}
#[repr(C, align(8))]
pub struct USubmixEffectMultibandCompressorPreset {
    #[doc(hidden)]
    __padding_208: [u8; 208],
    pub settings: FSubmixEffectMultibandCompressorSettings,
}
impl USubmixEffectMultibandCompressorPreset {}
#[repr(C, align(8))]
pub struct USubmixEffectStereoDelayPreset {
    #[doc(hidden)]
    __padding_188: [u8; 188],
    pub settings: FSubmixEffectStereoDelaySettings,
}
impl USubmixEffectStereoDelayPreset {}
#[repr(C, align(8))]
pub struct USubmixEffectStereoToQuadPreset {
    #[doc(hidden)]
    __padding_160: [u8; 160],
    pub settings: FSubmixEffectStereoToQuadSettings,
}
impl USubmixEffectStereoToQuadPreset {}
#[repr(C, align(8))]
pub struct USubmixEffectTapDelayPreset {
    #[doc(hidden)]
    __padding_176: [u8; 176],
    pub settings: FSubmixEffectTapDelaySettings,
    __padding_end: [u8; 24],
}
impl USubmixEffectTapDelayPreset {}
#[repr(C, align(16))]
pub struct UGranularSynth {
    __padding_end: [u8; 3392],
}
impl UGranularSynth {}
#[repr(C, align(8))]
pub struct UMonoWaveTableSynthPreset {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub preset_name: FString,
    pub flags_64: u8,
    pub lock_keyframes_to_grid: i32,
    pub wave_table_resolution: i32,
    pub wave_table: TArray<crate::bindings::engine::FRuntimeFloatCurve>,
    pub flags_96: u8,
    __padding_end: [u8; 279],
}
impl UMonoWaveTableSynthPreset {}
#[repr(C, align(16))]
pub struct USynthComponentMonoWaveTable {
    __padding_end: [u8; 4272],
}
impl USynthComponentMonoWaveTable {}
#[repr(C, align(16))]
pub struct USynthComponentToneGenerator {
    #[doc(hidden)]
    __padding_2384: [u8; 2384],
    pub frequency: f32,
    pub volume: f32,
    pub distance_attenuation_curve: crate::bindings::engine::FRuntimeFloatCurve,
    pub distance_range: crate::bindings::core_u_object::FVector2D,
    pub attenuation_db_at_max_range: f32,
    __padding_end: [u8; 76],
}
impl USynthComponentToneGenerator {}
#[repr(C, align(16))]
pub struct USynthSamplePlayer {
    #[doc(hidden)]
    __padding_2384: [u8; 2384],
    pub sound_wave: UPtr<crate::bindings::engine::USoundWave>,
    __padding_end: [u8; 312],
}
impl USynthSamplePlayer {}
#[repr(C, align(8))]
pub struct USynthesisUtilitiesBlueprintFunctionLibrary {
    __padding_end: [u8; 48],
}
impl USynthesisUtilitiesBlueprintFunctionLibrary {}
#[repr(C, align(16))]
pub struct USynth2DSlider {
    #[doc(hidden)]
    __padding_768: [u8; 768],
    pub widget_style: FSynth2DSliderStyle,
    pub slider_handle_color: crate::bindings::core_u_object::FLinearColor,
    pub indent_handle: bool,
    pub locked: bool,
    pub step_size: f32,
    pub is_focusable: bool,
    __padding_end: [u8; 183],
}
impl USynth2DSlider {}
#[repr(C, align(16))]
pub struct USynthKnob {
    #[doc(hidden)]
    __padding_700: [u8; 700],
    pub step_size: f32,
    pub mouse_speed: f32,
    pub mouse_fine_tune_speed: f32,
    pub flags_712: u8,
    pub parameter_name: FText,
    pub parameter_units: FText,
    #[doc(hidden)]
    __padding_784: [u8; 32],
    pub widget_style: FSynthKnobStyle,
    pub locked: bool,
    pub is_focusable: bool,
    __padding_end: [u8; 158],
}
impl USynthKnob {}
#[repr(C, align(8))]
pub struct FEnvelopeFollowerListener_OnEnvelopeFollowerUpdate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSynthComponentMonoWaveTable_OnTableAltered {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSynthComponentMonoWaveTable_OnNumTablesChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSynthSamplePlayer_OnSampleLoaded {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSynthSamplePlayer_OnSamplePlaybackProgress {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSynth2DSlider_ValueXDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSynth2DSlider_ValueYDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSynth2DSlider_OnMouseCaptureBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSynth2DSlider_OnMouseCaptureEnd {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSynth2DSlider_OnControllerCaptureBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSynth2DSlider_OnControllerCaptureEnd {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSynth2DSlider_OnValueChangedX {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSynth2DSlider_OnValueChangedY {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSynthKnob_ValueDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSynthKnob_OnMouseCaptureBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSynthKnob_OnMouseCaptureEnd {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSynthKnob_OnControllerCaptureBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSynthKnob_OnControllerCaptureEnd {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSynthKnob_OnValueChanged {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ESynth1PatchDestination(pub u8);
impl ESynth1PatchDestination {
    pub const OSC1_GAIN: ESynth1PatchDestination = ESynth1PatchDestination(0);
    pub const OSC1_FREQUENCY: ESynth1PatchDestination = ESynth1PatchDestination(1);
    pub const OSC1_PULSEWIDTH: ESynth1PatchDestination = ESynth1PatchDestination(2);
    pub const OSC2_GAIN: ESynth1PatchDestination = ESynth1PatchDestination(3);
    pub const OSC2_FREQUENCY: ESynth1PatchDestination = ESynth1PatchDestination(4);
    pub const OSC2_PULSEWIDTH: ESynth1PatchDestination = ESynth1PatchDestination(5);
    pub const FILTER_FREQUENCY: ESynth1PatchDestination = ESynth1PatchDestination(6);
    pub const FILTER_Q: ESynth1PatchDestination = ESynth1PatchDestination(7);
    pub const GAIN: ESynth1PatchDestination = ESynth1PatchDestination(8);
    pub const PAN: ESynth1PatchDestination = ESynth1PatchDestination(9);
    pub const LFO1_FREQUENCY: ESynth1PatchDestination = ESynth1PatchDestination(10);
    pub const LFO1_GAIN: ESynth1PatchDestination = ESynth1PatchDestination(11);
    pub const LFO2_FREQUENCY: ESynth1PatchDestination = ESynth1PatchDestination(12);
    pub const LFO2_GAIN: ESynth1PatchDestination = ESynth1PatchDestination(13);
    pub const COUNT: ESynth1PatchDestination = ESynth1PatchDestination(14);
}
#[repr(transparent)]
pub struct ESynth1PatchSource(pub u8);
impl ESynth1PatchSource {
    pub const LFO1: ESynth1PatchSource = ESynth1PatchSource(0);
    pub const LFO2: ESynth1PatchSource = ESynth1PatchSource(1);
    pub const ENVELOPE: ESynth1PatchSource = ESynth1PatchSource(2);
    pub const BIAS_ENVELOPE: ESynth1PatchSource = ESynth1PatchSource(3);
    pub const COUNT: ESynth1PatchSource = ESynth1PatchSource(4);
}
#[repr(transparent)]
pub struct ESynth1OscType(pub u8);
impl ESynth1OscType {
    pub const SINE: ESynth1OscType = ESynth1OscType(0);
    pub const SAW: ESynth1OscType = ESynth1OscType(1);
    pub const TRIANGLE: ESynth1OscType = ESynth1OscType(2);
    pub const SQUARE: ESynth1OscType = ESynth1OscType(3);
    pub const NOISE: ESynth1OscType = ESynth1OscType(4);
    pub const COUNT: ESynth1OscType = ESynth1OscType(5);
}
#[repr(transparent)]
pub struct ESynthLFOType(pub u8);
impl ESynthLFOType {
    pub const SINE: ESynthLFOType = ESynthLFOType(0);
    pub const UP_SAW: ESynthLFOType = ESynthLFOType(1);
    pub const DOWN_SAW: ESynthLFOType = ESynthLFOType(2);
    pub const SQUARE: ESynthLFOType = ESynthLFOType(3);
    pub const TRIANGLE: ESynthLFOType = ESynthLFOType(4);
    pub const EXPONENTIAL: ESynthLFOType = ESynthLFOType(5);
    pub const RANDOM_SAMPLE_HOLD: ESynthLFOType = ESynthLFOType(6);
    pub const COUNT: ESynthLFOType = ESynthLFOType(7);
}
#[repr(transparent)]
pub struct ESynthLFOMode(pub u8);
impl ESynthLFOMode {
    pub const SYNC: ESynthLFOMode = ESynthLFOMode(0);
    pub const ONE_SHOT: ESynthLFOMode = ESynthLFOMode(1);
    pub const FREE: ESynthLFOMode = ESynthLFOMode(2);
    pub const COUNT: ESynthLFOMode = ESynthLFOMode(3);
}
#[repr(transparent)]
pub struct ESynthLFOPatchType(pub u8);
impl ESynthLFOPatchType {
    pub const PATCH_TO_NONE: ESynthLFOPatchType = ESynthLFOPatchType(0);
    pub const PATCH_TO_GAIN: ESynthLFOPatchType = ESynthLFOPatchType(1);
    pub const PATCH_TO_OSC_FREQ: ESynthLFOPatchType = ESynthLFOPatchType(2);
    pub const PATCH_TO_FILTER_FREQ: ESynthLFOPatchType = ESynthLFOPatchType(3);
    pub const PATCH_TO_FILTER_Q: ESynthLFOPatchType = ESynthLFOPatchType(4);
    pub const PATCH_TO_OSC_PULSE_WIDTH: ESynthLFOPatchType = ESynthLFOPatchType(5);
    pub const PATCH_TO_OSC_PAN: ESynthLFOPatchType = ESynthLFOPatchType(6);
    pub const PATCH_LFO1_TO_LFO2_FREQUENCY: ESynthLFOPatchType = ESynthLFOPatchType(7);
    pub const PATCH_LFO1_TO_LFO2_GAIN: ESynthLFOPatchType = ESynthLFOPatchType(8);
    pub const COUNT: ESynthLFOPatchType = ESynthLFOPatchType(9);
}
#[repr(transparent)]
pub struct ESynthModEnvPatch(pub u8);
impl ESynthModEnvPatch {
    pub const PATCH_TO_NONE: ESynthModEnvPatch = ESynthModEnvPatch(0);
    pub const PATCH_TO_OSC_FREQ: ESynthModEnvPatch = ESynthModEnvPatch(1);
    pub const PATCH_TO_FILTER_FREQ: ESynthModEnvPatch = ESynthModEnvPatch(2);
    pub const PATCH_TO_FILTER_Q: ESynthModEnvPatch = ESynthModEnvPatch(3);
    pub const PATCH_TO_LFO1_GAIN: ESynthModEnvPatch = ESynthModEnvPatch(4);
    pub const PATCH_TO_LFO2_GAIN: ESynthModEnvPatch = ESynthModEnvPatch(5);
    pub const PATCH_TO_LFO1_FREQ: ESynthModEnvPatch = ESynthModEnvPatch(6);
    pub const PATCH_TO_LFO2_FREQ: ESynthModEnvPatch = ESynthModEnvPatch(7);
    pub const COUNT: ESynthModEnvPatch = ESynthModEnvPatch(8);
}
#[repr(transparent)]
pub struct ESynthModEnvBiasPatch(pub u8);
impl ESynthModEnvBiasPatch {
    pub const PATCH_TO_NONE: ESynthModEnvBiasPatch = ESynthModEnvBiasPatch(0);
    pub const PATCH_TO_OSC_FREQ: ESynthModEnvBiasPatch = ESynthModEnvBiasPatch(1);
    pub const PATCH_TO_FILTER_FREQ: ESynthModEnvBiasPatch = ESynthModEnvBiasPatch(2);
    pub const PATCH_TO_FILTER_Q: ESynthModEnvBiasPatch = ESynthModEnvBiasPatch(3);
    pub const PATCH_TO_LFO1_GAIN: ESynthModEnvBiasPatch = ESynthModEnvBiasPatch(4);
    pub const PATCH_TO_LFO2_GAIN: ESynthModEnvBiasPatch = ESynthModEnvBiasPatch(5);
    pub const PATCH_TO_LFO1_FREQ: ESynthModEnvBiasPatch = ESynthModEnvBiasPatch(6);
    pub const PATCH_TO_LFO2_FREQ: ESynthModEnvBiasPatch = ESynthModEnvBiasPatch(7);
    pub const COUNT: ESynthModEnvBiasPatch = ESynthModEnvBiasPatch(8);
}
#[repr(transparent)]
pub struct ESynthFilterType(pub u8);
impl ESynthFilterType {
    pub const LOW_PASS: ESynthFilterType = ESynthFilterType(0);
    pub const HIGH_PASS: ESynthFilterType = ESynthFilterType(1);
    pub const BAND_PASS: ESynthFilterType = ESynthFilterType(2);
    pub const BAND_STOP: ESynthFilterType = ESynthFilterType(3);
    pub const COUNT: ESynthFilterType = ESynthFilterType(4);
}
#[repr(transparent)]
pub struct ESynthFilterAlgorithm(pub u8);
impl ESynthFilterAlgorithm {
    pub const ONE_POLE: ESynthFilterAlgorithm = ESynthFilterAlgorithm(0);
    pub const STATE_VARIABLE: ESynthFilterAlgorithm = ESynthFilterAlgorithm(1);
    pub const LADDER: ESynthFilterAlgorithm = ESynthFilterAlgorithm(2);
    pub const COUNT: ESynthFilterAlgorithm = ESynthFilterAlgorithm(3);
}
#[repr(transparent)]
pub struct ESynthStereoDelayMode(pub u8);
impl ESynthStereoDelayMode {
    pub const NORMAL: ESynthStereoDelayMode = ESynthStereoDelayMode(0);
    pub const CROSS: ESynthStereoDelayMode = ESynthStereoDelayMode(1);
    pub const PING_PONG: ESynthStereoDelayMode = ESynthStereoDelayMode(2);
    pub const COUNT: ESynthStereoDelayMode = ESynthStereoDelayMode(3);
}
#[repr(transparent)]
pub struct ESourceEffectDynamicsProcessorType(pub u8);
impl ESourceEffectDynamicsProcessorType {
    pub const COMPRESSOR: ESourceEffectDynamicsProcessorType = ESourceEffectDynamicsProcessorType(
        0,
    );
    pub const LIMITER: ESourceEffectDynamicsProcessorType = ESourceEffectDynamicsProcessorType(
        1,
    );
    pub const EXPANDER: ESourceEffectDynamicsProcessorType = ESourceEffectDynamicsProcessorType(
        2,
    );
    pub const GATE: ESourceEffectDynamicsProcessorType = ESourceEffectDynamicsProcessorType(
        3,
    );
    pub const UPWARDS_COMPRESSOR: ESourceEffectDynamicsProcessorType = ESourceEffectDynamicsProcessorType(
        4,
    );
    pub const COUNT: ESourceEffectDynamicsProcessorType = ESourceEffectDynamicsProcessorType(
        5,
    );
}
#[repr(transparent)]
pub struct ESourceEffectDynamicsPeakMode(pub u8);
impl ESourceEffectDynamicsPeakMode {
    pub const MEAN_SQUARED: ESourceEffectDynamicsPeakMode = ESourceEffectDynamicsPeakMode(
        0,
    );
    pub const ROOT_MEAN_SQUARED: ESourceEffectDynamicsPeakMode = ESourceEffectDynamicsPeakMode(
        1,
    );
    pub const PEAK: ESourceEffectDynamicsPeakMode = ESourceEffectDynamicsPeakMode(2);
    pub const COUNT: ESourceEffectDynamicsPeakMode = ESourceEffectDynamicsPeakMode(3);
}
#[repr(transparent)]
pub struct EEnvelopeFollowerPeakMode(pub u8);
impl EEnvelopeFollowerPeakMode {
    pub const MEAN_SQUARED: EEnvelopeFollowerPeakMode = EEnvelopeFollowerPeakMode(0);
    pub const ROOT_MEAN_SQUARED: EEnvelopeFollowerPeakMode = EEnvelopeFollowerPeakMode(
        1,
    );
    pub const PEAK: EEnvelopeFollowerPeakMode = EEnvelopeFollowerPeakMode(2);
    pub const COUNT: EEnvelopeFollowerPeakMode = EEnvelopeFollowerPeakMode(3);
}
#[repr(transparent)]
pub struct ESourceEffectFilterParam(pub u8);
impl ESourceEffectFilterParam {
    pub const FILTER_FREQUENCY: ESourceEffectFilterParam = ESourceEffectFilterParam(0);
    pub const FILTER_RESONANCE: ESourceEffectFilterParam = ESourceEffectFilterParam(1);
    pub const COUNT: ESourceEffectFilterParam = ESourceEffectFilterParam(2);
}
#[repr(transparent)]
pub struct ESourceEffectFilterCircuit(pub u8);
impl ESourceEffectFilterCircuit {
    pub const ONE_POLE: ESourceEffectFilterCircuit = ESourceEffectFilterCircuit(0);
    pub const STATE_VARIABLE: ESourceEffectFilterCircuit = ESourceEffectFilterCircuit(1);
    pub const LADDER: ESourceEffectFilterCircuit = ESourceEffectFilterCircuit(2);
    pub const COUNT: ESourceEffectFilterCircuit = ESourceEffectFilterCircuit(3);
}
#[repr(transparent)]
pub struct ESourceEffectFilterType(pub u8);
impl ESourceEffectFilterType {
    pub const LOW_PASS: ESourceEffectFilterType = ESourceEffectFilterType(0);
    pub const HIGH_PASS: ESourceEffectFilterType = ESourceEffectFilterType(1);
    pub const BAND_PASS: ESourceEffectFilterType = ESourceEffectFilterType(2);
    pub const BAND_STOP: ESourceEffectFilterType = ESourceEffectFilterType(3);
    pub const COUNT: ESourceEffectFilterType = ESourceEffectFilterType(4);
}
#[repr(transparent)]
pub struct EStereoChannelMode(pub u8);
impl EStereoChannelMode {
    pub const MID_SIDE: EStereoChannelMode = EStereoChannelMode(0);
    pub const LEFT_RIGHT: EStereoChannelMode = EStereoChannelMode(1);
    pub const COUNT: EStereoChannelMode = EStereoChannelMode(2);
}
#[repr(transparent)]
pub struct ESourceEffectMotionFilterCircuit(pub u8);
impl ESourceEffectMotionFilterCircuit {
    pub const ONE_POLE: ESourceEffectMotionFilterCircuit = ESourceEffectMotionFilterCircuit(
        0,
    );
    pub const STATE_VARIABLE: ESourceEffectMotionFilterCircuit = ESourceEffectMotionFilterCircuit(
        1,
    );
    pub const LADDER: ESourceEffectMotionFilterCircuit = ESourceEffectMotionFilterCircuit(
        2,
    );
    pub const COUNT: ESourceEffectMotionFilterCircuit = ESourceEffectMotionFilterCircuit(
        3,
    );
}
#[repr(transparent)]
pub struct ESourceEffectMotionFilterType(pub u8);
impl ESourceEffectMotionFilterType {
    pub const LOW_PASS: ESourceEffectMotionFilterType = ESourceEffectMotionFilterType(0);
    pub const HIGH_PASS: ESourceEffectMotionFilterType = ESourceEffectMotionFilterType(
        1,
    );
    pub const BAND_PASS: ESourceEffectMotionFilterType = ESourceEffectMotionFilterType(
        2,
    );
    pub const BAND_STOP: ESourceEffectMotionFilterType = ESourceEffectMotionFilterType(
        3,
    );
    pub const COUNT: ESourceEffectMotionFilterType = ESourceEffectMotionFilterType(4);
}
#[repr(transparent)]
pub struct ESourceEffectMotionFilterModSource(pub u8);
impl ESourceEffectMotionFilterModSource {
    pub const DISTANCE_FROM_LISTENER: ESourceEffectMotionFilterModSource = ESourceEffectMotionFilterModSource(
        0,
    );
    pub const SPEED_RELATIVE_TO_LISTENER: ESourceEffectMotionFilterModSource = ESourceEffectMotionFilterModSource(
        1,
    );
    pub const SPEED_OF_SOURCE_EMITTER: ESourceEffectMotionFilterModSource = ESourceEffectMotionFilterModSource(
        2,
    );
    pub const SPEED_OF_LISTENER: ESourceEffectMotionFilterModSource = ESourceEffectMotionFilterModSource(
        3,
    );
    pub const SPEED_OF_ANGLE_DELTA: ESourceEffectMotionFilterModSource = ESourceEffectMotionFilterModSource(
        4,
    );
    pub const COUNT: ESourceEffectMotionFilterModSource = ESourceEffectMotionFilterModSource(
        5,
    );
}
#[repr(transparent)]
pub struct ESourceEffectMotionFilterTopology(pub u8);
impl ESourceEffectMotionFilterTopology {
    pub const SERIAL_MODE: ESourceEffectMotionFilterTopology = ESourceEffectMotionFilterTopology(
        0,
    );
    pub const PARALLEL_MODE: ESourceEffectMotionFilterTopology = ESourceEffectMotionFilterTopology(
        1,
    );
    pub const COUNT: ESourceEffectMotionFilterTopology = ESourceEffectMotionFilterTopology(
        2,
    );
}
#[repr(transparent)]
pub struct ESourceEffectMotionFilterModDestination(pub u8);
impl ESourceEffectMotionFilterModDestination {
    pub const FILTER_A_CUTOFF_FREQUENCY: ESourceEffectMotionFilterModDestination = ESourceEffectMotionFilterModDestination(
        0,
    );
    pub const FILTER_A_RESONANCE: ESourceEffectMotionFilterModDestination = ESourceEffectMotionFilterModDestination(
        1,
    );
    pub const FILTER_A_OUTPUT_VOLUME_DB: ESourceEffectMotionFilterModDestination = ESourceEffectMotionFilterModDestination(
        2,
    );
    pub const FILTER_B_CUTOFF_FREQUENCY: ESourceEffectMotionFilterModDestination = ESourceEffectMotionFilterModDestination(
        3,
    );
    pub const FILTER_B_RESONANCE: ESourceEffectMotionFilterModDestination = ESourceEffectMotionFilterModDestination(
        4,
    );
    pub const FILTER_B_OUTPUT_VOLUME_DB: ESourceEffectMotionFilterModDestination = ESourceEffectMotionFilterModDestination(
        5,
    );
    pub const FILTER_MIX: ESourceEffectMotionFilterModDestination = ESourceEffectMotionFilterModDestination(
        6,
    );
    pub const COUNT: ESourceEffectMotionFilterModDestination = ESourceEffectMotionFilterModDestination(
        7,
    );
}
#[repr(transparent)]
pub struct EPhaserLFOType(pub u8);
impl EPhaserLFOType {
    pub const SINE: EPhaserLFOType = EPhaserLFOType(0);
    pub const UP_SAW: EPhaserLFOType = EPhaserLFOType(1);
    pub const DOWN_SAW: EPhaserLFOType = EPhaserLFOType(2);
    pub const SQUARE: EPhaserLFOType = EPhaserLFOType(3);
    pub const TRIANGLE: EPhaserLFOType = EPhaserLFOType(4);
    pub const EXPONENTIAL: EPhaserLFOType = EPhaserLFOType(5);
    pub const RANDOM_SAMPLE_HOLD: EPhaserLFOType = EPhaserLFOType(6);
    pub const COUNT: EPhaserLFOType = EPhaserLFOType(7);
}
#[repr(transparent)]
pub struct ERingModulatorTypeSourceEffect(pub u8);
impl ERingModulatorTypeSourceEffect {
    pub const SINE: ERingModulatorTypeSourceEffect = ERingModulatorTypeSourceEffect(0);
    pub const SAW: ERingModulatorTypeSourceEffect = ERingModulatorTypeSourceEffect(1);
    pub const TRIANGLE: ERingModulatorTypeSourceEffect = ERingModulatorTypeSourceEffect(
        2,
    );
    pub const SQUARE: ERingModulatorTypeSourceEffect = ERingModulatorTypeSourceEffect(3);
    pub const COUNT: ERingModulatorTypeSourceEffect = ERingModulatorTypeSourceEffect(4);
}
#[repr(transparent)]
pub struct EStereoDelaySourceEffect(pub u8);
impl EStereoDelaySourceEffect {
    pub const NORMAL: EStereoDelaySourceEffect = EStereoDelaySourceEffect(0);
    pub const CROSS: EStereoDelaySourceEffect = EStereoDelaySourceEffect(1);
    pub const PING_PONG: EStereoDelaySourceEffect = EStereoDelaySourceEffect(2);
    pub const COUNT: EStereoDelaySourceEffect = EStereoDelaySourceEffect(3);
}
#[repr(transparent)]
pub struct EStereoDelayFiltertype(pub u8);
impl EStereoDelayFiltertype {
    pub const LOWPASS: EStereoDelayFiltertype = EStereoDelayFiltertype(0);
    pub const HIGHPASS: EStereoDelayFiltertype = EStereoDelayFiltertype(1);
    pub const BANDPASS: EStereoDelayFiltertype = EStereoDelayFiltertype(2);
    pub const NOTCH: EStereoDelayFiltertype = EStereoDelayFiltertype(3);
    pub const COUNT: EStereoDelayFiltertype = EStereoDelayFiltertype(4);
}
#[repr(transparent)]
pub struct ESubmixFilterType(pub u8);
impl ESubmixFilterType {
    pub const LOW_PASS: ESubmixFilterType = ESubmixFilterType(0);
    pub const HIGH_PASS: ESubmixFilterType = ESubmixFilterType(1);
    pub const BAND_PASS: ESubmixFilterType = ESubmixFilterType(2);
    pub const BAND_STOP: ESubmixFilterType = ESubmixFilterType(3);
    pub const COUNT: ESubmixFilterType = ESubmixFilterType(4);
}
#[repr(transparent)]
pub struct ESubmixFilterAlgorithm(pub u8);
impl ESubmixFilterAlgorithm {
    pub const ONE_POLE: ESubmixFilterAlgorithm = ESubmixFilterAlgorithm(0);
    pub const STATE_VARIABLE: ESubmixFilterAlgorithm = ESubmixFilterAlgorithm(1);
    pub const LADDER: ESubmixFilterAlgorithm = ESubmixFilterAlgorithm(2);
    pub const COUNT: ESubmixFilterAlgorithm = ESubmixFilterAlgorithm(3);
}
#[repr(transparent)]
pub struct ETapLineMode(pub u8);
impl ETapLineMode {
    pub const SEND_TO_CHANNEL: ETapLineMode = ETapLineMode(0);
    pub const PANNING: ETapLineMode = ETapLineMode(1);
    pub const DISABLED: ETapLineMode = ETapLineMode(2);
}
#[repr(transparent)]
pub struct ESynthKnobSize(pub u8);
impl ESynthKnobSize {
    pub const MEDIUM: ESynthKnobSize = ESynthKnobSize(0);
    pub const LARGE: ESynthKnobSize = ESynthKnobSize(1);
    pub const COUNT: ESynthKnobSize = ESynthKnobSize(2);
}
#[repr(transparent)]
pub struct ESynthSlateSizeType(pub u8);
impl ESynthSlateSizeType {
    pub const SMALL: ESynthSlateSizeType = ESynthSlateSizeType(0);
    pub const MEDIUM: ESynthSlateSizeType = ESynthSlateSizeType(1);
    pub const LARGE: ESynthSlateSizeType = ESynthSlateSizeType(2);
    pub const COUNT: ESynthSlateSizeType = ESynthSlateSizeType(3);
}
#[repr(transparent)]
pub struct ESynthSlateColorStyle(pub u8);
impl ESynthSlateColorStyle {
    pub const LIGHT: ESynthSlateColorStyle = ESynthSlateColorStyle(0);
    pub const DARK: ESynthSlateColorStyle = ESynthSlateColorStyle(1);
    pub const COUNT: ESynthSlateColorStyle = ESynthSlateColorStyle(2);
}
#[repr(transparent)]
pub struct EGranularSynthEnvelopeType(pub u8);
impl EGranularSynthEnvelopeType {
    pub const RECTANGULAR: EGranularSynthEnvelopeType = EGranularSynthEnvelopeType(0);
    pub const TRIANGLE: EGranularSynthEnvelopeType = EGranularSynthEnvelopeType(1);
    pub const DOWNWARD_TRIANGLE: EGranularSynthEnvelopeType = EGranularSynthEnvelopeType(
        2,
    );
    pub const UPWARD_TRIANGLE: EGranularSynthEnvelopeType = EGranularSynthEnvelopeType(
        3,
    );
    pub const EXPONENTIAL_DECAY: EGranularSynthEnvelopeType = EGranularSynthEnvelopeType(
        4,
    );
    pub const EXPONENTIAL_INCREASE: EGranularSynthEnvelopeType = EGranularSynthEnvelopeType(
        5,
    );
    pub const GAUSSIAN: EGranularSynthEnvelopeType = EGranularSynthEnvelopeType(6);
    pub const HANNING: EGranularSynthEnvelopeType = EGranularSynthEnvelopeType(7);
    pub const LANCZOS: EGranularSynthEnvelopeType = EGranularSynthEnvelopeType(8);
    pub const COSINE: EGranularSynthEnvelopeType = EGranularSynthEnvelopeType(9);
    pub const COSINE_SQUARED: EGranularSynthEnvelopeType = EGranularSynthEnvelopeType(
        10,
    );
    pub const WELCH: EGranularSynthEnvelopeType = EGranularSynthEnvelopeType(11);
    pub const BLACKMAN: EGranularSynthEnvelopeType = EGranularSynthEnvelopeType(12);
    pub const BLACKMAN_HARRIS: EGranularSynthEnvelopeType = EGranularSynthEnvelopeType(
        13,
    );
    pub const COUNT: EGranularSynthEnvelopeType = EGranularSynthEnvelopeType(14);
}
#[repr(transparent)]
pub struct EGranularSynthSeekType(pub u8);
impl EGranularSynthSeekType {
    pub const FROM_BEGINNING: EGranularSynthSeekType = EGranularSynthSeekType(0);
    pub const FROM_CURRENT_POSITION: EGranularSynthSeekType = EGranularSynthSeekType(1);
    pub const COUNT: EGranularSynthSeekType = EGranularSynthSeekType(2);
}
#[repr(transparent)]
pub struct CurveInterpolationType(pub u8);
impl CurveInterpolationType {
    pub const AUTOINTERP: CurveInterpolationType = CurveInterpolationType(0);
    pub const LINEAR: CurveInterpolationType = CurveInterpolationType(1);
    pub const CONSTANT: CurveInterpolationType = CurveInterpolationType(2);
}
#[repr(transparent)]
pub struct ESamplePlayerSeekType(pub u8);
impl ESamplePlayerSeekType {
    pub const FROM_BEGINNING: ESamplePlayerSeekType = ESamplePlayerSeekType(0);
    pub const FROM_CURRENT_POSITION: ESamplePlayerSeekType = ESamplePlayerSeekType(1);
    pub const FROM_END: ESamplePlayerSeekType = ESamplePlayerSeekType(2);
    pub const COUNT: ESamplePlayerSeekType = ESamplePlayerSeekType(3);
}
#[repr(transparent)]
pub struct ESubmixEffectConvolutionReverbBlockSize(pub u8);
impl ESubmixEffectConvolutionReverbBlockSize {
    pub const BLOCK_SIZE256: ESubmixEffectConvolutionReverbBlockSize = ESubmixEffectConvolutionReverbBlockSize(
        0,
    );
    pub const BLOCK_SIZE512: ESubmixEffectConvolutionReverbBlockSize = ESubmixEffectConvolutionReverbBlockSize(
        1,
    );
    pub const BLOCK_SIZE1024: ESubmixEffectConvolutionReverbBlockSize = ESubmixEffectConvolutionReverbBlockSize(
        2,
    );
}
