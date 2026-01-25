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
    pub u_audio_mixer_blueprint_library_unregister_audio_bus_from_submix: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_trim_audio_cache: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_swap_audio_output_device: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_stop_recording_output: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_stop_audio_bus: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_stop_analyzing_output: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_start_recording_output: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_start_audio_bus: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_start_analyzing_output: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_set_submix_effect_chain_override: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_set_bypass_source_effect_chain_entry: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_resume_recording_output: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_replace_submix_effect: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_replace_sound_effect_submix: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_remove_submix_effect_preset_at_index: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_remove_submix_effect_preset: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_remove_submix_effect_at_index: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_remove_submix_effect: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_remove_source_effect_from_preset_chain: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_remove_master_submix_effect: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_register_audio_bus_to_submix: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_prime_sound_for_playback: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_prime_sound_cue_for_playback: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_pause_recording_output: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_make_preset_spectral_analysis_band_settings: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_make_musical_spectral_analysis_band_settings: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_make_full_spectrum_spectral_analysis_band_settings: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_is_audio_bus_active: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_get_phase_for_frequencies: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_get_number_of_entries_in_source_effect_chain: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_get_magnitude_for_frequencies: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_get_current_audio_output_device_name: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_get_available_audio_output_devices: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_conv_audio_output_device_info_to_string: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_clear_submix_effects: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_clear_submix_effect_chain_override: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_clear_master_submix_effects: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_add_submix_effect: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_add_source_effect_to_preset_chain: *mut crate::ffi::UFunctionOpague,
    pub u_audio_mixer_blueprint_library_add_master_submix_effect: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_stop: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_start: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_set_volume_multiplier: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_set_submix_send: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_set_source_bus_send_pre_effect: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_set_source_bus_send_post_effect: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_set_output_to_bus_only: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_set_modulation_routing: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_set_low_pass_filter_frequency: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_set_low_pass_filter_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_set_audio_bus_send_pre_effect: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_set_audio_bus_send_post_effect: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_is_playing: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_get_modulators: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_fade_out: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_fade_in: *mut crate::ffi::UFunctionOpague,
    pub u_synth_component_adjust_volume: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_dynamics_processor_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_dynamics_processor_preset_set_external_submix: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_dynamics_processor_preset_set_audio_bus: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_dynamics_processor_preset_reset_key: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_submix_eq_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_reverb_preset_set_settings_with_reverb_effect: *mut crate::ffi::UFunctionOpague,
    pub u_submix_effect_reverb_preset_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_scrubbed_sound_set_sound_wave: *mut crate::ffi::UFunctionOpague,
    pub u_scrubbed_sound_set_playhead_time: *mut crate::ffi::UFunctionOpague,
    pub u_scrubbed_sound_set_is_scrubbing_while_stationary: *mut crate::ffi::UFunctionOpague,
    pub u_scrubbed_sound_set_is_scrubbing: *mut crate::ffi::UFunctionOpague,
    pub u_scrubbed_sound_set_grain_duration_range: *mut crate::ffi::UFunctionOpague,
    pub u_scrubbed_sound_get_playhead_time: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_unsubscribe_from_time_division: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_unsubscribe_from_all_time_divisions: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_subscribe_to_quantization_event: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_subscribe_to_all_quantization_events: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_stop_clock: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_start_other_clock: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_start_clock: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_set_ticks_per_second: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_set_thirty_second_notes_per_minute: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_set_seconds_per_tick: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_set_milliseconds_per_tick: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_set_beats_per_minute: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_resume_clock: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_reset_transport_quantized: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_reset_transport: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_pause_clock: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_notify_on_quantization_boundary: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_is_clock_running: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_get_ticks_per_second: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_get_thirty_second_notes_per_minute: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_get_seconds_per_tick: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_get_milliseconds_per_tick: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_get_estimated_run_time: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_get_duration_of_quantization_type_in_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_get_current_timestamp: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_get_beats_per_minute: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_clock_handle_get_beat_progress_percent: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_subsystem_set_quartz_subsystem_tickable_when_paused: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_subsystem_is_quartz_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_subsystem_is_clock_running: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_subsystem_get_round_trip_min_latency: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_subsystem_get_round_trip_max_latency: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_subsystem_get_round_trip_average_latency: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_subsystem_get_handle_for_clock: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_subsystem_get_game_thread_to_audio_render_thread_min_latency: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_subsystem_get_game_thread_to_audio_render_thread_max_latency: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_subsystem_get_game_thread_to_audio_render_thread_average_latency: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_subsystem_get_estimated_clock_run_time: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_subsystem_get_duration_of_quantization_type_in_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_subsystem_get_current_clock_timestamp: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_subsystem_get_audio_render_thread_to_game_thread_min_latency: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_subsystem_get_audio_render_thread_to_game_thread_max_latency: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_subsystem_get_audio_render_thread_to_game_thread_average_latency: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_subsystem_does_clock_exist: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_subsystem_delete_clock_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_subsystem_delete_clock_by_handle: *mut crate::ffi::UFunctionOpague,
    pub u_quartz_subsystem_create_new_clock: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_audio_mixer_blueprint_library_unregister_audio_bus_from_submix: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_trim_audio_cache: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_swap_audio_output_device: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_stop_recording_output: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_stop_audio_bus: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_stop_analyzing_output: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_start_recording_output: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_start_audio_bus: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_start_analyzing_output: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_set_submix_effect_chain_override: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_set_bypass_source_effect_chain_entry: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_resume_recording_output: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_replace_submix_effect: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_replace_sound_effect_submix: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_remove_submix_effect_preset_at_index: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_remove_submix_effect_preset: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_remove_submix_effect_at_index: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_remove_submix_effect: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_remove_source_effect_from_preset_chain: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_remove_master_submix_effect: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_register_audio_bus_to_submix: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_prime_sound_for_playback: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_prime_sound_cue_for_playback: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_pause_recording_output: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_make_preset_spectral_analysis_band_settings: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_make_musical_spectral_analysis_band_settings: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_make_full_spectrum_spectral_analysis_band_settings: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_is_audio_bus_active: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_get_phase_for_frequencies: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_get_number_of_entries_in_source_effect_chain: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_get_magnitude_for_frequencies: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_get_current_audio_output_device_name: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_get_available_audio_output_devices: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_conv_audio_output_device_info_to_string: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_clear_submix_effects: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_clear_submix_effect_chain_override: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_clear_master_submix_effects: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_add_submix_effect: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_add_source_effect_to_preset_chain: std::ptr::null_mut(),
            u_audio_mixer_blueprint_library_add_master_submix_effect: std::ptr::null_mut(),
            u_synth_component_stop: std::ptr::null_mut(),
            u_synth_component_start: std::ptr::null_mut(),
            u_synth_component_set_volume_multiplier: std::ptr::null_mut(),
            u_synth_component_set_submix_send: std::ptr::null_mut(),
            u_synth_component_set_source_bus_send_pre_effect: std::ptr::null_mut(),
            u_synth_component_set_source_bus_send_post_effect: std::ptr::null_mut(),
            u_synth_component_set_output_to_bus_only: std::ptr::null_mut(),
            u_synth_component_set_modulation_routing: std::ptr::null_mut(),
            u_synth_component_set_low_pass_filter_frequency: std::ptr::null_mut(),
            u_synth_component_set_low_pass_filter_enabled: std::ptr::null_mut(),
            u_synth_component_set_audio_bus_send_pre_effect: std::ptr::null_mut(),
            u_synth_component_set_audio_bus_send_post_effect: std::ptr::null_mut(),
            u_synth_component_is_playing: std::ptr::null_mut(),
            u_synth_component_get_modulators: std::ptr::null_mut(),
            u_synth_component_fade_out: std::ptr::null_mut(),
            u_synth_component_fade_in: std::ptr::null_mut(),
            u_synth_component_adjust_volume: std::ptr::null_mut(),
            u_submix_effect_dynamics_processor_preset_set_settings: std::ptr::null_mut(),
            u_submix_effect_dynamics_processor_preset_set_external_submix: std::ptr::null_mut(),
            u_submix_effect_dynamics_processor_preset_set_audio_bus: std::ptr::null_mut(),
            u_submix_effect_dynamics_processor_preset_reset_key: std::ptr::null_mut(),
            u_submix_effect_submix_eq_preset_set_settings: std::ptr::null_mut(),
            u_submix_effect_reverb_preset_set_settings_with_reverb_effect: std::ptr::null_mut(),
            u_submix_effect_reverb_preset_set_settings: std::ptr::null_mut(),
            u_scrubbed_sound_set_sound_wave: std::ptr::null_mut(),
            u_scrubbed_sound_set_playhead_time: std::ptr::null_mut(),
            u_scrubbed_sound_set_is_scrubbing_while_stationary: std::ptr::null_mut(),
            u_scrubbed_sound_set_is_scrubbing: std::ptr::null_mut(),
            u_scrubbed_sound_set_grain_duration_range: std::ptr::null_mut(),
            u_scrubbed_sound_get_playhead_time: std::ptr::null_mut(),
            u_quartz_clock_handle_unsubscribe_from_time_division: std::ptr::null_mut(),
            u_quartz_clock_handle_unsubscribe_from_all_time_divisions: std::ptr::null_mut(),
            u_quartz_clock_handle_subscribe_to_quantization_event: std::ptr::null_mut(),
            u_quartz_clock_handle_subscribe_to_all_quantization_events: std::ptr::null_mut(),
            u_quartz_clock_handle_stop_clock: std::ptr::null_mut(),
            u_quartz_clock_handle_start_other_clock: std::ptr::null_mut(),
            u_quartz_clock_handle_start_clock: std::ptr::null_mut(),
            u_quartz_clock_handle_set_ticks_per_second: std::ptr::null_mut(),
            u_quartz_clock_handle_set_thirty_second_notes_per_minute: std::ptr::null_mut(),
            u_quartz_clock_handle_set_seconds_per_tick: std::ptr::null_mut(),
            u_quartz_clock_handle_set_milliseconds_per_tick: std::ptr::null_mut(),
            u_quartz_clock_handle_set_beats_per_minute: std::ptr::null_mut(),
            u_quartz_clock_handle_resume_clock: std::ptr::null_mut(),
            u_quartz_clock_handle_reset_transport_quantized: std::ptr::null_mut(),
            u_quartz_clock_handle_reset_transport: std::ptr::null_mut(),
            u_quartz_clock_handle_pause_clock: std::ptr::null_mut(),
            u_quartz_clock_handle_notify_on_quantization_boundary: std::ptr::null_mut(),
            u_quartz_clock_handle_is_clock_running: std::ptr::null_mut(),
            u_quartz_clock_handle_get_ticks_per_second: std::ptr::null_mut(),
            u_quartz_clock_handle_get_thirty_second_notes_per_minute: std::ptr::null_mut(),
            u_quartz_clock_handle_get_seconds_per_tick: std::ptr::null_mut(),
            u_quartz_clock_handle_get_milliseconds_per_tick: std::ptr::null_mut(),
            u_quartz_clock_handle_get_estimated_run_time: std::ptr::null_mut(),
            u_quartz_clock_handle_get_duration_of_quantization_type_in_seconds: std::ptr::null_mut(),
            u_quartz_clock_handle_get_current_timestamp: std::ptr::null_mut(),
            u_quartz_clock_handle_get_beats_per_minute: std::ptr::null_mut(),
            u_quartz_clock_handle_get_beat_progress_percent: std::ptr::null_mut(),
            u_quartz_subsystem_set_quartz_subsystem_tickable_when_paused: std::ptr::null_mut(),
            u_quartz_subsystem_is_quartz_enabled: std::ptr::null_mut(),
            u_quartz_subsystem_is_clock_running: std::ptr::null_mut(),
            u_quartz_subsystem_get_round_trip_min_latency: std::ptr::null_mut(),
            u_quartz_subsystem_get_round_trip_max_latency: std::ptr::null_mut(),
            u_quartz_subsystem_get_round_trip_average_latency: std::ptr::null_mut(),
            u_quartz_subsystem_get_handle_for_clock: std::ptr::null_mut(),
            u_quartz_subsystem_get_game_thread_to_audio_render_thread_min_latency: std::ptr::null_mut(),
            u_quartz_subsystem_get_game_thread_to_audio_render_thread_max_latency: std::ptr::null_mut(),
            u_quartz_subsystem_get_game_thread_to_audio_render_thread_average_latency: std::ptr::null_mut(),
            u_quartz_subsystem_get_estimated_clock_run_time: std::ptr::null_mut(),
            u_quartz_subsystem_get_duration_of_quantization_type_in_seconds: std::ptr::null_mut(),
            u_quartz_subsystem_get_current_clock_timestamp: std::ptr::null_mut(),
            u_quartz_subsystem_get_audio_render_thread_to_game_thread_min_latency: std::ptr::null_mut(),
            u_quartz_subsystem_get_audio_render_thread_to_game_thread_max_latency: std::ptr::null_mut(),
            u_quartz_subsystem_get_audio_render_thread_to_game_thread_average_latency: std::ptr::null_mut(),
            u_quartz_subsystem_does_clock_exist: std::ptr::null_mut(),
            u_quartz_subsystem_delete_clock_by_name: std::ptr::null_mut(),
            u_quartz_subsystem_delete_clock_by_handle: std::ptr::null_mut(),
            u_quartz_subsystem_create_new_clock: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAudioMixerBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnregisterAudioBusFromSubmix"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_unregister_audio_bus_from_submix,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TrimAudioCache"),
            &raw mut __FUNCTION_PTRS.u_audio_mixer_blueprint_library_trim_audio_cache,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SwapAudioOutputDevice"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_swap_audio_output_device,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopRecordingOutput"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_stop_recording_output,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopAudioBus"),
            &raw mut __FUNCTION_PTRS.u_audio_mixer_blueprint_library_stop_audio_bus,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopAnalyzingOutput"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_stop_analyzing_output,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartRecordingOutput"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_start_recording_output,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartAudioBus"),
            &raw mut __FUNCTION_PTRS.u_audio_mixer_blueprint_library_start_audio_bus,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartAnalyzingOutput"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_start_analyzing_output,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSubmixEffectChainOverride"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_set_submix_effect_chain_override,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBypassSourceEffectChainEntry"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_set_bypass_source_effect_chain_entry,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResumeRecordingOutput"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_resume_recording_output,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReplaceSubmixEffect"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_replace_submix_effect,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReplaceSoundEffectSubmix"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_replace_sound_effect_submix,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveSubmixEffectPresetAtIndex"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_remove_submix_effect_preset_at_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveSubmixEffectPreset"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_remove_submix_effect_preset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveSubmixEffectAtIndex"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_remove_submix_effect_at_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveSubmixEffect"),
            &raw mut __FUNCTION_PTRS.u_audio_mixer_blueprint_library_remove_submix_effect,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveSourceEffectFromPresetChain"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_remove_source_effect_from_preset_chain,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveMasterSubmixEffect"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_remove_master_submix_effect,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterAudioBusToSubmix"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_register_audio_bus_to_submix,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PrimeSoundForPlayback"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_prime_sound_for_playback,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PrimeSoundCueForPlayback"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_prime_sound_cue_for_playback,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PauseRecordingOutput"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_pause_recording_output,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakePresetSpectralAnalysisBandSettings"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_make_preset_spectral_analysis_band_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeMusicalSpectralAnalysisBandSettings"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_make_musical_spectral_analysis_band_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeFullSpectrumSpectralAnalysisBandSettings"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_make_full_spectrum_spectral_analysis_band_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAudioBusActive"),
            &raw mut __FUNCTION_PTRS.u_audio_mixer_blueprint_library_is_audio_bus_active,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPhaseForFrequencies"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_get_phase_for_frequencies,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumberOfEntriesInSourceEffectChain"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_get_number_of_entries_in_source_effect_chain,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMagnitudeForFrequencies"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_get_magnitude_for_frequencies,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentAudioOutputDeviceName"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_get_current_audio_output_device_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAvailableAudioOutputDevices"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_get_available_audio_output_devices,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_AudioOutputDeviceInfoToString"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_conv_audio_output_device_info_to_string,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearSubmixEffects"),
            &raw mut __FUNCTION_PTRS.u_audio_mixer_blueprint_library_clear_submix_effects,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearSubmixEffectChainOverride"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_clear_submix_effect_chain_override,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearMasterSubmixEffects"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_clear_master_submix_effects,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSubmixEffect"),
            &raw mut __FUNCTION_PTRS.u_audio_mixer_blueprint_library_add_submix_effect,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSourceEffectToPresetChain"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_add_source_effect_to_preset_chain,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddMasterSubmixEffect"),
            &raw mut __FUNCTION_PTRS
                .u_audio_mixer_blueprint_library_add_master_submix_effect,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USynthComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Stop"),
            &raw mut __FUNCTION_PTRS.u_synth_component_stop,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Start"),
            &raw mut __FUNCTION_PTRS.u_synth_component_start,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVolumeMultiplier"),
            &raw mut __FUNCTION_PTRS.u_synth_component_set_volume_multiplier,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSubmixSend"),
            &raw mut __FUNCTION_PTRS.u_synth_component_set_submix_send,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSourceBusSendPreEffect"),
            &raw mut __FUNCTION_PTRS.u_synth_component_set_source_bus_send_pre_effect,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSourceBusSendPostEffect"),
            &raw mut __FUNCTION_PTRS.u_synth_component_set_source_bus_send_post_effect,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOutputToBusOnly"),
            &raw mut __FUNCTION_PTRS.u_synth_component_set_output_to_bus_only,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetModulationRouting"),
            &raw mut __FUNCTION_PTRS.u_synth_component_set_modulation_routing,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLowPassFilterFrequency"),
            &raw mut __FUNCTION_PTRS.u_synth_component_set_low_pass_filter_frequency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLowPassFilterEnabled"),
            &raw mut __FUNCTION_PTRS.u_synth_component_set_low_pass_filter_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAudioBusSendPreEffect"),
            &raw mut __FUNCTION_PTRS.u_synth_component_set_audio_bus_send_pre_effect,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAudioBusSendPostEffect"),
            &raw mut __FUNCTION_PTRS.u_synth_component_set_audio_bus_send_post_effect,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPlaying"),
            &raw mut __FUNCTION_PTRS.u_synth_component_is_playing,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetModulators"),
            &raw mut __FUNCTION_PTRS.u_synth_component_get_modulators,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FadeOut"),
            &raw mut __FUNCTION_PTRS.u_synth_component_fade_out,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FadeIn"),
            &raw mut __FUNCTION_PTRS.u_synth_component_fade_in,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AdjustVolume"),
            &raw mut __FUNCTION_PTRS.u_synth_component_adjust_volume,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USubmixEffectDynamicsProcessorPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS
                .u_submix_effect_dynamics_processor_preset_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetExternalSubmix"),
            &raw mut __FUNCTION_PTRS
                .u_submix_effect_dynamics_processor_preset_set_external_submix,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAudioBus"),
            &raw mut __FUNCTION_PTRS
                .u_submix_effect_dynamics_processor_preset_set_audio_bus,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetKey"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_dynamics_processor_preset_reset_key,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USubmixEffectSubmixEQPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_submix_eq_preset_set_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USubmixEffectReverbPreset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettingsWithReverbEffect"),
            &raw mut __FUNCTION_PTRS
                .u_submix_effect_reverb_preset_set_settings_with_reverb_effect,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.u_submix_effect_reverb_preset_set_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UScrubbedSound::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSoundWave"),
            &raw mut __FUNCTION_PTRS.u_scrubbed_sound_set_sound_wave,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlayheadTime"),
            &raw mut __FUNCTION_PTRS.u_scrubbed_sound_set_playhead_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsScrubbingWhileStationary"),
            &raw mut __FUNCTION_PTRS.u_scrubbed_sound_set_is_scrubbing_while_stationary,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsScrubbing"),
            &raw mut __FUNCTION_PTRS.u_scrubbed_sound_set_is_scrubbing,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGrainDurationRange"),
            &raw mut __FUNCTION_PTRS.u_scrubbed_sound_set_grain_duration_range,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlayheadTime"),
            &raw mut __FUNCTION_PTRS.u_scrubbed_sound_get_playhead_time,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UQuartzClockHandle::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnsubscribeFromTimeDivision"),
            &raw mut __FUNCTION_PTRS.u_quartz_clock_handle_unsubscribe_from_time_division,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnsubscribeFromAllTimeDivisions"),
            &raw mut __FUNCTION_PTRS
                .u_quartz_clock_handle_unsubscribe_from_all_time_divisions,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SubscribeToQuantizationEvent"),
            &raw mut __FUNCTION_PTRS
                .u_quartz_clock_handle_subscribe_to_quantization_event,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SubscribeToAllQuantizationEvents"),
            &raw mut __FUNCTION_PTRS
                .u_quartz_clock_handle_subscribe_to_all_quantization_events,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopClock"),
            &raw mut __FUNCTION_PTRS.u_quartz_clock_handle_stop_clock,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartOtherClock"),
            &raw mut __FUNCTION_PTRS.u_quartz_clock_handle_start_other_clock,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartClock"),
            &raw mut __FUNCTION_PTRS.u_quartz_clock_handle_start_clock,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTicksPerSecond"),
            &raw mut __FUNCTION_PTRS.u_quartz_clock_handle_set_ticks_per_second,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetThirtySecondNotesPerMinute"),
            &raw mut __FUNCTION_PTRS
                .u_quartz_clock_handle_set_thirty_second_notes_per_minute,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSecondsPerTick"),
            &raw mut __FUNCTION_PTRS.u_quartz_clock_handle_set_seconds_per_tick,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMillisecondsPerTick"),
            &raw mut __FUNCTION_PTRS.u_quartz_clock_handle_set_milliseconds_per_tick,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBeatsPerMinute"),
            &raw mut __FUNCTION_PTRS.u_quartz_clock_handle_set_beats_per_minute,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResumeClock"),
            &raw mut __FUNCTION_PTRS.u_quartz_clock_handle_resume_clock,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetTransportQuantized"),
            &raw mut __FUNCTION_PTRS.u_quartz_clock_handle_reset_transport_quantized,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetTransport"),
            &raw mut __FUNCTION_PTRS.u_quartz_clock_handle_reset_transport,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PauseClock"),
            &raw mut __FUNCTION_PTRS.u_quartz_clock_handle_pause_clock,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NotifyOnQuantizationBoundary"),
            &raw mut __FUNCTION_PTRS
                .u_quartz_clock_handle_notify_on_quantization_boundary,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsClockRunning"),
            &raw mut __FUNCTION_PTRS.u_quartz_clock_handle_is_clock_running,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTicksPerSecond"),
            &raw mut __FUNCTION_PTRS.u_quartz_clock_handle_get_ticks_per_second,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetThirtySecondNotesPerMinute"),
            &raw mut __FUNCTION_PTRS
                .u_quartz_clock_handle_get_thirty_second_notes_per_minute,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSecondsPerTick"),
            &raw mut __FUNCTION_PTRS.u_quartz_clock_handle_get_seconds_per_tick,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMillisecondsPerTick"),
            &raw mut __FUNCTION_PTRS.u_quartz_clock_handle_get_milliseconds_per_tick,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEstimatedRunTime"),
            &raw mut __FUNCTION_PTRS.u_quartz_clock_handle_get_estimated_run_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDurationOfQuantizationTypeInSeconds"),
            &raw mut __FUNCTION_PTRS
                .u_quartz_clock_handle_get_duration_of_quantization_type_in_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentTimestamp"),
            &raw mut __FUNCTION_PTRS.u_quartz_clock_handle_get_current_timestamp,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBeatsPerMinute"),
            &raw mut __FUNCTION_PTRS.u_quartz_clock_handle_get_beats_per_minute,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBeatProgressPercent"),
            &raw mut __FUNCTION_PTRS.u_quartz_clock_handle_get_beat_progress_percent,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UQuartzSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetQuartzSubsystemTickableWhenPaused"),
            &raw mut __FUNCTION_PTRS
                .u_quartz_subsystem_set_quartz_subsystem_tickable_when_paused,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsQuartzEnabled"),
            &raw mut __FUNCTION_PTRS.u_quartz_subsystem_is_quartz_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsClockRunning"),
            &raw mut __FUNCTION_PTRS.u_quartz_subsystem_is_clock_running,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRoundTripMinLatency"),
            &raw mut __FUNCTION_PTRS.u_quartz_subsystem_get_round_trip_min_latency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRoundTripMaxLatency"),
            &raw mut __FUNCTION_PTRS.u_quartz_subsystem_get_round_trip_max_latency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRoundTripAverageLatency"),
            &raw mut __FUNCTION_PTRS.u_quartz_subsystem_get_round_trip_average_latency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHandleForClock"),
            &raw mut __FUNCTION_PTRS.u_quartz_subsystem_get_handle_for_clock,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGameThreadToAudioRenderThreadMinLatency"),
            &raw mut __FUNCTION_PTRS
                .u_quartz_subsystem_get_game_thread_to_audio_render_thread_min_latency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGameThreadToAudioRenderThreadMaxLatency"),
            &raw mut __FUNCTION_PTRS
                .u_quartz_subsystem_get_game_thread_to_audio_render_thread_max_latency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGameThreadToAudioRenderThreadAverageLatency"),
            &raw mut __FUNCTION_PTRS
                .u_quartz_subsystem_get_game_thread_to_audio_render_thread_average_latency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEstimatedClockRunTime"),
            &raw mut __FUNCTION_PTRS.u_quartz_subsystem_get_estimated_clock_run_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDurationOfQuantizationTypeInSeconds"),
            &raw mut __FUNCTION_PTRS
                .u_quartz_subsystem_get_duration_of_quantization_type_in_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentClockTimestamp"),
            &raw mut __FUNCTION_PTRS.u_quartz_subsystem_get_current_clock_timestamp,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAudioRenderThreadToGameThreadMinLatency"),
            &raw mut __FUNCTION_PTRS
                .u_quartz_subsystem_get_audio_render_thread_to_game_thread_min_latency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAudioRenderThreadToGameThreadMaxLatency"),
            &raw mut __FUNCTION_PTRS
                .u_quartz_subsystem_get_audio_render_thread_to_game_thread_max_latency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAudioRenderThreadToGameThreadAverageLatency"),
            &raw mut __FUNCTION_PTRS
                .u_quartz_subsystem_get_audio_render_thread_to_game_thread_average_latency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DoesClockExist"),
            &raw mut __FUNCTION_PTRS.u_quartz_subsystem_does_clock_exist,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteClockByName"),
            &raw mut __FUNCTION_PTRS.u_quartz_subsystem_delete_clock_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteClockByHandle"),
            &raw mut __FUNCTION_PTRS.u_quartz_subsystem_delete_clock_by_handle,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateNewClock"),
            &raw mut __FUNCTION_PTRS.u_quartz_subsystem_create_new_clock,
        );
    }
}
#[repr(C, align(8))]
pub struct FAudioOutputDeviceInfo {
    pub name: FString,
    pub device_id: FString,
    pub num_channels: i32,
    pub sample_rate: i32,
    pub format: EAudioMixerStreamDataFormatType,
    pub output_channel_array: TArray<EAudioMixerChannelType>,
    pub flags_64: u8,
}
impl FAudioOutputDeviceInfo {}
#[repr(C, align(8))]
pub struct FSwapAudioOutputResult {
    pub current_device_id: FString,
    pub requested_device_id: FString,
    pub result: ESwapAudioOutputDeviceResultState,
}
impl FSwapAudioOutputResult {}
#[repr(C, align(4))]
pub struct FSubmixEffectDynamicProcessorFilterSettings {
    pub flags_0: u8,
    pub cutoff: f32,
    pub gain_db: f32,
}
impl FSubmixEffectDynamicProcessorFilterSettings {}
#[repr(C, align(8))]
pub struct FSubmixEffectDynamicsProcessorSettings {
    pub dynamics_processor_type: ESubmixEffectDynamicsProcessorType,
    pub peak_mode: ESubmixEffectDynamicsPeakMode,
    pub link_mode: ESubmixEffectDynamicsChannelLinkMode,
    pub input_gain_db: f32,
    pub threshold_db: f32,
    pub ratio: f32,
    pub knee_bandwidth_db: f32,
    pub look_ahead_msec: f32,
    pub attack_time_msec: f32,
    pub release_time_msec: f32,
    pub key_source: ESubmixEffectDynamicsKeySource,
    pub external_audio_bus: UPtr<crate::bindings::engine::UAudioBus>,
    pub external_submix: UPtr<crate::bindings::engine::USoundSubmix>,
    pub flags_56: u8,
    pub key_gain_db: f32,
    pub output_gain_db: f32,
    pub key_highshelf: FSubmixEffectDynamicProcessorFilterSettings,
    pub key_lowshelf: FSubmixEffectDynamicProcessorFilterSettings,
}
impl FSubmixEffectDynamicsProcessorSettings {}
#[repr(C, align(4))]
pub struct FSubmixEffectEQBand {
    pub frequency: f32,
    pub bandwidth: f32,
    pub gain_db: f32,
    pub flags_12: u8,
}
impl FSubmixEffectEQBand {}
#[repr(C, align(8))]
pub struct FSubmixEffectSubmixEQSettings {
    pub eq_bands: TArray<FSubmixEffectEQBand>,
}
impl FSubmixEffectSubmixEQSettings {}
#[repr(C, align(4))]
pub struct FSubmixEffectReverbSettings {
    pub b_bypass_early_reflections: bool,
    pub reflections_delay: f32,
    pub gain_hf: f32,
    pub reflections_gain: f32,
    pub b_bypass_late_reflections: bool,
    pub late_delay: f32,
    pub decay_time: f32,
    pub density: f32,
    pub diffusion: f32,
    pub air_absorption_gain_hf: f32,
    pub decay_hf_ratio: f32,
    pub late_gain: f32,
    pub gain: f32,
    pub wet_level: f32,
    pub dry_level: f32,
    pub b_bypass: bool,
}
impl FSubmixEffectReverbSettings {}
#[repr(C, align(8))]
pub struct UAudioBusSubsystem {
    __padding_end: [u8; 272],
}
impl UAudioBusSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioBusSubsystem")
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
pub struct UAudioDeviceNotificationSubsystem {
    __padding_end: [u8; 400],
}
impl UAudioDeviceNotificationSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioDeviceNotificationSubsystem")
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
pub struct UAudioMixerBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UAudioMixerBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioMixerBlueprintLibrary")
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
    pub fn unregister_audio_bus_from_submix(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        sound_submix: UPtr<crate::bindings::engine::USoundSubmix>,
        audio_bus: UPtr<crate::bindings::engine::UAudioBus>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_unregister_audio_bus_from_submix,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sound_submix,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &audio_bus,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UAudioBus>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_unregister_audio_bus_from_submix,
                __buffer,
            )
        };
    }
    pub fn trim_audio_cache(in_megabytes_to_free: f32) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_trim_audio_cache,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_megabytes_to_free,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_trim_audio_cache,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
    pub fn swap_audio_output_device(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        new_device_id: FString,
        on_completed_device_swap: &FSwapAudioOutputDevice_OnCompletedDeviceSwap,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_swap_audio_output_device,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_device_id,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                on_completed_device_swap,
                __buffer.add(24).cast::<FSwapAudioOutputDevice_OnCompletedDeviceSwap>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_swap_audio_output_device,
                __buffer,
            )
        };
    }
    pub fn stop_recording_output(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        export_type: crate::bindings::engine::EAudioRecordingExportType,
        name: FString,
        path: FString,
        submix_to_record: UPtr<crate::bindings::engine::USoundSubmix>,
        existing_sound_wave_to_overwrite: UPtr<crate::bindings::engine::USoundWave>,
    ) -> UPtr<crate::bindings::engine::USoundWave> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_stop_recording_output,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &export_type,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::engine::EAudioRecordingExportType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(16).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&path, __buffer.add(32).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix_to_record,
                __buffer.add(48).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &existing_sound_wave_to_overwrite,
                __buffer.add(56).cast::<UPtr<crate::bindings::engine::USoundWave>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_stop_recording_output,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(64).cast::<UPtr<crate::bindings::engine::USoundWave>>().read()
        }
    }
    pub fn stop_audio_bus(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        audio_bus: UPtr<crate::bindings::engine::UAudioBus>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_stop_audio_bus,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &audio_bus,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UAudioBus>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_stop_audio_bus,
                __buffer,
            )
        };
    }
    pub fn stop_analyzing_output(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        submix_to_stop_analyzing: UPtr<crate::bindings::engine::USoundSubmix>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_stop_analyzing_output,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix_to_stop_analyzing,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_stop_analyzing_output,
                __buffer,
            )
        };
    }
    pub fn start_recording_output(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        expected_duration: f32,
        submix_to_record: UPtr<crate::bindings::engine::USoundSubmix>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_start_recording_output,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expected_duration,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix_to_record,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_start_recording_output,
                __buffer,
            )
        };
    }
    pub fn start_audio_bus(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        audio_bus: UPtr<crate::bindings::engine::UAudioBus>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_start_audio_bus,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &audio_bus,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UAudioBus>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_start_audio_bus,
                __buffer,
            )
        };
    }
    pub fn start_analyzing_output(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        submix_to_analyze: UPtr<crate::bindings::engine::USoundSubmix>,
        fft_size: crate::bindings::engine::EFFTSize,
        interpolation_method: crate::bindings::engine::EFFTPeakInterpolationMethod,
        window_type: crate::bindings::engine::EFFTWindowType,
        hop_size: f32,
        spectrum_type: crate::bindings::engine::EAudioSpectrumType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_start_analyzing_output,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix_to_analyze,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &fft_size,
                __buffer.add(16).cast::<crate::bindings::engine::EFFTSize>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interpolation_method,
                __buffer
                    .add(17)
                    .cast::<crate::bindings::engine::EFFTPeakInterpolationMethod>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &window_type,
                __buffer.add(18).cast::<crate::bindings::engine::EFFTWindowType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&hop_size, __buffer.add(20).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spectrum_type,
                __buffer.add(24).cast::<crate::bindings::engine::EAudioSpectrumType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_start_analyzing_output,
                __buffer,
            )
        };
    }
    pub fn set_submix_effect_chain_override(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        sound_submix: UPtr<crate::bindings::engine::USoundSubmix>,
        submix_effect_preset_chain: TArray<
            UPtr<crate::bindings::engine::USoundEffectSubmixPreset>,
        >,
        fade_time_sec: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_set_submix_effect_chain_override,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sound_submix,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix_effect_preset_chain,
                __buffer
                    .add(16)
                    .cast::<
                        TArray<UPtr<crate::bindings::engine::USoundEffectSubmixPreset>>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &fade_time_sec,
                __buffer.add(32).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_set_submix_effect_chain_override,
                __buffer,
            )
        };
    }
    pub fn set_bypass_source_effect_chain_entry(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        preset_chain: UPtr<crate::bindings::engine::USoundEffectSourcePresetChain>,
        entry_index: i32,
        b_bypassed: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_set_bypass_source_effect_chain_entry,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &preset_chain,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<crate::bindings::engine::USoundEffectSourcePresetChain>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &entry_index,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_bypassed,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_set_bypass_source_effect_chain_entry,
                __buffer,
            )
        };
    }
    pub fn resume_recording_output(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        submix_to_pause: UPtr<crate::bindings::engine::USoundSubmix>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_resume_recording_output,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix_to_pause,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_resume_recording_output,
                __buffer,
            )
        };
    }
    pub fn replace_submix_effect(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_sound_submix: UPtr<crate::bindings::engine::USoundSubmix>,
        submix_chain_index: i32,
        submix_effect_preset: UPtr<crate::bindings::engine::USoundEffectSubmixPreset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_replace_submix_effect,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sound_submix,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix_chain_index,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix_effect_preset,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::engine::USoundEffectSubmixPreset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_replace_submix_effect,
                __buffer,
            )
        };
    }
    pub fn replace_sound_effect_submix(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_sound_submix: UPtr<crate::bindings::engine::USoundSubmix>,
        submix_chain_index: i32,
        submix_effect_preset: UPtr<crate::bindings::engine::USoundEffectSubmixPreset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_replace_sound_effect_submix,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sound_submix,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix_chain_index,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix_effect_preset,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::engine::USoundEffectSubmixPreset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_replace_sound_effect_submix,
                __buffer,
            )
        };
    }
    pub fn remove_submix_effect_preset_at_index(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        sound_submix: UPtr<crate::bindings::engine::USoundSubmix>,
        submix_chain_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_remove_submix_effect_preset_at_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sound_submix,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix_chain_index,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_remove_submix_effect_preset_at_index,
                __buffer,
            )
        };
    }
    pub fn remove_submix_effect_preset(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        sound_submix: UPtr<crate::bindings::engine::USoundSubmix>,
        submix_effect_preset: UPtr<crate::bindings::engine::USoundEffectSubmixPreset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_remove_submix_effect_preset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sound_submix,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix_effect_preset,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::USoundEffectSubmixPreset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_remove_submix_effect_preset,
                __buffer,
            )
        };
    }
    pub fn remove_submix_effect_at_index(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        sound_submix: UPtr<crate::bindings::engine::USoundSubmix>,
        submix_chain_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_remove_submix_effect_at_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sound_submix,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix_chain_index,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_remove_submix_effect_at_index,
                __buffer,
            )
        };
    }
    pub fn remove_submix_effect(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        sound_submix: UPtr<crate::bindings::engine::USoundSubmix>,
        submix_effect_preset: UPtr<crate::bindings::engine::USoundEffectSubmixPreset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_remove_submix_effect,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sound_submix,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix_effect_preset,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::USoundEffectSubmixPreset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_remove_submix_effect,
                __buffer,
            )
        };
    }
    pub fn remove_source_effect_from_preset_chain(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        preset_chain: UPtr<crate::bindings::engine::USoundEffectSourcePresetChain>,
        entry_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_remove_source_effect_from_preset_chain,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &preset_chain,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<crate::bindings::engine::USoundEffectSourcePresetChain>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &entry_index,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_remove_source_effect_from_preset_chain,
                __buffer,
            )
        };
    }
    pub fn remove_master_submix_effect(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        submix_effect_preset: UPtr<crate::bindings::engine::USoundEffectSubmixPreset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_remove_master_submix_effect,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix_effect_preset,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::USoundEffectSubmixPreset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_remove_master_submix_effect,
                __buffer,
            )
        };
    }
    pub fn register_audio_bus_to_submix(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        sound_submix: UPtr<crate::bindings::engine::USoundSubmix>,
        audio_bus: UPtr<crate::bindings::engine::UAudioBus>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_register_audio_bus_to_submix,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sound_submix,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &audio_bus,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UAudioBus>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_register_audio_bus_to_submix,
                __buffer,
            )
        };
    }
    pub fn prime_sound_for_playback(
        sound_wave: UPtr<crate::bindings::engine::USoundWave>,
        on_load_completion: FPrimeSoundForPlayback_OnLoadCompletion,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_prime_sound_for_playback,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sound_wave,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USoundWave>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_load_completion,
                __buffer.add(8).cast::<FPrimeSoundForPlayback_OnLoadCompletion>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_prime_sound_for_playback,
                __buffer,
            )
        };
    }
    pub fn prime_sound_cue_for_playback(
        sound_cue: UPtr<crate::bindings::engine::USoundCue>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_prime_sound_cue_for_playback,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sound_cue,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USoundCue>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_prime_sound_cue_for_playback,
                __buffer,
            )
        };
    }
    pub fn pause_recording_output(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        submix_to_pause: UPtr<crate::bindings::engine::USoundSubmix>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_pause_recording_output,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix_to_pause,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_pause_recording_output,
                __buffer,
            )
        };
    }
    pub fn make_preset_spectral_analysis_band_settings(
        in_band_preset_type: crate::bindings::engine::EAudioSpectrumBandPresetType,
        in_num_bands: i32,
        in_attack_time_msec: i32,
        in_release_time_msec: i32,
    ) -> TArray<crate::bindings::engine::FSoundSubmixSpectralAnalysisBandSettings> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_make_preset_spectral_analysis_band_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_band_preset_type,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::engine::EAudioSpectrumBandPresetType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_num_bands,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_attack_time_msec,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_release_time_msec,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_make_preset_spectral_analysis_band_settings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<
                    TArray<
                        crate::bindings::engine::FSoundSubmixSpectralAnalysisBandSettings,
                    >,
                >()
                .read()
        }
    }
    pub fn make_musical_spectral_analysis_band_settings(
        in_num_semitones: i32,
        in_starting_musical_note: EMusicalNoteName,
        in_starting_octave: i32,
        in_attack_time_msec: i32,
        in_release_time_msec: i32,
    ) -> TArray<crate::bindings::engine::FSoundSubmixSpectralAnalysisBandSettings> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_make_musical_spectral_analysis_band_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_num_semitones,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_starting_musical_note,
                __buffer.add(4).cast::<EMusicalNoteName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_starting_octave,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_attack_time_msec,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_release_time_msec,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_make_musical_spectral_analysis_band_settings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<
                    TArray<
                        crate::bindings::engine::FSoundSubmixSpectralAnalysisBandSettings,
                    >,
                >()
                .read()
        }
    }
    pub fn make_full_spectrum_spectral_analysis_band_settings(
        in_num_bands: i32,
        in_minimum_frequency: f32,
        in_maximum_frequency: f32,
        in_attack_time_msec: i32,
        in_release_time_msec: i32,
    ) -> TArray<crate::bindings::engine::FSoundSubmixSpectralAnalysisBandSettings> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_make_full_spectrum_spectral_analysis_band_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_num_bands,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_minimum_frequency,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_maximum_frequency,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_attack_time_msec,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_release_time_msec,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_make_full_spectrum_spectral_analysis_band_settings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<
                    TArray<
                        crate::bindings::engine::FSoundSubmixSpectralAnalysisBandSettings,
                    >,
                >()
                .read()
        }
    }
    pub fn is_audio_bus_active(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        audio_bus: UPtr<crate::bindings::engine::UAudioBus>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_is_audio_bus_active,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &audio_bus,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UAudioBus>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_is_audio_bus_active,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_phase_for_frequencies(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        frequencies: &TArray<f32>,
        phases: &mut TArray<f32>,
        submix_to_analyze: UPtr<crate::bindings::engine::USoundSubmix>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_get_phase_for_frequencies,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frequencies,
                __buffer.add(8).cast::<TArray<f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                phases,
                __buffer.add(24).cast::<TArray<f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix_to_analyze,
                __buffer.add(40).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_get_phase_for_frequencies,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<TArray<f32>>().swap(phases);
        }
    }
    pub fn get_number_of_entries_in_source_effect_chain(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        preset_chain: UPtr<crate::bindings::engine::USoundEffectSourcePresetChain>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_get_number_of_entries_in_source_effect_chain,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &preset_chain,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<crate::bindings::engine::USoundEffectSourcePresetChain>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_get_number_of_entries_in_source_effect_chain,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn get_magnitude_for_frequencies(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        frequencies: &TArray<f32>,
        magnitudes: &mut TArray<f32>,
        submix_to_analyze: UPtr<crate::bindings::engine::USoundSubmix>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_get_magnitude_for_frequencies,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frequencies,
                __buffer.add(8).cast::<TArray<f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                magnitudes,
                __buffer.add(24).cast::<TArray<f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix_to_analyze,
                __buffer.add(40).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_get_magnitude_for_frequencies,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<TArray<f32>>().swap(magnitudes);
        }
    }
    pub fn get_current_audio_output_device_name(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        on_obtain_current_device_event: &FGetCurrentAudioOutputDeviceName_OnObtainCurrentDeviceEvent,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_get_current_audio_output_device_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                on_obtain_current_device_event,
                __buffer
                    .add(8)
                    .cast::<
                        FGetCurrentAudioOutputDeviceName_OnObtainCurrentDeviceEvent,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_get_current_audio_output_device_name,
                __buffer,
            )
        };
    }
    pub fn get_available_audio_output_devices(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        on_obtain_devices_event: &FGetAvailableAudioOutputDevices_OnObtainDevicesEvent,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_get_available_audio_output_devices,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                on_obtain_devices_event,
                __buffer
                    .add(8)
                    .cast::<FGetAvailableAudioOutputDevices_OnObtainDevicesEvent>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_get_available_audio_output_devices,
                __buffer,
            )
        };
    }
    pub fn conv_audio_output_device_info_to_string(
        info: &FAudioOutputDeviceInfo,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_conv_audio_output_device_info_to_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                info,
                __buffer.add(0).cast::<FAudioOutputDeviceInfo>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_conv_audio_output_device_info_to_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<FString>().read() }
    }
    pub fn clear_submix_effects(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        sound_submix: UPtr<crate::bindings::engine::USoundSubmix>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_clear_submix_effects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sound_submix,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_clear_submix_effects,
                __buffer,
            )
        };
    }
    pub fn clear_submix_effect_chain_override(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        sound_submix: UPtr<crate::bindings::engine::USoundSubmix>,
        fade_time_sec: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_clear_submix_effect_chain_override,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sound_submix,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &fade_time_sec,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_clear_submix_effect_chain_override,
                __buffer,
            )
        };
    }
    pub fn clear_master_submix_effects(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_clear_master_submix_effects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_clear_master_submix_effects,
                __buffer,
            )
        };
    }
    pub fn add_submix_effect(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        sound_submix: UPtr<crate::bindings::engine::USoundSubmix>,
        submix_effect_preset: UPtr<crate::bindings::engine::USoundEffectSubmixPreset>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_add_submix_effect,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sound_submix,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USoundSubmix>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix_effect_preset,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::USoundEffectSubmixPreset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_add_submix_effect,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<i32>().read() }
    }
    pub fn add_source_effect_to_preset_chain(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        preset_chain: UPtr<crate::bindings::engine::USoundEffectSourcePresetChain>,
        entry: crate::bindings::engine::FSourceEffectChainEntry,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_add_source_effect_to_preset_chain,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &preset_chain,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<crate::bindings::engine::USoundEffectSourcePresetChain>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &entry,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::engine::FSourceEffectChainEntry>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_add_source_effect_to_preset_chain,
                __buffer,
            )
        };
    }
    pub fn add_master_submix_effect(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        submix_effect_preset: UPtr<crate::bindings::engine::USoundEffectSubmixPreset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_add_master_submix_effect,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix_effect_preset,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::USoundEffectSubmixPreset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::audio_mixer::UAudioMixerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_audio_mixer_blueprint_library_add_master_submix_effect,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct USynthSound {
    __padding_end: [u8; 2192],
}
impl USynthSound {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USynthSound")
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
pub struct USynthComponent {
    #[doc(hidden)]
    pub(crate) __padding_656: [u8; 656],
    pub flags_656: u8,
    #[doc(hidden)]
    pub(crate) __padding_660: [u8; 3],
    pub flags_660: u8,
    pub attenuation_settings: UPtr<crate::bindings::engine::USoundAttenuation>,
    pub attenuation_overrides: crate::bindings::engine::FSoundAttenuationSettings,
    #[doc(hidden)]
    pub(crate) __padding_1704: [u8; 8],
    pub concurrency_set: TSet<UPtr<crate::bindings::engine::USoundConcurrency>>,
    pub modulation_routing: crate::bindings::engine::FSoundModulationDefaultRoutingSettings,
    #[doc(hidden)]
    pub(crate) __padding_2200: [u8; 24],
    pub sound_submix_sends: TArray<crate::bindings::engine::FSoundSubmixSendInfo>,
    pub bus_sends: TArray<crate::bindings::engine::FSoundSourceBusSendInfo>,
    pub pre_effect_bus_sends: TArray<crate::bindings::engine::FSoundSourceBusSendInfo>,
    pub flags_2248: u8,
    pub envelope_follower_attack_time: i32,
    pub envelope_follower_release_time: i32,
    __padding_end: [u8; 124],
}
impl USynthComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USynthComponent")
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
    pub fn stop(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS.u_synth_component_stop,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS.u_synth_component_stop,
                __buffer,
            )
        };
    }
    pub fn start(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS.u_synth_component_start,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS.u_synth_component_start,
                __buffer,
            )
        };
    }
    pub fn set_volume_multiplier(&mut self, volume_multiplier: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_set_volume_multiplier,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &volume_multiplier,
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
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_set_volume_multiplier,
                __buffer,
            )
        };
    }
    pub fn set_submix_send(
        &mut self,
        submix: UPtr<crate::bindings::engine::USoundSubmixBase>,
        send_level: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_set_submix_send,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &submix,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::USoundSubmixBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&send_level, __buffer.add(8).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_set_submix_send,
                __buffer,
            )
        };
    }
    pub fn set_source_bus_send_pre_effect(
        &mut self,
        sound_source_bus: UPtr<crate::bindings::engine::USoundSourceBus>,
        source_bus_send_level: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_set_source_bus_send_pre_effect,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sound_source_bus,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USoundSourceBus>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_bus_send_level,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_set_source_bus_send_pre_effect,
                __buffer,
            )
        };
    }
    pub fn set_source_bus_send_post_effect(
        &mut self,
        sound_source_bus: UPtr<crate::bindings::engine::USoundSourceBus>,
        source_bus_send_level: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_set_source_bus_send_post_effect,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sound_source_bus,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USoundSourceBus>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_bus_send_level,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_set_source_bus_send_post_effect,
                __buffer,
            )
        };
    }
    pub fn set_output_to_bus_only(&mut self, b_in_output_to_bus_only: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_set_output_to_bus_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_output_to_bus_only,
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
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_set_output_to_bus_only,
                __buffer,
            )
        };
    }
    pub fn set_modulation_routing(
        &mut self,
        modulators: &TSet<UPtr<crate::bindings::audio_extensions::USoundModulatorBase>>,
        destination: crate::bindings::engine::EModulationDestination,
        routing_method: crate::bindings::engine::EModulationRouting,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<82>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_set_modulation_routing,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination,
                __buffer
                    .add(80)
                    .cast::<crate::bindings::engine::EModulationDestination>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &routing_method,
                __buffer.add(81).cast::<crate::bindings::engine::EModulationRouting>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_set_modulation_routing,
                __buffer,
            )
        };
    }
    pub fn set_low_pass_filter_frequency(&mut self, in_low_pass_filter_frequency: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_set_low_pass_filter_frequency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_low_pass_filter_frequency,
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
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_set_low_pass_filter_frequency,
                __buffer,
            )
        };
    }
    pub fn set_low_pass_filter_enabled(&mut self, in_low_pass_filter_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_set_low_pass_filter_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_low_pass_filter_enabled,
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
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_set_low_pass_filter_enabled,
                __buffer,
            )
        };
    }
    pub fn set_audio_bus_send_pre_effect(
        &mut self,
        audio_bus: UPtr<crate::bindings::engine::UAudioBus>,
        audio_bus_send_level: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_set_audio_bus_send_pre_effect,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &audio_bus_send_level,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_set_audio_bus_send_pre_effect,
                __buffer,
            )
        };
    }
    pub fn set_audio_bus_send_post_effect(
        &mut self,
        audio_bus: UPtr<crate::bindings::engine::UAudioBus>,
        audio_bus_send_level: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_set_audio_bus_send_post_effect,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &audio_bus_send_level,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_set_audio_bus_send_post_effect,
                __buffer,
            )
        };
    }
    pub fn is_playing(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_is_playing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_is_playing,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_modulators(
        &mut self,
        destination: crate::bindings::engine::EModulationDestination,
    ) -> TSet<UPtr<crate::bindings::audio_extensions::USoundModulatorBase>> {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_get_modulators,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::engine::EModulationDestination>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_get_modulators,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<
                    TSet<UPtr<crate::bindings::audio_extensions::USoundModulatorBase>>,
                >()
                .read()
        }
    }
    pub fn fade_out(
        &self,
        fade_out_duration: f32,
        fade_volume_level: f32,
        fade_curve: crate::bindings::engine::EAudioFaderCurve,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS.u_synth_component_fade_out,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &fade_out_duration,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &fade_volume_level,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &fade_curve,
                __buffer.add(8).cast::<crate::bindings::engine::EAudioFaderCurve>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS.u_synth_component_fade_out,
                __buffer,
            )
        };
    }
    pub fn fade_in(
        &self,
        fade_in_duration: f32,
        fade_volume_level: f32,
        start_time: f32,
        fade_curve: crate::bindings::engine::EAudioFaderCurve,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS.u_synth_component_fade_in,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &fade_in_duration,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &fade_volume_level,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&start_time, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &fade_curve,
                __buffer.add(12).cast::<crate::bindings::engine::EAudioFaderCurve>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS.u_synth_component_fade_in,
                __buffer,
            )
        };
    }
    pub fn adjust_volume(
        &self,
        adjust_volume_duration: f32,
        adjust_volume_level: f32,
        fade_curve: crate::bindings::engine::EAudioFaderCurve,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_adjust_volume,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &adjust_volume_duration,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &adjust_volume_level,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &fade_curve,
                __buffer.add(8).cast::<crate::bindings::engine::EAudioFaderCurve>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_synth_component_adjust_volume,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USubmixEffectDynamicsProcessorPreset {
    #[doc(hidden)]
    pub(crate) __padding_248: [u8; 248],
    pub settings: FSubmixEffectDynamicsProcessorSettings,
}
impl USubmixEffectDynamicsProcessorPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubmixEffectDynamicsProcessorPreset")
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
    pub fn set_settings(&mut self, settings: &FSubmixEffectDynamicsProcessorSettings) {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_submix_effect_dynamics_processor_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                settings,
                __buffer.add(0).cast::<FSubmixEffectDynamicsProcessorSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_submix_effect_dynamics_processor_preset_set_settings,
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
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_submix_effect_dynamics_processor_preset_set_external_submix,
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
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_submix_effect_dynamics_processor_preset_set_external_submix,
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
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_submix_effect_dynamics_processor_preset_set_audio_bus,
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
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_submix_effect_dynamics_processor_preset_set_audio_bus,
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
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_submix_effect_dynamics_processor_preset_reset_key,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_submix_effect_dynamics_processor_preset_reset_key,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USubmixEffectSubmixEQPreset {
    #[doc(hidden)]
    pub(crate) __padding_168: [u8; 168],
    pub settings: FSubmixEffectSubmixEQSettings,
}
impl USubmixEffectSubmixEQPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubmixEffectSubmixEQPreset")
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
    pub fn set_settings(&mut self, in_settings: &FSubmixEffectSubmixEQSettings) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_submix_effect_submix_eq_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSubmixEffectSubmixEQSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_submix_effect_submix_eq_preset_set_settings,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USubmixEffectReverbPreset {
    #[doc(hidden)]
    pub(crate) __padding_216: [u8; 216],
    pub settings: FSubmixEffectReverbSettings,
}
impl USubmixEffectReverbPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubmixEffectReverbPreset")
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
    pub fn set_settings_with_reverb_effect(
        &mut self,
        in_reverb_effect: UPtr<crate::bindings::engine::UReverbEffect>,
        wet_level: f32,
        dry_level: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_submix_effect_reverb_preset_set_settings_with_reverb_effect,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_reverb_effect,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UReverbEffect>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&wet_level, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&dry_level, __buffer.add(12).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_submix_effect_reverb_preset_set_settings_with_reverb_effect,
                __buffer,
            )
        };
    }
    pub fn set_settings(&mut self, in_settings: &FSubmixEffectReverbSettings) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_submix_effect_reverb_preset_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(0).cast::<FSubmixEffectReverbSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_submix_effect_reverb_preset_set_settings,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UAudioGenerator {
    __padding_end: [u8; 176],
}
impl UAudioGenerator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioGenerator")
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
pub struct UScrubbedSound {
    __padding_end: [u8; 2136],
}
impl UScrubbedSound {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UScrubbedSound")
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
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_scrubbed_sound_set_sound_wave,
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
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_scrubbed_sound_set_sound_wave,
                __buffer,
            )
        };
    }
    pub fn set_playhead_time(&mut self, in_playhead_time_seconds: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_scrubbed_sound_set_playhead_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_playhead_time_seconds,
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
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_scrubbed_sound_set_playhead_time,
                __buffer,
            )
        };
    }
    pub fn set_is_scrubbing_while_stationary(
        &mut self,
        b_in_scrub_while_stationary: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_scrubbed_sound_set_is_scrubbing_while_stationary,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_scrub_while_stationary,
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
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_scrubbed_sound_set_is_scrubbing_while_stationary,
                __buffer,
            )
        };
    }
    pub fn set_is_scrubbing(&mut self, b_in_is_scrubbing: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_scrubbed_sound_set_is_scrubbing,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_is_scrubbing,
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
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_scrubbed_sound_set_is_scrubbing,
                __buffer,
            )
        };
    }
    pub fn set_grain_duration_range(
        &mut self,
        in_grain_duration_range_seconds: &crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_scrubbed_sound_set_grain_duration_range,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_grain_duration_range_seconds,
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
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_scrubbed_sound_set_grain_duration_range,
                __buffer,
            )
        };
    }
    pub fn get_playhead_time(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_scrubbed_sound_get_playhead_time,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_scrubbed_sound_get_playhead_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UQuartzClockHandle {
    __padding_end: [u8; 688],
}
impl UQuartzClockHandle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UQuartzClockHandle")
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
    pub fn unsubscribe_from_time_division(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_quantization_boundary: crate::bindings::engine::EQuartzCommandQuantization,
        clock_handle: &mut UPtr<UQuartzClockHandle>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_unsubscribe_from_time_division,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_quantization_boundary,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::engine::EQuartzCommandQuantization>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                clock_handle,
                __buffer.add(16).cast::<UPtr<UQuartzClockHandle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_unsubscribe_from_time_division,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<UPtr<UQuartzClockHandle>>().swap(clock_handle);
        }
    }
    pub fn unsubscribe_from_all_time_divisions(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        clock_handle: &mut UPtr<UQuartzClockHandle>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_unsubscribe_from_all_time_divisions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                clock_handle,
                __buffer.add(8).cast::<UPtr<UQuartzClockHandle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_unsubscribe_from_all_time_divisions,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<UPtr<UQuartzClockHandle>>().swap(clock_handle);
        }
    }
    pub fn subscribe_to_quantization_event(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_quantization_boundary: crate::bindings::engine::EQuartzCommandQuantization,
        on_quantization_event: &FSubscribeToQuantizationEvent_OnQuantizationEvent,
        clock_handle: &mut UPtr<UQuartzClockHandle>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_subscribe_to_quantization_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_quantization_boundary,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::engine::EQuartzCommandQuantization>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                on_quantization_event,
                __buffer
                    .add(16)
                    .cast::<FSubscribeToQuantizationEvent_OnQuantizationEvent>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                clock_handle,
                __buffer.add(48).cast::<UPtr<UQuartzClockHandle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_subscribe_to_quantization_event,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(48).cast::<UPtr<UQuartzClockHandle>>().swap(clock_handle);
        }
    }
    pub fn subscribe_to_all_quantization_events(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        on_quantization_event: &FSubscribeToAllQuantizationEvents_OnQuantizationEvent,
        clock_handle: &mut UPtr<UQuartzClockHandle>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_subscribe_to_all_quantization_events,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                on_quantization_event,
                __buffer
                    .add(8)
                    .cast::<FSubscribeToAllQuantizationEvents_OnQuantizationEvent>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                clock_handle,
                __buffer.add(40).cast::<UPtr<UQuartzClockHandle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_subscribe_to_all_quantization_events,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<UPtr<UQuartzClockHandle>>().swap(clock_handle);
        }
    }
    pub fn stop_clock(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        cancel_pending_events: bool,
        clock_handle: &mut UPtr<UQuartzClockHandle>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_stop_clock,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &cancel_pending_events,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                clock_handle,
                __buffer.add(16).cast::<UPtr<UQuartzClockHandle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_stop_clock,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<UPtr<UQuartzClockHandle>>().swap(clock_handle);
        }
    }
    pub fn start_other_clock(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        other_clock_name: FName,
        in_quantization_boundary: crate::bindings::engine::FQuartzQuantizationBoundary,
        in_delegate: &FStartOtherClock_InDelegate,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_start_other_clock,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &other_clock_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_quantization_boundary,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::engine::FQuartzQuantizationBoundary>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_delegate,
                __buffer.add(56).cast::<FStartOtherClock_InDelegate>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_start_other_clock,
                __buffer,
            )
        };
    }
    pub fn start_clock(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        clock_handle: &mut UPtr<UQuartzClockHandle>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_start_clock,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                clock_handle,
                __buffer.add(8).cast::<UPtr<UQuartzClockHandle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_start_clock,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<UPtr<UQuartzClockHandle>>().swap(clock_handle);
        }
    }
    pub fn set_ticks_per_second(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        quantization_boundary: &crate::bindings::engine::FQuartzQuantizationBoundary,
        delegate: &FSetTicksPerSecond_Delegate,
        clock_handle: &mut UPtr<UQuartzClockHandle>,
        ticks_per_second: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<84>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_set_ticks_per_second,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                quantization_boundary,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::engine::FQuartzQuantizationBoundary>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                delegate,
                __buffer.add(40).cast::<FSetTicksPerSecond_Delegate>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                clock_handle,
                __buffer.add(72).cast::<UPtr<UQuartzClockHandle>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ticks_per_second,
                __buffer.add(80).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_set_ticks_per_second,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<UPtr<UQuartzClockHandle>>().swap(clock_handle);
        }
    }
    pub fn set_thirty_second_notes_per_minute(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        quantization_boundary: &crate::bindings::engine::FQuartzQuantizationBoundary,
        delegate: &FSetThirtySecondNotesPerMinute_Delegate,
        clock_handle: &mut UPtr<UQuartzClockHandle>,
        thirty_seconds_notes_per_minute: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<84>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_set_thirty_second_notes_per_minute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                quantization_boundary,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::engine::FQuartzQuantizationBoundary>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                delegate,
                __buffer.add(40).cast::<FSetThirtySecondNotesPerMinute_Delegate>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                clock_handle,
                __buffer.add(72).cast::<UPtr<UQuartzClockHandle>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &thirty_seconds_notes_per_minute,
                __buffer.add(80).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_set_thirty_second_notes_per_minute,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<UPtr<UQuartzClockHandle>>().swap(clock_handle);
        }
    }
    pub fn set_seconds_per_tick(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        quantization_boundary: &crate::bindings::engine::FQuartzQuantizationBoundary,
        delegate: &FSetSecondsPerTick_Delegate,
        clock_handle: &mut UPtr<UQuartzClockHandle>,
        seconds_per_tick: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<84>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_set_seconds_per_tick,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                quantization_boundary,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::engine::FQuartzQuantizationBoundary>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                delegate,
                __buffer.add(40).cast::<FSetSecondsPerTick_Delegate>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                clock_handle,
                __buffer.add(72).cast::<UPtr<UQuartzClockHandle>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &seconds_per_tick,
                __buffer.add(80).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_set_seconds_per_tick,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<UPtr<UQuartzClockHandle>>().swap(clock_handle);
        }
    }
    pub fn set_milliseconds_per_tick(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        quantization_boundary: &crate::bindings::engine::FQuartzQuantizationBoundary,
        delegate: &FSetMillisecondsPerTick_Delegate,
        clock_handle: &mut UPtr<UQuartzClockHandle>,
        milliseconds_per_tick: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<84>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_set_milliseconds_per_tick,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                quantization_boundary,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::engine::FQuartzQuantizationBoundary>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                delegate,
                __buffer.add(40).cast::<FSetMillisecondsPerTick_Delegate>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                clock_handle,
                __buffer.add(72).cast::<UPtr<UQuartzClockHandle>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &milliseconds_per_tick,
                __buffer.add(80).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_set_milliseconds_per_tick,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<UPtr<UQuartzClockHandle>>().swap(clock_handle);
        }
    }
    pub fn set_beats_per_minute(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        quantization_boundary: &crate::bindings::engine::FQuartzQuantizationBoundary,
        delegate: &FSetBeatsPerMinute_Delegate,
        clock_handle: &mut UPtr<UQuartzClockHandle>,
        beats_per_minute: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<84>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_set_beats_per_minute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                quantization_boundary,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::engine::FQuartzQuantizationBoundary>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                delegate,
                __buffer.add(40).cast::<FSetBeatsPerMinute_Delegate>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                clock_handle,
                __buffer.add(72).cast::<UPtr<UQuartzClockHandle>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &beats_per_minute,
                __buffer.add(80).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_set_beats_per_minute,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<UPtr<UQuartzClockHandle>>().swap(clock_handle);
        }
    }
    pub fn resume_clock(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        clock_handle: &mut UPtr<UQuartzClockHandle>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_resume_clock,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                clock_handle,
                __buffer.add(8).cast::<UPtr<UQuartzClockHandle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_resume_clock,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<UPtr<UQuartzClockHandle>>().swap(clock_handle);
        }
    }
    pub fn reset_transport_quantized(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_quantization_boundary: crate::bindings::engine::FQuartzQuantizationBoundary,
        in_delegate: &FResetTransportQuantized_InDelegate,
        clock_handle: &mut UPtr<UQuartzClockHandle>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_reset_transport_quantized,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_quantization_boundary,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::engine::FQuartzQuantizationBoundary>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_delegate,
                __buffer.add(40).cast::<FResetTransportQuantized_InDelegate>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                clock_handle,
                __buffer.add(72).cast::<UPtr<UQuartzClockHandle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_reset_transport_quantized,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<UPtr<UQuartzClockHandle>>().swap(clock_handle);
        }
    }
    pub fn reset_transport(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_delegate: &FResetTransport_InDelegate,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_reset_transport,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_delegate,
                __buffer.add(8).cast::<FResetTransport_InDelegate>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_reset_transport,
                __buffer,
            )
        };
    }
    pub fn pause_clock(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        clock_handle: &mut UPtr<UQuartzClockHandle>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_pause_clock,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                clock_handle,
                __buffer.add(8).cast::<UPtr<UQuartzClockHandle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_pause_clock,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<UPtr<UQuartzClockHandle>>().swap(clock_handle);
        }
    }
    pub fn notify_on_quantization_boundary(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_quantization_boundary: crate::bindings::engine::FQuartzQuantizationBoundary,
        in_delegate: &FNotifyOnQuantizationBoundary_InDelegate,
        in_ms_offset: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<76>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_notify_on_quantization_boundary,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_quantization_boundary,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::engine::FQuartzQuantizationBoundary>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_delegate,
                __buffer.add(40).cast::<FNotifyOnQuantizationBoundary_InDelegate>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_ms_offset,
                __buffer.add(72).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_notify_on_quantization_boundary,
                __buffer,
            )
        };
    }
    pub fn is_clock_running(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_is_clock_running,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_is_clock_running,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_ticks_per_second(
        &self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_get_ticks_per_second,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_get_ticks_per_second,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_thirty_second_notes_per_minute(
        &self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_get_thirty_second_notes_per_minute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_get_thirty_second_notes_per_minute,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_seconds_per_tick(
        &self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_get_seconds_per_tick,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_get_seconds_per_tick,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_milliseconds_per_tick(
        &self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_get_milliseconds_per_tick,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_get_milliseconds_per_tick,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_estimated_run_time(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_get_estimated_run_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_get_estimated_run_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_duration_of_quantization_type_in_seconds(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        quantization_type: &crate::bindings::engine::EQuartzCommandQuantization,
        multiplier: f32,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_get_duration_of_quantization_type_in_seconds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                quantization_type,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::engine::EQuartzCommandQuantization>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &multiplier,
                __buffer.add(12).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_get_duration_of_quantization_type_in_seconds,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn get_current_timestamp(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> crate::bindings::engine::FQuartzTransportTimeStamp {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_get_current_timestamp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_get_current_timestamp,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::engine::FQuartzTransportTimeStamp>()
                .read()
        }
    }
    pub fn get_beats_per_minute(
        &self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_get_beats_per_minute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_get_beats_per_minute,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_beat_progress_percent(
        &mut self,
        quantization_boundary: crate::bindings::engine::EQuartzCommandQuantization,
        phase_offset: f32,
        ms_offset: f32,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_get_beat_progress_percent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &quantization_boundary,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::engine::EQuartzCommandQuantization>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &phase_offset,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&ms_offset, __buffer.add(8).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_clock_handle_get_beat_progress_percent,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UQuartzSubsystem {
    __padding_end: [u8; 120],
}
impl UQuartzSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UQuartzSubsystem")
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
    pub fn set_quartz_subsystem_tickable_when_paused(
        &mut self,
        b_in_tickable_when_paused: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_set_quartz_subsystem_tickable_when_paused,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_tickable_when_paused,
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
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_set_quartz_subsystem_tickable_when_paused,
                __buffer,
            )
        };
    }
    pub fn is_quartz_enabled(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_is_quartz_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_is_quartz_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_clock_running(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        clock_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_is_clock_running,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &clock_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_is_clock_running,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_round_trip_min_latency(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_round_trip_min_latency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_round_trip_min_latency,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_round_trip_max_latency(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_round_trip_max_latency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_round_trip_max_latency,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_round_trip_average_latency(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_round_trip_average_latency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_round_trip_average_latency,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_handle_for_clock(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        clock_name: FName,
    ) -> UPtr<UQuartzClockHandle> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_handle_for_clock,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &clock_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_handle_for_clock,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UQuartzClockHandle>>().read() }
    }
    pub fn get_game_thread_to_audio_render_thread_min_latency(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_game_thread_to_audio_render_thread_min_latency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_game_thread_to_audio_render_thread_min_latency,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_game_thread_to_audio_render_thread_max_latency(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_game_thread_to_audio_render_thread_max_latency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_game_thread_to_audio_render_thread_max_latency,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_game_thread_to_audio_render_thread_average_latency(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_game_thread_to_audio_render_thread_average_latency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_game_thread_to_audio_render_thread_average_latency,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_estimated_clock_run_time(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_clock_name: &FName,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_estimated_clock_run_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_clock_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_estimated_clock_run_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<f32>().read() }
    }
    pub fn get_duration_of_quantization_type_in_seconds(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        clock_name: FName,
        quantization_type: &crate::bindings::engine::EQuartzCommandQuantization,
        multiplier: f32,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_duration_of_quantization_type_in_seconds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &clock_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                quantization_type,
                __buffer
                    .add(20)
                    .cast::<crate::bindings::engine::EQuartzCommandQuantization>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &multiplier,
                __buffer.add(24).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_duration_of_quantization_type_in_seconds,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<f32>().read() }
    }
    pub fn get_current_clock_timestamp(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_clock_name: &FName,
    ) -> crate::bindings::engine::FQuartzTransportTimeStamp {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_current_clock_timestamp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_clock_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_current_clock_timestamp,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(20)
                .cast::<crate::bindings::engine::FQuartzTransportTimeStamp>()
                .read()
        }
    }
    pub fn get_audio_render_thread_to_game_thread_min_latency(&mut self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_audio_render_thread_to_game_thread_min_latency,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_audio_render_thread_to_game_thread_min_latency,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_audio_render_thread_to_game_thread_max_latency(&mut self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_audio_render_thread_to_game_thread_max_latency,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_audio_render_thread_to_game_thread_max_latency,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_audio_render_thread_to_game_thread_average_latency(&mut self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_audio_render_thread_to_game_thread_average_latency,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_get_audio_render_thread_to_game_thread_average_latency,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn does_clock_exist(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        clock_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_does_clock_exist,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &clock_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_does_clock_exist,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn delete_clock_by_name(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        clock_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_delete_clock_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &clock_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_delete_clock_by_name,
                __buffer,
            )
        };
    }
    pub fn delete_clock_by_handle(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_clock_handle: &mut UPtr<UQuartzClockHandle>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_delete_clock_by_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_clock_handle,
                __buffer.add(8).cast::<UPtr<UQuartzClockHandle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_delete_clock_by_handle,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<UPtr<UQuartzClockHandle>>().swap(in_clock_handle);
        }
    }
    pub fn create_new_clock(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        clock_name: FName,
        in_settings: crate::bindings::engine::FQuartzClockSettings,
        b_override_settings_if_clock_exists: bool,
        b_use_audio_engine_clock_manager: bool,
    ) -> UPtr<UQuartzClockHandle> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_create_new_clock,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &clock_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(24).cast::<crate::bindings::engine::FQuartzClockSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_override_settings_if_clock_exists,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_audio_engine_clock_manager,
                __buffer.add(57).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_mixer::__FUNCTION_PTRS
                    .u_quartz_subsystem_create_new_clock,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<UPtr<UQuartzClockHandle>>().read() }
    }
}
#[repr(C, align(8))]
pub struct FGetAvailableAudioOutputDevices_OnObtainDevicesEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FGetCurrentAudioOutputDeviceName_OnObtainCurrentDeviceEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FPrimeSoundForPlayback_OnLoadCompletion {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSwapAudioOutputDevice_OnCompletedDeviceSwap {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FNotifyOnQuantizationBoundary_InDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FResetTransport_InDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FResetTransportQuantized_InDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetBeatsPerMinute_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetMillisecondsPerTick_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetSecondsPerTick_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetThirtySecondNotesPerMinute_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetTicksPerSecond_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FStartOtherClock_InDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSubscribeToAllQuantizationEvents_OnQuantizationEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSubscribeToQuantizationEvent_OnQuantizationEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAudioDeviceNotificationSubsystem_DefaultCaptureDeviceChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioDeviceNotificationSubsystem_DefaultRenderDeviceChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioDeviceNotificationSubsystem_DeviceAdded {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioDeviceNotificationSubsystem_DeviceRemoved {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioDeviceNotificationSubsystem_DeviceStateChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioDeviceNotificationSubsystem_DeviceSwitched {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSynthComponent_OnAudioEnvelopeValue {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EAudioMixerStreamDataFormatType(pub u8);
impl EAudioMixerStreamDataFormatType {
    pub const UNKNOWN: EAudioMixerStreamDataFormatType = EAudioMixerStreamDataFormatType(
        0,
    );
    pub const FLOAT: EAudioMixerStreamDataFormatType = EAudioMixerStreamDataFormatType(
        1,
    );
    pub const INT16: EAudioMixerStreamDataFormatType = EAudioMixerStreamDataFormatType(
        2,
    );
    pub const UNSUPPORTED: EAudioMixerStreamDataFormatType = EAudioMixerStreamDataFormatType(
        3,
    );
}
#[repr(transparent)]
pub struct EAudioMixerChannelType(pub u8);
impl EAudioMixerChannelType {
    pub const FRONT_LEFT: EAudioMixerChannelType = EAudioMixerChannelType(0);
    pub const FRONT_RIGHT: EAudioMixerChannelType = EAudioMixerChannelType(1);
    pub const FRONT_CENTER: EAudioMixerChannelType = EAudioMixerChannelType(2);
    pub const LOW_FREQUENCY: EAudioMixerChannelType = EAudioMixerChannelType(3);
    pub const BACK_LEFT: EAudioMixerChannelType = EAudioMixerChannelType(4);
    pub const BACK_RIGHT: EAudioMixerChannelType = EAudioMixerChannelType(5);
    pub const FRONT_LEFT_OF_CENTER: EAudioMixerChannelType = EAudioMixerChannelType(6);
    pub const FRONT_RIGHT_OF_CENTER: EAudioMixerChannelType = EAudioMixerChannelType(7);
    pub const BACK_CENTER: EAudioMixerChannelType = EAudioMixerChannelType(8);
    pub const SIDE_LEFT: EAudioMixerChannelType = EAudioMixerChannelType(9);
    pub const SIDE_RIGHT: EAudioMixerChannelType = EAudioMixerChannelType(10);
    pub const TOP_CENTER: EAudioMixerChannelType = EAudioMixerChannelType(11);
    pub const TOP_FRONT_LEFT: EAudioMixerChannelType = EAudioMixerChannelType(12);
    pub const TOP_FRONT_CENTER: EAudioMixerChannelType = EAudioMixerChannelType(13);
    pub const TOP_FRONT_RIGHT: EAudioMixerChannelType = EAudioMixerChannelType(14);
    pub const TOP_BACK_LEFT: EAudioMixerChannelType = EAudioMixerChannelType(15);
    pub const TOP_BACK_CENTER: EAudioMixerChannelType = EAudioMixerChannelType(16);
    pub const TOP_BACK_RIGHT: EAudioMixerChannelType = EAudioMixerChannelType(17);
    pub const UNKNOWN: EAudioMixerChannelType = EAudioMixerChannelType(18);
    pub const CHANNEL_TYPE_COUNT: EAudioMixerChannelType = EAudioMixerChannelType(19);
    pub const DEFAULT_CHANNEL: EAudioMixerChannelType = EAudioMixerChannelType(0);
}
#[repr(transparent)]
pub struct ESwapAudioOutputDeviceResultState(pub u8);
impl ESwapAudioOutputDeviceResultState {
    pub const FAILURE: ESwapAudioOutputDeviceResultState = ESwapAudioOutputDeviceResultState(
        0,
    );
    pub const SUCCESS: ESwapAudioOutputDeviceResultState = ESwapAudioOutputDeviceResultState(
        1,
    );
    pub const NONE: ESwapAudioOutputDeviceResultState = ESwapAudioOutputDeviceResultState(
        2,
    );
}
#[repr(transparent)]
pub struct ESubmixEffectDynamicsProcessorType(pub u8);
impl ESubmixEffectDynamicsProcessorType {
    pub const COMPRESSOR: ESubmixEffectDynamicsProcessorType = ESubmixEffectDynamicsProcessorType(
        0,
    );
    pub const LIMITER: ESubmixEffectDynamicsProcessorType = ESubmixEffectDynamicsProcessorType(
        1,
    );
    pub const EXPANDER: ESubmixEffectDynamicsProcessorType = ESubmixEffectDynamicsProcessorType(
        2,
    );
    pub const GATE: ESubmixEffectDynamicsProcessorType = ESubmixEffectDynamicsProcessorType(
        3,
    );
    pub const UPWARDS_COMPRESSOR: ESubmixEffectDynamicsProcessorType = ESubmixEffectDynamicsProcessorType(
        4,
    );
    pub const COUNT: ESubmixEffectDynamicsProcessorType = ESubmixEffectDynamicsProcessorType(
        5,
    );
}
#[repr(transparent)]
pub struct ESubmixEffectDynamicsPeakMode(pub u8);
impl ESubmixEffectDynamicsPeakMode {
    pub const MEAN_SQUARED: ESubmixEffectDynamicsPeakMode = ESubmixEffectDynamicsPeakMode(
        0,
    );
    pub const ROOT_MEAN_SQUARED: ESubmixEffectDynamicsPeakMode = ESubmixEffectDynamicsPeakMode(
        1,
    );
    pub const PEAK: ESubmixEffectDynamicsPeakMode = ESubmixEffectDynamicsPeakMode(2);
    pub const COUNT: ESubmixEffectDynamicsPeakMode = ESubmixEffectDynamicsPeakMode(3);
}
#[repr(transparent)]
pub struct ESubmixEffectDynamicsChannelLinkMode(pub u8);
impl ESubmixEffectDynamicsChannelLinkMode {
    pub const DISABLED: ESubmixEffectDynamicsChannelLinkMode = ESubmixEffectDynamicsChannelLinkMode(
        0,
    );
    pub const AVERAGE: ESubmixEffectDynamicsChannelLinkMode = ESubmixEffectDynamicsChannelLinkMode(
        1,
    );
    pub const PEAK: ESubmixEffectDynamicsChannelLinkMode = ESubmixEffectDynamicsChannelLinkMode(
        2,
    );
    pub const COUNT: ESubmixEffectDynamicsChannelLinkMode = ESubmixEffectDynamicsChannelLinkMode(
        3,
    );
}
#[repr(transparent)]
pub struct ESubmixEffectDynamicsKeySource(pub u8);
impl ESubmixEffectDynamicsKeySource {
    pub const DEFAULT: ESubmixEffectDynamicsKeySource = ESubmixEffectDynamicsKeySource(
        0,
    );
    pub const AUDIO_BUS: ESubmixEffectDynamicsKeySource = ESubmixEffectDynamicsKeySource(
        1,
    );
    pub const SUBMIX: ESubmixEffectDynamicsKeySource = ESubmixEffectDynamicsKeySource(2);
    pub const COUNT: ESubmixEffectDynamicsKeySource = ESubmixEffectDynamicsKeySource(3);
}
#[repr(transparent)]
pub struct EMusicalNoteName(pub u8);
impl EMusicalNoteName {
    pub const C: EMusicalNoteName = EMusicalNoteName(0);
    pub const DB: EMusicalNoteName = EMusicalNoteName(1);
    pub const D: EMusicalNoteName = EMusicalNoteName(2);
    pub const EB: EMusicalNoteName = EMusicalNoteName(3);
    pub const E: EMusicalNoteName = EMusicalNoteName(4);
    pub const F: EMusicalNoteName = EMusicalNoteName(5);
    pub const GB: EMusicalNoteName = EMusicalNoteName(6);
    pub const G: EMusicalNoteName = EMusicalNoteName(7);
    pub const AB: EMusicalNoteName = EMusicalNoteName(8);
    pub const A: EMusicalNoteName = EMusicalNoteName(9);
    pub const BB: EMusicalNoteName = EMusicalNoteName(10);
    pub const B: EMusicalNoteName = EMusicalNoteName(11);
}
#[repr(transparent)]
pub struct EAudioDeviceChangedRole(pub u8);
impl EAudioDeviceChangedRole {
    pub const INVALID: EAudioDeviceChangedRole = EAudioDeviceChangedRole(0);
    pub const CONSOLE: EAudioDeviceChangedRole = EAudioDeviceChangedRole(1);
    pub const MULTIMEDIA: EAudioDeviceChangedRole = EAudioDeviceChangedRole(2);
    pub const COMMUNICATIONS: EAudioDeviceChangedRole = EAudioDeviceChangedRole(3);
    pub const COUNT: EAudioDeviceChangedRole = EAudioDeviceChangedRole(4);
}
#[repr(transparent)]
pub struct EAudioDeviceChangedState(pub u8);
impl EAudioDeviceChangedState {
    pub const INVALID: EAudioDeviceChangedState = EAudioDeviceChangedState(0);
    pub const ACTIVE: EAudioDeviceChangedState = EAudioDeviceChangedState(1);
    pub const DISABLED: EAudioDeviceChangedState = EAudioDeviceChangedState(2);
    pub const NOT_PRESENT: EAudioDeviceChangedState = EAudioDeviceChangedState(3);
    pub const UNPLUGGED: EAudioDeviceChangedState = EAudioDeviceChangedState(4);
    pub const COUNT: EAudioDeviceChangedState = EAudioDeviceChangedState(5);
}
