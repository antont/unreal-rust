#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(forgetting_copy_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_movie_scene_section_set_row_index: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_set_pre_roll_frames: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_set_post_roll_frames: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_set_overlap_priority: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_set_is_locked: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_set_is_active: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_set_completion_mode: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_set_color_tint: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_set_blend_type: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_is_locked: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_is_active: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_get_row_index: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_get_pre_roll_frames: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_get_post_roll_frames: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_get_overlap_priority: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_get_completion_mode: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_get_color_tint: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_get_blend_type: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_get_earliest_timecode_source: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_find_bindings_by_tag: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_find_binding_by_tag: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_custom_binding_get_base_engine_priority: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_custom_binding_get_base_custom_priority: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_condition_bp_get_scope: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_condition_bp_get_check_frequency: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_condition_bp_evaluate_condition: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sub_section_set_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sub_section_get_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_bound_object_proxy_bp_get_bound_object_for_sequencer: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_event_receiver_interface_on_object_unbound_by_sequencer: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_event_receiver_interface_on_object_bound_by_sequencer: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_easing_function_on_evaluate: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_custom_clock_source_on_tick: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_custom_clock_source_on_stop_playing: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_custom_clock_source_on_start_playing: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_custom_clock_source_on_request_current_time: *mut crate::ffi::UFunctionOpague,
    pub u_built_in_dynamic_binding_resolver_library_resolve_to_player_pawn: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_meta_data_set_notes: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_meta_data_set_created: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_meta_data_set_author: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_meta_data_get_notes: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_meta_data_get_created: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_meta_data_get_author: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_stop_at_current_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_stop: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_set_weight: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_set_time_range: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_set_play_rate: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_set_playback_position: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_set_hide_hud: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_set_frame_rate: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_set_frame_range: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_set_disable_camera_cuts: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_set_completion_mode_override: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_scrub: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_rpc_on_stop_event: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_rpc_on_finish_playback_event: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_rpc_explicit_server_update_event: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_restore_state: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_request_invalidate_binding: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_remove_weight: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_play_to: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_play_reverse: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_play_looping: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_play: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_pause: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_is_reversed: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_is_playing: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_is_paused: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_go_to_end_and_stop: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_get_start_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_get_sequence_name: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_get_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_get_play_rate: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_get_object_bindings: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_get_hide_hud: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_get_frame_rate: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_get_frame_duration: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_get_end_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_get_duration: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_get_disable_camera_cuts: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_get_current_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_get_completion_mode_override: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_get_bound_objects: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_player_change_playback_direction: *mut crate::ffi::UFunctionOpague,
    pub a_test_movie_scene_array_properties_actor_set_test_setter_float: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_movie_scene_section_set_row_index: std::ptr::null_mut(),
            u_movie_scene_section_set_pre_roll_frames: std::ptr::null_mut(),
            u_movie_scene_section_set_post_roll_frames: std::ptr::null_mut(),
            u_movie_scene_section_set_overlap_priority: std::ptr::null_mut(),
            u_movie_scene_section_set_is_locked: std::ptr::null_mut(),
            u_movie_scene_section_set_is_active: std::ptr::null_mut(),
            u_movie_scene_section_set_completion_mode: std::ptr::null_mut(),
            u_movie_scene_section_set_color_tint: std::ptr::null_mut(),
            u_movie_scene_section_set_blend_type: std::ptr::null_mut(),
            u_movie_scene_section_is_locked: std::ptr::null_mut(),
            u_movie_scene_section_is_active: std::ptr::null_mut(),
            u_movie_scene_section_get_row_index: std::ptr::null_mut(),
            u_movie_scene_section_get_pre_roll_frames: std::ptr::null_mut(),
            u_movie_scene_section_get_post_roll_frames: std::ptr::null_mut(),
            u_movie_scene_section_get_overlap_priority: std::ptr::null_mut(),
            u_movie_scene_section_get_completion_mode: std::ptr::null_mut(),
            u_movie_scene_section_get_color_tint: std::ptr::null_mut(),
            u_movie_scene_section_get_blend_type: std::ptr::null_mut(),
            u_movie_scene_sequence_get_earliest_timecode_source: std::ptr::null_mut(),
            u_movie_scene_sequence_find_bindings_by_tag: std::ptr::null_mut(),
            u_movie_scene_sequence_find_binding_by_tag: std::ptr::null_mut(),
            u_movie_scene_custom_binding_get_base_engine_priority: std::ptr::null_mut(),
            u_movie_scene_custom_binding_get_base_custom_priority: std::ptr::null_mut(),
            u_movie_scene_condition_bp_get_scope: std::ptr::null_mut(),
            u_movie_scene_condition_bp_get_check_frequency: std::ptr::null_mut(),
            u_movie_scene_condition_bp_evaluate_condition: std::ptr::null_mut(),
            u_movie_scene_sub_section_set_sequence: std::ptr::null_mut(),
            u_movie_scene_sub_section_get_sequence: std::ptr::null_mut(),
            u_movie_scene_bound_object_proxy_bp_get_bound_object_for_sequencer: std::ptr::null_mut(),
            u_movie_scene_binding_event_receiver_interface_on_object_unbound_by_sequencer: std::ptr::null_mut(),
            u_movie_scene_binding_event_receiver_interface_on_object_bound_by_sequencer: std::ptr::null_mut(),
            u_movie_scene_easing_function_on_evaluate: std::ptr::null_mut(),
            u_movie_scene_custom_clock_source_on_tick: std::ptr::null_mut(),
            u_movie_scene_custom_clock_source_on_stop_playing: std::ptr::null_mut(),
            u_movie_scene_custom_clock_source_on_start_playing: std::ptr::null_mut(),
            u_movie_scene_custom_clock_source_on_request_current_time: std::ptr::null_mut(),
            u_built_in_dynamic_binding_resolver_library_resolve_to_player_pawn: std::ptr::null_mut(),
            u_movie_scene_meta_data_set_notes: std::ptr::null_mut(),
            u_movie_scene_meta_data_set_created: std::ptr::null_mut(),
            u_movie_scene_meta_data_set_author: std::ptr::null_mut(),
            u_movie_scene_meta_data_get_notes: std::ptr::null_mut(),
            u_movie_scene_meta_data_get_created: std::ptr::null_mut(),
            u_movie_scene_meta_data_get_author: std::ptr::null_mut(),
            u_movie_scene_sequence_player_stop_at_current_time: std::ptr::null_mut(),
            u_movie_scene_sequence_player_stop: std::ptr::null_mut(),
            u_movie_scene_sequence_player_set_weight: std::ptr::null_mut(),
            u_movie_scene_sequence_player_set_time_range: std::ptr::null_mut(),
            u_movie_scene_sequence_player_set_play_rate: std::ptr::null_mut(),
            u_movie_scene_sequence_player_set_playback_position: std::ptr::null_mut(),
            u_movie_scene_sequence_player_set_hide_hud: std::ptr::null_mut(),
            u_movie_scene_sequence_player_set_frame_rate: std::ptr::null_mut(),
            u_movie_scene_sequence_player_set_frame_range: std::ptr::null_mut(),
            u_movie_scene_sequence_player_set_disable_camera_cuts: std::ptr::null_mut(),
            u_movie_scene_sequence_player_set_completion_mode_override: std::ptr::null_mut(),
            u_movie_scene_sequence_player_scrub: std::ptr::null_mut(),
            u_movie_scene_sequence_player_rpc_on_stop_event: std::ptr::null_mut(),
            u_movie_scene_sequence_player_rpc_on_finish_playback_event: std::ptr::null_mut(),
            u_movie_scene_sequence_player_rpc_explicit_server_update_event: std::ptr::null_mut(),
            u_movie_scene_sequence_player_restore_state: std::ptr::null_mut(),
            u_movie_scene_sequence_player_request_invalidate_binding: std::ptr::null_mut(),
            u_movie_scene_sequence_player_remove_weight: std::ptr::null_mut(),
            u_movie_scene_sequence_player_play_to: std::ptr::null_mut(),
            u_movie_scene_sequence_player_play_reverse: std::ptr::null_mut(),
            u_movie_scene_sequence_player_play_looping: std::ptr::null_mut(),
            u_movie_scene_sequence_player_play: std::ptr::null_mut(),
            u_movie_scene_sequence_player_pause: std::ptr::null_mut(),
            u_movie_scene_sequence_player_is_reversed: std::ptr::null_mut(),
            u_movie_scene_sequence_player_is_playing: std::ptr::null_mut(),
            u_movie_scene_sequence_player_is_paused: std::ptr::null_mut(),
            u_movie_scene_sequence_player_go_to_end_and_stop: std::ptr::null_mut(),
            u_movie_scene_sequence_player_get_start_time: std::ptr::null_mut(),
            u_movie_scene_sequence_player_get_sequence_name: std::ptr::null_mut(),
            u_movie_scene_sequence_player_get_sequence: std::ptr::null_mut(),
            u_movie_scene_sequence_player_get_play_rate: std::ptr::null_mut(),
            u_movie_scene_sequence_player_get_object_bindings: std::ptr::null_mut(),
            u_movie_scene_sequence_player_get_hide_hud: std::ptr::null_mut(),
            u_movie_scene_sequence_player_get_frame_rate: std::ptr::null_mut(),
            u_movie_scene_sequence_player_get_frame_duration: std::ptr::null_mut(),
            u_movie_scene_sequence_player_get_end_time: std::ptr::null_mut(),
            u_movie_scene_sequence_player_get_duration: std::ptr::null_mut(),
            u_movie_scene_sequence_player_get_disable_camera_cuts: std::ptr::null_mut(),
            u_movie_scene_sequence_player_get_current_time: std::ptr::null_mut(),
            u_movie_scene_sequence_player_get_completion_mode_override: std::ptr::null_mut(),
            u_movie_scene_sequence_player_get_bound_objects: std::ptr::null_mut(),
            u_movie_scene_sequence_player_change_playback_direction: std::ptr::null_mut(),
            a_test_movie_scene_array_properties_actor_set_test_setter_float: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMovieSceneSection::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetRowIndex"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_section_set_row_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPreRollFrames"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_section_set_pre_roll_frames,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPostRollFrames"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_section_set_post_roll_frames,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetOverlapPriority"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_section_set_overlap_priority,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetIsLocked"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_section_set_is_locked,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetIsActive"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_section_set_is_active,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCompletionMode"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_section_set_completion_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetColorTint"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_section_set_color_tint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBlendType"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_section_set_blend_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsLocked"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_section_is_locked,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsActive"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_section_is_active,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRowIndex"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_section_get_row_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPreRollFrames"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_section_get_pre_roll_frames,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPostRollFrames"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_section_get_post_roll_frames,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOverlapPriority"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_section_get_overlap_priority,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCompletionMode"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_section_get_completion_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetColorTint"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_section_get_color_tint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBlendType"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_section_get_blend_type,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMovieSceneSequence::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEarliestTimecodeSource"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_sequence_get_earliest_timecode_source,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindBindingsByTag"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_find_bindings_by_tag,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindBindingByTag"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_find_binding_by_tag,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMovieSceneCustomBinding::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBaseEnginePriority"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_custom_binding_get_base_engine_priority,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBaseCustomPriority"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_custom_binding_get_base_custom_priority,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMovieSceneCondition::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_GetScope"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_condition_bp_get_scope,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_GetCheckFrequency"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_condition_bp_get_check_frequency,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_EvaluateCondition"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_condition_bp_evaluate_condition,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMovieSceneSubSection::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSequence"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sub_section_set_sequence,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSequence"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sub_section_get_sequence,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMovieSceneBoundObjectProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_GetBoundObjectForSequencer"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_bound_object_proxy_bp_get_bound_object_for_sequencer,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMovieSceneBindingEventReceiverInterface::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnObjectUnboundBySequencer"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_binding_event_receiver_interface_on_object_unbound_by_sequencer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnObjectBoundBySequencer"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_binding_event_receiver_interface_on_object_bound_by_sequencer,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMovieSceneEasingFunction::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnEvaluate"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_easing_function_on_evaluate,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMovieSceneCustomClockSource::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnTick"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_custom_clock_source_on_tick,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnStopPlaying"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_custom_clock_source_on_stop_playing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnStartPlaying"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_custom_clock_source_on_start_playing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnRequestCurrentTime"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_custom_clock_source_on_request_current_time,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBuiltInDynamicBindingResolverLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResolveToPlayerPawn"),
                &raw mut __FUNCTION_PTRS
                    .u_built_in_dynamic_binding_resolver_library_resolve_to_player_pawn,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMovieSceneMetaData::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNotes"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_meta_data_set_notes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCreated"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_meta_data_set_created,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAuthor"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_meta_data_set_author,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNotes"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_meta_data_get_notes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCreated"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_meta_data_get_created,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAuthor"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_meta_data_get_author,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMovieSceneSequencePlayer::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StopAtCurrentTime"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_sequence_player_stop_at_current_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Stop"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_stop,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetWeight"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_set_weight,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTimeRange"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_set_time_range,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPlayRate"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_set_play_rate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPlaybackPosition"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_playback_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetHideHud"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_set_hide_hud,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetFrameRate"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_set_frame_rate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetFrameRange"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_set_frame_range,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDisableCameraCuts"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_disable_camera_cuts,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCompletionModeOverride"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_completion_mode_override,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Scrub"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_scrub,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RPC_OnStopEvent"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_rpc_on_stop_event,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RPC_OnFinishPlaybackEvent"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_sequence_player_rpc_on_finish_playback_event,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RPC_ExplicitServerUpdateEvent"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_sequence_player_rpc_explicit_server_update_event,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RestoreState"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_restore_state,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RequestInvalidateBinding"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_sequence_player_request_invalidate_binding,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveWeight"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_remove_weight,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PlayTo"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_play_to,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PlayReverse"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_play_reverse,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PlayLooping"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_play_looping,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Play"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_play,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Pause"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_pause,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsReversed"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_is_reversed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsPlaying"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_is_playing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsPaused"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_is_paused,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GoToEndAndStop"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_go_to_end_and_stop,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetStartTime"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_get_start_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSequenceName"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_get_sequence_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSequence"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_get_sequence,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPlayRate"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_get_play_rate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetObjectBindings"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_object_bindings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetHideHud"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_get_hide_hud,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFrameRate"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_get_frame_rate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFrameDuration"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_get_frame_duration,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEndTime"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_get_end_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDuration"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_get_duration,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDisableCameraCuts"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_disable_camera_cuts,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentTime"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_get_current_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCompletionModeOverride"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_completion_mode_override,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBoundObjects"),
                &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_player_get_bound_objects,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ChangePlaybackDirection"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_sequence_player_change_playback_direction,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ATestMovieSceneArrayPropertiesActor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTestSetterFloat"),
                &raw mut __FUNCTION_PTRS
                    .a_test_movie_scene_array_properties_actor_set_test_setter_float,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FMovieSceneBindingProxy {
    pub binding_id: crate::bindings::core_u_object::FGuid,
    pub sequence: UPtr<UMovieSceneSequence>,
}
impl FMovieSceneBindingProxy {}
#[repr(C, align(8))]
pub struct FMovieSceneDynamicBindingPayloadVariable {
    pub(crate) __padding_end: [u8; 56],
}
impl FMovieSceneDynamicBindingPayloadVariable {}
#[repr(C, align(4))]
pub struct FMovieSceneObjectBindingID {
    pub(crate) __padding_end: [u8; 28],
}
impl FMovieSceneObjectBindingID {}
#[repr(C, align(8))]
pub struct FMovieSceneTimeWarpVariant {
    pub(crate) __padding_end: [u8; 16],
}
impl FMovieSceneTimeWarpVariant {}
#[repr(C, align(8))]
pub struct FMovieSceneNumericVariant {
    pub(crate) __padding_end: [u8; 16],
}
impl FMovieSceneNumericVariant {}
#[repr(C, align(4))]
pub struct FActorForWorldTransforms {
    pub actor: TWeakObjectPtr<crate::bindings::engine::AActor>,
    pub component: TWeakObjectPtr<crate::bindings::engine::USceneComponent>,
    pub socket_name: FName,
}
impl FActorForWorldTransforms {}
#[repr(C, align(8))]
pub struct FMovieSceneSectionParameters {
    pub start_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub b_can_loop: bool,
    pub end_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub first_loop_start_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub time_scale: FMovieSceneTimeWarpVariant,
    pub hierarchical_bias: i32,
    pub flags: EMovieSceneSubSectionFlags,
    pub(crate) __padding_end: [u8; 19],
}
impl FMovieSceneSectionParameters {}
#[repr(C, align(4))]
pub struct FMovieSceneSequenceLoopCount {
    pub value: i32,
}
impl FMovieSceneSequenceLoopCount {}
#[repr(C, align(4))]
pub struct FMovieSceneSequencePlaybackSettings {
    pub flags_0: u8,
    pub loop_count: FMovieSceneSequenceLoopCount,
    pub tick_interval: FMovieSceneSequenceTickInterval,
    pub play_rate: f32,
    pub start_time: f32,
    pub flags_28: u8,
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 3],
    pub finish_completion_state_override: EMovieSceneCompletionModeOverride,
    #[doc(hidden)]
    pub(crate) __padding_36: [u8; 3],
    pub flags_36: u8,
}
impl FMovieSceneSequencePlaybackSettings {}
#[repr(C, align(4))]
pub struct FMovieSceneSequenceTickInterval {
    pub tick_interval_seconds: f32,
    pub evaluation_budget_microseconds: f32,
    pub b_tick_when_paused: bool,
    pub b_allow_rounding: bool,
}
impl FMovieSceneSequenceTickInterval {}
#[repr(C, align(8))]
pub struct FMovieSceneBindingResolveResult {
    pub objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub object: UPtr<crate::bindings::core_u_object::UObject>,
}
impl FMovieSceneBindingResolveResult {}
#[repr(C, align(8))]
pub struct FMovieSceneBindingResolveContext {
    pub world_context: UPtr<crate::bindings::core_u_object::UObject>,
    pub binding: FMovieSceneBindingProxy,
}
impl FMovieSceneBindingResolveContext {}
#[repr(C, align(8))]
pub struct FMovieSceneConditionContext {
    pub world_context: UPtr<crate::bindings::core_u_object::UObject>,
    pub binding: FMovieSceneBindingProxy,
    pub bound_objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
impl FMovieSceneConditionContext {}
#[repr(C, align(8))]
pub struct FMovieSceneConditionContainer {
    pub condition: UPtr<UMovieSceneCondition>,
}
impl FMovieSceneConditionContainer {}
#[repr(C, align(1))]
pub struct FOptionalMovieSceneBlendType {
    pub blend_type: EMovieSceneBlendType,
    pub b_is_valid: bool,
}
impl FOptionalMovieSceneBlendType {}
#[repr(C, align(8))]
pub struct FMovieSceneMarkedFrame {
    pub frame_number: crate::bindings::core_u_object::FFrameNumber,
    pub label: FString,
    #[doc(hidden)]
    pub(crate) __padding_76: [u8; 52],
    pub b_is_determinism_fence: bool,
    pub b_is_inclusive_time: bool,
}
impl FMovieSceneMarkedFrame {}
#[repr(C, align(8))]
pub struct FMovieSceneDynamicBindingResolveParams {
    pub sequence: UPtr<UMovieSceneSequence>,
    pub object_binding_id: crate::bindings::core_u_object::FGuid,
    pub root_sequence: UPtr<UMovieSceneSequence>,
}
impl FMovieSceneDynamicBindingResolveParams {}
#[repr(C, align(8))]
pub struct FMovieSceneDynamicBindingResolveResult {
    pub objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub object: UPtr<crate::bindings::core_u_object::UObject>,
    pub b_is_possessed_object: bool,
}
impl FMovieSceneDynamicBindingResolveResult {}
#[repr(C, align(4))]
pub struct FMovieSceneTimecodeSource {
    pub timecode: crate::bindings::core_u_object::FTimecode,
}
impl FMovieSceneTimecodeSource {}
#[repr(C, align(8))]
pub struct FMovieSceneSequencePlaybackParams {
    pub frame: crate::bindings::core_u_object::FFrameTime,
    pub time: f32,
    pub marked_frame: FString,
    pub timecode: crate::bindings::core_u_object::FTimecode,
    pub position_type: EMovieScenePositionType,
    pub update_method: EUpdatePositionMethod,
    pub b_has_jumped: bool,
}
impl FMovieSceneSequencePlaybackParams {}
#[repr(C, align(1))]
pub struct FMovieSceneSequencePlayToParams {
    pub b_exclusive: bool,
}
impl FMovieSceneSequencePlayToParams {}
#[repr(C, align(8))]
pub struct FMovieSceneSectionTimingParametersSeconds {
    pub play_rate: FMovieSceneTimeWarpVariant,
    pub inner_start_offset: f32,
    pub inner_end_offset: f32,
    pub first_loop_start_offset: f32,
    pub flags_28: u8,
}
impl FMovieSceneSectionTimingParametersSeconds {}
#[repr(C, align(8))]
pub struct FMovieSceneSectionTimingParametersFrames {
    pub play_rate: FMovieSceneTimeWarpVariant,
    pub inner_start_offset: crate::bindings::core_u_object::FFrameNumber,
    pub inner_end_offset: crate::bindings::core_u_object::FFrameNumber,
    pub first_loop_start_offset: crate::bindings::core_u_object::FFrameNumber,
    pub flags_28: u8,
}
impl FMovieSceneSectionTimingParametersFrames {}
#[repr(C, align(8))]
pub struct FTestMovieSceneStruct {
    pub first: f32,
    pub second: f32,
    pub enum_: ETestMovieSceneEnum,
    pub vector: crate::bindings::core_u_object::FVector,
    pub multiple_integers: TArray<i32>,
    pub multiple_vectors: TArray<crate::bindings::core_u_object::FVector>,
}
impl FTestMovieSceneStruct {}
#[repr(C, align(4))]
pub struct FTestMovieSceneStruct2 {
    pub first: f32,
    pub second: f32,
}
impl FTestMovieSceneStruct2 {}
#[repr(C, align(8))]
pub struct UMovieSceneEntitySystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneEntitySystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEntitySystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEntitySystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneSignedObject {
    __padding_end: [u8; 112],
}
impl UMovieSceneSignedObject {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSignedObject")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSignedObject")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneDecorationContainerObject {
    __padding_end: [u8; 128],
}
impl UMovieSceneDecorationContainerObject {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneDecorationContainerObject")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneDecorationContainerObject")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneSection {
    __padding_end: [u8; 360],
}
impl UMovieSceneSection {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSection")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSection")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn set_row_index(&mut self, new_row_index: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_set_row_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_row_index,
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
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_set_row_index,
                __buffer,
            )
        };
        std::mem::forget(new_row_index);
    }
    pub fn set_pre_roll_frames(&mut self, in_pre_roll_frames: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_set_pre_roll_frames,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pre_roll_frames,
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
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_set_pre_roll_frames,
                __buffer,
            )
        };
        std::mem::forget(in_pre_roll_frames);
    }
    pub fn set_post_roll_frames(&mut self, in_post_roll_frames: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_set_post_roll_frames,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_post_roll_frames,
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
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_set_post_roll_frames,
                __buffer,
            )
        };
        std::mem::forget(in_post_roll_frames);
    }
    pub fn set_overlap_priority(&mut self, new_priority: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_set_overlap_priority,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_priority,
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
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_set_overlap_priority,
                __buffer,
            )
        };
        std::mem::forget(new_priority);
    }
    pub fn set_is_locked(&mut self, b_in_is_locked: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_set_is_locked,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_is_locked,
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
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_set_is_locked,
                __buffer,
            )
        };
        std::mem::forget(b_in_is_locked);
    }
    pub fn set_is_active(&mut self, b_in_is_active: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_set_is_active,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_is_active,
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
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_set_is_active,
                __buffer,
            )
        };
        std::mem::forget(b_in_is_active);
    }
    pub fn set_completion_mode(
        &mut self,
        in_completion_mode: EMovieSceneCompletionMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_set_completion_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_completion_mode,
                __buffer.add(0).cast::<EMovieSceneCompletionMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_set_completion_mode,
                __buffer,
            )
        };
        std::mem::forget(in_completion_mode);
    }
    pub fn set_color_tint(
        &mut self,
        in_color_tint: &crate::bindings::core_u_object::FColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_set_color_tint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_color_tint,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_set_color_tint,
                __buffer,
            )
        };
    }
    pub fn set_blend_type(&mut self, in_blend_type: EMovieSceneBlendType) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_set_blend_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_blend_type,
                __buffer.add(0).cast::<EMovieSceneBlendType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_set_blend_type,
                __buffer,
            )
        };
        std::mem::forget(in_blend_type);
    }
    pub fn is_locked(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_is_locked,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_is_locked,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_active(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_is_active,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_is_active,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_row_index(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_get_row_index,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_get_row_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_pre_roll_frames(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_get_pre_roll_frames,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_get_pre_roll_frames,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_post_roll_frames(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_get_post_roll_frames,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_get_post_roll_frames,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_overlap_priority(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_get_overlap_priority,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_get_overlap_priority,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_completion_mode(&self) -> EMovieSceneCompletionMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_get_completion_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_get_completion_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EMovieSceneCompletionMode>().read() }
    }
    pub fn get_color_tint(&self) -> crate::bindings::core_u_object::FColor {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_get_color_tint,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_get_color_tint,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FColor>().read()
        }
    }
    pub fn get_blend_type(&self) -> FOptionalMovieSceneBlendType {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_get_blend_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_section_get_blend_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FOptionalMovieSceneBlendType>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneTrack {
    __padding_end: [u8; 352],
}
impl UMovieSceneTrack {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrack")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrack")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneNameableTrack {
    __padding_end: [u8; 384],
}
impl UMovieSceneNameableTrack {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNameableTrack")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNameableTrack")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneSequence {
    __padding_end: [u8; 128],
}
impl UMovieSceneSequence {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSequence")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSequence")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn get_earliest_timecode_source(&self) -> FMovieSceneTimecodeSource {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_get_earliest_timecode_source,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_get_earliest_timecode_source,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FMovieSceneTimecodeSource>().read() }
    }
    pub fn find_bindings_by_tag(
        &self,
        in_binding_name: FName,
    ) -> TArray<FMovieSceneObjectBindingID> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_find_bindings_by_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_binding_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_find_bindings_by_tag,
                __buffer,
            )
        };
        std::mem::forget(in_binding_name);
        unsafe { __buffer.add(16).cast::<TArray<FMovieSceneObjectBindingID>>().read() }
    }
    pub fn find_binding_by_tag(
        &self,
        in_binding_name: FName,
    ) -> FMovieSceneObjectBindingID {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_find_binding_by_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_binding_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_find_binding_by_tag,
                __buffer,
            )
        };
        std::mem::forget(in_binding_name);
        unsafe { __buffer.add(12).cast::<FMovieSceneObjectBindingID>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneCustomBinding {
    __padding_end: [u8; 48],
}
impl UMovieSceneCustomBinding {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCustomBinding")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCustomBinding")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn get_base_engine_priority() -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_custom_binding_get_base_engine_priority,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::movie_scene::UMovieSceneCustomBinding::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_custom_binding_get_base_engine_priority,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_base_custom_priority() -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_custom_binding_get_base_custom_priority,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::movie_scene::UMovieSceneCustomBinding::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_custom_binding_get_base_custom_priority,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneReplaceableBindingBase {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub preview_spawnable: UPtr<UMovieSceneSpawnableBindingBase>,
}
impl UMovieSceneReplaceableBindingBase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneReplaceableBindingBase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneReplaceableBindingBase")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneSpawnableBindingBase {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub spawn_ownership: ESpawnOwnership,
    pub b_continuously_respawn: bool,
}
impl UMovieSceneSpawnableBindingBase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSpawnableBindingBase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSpawnableBindingBase")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneChannelOverrideContainer {
    __padding_end: [u8; 112],
}
impl UMovieSceneChannelOverrideContainer {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneChannelOverrideContainer")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneChannelOverrideContainer")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneCondition {
    #[doc(hidden)]
    pub(crate) __padding_112: [u8; 112],
    pub b_editor_force_true: bool,
    pub b_invert: bool,
}
impl UMovieSceneCondition {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCondition")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCondition")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn bp_get_scope(&self) -> EMovieSceneConditionScope {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_condition_bp_get_scope,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_condition_bp_get_scope,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EMovieSceneConditionScope>().read() }
    }
    pub fn bp_get_check_frequency(&self) -> EMovieSceneConditionCheckFrequency {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_condition_bp_get_check_frequency,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_condition_bp_get_check_frequency,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EMovieSceneConditionCheckFrequency>().read() }
    }
    pub fn bp_evaluate_condition(
        &self,
        condition_context: &FMovieSceneConditionContext,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_condition_bp_evaluate_condition,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                condition_context,
                __buffer.add(0).cast::<FMovieSceneConditionContext>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_condition_bp_evaluate_condition,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneEntityInstantiatorSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneEntityInstantiatorSystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEntityInstantiatorSystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEntityInstantiatorSystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneSubSection {
    #[doc(hidden)]
    pub(crate) __padding_368: [u8; 368],
    pub parameters: FMovieSceneSectionParameters,
    __padding_end: [u8; 2000],
}
impl UMovieSceneSubSection {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSubSection")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSubSection")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn set_sequence(&mut self, sequence: UPtr<UMovieSceneSequence>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sub_section_set_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer.add(0).cast::<UPtr<UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sub_section_set_sequence,
                __buffer,
            )
        };
        std::mem::forget(sequence);
    }
    pub fn get_sequence(&self) -> UPtr<UMovieSceneSequence> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sub_section_get_sequence,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sub_section_get_sequence,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UMovieSceneSequence>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneBoolSection {
    __padding_end: [u8; 664],
}
impl UMovieSceneBoolSection {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBoolSection")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBoolSection")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneBlenderSystem {
    __padding_end: [u8; 120],
}
impl UMovieSceneBlenderSystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBlenderSystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBlenderSystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneTrackInstance {
    __padding_end: [u8; 88],
}
impl UMovieSceneTrackInstance {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrackInstance")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrackInstance")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneSubTrack {
    __padding_end: [u8; 416],
}
impl UMovieSceneSubTrack {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSubTrack")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSubTrack")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneBlenderSystemSupport {}
#[repr(C, align(8))]
pub struct UMovieSceneBlenderSystemSupport {
    __padding_end: [u8; 48],
}
impl UMovieSceneBlenderSystemSupport {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBlenderSystemSupport")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBlenderSystemSupport")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneBoundObjectProxy {}
#[repr(C, align(8))]
pub struct UMovieSceneBoundObjectProxy {
    __padding_end: [u8; 48],
}
impl UMovieSceneBoundObjectProxy {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBoundObjectProxy")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBoundObjectProxy")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn bp_get_bound_object_for_sequencer(
        &mut self,
        resolved_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_bound_object_proxy_bp_get_bound_object_for_sequencer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &resolved_object,
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
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_bound_object_proxy_bp_get_bound_object_for_sequencer,
                __buffer,
            )
        };
        std::mem::forget(resolved_object);
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
}
pub struct IMovieSceneChannelDecoration {}
#[repr(C, align(8))]
pub struct UMovieSceneChannelDecoration {
    __padding_end: [u8; 48],
}
impl UMovieSceneChannelDecoration {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneChannelDecoration")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneChannelDecoration")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneChannelOverrideProvider {}
#[repr(C, align(8))]
pub struct UMovieSceneChannelOverrideProvider {
    __padding_end: [u8; 48],
}
impl UMovieSceneChannelOverrideProvider {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneChannelOverrideProvider")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneChannelOverrideProvider")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneChannelOwner {}
#[repr(C, align(8))]
pub struct UMovieSceneChannelOwner {
    __padding_end: [u8; 48],
}
impl UMovieSceneChannelOwner {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneChannelOwner")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneChannelOwner")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneDecoration {}
#[repr(C, align(8))]
pub struct UMovieSceneDecoration {
    __padding_end: [u8; 48],
}
impl UMovieSceneDecoration {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneDecoration")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneDecoration")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneDeterminismSource {}
#[repr(C, align(8))]
pub struct UMovieSceneDeterminismSource {
    __padding_end: [u8; 48],
}
impl UMovieSceneDeterminismSource {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneDeterminismSource")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneDeterminismSource")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneEntityDecorator {}
#[repr(C, align(8))]
pub struct UMovieSceneEntityDecorator {
    __padding_end: [u8; 48],
}
impl UMovieSceneEntityDecorator {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEntityDecorator")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEntityDecorator")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneLifetimeDecoration {}
#[repr(C, align(8))]
pub struct UMovieSceneLifetimeDecoration {
    __padding_end: [u8; 48],
}
impl UMovieSceneLifetimeDecoration {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneLifetimeDecoration")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneLifetimeDecoration")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneMetaDataInterface {}
#[repr(C, align(8))]
pub struct UMovieSceneMetaDataInterface {
    __padding_end: [u8; 48],
}
impl UMovieSceneMetaDataInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneMetaDataInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneMetaDataInterface")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieScenePlaybackClient {}
#[repr(C, align(8))]
pub struct UMovieScenePlaybackClient {
    __padding_end: [u8; 48],
}
impl UMovieScenePlaybackClient {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieScenePlaybackClient")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieScenePlaybackClient")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneSectionDecoration {}
#[repr(C, align(8))]
pub struct UMovieSceneSectionDecoration {
    __padding_end: [u8; 48],
}
impl UMovieSceneSectionDecoration {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSectionDecoration")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSectionDecoration")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneSequencePlayerObserver {}
#[repr(C, align(8))]
pub struct UMovieSceneSequencePlayerObserver {
    __padding_end: [u8; 48],
}
impl UMovieSceneSequencePlayerObserver {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSequencePlayerObserver")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSequencePlayerObserver")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneTrackDecoration {}
#[repr(C, align(8))]
pub struct UMovieSceneTrackDecoration {
    __padding_end: [u8; 48],
}
impl UMovieSceneTrackDecoration {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrackDecoration")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrackDecoration")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneBindingEventReceiverInterface {}
#[repr(C, align(8))]
pub struct UMovieSceneBindingEventReceiverInterface {
    __padding_end: [u8; 48],
}
impl UMovieSceneBindingEventReceiverInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBindingEventReceiverInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBindingEventReceiverInterface")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn on_object_unbound_by_sequencer(
        &mut self,
        player: UPtr<UMovieSceneSequencePlayer>,
        binding_id: FMovieSceneObjectBindingID,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_binding_event_receiver_interface_on_object_unbound_by_sequencer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player,
                __buffer.add(0).cast::<UPtr<UMovieSceneSequencePlayer>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &binding_id,
                __buffer.add(8).cast::<FMovieSceneObjectBindingID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_binding_event_receiver_interface_on_object_unbound_by_sequencer,
                __buffer,
            )
        };
        std::mem::forget(player);
        std::mem::forget(binding_id);
    }
    pub fn on_object_bound_by_sequencer(
        &mut self,
        player: UPtr<UMovieSceneSequencePlayer>,
        binding_id: FMovieSceneObjectBindingID,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_binding_event_receiver_interface_on_object_bound_by_sequencer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player,
                __buffer.add(0).cast::<UPtr<UMovieSceneSequencePlayer>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &binding_id,
                __buffer.add(8).cast::<FMovieSceneObjectBindingID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_binding_event_receiver_interface_on_object_bound_by_sequencer,
                __buffer,
            )
        };
        std::mem::forget(player);
        std::mem::forget(binding_id);
    }
}
pub struct IMovieSceneBindingOwnerInterface {}
#[repr(C, align(8))]
pub struct UMovieSceneBindingOwnerInterface {
    __padding_end: [u8; 48],
}
impl UMovieSceneBindingOwnerInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBindingOwnerInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBindingOwnerInterface")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneCachedTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneCachedTrack {
    __padding_end: [u8; 48],
}
impl UMovieSceneCachedTrack {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCachedTrack")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCachedTrack")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneEasingFunction {}
#[repr(C, align(8))]
pub struct UMovieSceneEasingFunction {
    __padding_end: [u8; 48],
}
impl UMovieSceneEasingFunction {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEasingFunction")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEasingFunction")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn on_evaluate(&self, interp: f32) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_easing_function_on_evaluate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&interp, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_easing_function_on_evaluate,
                __buffer,
            )
        };
        std::mem::forget(interp);
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
}
pub struct IMovieSceneKeyProxy {}
#[repr(C, align(8))]
pub struct UMovieSceneKeyProxy {
    __padding_end: [u8; 48],
}
impl UMovieSceneKeyProxy {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneKeyProxy")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneKeyProxy")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneNumericVariantGetter {
    __padding_end: [u8; 120],
}
impl UMovieSceneNumericVariantGetter {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNumericVariantGetter")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNumericVariantGetter")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneSequenceTickManagerClient {}
#[repr(C, align(8))]
pub struct UMovieSceneSequenceTickManagerClient {
    __padding_end: [u8; 48],
}
impl UMovieSceneSequenceTickManagerClient {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSequenceTickManagerClient")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSequenceTickManagerClient")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneSectionChannelOverrideRegistry {
    __padding_end: [u8; 128],
}
impl UMovieSceneSectionChannelOverrideRegistry {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSectionChannelOverrideRegistry")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSectionChannelOverrideRegistry")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneTrackTemplateProducer {}
#[repr(C, align(8))]
pub struct UMovieSceneTrackTemplateProducer {
    __padding_end: [u8; 48],
}
impl UMovieSceneTrackTemplateProducer {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrackTemplateProducer")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrackTemplateProducer")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneCompiledData {
    __padding_end: [u8; 1080],
}
impl UMovieSceneCompiledData {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCompiledData")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCompiledData")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneCompiledDataManager {
    __padding_end: [u8; 568],
}
impl UMovieSceneCompiledDataManager {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCompiledDataManager")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCompiledDataManager")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneGroupCondition {
    #[doc(hidden)]
    pub(crate) __padding_120: [u8; 120],
    pub operator: EMovieSceneGroupConditionOperator,
    pub sub_conditions: TArray<FMovieSceneConditionContainer>,
}
impl UMovieSceneGroupCondition {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneGroupCondition")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneGroupCondition")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneLanguagePreviewDecoration {
    __padding_end: [u8; 64],
}
impl UMovieSceneLanguagePreviewDecoration {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneLanguagePreviewDecoration")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneLanguagePreviewDecoration")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneMuteSoloDecoration {
    __padding_end: [u8; 64],
}
impl UMovieSceneMuteSoloDecoration {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneMuteSoloDecoration")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneMuteSoloDecoration")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneScalingDriver {}
#[repr(C, align(8))]
pub struct UMovieSceneScalingDriver {
    __padding_end: [u8; 48],
}
impl UMovieSceneScalingDriver {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScalingDriver")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScalingDriver")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneTimeWarpGetter {
    __padding_end: [u8; 136],
}
impl UMovieSceneTimeWarpGetter {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTimeWarpGetter")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTimeWarpGetter")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieScenePlayRateCurve {
    __padding_end: [u8; 496],
}
impl UMovieScenePlayRateCurve {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieScenePlayRateCurve")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieScenePlayRateCurve")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScalingAnchors {
    __padding_end: [u8; 784],
}
impl UMovieSceneScalingAnchors {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScalingAnchors")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScalingAnchors")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneSectionAnchorsDecoration {
    __padding_end: [u8; 88],
}
impl UMovieSceneSectionAnchorsDecoration {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSectionAnchorsDecoration")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSectionAnchorsDecoration")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneTimeWarpSource {}
#[repr(C, align(8))]
pub struct UMovieSceneTimeWarpSource {
    __padding_end: [u8; 48],
}
impl UMovieSceneTimeWarpSource {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTimeWarpSource")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTimeWarpSource")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneTimeWarpDecoration {
    __padding_end: [u8; 72],
}
impl UMovieSceneTimeWarpDecoration {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTimeWarpDecoration")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTimeWarpDecoration")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneTrackRowDecoration {
    __padding_end: [u8; 136],
}
impl UMovieSceneTrackRowDecoration {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrackRowDecoration")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrackRowDecoration")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneEntityProvider {}
#[repr(C, align(8))]
pub struct UMovieSceneEntityProvider {
    __padding_end: [u8; 48],
}
impl UMovieSceneEntityProvider {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEntityProvider")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEntityProvider")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneBindingLifetimeSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneBindingLifetimeSystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBindingLifetimeSystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBindingLifetimeSystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneGenericBoundObjectInstantiator {
    __padding_end: [u8; 80],
}
impl UMovieSceneGenericBoundObjectInstantiator {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneGenericBoundObjectInstantiator")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneGenericBoundObjectInstantiator")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneBoundSceneComponentInstantiator {
    __padding_end: [u8; 80],
}
impl UMovieSceneBoundSceneComponentInstantiator {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBoundSceneComponentInstantiator")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBoundSceneComponentInstantiator")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneValueDecomposer {}
#[repr(C, align(8))]
pub struct UMovieSceneValueDecomposer {
    __padding_end: [u8; 48],
}
impl UMovieSceneValueDecomposer {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneValueDecomposer")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneValueDecomposer")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneEntityGroupingSystem {
    __padding_end: [u8; 360],
}
impl UMovieSceneEntityGroupingSystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEntityGroupingSystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEntityGroupingSystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneEntitySystemLinker {
    __padding_end: [u8; 1952],
}
impl UMovieSceneEntitySystemLinker {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEntitySystemLinker")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEntitySystemLinker")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneEvalTimeSystem {
    __padding_end: [u8; 480],
}
impl UMovieSceneEvalTimeSystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEvalTimeSystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEvalTimeSystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneEvaluationHookSystem {
    __padding_end: [u8; 160],
}
impl UMovieSceneEvaluationHookSystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEvaluationHookSystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEvaluationHookSystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneInitialValueSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneInitialValueSystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneInitialValueSystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneInitialValueSystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieScenePreAnimatedStateSystemInterface {}
#[repr(C, align(8))]
pub struct UMovieScenePreAnimatedStateSystemInterface {
    __padding_end: [u8; 48],
}
impl UMovieScenePreAnimatedStateSystemInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieScenePreAnimatedStateSystemInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieScenePreAnimatedStateSystemInterface")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneCachePreAnimatedStateSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneCachePreAnimatedStateSystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCachePreAnimatedStateSystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCachePreAnimatedStateSystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneRestorePreAnimatedStateSystem {
    __padding_end: [u8; 96],
}
impl UMovieSceneRestorePreAnimatedStateSystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneRestorePreAnimatedStateSystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneRestorePreAnimatedStateSystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneRootInstantiatorSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneRootInstantiatorSystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneRootInstantiatorSystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneRootInstantiatorSystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneSpawnablesSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneSpawnablesSystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSpawnablesSystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSpawnablesSystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneTrackInstanceInstantiator {
    __padding_end: [u8; 256],
}
impl UMovieSceneTrackInstanceInstantiator {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrackInstanceInstantiator")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrackInstanceInstantiator")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneTrackInstanceSystem {
    __padding_end: [u8; 88],
}
impl UMovieSceneTrackInstanceSystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrackInstanceSystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrackInstanceSystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneCustomClockSource {}
#[repr(C, align(8))]
pub struct UMovieSceneCustomClockSource {
    __padding_end: [u8; 48],
}
impl UMovieSceneCustomClockSource {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCustomClockSource")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCustomClockSource")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct IMovieSceneEvaluationHook {}
#[repr(C, align(8))]
pub struct UMovieSceneEvaluationHook {
    __padding_end: [u8; 48],
}
impl UMovieSceneEvaluationHook {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEvaluationHook")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEvaluationHook")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneBuiltInEasingFunction {
    __padding_end: [u8; 64],
}
impl UMovieSceneBuiltInEasingFunction {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBuiltInEasingFunction")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBuiltInEasingFunction")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneEasingExternalCurve {
    __padding_end: [u8; 64],
}
impl UMovieSceneEasingExternalCurve {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEasingExternalCurve")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEasingExternalCurve")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
pub struct INodeAndChannelMappings {}
#[repr(C, align(8))]
pub struct UNodeAndChannelMappings {
    __padding_end: [u8; 48],
}
impl UNodeAndChannelMappings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNodeAndChannelMappings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNodeAndChannelMappings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneShotMetaData {
    __padding_end: [u8; 72],
}
impl UMovieSceneShotMetaData {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneShotMetaData")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneShotMetaData")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneNodeGroup {
    __padding_end: [u8; 120],
}
impl UMovieSceneNodeGroup {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNodeGroup")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNodeGroup")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneNodeGroupCollection {
    __padding_end: [u8; 104],
}
impl UMovieSceneNodeGroupCollection {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNodeGroupCollection")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNodeGroupCollection")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieScene {
    __padding_end: [u8; 1304],
}
impl UMovieScene {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieScene")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieScene")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneBindingOverrides {
    __padding_end: [u8; 152],
}
impl UMovieSceneBindingOverrides {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBindingOverrides")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBindingOverrides")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneClock {
    __padding_end: [u8; 112],
}
impl UMovieSceneClock {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneClock")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneClock")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneExternalClock {
    __padding_end: [u8; 152],
}
impl UMovieSceneExternalClock {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneExternalClock")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneExternalClock")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBuiltInDynamicBindingResolverLibrary {
    __padding_end: [u8; 48],
}
impl UBuiltInDynamicBindingResolverLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBuiltInDynamicBindingResolverLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBuiltInDynamicBindingResolverLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn resolve_to_player_pawn(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller_index: i32,
    ) -> FMovieSceneDynamicBindingResolveResult {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_built_in_dynamic_binding_resolver_library_resolve_to_player_pawn,
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
                &player_controller_index,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::movie_scene::UBuiltInDynamicBindingResolverLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_built_in_dynamic_binding_resolver_library_resolve_to_player_pawn,
                __buffer,
            )
        };
        std::mem::forget(world_context_object);
        std::mem::forget(player_controller_index);
        unsafe {
            __buffer.add(16).cast::<FMovieSceneDynamicBindingResolveResult>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneFolder {
    __padding_end: [u8; 248],
}
impl UMovieSceneFolder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneFolder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneFolder")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneMetaData {
    __padding_end: [u8; 96],
}
impl UMovieSceneMetaData {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneMetaData")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneMetaData")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn set_notes(&mut self, in_notes: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_meta_data_set_notes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_notes,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_meta_data_set_notes,
                __buffer,
            )
        };
        std::mem::forget(in_notes);
    }
    pub fn set_created(
        &mut self,
        in_created: crate::bindings::core_u_object::FDateTime,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_meta_data_set_created,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_created,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FDateTime>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_meta_data_set_created,
                __buffer,
            )
        };
        std::mem::forget(in_created);
    }
    pub fn set_author(&mut self, in_author: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_meta_data_set_author,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_author,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_meta_data_set_author,
                __buffer,
            )
        };
        std::mem::forget(in_author);
    }
    pub fn get_notes(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_meta_data_get_notes,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_meta_data_get_notes,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_created(&self) -> crate::bindings::core_u_object::FDateTime {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_meta_data_get_created,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_meta_data_get_created,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FDateTime>().read()
        }
    }
    pub fn get_author(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_meta_data_get_author,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_meta_data_get_author,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneSequencePlayer {
    __padding_end: [u8; 1216],
}
impl UMovieSceneSequencePlayer {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSequencePlayer")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSequencePlayer")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn stop_at_current_time(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_stop_at_current_time,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_stop_at_current_time,
                __buffer,
            )
        };
    }
    pub fn stop(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_stop,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_stop,
                __buffer,
            )
        };
    }
    pub fn set_weight(&mut self, in_weight: f64) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_weight,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_weight, __buffer.add(0).cast::<f64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_weight,
                __buffer,
            )
        };
        std::mem::forget(in_weight);
    }
    pub fn set_time_range(&mut self, start_time: f32, duration: f32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_time_range,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&start_time, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(4).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_time_range,
                __buffer,
            )
        };
        std::mem::forget(start_time);
        std::mem::forget(duration);
    }
    pub fn set_play_rate(&mut self, play_rate: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_play_rate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&play_rate, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_play_rate,
                __buffer,
            )
        };
        std::mem::forget(play_rate);
    }
    pub fn set_playback_position(
        &mut self,
        playback_params: FMovieSceneSequencePlaybackParams,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_playback_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &playback_params,
                __buffer.add(0).cast::<FMovieSceneSequencePlaybackParams>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_playback_position,
                __buffer,
            )
        };
        std::mem::forget(playback_params);
    }
    pub fn set_hide_hud(&mut self, hide_hud: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_hide_hud,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&hide_hud, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_hide_hud,
                __buffer,
            )
        };
        std::mem::forget(hide_hud);
    }
    pub fn set_frame_rate(
        &mut self,
        frame_rate: crate::bindings::core_u_object::FFrameRate,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_frame_rate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_rate,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameRate>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_frame_rate,
                __buffer,
            )
        };
        std::mem::forget(frame_rate);
    }
    pub fn set_frame_range(&mut self, start_frame: i32, duration: i32, sub_frames: f32) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_frame_range,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_frame,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frames, __buffer.add(8).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_frame_range,
                __buffer,
            )
        };
        std::mem::forget(start_frame);
        std::mem::forget(duration);
        std::mem::forget(sub_frames);
    }
    pub fn set_disable_camera_cuts(&mut self, b_in_disable_camera_cuts: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_disable_camera_cuts,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_disable_camera_cuts,
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
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_disable_camera_cuts,
                __buffer,
            )
        };
        std::mem::forget(b_in_disable_camera_cuts);
    }
    pub fn set_completion_mode_override(
        &mut self,
        completion_mode_override: EMovieSceneCompletionModeOverride,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_completion_mode_override,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &completion_mode_override,
                __buffer.add(0).cast::<EMovieSceneCompletionModeOverride>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_set_completion_mode_override,
                __buffer,
            )
        };
        std::mem::forget(completion_mode_override);
    }
    pub fn scrub(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_scrub,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_scrub,
                __buffer,
            )
        };
    }
    pub fn restore_state(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_restore_state,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_restore_state,
                __buffer,
            )
        };
    }
    pub fn request_invalidate_binding(
        &mut self,
        object_binding: FMovieSceneObjectBindingID,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_request_invalidate_binding,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_binding,
                __buffer.add(0).cast::<FMovieSceneObjectBindingID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_request_invalidate_binding,
                __buffer,
            )
        };
        std::mem::forget(object_binding);
    }
    pub fn remove_weight(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_remove_weight,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_remove_weight,
                __buffer,
            )
        };
    }
    pub fn play_to(
        &mut self,
        playback_params: FMovieSceneSequencePlaybackParams,
        play_to_params: FMovieSceneSequencePlayToParams,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_play_to,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &playback_params,
                __buffer.add(0).cast::<FMovieSceneSequencePlaybackParams>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &play_to_params,
                __buffer.add(64).cast::<FMovieSceneSequencePlayToParams>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_play_to,
                __buffer,
            )
        };
        std::mem::forget(playback_params);
        std::mem::forget(play_to_params);
    }
    pub fn play_reverse(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_play_reverse,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_play_reverse,
                __buffer,
            )
        };
    }
    pub fn play_looping(&mut self, num_loops: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_play_looping,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&num_loops, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_play_looping,
                __buffer,
            )
        };
        std::mem::forget(num_loops);
    }
    pub fn play(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_play,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_play,
                __buffer,
            )
        };
    }
    pub fn pause(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_pause,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_pause,
                __buffer,
            )
        };
    }
    pub fn is_reversed(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_is_reversed,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_is_reversed,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_playing(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_is_playing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_is_playing,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_paused(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_is_paused,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_is_paused,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn go_to_end_and_stop(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_go_to_end_and_stop,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_go_to_end_and_stop,
                __buffer,
            )
        };
    }
    pub fn get_start_time(&self) -> crate::bindings::core_u_object::FQualifiedFrameTime {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_start_time,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_start_time,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FQualifiedFrameTime>()
                .read()
        }
    }
    pub fn get_sequence_name(&self, b_add_client_info: bool) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_sequence_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_add_client_info,
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
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_sequence_name,
                __buffer,
            )
        };
        std::mem::forget(b_add_client_info);
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn get_sequence(&self) -> UPtr<UMovieSceneSequence> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_sequence,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_sequence,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UMovieSceneSequence>>().read() }
    }
    pub fn get_play_rate(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_play_rate,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_play_rate,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_object_bindings(
        &mut self,
        in_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> TArray<FMovieSceneObjectBindingID> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_object_bindings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_object,
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
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_object_bindings,
                __buffer,
            )
        };
        std::mem::forget(in_object);
        unsafe { __buffer.add(8).cast::<TArray<FMovieSceneObjectBindingID>>().read() }
    }
    pub fn get_hide_hud(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_hide_hud,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_hide_hud,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_frame_rate(&self) -> crate::bindings::core_u_object::FFrameRate {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_frame_rate,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_frame_rate,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameRate>().read()
        }
    }
    pub fn get_frame_duration(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_frame_duration,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_frame_duration,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_end_time(&self) -> crate::bindings::core_u_object::FQualifiedFrameTime {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_end_time,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_end_time,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FQualifiedFrameTime>()
                .read()
        }
    }
    pub fn get_duration(&self) -> crate::bindings::core_u_object::FQualifiedFrameTime {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_duration,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_duration,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FQualifiedFrameTime>()
                .read()
        }
    }
    pub fn get_disable_camera_cuts(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_disable_camera_cuts,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_disable_camera_cuts,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_current_time(
        &self,
    ) -> crate::bindings::core_u_object::FQualifiedFrameTime {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_current_time,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_current_time,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FQualifiedFrameTime>()
                .read()
        }
    }
    pub fn get_completion_mode_override(&self) -> EMovieSceneCompletionModeOverride {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_completion_mode_override,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_completion_mode_override,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EMovieSceneCompletionModeOverride>().read() }
    }
    pub fn get_bound_objects(
        &mut self,
        object_binding: FMovieSceneObjectBindingID,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_bound_objects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_binding,
                __buffer.add(0).cast::<FMovieSceneObjectBindingID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_get_bound_objects,
                __buffer,
            )
        };
        std::mem::forget(object_binding);
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
    pub fn change_playback_direction(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_change_playback_direction,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .u_movie_scene_sequence_player_change_playback_direction,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneSequenceTickManager {
    __padding_end: [u8; 160],
}
impl UMovieSceneSequenceTickManager {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSequenceTickManager")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSequenceTickManager")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneBindingLifetimeSection {
    __padding_end: [u8; 368],
}
impl UMovieSceneBindingLifetimeSection {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBindingLifetimeSection")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBindingLifetimeSection")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneHookSection {
    __padding_end: [u8; 384],
}
impl UMovieSceneHookSection {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneHookSection")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneHookSection")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneSpawnSection {
    __padding_end: [u8; 672],
}
impl UMovieSceneSpawnSection {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSpawnSection")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSpawnSection")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneTimeWarpSection {
    __padding_end: [u8; 376],
}
impl UMovieSceneTimeWarpSection {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTimeWarpSection")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTimeWarpSection")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTestMovieSceneTrack {
    __padding_end: [u8; 384],
}
impl UTestMovieSceneTrack {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneTrack")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneTrack")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTestMovieSceneSection {
    __padding_end: [u8; 360],
}
impl UTestMovieSceneSection {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneSection")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneSection")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTestMovieSceneSequence {
    __padding_end: [u8; 136],
}
impl UTestMovieSceneSequence {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneSequence")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneSequence")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTestMovieSceneSubTrack {
    __padding_end: [u8; 432],
}
impl UTestMovieSceneSubTrack {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneSubTrack")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneSubTrack")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTestMovieSceneSubSection {
    __padding_end: [u8; 2424],
}
impl UTestMovieSceneSubSection {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneSubSection")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneSubSection")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTestMovieSceneEvalHookTrack {
    __padding_end: [u8; 368],
}
impl UTestMovieSceneEvalHookTrack {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneEvalHookTrack")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneEvalHookTrack")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTestMovieSceneEvalHookSection {
    __padding_end: [u8; 408],
}
impl UTestMovieSceneEvalHookSection {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneEvalHookSection")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneEvalHookSection")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTestMovieSceneObject {
    __padding_end: [u8; 48],
}
impl UTestMovieSceneObject {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneObject")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneObject")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ATestMovieSceneArrayPropertiesActor {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub test_bool: bool,
    pub test_enum: ETestMovieSceneEnum,
    pub test_int32: i32,
    pub test_object: UPtr<UTestMovieSceneObject>,
    pub test_vector: crate::bindings::core_u_object::FVector,
    pub multiple_floats: TArray<f32>,
    pub single_struct: FTestMovieSceneStruct,
    pub multiple_structs: TArray<FTestMovieSceneStruct>,
    pub single_instanced_struct: crate::bindings::core_u_object::FInstancedStruct,
    pub multiple_instanced_structs: TArray<
        crate::bindings::core_u_object::FInstancedStruct,
    >,
    pub test_setter_float: f32,
}
impl ATestMovieSceneArrayPropertiesActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ATestMovieSceneArrayPropertiesActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ATestMovieSceneArrayPropertiesActor")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn set_test_setter_float(&mut self, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .a_test_movie_scene_array_properties_actor_set_test_setter_float,
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
                crate::bindings::movie_scene::__FUNCTION_PTRS
                    .a_test_movie_scene_array_properties_actor_set_test_setter_float,
                __buffer,
            )
        };
        std::mem::forget(in_value);
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneBindingLifetimeTrack {
    __padding_end: [u8; 376],
}
impl UMovieSceneBindingLifetimeTrack {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBindingLifetimeTrack")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBindingLifetimeTrack")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneSpawnTrack {
    __padding_end: [u8; 384],
}
impl UMovieSceneSpawnTrack {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSpawnTrack")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSpawnTrack")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneTimeWarpTrack {
    __padding_end: [u8; 384],
}
impl UMovieSceneTimeWarpTrack {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTimeWarpTrack")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTimeWarpTrack")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneTimeWarpCurve {
    __padding_end: [u8; 464],
}
impl UMovieSceneTimeWarpCurve {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTimeWarpCurve")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTimeWarpCurve")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct FMovieSceneSequencePlayer_OnPlay {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMovieSceneSequencePlayer_OnPlayReverse {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMovieSceneSequencePlayer_OnStop {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMovieSceneSequencePlayer_OnPause {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMovieSceneSequencePlayer_OnFinished {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EMovieSceneCompletionMode(pub u8);
impl EMovieSceneCompletionMode {
    pub const KEEP_STATE: EMovieSceneCompletionMode = EMovieSceneCompletionMode(0);
    pub const RESTORE_STATE: EMovieSceneCompletionMode = EMovieSceneCompletionMode(1);
    pub const PROJECT_DEFAULT: EMovieSceneCompletionMode = EMovieSceneCompletionMode(2);
}
#[repr(transparent)]
pub struct EMovieSceneObjectBindingSpace(pub u8);
impl EMovieSceneObjectBindingSpace {
    pub const LOCAL: EMovieSceneObjectBindingSpace = EMovieSceneObjectBindingSpace(0);
    pub const ROOT: EMovieSceneObjectBindingSpace = EMovieSceneObjectBindingSpace(1);
    pub const UNUSED: EMovieSceneObjectBindingSpace = EMovieSceneObjectBindingSpace(2);
}
#[repr(transparent)]
pub struct EMovieSceneSubSectionFlags(pub u8);
impl EMovieSceneSubSectionFlags {
    pub const NONE: EMovieSceneSubSectionFlags = EMovieSceneSubSectionFlags(0);
    pub const OVERRIDE_KEEP_STATE: EMovieSceneSubSectionFlags = EMovieSceneSubSectionFlags(
        1,
    );
    pub const OVERRIDE_RESTORE_STATE: EMovieSceneSubSectionFlags = EMovieSceneSubSectionFlags(
        2,
    );
    pub const IGNORE_HIERARCHICAL_BIAS: EMovieSceneSubSectionFlags = EMovieSceneSubSectionFlags(
        4,
    );
    pub const BLEND_HIERARCHICAL_BIAS: EMovieSceneSubSectionFlags = EMovieSceneSubSectionFlags(
        8,
    );
    pub const ANY_RESTORE_STATE_OVERRIDE: EMovieSceneSubSectionFlags = EMovieSceneSubSectionFlags(
        3,
    );
}
#[repr(transparent)]
pub struct ESectionEvaluationFlags(pub u8);
impl ESectionEvaluationFlags {
    pub const NONE: ESectionEvaluationFlags = ESectionEvaluationFlags(0);
    pub const PRE_ROLL: ESectionEvaluationFlags = ESectionEvaluationFlags(1);
    pub const POST_ROLL: ESectionEvaluationFlags = ESectionEvaluationFlags(2);
    pub const FORCE_KEEP_STATE: ESectionEvaluationFlags = ESectionEvaluationFlags(4);
    pub const FORCE_RESTORE_STATE: ESectionEvaluationFlags = ESectionEvaluationFlags(8);
}
#[repr(transparent)]
pub struct EMovieSceneCompletionModeOverride(pub u8);
impl EMovieSceneCompletionModeOverride {
    pub const NONE: EMovieSceneCompletionModeOverride = EMovieSceneCompletionModeOverride(
        0,
    );
    pub const FORCE_KEEP_STATE: EMovieSceneCompletionModeOverride = EMovieSceneCompletionModeOverride(
        1,
    );
    pub const FORCE_RESTORE_STATE: EMovieSceneCompletionModeOverride = EMovieSceneCompletionModeOverride(
        2,
    );
}
#[repr(transparent)]
pub struct ENavigationToolItemFlags(pub u8);
impl ENavigationToolItemFlags {
    pub const NONE: ENavigationToolItemFlags = ENavigationToolItemFlags(0);
    pub const IGNORE_PENDING_KILL: ENavigationToolItemFlags = ENavigationToolItemFlags(
        1,
    );
    pub const PENDING_REMOVAL: ENavigationToolItemFlags = ENavigationToolItemFlags(2);
    pub const EXPANDED: ENavigationToolItemFlags = ENavigationToolItemFlags(4);
}
#[repr(transparent)]
pub struct EMovieSceneBlendType(pub u8);
impl EMovieSceneBlendType {
    pub const INVALID: EMovieSceneBlendType = EMovieSceneBlendType(0);
    pub const ABSOLUTE: EMovieSceneBlendType = EMovieSceneBlendType(1);
    pub const ADDITIVE: EMovieSceneBlendType = EMovieSceneBlendType(2);
    pub const RELATIVE: EMovieSceneBlendType = EMovieSceneBlendType(4);
    pub const ADDITIVE_FROM_BASE: EMovieSceneBlendType = EMovieSceneBlendType(8);
    pub const OVERRIDE: EMovieSceneBlendType = EMovieSceneBlendType(16);
}
#[repr(transparent)]
pub struct EEvaluationMethod(pub u8);
impl EEvaluationMethod {
    pub const STATIC: EEvaluationMethod = EEvaluationMethod(0);
    pub const SWEPT: EEvaluationMethod = EEvaluationMethod(1);
}
#[repr(transparent)]
pub struct EMovieSceneBreadcrumbMode(pub u8);
impl EMovieSceneBreadcrumbMode {
    pub const SPARSE: EMovieSceneBreadcrumbMode = EMovieSceneBreadcrumbMode(0);
    pub const DENSE: EMovieSceneBreadcrumbMode = EMovieSceneBreadcrumbMode(1);
}
#[repr(transparent)]
pub struct EMovieSceneServerClientMask(pub u8);
impl EMovieSceneServerClientMask {
    pub const NONE: EMovieSceneServerClientMask = EMovieSceneServerClientMask(0);
    pub const SERVER: EMovieSceneServerClientMask = EMovieSceneServerClientMask(1);
    pub const CLIENT: EMovieSceneServerClientMask = EMovieSceneServerClientMask(2);
    pub const ALL: EMovieSceneServerClientMask = EMovieSceneServerClientMask(3);
}
#[repr(transparent)]
pub struct EMovieScenePlayerStatus(pub u8);
impl EMovieScenePlayerStatus {
    pub const STOPPED: EMovieScenePlayerStatus = EMovieScenePlayerStatus(0);
    pub const PLAYING: EMovieScenePlayerStatus = EMovieScenePlayerStatus(1);
    pub const SCRUBBING: EMovieScenePlayerStatus = EMovieScenePlayerStatus(2);
    pub const JUMPING: EMovieScenePlayerStatus = EMovieScenePlayerStatus(3);
    pub const STEPPING: EMovieScenePlayerStatus = EMovieScenePlayerStatus(4);
    pub const PAUSED: EMovieScenePlayerStatus = EMovieScenePlayerStatus(5);
    pub const MAX: EMovieScenePlayerStatus = EMovieScenePlayerStatus(6);
}
#[repr(transparent)]
pub struct EMovieScenePositionType(pub u8);
impl EMovieScenePositionType {
    pub const FRAME: EMovieScenePositionType = EMovieScenePositionType(0);
    pub const TIME: EMovieScenePositionType = EMovieScenePositionType(1);
    pub const MARKED_FRAME: EMovieScenePositionType = EMovieScenePositionType(2);
    pub const TIMECODE: EMovieScenePositionType = EMovieScenePositionType(3);
}
#[repr(transparent)]
pub struct EUpdatePositionMethod(pub u8);
impl EUpdatePositionMethod {
    pub const PLAY: EUpdatePositionMethod = EUpdatePositionMethod(0);
    pub const JUMP: EUpdatePositionMethod = EUpdatePositionMethod(1);
    pub const SCRUB: EUpdatePositionMethod = EUpdatePositionMethod(2);
}
#[repr(transparent)]
pub struct ESpawnOwnership(pub u8);
impl ESpawnOwnership {
    pub const INNER_SEQUENCE: ESpawnOwnership = ESpawnOwnership(0);
    pub const ROOT_SEQUENCE: ESpawnOwnership = ESpawnOwnership(1);
    pub const EXTERNAL: ESpawnOwnership = ESpawnOwnership(2);
}
#[repr(transparent)]
pub struct ETestMovieSceneEnum(pub u8);
impl ETestMovieSceneEnum {
    pub const ONE: ETestMovieSceneEnum = ETestMovieSceneEnum(0);
    pub const TWO: ETestMovieSceneEnum = ETestMovieSceneEnum(1);
    pub const THREE: ETestMovieSceneEnum = ETestMovieSceneEnum(2);
}
#[repr(transparent)]
pub struct EUpdateClockSource(pub u8);
impl EUpdateClockSource {
    pub const TICK: EUpdateClockSource = EUpdateClockSource(0);
    pub const PLATFORM: EUpdateClockSource = EUpdateClockSource(1);
    pub const AUDIO: EUpdateClockSource = EUpdateClockSource(2);
    pub const RELATIVE_TIMECODE: EUpdateClockSource = EUpdateClockSource(3);
    pub const TIMECODE: EUpdateClockSource = EUpdateClockSource(4);
    pub const PLAY_EVERY_FRAME: EUpdateClockSource = EUpdateClockSource(5);
    pub const CUSTOM: EUpdateClockSource = EUpdateClockSource(6);
}
#[repr(transparent)]
pub struct EMovieSceneTimeUnit(pub u8);
impl EMovieSceneTimeUnit {
    pub const DISPLAY_RATE: EMovieSceneTimeUnit = EMovieSceneTimeUnit(0);
    pub const TICK_RESOLUTION: EMovieSceneTimeUnit = EMovieSceneTimeUnit(1);
}
#[repr(transparent)]
pub struct EMovieSceneConditionCheckFrequency(pub u8);
impl EMovieSceneConditionCheckFrequency {
    pub const ONCE: EMovieSceneConditionCheckFrequency = EMovieSceneConditionCheckFrequency(
        0,
    );
    pub const ON_TICK: EMovieSceneConditionCheckFrequency = EMovieSceneConditionCheckFrequency(
        1,
    );
}
#[repr(transparent)]
pub struct EMovieSceneConditionScope(pub u8);
impl EMovieSceneConditionScope {
    pub const GLOBAL: EMovieSceneConditionScope = EMovieSceneConditionScope(0);
    pub const BINDING: EMovieSceneConditionScope = EMovieSceneConditionScope(1);
    pub const OWNER_OBJECT: EMovieSceneConditionScope = EMovieSceneConditionScope(2);
}
#[repr(transparent)]
pub struct EMovieSceneKeyInterpolation(pub u8);
impl EMovieSceneKeyInterpolation {
    pub const AUTO: EMovieSceneKeyInterpolation = EMovieSceneKeyInterpolation(0);
    pub const USER: EMovieSceneKeyInterpolation = EMovieSceneKeyInterpolation(1);
    pub const BREAK: EMovieSceneKeyInterpolation = EMovieSceneKeyInterpolation(2);
    pub const LINEAR: EMovieSceneKeyInterpolation = EMovieSceneKeyInterpolation(3);
    pub const CONSTANT: EMovieSceneKeyInterpolation = EMovieSceneKeyInterpolation(4);
    pub const SMART_AUTO: EMovieSceneKeyInterpolation = EMovieSceneKeyInterpolation(5);
}
#[repr(transparent)]
pub struct EMovieSceneEvaluationType(pub u8);
impl EMovieSceneEvaluationType {
    pub const FRAME_LOCKED: EMovieSceneEvaluationType = EMovieSceneEvaluationType(0);
    pub const WITH_SUB_FRAMES: EMovieSceneEvaluationType = EMovieSceneEvaluationType(1);
}
#[repr(transparent)]
pub struct EMovieSceneSequenceFlags(pub u8);
impl EMovieSceneSequenceFlags {
    pub const NONE: EMovieSceneSequenceFlags = EMovieSceneSequenceFlags(0);
    pub const VOLATILE: EMovieSceneSequenceFlags = EMovieSceneSequenceFlags(1);
    pub const BLOCKING_EVALUATION: EMovieSceneSequenceFlags = EMovieSceneSequenceFlags(
        2,
    );
    pub const DYNAMIC_WEIGHTING: EMovieSceneSequenceFlags = EMovieSceneSequenceFlags(4);
    pub const LOOP_CUTS: EMovieSceneSequenceFlags = EMovieSceneSequenceFlags(8);
    pub const INHERITED_FLAGS: EMovieSceneSequenceFlags = EMovieSceneSequenceFlags(1);
}
#[repr(transparent)]
pub struct EMovieSceneSequenceCompilerMask(pub u8);
impl EMovieSceneSequenceCompilerMask {
    pub const HIERARCHY: EMovieSceneSequenceCompilerMask = EMovieSceneSequenceCompilerMask(
        1,
    );
    pub const EVALUATION_TEMPLATE: EMovieSceneSequenceCompilerMask = EMovieSceneSequenceCompilerMask(
        2,
    );
    pub const EVALUATION_TEMPLATE_FIELD: EMovieSceneSequenceCompilerMask = EMovieSceneSequenceCompilerMask(
        4,
    );
    pub const ENTITY_COMPONENT_FIELD: EMovieSceneSequenceCompilerMask = EMovieSceneSequenceCompilerMask(
        8,
    );
    pub const NONE: EMovieSceneSequenceCompilerMask = EMovieSceneSequenceCompilerMask(0);
}
#[repr(transparent)]
pub struct EMovieSceneGroupConditionOperator(pub u8);
impl EMovieSceneGroupConditionOperator {
    pub const AND: EMovieSceneGroupConditionOperator = EMovieSceneGroupConditionOperator(
        0,
    );
    pub const OR: EMovieSceneGroupConditionOperator = EMovieSceneGroupConditionOperator(
        1,
    );
    pub const XOR: EMovieSceneGroupConditionOperator = EMovieSceneGroupConditionOperator(
        2,
    );
}
#[repr(transparent)]
pub struct EMovieSceneBuiltInEasing(pub u8);
impl EMovieSceneBuiltInEasing {
    pub const LINEAR: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(0);
    pub const SIN_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(1);
    pub const SIN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(2);
    pub const SIN_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(3);
    pub const QUAD_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(4);
    pub const QUAD_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(5);
    pub const QUAD_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(6);
    pub const CUBIC: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(7);
    pub const CUBIC_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(8);
    pub const CUBIC_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(9);
    pub const CUBIC_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(10);
    pub const HERMITE_CUBIC_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(
        11,
    );
    pub const QUART_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(12);
    pub const QUART_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(13);
    pub const QUART_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(14);
    pub const QUINT_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(15);
    pub const QUINT_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(16);
    pub const QUINT_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(17);
    pub const EXPO_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(18);
    pub const EXPO_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(19);
    pub const EXPO_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(20);
    pub const CIRC_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(21);
    pub const CIRC_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(22);
    pub const CIRC_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(23);
    pub const CUSTOM: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(24);
}
