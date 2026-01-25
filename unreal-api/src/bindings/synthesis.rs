#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_modular_synth_library_add_modular_synth_preset_to_bank_asset: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_synth_preset: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_sustain_gain: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_stereo_delay_wetlevel: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_stereo_delay_time: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_stereo_delay_ratio: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_stereo_delay_mode: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_stereo_delay_is_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_stereo_delay_feedback: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_spread: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_release_time: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_portamento: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_pitch_bend: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_pan: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_osc_type: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_osc_sync: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_osc_semitones: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_osc_pulsewidth: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_osc_octave: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_osc_gain_mod: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_osc_gain: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_osc_frequency_mod: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_osc_cents: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_mod_env_sustain_gain: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_mod_env_release_time: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_mod_env_patch: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_mod_env_invert: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_mod_env_depth: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_mod_env_decay_time: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_mod_env_bias_patch: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_mod_env_bias_invert: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_mod_env_attack_time: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_lfo_type: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_lfo_patch: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_lfo_mode: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_lfo_gain_mod: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_lfo_gain: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_lfo_frequency_mod: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_lfo_frequency: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_gain_db: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_filter_type: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_filter_q_mod: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_filter_q: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_filter_frequency_mod: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_filter_frequency: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_filter_algorithm: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_enable_unison: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_enable_retrigger: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_enable_polyphony: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_enable_patch: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_enable_legato: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_decay_time: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_chorus_frequency: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_chorus_feedback: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_chorus_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_chorus_depth: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_set_attack_time: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_note_on: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_note_off: *mut crate::ffi::UFunctionOpague,
    pub u_modular_synth_component_create_patch: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_bit_crusher_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_bit_crusher_preset_set_sample_rate_modulators: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_bit_crusher_preset_set_sample_rate_modulator: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_bit_crusher_preset_set_sample_rate: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_bit_crusher_preset_set_modulation_settings: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_bit_crusher_preset_set_bits: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_bit_crusher_preset_set_bit_modulators: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_bit_crusher_preset_set_bit_modulator: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_chorus_preset_set_wet_modulators: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_chorus_preset_set_wet_modulator: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_chorus_preset_set_wet: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_chorus_preset_set_spread_modulators: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_chorus_preset_set_spread_modulator: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_chorus_preset_set_spread: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_chorus_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_chorus_preset_set_modulation_settings: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_chorus_preset_set_frequency_modulators: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_chorus_preset_set_frequency_modulator: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_chorus_preset_set_frequency: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_chorus_preset_set_feedback_modulators: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_chorus_preset_set_feedback_modulator: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_chorus_preset_set_feedback: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_chorus_preset_set_dry_modulators: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_chorus_preset_set_dry_modulator: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_chorus_preset_set_dry: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_chorus_preset_set_depth_modulators: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_chorus_preset_set_depth_modulator: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_chorus_preset_set_depth: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_convolution_reverb_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_convolution_reverb_preset_set_impulse_response: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_dynamics_processor_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_envelope_follower_preset_unregister_envelope_follower_listener: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_envelope_follower_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_envelope_follower_preset_register_envelope_follower_listener: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_eq_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_filter_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_foldback_distortion_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_mid_side_spreader_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_motion_filter_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_panner_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_phaser_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_ring_modulation_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_simple_delay_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_stereo_delay_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_source_effect_wave_shaper_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_convolution_reverb_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_convolution_reverb_preset_set_impulse_response: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_delay_statics_set_maximum_delay_length: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_delay_statics_set_interpolation_time: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_delay_statics_set_delay_length: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_delay_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_delay_preset_set_interpolation_time: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_delay_preset_set_delay: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_delay_preset_set_default_settings: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_delay_preset_get_max_delay_in_milliseconds: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_filter_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_filter_preset_set_filter_type: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_filter_preset_set_filter_q_mod: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_filter_preset_set_filter_q: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_filter_preset_set_filter_cutoff_frequency_mod: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_filter_preset_set_filter_cutoff_frequency: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_filter_preset_set_filter_algorithm: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_flexiverb_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_multiband_compressor_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_multiband_compressor_preset_set_external_submix: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_multiband_compressor_preset_set_audio_bus: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_multiband_compressor_preset_reset_key: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_stereo_delay_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_stereo_to_quad_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_tap_delay_preset_set_tap: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_tap_delay_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_tap_delay_preset_set_interpolation_time: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_tap_delay_preset_remove_tap: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_tap_delay_preset_get_tap_ids: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_tap_delay_preset_get_tap: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_tap_delay_preset_get_max_delay_in_milliseconds: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_tap_delay_preset_add_tap: *mut crate::ffi::UFunctionOpague,
    pub u_granular_synth_set_sustain_gain: *mut crate::ffi::UFunctionOpague,
    pub u_granular_synth_set_sound_wave: *mut crate::ffi::UFunctionOpague,
    pub u_granular_synth_set_scrub_mode: *mut crate::ffi::UFunctionOpague,
    pub u_granular_synth_set_release_time_msec: *mut crate::ffi::UFunctionOpague,
    pub u_granular_synth_set_playhead_time: *mut crate::ffi::UFunctionOpague,
    pub u_granular_synth_set_playback_speed: *mut crate::ffi::UFunctionOpague,
    pub u_granular_synth_set_grain_volume: *mut crate::ffi::UFunctionOpague,
    pub u_granular_synth_set_grains_per_second: *mut crate::ffi::UFunctionOpague,
    pub u_granular_synth_set_grain_probability: *mut crate::ffi::UFunctionOpague,
    pub u_granular_synth_set_grain_pitch: *mut crate::ffi::UFunctionOpague,
    pub u_granular_synth_set_grain_pan: *mut crate::ffi::UFunctionOpague,
    pub u_granular_synth_set_grain_envelope_type: *mut crate::ffi::UFunctionOpague,
    pub u_granular_synth_set_grain_duration: *mut crate::ffi::UFunctionOpague,
    pub u_granular_synth_set_decay_time: *mut crate::ffi::UFunctionOpague,
    pub u_granular_synth_set_attack_time: *mut crate::ffi::UFunctionOpague,
    pub u_granular_synth_note_on: *mut crate::ffi::UFunctionOpague,
    pub u_granular_synth_note_off: *mut crate::ffi::UFunctionOpague,
    pub u_granular_synth_is_loaded: *mut crate::ffi::UFunctionOpague,
    pub u_granular_synth_get_sample_duration: *mut crate::ffi::UFunctionOpague,
    pub u_granular_synth_get_current_playhead_time: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_wave_table_position: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_sustain_pedal_state: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_pos_lfo_type: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_pos_lfo_frequency: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_pos_lfo_depth: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_position_envelope_sustain_gain: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_position_envelope_release_time: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_position_envelope_invert: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_position_envelope_depth: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_position_envelope_decay_time: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_position_envelope_bias_invert: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_position_envelope_bias_depth: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_position_envelope_attack_time: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_low_pass_filter_resonance: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_frequency_with_midi_note: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_frequency_pitch_bend: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_frequency: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_filter_envelope_sustain_gain: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_filter_envelope_release_time: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_filter_envelopen_decay_time: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_filter_envelope_invert: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_filter_envelope_depth: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_filter_envelope_bias_invert: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_filter_envelope_bias_depth: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_filter_envelope_attack_time: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_curve_value: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_curve_tangent: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_curve_interpolation_type: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_amp_envelope_sustain_gain: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_amp_envelope_release_time: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_amp_envelope_invert: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_amp_envelope_depth: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_amp_envelope_decay_time: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_amp_envelope_bias_invert: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_amp_envelope_bias_depth: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_set_amp_envelope_attack_time: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_refresh_wave_table: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_refresh_all_wave_tables: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_note_on: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_note_off: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_get_num_table_entries: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_get_max_table_index: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_get_key_frame_values_for_table: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_mono_wave_table_get_curve_tangent: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_tone_generator_set_volume: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_tone_generator_set_frequency: *mut crate::ffi::UFunctionOpague,
    pub u_synth_sample_player_set_sound_wave: *mut crate::ffi::UFunctionOpague,
    pub u_synth_sample_player_set_scrub_time_width: *mut crate::ffi::UFunctionOpague,
    pub u_synth_sample_player_set_scrub_mode: *mut crate::ffi::UFunctionOpague,
    pub u_synth_sample_player_set_pitch: *mut crate::ffi::UFunctionOpague,
    pub u_synth_sample_player_seek_to_time: *mut crate::ffi::UFunctionOpague,
    pub u_synth_sample_player_is_loaded: *mut crate::ffi::UFunctionOpague,
    pub u_synth_sample_player_get_sample_duration: *mut crate::ffi::UFunctionOpague,
    pub u_synth_sample_player_get_current_playback_progress_time: *mut crate::ffi::UFunctionOpague,
    pub u_synth_sample_player_get_current_playback_progress_percent: *mut crate::ffi::UFunctionOpague,
    pub u_synthesis_utilities_blueprint_function_library_get_log_frequency: *mut crate::ffi::UFunctionOpague,
    pub u_synthesis_utilities_blueprint_function_library_get_linear_frequency: *mut crate::ffi::UFunctionOpague,
    pub u_synth2_d_slider_set_value: *mut crate::ffi::UFunctionOpague,
    pub u_synth2_d_slider_set_step_size: *mut crate::ffi::UFunctionOpague,
    pub u_synth2_d_slider_set_slider_handle_color: *mut crate::ffi::UFunctionOpague,
    pub u_synth2_d_slider_set_locked: *mut crate::ffi::UFunctionOpague,
    pub u_synth2_d_slider_set_indent_handle: *mut crate::ffi::UFunctionOpague,
    pub u_synth2_d_slider_get_value: *mut crate::ffi::UFunctionOpague,
    pub u_synth_knob_set_value: *mut crate::ffi::UFunctionOpague,
    pub u_synth_knob_set_step_size: *mut crate::ffi::UFunctionOpague,
    pub u_synth_knob_set_locked: *mut crate::ffi::UFunctionOpague,
    pub u_synth_knob_get_value: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_modular_synth_library_add_modular_synth_preset_to_bank_asset: std::ptr::null_mut(),
            u_modular_synth_component_set_synth_preset: std::ptr::null_mut(),
            u_modular_synth_component_set_sustain_gain: std::ptr::null_mut(),
            u_modular_synth_component_set_stereo_delay_wetlevel: std::ptr::null_mut(),
            u_modular_synth_component_set_stereo_delay_time: std::ptr::null_mut(),
            u_modular_synth_component_set_stereo_delay_ratio: std::ptr::null_mut(),
            u_modular_synth_component_set_stereo_delay_mode: std::ptr::null_mut(),
            u_modular_synth_component_set_stereo_delay_is_enabled: std::ptr::null_mut(),
            u_modular_synth_component_set_stereo_delay_feedback: std::ptr::null_mut(),
            u_modular_synth_component_set_spread: std::ptr::null_mut(),
            u_modular_synth_component_set_release_time: std::ptr::null_mut(),
            u_modular_synth_component_set_portamento: std::ptr::null_mut(),
            u_modular_synth_component_set_pitch_bend: std::ptr::null_mut(),
            u_modular_synth_component_set_pan: std::ptr::null_mut(),
            u_modular_synth_component_set_osc_type: std::ptr::null_mut(),
            u_modular_synth_component_set_osc_sync: std::ptr::null_mut(),
            u_modular_synth_component_set_osc_semitones: std::ptr::null_mut(),
            u_modular_synth_component_set_osc_pulsewidth: std::ptr::null_mut(),
            u_modular_synth_component_set_osc_octave: std::ptr::null_mut(),
            u_modular_synth_component_set_osc_gain_mod: std::ptr::null_mut(),
            u_modular_synth_component_set_osc_gain: std::ptr::null_mut(),
            u_modular_synth_component_set_osc_frequency_mod: std::ptr::null_mut(),
            u_modular_synth_component_set_osc_cents: std::ptr::null_mut(),
            u_modular_synth_component_set_mod_env_sustain_gain: std::ptr::null_mut(),
            u_modular_synth_component_set_mod_env_release_time: std::ptr::null_mut(),
            u_modular_synth_component_set_mod_env_patch: std::ptr::null_mut(),
            u_modular_synth_component_set_mod_env_invert: std::ptr::null_mut(),
            u_modular_synth_component_set_mod_env_depth: std::ptr::null_mut(),
            u_modular_synth_component_set_mod_env_decay_time: std::ptr::null_mut(),
            u_modular_synth_component_set_mod_env_bias_patch: std::ptr::null_mut(),
            u_modular_synth_component_set_mod_env_bias_invert: std::ptr::null_mut(),
            u_modular_synth_component_set_mod_env_attack_time: std::ptr::null_mut(),
            u_modular_synth_component_set_lfo_type: std::ptr::null_mut(),
            u_modular_synth_component_set_lfo_patch: std::ptr::null_mut(),
            u_modular_synth_component_set_lfo_mode: std::ptr::null_mut(),
            u_modular_synth_component_set_lfo_gain_mod: std::ptr::null_mut(),
            u_modular_synth_component_set_lfo_gain: std::ptr::null_mut(),
            u_modular_synth_component_set_lfo_frequency_mod: std::ptr::null_mut(),
            u_modular_synth_component_set_lfo_frequency: std::ptr::null_mut(),
            u_modular_synth_component_set_gain_db: std::ptr::null_mut(),
            u_modular_synth_component_set_filter_type: std::ptr::null_mut(),
            u_modular_synth_component_set_filter_q_mod: std::ptr::null_mut(),
            u_modular_synth_component_set_filter_q: std::ptr::null_mut(),
            u_modular_synth_component_set_filter_frequency_mod: std::ptr::null_mut(),
            u_modular_synth_component_set_filter_frequency: std::ptr::null_mut(),
            u_modular_synth_component_set_filter_algorithm: std::ptr::null_mut(),
            u_modular_synth_component_set_enable_unison: std::ptr::null_mut(),
            u_modular_synth_component_set_enable_retrigger: std::ptr::null_mut(),
            u_modular_synth_component_set_enable_polyphony: std::ptr::null_mut(),
            u_modular_synth_component_set_enable_patch: std::ptr::null_mut(),
            u_modular_synth_component_set_enable_legato: std::ptr::null_mut(),
            u_modular_synth_component_set_decay_time: std::ptr::null_mut(),
            u_modular_synth_component_set_chorus_frequency: std::ptr::null_mut(),
            u_modular_synth_component_set_chorus_feedback: std::ptr::null_mut(),
            u_modular_synth_component_set_chorus_enabled: std::ptr::null_mut(),
            u_modular_synth_component_set_chorus_depth: std::ptr::null_mut(),
            u_modular_synth_component_set_attack_time: std::ptr::null_mut(),
            u_modular_synth_component_note_on: std::ptr::null_mut(),
            u_modular_synth_component_note_off: std::ptr::null_mut(),
            u_modular_synth_component_create_patch: std::ptr::null_mut(),
            u_source_effect_bit_crusher_preset_set_settings: std::ptr::null_mut(),
            u_source_effect_bit_crusher_preset_set_sample_rate_modulators: std::ptr::null_mut(),
            u_source_effect_bit_crusher_preset_set_sample_rate_modulator: std::ptr::null_mut(),
            u_source_effect_bit_crusher_preset_set_sample_rate: std::ptr::null_mut(),
            u_source_effect_bit_crusher_preset_set_modulation_settings: std::ptr::null_mut(),
            u_source_effect_bit_crusher_preset_set_bits: std::ptr::null_mut(),
            u_source_effect_bit_crusher_preset_set_bit_modulators: std::ptr::null_mut(),
            u_source_effect_bit_crusher_preset_set_bit_modulator: std::ptr::null_mut(),
            u_source_effect_chorus_preset_set_wet_modulators: std::ptr::null_mut(),
            u_source_effect_chorus_preset_set_wet_modulator: std::ptr::null_mut(),
            u_source_effect_chorus_preset_set_wet: std::ptr::null_mut(),
            u_source_effect_chorus_preset_set_spread_modulators: std::ptr::null_mut(),
            u_source_effect_chorus_preset_set_spread_modulator: std::ptr::null_mut(),
            u_source_effect_chorus_preset_set_spread: std::ptr::null_mut(),
            u_source_effect_chorus_preset_set_settings: std::ptr::null_mut(),
            u_source_effect_chorus_preset_set_modulation_settings: std::ptr::null_mut(),
            u_source_effect_chorus_preset_set_frequency_modulators: std::ptr::null_mut(),
            u_source_effect_chorus_preset_set_frequency_modulator: std::ptr::null_mut(),
            u_source_effect_chorus_preset_set_frequency: std::ptr::null_mut(),
            u_source_effect_chorus_preset_set_feedback_modulators: std::ptr::null_mut(),
            u_source_effect_chorus_preset_set_feedback_modulator: std::ptr::null_mut(),
            u_source_effect_chorus_preset_set_feedback: std::ptr::null_mut(),
            u_source_effect_chorus_preset_set_dry_modulators: std::ptr::null_mut(),
            u_source_effect_chorus_preset_set_dry_modulator: std::ptr::null_mut(),
            u_source_effect_chorus_preset_set_dry: std::ptr::null_mut(),
            u_source_effect_chorus_preset_set_depth_modulators: std::ptr::null_mut(),
            u_source_effect_chorus_preset_set_depth_modulator: std::ptr::null_mut(),
            u_source_effect_chorus_preset_set_depth: std::ptr::null_mut(),
            u_source_effect_convolution_reverb_preset_set_settings: std::ptr::null_mut(),
            u_source_effect_convolution_reverb_preset_set_impulse_response: std::ptr::null_mut(),
            u_source_effect_dynamics_processor_preset_set_settings: std::ptr::null_mut(),
            u_source_effect_envelope_follower_preset_unregister_envelope_follower_listener: std::ptr::null_mut(),
            u_source_effect_envelope_follower_preset_set_settings: std::ptr::null_mut(),
            u_source_effect_envelope_follower_preset_register_envelope_follower_listener: std::ptr::null_mut(),
            u_source_effect_eq_preset_set_settings: std::ptr::null_mut(),
            u_source_effect_filter_preset_set_settings: std::ptr::null_mut(),
            u_source_effect_foldback_distortion_preset_set_settings: std::ptr::null_mut(),
            u_source_effect_mid_side_spreader_preset_set_settings: std::ptr::null_mut(),
            u_source_effect_motion_filter_preset_set_settings: std::ptr::null_mut(),
            u_source_effect_panner_preset_set_settings: std::ptr::null_mut(),
            u_source_effect_phaser_preset_set_settings: std::ptr::null_mut(),
            u_source_effect_ring_modulation_preset_set_settings: std::ptr::null_mut(),
            u_source_effect_simple_delay_preset_set_settings: std::ptr::null_mut(),
            u_source_effect_stereo_delay_preset_set_settings: std::ptr::null_mut(),
            u_source_effect_wave_shaper_preset_set_settings: std::ptr::null_mut(),
            u_submix_effect_convolution_reverb_preset_set_settings: std::ptr::null_mut(),
            u_submix_effect_convolution_reverb_preset_set_impulse_response: std::ptr::null_mut(),
            u_submix_effect_delay_statics_set_maximum_delay_length: std::ptr::null_mut(),
            u_submix_effect_delay_statics_set_interpolation_time: std::ptr::null_mut(),
            u_submix_effect_delay_statics_set_delay_length: std::ptr::null_mut(),
            u_submix_effect_delay_preset_set_settings: std::ptr::null_mut(),
            u_submix_effect_delay_preset_set_interpolation_time: std::ptr::null_mut(),
            u_submix_effect_delay_preset_set_delay: std::ptr::null_mut(),
            u_submix_effect_delay_preset_set_default_settings: std::ptr::null_mut(),
            u_submix_effect_delay_preset_get_max_delay_in_milliseconds: std::ptr::null_mut(),
            u_submix_effect_filter_preset_set_settings: std::ptr::null_mut(),
            u_submix_effect_filter_preset_set_filter_type: std::ptr::null_mut(),
            u_submix_effect_filter_preset_set_filter_q_mod: std::ptr::null_mut(),
            u_submix_effect_filter_preset_set_filter_q: std::ptr::null_mut(),
            u_submix_effect_filter_preset_set_filter_cutoff_frequency_mod: std::ptr::null_mut(),
            u_submix_effect_filter_preset_set_filter_cutoff_frequency: std::ptr::null_mut(),
            u_submix_effect_filter_preset_set_filter_algorithm: std::ptr::null_mut(),
            u_submix_effect_flexiverb_preset_set_settings: std::ptr::null_mut(),
            u_submix_effect_multiband_compressor_preset_set_settings: std::ptr::null_mut(),
            u_submix_effect_multiband_compressor_preset_set_external_submix: std::ptr::null_mut(),
            u_submix_effect_multiband_compressor_preset_set_audio_bus: std::ptr::null_mut(),
            u_submix_effect_multiband_compressor_preset_reset_key: std::ptr::null_mut(),
            u_submix_effect_stereo_delay_preset_set_settings: std::ptr::null_mut(),
            u_submix_effect_stereo_to_quad_preset_set_settings: std::ptr::null_mut(),
            u_submix_effect_tap_delay_preset_set_tap: std::ptr::null_mut(),
            u_submix_effect_tap_delay_preset_set_settings: std::ptr::null_mut(),
            u_submix_effect_tap_delay_preset_set_interpolation_time: std::ptr::null_mut(),
            u_submix_effect_tap_delay_preset_remove_tap: std::ptr::null_mut(),
            u_submix_effect_tap_delay_preset_get_tap_ids: std::ptr::null_mut(),
            u_submix_effect_tap_delay_preset_get_tap: std::ptr::null_mut(),
            u_submix_effect_tap_delay_preset_get_max_delay_in_milliseconds: std::ptr::null_mut(),
            u_submix_effect_tap_delay_preset_add_tap: std::ptr::null_mut(),
            u_granular_synth_set_sustain_gain: std::ptr::null_mut(),
            u_granular_synth_set_sound_wave: std::ptr::null_mut(),
            u_granular_synth_set_scrub_mode: std::ptr::null_mut(),
            u_granular_synth_set_release_time_msec: std::ptr::null_mut(),
            u_granular_synth_set_playhead_time: std::ptr::null_mut(),
            u_granular_synth_set_playback_speed: std::ptr::null_mut(),
            u_granular_synth_set_grain_volume: std::ptr::null_mut(),
            u_granular_synth_set_grains_per_second: std::ptr::null_mut(),
            u_granular_synth_set_grain_probability: std::ptr::null_mut(),
            u_granular_synth_set_grain_pitch: std::ptr::null_mut(),
            u_granular_synth_set_grain_pan: std::ptr::null_mut(),
            u_granular_synth_set_grain_envelope_type: std::ptr::null_mut(),
            u_granular_synth_set_grain_duration: std::ptr::null_mut(),
            u_granular_synth_set_decay_time: std::ptr::null_mut(),
            u_granular_synth_set_attack_time: std::ptr::null_mut(),
            u_granular_synth_note_on: std::ptr::null_mut(),
            u_granular_synth_note_off: std::ptr::null_mut(),
            u_granular_synth_is_loaded: std::ptr::null_mut(),
            u_granular_synth_get_sample_duration: std::ptr::null_mut(),
            u_granular_synth_get_current_playhead_time: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_wave_table_position: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_sustain_pedal_state: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_pos_lfo_type: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_pos_lfo_frequency: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_pos_lfo_depth: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_position_envelope_sustain_gain: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_position_envelope_release_time: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_position_envelope_invert: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_position_envelope_depth: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_position_envelope_decay_time: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_position_envelope_bias_invert: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_position_envelope_bias_depth: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_position_envelope_attack_time: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_low_pass_filter_resonance: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_frequency_with_midi_note: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_frequency_pitch_bend: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_frequency: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_filter_envelope_sustain_gain: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_filter_envelope_release_time: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_filter_envelopen_decay_time: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_filter_envelope_invert: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_filter_envelope_depth: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_filter_envelope_bias_invert: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_filter_envelope_bias_depth: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_filter_envelope_attack_time: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_curve_value: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_curve_tangent: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_curve_interpolation_type: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_amp_envelope_sustain_gain: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_amp_envelope_release_time: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_amp_envelope_invert: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_amp_envelope_depth: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_amp_envelope_decay_time: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_amp_envelope_bias_invert: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_amp_envelope_bias_depth: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_set_amp_envelope_attack_time: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_refresh_wave_table: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_refresh_all_wave_tables: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_note_on: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_note_off: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_get_num_table_entries: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_get_max_table_index: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_get_key_frame_values_for_table: std::ptr::null_mut(),
            u_synth_component_mono_wave_table_get_curve_tangent: std::ptr::null_mut(),
            u_synth_component_tone_generator_set_volume: std::ptr::null_mut(),
            u_synth_component_tone_generator_set_frequency: std::ptr::null_mut(),
            u_synth_sample_player_set_sound_wave: std::ptr::null_mut(),
            u_synth_sample_player_set_scrub_time_width: std::ptr::null_mut(),
            u_synth_sample_player_set_scrub_mode: std::ptr::null_mut(),
            u_synth_sample_player_set_pitch: std::ptr::null_mut(),
            u_synth_sample_player_seek_to_time: std::ptr::null_mut(),
            u_synth_sample_player_is_loaded: std::ptr::null_mut(),
            u_synth_sample_player_get_sample_duration: std::ptr::null_mut(),
            u_synth_sample_player_get_current_playback_progress_time: std::ptr::null_mut(),
            u_synth_sample_player_get_current_playback_progress_percent: std::ptr::null_mut(),
            u_synthesis_utilities_blueprint_function_library_get_log_frequency: std::ptr::null_mut(),
            u_synthesis_utilities_blueprint_function_library_get_linear_frequency: std::ptr::null_mut(),
            u_synth2_d_slider_set_value: std::ptr::null_mut(),
            u_synth2_d_slider_set_step_size: std::ptr::null_mut(),
            u_synth2_d_slider_set_slider_handle_color: std::ptr::null_mut(),
            u_synth2_d_slider_set_locked: std::ptr::null_mut(),
            u_synth2_d_slider_set_indent_handle: std::ptr::null_mut(),
            u_synth2_d_slider_get_value: std::ptr::null_mut(),
            u_synth_knob_set_value: std::ptr::null_mut(),
            u_synth_knob_set_step_size: std::ptr::null_mut(),
            u_synth_knob_set_locked: std::ptr::null_mut(),
            u_synth_knob_get_value: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UModularSynthLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddModularSynthPresetToBankAsset"),
            &raw mut __FUNCTION_PTRS
                .u_modular_synth_library_add_modular_synth_preset_to_bank_asset,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UModularSynthComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSynthPreset"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_synth_preset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSustainGain"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_sustain_gain,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStereoDelayWetlevel"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_stereo_delay_wetlevel,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStereoDelayTime"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_stereo_delay_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStereoDelayRatio"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_stereo_delay_ratio,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStereoDelayMode"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_stereo_delay_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStereoDelayIsEnabled"),
            &raw mut __FUNCTION_PTRS
                .u_modular_synth_component_set_stereo_delay_is_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStereoDelayFeedback"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_stereo_delay_feedback,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSpread"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_spread,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetReleaseTime"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_release_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPortamento"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_portamento,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPitchBend"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_pitch_bend,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPan"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_pan,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOscType"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_osc_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOscSync"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_osc_sync,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOscSemitones"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_osc_semitones,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOscPulsewidth"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_osc_pulsewidth,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOscOctave"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_osc_octave,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOscGainMod"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_osc_gain_mod,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOscGain"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_osc_gain,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOscFrequencyMod"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_osc_frequency_mod,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOscCents"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_osc_cents,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetModEnvSustainGain"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_mod_env_sustain_gain,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetModEnvReleaseTime"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_mod_env_release_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetModEnvPatch"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_mod_env_patch,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetModEnvInvert"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_mod_env_invert,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetModEnvDepth"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_mod_env_depth,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetModEnvDecayTime"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_mod_env_decay_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetModEnvBiasPatch"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_mod_env_bias_patch,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetModEnvBiasInvert"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_mod_env_bias_invert,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetModEnvAttackTime"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_mod_env_attack_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLFOType"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_lfo_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLFOPatch"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_lfo_patch,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLFOMode"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_lfo_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLFOGainMod"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_lfo_gain_mod,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLFOGain"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_lfo_gain,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLFOFrequencyMod"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_lfo_frequency_mod,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLFOFrequency"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_lfo_frequency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGainDb"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_gain_db,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterType"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_filter_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterQMod"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_filter_q_mod,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterQ"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_filter_q,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterFrequencyMod"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_filter_frequency_mod,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterFrequency"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_filter_frequency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterAlgorithm"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_filter_algorithm,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnableUnison"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_enable_unison,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnableRetrigger"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_enable_retrigger,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnablePolyphony"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_enable_polyphony,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnablePatch"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_enable_patch,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnableLegato"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_enable_legato,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDecayTime"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_decay_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetChorusFrequency"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_chorus_frequency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetChorusFeedback"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_chorus_feedback,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetChorusEnabled"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_chorus_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetChorusDepth"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_chorus_depth,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAttackTime"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_set_attack_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NoteOn"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_note_on,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NoteOff"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_note_off,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreatePatch"),
            &raw mut __FUNCTION_PTRS.u_modular_synth_component_create_patch,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USourceEffectBitCrusherPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.u_source_effect_bit_crusher_preset_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSampleRateModulators"),
            &raw mut __FUNCTION_PTRS
                .u_source_effect_bit_crusher_preset_set_sample_rate_modulators,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSampleRateModulator"),
            &raw mut __FUNCTION_PTRS
                .u_source_effect_bit_crusher_preset_set_sample_rate_modulator,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSampleRate"),
            &raw mut __FUNCTION_PTRS.u_source_effect_bit_crusher_preset_set_sample_rate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetModulationSettings"),
            &raw mut __FUNCTION_PTRS
                .u_source_effect_bit_crusher_preset_set_modulation_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBits"),
            &raw mut __FUNCTION_PTRS.u_source_effect_bit_crusher_preset_set_bits,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBitModulators"),
            &raw mut __FUNCTION_PTRS
                .u_source_effect_bit_crusher_preset_set_bit_modulators,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBitModulator"),
            &raw mut __FUNCTION_PTRS.u_source_effect_bit_crusher_preset_set_bit_modulator,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USourceEffectChorusPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWetModulators"),
            &raw mut __FUNCTION_PTRS.u_source_effect_chorus_preset_set_wet_modulators,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWetModulator"),
            &raw mut __FUNCTION_PTRS.u_source_effect_chorus_preset_set_wet_modulator,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWet"),
            &raw mut __FUNCTION_PTRS.u_source_effect_chorus_preset_set_wet,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSpreadModulators"),
            &raw mut __FUNCTION_PTRS.u_source_effect_chorus_preset_set_spread_modulators,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSpreadModulator"),
            &raw mut __FUNCTION_PTRS.u_source_effect_chorus_preset_set_spread_modulator,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSpread"),
            &raw mut __FUNCTION_PTRS.u_source_effect_chorus_preset_set_spread,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.u_source_effect_chorus_preset_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetModulationSettings"),
            &raw mut __FUNCTION_PTRS
                .u_source_effect_chorus_preset_set_modulation_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFrequencyModulators"),
            &raw mut __FUNCTION_PTRS
                .u_source_effect_chorus_preset_set_frequency_modulators,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFrequencyModulator"),
            &raw mut __FUNCTION_PTRS
                .u_source_effect_chorus_preset_set_frequency_modulator,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFrequency"),
            &raw mut __FUNCTION_PTRS.u_source_effect_chorus_preset_set_frequency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFeedbackModulators"),
            &raw mut __FUNCTION_PTRS
                .u_source_effect_chorus_preset_set_feedback_modulators,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFeedbackModulator"),
            &raw mut __FUNCTION_PTRS.u_source_effect_chorus_preset_set_feedback_modulator,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFeedback"),
            &raw mut __FUNCTION_PTRS.u_source_effect_chorus_preset_set_feedback,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDryModulators"),
            &raw mut __FUNCTION_PTRS.u_source_effect_chorus_preset_set_dry_modulators,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDryModulator"),
            &raw mut __FUNCTION_PTRS.u_source_effect_chorus_preset_set_dry_modulator,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDry"),
            &raw mut __FUNCTION_PTRS.u_source_effect_chorus_preset_set_dry,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDepthModulators"),
            &raw mut __FUNCTION_PTRS.u_source_effect_chorus_preset_set_depth_modulators,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDepthModulator"),
            &raw mut __FUNCTION_PTRS.u_source_effect_chorus_preset_set_depth_modulator,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDepth"),
            &raw mut __FUNCTION_PTRS.u_source_effect_chorus_preset_set_depth,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USourceEffectConvolutionReverbPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS
                .u_source_effect_convolution_reverb_preset_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetImpulseResponse"),
            &raw mut __FUNCTION_PTRS
                .u_source_effect_convolution_reverb_preset_set_impulse_response,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USourceEffectDynamicsProcessorPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS
                .u_source_effect_dynamics_processor_preset_set_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USourceEffectEnvelopeFollowerPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnregisterEnvelopeFollowerListener"),
            &raw mut __FUNCTION_PTRS
                .u_source_effect_envelope_follower_preset_unregister_envelope_follower_listener,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS
                .u_source_effect_envelope_follower_preset_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterEnvelopeFollowerListener"),
            &raw mut __FUNCTION_PTRS
                .u_source_effect_envelope_follower_preset_register_envelope_follower_listener,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USourceEffectEQPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.u_source_effect_eq_preset_set_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USourceEffectFilterPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.u_source_effect_filter_preset_set_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USourceEffectFoldbackDistortionPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS
                .u_source_effect_foldback_distortion_preset_set_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USourceEffectMidSideSpreaderPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS
                .u_source_effect_mid_side_spreader_preset_set_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USourceEffectMotionFilterPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.u_source_effect_motion_filter_preset_set_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USourceEffectPannerPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.u_source_effect_panner_preset_set_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USourceEffectPhaserPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.u_source_effect_phaser_preset_set_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USourceEffectRingModulationPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.u_source_effect_ring_modulation_preset_set_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USourceEffectSimpleDelayPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.u_source_effect_simple_delay_preset_set_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USourceEffectStereoDelayPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.u_source_effect_stereo_delay_preset_set_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USourceEffectWaveShaperPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.u_source_effect_wave_shaper_preset_set_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USubmixEffectConvolutionReverbPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS
                .u_submix_effect_convolution_reverb_preset_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetImpulseResponse"),
            &raw mut __FUNCTION_PTRS
                .u_submix_effect_convolution_reverb_preset_set_impulse_response,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USubmixEffectDelayStatics::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaximumDelayLength"),
            &raw mut __FUNCTION_PTRS
                .u_submix_effect_delay_statics_set_maximum_delay_length,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInterpolationTime"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_delay_statics_set_interpolation_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDelayLength"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_delay_statics_set_delay_length,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USubmixEffectDelayPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_delay_preset_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInterpolationTime"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_delay_preset_set_interpolation_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDelay"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_delay_preset_set_delay,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefaultSettings"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_delay_preset_set_default_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaxDelayInMilliseconds"),
            &raw mut __FUNCTION_PTRS
                .u_submix_effect_delay_preset_get_max_delay_in_milliseconds,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USubmixEffectFilterPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_filter_preset_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterType"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_filter_preset_set_filter_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterQMod"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_filter_preset_set_filter_q_mod,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterQ"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_filter_preset_set_filter_q,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterCutoffFrequencyMod"),
            &raw mut __FUNCTION_PTRS
                .u_submix_effect_filter_preset_set_filter_cutoff_frequency_mod,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterCutoffFrequency"),
            &raw mut __FUNCTION_PTRS
                .u_submix_effect_filter_preset_set_filter_cutoff_frequency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterAlgorithm"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_filter_preset_set_filter_algorithm,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USubmixEffectFlexiverbPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_flexiverb_preset_set_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USubmixEffectMultibandCompressorPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS
                .u_submix_effect_multiband_compressor_preset_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetExternalSubmix"),
            &raw mut __FUNCTION_PTRS
                .u_submix_effect_multiband_compressor_preset_set_external_submix,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAudioBus"),
            &raw mut __FUNCTION_PTRS
                .u_submix_effect_multiband_compressor_preset_set_audio_bus,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetKey"),
            &raw mut __FUNCTION_PTRS
                .u_submix_effect_multiband_compressor_preset_reset_key,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USubmixEffectStereoDelayPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_stereo_delay_preset_set_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USubmixEffectStereoToQuadPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_stereo_to_quad_preset_set_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USubmixEffectTapDelayPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTap"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_tap_delay_preset_set_tap,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_tap_delay_preset_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInterpolationTime"),
            &raw mut __FUNCTION_PTRS
                .u_submix_effect_tap_delay_preset_set_interpolation_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveTap"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_tap_delay_preset_remove_tap,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTapIds"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_tap_delay_preset_get_tap_ids,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTap"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_tap_delay_preset_get_tap,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaxDelayInMilliseconds"),
            &raw mut __FUNCTION_PTRS
                .u_submix_effect_tap_delay_preset_get_max_delay_in_milliseconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddTap"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_tap_delay_preset_add_tap,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGranularSynth::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSustainGain"),
            &raw mut __FUNCTION_PTRS.u_granular_synth_set_sustain_gain,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSoundWave"),
            &raw mut __FUNCTION_PTRS.u_granular_synth_set_sound_wave,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetScrubMode"),
            &raw mut __FUNCTION_PTRS.u_granular_synth_set_scrub_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetReleaseTimeMsec"),
            &raw mut __FUNCTION_PTRS.u_granular_synth_set_release_time_msec,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlayheadTime"),
            &raw mut __FUNCTION_PTRS.u_granular_synth_set_playhead_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlaybackSpeed"),
            &raw mut __FUNCTION_PTRS.u_granular_synth_set_playback_speed,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGrainVolume"),
            &raw mut __FUNCTION_PTRS.u_granular_synth_set_grain_volume,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGrainsPerSecond"),
            &raw mut __FUNCTION_PTRS.u_granular_synth_set_grains_per_second,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGrainProbability"),
            &raw mut __FUNCTION_PTRS.u_granular_synth_set_grain_probability,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGrainPitch"),
            &raw mut __FUNCTION_PTRS.u_granular_synth_set_grain_pitch,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGrainPan"),
            &raw mut __FUNCTION_PTRS.u_granular_synth_set_grain_pan,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGrainEnvelopeType"),
            &raw mut __FUNCTION_PTRS.u_granular_synth_set_grain_envelope_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGrainDuration"),
            &raw mut __FUNCTION_PTRS.u_granular_synth_set_grain_duration,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDecayTime"),
            &raw mut __FUNCTION_PTRS.u_granular_synth_set_decay_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAttackTime"),
            &raw mut __FUNCTION_PTRS.u_granular_synth_set_attack_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NoteOn"),
            &raw mut __FUNCTION_PTRS.u_granular_synth_note_on,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NoteOff"),
            &raw mut __FUNCTION_PTRS.u_granular_synth_note_off,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLoaded"),
            &raw mut __FUNCTION_PTRS.u_granular_synth_is_loaded,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSampleDuration"),
            &raw mut __FUNCTION_PTRS.u_granular_synth_get_sample_duration,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentPlayheadTime"),
            &raw mut __FUNCTION_PTRS.u_granular_synth_get_current_playhead_time,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USynthComponentMonoWaveTable::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWaveTablePosition"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_wave_table_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSustainPedalState"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_sustain_pedal_state,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPosLfoType"),
            &raw mut __FUNCTION_PTRS.u_synth_component_mono_wave_table_set_pos_lfo_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPosLfoFrequency"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_pos_lfo_frequency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPosLfoDepth"),
            &raw mut __FUNCTION_PTRS.u_synth_component_mono_wave_table_set_pos_lfo_depth,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPositionEnvelopeSustainGain"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_position_envelope_sustain_gain,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPositionEnvelopeReleaseTime"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_position_envelope_release_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPositionEnvelopeInvert"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_position_envelope_invert,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPositionEnvelopeDepth"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_position_envelope_depth,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPositionEnvelopeDecayTime"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_position_envelope_decay_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPositionEnvelopeBiasInvert"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_position_envelope_bias_invert,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPositionEnvelopeBiasDepth"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_position_envelope_bias_depth,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPositionEnvelopeAttackTime"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_position_envelope_attack_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLowPassFilterResonance"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_low_pass_filter_resonance,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFrequencyWithMidiNote"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_frequency_with_midi_note,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFrequencyPitchBend"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_frequency_pitch_bend,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFrequency"),
            &raw mut __FUNCTION_PTRS.u_synth_component_mono_wave_table_set_frequency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterEnvelopeSustainGain"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_filter_envelope_sustain_gain,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterEnvelopeReleaseTime"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_filter_envelope_release_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterEnvelopenDecayTime"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_filter_envelopen_decay_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterEnvelopeInvert"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_filter_envelope_invert,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterEnvelopeDepth"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_filter_envelope_depth,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterEnvelopeBiasInvert"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_filter_envelope_bias_invert,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterEnvelopeBiasDepth"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_filter_envelope_bias_depth,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFilterEnvelopeAttackTime"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_filter_envelope_attack_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCurveValue"),
            &raw mut __FUNCTION_PTRS.u_synth_component_mono_wave_table_set_curve_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCurveTangent"),
            &raw mut __FUNCTION_PTRS.u_synth_component_mono_wave_table_set_curve_tangent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCurveInterpolationType"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_curve_interpolation_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAmpEnvelopeSustainGain"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_amp_envelope_sustain_gain,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAmpEnvelopeReleaseTime"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_amp_envelope_release_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAmpEnvelopeInvert"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_amp_envelope_invert,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAmpEnvelopeDepth"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_amp_envelope_depth,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAmpEnvelopeDecayTime"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_amp_envelope_decay_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAmpEnvelopeBiasInvert"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_amp_envelope_bias_invert,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAmpEnvelopeBiasDepth"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_amp_envelope_bias_depth,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAmpEnvelopeAttackTime"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_set_amp_envelope_attack_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RefreshWaveTable"),
            &raw mut __FUNCTION_PTRS.u_synth_component_mono_wave_table_refresh_wave_table,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RefreshAllWaveTables"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_refresh_all_wave_tables,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NoteOn"),
            &raw mut __FUNCTION_PTRS.u_synth_component_mono_wave_table_note_on,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NoteOff"),
            &raw mut __FUNCTION_PTRS.u_synth_component_mono_wave_table_note_off,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumTableEntries"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_get_num_table_entries,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaxTableIndex"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_get_max_table_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeyFrameValuesForTable"),
            &raw mut __FUNCTION_PTRS
                .u_synth_component_mono_wave_table_get_key_frame_values_for_table,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurveTangent"),
            &raw mut __FUNCTION_PTRS.u_synth_component_mono_wave_table_get_curve_tangent,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USynthComponentToneGenerator::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVolume"),
            &raw mut __FUNCTION_PTRS.u_synth_component_tone_generator_set_volume,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFrequency"),
            &raw mut __FUNCTION_PTRS.u_synth_component_tone_generator_set_frequency,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USynthSamplePlayer::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSoundWave"),
            &raw mut __FUNCTION_PTRS.u_synth_sample_player_set_sound_wave,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetScrubTimeWidth"),
            &raw mut __FUNCTION_PTRS.u_synth_sample_player_set_scrub_time_width,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetScrubMode"),
            &raw mut __FUNCTION_PTRS.u_synth_sample_player_set_scrub_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPitch"),
            &raw mut __FUNCTION_PTRS.u_synth_sample_player_set_pitch,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SeekToTime"),
            &raw mut __FUNCTION_PTRS.u_synth_sample_player_seek_to_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLoaded"),
            &raw mut __FUNCTION_PTRS.u_synth_sample_player_is_loaded,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSampleDuration"),
            &raw mut __FUNCTION_PTRS.u_synth_sample_player_get_sample_duration,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentPlaybackProgressTime"),
            &raw mut __FUNCTION_PTRS
                .u_synth_sample_player_get_current_playback_progress_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentPlaybackProgressPercent"),
            &raw mut __FUNCTION_PTRS
                .u_synth_sample_player_get_current_playback_progress_percent,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USynthesisUtilitiesBlueprintFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLogFrequency"),
            &raw mut __FUNCTION_PTRS
                .u_synthesis_utilities_blueprint_function_library_get_log_frequency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLinearFrequency"),
            &raw mut __FUNCTION_PTRS
                .u_synthesis_utilities_blueprint_function_library_get_linear_frequency,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USynth2DSlider::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValue"),
            &raw mut __FUNCTION_PTRS.u_synth2_d_slider_set_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStepSize"),
            &raw mut __FUNCTION_PTRS.u_synth2_d_slider_set_step_size,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSliderHandleColor"),
            &raw mut __FUNCTION_PTRS.u_synth2_d_slider_set_slider_handle_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocked"),
            &raw mut __FUNCTION_PTRS.u_synth2_d_slider_set_locked,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIndentHandle"),
            &raw mut __FUNCTION_PTRS.u_synth2_d_slider_set_indent_handle,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut __FUNCTION_PTRS.u_synth2_d_slider_get_value,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USynthKnob::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValue"),
            &raw mut __FUNCTION_PTRS.u_synth_knob_set_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStepSize"),
            &raw mut __FUNCTION_PTRS.u_synth_knob_set_step_size,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocked"),
            &raw mut __FUNCTION_PTRS.u_synth_knob_set_locked,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut __FUNCTION_PTRS.u_synth_knob_get_value,
        );
    }
}
#[repr(C, align(4))]
pub struct FSynth1PatchCable {
    pub depth: f32,
    pub destination: ESynth1PatchDestination,
}
impl FSynth1PatchCable {}
#[repr(C, align(4))]
pub struct FPatchId {
    pub(crate) __padding_end: [u8; 4],
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
    pub(crate) __padding_8: [u8; 8],
    pub flags_8: u8,
    #[doc(hidden)]
    pub(crate) __padding_12: [u8; 3],
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
    pub(crate) __padding_124: [u8; 2],
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
    pub(crate) __padding_164: [u8; 2],
    pub flags_164: u8,
    #[doc(hidden)]
    pub(crate) __padding_168: [u8; 3],
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
    pub(crate) __padding_24: [u8; 24],
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
    pub(crate) __padding_4: [u8; 4],
    pub wet_volume_db: f32,
    pub dry_volume_db: f32,
    pub b_bypass: bool,
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
    pub(crate) __padding_40: [u8; 3],
    pub flags_40: u8,
}
impl FSourceEffectDynamicsProcessorSettings {}
#[repr(C, align(4))]
pub struct FSourceEffectEnvelopeFollowerSettings {
    pub attack_time: f32,
    pub release_time: f32,
    pub peak_mode: EEnvelopeFollowerPeakMode,
    pub b_is_analog_mode: bool,
}
impl FSourceEffectEnvelopeFollowerSettings {}
#[repr(C, align(4))]
pub struct FSourceEffectEQBand {
    pub frequency: f32,
    pub bandwidth: f32,
    pub gain_db: f32,
    pub flags_12: u8,
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
    pub(crate) __padding_24: [u8; 4],
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
    pub(crate) __padding_4: [u8; 4],
    pub wet_volume_db: f32,
    pub dry_volume_db: f32,
    pub b_bypass: bool,
    pub b_mix_input_channel_format_to_impulse_response_format: bool,
    pub b_mix_reverb_output_to_output_channel_format: bool,
    pub surround_rear_channel_bleed_db: f32,
    pub b_invert_rear_channel_bleed_phase: bool,
    pub b_surround_rear_channel_flip: bool,
    pub(crate) __padding_end: [u8; 26],
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
    pub(crate) __padding_end: [u8; 4],
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
    pub(crate) __padding_16: [u8; 16],
    pub normal_thumb_image: crate::bindings::slate_core::FSlateBrush,
    pub disabled_thumb_image: crate::bindings::slate_core::FSlateBrush,
    pub normal_bar_image: crate::bindings::slate_core::FSlateBrush,
    pub disabled_bar_image: crate::bindings::slate_core::FSlateBrush,
    pub background_image: crate::bindings::slate_core::FSlateBrush,
    pub bar_thickness: f32,
}
impl FSynth2DSliderStyle {}
#[repr(C, align(16))]
pub struct FSynthKnobStyle {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub large_knob: crate::bindings::slate_core::FSlateBrush,
    pub large_knob_overlay: crate::bindings::slate_core::FSlateBrush,
    pub medium_knob: crate::bindings::slate_core::FSlateBrush,
    pub medium_knob_overlay: crate::bindings::slate_core::FSlateBrush,
    pub min_value_angle: f32,
    pub max_value_angle: f32,
    pub knob_size: ESynthKnobSize,
}
impl FSynthKnobStyle {}
#[repr(C, align(8))]
pub struct FSynthSlateStyle {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub size_type: ESynthSlateSizeType,
    pub color_style: ESynthSlateColorStyle,
}
impl FSynthSlateStyle {}
#[repr(C, align(8))]
pub struct UAudioImpulseResponse {
    #[doc(hidden)]
    pub(crate) __padding_72: [u8; 72],
    pub normalization_volume_db: f32,
    pub b_true_stereo: bool,
    __padding_end: [u8; 59],
}
impl UAudioImpulseResponse {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioImpulseResponse")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UModularSynthPresetBank {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub presets: TArray<FModularSynthPresetBankEntry>,
}
impl UModularSynthPresetBank {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModularSynthPresetBank")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UModularSynthLibrary {
    __padding_end: [u8; 48],
}
impl UModularSynthLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModularSynthLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn add_modular_synth_preset_to_bank_asset(
        in_bank: UPtr<UModularSynthPresetBank>,
        preset: &FModularSynthPreset,
        preset_name: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<248>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_library_add_modular_synth_preset_to_bank_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_bank,
                __buffer.add(0).cast::<UPtr<UModularSynthPresetBank>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                preset,
                __buffer.add(8).cast::<FModularSynthPreset>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &preset_name,
                __buffer.add(232).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::synthesis::UModularSynthLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_library_add_modular_synth_preset_to_bank_asset,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct UModularSynthComponent {
    #[doc(hidden)]
    pub(crate) __padding_2384: [u8; 2384],
    pub voice_count: i32,
    __padding_end: [u8; 1788],
}
impl UModularSynthComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModularSynthComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_synth_preset(&mut self, synth_preset: &FModularSynthPreset) {
        let mut __stack = crate::core_data::StackAlloc::<224>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_synth_preset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                synth_preset,
                __buffer.add(0).cast::<FModularSynthPreset>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_synth_preset,
                __buffer,
            )
        };
    }
    pub fn set_sustain_gain(&mut self, sustain_gain: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_sustain_gain,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sustain_gain,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_sustain_gain,
                __buffer,
            )
        };
    }
    pub fn set_stereo_delay_wetlevel(&mut self, delay_wetlevel: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_stereo_delay_wetlevel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delay_wetlevel,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_stereo_delay_wetlevel,
                __buffer,
            )
        };
    }
    pub fn set_stereo_delay_time(&mut self, delay_time_msec: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_stereo_delay_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delay_time_msec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_stereo_delay_time,
                __buffer,
            )
        };
    }
    pub fn set_stereo_delay_ratio(&mut self, delay_ratio: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_stereo_delay_ratio,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delay_ratio,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_stereo_delay_ratio,
                __buffer,
            )
        };
    }
    pub fn set_stereo_delay_mode(&mut self, stereo_delay_mode: ESynthStereoDelayMode) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_stereo_delay_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &stereo_delay_mode,
                __buffer.add(0).cast::<ESynthStereoDelayMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_stereo_delay_mode,
                __buffer,
            )
        };
    }
    pub fn set_stereo_delay_is_enabled(&mut self, stereo_delay_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_stereo_delay_is_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &stereo_delay_enabled,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_stereo_delay_is_enabled,
                __buffer,
            )
        };
    }
    pub fn set_stereo_delay_feedback(&mut self, delay_feedback: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_stereo_delay_feedback,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delay_feedback,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_stereo_delay_feedback,
                __buffer,
            )
        };
    }
    pub fn set_spread(&mut self, spread: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_spread,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&spread, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_spread,
                __buffer,
            )
        };
    }
    pub fn set_release_time(&mut self, release_time_msec: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_release_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &release_time_msec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_release_time,
                __buffer,
            )
        };
    }
    pub fn set_portamento(&mut self, portamento: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_portamento,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&portamento, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_portamento,
                __buffer,
            )
        };
    }
    pub fn set_pitch_bend(&mut self, pitch_bend: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_pitch_bend,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&pitch_bend, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_pitch_bend,
                __buffer,
            )
        };
    }
    pub fn set_pan(&mut self, pan: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_pan,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&pan, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_pan,
                __buffer,
            )
        };
    }
    pub fn set_osc_type(&mut self, osc_index: i32, osc_type: ESynth1OscType) {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_osc_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&osc_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &osc_type,
                __buffer.add(4).cast::<ESynth1OscType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_osc_type,
                __buffer,
            )
        };
    }
    pub fn set_osc_sync(&mut self, b_is_synced: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_osc_sync,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_synced,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_osc_sync,
                __buffer,
            )
        };
    }
    pub fn set_osc_semitones(&mut self, osc_index: i32, semitones: f32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_osc_semitones,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&osc_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&semitones, __buffer.add(4).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_osc_semitones,
                __buffer,
            )
        };
    }
    pub fn set_osc_pulsewidth(&mut self, osc_index: i32, pulsewidth: f32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_osc_pulsewidth,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&osc_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&pulsewidth, __buffer.add(4).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_osc_pulsewidth,
                __buffer,
            )
        };
    }
    pub fn set_osc_octave(&mut self, osc_index: i32, octave: f32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_osc_octave,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&osc_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&octave, __buffer.add(4).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_osc_octave,
                __buffer,
            )
        };
    }
    pub fn set_osc_gain_mod(&mut self, osc_index: i32, osc_gain_mod: f32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_osc_gain_mod,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&osc_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &osc_gain_mod,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_osc_gain_mod,
                __buffer,
            )
        };
    }
    pub fn set_osc_gain(&mut self, osc_index: i32, osc_gain: f32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_osc_gain,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&osc_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&osc_gain, __buffer.add(4).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_osc_gain,
                __buffer,
            )
        };
    }
    pub fn set_osc_frequency_mod(&mut self, osc_index: i32, osc_freq_mod: f32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_osc_frequency_mod,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&osc_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &osc_freq_mod,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_osc_frequency_mod,
                __buffer,
            )
        };
    }
    pub fn set_osc_cents(&mut self, osc_index: i32, cents: f32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_osc_cents,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&osc_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&cents, __buffer.add(4).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_osc_cents,
                __buffer,
            )
        };
    }
    pub fn set_mod_env_sustain_gain(&mut self, sustain_gain: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_mod_env_sustain_gain,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sustain_gain,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_mod_env_sustain_gain,
                __buffer,
            )
        };
    }
    pub fn set_mod_env_release_time(&mut self, release: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_mod_env_release_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&release, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_mod_env_release_time,
                __buffer,
            )
        };
    }
    pub fn set_mod_env_patch(&mut self, in_patch_type: ESynthModEnvPatch) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_mod_env_patch,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_patch_type,
                __buffer.add(0).cast::<ESynthModEnvPatch>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_mod_env_patch,
                __buffer,
            )
        };
    }
    pub fn set_mod_env_invert(&mut self, b_invert: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_mod_env_invert,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_invert, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_mod_env_invert,
                __buffer,
            )
        };
    }
    pub fn set_mod_env_depth(&mut self, depth: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_mod_env_depth,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&depth, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_mod_env_depth,
                __buffer,
            )
        };
    }
    pub fn set_mod_env_decay_time(&mut self, decay_time_msec: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_mod_env_decay_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &decay_time_msec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_mod_env_decay_time,
                __buffer,
            )
        };
    }
    pub fn set_mod_env_bias_patch(&mut self, in_patch_type: ESynthModEnvBiasPatch) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_mod_env_bias_patch,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_patch_type,
                __buffer.add(0).cast::<ESynthModEnvBiasPatch>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_mod_env_bias_patch,
                __buffer,
            )
        };
    }
    pub fn set_mod_env_bias_invert(&mut self, b_invert: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_mod_env_bias_invert,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_invert, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_mod_env_bias_invert,
                __buffer,
            )
        };
    }
    pub fn set_mod_env_attack_time(&mut self, attack_time_msec: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_mod_env_attack_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attack_time_msec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_mod_env_attack_time,
                __buffer,
            )
        };
    }
    pub fn set_lfo_type(&mut self, lfo_index: i32, lfo_type: ESynthLFOType) {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_lfo_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&lfo_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &lfo_type,
                __buffer.add(4).cast::<ESynthLFOType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_lfo_type,
                __buffer,
            )
        };
    }
    pub fn set_lfo_patch(&mut self, lfo_index: i32, lfo_patch_type: ESynthLFOPatchType) {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_lfo_patch,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&lfo_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &lfo_patch_type,
                __buffer.add(4).cast::<ESynthLFOPatchType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_lfo_patch,
                __buffer,
            )
        };
    }
    pub fn set_lfo_mode(&mut self, lfo_index: i32, lfo_mode: ESynthLFOMode) {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_lfo_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&lfo_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &lfo_mode,
                __buffer.add(4).cast::<ESynthLFOMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_lfo_mode,
                __buffer,
            )
        };
    }
    pub fn set_lfo_gain_mod(&mut self, lfo_index: i32, gain_mod: f32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_lfo_gain_mod,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&lfo_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&gain_mod, __buffer.add(4).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_lfo_gain_mod,
                __buffer,
            )
        };
    }
    pub fn set_lfo_gain(&mut self, lfo_index: i32, gain: f32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_lfo_gain,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&lfo_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&gain, __buffer.add(4).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_lfo_gain,
                __buffer,
            )
        };
    }
    pub fn set_lfo_frequency_mod(&mut self, lfo_index: i32, frequency_mod_hz: f32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_lfo_frequency_mod,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&lfo_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frequency_mod_hz,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_lfo_frequency_mod,
                __buffer,
            )
        };
    }
    pub fn set_lfo_frequency(&mut self, lfo_index: i32, frequency_hz: f32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_lfo_frequency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&lfo_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frequency_hz,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_lfo_frequency,
                __buffer,
            )
        };
    }
    pub fn set_gain_db(&mut self, gain_db: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_gain_db,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&gain_db, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_gain_db,
                __buffer,
            )
        };
    }
    pub fn set_filter_type(&mut self, filter_type: ESynthFilterType) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_filter_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_type,
                __buffer.add(0).cast::<ESynthFilterType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_filter_type,
                __buffer,
            )
        };
    }
    pub fn set_filter_q_mod(&mut self, filter_q: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_filter_q_mod,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&filter_q, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_filter_q_mod,
                __buffer,
            )
        };
    }
    pub fn set_filter_q(&mut self, filter_q: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_filter_q,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&filter_q, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_filter_q,
                __buffer,
            )
        };
    }
    pub fn set_filter_frequency_mod(&mut self, filter_frequency_hz: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_filter_frequency_mod,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_frequency_hz,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_filter_frequency_mod,
                __buffer,
            )
        };
    }
    pub fn set_filter_frequency(&mut self, filter_frequency_hz: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_filter_frequency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_frequency_hz,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_filter_frequency,
                __buffer,
            )
        };
    }
    pub fn set_filter_algorithm(&mut self, filter_algorithm: ESynthFilterAlgorithm) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_filter_algorithm,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_algorithm,
                __buffer.add(0).cast::<ESynthFilterAlgorithm>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_filter_algorithm,
                __buffer,
            )
        };
    }
    pub fn set_enable_unison(&mut self, enable_unison: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_enable_unison,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &enable_unison,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_enable_unison,
                __buffer,
            )
        };
    }
    pub fn set_enable_retrigger(&mut self, retrigger_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_enable_retrigger,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &retrigger_enabled,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_enable_retrigger,
                __buffer,
            )
        };
    }
    pub fn set_enable_polyphony(&mut self, b_enable_polyphony: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_enable_polyphony,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_enable_polyphony,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_enable_polyphony,
                __buffer,
            )
        };
    }
    pub fn set_enable_patch(&mut self, patch_id: FPatchId, b_is_enabled: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<6>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_enable_patch,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &patch_id,
                __buffer.add(0).cast::<FPatchId>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_enabled,
                __buffer.add(4).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_enable_patch,
                __buffer,
            )
        };
        unsafe { __buffer.add(5).cast::<bool>().read() }
    }
    pub fn set_enable_legato(&mut self, legato_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_enable_legato,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &legato_enabled,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_enable_legato,
                __buffer,
            )
        };
    }
    pub fn set_decay_time(&mut self, decay_time_msec: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_decay_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &decay_time_msec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_decay_time,
                __buffer,
            )
        };
    }
    pub fn set_chorus_frequency(&mut self, frequency: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_chorus_frequency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&frequency, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_chorus_frequency,
                __buffer,
            )
        };
    }
    pub fn set_chorus_feedback(&mut self, feedback: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_chorus_feedback,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&feedback, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_chorus_feedback,
                __buffer,
            )
        };
    }
    pub fn set_chorus_enabled(&mut self, enable_chorus: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_chorus_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &enable_chorus,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_chorus_enabled,
                __buffer,
            )
        };
    }
    pub fn set_chorus_depth(&mut self, depth: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_chorus_depth,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&depth, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_chorus_depth,
                __buffer,
            )
        };
    }
    pub fn set_attack_time(&mut self, attack_time_msec: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_attack_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attack_time_msec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_set_attack_time,
                __buffer,
            )
        };
    }
    pub fn note_on(&mut self, note: f32, velocity: i32, duration: f32) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_note_on,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&note, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&velocity, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(8).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_note_on,
                __buffer,
            )
        };
    }
    pub fn note_off(
        &mut self,
        note: f32,
        b_all_notes_off: bool,
        b_kill_all_notes: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<6>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_note_off,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&note, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_all_notes_off,
                __buffer.add(4).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_kill_all_notes,
                __buffer.add(5).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_note_off,
                __buffer,
            )
        };
    }
    pub fn create_patch(
        &mut self,
        patch_source: ESynth1PatchSource,
        patch_cables: &TArray<FSynth1PatchCable>,
        b_enable_by_default: bool,
    ) -> FPatchId {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_create_patch,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &patch_source,
                __buffer.add(0).cast::<ESynth1PatchSource>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                patch_cables,
                __buffer.add(8).cast::<TArray<FSynth1PatchCable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_enable_by_default,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_modular_synth_component_create_patch,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<FPatchId>().read() }
    }
}
#[repr(C, align(8))]
pub struct USourceEffectBitCrusherPreset {
    #[doc(hidden)]
    pub(crate) __padding_344: [u8; 344],
    pub settings: FSourceEffectBitCrusherSettings,
}
impl USourceEffectBitCrusherPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceEffectBitCrusherPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(&mut self, settings: &FSourceEffectBitCrusherBaseSettings) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_bit_crusher_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                settings,
                __buffer.add(0).cast::<FSourceEffectBitCrusherBaseSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_bit_crusher_preset_set_settings,
                __buffer,
            )
        };
    }
    pub fn set_sample_rate_modulators(
        &mut self,
        in_modulators: &TSet<
            UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_bit_crusher_preset_set_sample_rate_modulators,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_modulators,
                __buffer
                    .add(0)
                    .cast::<
                        TSet<
                            UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_bit_crusher_preset_set_sample_rate_modulators,
                __buffer,
            )
        };
    }
    pub fn set_sample_rate_modulator(
        &mut self,
        modulator: UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_bit_crusher_preset_set_sample_rate_modulator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &modulator,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_bit_crusher_preset_set_sample_rate_modulator,
                __buffer,
            )
        };
    }
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_bit_crusher_preset_set_sample_rate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sample_rate,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_bit_crusher_preset_set_sample_rate,
                __buffer,
            )
        };
    }
    pub fn set_modulation_settings(
        &mut self,
        modulation_settings: &FSourceEffectBitCrusherSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<192>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_bit_crusher_preset_set_modulation_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                modulation_settings,
                __buffer.add(0).cast::<FSourceEffectBitCrusherSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_bit_crusher_preset_set_modulation_settings,
                __buffer,
            )
        };
    }
    pub fn set_bits(&mut self, bits: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_bit_crusher_preset_set_bits,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&bits, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_bit_crusher_preset_set_bits,
                __buffer,
            )
        };
    }
    pub fn set_bit_modulators(
        &mut self,
        in_modulators: &TSet<
            UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_bit_crusher_preset_set_bit_modulators,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_modulators,
                __buffer
                    .add(0)
                    .cast::<
                        TSet<
                            UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_bit_crusher_preset_set_bit_modulators,
                __buffer,
            )
        };
    }
    pub fn set_bit_modulator(
        &mut self,
        modulator: UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_bit_crusher_preset_set_bit_modulator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &modulator,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_bit_crusher_preset_set_bit_modulator,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USourceEffectChorusPreset {
    #[doc(hidden)]
    pub(crate) __padding_752: [u8; 752],
    pub settings: FSourceEffectChorusSettings,
}
impl USourceEffectChorusPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceEffectChorusPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_wet_modulators(
        &mut self,
        modulators: &TSet<UPtr<crate::bindings::audio_extensions::USoundModulatorBase>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_wet_modulators,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                modulators,
                __buffer
                    .add(0)
                    .cast::<
                        TSet<
                            UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_wet_modulators,
                __buffer,
            )
        };
    }
    pub fn set_wet_modulator(
        &mut self,
        modulator: UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_wet_modulator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &modulator,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_wet_modulator,
                __buffer,
            )
        };
    }
    pub fn set_wet(&mut self, wet_amount: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_wet,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&wet_amount, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_wet,
                __buffer,
            )
        };
    }
    pub fn set_spread_modulators(
        &mut self,
        modulators: &TSet<UPtr<crate::bindings::audio_extensions::USoundModulatorBase>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_spread_modulators,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                modulators,
                __buffer
                    .add(0)
                    .cast::<
                        TSet<
                            UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_spread_modulators,
                __buffer,
            )
        };
    }
    pub fn set_spread_modulator(
        &mut self,
        modulator: UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_spread_modulator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &modulator,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_spread_modulator,
                __buffer,
            )
        };
    }
    pub fn set_spread(&mut self, spread: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_spread,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&spread, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_spread,
                __buffer,
            )
        };
    }
    pub fn set_settings(&mut self, settings: &FSourceEffectChorusBaseSettings) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                settings,
                __buffer.add(0).cast::<FSourceEffectChorusBaseSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_settings,
                __buffer,
            )
        };
    }
    pub fn set_modulation_settings(
        &mut self,
        modulation_settings: &FSourceEffectChorusSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<600>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_modulation_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                modulation_settings,
                __buffer.add(0).cast::<FSourceEffectChorusSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_modulation_settings,
                __buffer,
            )
        };
    }
    pub fn set_frequency_modulators(
        &mut self,
        modulators: &TSet<UPtr<crate::bindings::audio_extensions::USoundModulatorBase>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_frequency_modulators,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                modulators,
                __buffer
                    .add(0)
                    .cast::<
                        TSet<
                            UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_frequency_modulators,
                __buffer,
            )
        };
    }
    pub fn set_frequency_modulator(
        &mut self,
        modulator: UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_frequency_modulator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &modulator,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_frequency_modulator,
                __buffer,
            )
        };
    }
    pub fn set_frequency(&mut self, frequency: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_frequency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&frequency, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_frequency,
                __buffer,
            )
        };
    }
    pub fn set_feedback_modulators(
        &mut self,
        modulators: &TSet<UPtr<crate::bindings::audio_extensions::USoundModulatorBase>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_feedback_modulators,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                modulators,
                __buffer
                    .add(0)
                    .cast::<
                        TSet<
                            UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_feedback_modulators,
                __buffer,
            )
        };
    }
    pub fn set_feedback_modulator(
        &mut self,
        modulator: UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_feedback_modulator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &modulator,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_feedback_modulator,
                __buffer,
            )
        };
    }
    pub fn set_feedback(&mut self, feedback: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_feedback,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&feedback, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_feedback,
                __buffer,
            )
        };
    }
    pub fn set_dry_modulators(
        &mut self,
        modulators: &TSet<UPtr<crate::bindings::audio_extensions::USoundModulatorBase>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_dry_modulators,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                modulators,
                __buffer
                    .add(0)
                    .cast::<
                        TSet<
                            UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_dry_modulators,
                __buffer,
            )
        };
    }
    pub fn set_dry_modulator(
        &mut self,
        modulator: UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_dry_modulator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &modulator,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_dry_modulator,
                __buffer,
            )
        };
    }
    pub fn set_dry(&mut self, dry_amount: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_dry,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&dry_amount, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_dry,
                __buffer,
            )
        };
    }
    pub fn set_depth_modulators(
        &mut self,
        modulators: &TSet<UPtr<crate::bindings::audio_extensions::USoundModulatorBase>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_depth_modulators,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                modulators,
                __buffer
                    .add(0)
                    .cast::<
                        TSet<
                            UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_depth_modulators,
                __buffer,
            )
        };
    }
    pub fn set_depth_modulator(
        &mut self,
        modulator: UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_depth_modulator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &modulator,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::audio_extensions::USoundModulatorBase>,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_depth_modulator,
                __buffer,
            )
        };
    }
    pub fn set_depth(&mut self, depth: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_depth,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&depth, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_chorus_preset_set_depth,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USourceEffectConvolutionReverbPreset {
    #[doc(hidden)]
    pub(crate) __padding_112: [u8; 112],
    pub impulse_response: UPtr<UAudioImpulseResponse>,
    pub settings: FSourceEffectConvolutionReverbSettings,
    pub block_size: ESubmixEffectConvolutionReverbBlockSize,
    pub b_enable_hardware_acceleration: bool,
    __padding_end: [u8; 142],
}
impl USourceEffectConvolutionReverbPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceEffectConvolutionReverbPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(
        &mut self,
        in_settings: &FSourceEffectConvolutionReverbSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_convolution_reverb_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSourceEffectConvolutionReverbSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_convolution_reverb_preset_set_settings,
                __buffer,
            )
        };
    }
    pub fn set_impulse_response(
        &mut self,
        in_impulse_response: UPtr<UAudioImpulseResponse>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_convolution_reverb_preset_set_impulse_response,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_impulse_response,
                __buffer.add(0).cast::<UPtr<UAudioImpulseResponse>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_convolution_reverb_preset_set_impulse_response,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USourceEffectDynamicsProcessorPreset {
    #[doc(hidden)]
    pub(crate) __padding_196: [u8; 196],
    pub settings: FSourceEffectDynamicsProcessorSettings,
}
impl USourceEffectDynamicsProcessorPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceEffectDynamicsProcessorPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(
        &mut self,
        in_settings: &FSourceEffectDynamicsProcessorSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_dynamics_processor_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSourceEffectDynamicsProcessorSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_dynamics_processor_preset_set_settings,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UEnvelopeFollowerListener {
    __padding_end: [u8; 280],
}
impl UEnvelopeFollowerListener {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvelopeFollowerListener")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USourceEffectEnvelopeFollowerPreset {
    #[doc(hidden)]
    pub(crate) __padding_164: [u8; 164],
    pub settings: FSourceEffectEnvelopeFollowerSettings,
}
impl USourceEffectEnvelopeFollowerPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceEffectEnvelopeFollowerPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn unregister_envelope_follower_listener(
        &mut self,
        envelope_follower_listener: UPtr<UEnvelopeFollowerListener>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_envelope_follower_preset_unregister_envelope_follower_listener,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &envelope_follower_listener,
                __buffer.add(0).cast::<UPtr<UEnvelopeFollowerListener>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_envelope_follower_preset_unregister_envelope_follower_listener,
                __buffer,
            )
        };
    }
    pub fn set_settings(&mut self, in_settings: &FSourceEffectEnvelopeFollowerSettings) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_envelope_follower_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSourceEffectEnvelopeFollowerSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_envelope_follower_preset_set_settings,
                __buffer,
            )
        };
    }
    pub fn register_envelope_follower_listener(
        &mut self,
        envelope_follower_listener: UPtr<UEnvelopeFollowerListener>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_envelope_follower_preset_register_envelope_follower_listener,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &envelope_follower_listener,
                __buffer.add(0).cast::<UPtr<UEnvelopeFollowerListener>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_envelope_follower_preset_register_envelope_follower_listener,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USourceEffectEQPreset {
    #[doc(hidden)]
    pub(crate) __padding_168: [u8; 168],
    pub settings: FSourceEffectEQSettings,
}
impl USourceEffectEQPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceEffectEQPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(&mut self, in_settings: &FSourceEffectEQSettings) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_eq_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSourceEffectEQSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_eq_preset_set_settings,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USourceEffectFilterPreset {
    #[doc(hidden)]
    pub(crate) __padding_184: [u8; 184],
    pub settings: FSourceEffectFilterSettings,
}
impl USourceEffectFilterPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceEffectFilterPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(&mut self, in_settings: &FSourceEffectFilterSettings) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_filter_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSourceEffectFilterSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_filter_preset_set_settings,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USourceEffectFoldbackDistortionPreset {
    #[doc(hidden)]
    pub(crate) __padding_164: [u8; 164],
    pub settings: FSourceEffectFoldbackDistortionSettings,
}
impl USourceEffectFoldbackDistortionPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceEffectFoldbackDistortionPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(
        &mut self,
        in_settings: &FSourceEffectFoldbackDistortionSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_foldback_distortion_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSourceEffectFoldbackDistortionSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_foldback_distortion_preset_set_settings,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USourceEffectMidSideSpreaderPreset {
    #[doc(hidden)]
    pub(crate) __padding_160: [u8; 160],
    pub settings: FSourceEffectMidSideSpreaderSettings,
}
impl USourceEffectMidSideSpreaderPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceEffectMidSideSpreaderPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(&mut self, in_settings: &FSourceEffectMidSideSpreaderSettings) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_mid_side_spreader_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSourceEffectMidSideSpreaderSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_mid_side_spreader_preset_set_settings,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USourceEffectMotionFilterPreset {
    #[doc(hidden)]
    pub(crate) __padding_272: [u8; 272],
    pub settings: FSourceEffectMotionFilterSettings,
}
impl USourceEffectMotionFilterPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceEffectMotionFilterPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(&mut self, in_settings: &FSourceEffectMotionFilterSettings) {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_motion_filter_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSourceEffectMotionFilterSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_motion_filter_preset_set_settings,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USourceEffectPannerPreset {
    #[doc(hidden)]
    pub(crate) __padding_160: [u8; 160],
    pub settings: FSourceEffectPannerSettings,
}
impl USourceEffectPannerPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceEffectPannerPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(&mut self, in_settings: &FSourceEffectPannerSettings) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_panner_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSourceEffectPannerSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_panner_preset_set_settings,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USourceEffectPhaserPreset {
    #[doc(hidden)]
    pub(crate) __padding_168: [u8; 168],
    pub settings: FSourceEffectPhaserSettings,
}
impl USourceEffectPhaserPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceEffectPhaserPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(&mut self, in_settings: &FSourceEffectPhaserSettings) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_phaser_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSourceEffectPhaserSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_phaser_preset_set_settings,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USourceEffectRingModulationPreset {
    #[doc(hidden)]
    pub(crate) __padding_184: [u8; 184],
    pub settings: FSourceEffectRingModulationSettings,
}
impl USourceEffectRingModulationPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceEffectRingModulationPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(&mut self, in_settings: &FSourceEffectRingModulationSettings) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_ring_modulation_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSourceEffectRingModulationSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_ring_modulation_preset_set_settings,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USourceEffectSimpleDelayPreset {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub settings: FSourceEffectSimpleDelaySettings,
}
impl USourceEffectSimpleDelayPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceEffectSimpleDelayPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(&mut self, in_settings: &FSourceEffectSimpleDelaySettings) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_simple_delay_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSourceEffectSimpleDelaySettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_simple_delay_preset_set_settings,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USourceEffectStereoDelayPreset {
    #[doc(hidden)]
    pub(crate) __padding_188: [u8; 188],
    pub settings: FSourceEffectStereoDelaySettings,
}
impl USourceEffectStereoDelayPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceEffectStereoDelayPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(&mut self, in_settings: &FSourceEffectStereoDelaySettings) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_stereo_delay_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSourceEffectStereoDelaySettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_stereo_delay_preset_set_settings,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USourceEffectWaveShaperPreset {
    #[doc(hidden)]
    pub(crate) __padding_160: [u8; 160],
    pub settings: FSourceEffectWaveShaperSettings,
}
impl USourceEffectWaveShaperPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USourceEffectWaveShaperPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(&mut self, in_settings: &FSourceEffectWaveShaperSettings) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_wave_shaper_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSourceEffectWaveShaperSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_source_effect_wave_shaper_preset_set_settings,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USubmixEffectConvolutionReverbPreset {
    #[doc(hidden)]
    pub(crate) __padding_112: [u8; 112],
    pub impulse_response: UPtr<UAudioImpulseResponse>,
    pub settings: FSubmixEffectConvolutionReverbSettings,
    pub block_size: ESubmixEffectConvolutionReverbBlockSize,
    pub b_enable_hardware_acceleration: bool,
    __padding_end: [u8; 174],
}
impl USubmixEffectConvolutionReverbPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubmixEffectConvolutionReverbPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(
        &mut self,
        in_settings: &FSubmixEffectConvolutionReverbSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_convolution_reverb_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSubmixEffectConvolutionReverbSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_convolution_reverb_preset_set_settings,
                __buffer,
            )
        };
    }
    pub fn set_impulse_response(
        &mut self,
        in_impulse_response: UPtr<UAudioImpulseResponse>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_convolution_reverb_preset_set_impulse_response,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_impulse_response,
                __buffer.add(0).cast::<UPtr<UAudioImpulseResponse>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_convolution_reverb_preset_set_impulse_response,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USubmixEffectDelayStatics {
    __padding_end: [u8; 48],
}
impl USubmixEffectDelayStatics {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubmixEffectDelayStatics")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_maximum_delay_length(
        delay_settings: &mut FSubmixEffectDelaySettings,
        maximum_delay_length: f32,
    ) -> FSubmixEffectDelaySettings {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_delay_statics_set_maximum_delay_length,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                delay_settings,
                __buffer.add(0).cast::<FSubmixEffectDelaySettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &maximum_delay_length,
                __buffer.add(12).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::synthesis::USubmixEffectDelayStatics::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_delay_statics_set_maximum_delay_length,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FSubmixEffectDelaySettings>().swap(delay_settings);
        }
        unsafe { __buffer.add(16).cast::<FSubmixEffectDelaySettings>().read() }
    }
    pub fn set_interpolation_time(
        delay_settings: &mut FSubmixEffectDelaySettings,
        interpolation_time: f32,
    ) -> FSubmixEffectDelaySettings {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_delay_statics_set_interpolation_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                delay_settings,
                __buffer.add(0).cast::<FSubmixEffectDelaySettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interpolation_time,
                __buffer.add(12).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::synthesis::USubmixEffectDelayStatics::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_delay_statics_set_interpolation_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FSubmixEffectDelaySettings>().swap(delay_settings);
        }
        unsafe { __buffer.add(16).cast::<FSubmixEffectDelaySettings>().read() }
    }
    pub fn set_delay_length(
        delay_settings: &mut FSubmixEffectDelaySettings,
        delay_length: f32,
    ) -> FSubmixEffectDelaySettings {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_delay_statics_set_delay_length,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                delay_settings,
                __buffer.add(0).cast::<FSubmixEffectDelaySettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delay_length,
                __buffer.add(12).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::synthesis::USubmixEffectDelayStatics::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_delay_statics_set_delay_length,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FSubmixEffectDelaySettings>().swap(delay_settings);
        }
        unsafe { __buffer.add(16).cast::<FSubmixEffectDelaySettings>().read() }
    }
}
#[repr(C, align(8))]
pub struct USubmixEffectDelayPreset {
    #[doc(hidden)]
    pub(crate) __padding_164: [u8; 164],
    pub settings: FSubmixEffectDelaySettings,
    __padding_end: [u8; 16],
}
impl USubmixEffectDelayPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubmixEffectDelayPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(&mut self, in_settings: &FSubmixEffectDelaySettings) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_delay_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSubmixEffectDelaySettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_delay_preset_set_settings,
                __buffer,
            )
        };
    }
    pub fn set_interpolation_time(&mut self, time: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_delay_preset_set_interpolation_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_delay_preset_set_interpolation_time,
                __buffer,
            )
        };
    }
    pub fn set_delay(&mut self, length: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_delay_preset_set_delay,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&length, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_delay_preset_set_delay,
                __buffer,
            )
        };
    }
    pub fn set_default_settings(&mut self, in_settings: &FSubmixEffectDelaySettings) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_delay_preset_set_default_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSubmixEffectDelaySettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_delay_preset_set_default_settings,
                __buffer,
            )
        };
    }
    pub fn get_max_delay_in_milliseconds(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_delay_preset_get_max_delay_in_milliseconds,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_delay_preset_get_max_delay_in_milliseconds,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct USubmixEffectFilterPreset {
    #[doc(hidden)]
    pub(crate) __padding_164: [u8; 164],
    pub settings: FSubmixEffectFilterSettings,
}
impl USubmixEffectFilterPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubmixEffectFilterPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(&mut self, in_settings: &FSubmixEffectFilterSettings) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_filter_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSubmixEffectFilterSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_filter_preset_set_settings,
                __buffer,
            )
        };
    }
    pub fn set_filter_type(&mut self, in_type: ESubmixFilterType) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_filter_preset_set_filter_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_type,
                __buffer.add(0).cast::<ESubmixFilterType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_filter_preset_set_filter_type,
                __buffer,
            )
        };
    }
    pub fn set_filter_q_mod(&mut self, in_q: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_filter_preset_set_filter_q_mod,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_q, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_filter_preset_set_filter_q_mod,
                __buffer,
            )
        };
    }
    pub fn set_filter_q(&mut self, in_q: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_filter_preset_set_filter_q,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_q, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_filter_preset_set_filter_q,
                __buffer,
            )
        };
    }
    pub fn set_filter_cutoff_frequency_mod(&mut self, in_frequency: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_filter_preset_set_filter_cutoff_frequency_mod,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_frequency,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_filter_preset_set_filter_cutoff_frequency_mod,
                __buffer,
            )
        };
    }
    pub fn set_filter_cutoff_frequency(&mut self, in_frequency: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_filter_preset_set_filter_cutoff_frequency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_frequency,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_filter_preset_set_filter_cutoff_frequency,
                __buffer,
            )
        };
    }
    pub fn set_filter_algorithm(&mut self, in_algorithm: ESubmixFilterAlgorithm) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_filter_preset_set_filter_algorithm,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_algorithm,
                __buffer.add(0).cast::<ESubmixFilterAlgorithm>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_filter_preset_set_filter_algorithm,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USubmixEffectFlexiverbPreset {
    #[doc(hidden)]
    pub(crate) __padding_168: [u8; 168],
    pub settings: FSubmixEffectFlexiverbSettings,
}
impl USubmixEffectFlexiverbPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubmixEffectFlexiverbPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(&mut self, in_settings: &FSubmixEffectFlexiverbSettings) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_flexiverb_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSubmixEffectFlexiverbSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_flexiverb_preset_set_settings,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USubmixEffectMultibandCompressorPreset {
    #[doc(hidden)]
    pub(crate) __padding_208: [u8; 208],
    pub settings: FSubmixEffectMultibandCompressorSettings,
}
impl USubmixEffectMultibandCompressorPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubmixEffectMultibandCompressorPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(
        &mut self,
        in_settings: &FSubmixEffectMultibandCompressorSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_multiband_compressor_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSubmixEffectMultibandCompressorSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_multiband_compressor_preset_set_settings,
                __buffer,
            )
        };
    }
    pub fn set_external_submix(
        &mut self,
        submix: UPtr<crate::bindings::engine::USoundSubmix>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_multiband_compressor_preset_set_external_submix,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_multiband_compressor_preset_set_external_submix,
                __buffer,
            )
        };
    }
    pub fn set_audio_bus(
        &mut self,
        audio_bus: UPtr<crate::bindings::engine::UAudioBus>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_multiband_compressor_preset_set_audio_bus,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &audio_bus,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAudioBus>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_multiband_compressor_preset_set_audio_bus,
                __buffer,
            )
        };
    }
    pub fn reset_key(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_multiband_compressor_preset_reset_key,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_multiband_compressor_preset_reset_key,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USubmixEffectStereoDelayPreset {
    #[doc(hidden)]
    pub(crate) __padding_188: [u8; 188],
    pub settings: FSubmixEffectStereoDelaySettings,
}
impl USubmixEffectStereoDelayPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubmixEffectStereoDelayPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(&mut self, in_settings: &FSubmixEffectStereoDelaySettings) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_stereo_delay_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSubmixEffectStereoDelaySettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_stereo_delay_preset_set_settings,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USubmixEffectStereoToQuadPreset {
    #[doc(hidden)]
    pub(crate) __padding_160: [u8; 160],
    pub settings: FSubmixEffectStereoToQuadSettings,
}
impl USubmixEffectStereoToQuadPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubmixEffectStereoToQuadPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_settings(&mut self, in_settings: &FSubmixEffectStereoToQuadSettings) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_stereo_to_quad_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSubmixEffectStereoToQuadSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_stereo_to_quad_preset_set_settings,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USubmixEffectTapDelayPreset {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub settings: FSubmixEffectTapDelaySettings,
    __padding_end: [u8; 24],
}
impl USubmixEffectTapDelayPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubmixEffectTapDelayPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_tap(&mut self, tap_id: i32, tap_info: &FTapDelayInfo) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_tap_delay_preset_set_tap,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&tap_id, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tap_info,
                __buffer.add(4).cast::<FTapDelayInfo>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_tap_delay_preset_set_tap,
                __buffer,
            )
        };
    }
    pub fn set_settings(&mut self, in_settings: &FSubmixEffectTapDelaySettings) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_tap_delay_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSubmixEffectTapDelaySettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_tap_delay_preset_set_settings,
                __buffer,
            )
        };
    }
    pub fn set_interpolation_time(&mut self, time: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_tap_delay_preset_set_interpolation_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_tap_delay_preset_set_interpolation_time,
                __buffer,
            )
        };
    }
    pub fn remove_tap(&mut self, tap_id: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_tap_delay_preset_remove_tap,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&tap_id, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_tap_delay_preset_remove_tap,
                __buffer,
            )
        };
    }
    pub fn get_tap_ids(&mut self, tap_ids: &mut TArray<i32>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_tap_delay_preset_get_tap_ids,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tap_ids,
                __buffer.add(0).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_tap_delay_preset_get_tap_ids,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<i32>>().swap(tap_ids);
        }
    }
    pub fn get_tap(&mut self, tap_id: i32, tap_info: &mut FTapDelayInfo) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_tap_delay_preset_get_tap,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&tap_id, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tap_info,
                __buffer.add(4).cast::<FTapDelayInfo>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_tap_delay_preset_get_tap,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<FTapDelayInfo>().swap(tap_info);
        }
    }
    pub fn get_max_delay_in_milliseconds(&mut self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_tap_delay_preset_get_max_delay_in_milliseconds,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_tap_delay_preset_get_max_delay_in_milliseconds,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn add_tap(&mut self, tap_id: &mut i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_tap_delay_preset_add_tap,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(tap_id, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_submix_effect_tap_delay_preset_add_tap,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(tap_id);
        }
    }
}
#[repr(C, align(16))]
pub struct UGranularSynth {
    __padding_end: [u8; 3392],
}
impl UGranularSynth {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGranularSynth")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_sustain_gain(&mut self, sustain_gain: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_sustain_gain,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sustain_gain,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_sustain_gain,
                __buffer,
            )
        };
    }
    pub fn set_sound_wave(
        &mut self,
        in_sound_wave: UPtr<crate::bindings::engine::USoundWave>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_sound_wave,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sound_wave,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USoundWave>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_sound_wave,
                __buffer,
            )
        };
    }
    pub fn set_scrub_mode(&mut self, b_scrub_mode: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_scrub_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_scrub_mode,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_scrub_mode,
                __buffer,
            )
        };
    }
    pub fn set_release_time_msec(&mut self, release_time_msec: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_release_time_msec,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &release_time_msec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_release_time_msec,
                __buffer,
            )
        };
    }
    pub fn set_playhead_time(
        &mut self,
        in_position_sec: f32,
        lerp_time_sec: f32,
        seek_type: EGranularSynthSeekType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_playhead_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_position_sec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &lerp_time_sec,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &seek_type,
                __buffer.add(8).cast::<EGranularSynthSeekType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_playhead_time,
                __buffer,
            )
        };
    }
    pub fn set_playback_speed(&mut self, in_playhead_rate: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_playback_speed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_playhead_rate,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_playback_speed,
                __buffer,
            )
        };
    }
    pub fn set_grain_volume(
        &mut self,
        base_volume: f32,
        volume_range: crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_grain_volume,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_volume,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &volume_range,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_grain_volume,
                __buffer,
            )
        };
    }
    pub fn set_grains_per_second(&mut self, in_grains_per_second: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_grains_per_second,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_grains_per_second,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_grains_per_second,
                __buffer,
            )
        };
    }
    pub fn set_grain_probability(&mut self, in_grain_probability: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_grain_probability,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_grain_probability,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_grain_probability,
                __buffer,
            )
        };
    }
    pub fn set_grain_pitch(
        &mut self,
        base_pitch: f32,
        pitch_range: crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_grain_pitch,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&base_pitch, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pitch_range,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_grain_pitch,
                __buffer,
            )
        };
    }
    pub fn set_grain_pan(
        &mut self,
        base_pan: f32,
        pan_range: crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_grain_pan,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&base_pan, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pan_range,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_grain_pan,
                __buffer,
            )
        };
    }
    pub fn set_grain_envelope_type(
        &mut self,
        envelope_type: EGranularSynthEnvelopeType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_grain_envelope_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &envelope_type,
                __buffer.add(0).cast::<EGranularSynthEnvelopeType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_grain_envelope_type,
                __buffer,
            )
        };
    }
    pub fn set_grain_duration(
        &mut self,
        base_duration_msec: f32,
        duration_range: crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_grain_duration,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_duration_msec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &duration_range,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_grain_duration,
                __buffer,
            )
        };
    }
    pub fn set_decay_time(&mut self, decay_time_msec: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_decay_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &decay_time_msec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_decay_time,
                __buffer,
            )
        };
    }
    pub fn set_attack_time(&mut self, attack_time_msec: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_attack_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attack_time_msec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_set_attack_time,
                __buffer,
            )
        };
    }
    pub fn note_on(&mut self, note: f32, velocity: i32, duration: f32) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS.u_granular_synth_note_on,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&note, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&velocity, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(8).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS.u_granular_synth_note_on,
                __buffer,
            )
        };
    }
    pub fn note_off(&mut self, note: f32, b_kill: bool) {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS.u_granular_synth_note_off,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&note, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_kill, __buffer.add(4).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS.u_granular_synth_note_off,
                __buffer,
            )
        };
    }
    pub fn is_loaded(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS.u_granular_synth_is_loaded,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS.u_granular_synth_is_loaded,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_sample_duration(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_get_sample_duration,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_get_sample_duration,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_current_playhead_time(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_get_current_playhead_time,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_granular_synth_get_current_playhead_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMonoWaveTableSynthPreset {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub preset_name: FString,
    pub flags_64: u8,
    pub lock_keyframes_to_grid: i32,
    pub wave_table_resolution: i32,
    pub wave_table: TArray<crate::bindings::engine::FRuntimeFloatCurve>,
    pub flags_96: u8,
    __padding_end: [u8; 279],
}
impl UMonoWaveTableSynthPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMonoWaveTableSynthPreset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(16))]
pub struct USynthComponentMonoWaveTable {
    __padding_end: [u8; 4272],
}
impl USynthComponentMonoWaveTable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USynthComponentMonoWaveTable")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_wave_table_position(&mut self, in_position: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_wave_table_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_position,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_wave_table_position,
                __buffer,
            )
        };
    }
    pub fn set_sustain_pedal_state(&mut self, in_sustain_pedal_state: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_sustain_pedal_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sustain_pedal_state,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_sustain_pedal_state,
                __buffer,
            )
        };
    }
    pub fn set_pos_lfo_type(&mut self, in_lfo_type: ESynthLFOType) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_pos_lfo_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_lfo_type,
                __buffer.add(0).cast::<ESynthLFOType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_pos_lfo_type,
                __buffer,
            )
        };
    }
    pub fn set_pos_lfo_frequency(&mut self, in_lfo_frequency: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_pos_lfo_frequency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_lfo_frequency,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_pos_lfo_frequency,
                __buffer,
            )
        };
    }
    pub fn set_pos_lfo_depth(&mut self, in_lfo_depth: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_pos_lfo_depth,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_lfo_depth,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_pos_lfo_depth,
                __buffer,
            )
        };
    }
    pub fn set_position_envelope_sustain_gain(&mut self, in_sustain_gain: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_position_envelope_sustain_gain,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sustain_gain,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_position_envelope_sustain_gain,
                __buffer,
            )
        };
    }
    pub fn set_position_envelope_release_time(&mut self, in_release_time_msec: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_position_envelope_release_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_release_time_msec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_position_envelope_release_time,
                __buffer,
            )
        };
    }
    pub fn set_position_envelope_invert(&mut self, b_in_invert: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_position_envelope_invert,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_invert,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_position_envelope_invert,
                __buffer,
            )
        };
    }
    pub fn set_position_envelope_depth(&mut self, in_depth: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_position_envelope_depth,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_depth, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_position_envelope_depth,
                __buffer,
            )
        };
    }
    pub fn set_position_envelope_decay_time(&mut self, in_decay_time_msec: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_position_envelope_decay_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_decay_time_msec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_position_envelope_decay_time,
                __buffer,
            )
        };
    }
    pub fn set_position_envelope_bias_invert(&mut self, b_in_bias_invert: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_position_envelope_bias_invert,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_bias_invert,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_position_envelope_bias_invert,
                __buffer,
            )
        };
    }
    pub fn set_position_envelope_bias_depth(&mut self, in_depth: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_position_envelope_bias_depth,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_depth, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_position_envelope_bias_depth,
                __buffer,
            )
        };
    }
    pub fn set_position_envelope_attack_time(&mut self, in_attack_time_msec: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_position_envelope_attack_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_attack_time_msec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_position_envelope_attack_time,
                __buffer,
            )
        };
    }
    pub fn set_low_pass_filter_resonance(&mut self, in_new_q: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_low_pass_filter_resonance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_new_q, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_low_pass_filter_resonance,
                __buffer,
            )
        };
    }
    pub fn set_frequency_with_midi_note(&mut self, in_midi_note: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_frequency_with_midi_note,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_midi_note,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_frequency_with_midi_note,
                __buffer,
            )
        };
    }
    pub fn set_frequency_pitch_bend(&mut self, frequency_offset_cents: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_frequency_pitch_bend,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frequency_offset_cents,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_frequency_pitch_bend,
                __buffer,
            )
        };
    }
    pub fn set_frequency(&mut self, frequency_hz: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_frequency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frequency_hz,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_frequency,
                __buffer,
            )
        };
    }
    pub fn set_filter_envelope_sustain_gain(&mut self, in_sustain_gain: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_filter_envelope_sustain_gain,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sustain_gain,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_filter_envelope_sustain_gain,
                __buffer,
            )
        };
    }
    pub fn set_filter_envelope_release_time(&mut self, in_release_time_msec: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_filter_envelope_release_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_release_time_msec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_filter_envelope_release_time,
                __buffer,
            )
        };
    }
    pub fn set_filter_envelopen_decay_time(&mut self, in_decay_time_msec: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_filter_envelopen_decay_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_decay_time_msec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_filter_envelopen_decay_time,
                __buffer,
            )
        };
    }
    pub fn set_filter_envelope_invert(&mut self, b_in_invert: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_filter_envelope_invert,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_invert,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_filter_envelope_invert,
                __buffer,
            )
        };
    }
    pub fn set_filter_envelope_depth(&mut self, in_depth: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_filter_envelope_depth,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_depth, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_filter_envelope_depth,
                __buffer,
            )
        };
    }
    pub fn set_filter_envelope_bias_invert(&mut self, b_in_bias_invert: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_filter_envelope_bias_invert,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_bias_invert,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_filter_envelope_bias_invert,
                __buffer,
            )
        };
    }
    pub fn set_filter_envelope_bias_depth(&mut self, in_depth: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_filter_envelope_bias_depth,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_depth, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_filter_envelope_bias_depth,
                __buffer,
            )
        };
    }
    pub fn set_filter_envelope_attack_time(&mut self, in_attack_time_msec: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_filter_envelope_attack_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_attack_time_msec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_filter_envelope_attack_time,
                __buffer,
            )
        };
    }
    pub fn set_curve_value(
        &mut self,
        table_index: i32,
        keyframe_index: i32,
        new_value: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_curve_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &table_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &keyframe_index,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&new_value, __buffer.add(8).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_curve_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn set_curve_tangent(&mut self, table_index: i32, in_new_tangent: f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_curve_tangent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &table_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_tangent,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_curve_tangent,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_curve_interpolation_type(
        &mut self,
        interpolation_type: CurveInterpolationType,
        table_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_curve_interpolation_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interpolation_type,
                __buffer.add(0).cast::<CurveInterpolationType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &table_index,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_curve_interpolation_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_amp_envelope_sustain_gain(&mut self, in_sustain_gain: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_amp_envelope_sustain_gain,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sustain_gain,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_amp_envelope_sustain_gain,
                __buffer,
            )
        };
    }
    pub fn set_amp_envelope_release_time(&mut self, in_release_time_msec: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_amp_envelope_release_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_release_time_msec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_amp_envelope_release_time,
                __buffer,
            )
        };
    }
    pub fn set_amp_envelope_invert(&mut self, b_in_invert: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_amp_envelope_invert,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_invert,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_amp_envelope_invert,
                __buffer,
            )
        };
    }
    pub fn set_amp_envelope_depth(&mut self, in_depth: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_amp_envelope_depth,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_depth, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_amp_envelope_depth,
                __buffer,
            )
        };
    }
    pub fn set_amp_envelope_decay_time(&mut self, in_decay_time_msec: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_amp_envelope_decay_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_decay_time_msec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_amp_envelope_decay_time,
                __buffer,
            )
        };
    }
    pub fn set_amp_envelope_bias_invert(&mut self, b_in_bias_invert: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_amp_envelope_bias_invert,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_bias_invert,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_amp_envelope_bias_invert,
                __buffer,
            )
        };
    }
    pub fn set_amp_envelope_bias_depth(&mut self, in_depth: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_amp_envelope_bias_depth,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_depth, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_amp_envelope_bias_depth,
                __buffer,
            )
        };
    }
    pub fn set_amp_envelope_attack_time(&mut self, in_attack_time_msec: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_amp_envelope_attack_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_attack_time_msec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_set_amp_envelope_attack_time,
                __buffer,
            )
        };
    }
    pub fn refresh_wave_table(&mut self, index: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_refresh_wave_table,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_refresh_wave_table,
                __buffer,
            )
        };
    }
    pub fn refresh_all_wave_tables(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_refresh_all_wave_tables,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_refresh_all_wave_tables,
                __buffer,
            )
        };
    }
    pub fn note_on(&mut self, in_midi_note: f32, in_velocity: f32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_note_on,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_midi_note,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_velocity,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_note_on,
                __buffer,
            )
        };
    }
    pub fn note_off(&mut self, in_midi_note: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_note_off,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_midi_note,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_note_off,
                __buffer,
            )
        };
    }
    pub fn get_num_table_entries(&mut self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_get_num_table_entries,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_get_num_table_entries,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_max_table_index(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_get_max_table_index,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_get_max_table_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_key_frame_values_for_table(&self, table_index: f32) -> TArray<f32> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_get_key_frame_values_for_table,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &table_index,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_get_key_frame_values_for_table,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<f32>>().read() }
    }
    pub fn get_curve_tangent(&mut self, table_index: i32) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_get_curve_tangent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &table_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_mono_wave_table_get_curve_tangent,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
}
#[repr(C, align(16))]
pub struct USynthComponentToneGenerator {
    #[doc(hidden)]
    pub(crate) __padding_2384: [u8; 2384],
    pub frequency: f32,
    pub volume: f32,
    pub distance_attenuation_curve: crate::bindings::engine::FRuntimeFloatCurve,
    pub distance_range: crate::bindings::core_u_object::FVector2D,
    pub attenuation_db_at_max_range: f32,
    __padding_end: [u8; 76],
}
impl USynthComponentToneGenerator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USynthComponentToneGenerator")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_volume(&mut self, in_volume: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_tone_generator_set_volume,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_volume, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_tone_generator_set_volume,
                __buffer,
            )
        };
    }
    pub fn set_frequency(&mut self, in_frequency: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_tone_generator_set_frequency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_frequency,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_component_tone_generator_set_frequency,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct USynthSamplePlayer {
    #[doc(hidden)]
    pub(crate) __padding_2384: [u8; 2384],
    pub sound_wave: UPtr<crate::bindings::engine::USoundWave>,
    __padding_end: [u8; 312],
}
impl USynthSamplePlayer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USynthSamplePlayer")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_sound_wave(
        &mut self,
        in_sound_wave: UPtr<crate::bindings::engine::USoundWave>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_sample_player_set_sound_wave,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sound_wave,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USoundWave>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_sample_player_set_sound_wave,
                __buffer,
            )
        };
    }
    pub fn set_scrub_time_width(&mut self, in_scrub_time_width_sec: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_sample_player_set_scrub_time_width,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_scrub_time_width_sec,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_sample_player_set_scrub_time_width,
                __buffer,
            )
        };
    }
    pub fn set_scrub_mode(&mut self, b_scrub_mode: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_sample_player_set_scrub_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_scrub_mode,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_sample_player_set_scrub_mode,
                __buffer,
            )
        };
    }
    pub fn set_pitch(&mut self, in_pitch: f32, time_sec: f32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_sample_player_set_pitch,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_pitch, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time_sec, __buffer.add(4).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_sample_player_set_pitch,
                __buffer,
            )
        };
    }
    pub fn seek_to_time(
        &mut self,
        time_sec: f32,
        seek_type: ESamplePlayerSeekType,
        b_wrap: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<6>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_sample_player_seek_to_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&time_sec, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &seek_type,
                __buffer.add(4).cast::<ESamplePlayerSeekType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_wrap, __buffer.add(5).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_sample_player_seek_to_time,
                __buffer,
            )
        };
    }
    pub fn is_loaded(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_sample_player_is_loaded,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_sample_player_is_loaded,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_sample_duration(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_sample_player_get_sample_duration,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_sample_player_get_sample_duration,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_current_playback_progress_time(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_sample_player_get_current_playback_progress_time,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_sample_player_get_current_playback_progress_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_current_playback_progress_percent(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_sample_player_get_current_playback_progress_percent,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth_sample_player_get_current_playback_progress_percent,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct USynthesisUtilitiesBlueprintFunctionLibrary {
    __padding_end: [u8; 48],
}
impl USynthesisUtilitiesBlueprintFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USynthesisUtilitiesBlueprintFunctionLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_log_frequency(
        in_linear_value: f32,
        in_domain_min: f32,
        in_domain_max: f32,
        in_range_min: f32,
        in_range_max: f32,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synthesis_utilities_blueprint_function_library_get_log_frequency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_linear_value,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_domain_min,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_domain_max,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_range_min,
                __buffer.add(12).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_range_max,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::synthesis::USynthesisUtilitiesBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synthesis_utilities_blueprint_function_library_get_log_frequency,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<f32>().read() }
    }
    pub fn get_linear_frequency(
        in_log_frequency_value: f32,
        in_domain_min: f32,
        in_domain_max: f32,
        in_range_min: f32,
        in_range_max: f32,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synthesis_utilities_blueprint_function_library_get_linear_frequency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_log_frequency_value,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_domain_min,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_domain_max,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_range_min,
                __buffer.add(12).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_range_max,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::synthesis::USynthesisUtilitiesBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synthesis_utilities_blueprint_function_library_get_linear_frequency,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<f32>().read() }
    }
}
#[repr(C, align(16))]
pub struct USynth2DSlider {
    #[doc(hidden)]
    pub(crate) __padding_768: [u8; 768],
    pub widget_style: FSynth2DSliderStyle,
    pub slider_handle_color: crate::bindings::core_u_object::FLinearColor,
    pub indent_handle: bool,
    pub locked: bool,
    pub step_size: f32,
    pub is_focusable: bool,
    __padding_end: [u8; 183],
}
impl USynth2DSlider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USynth2DSlider")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_value(&mut self, in_value: crate::bindings::core_u_object::FVector2D) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS.u_synth2_d_slider_set_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS.u_synth2_d_slider_set_value,
                __buffer,
            )
        };
    }
    pub fn set_step_size(&mut self, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth2_d_slider_set_step_size,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth2_d_slider_set_step_size,
                __buffer,
            )
        };
    }
    pub fn set_slider_handle_color(
        &mut self,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth2_d_slider_set_slider_handle_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth2_d_slider_set_slider_handle_color,
                __buffer,
            )
        };
    }
    pub fn set_locked(&mut self, in_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS.u_synth2_d_slider_set_locked,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS.u_synth2_d_slider_set_locked,
                __buffer,
            )
        };
    }
    pub fn set_indent_handle(&mut self, in_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth2_d_slider_set_indent_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS
                    .u_synth2_d_slider_set_indent_handle,
                __buffer,
            )
        };
    }
    pub fn get_value(&self) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS.u_synth2_d_slider_get_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS.u_synth2_d_slider_get_value,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
}
#[repr(C, align(16))]
pub struct USynthKnob {
    #[doc(hidden)]
    pub(crate) __padding_700: [u8; 700],
    pub step_size: f32,
    pub mouse_speed: f32,
    pub mouse_fine_tune_speed: f32,
    pub flags_712: u8,
    pub parameter_name: FText,
    pub parameter_units: FText,
    #[doc(hidden)]
    pub(crate) __padding_784: [u8; 32],
    pub widget_style: FSynthKnobStyle,
    pub locked: bool,
    pub is_focusable: bool,
    __padding_end: [u8; 158],
}
impl USynthKnob {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USynthKnob")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_value(&mut self, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS.u_synth_knob_set_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS.u_synth_knob_set_value,
                __buffer,
            )
        };
    }
    pub fn set_step_size(&mut self, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS.u_synth_knob_set_step_size,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS.u_synth_knob_set_step_size,
                __buffer,
            )
        };
    }
    pub fn set_locked(&mut self, in_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS.u_synth_knob_set_locked,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS.u_synth_knob_set_locked,
                __buffer,
            )
        };
    }
    pub fn get_value(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::synthesis::__FUNCTION_PTRS.u_synth_knob_get_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::synthesis::__FUNCTION_PTRS.u_synth_knob_get_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
}
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
