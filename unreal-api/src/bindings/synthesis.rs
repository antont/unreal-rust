#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FSynth1PatchCable {
    pub depth: f32,
    pub destination: ESynth1PatchDestination,
}
#[repr(C, align(4))]
pub struct FPatchId {
    pub id: i32,
}
#[repr(C, align(8))]
pub struct FEpicSynth1Patch {
    pub patch_source: ESynth1PatchSource,
    pub patch_cables: TArray<FSynth1PatchCable>,
}
#[repr(C, align(8))]
pub struct FModularSynthPreset {
    pub flags_8: u8,
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
    pub flags_164: u8,
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
#[repr(C, align(8))]
pub struct FModularSynthPresetBankEntry {
    pub preset_name: FString,
    pub preset: FModularSynthPreset,
}
#[repr(C, align(4))]
pub struct FSourceEffectBitCrusherBaseSettings {
    pub sample_rate: f32,
    pub bit_depth: f32,
}
#[repr(C, align(8))]
pub struct FSourceEffectBitCrusherSettings {
    pub sample_rate_modulation: FSoundModulationDestinationSettings,
    pub bit_modulation: FSoundModulationDestinationSettings,
}
#[repr(C, align(4))]
pub struct FSourceEffectChorusBaseSettings {
    pub depth: f32,
    pub frequency: f32,
    pub feedback: f32,
    pub wet_level: f32,
    pub dry_level: f32,
    pub spread: f32,
}
#[repr(C, align(8))]
pub struct FSourceEffectChorusSettings {
    pub depth: f32,
    pub frequency: f32,
    pub feedback: f32,
    pub wet_level: f32,
    pub dry_level: f32,
    pub spread: f32,
    pub depth_modulation: FSoundModulationDestinationSettings,
    pub frequency_modulation: FSoundModulationDestinationSettings,
    pub feedback_modulation: FSoundModulationDestinationSettings,
    pub wet_modulation: FSoundModulationDestinationSettings,
    pub dry_modulation: FSoundModulationDestinationSettings,
    pub spread_modulation: FSoundModulationDestinationSettings,
}
#[repr(C, align(4))]
pub struct FSourceEffectConvolutionReverbSettings {
    pub normalization_volume_db: f32,
    pub wet_volume_db: f32,
    pub dry_volume_db: f32,
    pub b_bypass: bool,
}
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
    pub flags_40: u8,
}
#[repr(C, align(4))]
pub struct FSourceEffectEnvelopeFollowerSettings {
    pub attack_time: f32,
    pub release_time: f32,
    pub peak_mode: EEnvelopeFollowerPeakMode,
    pub b_is_analog_mode: bool,
}
#[repr(C, align(4))]
pub struct FSourceEffectEQBand {
    pub frequency: f32,
    pub bandwidth: f32,
    pub gain_db: f32,
    pub flags_12: u8,
}
#[repr(C, align(8))]
pub struct FSourceEffectEQSettings {
    pub eq_bands: TArray<FSourceEffectEQBand>,
}
#[repr(C, align(8))]
pub struct FSourceEffectFilterAudioBusModulationSettings {
    pub audio_bus: UPtr<UAudioBus>,
    pub envelope_follower_attack_time_msec: i32,
    pub envelope_follower_release_time_msec: i32,
    pub envelope_gain_multiplier: f32,
    pub filter_param: ESourceEffectFilterParam,
    pub min_frequency_modulation: f32,
    pub max_frequency_modulation: f32,
    pub min_resonance_modulation: f32,
    pub max_resonance_modulation: f32,
}
#[repr(C, align(8))]
pub struct FSourceEffectFilterSettings {
    pub filter_circuit: ESourceEffectFilterCircuit,
    pub filter_type: ESourceEffectFilterType,
    pub cutoff_frequency: f32,
    pub filter_q: f32,
    pub audio_bus_modulation: TArray<FSourceEffectFilterAudioBusModulationSettings>,
}
#[repr(C, align(4))]
pub struct FSourceEffectFoldbackDistortionSettings {
    pub input_gain_db: f32,
    pub threshold_db: f32,
    pub output_gain_db: f32,
}
#[repr(C, align(4))]
pub struct FSourceEffectMidSideSpreaderSettings {
    pub spread_amount: f32,
    pub input_mode: EStereoChannelMode,
    pub output_mode: EStereoChannelMode,
    pub b_equal_power: bool,
}
#[repr(C, align(4))]
pub struct FSourceEffectIndividualFilterSettings {
    pub filter_circuit: ESourceEffectMotionFilterCircuit,
    pub filter_type: ESourceEffectMotionFilterType,
    pub cutoff_frequency: f32,
    pub filter_q: f32,
}
#[repr(C, align(8))]
pub struct FSourceEffectMotionFilterModulationSettings {
    pub modulation_source: ESourceEffectMotionFilterModSource,
    pub modulation_input_range: FVector2D,
    pub modulation_output_minimum_range: FVector2D,
    pub modulation_output_maximum_range: FVector2D,
    pub update_ease_ms: f32,
}
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
}
#[repr(C, align(4))]
pub struct FSourceEffectPannerSettings {
    pub spread: f32,
    pub pan: f32,
}
#[repr(C, align(4))]
pub struct FSourceEffectPhaserSettings {
    pub wet_level: f32,
    pub frequency: f32,
    pub feedback: f32,
    pub lfo_type: EPhaserLFOType,
    pub use_quadrature_phase: bool,
}
#[repr(C, align(8))]
pub struct FSourceEffectRingModulationSettings {
    pub modulator_type: ERingModulatorTypeSourceEffect,
    pub frequency: f32,
    pub depth: f32,
    pub dry_level: f32,
    pub wet_level: f32,
    pub audio_bus_modulator: UPtr<UAudioBus>,
}
#[repr(C, align(4))]
pub struct FSourceEffectSimpleDelaySettings {
    pub speed_of_sound: f32,
    pub delay_amount: f32,
    pub dry_amount: f32,
    pub wet_amount: f32,
    pub feedback: f32,
    pub flags_20: u8,
}
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
#[repr(C, align(4))]
pub struct FSourceEffectWaveShaperSettings {
    pub amount: f32,
    pub output_gain_db: f32,
}
#[repr(C, align(8))]
pub struct FSubmixEffectConvolutionReverbSettings {
    pub normalization_volume_db: f32,
    pub wet_volume_db: f32,
    pub dry_volume_db: f32,
    pub b_bypass: bool,
    pub b_mix_input_channel_format_to_impulse_response_format: bool,
    pub b_mix_reverb_output_to_output_channel_format: bool,
    pub surround_rear_channel_bleed_db: f32,
    pub b_invert_rear_channel_bleed_phase: bool,
    pub b_surround_rear_channel_flip: bool,
    pub surround_rear_channel_bleed_amount_deprecated: f32,
    pub impulse_response_deprecated: UPtr<UAudioImpulseResponse>,
    pub allow_hardware_acceleration_deprecated: bool,
}
#[repr(C, align(4))]
pub struct FSubmixEffectDelaySettings {
    pub maximum_delay_length: f32,
    pub interpolation_time: f32,
    pub delay_length: f32,
}
#[repr(C, align(4))]
pub struct FSubmixEffectFilterSettings {
    pub filter_type: ESubmixFilterType,
    pub filter_algorithm: ESubmixFilterAlgorithm,
    pub filter_frequency: f32,
    pub filter_q: f32,
}
#[repr(C, align(4))]
pub struct FSubmixEffectFlexiverbSettings {
    pub pre_delay: f32,
    pub decay_time: f32,
    pub room_dampening: f32,
    pub complexity: i32,
}
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
#[repr(C, align(8))]
pub struct FSubmixEffectMultibandCompressorSettings {
    pub dynamics_processor_type: ESubmixEffectDynamicsProcessorType,
    pub peak_mode: ESubmixEffectDynamicsPeakMode,
    pub link_mode: ESubmixEffectDynamicsChannelLinkMode,
    pub look_ahead_msec: f32,
    pub b_analog_mode: bool,
    pub b_four_pole: bool,
    pub b_bypass: bool,
    pub key_source: ESubmixEffectDynamicsKeySource,
    pub external_audio_bus: UPtr<UAudioBus>,
    pub external_submix: UPtr<USoundSubmix>,
    pub key_gain_db: f32,
    pub b_key_audition: bool,
    pub bands: TArray<FDynamicsBandSettings>,
}
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
#[repr(C, align(4))]
pub struct FSubmixEffectStereoToQuadSettings {
    pub b_flip_channels: bool,
    pub rear_channel_gain: f32,
}
#[repr(C, align(4))]
pub struct FTapDelayInfo {
    pub tap_line_mode: ETapLineMode,
    pub delay_length: f32,
    pub gain: f32,
    pub output_channel: i32,
    pub pan_in_degrees: f32,
    pub tap_id: i32,
}
#[repr(C, align(8))]
pub struct FSubmixEffectTapDelaySettings {
    pub maximum_delay_length: f32,
    pub interpolation_time: f32,
    pub taps: TArray<FTapDelayInfo>,
}
#[repr(C, align(16))]
pub struct FSynth2DSliderStyle {
    pub normal_thumb_image: FSlateBrush,
    pub disabled_thumb_image: FSlateBrush,
    pub normal_bar_image: FSlateBrush,
    pub disabled_bar_image: FSlateBrush,
    pub background_image: FSlateBrush,
    pub bar_thickness: f32,
}
#[repr(C, align(16))]
pub struct FSynthKnobStyle {
    pub large_knob: FSlateBrush,
    pub large_knob_overlay: FSlateBrush,
    pub medium_knob: FSlateBrush,
    pub medium_knob_overlay: FSlateBrush,
    pub min_value_angle: f32,
    pub max_value_angle: f32,
    pub knob_size: ESynthKnobSize,
}
#[repr(C, align(8))]
pub struct FSynthSlateStyle {
    pub size_type: ESynthSlateSizeType,
    pub color_style: ESynthSlateColorStyle,
}
pub struct UAudioImpulseResponse {
    pub impulse_response: TArray<f32>,
    pub num_channels: i32,
    pub sample_rate: i32,
    pub normalization_volume_db: f32,
    pub b_true_stereo: bool,
    pub ir_data_deprecated: TArray<f32>,
    pub b_is_even_channel_count: bool,
}
pub struct UModularSynthPresetBank {
    pub presets: TArray<FModularSynthPresetBankEntry>,
}
pub struct UModularSynthLibrary {}
pub struct UModularSynthComponent {
    pub voice_count: i32,
}
pub struct USourceEffectBitCrusherPreset {
    pub settings: FSourceEffectBitCrusherSettings,
}
pub struct USourceEffectChorusPreset {
    pub settings: FSourceEffectChorusSettings,
}
pub struct USourceEffectConvolutionReverbPreset {
    pub impulse_response: UPtr<UAudioImpulseResponse>,
    pub settings: FSourceEffectConvolutionReverbSettings,
    pub block_size: ESubmixEffectConvolutionReverbBlockSize,
    pub b_enable_hardware_acceleration: bool,
}
pub struct USourceEffectDynamicsProcessorPreset {
    pub settings: FSourceEffectDynamicsProcessorSettings,
}
pub struct UEnvelopeFollowerListener {
    pub on_envelope_follower_update: FEnvelopeFollowerListener_OnEnvelopeFollowerUpdate,
}
pub struct USourceEffectEnvelopeFollowerPreset {
    pub settings: FSourceEffectEnvelopeFollowerSettings,
}
pub struct USourceEffectEQPreset {
    pub settings: FSourceEffectEQSettings,
}
pub struct USourceEffectFilterPreset {
    pub settings: FSourceEffectFilterSettings,
}
pub struct USourceEffectFoldbackDistortionPreset {
    pub settings: FSourceEffectFoldbackDistortionSettings,
}
pub struct USourceEffectMidSideSpreaderPreset {
    pub settings: FSourceEffectMidSideSpreaderSettings,
}
pub struct USourceEffectMotionFilterPreset {
    pub settings: FSourceEffectMotionFilterSettings,
}
pub struct USourceEffectPannerPreset {
    pub settings: FSourceEffectPannerSettings,
}
pub struct USourceEffectPhaserPreset {
    pub settings: FSourceEffectPhaserSettings,
}
pub struct USourceEffectRingModulationPreset {
    pub settings: FSourceEffectRingModulationSettings,
}
pub struct USourceEffectSimpleDelayPreset {
    pub settings: FSourceEffectSimpleDelaySettings,
}
pub struct USourceEffectStereoDelayPreset {
    pub settings: FSourceEffectStereoDelaySettings,
}
pub struct USourceEffectWaveShaperPreset {
    pub settings: FSourceEffectWaveShaperSettings,
}
pub struct USubmixEffectConvolutionReverbPreset {
    pub impulse_response: UPtr<UAudioImpulseResponse>,
    pub settings: FSubmixEffectConvolutionReverbSettings,
    pub block_size: ESubmixEffectConvolutionReverbBlockSize,
    pub b_enable_hardware_acceleration: bool,
}
pub struct USubmixEffectDelayStatics {}
pub struct USubmixEffectDelayPreset {
    pub settings: FSubmixEffectDelaySettings,
    pub dynamic_settings: FSubmixEffectDelaySettings,
}
pub struct USubmixEffectFilterPreset {
    pub settings: FSubmixEffectFilterSettings,
}
pub struct USubmixEffectFlexiverbPreset {
    pub settings: FSubmixEffectFlexiverbSettings,
}
pub struct USubmixEffectMultibandCompressorPreset {
    pub settings: FSubmixEffectMultibandCompressorSettings,
}
pub struct USubmixEffectStereoDelayPreset {
    pub settings: FSubmixEffectStereoDelaySettings,
}
pub struct USubmixEffectStereoToQuadPreset {
    pub settings: FSubmixEffectStereoToQuadSettings,
}
pub struct USubmixEffectTapDelayPreset {
    pub settings: FSubmixEffectTapDelaySettings,
}
pub struct UGranularSynth {
    pub granulated_sound_wave: UPtr<USoundWave>,
}
pub struct UMonoWaveTableSynthPreset {
    pub preset_name: FString,
    pub flags_64: u8,
    pub lock_keyframes_to_grid: i32,
    pub wave_table_resolution: i32,
    pub wave_table: TArray<FRuntimeFloatCurve>,
    pub flags_96: u8,
}
pub struct USynthComponentMonoWaveTable {
    pub on_table_altered: FSynthComponentMonoWaveTable_OnTableAltered,
    pub on_num_tables_changed: FSynthComponentMonoWaveTable_OnNumTablesChanged,
    pub current_preset: UPtr<UMonoWaveTableSynthPreset>,
}
pub struct USynthComponentToneGenerator {
    pub frequency: f32,
    pub volume: f32,
    pub distance_attenuation_curve: FRuntimeFloatCurve,
    pub distance_range: FVector2D,
    pub attenuation_db_at_max_range: f32,
}
pub struct USynthSamplePlayer {
    pub sound_wave: UPtr<USoundWave>,
    pub on_sample_loaded: FSynthSamplePlayer_OnSampleLoaded,
    pub on_sample_playback_progress: FSynthSamplePlayer_OnSamplePlaybackProgress,
}
pub struct USynthesisUtilitiesBlueprintFunctionLibrary {}
pub struct USynth2DSlider {
    pub value_x: f32,
    pub value_y: f32,
    pub value_x_delegate: FSynth2DSlider_ValueXDelegate,
    pub value_y_delegate: FSynth2DSlider_ValueYDelegate,
    pub widget_style: FSynth2DSliderStyle,
    pub slider_handle_color: FLinearColor,
    pub indent_handle: bool,
    pub locked: bool,
    pub step_size: f32,
    pub is_focusable: bool,
    pub on_mouse_capture_begin: FSynth2DSlider_OnMouseCaptureBegin,
    pub on_mouse_capture_end: FSynth2DSlider_OnMouseCaptureEnd,
    pub on_controller_capture_begin: FSynth2DSlider_OnControllerCaptureBegin,
    pub on_controller_capture_end: FSynth2DSlider_OnControllerCaptureEnd,
    pub on_value_changed_x: FSynth2DSlider_OnValueChangedX,
    pub on_value_changed_y: FSynth2DSlider_OnValueChangedY,
}
pub struct USynthKnob {
    pub value: f32,
    pub step_size: f32,
    pub mouse_speed: f32,
    pub mouse_fine_tune_speed: f32,
    pub flags_712: u8,
    pub parameter_name: FText,
    pub parameter_units: FText,
    pub value_delegate: FSynthKnob_ValueDelegate,
    pub widget_style: FSynthKnobStyle,
    pub locked: bool,
    pub is_focusable: bool,
    pub on_mouse_capture_begin: FSynthKnob_OnMouseCaptureBegin,
    pub on_mouse_capture_end: FSynthKnob_OnMouseCaptureEnd,
    pub on_controller_capture_begin: FSynthKnob_OnControllerCaptureBegin,
    pub on_controller_capture_end: FSynthKnob_OnControllerCaptureEnd,
    pub on_value_changed: FSynthKnob_OnValueChanged,
}
