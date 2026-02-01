#![allow(clippy::all)]
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
    pub u_sequencer_animation_override_get_sequencer_anim_slot_names: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_animation_override_allows_cinematic_override: *mut crate::ffi::UFunctionOpague,
    pub u_animation_state_machine_library_set_state: *mut crate::ffi::UFunctionOpague,
    pub u_animation_state_machine_library_is_state_blending_out: *mut crate::ffi::UFunctionOpague,
    pub u_animation_state_machine_library_is_state_blending_in: *mut crate::ffi::UFunctionOpague,
    pub u_animation_state_machine_library_get_state: *mut crate::ffi::UFunctionOpague,
    pub u_animation_state_machine_library_get_relevant_anim_time_remaining_fraction: *mut crate::ffi::UFunctionOpague,
    pub u_animation_state_machine_library_get_relevant_anim_time_remaining: *mut crate::ffi::UFunctionOpague,
    pub u_animation_state_machine_library_convert_to_animation_state_result_pure: *mut crate::ffi::UFunctionOpague,
    pub u_animation_state_machine_library_convert_to_animation_state_result: *mut crate::ffi::UFunctionOpague,
    pub u_animation_state_machine_library_convert_to_animation_state_machine_pure: *mut crate::ffi::UFunctionOpague,
    pub u_animation_state_machine_library_convert_to_animation_state_machine: *mut crate::ffi::UFunctionOpague,
    pub u_anim_execution_context_library_prototype_thread_safe_anim_update_call: *mut crate::ffi::UFunctionOpague,
    pub u_anim_execution_context_library_prototype_thread_safe_anim_node_call: *mut crate::ffi::UFunctionOpague,
    pub u_anim_execution_context_library_is_active: *mut crate::ffi::UFunctionOpague,
    pub u_anim_execution_context_library_get_delta_time: *mut crate::ffi::UFunctionOpague,
    pub u_anim_execution_context_library_get_current_weight: *mut crate::ffi::UFunctionOpague,
    pub u_anim_execution_context_library_get_anim_node_reference: *mut crate::ffi::UFunctionOpague,
    pub u_anim_execution_context_library_get_anim_instance: *mut crate::ffi::UFunctionOpague,
    pub u_anim_execution_context_library_convert_to_update_context: *mut crate::ffi::UFunctionOpague,
    pub u_anim_execution_context_library_convert_to_pose_context: *mut crate::ffi::UFunctionOpague,
    pub u_anim_execution_context_library_convert_to_initialization_context: *mut crate::ffi::UFunctionOpague,
    pub u_anim_execution_context_library_convert_to_component_space_pose_context: *mut crate::ffi::UFunctionOpague,
    pub u_blend_list_base_library_reset_node: *mut crate::ffi::UFunctionOpague,
    pub u_blend_list_base_library_convert_to_blend_list_base: *mut crate::ffi::UFunctionOpague,
    pub u_blend_space_library_snap_to_position: *mut crate::ffi::UFunctionOpague,
    pub u_blend_space_library_get_position: *mut crate::ffi::UFunctionOpague,
    pub u_blend_space_library_get_filtered_position: *mut crate::ffi::UFunctionOpague,
    pub u_blend_space_library_convert_to_blend_space_pure: *mut crate::ffi::UFunctionOpague,
    pub u_blend_space_library_convert_to_blend_space: *mut crate::ffi::UFunctionOpague,
    pub u_blend_space_player_library_snap_to_position: *mut crate::ffi::UFunctionOpague,
    pub u_blend_space_player_library_should_reset_play_time_when_blend_space_changes: *mut crate::ffi::UFunctionOpague,
    pub u_blend_space_player_library_set_reset_play_time_when_blend_space_changes: *mut crate::ffi::UFunctionOpague,
    pub u_blend_space_player_library_set_play_rate: *mut crate::ffi::UFunctionOpague,
    pub u_blend_space_player_library_set_loop: *mut crate::ffi::UFunctionOpague,
    pub u_blend_space_player_library_set_blend_space_with_inertial_blending: *mut crate::ffi::UFunctionOpague,
    pub u_blend_space_player_library_set_blend_space: *mut crate::ffi::UFunctionOpague,
    pub u_blend_space_player_library_get_start_position: *mut crate::ffi::UFunctionOpague,
    pub u_blend_space_player_library_get_position: *mut crate::ffi::UFunctionOpague,
    pub u_blend_space_player_library_get_play_rate: *mut crate::ffi::UFunctionOpague,
    pub u_blend_space_player_library_get_loop: *mut crate::ffi::UFunctionOpague,
    pub u_blend_space_player_library_get_blend_space: *mut crate::ffi::UFunctionOpague,
    pub u_blend_space_player_library_convert_to_blend_space_player_pure: *mut crate::ffi::UFunctionOpague,
    pub u_blend_space_player_library_convert_to_blend_space_player: *mut crate::ffi::UFunctionOpague,
    pub u_anim_node_rigid_body_library_set_override_physics_asset: *mut crate::ffi::UFunctionOpague,
    pub u_anim_node_rigid_body_library_convert_to_rigid_body_anim_node_pure: *mut crate::ffi::UFunctionOpague,
    pub u_anim_node_rigid_body_library_convert_to_rigid_body_anim_node: *mut crate::ffi::UFunctionOpague,
    pub u_kismet_animation_library_k2_two_bone_ik: *mut crate::ffi::UFunctionOpague,
    pub u_kismet_animation_library_k2_start_profiling_timer: *mut crate::ffi::UFunctionOpague,
    pub u_kismet_animation_library_k2_make_perlin_noise_vector_and_remap: *mut crate::ffi::UFunctionOpague,
    pub u_kismet_animation_library_k2_make_perlin_noise_and_remap: *mut crate::ffi::UFunctionOpague,
    pub u_kismet_animation_library_k2_look_at: *mut crate::ffi::UFunctionOpague,
    pub u_kismet_animation_library_k2_end_profiling_timer: *mut crate::ffi::UFunctionOpague,
    pub u_kismet_animation_library_k2_distance_between_two_sockets_and_map_range: *mut crate::ffi::UFunctionOpague,
    pub u_kismet_animation_library_k2_direction_between_sockets: *mut crate::ffi::UFunctionOpague,
    pub u_kismet_animation_library_k2_calculate_velocity_from_sockets: *mut crate::ffi::UFunctionOpague,
    pub u_kismet_animation_library_k2_calculate_velocity_from_position_history: *mut crate::ffi::UFunctionOpague,
    pub u_kismet_animation_library_calculate_direction: *mut crate::ffi::UFunctionOpague,
    pub u_layered_bone_blend_library_set_blend_mask: *mut crate::ffi::UFunctionOpague,
    pub u_layered_bone_blend_library_get_num_poses: *mut crate::ffi::UFunctionOpague,
    pub u_layered_bone_blend_library_convert_to_layered_bone_blend: *mut crate::ffi::UFunctionOpague,
    pub u_layered_bone_blend_library_convert_to_layered_blend_per_bone_pure: *mut crate::ffi::UFunctionOpague,
    pub u_linked_anim_graph_library_has_linked_anim_instance: *mut crate::ffi::UFunctionOpague,
    pub u_linked_anim_graph_library_get_linked_anim_instance: *mut crate::ffi::UFunctionOpague,
    pub u_linked_anim_graph_library_convert_to_linked_anim_graph_pure: *mut crate::ffi::UFunctionOpague,
    pub u_linked_anim_graph_library_convert_to_linked_anim_graph: *mut crate::ffi::UFunctionOpague,
    pub u_mirror_anim_library_set_mirror_transition_blend_time: *mut crate::ffi::UFunctionOpague,
    pub u_mirror_anim_library_set_mirror: *mut crate::ffi::UFunctionOpague,
    pub u_mirror_anim_library_get_mirror_transition_blend_time: *mut crate::ffi::UFunctionOpague,
    pub u_mirror_anim_library_get_mirror_data_table: *mut crate::ffi::UFunctionOpague,
    pub u_mirror_anim_library_get_mirror: *mut crate::ffi::UFunctionOpague,
    pub u_mirror_anim_library_convert_to_mirror_node_pure: *mut crate::ffi::UFunctionOpague,
    pub u_mirror_anim_library_convert_to_mirror_node: *mut crate::ffi::UFunctionOpague,
    pub u_modify_curve_anim_library_set_curve_map: *mut crate::ffi::UFunctionOpague,
    pub u_modify_curve_anim_library_set_apply_mode: *mut crate::ffi::UFunctionOpague,
    pub u_modify_curve_anim_library_set_alpha: *mut crate::ffi::UFunctionOpague,
    pub u_modify_curve_anim_library_get_apply_mode: *mut crate::ffi::UFunctionOpague,
    pub u_modify_curve_anim_library_get_alpha: *mut crate::ffi::UFunctionOpague,
    pub u_modify_curve_anim_library_convert_to_modify_curve_node_pure: *mut crate::ffi::UFunctionOpague,
    pub u_modify_curve_anim_library_convert_to_modify_curve_node: *mut crate::ffi::UFunctionOpague,
    pub u_play_montage_callback_proxy_on_notify_end_received: *mut crate::ffi::UFunctionOpague,
    pub u_play_montage_callback_proxy_on_notify_begin_received: *mut crate::ffi::UFunctionOpague,
    pub u_play_montage_callback_proxy_on_montage_ended: *mut crate::ffi::UFunctionOpague,
    pub u_play_montage_callback_proxy_on_montage_blending_out: *mut crate::ffi::UFunctionOpague,
    pub u_play_montage_callback_proxy_create_proxy_object_for_play_montage: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_evaluator_library_set_sequence_with_inertial_blending: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_evaluator_library_set_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_evaluator_library_set_explicit_time: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_evaluator_library_set_explicit_frame: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_evaluator_library_get_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_evaluator_library_get_accumulated_time: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_evaluator_library_convert_to_sequence_evaluator_pure: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_evaluator_library_convert_to_sequence_evaluator: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_evaluator_library_advance_time: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_player_library_set_start_position: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_player_library_set_sequence_with_inertial_blending: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_player_library_set_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_player_library_set_play_rate: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_player_library_set_accumulated_time: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_player_library_get_start_position: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_player_library_get_sequence_pure: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_player_library_get_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_player_library_get_play_rate: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_player_library_get_loop_animation: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_player_library_get_accumulated_time: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_player_library_convert_to_sequence_player_pure: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_player_library_convert_to_sequence_player: *mut crate::ffi::UFunctionOpague,
    pub u_sequence_player_library_compute_play_rate_from_duration: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_control_library_set_alpha: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_control_library_get_alpha: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_control_library_convert_to_skeletal_control_pure: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_control_library_convert_to_skeletal_control: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_sequencer_animation_override_get_sequencer_anim_slot_names: std::ptr::null_mut(),
            u_sequencer_animation_override_allows_cinematic_override: std::ptr::null_mut(),
            u_animation_state_machine_library_set_state: std::ptr::null_mut(),
            u_animation_state_machine_library_is_state_blending_out: std::ptr::null_mut(),
            u_animation_state_machine_library_is_state_blending_in: std::ptr::null_mut(),
            u_animation_state_machine_library_get_state: std::ptr::null_mut(),
            u_animation_state_machine_library_get_relevant_anim_time_remaining_fraction: std::ptr::null_mut(),
            u_animation_state_machine_library_get_relevant_anim_time_remaining: std::ptr::null_mut(),
            u_animation_state_machine_library_convert_to_animation_state_result_pure: std::ptr::null_mut(),
            u_animation_state_machine_library_convert_to_animation_state_result: std::ptr::null_mut(),
            u_animation_state_machine_library_convert_to_animation_state_machine_pure: std::ptr::null_mut(),
            u_animation_state_machine_library_convert_to_animation_state_machine: std::ptr::null_mut(),
            u_anim_execution_context_library_prototype_thread_safe_anim_update_call: std::ptr::null_mut(),
            u_anim_execution_context_library_prototype_thread_safe_anim_node_call: std::ptr::null_mut(),
            u_anim_execution_context_library_is_active: std::ptr::null_mut(),
            u_anim_execution_context_library_get_delta_time: std::ptr::null_mut(),
            u_anim_execution_context_library_get_current_weight: std::ptr::null_mut(),
            u_anim_execution_context_library_get_anim_node_reference: std::ptr::null_mut(),
            u_anim_execution_context_library_get_anim_instance: std::ptr::null_mut(),
            u_anim_execution_context_library_convert_to_update_context: std::ptr::null_mut(),
            u_anim_execution_context_library_convert_to_pose_context: std::ptr::null_mut(),
            u_anim_execution_context_library_convert_to_initialization_context: std::ptr::null_mut(),
            u_anim_execution_context_library_convert_to_component_space_pose_context: std::ptr::null_mut(),
            u_blend_list_base_library_reset_node: std::ptr::null_mut(),
            u_blend_list_base_library_convert_to_blend_list_base: std::ptr::null_mut(),
            u_blend_space_library_snap_to_position: std::ptr::null_mut(),
            u_blend_space_library_get_position: std::ptr::null_mut(),
            u_blend_space_library_get_filtered_position: std::ptr::null_mut(),
            u_blend_space_library_convert_to_blend_space_pure: std::ptr::null_mut(),
            u_blend_space_library_convert_to_blend_space: std::ptr::null_mut(),
            u_blend_space_player_library_snap_to_position: std::ptr::null_mut(),
            u_blend_space_player_library_should_reset_play_time_when_blend_space_changes: std::ptr::null_mut(),
            u_blend_space_player_library_set_reset_play_time_when_blend_space_changes: std::ptr::null_mut(),
            u_blend_space_player_library_set_play_rate: std::ptr::null_mut(),
            u_blend_space_player_library_set_loop: std::ptr::null_mut(),
            u_blend_space_player_library_set_blend_space_with_inertial_blending: std::ptr::null_mut(),
            u_blend_space_player_library_set_blend_space: std::ptr::null_mut(),
            u_blend_space_player_library_get_start_position: std::ptr::null_mut(),
            u_blend_space_player_library_get_position: std::ptr::null_mut(),
            u_blend_space_player_library_get_play_rate: std::ptr::null_mut(),
            u_blend_space_player_library_get_loop: std::ptr::null_mut(),
            u_blend_space_player_library_get_blend_space: std::ptr::null_mut(),
            u_blend_space_player_library_convert_to_blend_space_player_pure: std::ptr::null_mut(),
            u_blend_space_player_library_convert_to_blend_space_player: std::ptr::null_mut(),
            u_anim_node_rigid_body_library_set_override_physics_asset: std::ptr::null_mut(),
            u_anim_node_rigid_body_library_convert_to_rigid_body_anim_node_pure: std::ptr::null_mut(),
            u_anim_node_rigid_body_library_convert_to_rigid_body_anim_node: std::ptr::null_mut(),
            u_kismet_animation_library_k2_two_bone_ik: std::ptr::null_mut(),
            u_kismet_animation_library_k2_start_profiling_timer: std::ptr::null_mut(),
            u_kismet_animation_library_k2_make_perlin_noise_vector_and_remap: std::ptr::null_mut(),
            u_kismet_animation_library_k2_make_perlin_noise_and_remap: std::ptr::null_mut(),
            u_kismet_animation_library_k2_look_at: std::ptr::null_mut(),
            u_kismet_animation_library_k2_end_profiling_timer: std::ptr::null_mut(),
            u_kismet_animation_library_k2_distance_between_two_sockets_and_map_range: std::ptr::null_mut(),
            u_kismet_animation_library_k2_direction_between_sockets: std::ptr::null_mut(),
            u_kismet_animation_library_k2_calculate_velocity_from_sockets: std::ptr::null_mut(),
            u_kismet_animation_library_k2_calculate_velocity_from_position_history: std::ptr::null_mut(),
            u_kismet_animation_library_calculate_direction: std::ptr::null_mut(),
            u_layered_bone_blend_library_set_blend_mask: std::ptr::null_mut(),
            u_layered_bone_blend_library_get_num_poses: std::ptr::null_mut(),
            u_layered_bone_blend_library_convert_to_layered_bone_blend: std::ptr::null_mut(),
            u_layered_bone_blend_library_convert_to_layered_blend_per_bone_pure: std::ptr::null_mut(),
            u_linked_anim_graph_library_has_linked_anim_instance: std::ptr::null_mut(),
            u_linked_anim_graph_library_get_linked_anim_instance: std::ptr::null_mut(),
            u_linked_anim_graph_library_convert_to_linked_anim_graph_pure: std::ptr::null_mut(),
            u_linked_anim_graph_library_convert_to_linked_anim_graph: std::ptr::null_mut(),
            u_mirror_anim_library_set_mirror_transition_blend_time: std::ptr::null_mut(),
            u_mirror_anim_library_set_mirror: std::ptr::null_mut(),
            u_mirror_anim_library_get_mirror_transition_blend_time: std::ptr::null_mut(),
            u_mirror_anim_library_get_mirror_data_table: std::ptr::null_mut(),
            u_mirror_anim_library_get_mirror: std::ptr::null_mut(),
            u_mirror_anim_library_convert_to_mirror_node_pure: std::ptr::null_mut(),
            u_mirror_anim_library_convert_to_mirror_node: std::ptr::null_mut(),
            u_modify_curve_anim_library_set_curve_map: std::ptr::null_mut(),
            u_modify_curve_anim_library_set_apply_mode: std::ptr::null_mut(),
            u_modify_curve_anim_library_set_alpha: std::ptr::null_mut(),
            u_modify_curve_anim_library_get_apply_mode: std::ptr::null_mut(),
            u_modify_curve_anim_library_get_alpha: std::ptr::null_mut(),
            u_modify_curve_anim_library_convert_to_modify_curve_node_pure: std::ptr::null_mut(),
            u_modify_curve_anim_library_convert_to_modify_curve_node: std::ptr::null_mut(),
            u_play_montage_callback_proxy_on_notify_end_received: std::ptr::null_mut(),
            u_play_montage_callback_proxy_on_notify_begin_received: std::ptr::null_mut(),
            u_play_montage_callback_proxy_on_montage_ended: std::ptr::null_mut(),
            u_play_montage_callback_proxy_on_montage_blending_out: std::ptr::null_mut(),
            u_play_montage_callback_proxy_create_proxy_object_for_play_montage: std::ptr::null_mut(),
            u_sequence_evaluator_library_set_sequence_with_inertial_blending: std::ptr::null_mut(),
            u_sequence_evaluator_library_set_sequence: std::ptr::null_mut(),
            u_sequence_evaluator_library_set_explicit_time: std::ptr::null_mut(),
            u_sequence_evaluator_library_set_explicit_frame: std::ptr::null_mut(),
            u_sequence_evaluator_library_get_sequence: std::ptr::null_mut(),
            u_sequence_evaluator_library_get_accumulated_time: std::ptr::null_mut(),
            u_sequence_evaluator_library_convert_to_sequence_evaluator_pure: std::ptr::null_mut(),
            u_sequence_evaluator_library_convert_to_sequence_evaluator: std::ptr::null_mut(),
            u_sequence_evaluator_library_advance_time: std::ptr::null_mut(),
            u_sequence_player_library_set_start_position: std::ptr::null_mut(),
            u_sequence_player_library_set_sequence_with_inertial_blending: std::ptr::null_mut(),
            u_sequence_player_library_set_sequence: std::ptr::null_mut(),
            u_sequence_player_library_set_play_rate: std::ptr::null_mut(),
            u_sequence_player_library_set_accumulated_time: std::ptr::null_mut(),
            u_sequence_player_library_get_start_position: std::ptr::null_mut(),
            u_sequence_player_library_get_sequence_pure: std::ptr::null_mut(),
            u_sequence_player_library_get_sequence: std::ptr::null_mut(),
            u_sequence_player_library_get_play_rate: std::ptr::null_mut(),
            u_sequence_player_library_get_loop_animation: std::ptr::null_mut(),
            u_sequence_player_library_get_accumulated_time: std::ptr::null_mut(),
            u_sequence_player_library_convert_to_sequence_player_pure: std::ptr::null_mut(),
            u_sequence_player_library_convert_to_sequence_player: std::ptr::null_mut(),
            u_sequence_player_library_compute_play_rate_from_duration: std::ptr::null_mut(),
            u_skeletal_control_library_set_alpha: std::ptr::null_mut(),
            u_skeletal_control_library_get_alpha: std::ptr::null_mut(),
            u_skeletal_control_library_convert_to_skeletal_control_pure: std::ptr::null_mut(),
            u_skeletal_control_library_convert_to_skeletal_control: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USequencerAnimationOverride::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSequencerAnimSlotNames"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_animation_override_get_sequencer_anim_slot_names,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AllowsCinematicOverride"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_animation_override_allows_cinematic_override,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAnimationStateMachineLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetState"),
                &raw mut __FUNCTION_PTRS.u_animation_state_machine_library_set_state,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsStateBlendingOut"),
                &raw mut __FUNCTION_PTRS
                    .u_animation_state_machine_library_is_state_blending_out,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsStateBlendingIn"),
                &raw mut __FUNCTION_PTRS
                    .u_animation_state_machine_library_is_state_blending_in,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetState"),
                &raw mut __FUNCTION_PTRS.u_animation_state_machine_library_get_state,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRelevantAnimTimeRemainingFraction"),
                &raw mut __FUNCTION_PTRS
                    .u_animation_state_machine_library_get_relevant_anim_time_remaining_fraction,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRelevantAnimTimeRemaining"),
                &raw mut __FUNCTION_PTRS
                    .u_animation_state_machine_library_get_relevant_anim_time_remaining,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToAnimationStateResultPure"),
                &raw mut __FUNCTION_PTRS
                    .u_animation_state_machine_library_convert_to_animation_state_result_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToAnimationStateResult"),
                &raw mut __FUNCTION_PTRS
                    .u_animation_state_machine_library_convert_to_animation_state_result,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToAnimationStateMachinePure"),
                &raw mut __FUNCTION_PTRS
                    .u_animation_state_machine_library_convert_to_animation_state_machine_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToAnimationStateMachine"),
                &raw mut __FUNCTION_PTRS
                    .u_animation_state_machine_library_convert_to_animation_state_machine,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAnimExecutionContextLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Prototype_ThreadSafeAnimUpdateCall"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_execution_context_library_prototype_thread_safe_anim_update_call,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Prototype_ThreadSafeAnimNodeCall"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_execution_context_library_prototype_thread_safe_anim_node_call,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsActive"),
                &raw mut __FUNCTION_PTRS.u_anim_execution_context_library_is_active,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDeltaTime"),
                &raw mut __FUNCTION_PTRS.u_anim_execution_context_library_get_delta_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentWeight"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_execution_context_library_get_current_weight,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAnimNodeReference"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_execution_context_library_get_anim_node_reference,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAnimInstance"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_execution_context_library_get_anim_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToUpdateContext"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_execution_context_library_convert_to_update_context,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToPoseContext"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_execution_context_library_convert_to_pose_context,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToInitializationContext"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_execution_context_library_convert_to_initialization_context,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToComponentSpacePoseContext"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_execution_context_library_convert_to_component_space_pose_context,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBlendListBaseLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResetNode"),
                &raw mut __FUNCTION_PTRS.u_blend_list_base_library_reset_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToBlendListBase"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_list_base_library_convert_to_blend_list_base,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBlendSpaceLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SnapToPosition"),
                &raw mut __FUNCTION_PTRS.u_blend_space_library_snap_to_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPosition"),
                &raw mut __FUNCTION_PTRS.u_blend_space_library_get_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFilteredPosition"),
                &raw mut __FUNCTION_PTRS.u_blend_space_library_get_filtered_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToBlendSpacePure"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_space_library_convert_to_blend_space_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToBlendSpace"),
                &raw mut __FUNCTION_PTRS.u_blend_space_library_convert_to_blend_space,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBlendSpacePlayerLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SnapToPosition"),
                &raw mut __FUNCTION_PTRS.u_blend_space_player_library_snap_to_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ShouldResetPlayTimeWhenBlendSpaceChanges"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_space_player_library_should_reset_play_time_when_blend_space_changes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetResetPlayTimeWhenBlendSpaceChanges"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_space_player_library_set_reset_play_time_when_blend_space_changes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPlayRate"),
                &raw mut __FUNCTION_PTRS.u_blend_space_player_library_set_play_rate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLoop"),
                &raw mut __FUNCTION_PTRS.u_blend_space_player_library_set_loop,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBlendSpaceWithInertialBlending"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_space_player_library_set_blend_space_with_inertial_blending,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBlendSpace"),
                &raw mut __FUNCTION_PTRS.u_blend_space_player_library_set_blend_space,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetStartPosition"),
                &raw mut __FUNCTION_PTRS.u_blend_space_player_library_get_start_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPosition"),
                &raw mut __FUNCTION_PTRS.u_blend_space_player_library_get_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPlayRate"),
                &raw mut __FUNCTION_PTRS.u_blend_space_player_library_get_play_rate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLoop"),
                &raw mut __FUNCTION_PTRS.u_blend_space_player_library_get_loop,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBlendSpace"),
                &raw mut __FUNCTION_PTRS.u_blend_space_player_library_get_blend_space,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToBlendSpacePlayerPure"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_space_player_library_convert_to_blend_space_player_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToBlendSpacePlayer"),
                &raw mut __FUNCTION_PTRS
                    .u_blend_space_player_library_convert_to_blend_space_player,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAnimNodeRigidBodyLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetOverridePhysicsAsset"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_node_rigid_body_library_set_override_physics_asset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToRigidBodyAnimNodePure"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_node_rigid_body_library_convert_to_rigid_body_anim_node_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToRigidBodyAnimNode"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_node_rigid_body_library_convert_to_rigid_body_anim_node,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UKismetAnimationLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_TwoBoneIK"),
                &raw mut __FUNCTION_PTRS.u_kismet_animation_library_k2_two_bone_ik,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_StartProfilingTimer"),
                &raw mut __FUNCTION_PTRS
                    .u_kismet_animation_library_k2_start_profiling_timer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_MakePerlinNoiseVectorAndRemap"),
                &raw mut __FUNCTION_PTRS
                    .u_kismet_animation_library_k2_make_perlin_noise_vector_and_remap,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_MakePerlinNoiseAndRemap"),
                &raw mut __FUNCTION_PTRS
                    .u_kismet_animation_library_k2_make_perlin_noise_and_remap,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_LookAt"),
                &raw mut __FUNCTION_PTRS.u_kismet_animation_library_k2_look_at,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_EndProfilingTimer"),
                &raw mut __FUNCTION_PTRS
                    .u_kismet_animation_library_k2_end_profiling_timer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_DistanceBetweenTwoSocketsAndMapRange"),
                &raw mut __FUNCTION_PTRS
                    .u_kismet_animation_library_k2_distance_between_two_sockets_and_map_range,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_DirectionBetweenSockets"),
                &raw mut __FUNCTION_PTRS
                    .u_kismet_animation_library_k2_direction_between_sockets,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_CalculateVelocityFromSockets"),
                &raw mut __FUNCTION_PTRS
                    .u_kismet_animation_library_k2_calculate_velocity_from_sockets,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_CalculateVelocityFromPositionHistory"),
                &raw mut __FUNCTION_PTRS
                    .u_kismet_animation_library_k2_calculate_velocity_from_position_history,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CalculateDirection"),
                &raw mut __FUNCTION_PTRS.u_kismet_animation_library_calculate_direction,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ULayeredBoneBlendLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBlendMask"),
                &raw mut __FUNCTION_PTRS.u_layered_bone_blend_library_set_blend_mask,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumPoses"),
                &raw mut __FUNCTION_PTRS.u_layered_bone_blend_library_get_num_poses,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToLayeredBoneBlend"),
                &raw mut __FUNCTION_PTRS
                    .u_layered_bone_blend_library_convert_to_layered_bone_blend,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToLayeredBlendPerBonePure"),
                &raw mut __FUNCTION_PTRS
                    .u_layered_bone_blend_library_convert_to_layered_blend_per_bone_pure,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ULinkedAnimGraphLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasLinkedAnimInstance"),
                &raw mut __FUNCTION_PTRS
                    .u_linked_anim_graph_library_has_linked_anim_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLinkedAnimInstance"),
                &raw mut __FUNCTION_PTRS
                    .u_linked_anim_graph_library_get_linked_anim_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToLinkedAnimGraphPure"),
                &raw mut __FUNCTION_PTRS
                    .u_linked_anim_graph_library_convert_to_linked_anim_graph_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToLinkedAnimGraph"),
                &raw mut __FUNCTION_PTRS
                    .u_linked_anim_graph_library_convert_to_linked_anim_graph,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMirrorAnimLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMirrorTransitionBlendTime"),
                &raw mut __FUNCTION_PTRS
                    .u_mirror_anim_library_set_mirror_transition_blend_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMirror"),
                &raw mut __FUNCTION_PTRS.u_mirror_anim_library_set_mirror,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMirrorTransitionBlendTime"),
                &raw mut __FUNCTION_PTRS
                    .u_mirror_anim_library_get_mirror_transition_blend_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMirrorDataTable"),
                &raw mut __FUNCTION_PTRS.u_mirror_anim_library_get_mirror_data_table,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMirror"),
                &raw mut __FUNCTION_PTRS.u_mirror_anim_library_get_mirror,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToMirrorNodePure"),
                &raw mut __FUNCTION_PTRS
                    .u_mirror_anim_library_convert_to_mirror_node_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToMirrorNode"),
                &raw mut __FUNCTION_PTRS.u_mirror_anim_library_convert_to_mirror_node,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UModifyCurveAnimLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCurveMap"),
                &raw mut __FUNCTION_PTRS.u_modify_curve_anim_library_set_curve_map,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetApplyMode"),
                &raw mut __FUNCTION_PTRS.u_modify_curve_anim_library_set_apply_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAlpha"),
                &raw mut __FUNCTION_PTRS.u_modify_curve_anim_library_set_alpha,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetApplyMode"),
                &raw mut __FUNCTION_PTRS.u_modify_curve_anim_library_get_apply_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAlpha"),
                &raw mut __FUNCTION_PTRS.u_modify_curve_anim_library_get_alpha,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToModifyCurveNodePure"),
                &raw mut __FUNCTION_PTRS
                    .u_modify_curve_anim_library_convert_to_modify_curve_node_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToModifyCurveNode"),
                &raw mut __FUNCTION_PTRS
                    .u_modify_curve_anim_library_convert_to_modify_curve_node,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPlayMontageCallbackProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnNotifyEndReceived"),
                &raw mut __FUNCTION_PTRS
                    .u_play_montage_callback_proxy_on_notify_end_received,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnNotifyBeginReceived"),
                &raw mut __FUNCTION_PTRS
                    .u_play_montage_callback_proxy_on_notify_begin_received,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMontageEnded"),
                &raw mut __FUNCTION_PTRS.u_play_montage_callback_proxy_on_montage_ended,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMontageBlendingOut"),
                &raw mut __FUNCTION_PTRS
                    .u_play_montage_callback_proxy_on_montage_blending_out,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateProxyObjectForPlayMontage"),
                &raw mut __FUNCTION_PTRS
                    .u_play_montage_callback_proxy_create_proxy_object_for_play_montage,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USequenceEvaluatorLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSequenceWithInertialBlending"),
                &raw mut __FUNCTION_PTRS
                    .u_sequence_evaluator_library_set_sequence_with_inertial_blending,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSequence"),
                &raw mut __FUNCTION_PTRS.u_sequence_evaluator_library_set_sequence,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetExplicitTime"),
                &raw mut __FUNCTION_PTRS.u_sequence_evaluator_library_set_explicit_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetExplicitFrame"),
                &raw mut __FUNCTION_PTRS.u_sequence_evaluator_library_set_explicit_frame,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSequence"),
                &raw mut __FUNCTION_PTRS.u_sequence_evaluator_library_get_sequence,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAccumulatedTime"),
                &raw mut __FUNCTION_PTRS
                    .u_sequence_evaluator_library_get_accumulated_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToSequenceEvaluatorPure"),
                &raw mut __FUNCTION_PTRS
                    .u_sequence_evaluator_library_convert_to_sequence_evaluator_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToSequenceEvaluator"),
                &raw mut __FUNCTION_PTRS
                    .u_sequence_evaluator_library_convert_to_sequence_evaluator,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AdvanceTime"),
                &raw mut __FUNCTION_PTRS.u_sequence_evaluator_library_advance_time,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USequencePlayerLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetStartPosition"),
                &raw mut __FUNCTION_PTRS.u_sequence_player_library_set_start_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSequenceWithInertialBlending"),
                &raw mut __FUNCTION_PTRS
                    .u_sequence_player_library_set_sequence_with_inertial_blending,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSequence"),
                &raw mut __FUNCTION_PTRS.u_sequence_player_library_set_sequence,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPlayRate"),
                &raw mut __FUNCTION_PTRS.u_sequence_player_library_set_play_rate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAccumulatedTime"),
                &raw mut __FUNCTION_PTRS.u_sequence_player_library_set_accumulated_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetStartPosition"),
                &raw mut __FUNCTION_PTRS.u_sequence_player_library_get_start_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSequencePure"),
                &raw mut __FUNCTION_PTRS.u_sequence_player_library_get_sequence_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSequence"),
                &raw mut __FUNCTION_PTRS.u_sequence_player_library_get_sequence,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPlayRate"),
                &raw mut __FUNCTION_PTRS.u_sequence_player_library_get_play_rate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLoopAnimation"),
                &raw mut __FUNCTION_PTRS.u_sequence_player_library_get_loop_animation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAccumulatedTime"),
                &raw mut __FUNCTION_PTRS.u_sequence_player_library_get_accumulated_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToSequencePlayerPure"),
                &raw mut __FUNCTION_PTRS
                    .u_sequence_player_library_convert_to_sequence_player_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToSequencePlayer"),
                &raw mut __FUNCTION_PTRS
                    .u_sequence_player_library_convert_to_sequence_player,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ComputePlayRateFromDuration"),
                &raw mut __FUNCTION_PTRS
                    .u_sequence_player_library_compute_play_rate_from_duration,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USkeletalControlLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAlpha"),
                &raw mut __FUNCTION_PTRS.u_skeletal_control_library_set_alpha,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAlpha"),
                &raw mut __FUNCTION_PTRS.u_skeletal_control_library_get_alpha,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToSkeletalControlPure"),
                &raw mut __FUNCTION_PTRS
                    .u_skeletal_control_library_convert_to_skeletal_control_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToSkeletalControl"),
                &raw mut __FUNCTION_PTRS
                    .u_skeletal_control_library_convert_to_skeletal_control,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpaceGraphBase {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub x: f32,
    pub y: f32,
    pub group_name: FName,
    pub group_role: crate::bindings::engine::EAnimGroupRole,
    pub(crate) __padding_end: [u8; 99],
}
impl FAnimNode_BlendSpaceGraphBase {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpaceGraph {
    pub(crate) __padding_end: [u8; 256],
}
impl FAnimNode_BlendSpaceGraph {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpaceSampleResult {
    pub(crate) __padding_end: [u8; 200],
}
impl FAnimNode_BlendSpaceSampleResult {}
#[repr(C, align(8))]
pub struct FAnimNode_SkeletalControlBase {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub component_pose: crate::bindings::engine::FComponentSpacePoseLink,
    pub lod_threshold: i32,
    #[doc(hidden)]
    pub(crate) __padding_168: [u8; 4],
    pub alpha_input_type: crate::bindings::engine::EAnimAlphaInputType,
    pub b_alpha_bool_enabled: bool,
    pub alpha: f32,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub alpha_bool_blend: crate::bindings::engine::FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    pub(crate) __padding_end: [u8; 108],
}
impl FAnimNode_SkeletalControlBase {}
#[repr(C, align(8))]
pub struct FAnimNode_RotationMultiplier {
    #[doc(hidden)]
    pub(crate) __padding_464: [u8; 464],
    pub multiplier: f32,
}
impl FAnimNode_RotationMultiplier {}
#[repr(C, align(16))]
pub struct FRotationRetargetingInfo {
    pub(crate) __padding_end: [u8; 416],
}
impl FRotationRetargetingInfo {}
#[repr(C, align(8))]
pub struct FPositionHistory {
    pub(crate) __padding_end: [u8; 48],
}
impl FPositionHistory {}
#[repr(C, align(8))]
pub struct FAnimationStateResultReference {
    pub(crate) __padding_end: [u8; 16],
}
impl FAnimationStateResultReference {}
#[repr(C, align(8))]
pub struct FAnimationStateMachineReference {
    pub(crate) __padding_end: [u8; 16],
}
impl FAnimationStateMachineReference {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpacePlayerBase {
    pub(crate) __padding_end: [u8; 232],
}
impl FAnimNode_BlendSpacePlayerBase {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpacePlayer {
    pub(crate) __padding_end: [u8; 280],
}
impl FAnimNode_BlendSpacePlayer {}
#[repr(C, align(16))]
pub struct FAnimNode_AimOffsetLookAt {
    #[doc(hidden)]
    pub(crate) __padding_480: [u8; 480],
    pub base_pose: crate::bindings::engine::FPoseLink,
    pub lod_threshold: i32,
    pub source_socket_name: FName,
    pub pivot_socket_name: FName,
    pub look_at_location: crate::bindings::core_u_object::FVector,
    pub socket_axis: crate::bindings::core_u_object::FVector,
    pub alpha: f32,
    pub(crate) __padding_end: [u8; 84],
}
impl FAnimNode_AimOffsetLookAt {}
#[repr(C, align(8))]
pub struct FAnimNode_ApplyAdditive {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub base: crate::bindings::engine::FPoseLink,
    pub additive: crate::bindings::engine::FPoseLink,
    pub alpha: f32,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub lod_threshold: i32,
    pub alpha_bool_blend: crate::bindings::engine::FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    #[doc(hidden)]
    pub(crate) __padding_336: [u8; 4],
    pub alpha_input_type: crate::bindings::engine::EAnimAlphaInputType,
    pub b_alpha_bool_enabled: bool,
}
impl FAnimNode_ApplyAdditive {}
#[repr(C, align(4))]
pub struct FBlendBoneByChannelEntry {
    pub(crate) __padding_end: [u8; 44],
}
impl FBlendBoneByChannelEntry {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendBoneByChannel {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub a: crate::bindings::engine::FPoseLink,
    pub b: crate::bindings::engine::FPoseLink,
    #[doc(hidden)]
    pub(crate) __padding_216: [u8; 32],
    pub alpha: f32,
    #[doc(hidden)]
    pub(crate) __padding_224: [u8; 4],
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub(crate) __padding_end: [u8; 8],
}
impl FAnimNode_BlendBoneByChannel {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendListBase {
    pub(crate) __padding_end: [u8; 240],
}
impl FAnimNode_BlendListBase {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendListByBool {
    pub(crate) __padding_end: [u8; 256],
}
impl FAnimNode_BlendListByBool {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendListByEnum {
    pub(crate) __padding_end: [u8; 264],
}
impl FAnimNode_BlendListByEnum {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendListByInt {
    pub(crate) __padding_end: [u8; 248],
}
impl FAnimNode_BlendListByInt {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpaceEvaluator {
    #[doc(hidden)]
    pub(crate) __padding_280: [u8; 280],
    pub normalized_time: f32,
}
impl FAnimNode_BlendSpaceEvaluator {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpacePlayer_Standalone {
    pub(crate) __padding_end: [u8; 280],
}
impl FAnimNode_BlendSpacePlayer_Standalone {}
#[repr(C, align(8))]
pub struct FAnimNode_CallFunction {
    pub(crate) __padding_end: [u8; 224],
}
impl FAnimNode_CallFunction {}
#[repr(C, align(8))]
pub struct FAnimNode_CopyPoseFromMesh {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub source_mesh_component: TWeakObjectPtr<
        crate::bindings::engine::USkeletalMeshComponent,
    >,
    pub flags_144: u8,
    pub b_copy_custom_attributes: bool,
    pub flags_146: u8,
    pub root_bone_to_copy: FName,
    pub(crate) __padding_end: [u8; 304],
}
impl FAnimNode_CopyPoseFromMesh {}
#[repr(C, align(8))]
pub struct FAnimNode_CurveSource {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub source_pose: crate::bindings::engine::FPoseLink,
    pub source_binding: FName,
    pub alpha: f32,
    pub(crate) __padding_end: [u8; 16],
}
impl FAnimNode_CurveSource {}
#[repr(C, align(8))]
pub struct FAnimNode_LayeredBoneBlend {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub base_pose: crate::bindings::engine::FPoseLink,
    pub blend_poses: TArray<crate::bindings::engine::FPoseLink>,
    #[doc(hidden)]
    pub(crate) __padding_216: [u8; 40],
    pub blend_weights: TArray<f32>,
    #[doc(hidden)]
    pub(crate) __padding_348: [u8; 116],
    pub lod_threshold: i32,
    pub b_mesh_space_rotation_blend: bool,
    pub b_root_space_rotation_blend: bool,
    pub b_mesh_space_scale_blend: bool,
    pub curve_blend_option: crate::bindings::engine::ECurveBlendOption,
}
impl FAnimNode_LayeredBoneBlend {}
#[repr(C, align(8))]
pub struct FAnimNode_MakeDynamicAdditive {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub base: crate::bindings::engine::FPoseLink,
    pub additive: crate::bindings::engine::FPoseLink,
    pub b_mesh_space_additive: bool,
}
impl FAnimNode_MakeDynamicAdditive {}
#[repr(C, align(8))]
pub struct FAnimNode_MirrorBase {
    pub(crate) __padding_end: [u8; 200],
}
impl FAnimNode_MirrorBase {}
#[repr(C, align(8))]
pub struct FAnimNode_Mirror {
    pub(crate) __padding_end: [u8; 224],
}
impl FAnimNode_Mirror {}
#[repr(C, align(8))]
pub struct FAnimNode_Mirror_Standalone {
    pub(crate) __padding_end: [u8; 224],
}
impl FAnimNode_Mirror_Standalone {}
#[repr(C, align(8))]
pub struct FAnimNode_ModifyCurve {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub source_pose: crate::bindings::engine::FPoseLink,
    pub curve_map: TMap<FName, f32>,
    pub curve_values: TArray<f32>,
    #[doc(hidden)]
    pub(crate) __padding_408: [u8; 152],
    pub alpha: f32,
    pub apply_mode: EModifyCurveApplyMode,
}
impl FAnimNode_ModifyCurve {}
#[repr(C, align(8))]
pub struct FAnimNode_MultiWayBlend {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub poses: TArray<crate::bindings::engine::FPoseLink>,
    pub desired_alphas: TArray<f32>,
    #[doc(hidden)]
    pub(crate) __padding_184: [u8; 16],
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub b_additive_node: bool,
    pub b_normalize_alpha: bool,
}
impl FAnimNode_MultiWayBlend {}
#[repr(C, align(8))]
pub struct FAnimNode_PoseHandler {
    #[doc(hidden)]
    pub(crate) __padding_184: [u8; 184],
    pub pose_asset: UPtr<crate::bindings::engine::UPoseAsset>,
    pub(crate) __padding_end: [u8; 128],
}
impl FAnimNode_PoseHandler {}
#[repr(C, align(8))]
pub struct FAnimNode_PoseBlendNode {
    #[doc(hidden)]
    pub(crate) __padding_320: [u8; 320],
    pub source_pose: crate::bindings::engine::FPoseLink,
    pub(crate) __padding_end: [u8; 40],
}
impl FAnimNode_PoseBlendNode {}
#[repr(C, align(8))]
pub struct FAnimNode_PoseByName {
    #[doc(hidden)]
    pub(crate) __padding_320: [u8; 320],
    pub pose_name: FName,
    pub pose_weight: f32,
    pub(crate) __padding_end: [u8; 16],
}
impl FAnimNode_PoseByName {}
#[repr(C, align(8))]
pub struct FAnimNode_PoseDriver {
    #[doc(hidden)]
    pub(crate) __padding_320: [u8; 320],
    pub source_pose: crate::bindings::engine::FPoseLink,
    pub(crate) __padding_end: [u8; 288],
}
impl FAnimNode_PoseDriver {}
#[repr(C, align(8))]
pub struct FRBFParams {
    #[doc(hidden)]
    pub(crate) __padding_4: [u8; 4],
    pub solver_type: ERBFSolverType,
    pub radius: f32,
    pub b_automatic_radius: bool,
    pub function: ERBFFunctionType,
    pub distance_method: ERBFDistanceMethod,
    pub twist_axis: crate::bindings::engine::EBoneAxis,
    pub weight_threshold: f32,
    pub normalize_method: ERBFNormalizeMethod,
    pub median_reference: crate::bindings::core_u_object::FVector,
    pub median_min: f32,
    pub median_max: f32,
}
impl FRBFParams {}
#[repr(C, align(8))]
pub struct FAnimNode_PoseSnapshot {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub snapshot_name: FName,
    pub snapshot: crate::bindings::engine::FPoseSnapshot,
    pub mode: ESnapshotSourceMode,
    pub(crate) __padding_end: [u8; 79],
}
impl FAnimNode_PoseSnapshot {}
#[repr(C, align(8))]
pub struct FRandomPlayerSequenceEntry {
    pub sequence: UPtr<crate::bindings::engine::UAnimSequenceBase>,
    pub chance_to_play: f32,
    pub min_loop_count: i32,
    pub max_loop_count: i32,
    pub min_play_rate: f32,
    pub max_play_rate: f32,
    pub(crate) __padding_end: [u8; 52],
}
impl FRandomPlayerSequenceEntry {}
#[repr(C, align(8))]
pub struct FAnimNode_RandomPlayer {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub entries: TArray<FRandomPlayerSequenceEntry>,
    #[doc(hidden)]
    pub(crate) __padding_236: [u8; 84],
    pub blend_weight: f32,
    pub b_shuffle_mode: bool,
}
impl FAnimNode_RandomPlayer {}
#[repr(C, align(8))]
pub struct FAnimNode_RotateRootBone {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub base_pose: crate::bindings::engine::FPoseLink,
    pub pitch: f32,
    pub yaw: f32,
    pub pitch_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    pub yaw_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    pub mesh_to_component: crate::bindings::core_u_object::FRotator,
    pub b_rotate_root_motion_attribute: bool,
    pub(crate) __padding_end: [u8; 15],
}
impl FAnimNode_RotateRootBone {}
#[repr(C, align(8))]
pub struct FAnimNode_RotationOffsetBlendSpace {
    #[doc(hidden)]
    pub(crate) __padding_280: [u8; 280],
    pub base_pose: crate::bindings::engine::FPoseLink,
    pub lod_threshold: i32,
    pub alpha: f32,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub alpha_bool_blend: crate::bindings::engine::FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    #[doc(hidden)]
    pub(crate) __padding_456: [u8; 4],
    pub alpha_input_type: crate::bindings::engine::EAnimAlphaInputType,
    pub b_alpha_bool_enabled: bool,
}
impl FAnimNode_RotationOffsetBlendSpace {}
#[repr(C, align(8))]
pub struct FAnimNode_RotationOffsetBlendSpaceGraph {
    #[doc(hidden)]
    pub(crate) __padding_256: [u8; 256],
    pub base_pose: crate::bindings::engine::FPoseLink,
    pub lod_threshold: i32,
    pub alpha: f32,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub alpha_bool_blend: crate::bindings::engine::FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    #[doc(hidden)]
    pub(crate) __padding_432: [u8; 4],
    pub alpha_input_type: crate::bindings::engine::EAnimAlphaInputType,
    pub b_alpha_bool_enabled: bool,
}
impl FAnimNode_RotationOffsetBlendSpaceGraph {}
#[repr(C, align(8))]
pub struct FAnimNode_SequenceEvaluatorBase {
    pub(crate) __padding_end: [u8; 192],
}
impl FAnimNode_SequenceEvaluatorBase {}
#[repr(C, align(8))]
pub struct FAnimNode_SequenceEvaluator {
    pub(crate) __padding_end: [u8; 240],
}
impl FAnimNode_SequenceEvaluator {}
#[repr(C, align(8))]
pub struct FAnimNode_SequenceEvaluator_Standalone {
    pub(crate) __padding_end: [u8; 240],
}
impl FAnimNode_SequenceEvaluator_Standalone {}
#[repr(C, align(8))]
pub struct FAnimNode_Slot {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub source: crate::bindings::engine::FPoseLink,
    pub slot_name: FName,
    pub b_always_update_source_pose: bool,
    pub(crate) __padding_end: [u8; 35],
}
impl FAnimNode_Slot {}
#[repr(C, align(8))]
pub struct FAnimNode_Sync {
    pub(crate) __padding_end: [u8; 176],
}
impl FAnimNode_Sync {}
#[repr(C, align(8))]
pub struct FAnimNode_TwoWayBlend {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub a: crate::bindings::engine::FPoseLink,
    pub b: crate::bindings::engine::FPoseLink,
    pub alpha_input_type: crate::bindings::engine::EAnimAlphaInputType,
    pub flags_185: u8,
    pub alpha: f32,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub alpha_bool_blend: crate::bindings::engine::FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
}
impl FAnimNode_TwoWayBlend {}
#[repr(C, align(8))]
pub struct FBlendListBaseReference {
    pub(crate) __padding_end: [u8; 16],
}
impl FBlendListBaseReference {}
#[repr(C, align(8))]
pub struct FBlendSpaceReference {
    pub(crate) __padding_end: [u8; 16],
}
impl FBlendSpaceReference {}
#[repr(C, align(8))]
pub struct FBlendSpacePlayerReference {
    pub(crate) __padding_end: [u8; 16],
}
impl FBlendSpacePlayerReference {}
#[repr(C, align(8))]
pub struct FAnimPhysSimSpaceSettings {
    pub sim_space_angular_alpha: f32,
    pub max_angular_velocity: f32,
    pub max_angular_acceleration: f32,
    pub external_angular_velocity: crate::bindings::core_u_object::FVector,
}
impl FAnimPhysSimSpaceSettings {}
#[repr(C, align(16))]
pub struct FAnimNode_AnimDynamics {
    #[doc(hidden)]
    pub(crate) __padding_424: [u8; 424],
    pub linear_damping_override: f32,
    pub angular_damping_override: f32,
    #[doc(hidden)]
    pub(crate) __padding_800: [u8; 368],
    pub gravity_scale: f32,
    pub gravity_override: crate::bindings::core_u_object::FVector,
    pub linear_spring_constant: f32,
    pub angular_spring_constant: f32,
    #[doc(hidden)]
    pub(crate) __padding_960: [u8; 120],
    pub angular_bias_override: f32,
    #[doc(hidden)]
    pub(crate) __padding_1032: [u8; 68],
    pub simulation_space: AnimPhysSimSpaceType,
    #[doc(hidden)]
    pub(crate) __padding_1036: [u8; 3],
    pub flags_1036: u8,
    pub(crate) __padding_end: [u8; 883],
}
impl FAnimNode_AnimDynamics {}
#[repr(C, align(8))]
pub struct FAnimNode_ApplyLimits {
    #[doc(hidden)]
    pub(crate) __padding_440: [u8; 440],
    pub angular_offsets: TArray<crate::bindings::core_u_object::FVector>,
}
impl FAnimNode_ApplyLimits {}
#[repr(C, align(8))]
pub struct FAnimNode_BoneDrivenController {
    pub(crate) __padding_end: [u8; 536],
}
impl FAnimNode_BoneDrivenController {}
#[repr(C, align(16))]
pub struct FAnimNode_CCDIK {
    pub(crate) __padding_end: [u8; 720],
}
impl FAnimNode_CCDIK {}
#[repr(C, align(16))]
pub struct FAnimNode_Constraint {
    pub(crate) __padding_end: [u8; 704],
}
impl FAnimNode_Constraint {}
#[repr(C, align(8))]
pub struct FAnimNode_CopyBone {
    #[doc(hidden)]
    pub(crate) __padding_464: [u8; 464],
    pub b_copy_translation: bool,
    pub b_copy_rotation: bool,
    pub b_copy_scale: bool,
}
impl FAnimNode_CopyBone {}
#[repr(C, align(8))]
pub struct FAnimNode_CopyBoneDelta {
    #[doc(hidden)]
    pub(crate) __padding_464: [u8; 464],
    pub b_copy_translation: bool,
    pub b_copy_rotation: bool,
    pub b_copy_scale: bool,
    pub translation_multiplier: f32,
    pub rotation_multiplier: f32,
    pub scale_multiplier: f32,
}
impl FAnimNode_CopyBoneDelta {}
#[repr(C, align(16))]
pub struct FAnimNode_Fabrik {
    #[doc(hidden)]
    pub(crate) __padding_432: [u8; 432],
    pub effector_transform: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 336],
}
impl FAnimNode_Fabrik {}
#[repr(C, align(8))]
pub struct FAnimNode_HandIKRetargeting {
    #[doc(hidden)]
    pub(crate) __padding_520: [u8; 520],
    pub per_axis_alpha: crate::bindings::core_u_object::FVector,
    pub hand_fk_weight: f32,
}
impl FAnimNode_HandIKRetargeting {}
#[repr(C, align(8))]
pub struct FAnimNode_LegIK {
    pub(crate) __padding_end: [u8; 480],
}
impl FAnimNode_LegIK {}
#[repr(C, align(16))]
pub struct FAnimNode_LookAt {
    #[doc(hidden)]
    pub(crate) __padding_608: [u8; 608],
    pub look_at_location: crate::bindings::core_u_object::FVector,
    #[doc(hidden)]
    pub(crate) __padding_665: [u8; 33],
    pub interpolation_type: EInterpolationBlend,
    #[doc(hidden)]
    pub(crate) __padding_704: [u8; 36],
    pub look_at_clamp: f32,
    pub interpolation_time: f32,
    pub interpolation_trigger_threashold: f32,
    pub(crate) __padding_end: [u8; 548],
}
impl FAnimNode_LookAt {}
#[repr(C, align(8))]
pub struct FAnimNode_ModifyBone {
    #[doc(hidden)]
    pub(crate) __padding_448: [u8; 448],
    pub translation: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub scale: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 8],
}
impl FAnimNode_ModifyBone {}
#[repr(C, align(8))]
pub struct FAnimNode_ObserveBone {
    pub(crate) __padding_end: [u8; 520],
}
impl FAnimNode_ObserveBone {}
#[repr(C, align(8))]
pub struct FAnimNode_ResetRoot {
    pub(crate) __padding_end: [u8; 440],
}
impl FAnimNode_ResetRoot {}
#[repr(C, align(8))]
pub struct FSimSpaceSettings {
    pub world_alpha: f32,
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 4],
    pub velocity_scale_z: f32,
    pub damping_alpha: f32,
    pub max_linear_velocity: f32,
    pub max_angular_velocity: f32,
    pub max_linear_acceleration: f32,
    pub max_angular_acceleration: f32,
    #[doc(hidden)]
    pub(crate) __padding_40: [u8; 8],
    pub external_linear_drag_v: crate::bindings::core_u_object::FVector,
    pub external_linear_velocity: crate::bindings::core_u_object::FVector,
    pub external_angular_velocity: crate::bindings::core_u_object::FVector,
}
impl FSimSpaceSettings {}
#[repr(C, align(16))]
pub struct FAnimNode_RigidBody {
    #[doc(hidden)]
    pub(crate) __padding_744: [u8; 744],
    pub b_use_local_lod_threshold_only: bool,
    pub(crate) __padding_end: [u8; 1895],
}
impl FAnimNode_RigidBody {}
#[repr(C, align(8))]
pub struct FRigidBodyAnimNodeReference {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigidBodyAnimNodeReference {}
#[repr(C, align(8))]
pub struct FAnimNode_ScaleChainLength {
    #[doc(hidden)]
    pub(crate) __padding_160: [u8; 160],
    pub default_chain_length: f32,
    #[doc(hidden)]
    pub(crate) __padding_208: [u8; 40],
    pub target_location: crate::bindings::core_u_object::FVector,
    pub alpha: f32,
    pub(crate) __padding_end: [u8; 36],
}
impl FAnimNode_ScaleChainLength {}
#[repr(C, align(8))]
pub struct FAnimNode_SplineIK {
    #[doc(hidden)]
    pub(crate) __padding_472: [u8; 472],
    pub control_points: TArray<crate::bindings::core_u_object::FTransform>,
    pub roll: f32,
    pub twist_start: f32,
    pub twist_end: f32,
    #[doc(hidden)]
    pub(crate) __padding_552: [u8; 52],
    pub stretch: f32,
    pub offset: f32,
    pub(crate) __padding_end: [u8; 296],
}
impl FAnimNode_SplineIK {}
#[repr(C, align(8))]
pub struct FAnimNode_SpringBone {
    #[doc(hidden)]
    pub(crate) __padding_448: [u8; 448],
    pub max_displacement: f64,
    pub(crate) __padding_end: [u8; 160],
}
impl FAnimNode_SpringBone {}
#[repr(C, align(16))]
pub struct FAnimNode_Trail {
    #[doc(hidden)]
    pub(crate) __padding_568: [u8; 568],
    pub relaxation_speed_scale: f32,
    #[doc(hidden)]
    pub(crate) __padding_712: [u8; 140],
    pub relaxation_speed_scale_input_processor: crate::bindings::engine::FInputScaleBiasClamp,
    #[doc(hidden)]
    pub(crate) __padding_776: [u8; 16],
    pub rotation_offsets: TArray<crate::bindings::core_u_object::FVector>,
    pub(crate) __padding_end: [u8; 168],
}
impl FAnimNode_Trail {}
#[repr(C, align(8))]
pub struct FAnimNode_TwistCorrectiveNode {
    pub(crate) __padding_end: [u8; 616],
}
impl FAnimNode_TwistCorrectiveNode {}
#[repr(C, align(16))]
pub struct FAnimNode_TwoBoneIK {
    #[doc(hidden)]
    pub(crate) __padding_512: [u8; 512],
    pub effector_location: crate::bindings::core_u_object::FVector,
    #[doc(hidden)]
    pub(crate) __padding_704: [u8; 168],
    pub joint_target_location: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 312],
}
impl FAnimNode_TwoBoneIK {}
#[repr(C, align(8))]
pub struct FIKFootPelvisPullDownSolver {
    pub(crate) __padding_end: [u8; 128],
}
impl FIKFootPelvisPullDownSolver {}
#[repr(C, align(8))]
pub struct FWarpingVectorValue {
    pub mode: EWarpingVectorMode,
    pub value: crate::bindings::core_u_object::FVector,
}
impl FWarpingVectorValue {}
#[repr(C, align(8))]
pub struct FLayeredBoneBlendReference {
    pub(crate) __padding_end: [u8; 16],
}
impl FLayeredBoneBlendReference {}
#[repr(C, align(8))]
pub struct FLinkedAnimGraphReference {
    pub(crate) __padding_end: [u8; 16],
}
impl FLinkedAnimGraphReference {}
#[repr(C, align(8))]
pub struct FMirrorAnimNodeReference {
    pub(crate) __padding_end: [u8; 16],
}
impl FMirrorAnimNodeReference {}
#[repr(C, align(8))]
pub struct FModifyCurveAnimNodeReference {
    pub(crate) __padding_end: [u8; 16],
}
impl FModifyCurveAnimNodeReference {}
#[repr(C, align(8))]
pub struct FSequenceEvaluatorReference {
    pub(crate) __padding_end: [u8; 16],
}
impl FSequenceEvaluatorReference {}
#[repr(C, align(8))]
pub struct FSequencePlayerReference {
    pub(crate) __padding_end: [u8; 16],
}
impl FSequencePlayerReference {}
#[repr(C, align(8))]
pub struct FSkeletalControlReference {
    pub(crate) __padding_end: [u8; 16],
}
impl FSkeletalControlReference {}
pub struct ISequencerAnimationOverride {}
#[repr(C, align(8))]
pub struct USequencerAnimationOverride {
    __padding_end: [u8; 48],
}
impl USequencerAnimationOverride {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerAnimationOverride")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerAnimationOverride")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_sequencer_anim_slot_names(&self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequencer_animation_override_get_sequencer_anim_slot_names,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequencer_animation_override_get_sequencer_anim_slot_names,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn allows_cinematic_override(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequencer_animation_override_allows_cinematic_override,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequencer_animation_override_allows_cinematic_override,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAnimationStateMachineLibrary {
    __padding_end: [u8; 48],
}
impl UAnimationStateMachineLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationStateMachineLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationStateMachineLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_state(
        update_context: &crate::bindings::engine::FAnimUpdateContext,
        node: &FAnimationStateMachineReference,
        target_state: FName,
        duration: f32,
        blend_type: crate::bindings::engine::ETransitionLogicType,
        blend_profile: UPtr<crate::bindings::engine::UBlendProfile>,
        alpha_blend_option: crate::bindings::engine::EAlphaBlendOption,
        custom_blend_curve: UPtr<crate::bindings::engine::UCurveFloat>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_animation_state_machine_library_set_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                update_context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimUpdateContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(16).cast::<FAnimationStateMachineReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_state,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(44).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_type,
                __buffer.add(48).cast::<crate::bindings::engine::ETransitionLogicType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_profile,
                __buffer.add(56).cast::<UPtr<crate::bindings::engine::UBlendProfile>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &alpha_blend_option,
                __buffer.add(64).cast::<crate::bindings::engine::EAlphaBlendOption>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &custom_blend_curve,
                __buffer.add(72).cast::<UPtr<crate::bindings::engine::UCurveFloat>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimationStateMachineLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_animation_state_machine_library_set_state,
                __buffer,
            )
        };
    }
    pub fn is_state_blending_out(
        update_context: &crate::bindings::engine::FAnimUpdateContext,
        node: &FAnimationStateResultReference,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_animation_state_machine_library_is_state_blending_out,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                update_context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimUpdateContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(16).cast::<FAnimationStateResultReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimationStateMachineLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_animation_state_machine_library_is_state_blending_out,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn is_state_blending_in(
        update_context: &crate::bindings::engine::FAnimUpdateContext,
        node: &FAnimationStateResultReference,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_animation_state_machine_library_is_state_blending_in,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                update_context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimUpdateContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(16).cast::<FAnimationStateResultReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimationStateMachineLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_animation_state_machine_library_is_state_blending_in,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_state(
        update_context: &crate::bindings::engine::FAnimUpdateContext,
        node: &FAnimationStateMachineReference,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_animation_state_machine_library_get_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                update_context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimUpdateContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(16).cast::<FAnimationStateMachineReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimationStateMachineLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_animation_state_machine_library_get_state,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FName>().read() }
    }
    pub fn get_relevant_anim_time_remaining_fraction(
        update_context: &crate::bindings::engine::FAnimUpdateContext,
        node: &FAnimationStateResultReference,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_animation_state_machine_library_get_relevant_anim_time_remaining_fraction,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                update_context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimUpdateContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(16).cast::<FAnimationStateResultReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimationStateMachineLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_animation_state_machine_library_get_relevant_anim_time_remaining_fraction,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<f32>().read() }
    }
    pub fn get_relevant_anim_time_remaining(
        update_context: &crate::bindings::engine::FAnimUpdateContext,
        node: &FAnimationStateResultReference,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_animation_state_machine_library_get_relevant_anim_time_remaining,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                update_context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimUpdateContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(16).cast::<FAnimationStateResultReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimationStateMachineLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_animation_state_machine_library_get_relevant_anim_time_remaining,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<f32>().read() }
    }
    pub fn convert_to_animation_state_result_pure(
        node: &crate::bindings::engine::FAnimNodeReference,
        animation_state: &mut FAnimationStateResultReference,
        result: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_animation_state_machine_library_convert_to_animation_state_result_pure,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                animation_state,
                __buffer.add(16).cast::<FAnimationStateResultReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(result, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimationStateMachineLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_animation_state_machine_library_convert_to_animation_state_result_pure,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<FAnimationStateResultReference>()
                .swap(animation_state);
        }
        unsafe {
            __buffer.add(32).cast::<bool>().swap(result);
        }
    }
    pub fn convert_to_animation_state_result(
        node: &crate::bindings::engine::FAnimNodeReference,
        animation_state: &mut FAnimationStateResultReference,
        result: &mut crate::bindings::engine::EAnimNodeReferenceConversionResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_animation_state_machine_library_convert_to_animation_state_result,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                animation_state,
                __buffer.add(16).cast::<FAnimationStateResultReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(32)
                    .cast::<
                        crate::bindings::engine::EAnimNodeReferenceConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimationStateMachineLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_animation_state_machine_library_convert_to_animation_state_result,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<FAnimationStateResultReference>()
                .swap(animation_state);
        }
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::engine::EAnimNodeReferenceConversionResult>()
                .swap(result);
        }
    }
    pub fn convert_to_animation_state_machine_pure(
        node: &crate::bindings::engine::FAnimNodeReference,
        animation_state: &mut FAnimationStateMachineReference,
        result: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_animation_state_machine_library_convert_to_animation_state_machine_pure,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                animation_state,
                __buffer.add(16).cast::<FAnimationStateMachineReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(result, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimationStateMachineLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_animation_state_machine_library_convert_to_animation_state_machine_pure,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<FAnimationStateMachineReference>()
                .swap(animation_state);
        }
        unsafe {
            __buffer.add(32).cast::<bool>().swap(result);
        }
    }
    pub fn convert_to_animation_state_machine(
        node: &crate::bindings::engine::FAnimNodeReference,
        animation_state: &mut FAnimationStateMachineReference,
        result: &mut crate::bindings::engine::EAnimNodeReferenceConversionResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_animation_state_machine_library_convert_to_animation_state_machine,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                animation_state,
                __buffer.add(16).cast::<FAnimationStateMachineReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(32)
                    .cast::<
                        crate::bindings::engine::EAnimNodeReferenceConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimationStateMachineLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_animation_state_machine_library_convert_to_animation_state_machine,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<FAnimationStateMachineReference>()
                .swap(animation_state);
        }
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::engine::EAnimNodeReferenceConversionResult>()
                .swap(result);
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimExecutionContextLibrary {
    __padding_end: [u8; 48],
}
impl UAnimExecutionContextLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimExecutionContextLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimExecutionContextLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn is_active(context: &crate::bindings::engine::FAnimExecutionContext) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_execution_context_library_is_active,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimExecutionContext>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimExecutionContextLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_execution_context_library_is_active,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_delta_time(context: &crate::bindings::engine::FAnimUpdateContext) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_execution_context_library_get_delta_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimUpdateContext>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimExecutionContextLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_execution_context_library_get_delta_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn get_current_weight(
        context: &crate::bindings::engine::FAnimUpdateContext,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_execution_context_library_get_current_weight,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimUpdateContext>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimExecutionContextLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_execution_context_library_get_current_weight,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn get_anim_node_reference(
        instance: UPtr<crate::bindings::engine::UAnimInstance>,
        index: i32,
    ) -> crate::bindings::engine::FAnimNodeReference {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_execution_context_library_get_anim_node_reference,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimInstance>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimExecutionContextLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_execution_context_library_get_anim_node_reference,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::engine::FAnimNodeReference>().read()
        }
    }
    pub fn get_anim_instance(
        context: &crate::bindings::engine::FAnimExecutionContext,
    ) -> UPtr<crate::bindings::engine::UAnimInstance> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_execution_context_library_get_anim_instance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimExecutionContext>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimExecutionContextLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_execution_context_library_get_anim_instance,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::engine::UAnimInstance>>()
                .read()
        }
    }
    pub fn convert_to_update_context(
        context: &crate::bindings::engine::FAnimExecutionContext,
        result: &mut crate::bindings::engine::EAnimExecutionContextConversionResult,
    ) -> crate::bindings::engine::FAnimUpdateContext {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_execution_context_library_convert_to_update_context,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimExecutionContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::engine::EAnimExecutionContextConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimExecutionContextLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_execution_context_library_convert_to_update_context,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimExecutionContextConversionResult>()
                .swap(result);
        }
        unsafe {
            __buffer.add(24).cast::<crate::bindings::engine::FAnimUpdateContext>().read()
        }
    }
    pub fn convert_to_pose_context(
        context: &crate::bindings::engine::FAnimExecutionContext,
        result: &mut crate::bindings::engine::EAnimExecutionContextConversionResult,
    ) -> crate::bindings::engine::FAnimPoseContext {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_execution_context_library_convert_to_pose_context,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimExecutionContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::engine::EAnimExecutionContextConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimExecutionContextLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_execution_context_library_convert_to_pose_context,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimExecutionContextConversionResult>()
                .swap(result);
        }
        unsafe {
            __buffer.add(24).cast::<crate::bindings::engine::FAnimPoseContext>().read()
        }
    }
    pub fn convert_to_initialization_context(
        context: &crate::bindings::engine::FAnimExecutionContext,
        result: &mut crate::bindings::engine::EAnimExecutionContextConversionResult,
    ) -> crate::bindings::engine::FAnimInitializationContext {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_execution_context_library_convert_to_initialization_context,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimExecutionContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::engine::EAnimExecutionContextConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimExecutionContextLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_execution_context_library_convert_to_initialization_context,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimExecutionContextConversionResult>()
                .swap(result);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::engine::FAnimInitializationContext>()
                .read()
        }
    }
    pub fn convert_to_component_space_pose_context(
        context: &crate::bindings::engine::FAnimExecutionContext,
        result: &mut crate::bindings::engine::EAnimExecutionContextConversionResult,
    ) -> crate::bindings::engine::FAnimComponentSpacePoseContext {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_execution_context_library_convert_to_component_space_pose_context,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimExecutionContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::engine::EAnimExecutionContextConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimExecutionContextLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_execution_context_library_convert_to_component_space_pose_context,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimExecutionContextConversionResult>()
                .swap(result);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::engine::FAnimComponentSpacePoseContext>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimNotify_PlayMontageNotify {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub notify_name: FName,
}
impl UAnimNotify_PlayMontageNotify {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotify_PlayMontageNotify")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotify_PlayMontageNotify")
            .copied()
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
pub struct UAnimNotify_PlayMontageNotifyWindow {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub notify_name: FName,
}
impl UAnimNotify_PlayMontageNotifyWindow {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotify_PlayMontageNotifyWindow")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotify_PlayMontageNotifyWindow")
            .copied()
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
pub struct UAnimSequencerInstance {
    __padding_end: [u8; 1136],
}
impl UAnimSequencerInstance {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSequencerInstance")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSequencerInstance")
            .copied()
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
pub struct UBlendListBaseLibrary {
    __padding_end: [u8; 48],
}
impl UBlendListBaseLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendListBaseLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendListBaseLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn reset_node(blend_list_base: &FBlendListBaseReference) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_list_base_library_reset_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_list_base,
                __buffer.add(0).cast::<FBlendListBaseReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendListBaseLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_list_base_library_reset_node,
                __buffer,
            )
        };
    }
    pub fn convert_to_blend_list_base(
        node: &crate::bindings::engine::FAnimNodeReference,
        result: &mut crate::bindings::engine::EAnimNodeReferenceConversionResult,
    ) -> FBlendListBaseReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_list_base_library_convert_to_blend_list_base,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::engine::EAnimNodeReferenceConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendListBaseLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_list_base_library_convert_to_blend_list_base,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimNodeReferenceConversionResult>()
                .swap(result);
        }
        unsafe { __buffer.add(24).cast::<FBlendListBaseReference>().read() }
    }
}
#[repr(C, align(8))]
pub struct UBlendSpaceLibrary {
    __padding_end: [u8; 48],
}
impl UBlendSpaceLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendSpaceLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendSpaceLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn snap_to_position(
        blend_space: &FBlendSpaceReference,
        new_position: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_library_snap_to_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_space,
                __buffer.add(0).cast::<FBlendSpaceReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendSpaceLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_library_snap_to_position,
                __buffer,
            )
        };
    }
    pub fn get_position(
        blend_space: &FBlendSpaceReference,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_library_get_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_space,
                __buffer.add(0).cast::<FBlendSpaceReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendSpaceLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_library_get_position,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_filtered_position(
        blend_space: &FBlendSpaceReference,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_library_get_filtered_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_space,
                __buffer.add(0).cast::<FBlendSpaceReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendSpaceLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_library_get_filtered_position,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn convert_to_blend_space_pure(
        node: &crate::bindings::engine::FAnimNodeReference,
        blend_space: &mut FBlendSpaceReference,
        result: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_library_convert_to_blend_space_pure,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_space,
                __buffer.add(16).cast::<FBlendSpaceReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(result, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendSpaceLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_library_convert_to_blend_space_pure,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FBlendSpaceReference>().swap(blend_space);
        }
        unsafe {
            __buffer.add(32).cast::<bool>().swap(result);
        }
    }
    pub fn convert_to_blend_space(
        node: &crate::bindings::engine::FAnimNodeReference,
        result: &mut crate::bindings::engine::EAnimNodeReferenceConversionResult,
    ) -> FBlendSpaceReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_library_convert_to_blend_space,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::engine::EAnimNodeReferenceConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendSpaceLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_library_convert_to_blend_space,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimNodeReferenceConversionResult>()
                .swap(result);
        }
        unsafe { __buffer.add(24).cast::<FBlendSpaceReference>().read() }
    }
}
#[repr(C, align(8))]
pub struct UBlendSpacePlayerLibrary {
    __padding_end: [u8; 48],
}
impl UBlendSpacePlayerLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendSpacePlayerLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendSpacePlayerLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn snap_to_position(
        blend_space_player: &FBlendSpacePlayerReference,
        new_position: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_snap_to_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_space_player,
                __buffer.add(0).cast::<FBlendSpacePlayerReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendSpacePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_snap_to_position,
                __buffer,
            )
        };
    }
    pub fn should_reset_play_time_when_blend_space_changes(
        blend_space_player: &FBlendSpacePlayerReference,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_should_reset_play_time_when_blend_space_changes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_space_player,
                __buffer.add(0).cast::<FBlendSpacePlayerReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendSpacePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_should_reset_play_time_when_blend_space_changes,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_reset_play_time_when_blend_space_changes(
        blend_space_player: &FBlendSpacePlayerReference,
        b_reset: bool,
    ) -> FBlendSpacePlayerReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_set_reset_play_time_when_blend_space_changes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_space_player,
                __buffer.add(0).cast::<FBlendSpacePlayerReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_reset, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendSpacePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_set_reset_play_time_when_blend_space_changes,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FBlendSpacePlayerReference>().read() }
    }
    pub fn set_play_rate(
        blend_space_player: &FBlendSpacePlayerReference,
        play_rate: f32,
    ) -> FBlendSpacePlayerReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_set_play_rate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_space_player,
                __buffer.add(0).cast::<FBlendSpacePlayerReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&play_rate, __buffer.add(16).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendSpacePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_set_play_rate,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FBlendSpacePlayerReference>().read() }
    }
    pub fn set_loop(
        blend_space_player: &FBlendSpacePlayerReference,
        b_loop: bool,
    ) -> FBlendSpacePlayerReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_set_loop,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_space_player,
                __buffer.add(0).cast::<FBlendSpacePlayerReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_loop, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendSpacePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_set_loop,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FBlendSpacePlayerReference>().read() }
    }
    pub fn set_blend_space_with_inertial_blending(
        update_context: &crate::bindings::engine::FAnimUpdateContext,
        blend_space_player: &FBlendSpacePlayerReference,
        blend_space: UPtr<crate::bindings::engine::UBlendSpace>,
        blend_time: f32,
    ) -> FBlendSpacePlayerReference {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_set_blend_space_with_inertial_blending,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                update_context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimUpdateContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_space_player,
                __buffer.add(16).cast::<FBlendSpacePlayerReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_space,
                __buffer.add(32).cast::<UPtr<crate::bindings::engine::UBlendSpace>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_time,
                __buffer.add(40).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendSpacePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_set_blend_space_with_inertial_blending,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<FBlendSpacePlayerReference>().read() }
    }
    pub fn set_blend_space(
        blend_space_player: &FBlendSpacePlayerReference,
        blend_space: UPtr<crate::bindings::engine::UBlendSpace>,
    ) -> FBlendSpacePlayerReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_set_blend_space,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_space_player,
                __buffer.add(0).cast::<FBlendSpacePlayerReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_space,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UBlendSpace>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendSpacePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_set_blend_space,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FBlendSpacePlayerReference>().read() }
    }
    pub fn get_start_position(blend_space_player: &FBlendSpacePlayerReference) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_get_start_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_space_player,
                __buffer.add(0).cast::<FBlendSpacePlayerReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendSpacePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_get_start_position,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn get_position(
        blend_space_player: &FBlendSpacePlayerReference,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_get_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_space_player,
                __buffer.add(0).cast::<FBlendSpacePlayerReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendSpacePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_get_position,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_play_rate(blend_space_player: &FBlendSpacePlayerReference) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_get_play_rate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_space_player,
                __buffer.add(0).cast::<FBlendSpacePlayerReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendSpacePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_get_play_rate,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn get_loop(blend_space_player: &FBlendSpacePlayerReference) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_get_loop,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_space_player,
                __buffer.add(0).cast::<FBlendSpacePlayerReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendSpacePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_get_loop,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_blend_space(
        blend_space_player: &FBlendSpacePlayerReference,
    ) -> UPtr<crate::bindings::engine::UBlendSpace> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_get_blend_space,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_space_player,
                __buffer.add(0).cast::<FBlendSpacePlayerReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendSpacePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_get_blend_space,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<UPtr<crate::bindings::engine::UBlendSpace>>().read()
        }
    }
    pub fn convert_to_blend_space_player_pure(
        node: &crate::bindings::engine::FAnimNodeReference,
        blend_space_player: &mut FBlendSpacePlayerReference,
        result: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_convert_to_blend_space_player_pure,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_space_player,
                __buffer.add(16).cast::<FBlendSpacePlayerReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(result, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendSpacePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_convert_to_blend_space_player_pure,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<FBlendSpacePlayerReference>()
                .swap(blend_space_player);
        }
        unsafe {
            __buffer.add(32).cast::<bool>().swap(result);
        }
    }
    pub fn convert_to_blend_space_player(
        node: &crate::bindings::engine::FAnimNodeReference,
        result: &mut crate::bindings::engine::EAnimNodeReferenceConversionResult,
    ) -> FBlendSpacePlayerReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_convert_to_blend_space_player,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::engine::EAnimNodeReferenceConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UBlendSpacePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_blend_space_player_library_convert_to_blend_space_player,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimNodeReferenceConversionResult>()
                .swap(result);
        }
        unsafe { __buffer.add(24).cast::<FBlendSpacePlayerReference>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAnimNodeRigidBodyLibrary {
    __padding_end: [u8; 48],
}
impl UAnimNodeRigidBodyLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNodeRigidBodyLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNodeRigidBodyLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_override_physics_asset(
        node: &FRigidBodyAnimNodeReference,
        physics_asset: UPtr<crate::bindings::engine::UPhysicsAsset>,
    ) -> FRigidBodyAnimNodeReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_node_rigid_body_library_set_override_physics_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<FRigidBodyAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &physics_asset,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UPhysicsAsset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimNodeRigidBodyLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_node_rigid_body_library_set_override_physics_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FRigidBodyAnimNodeReference>().read() }
    }
    pub fn convert_to_rigid_body_anim_node_pure(
        node: &crate::bindings::engine::FAnimNodeReference,
        rigid_body_anim_node: &mut FRigidBodyAnimNodeReference,
        result: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_node_rigid_body_library_convert_to_rigid_body_anim_node_pure,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                rigid_body_anim_node,
                __buffer.add(16).cast::<FRigidBodyAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(result, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimNodeRigidBodyLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_node_rigid_body_library_convert_to_rigid_body_anim_node_pure,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<FRigidBodyAnimNodeReference>()
                .swap(rigid_body_anim_node);
        }
        unsafe {
            __buffer.add(32).cast::<bool>().swap(result);
        }
    }
    pub fn convert_to_rigid_body_anim_node(
        node: &crate::bindings::engine::FAnimNodeReference,
        result: &mut crate::bindings::engine::EAnimNodeReferenceConversionResult,
    ) -> FRigidBodyAnimNodeReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_node_rigid_body_library_convert_to_rigid_body_anim_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::engine::EAnimNodeReferenceConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UAnimNodeRigidBodyLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_anim_node_rigid_body_library_convert_to_rigid_body_anim_node,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimNodeReferenceConversionResult>()
                .swap(result);
        }
        unsafe { __buffer.add(24).cast::<FRigidBodyAnimNodeReference>().read() }
    }
}
#[repr(C, align(8))]
pub struct UKismetAnimationLibrary {
    __padding_end: [u8; 48],
}
impl UKismetAnimationLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UKismetAnimationLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UKismetAnimationLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn two_bone_ik(
        root_pos: &crate::bindings::core_u_object::FVector,
        joint_pos: &crate::bindings::core_u_object::FVector,
        end_pos: &crate::bindings::core_u_object::FVector,
        joint_target: &crate::bindings::core_u_object::FVector,
        effector: &crate::bindings::core_u_object::FVector,
        out_joint_pos: &mut crate::bindings::core_u_object::FVector,
        out_end_pos: &mut crate::bindings::core_u_object::FVector,
        b_allow_stretching: bool,
        start_stretch_ratio: f32,
        max_stretch_scale: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<180>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_k2_two_bone_ik,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                root_pos,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                joint_pos,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                end_pos,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                joint_target,
                __buffer.add(72).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                effector,
                __buffer.add(96).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_joint_pos,
                __buffer.add(120).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_end_pos,
                __buffer.add(144).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_allow_stretching,
                __buffer.add(168).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_stretch_ratio,
                __buffer.add(172).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_stretch_scale,
                __buffer.add(176).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UKismetAnimationLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_k2_two_bone_ik,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(120)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_joint_pos);
        }
        unsafe {
            __buffer
                .add(144)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_end_pos);
        }
    }
    pub fn k2_start_profiling_timer() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_k2_start_profiling_timer,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::anim_graph_runtime::UKismetAnimationLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_k2_start_profiling_timer,
                __buffer,
            )
        };
    }
    pub fn make_vector_from_perlin_noise(
        x: f32,
        y: f32,
        z: f32,
        range_out_min_x: f32,
        range_out_max_x: f32,
        range_out_min_y: f32,
        range_out_max_y: f32,
        range_out_min_z: f32,
        range_out_max_z: f32,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_k2_make_perlin_noise_vector_and_remap,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&x, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&y, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&z, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &range_out_min_x,
                __buffer.add(12).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &range_out_max_x,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &range_out_min_y,
                __buffer.add(20).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &range_out_max_y,
                __buffer.add(24).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &range_out_min_z,
                __buffer.add(28).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &range_out_max_z,
                __buffer.add(32).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UKismetAnimationLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_k2_make_perlin_noise_vector_and_remap,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn make_float_from_perlin_noise(
        value: f32,
        range_out_min: f32,
        range_out_max: f32,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_k2_make_perlin_noise_and_remap,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &range_out_min,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &range_out_max,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UKismetAnimationLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_k2_make_perlin_noise_and_remap,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<f32>().read() }
    }
    pub fn look_at(
        current_transform: &crate::bindings::core_u_object::FTransform,
        target_position: &crate::bindings::core_u_object::FVector,
        look_at_vector: crate::bindings::core_u_object::FVector,
        b_use_up_vector: bool,
        up_vector: crate::bindings::core_u_object::FVector,
        clamp_cone_in_degree: f32,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<288>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_k2_look_at,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                current_transform,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_position,
                __buffer.add(96).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &look_at_vector,
                __buffer.add(120).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_up_vector,
                __buffer.add(144).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &up_vector,
                __buffer.add(152).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &clamp_cone_in_degree,
                __buffer.add(176).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UKismetAnimationLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_k2_look_at,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(192).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn k2_end_profiling_timer(b_log: bool, log_prefix: FString) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_k2_end_profiling_timer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_log, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &log_prefix,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UKismetAnimationLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_k2_end_profiling_timer,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<f32>().read() }
    }
    pub fn distance_between_sockets(
        component: UPtr<crate::bindings::engine::USkeletalMeshComponent>,
        socket_or_bone_name_a: FName,
        socket_space_a: crate::bindings::engine::ERelativeTransformSpace,
        socket_or_bone_name_b: FName,
        socket_space_b: crate::bindings::engine::ERelativeTransformSpace,
        b_remap_range: bool,
        in_range_min: f32,
        in_range_max: f32,
        out_range_min: f32,
        out_range_max: f32,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_k2_distance_between_two_sockets_and_map_range,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::USkeletalMeshComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &socket_or_bone_name_a,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &socket_space_a,
                __buffer
                    .add(20)
                    .cast::<crate::bindings::engine::ERelativeTransformSpace>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &socket_or_bone_name_b,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &socket_space_b,
                __buffer
                    .add(36)
                    .cast::<crate::bindings::engine::ERelativeTransformSpace>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_remap_range,
                __buffer.add(37).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_range_min,
                __buffer.add(40).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_range_max,
                __buffer.add(44).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &out_range_min,
                __buffer.add(48).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &out_range_max,
                __buffer.add(52).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UKismetAnimationLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_k2_distance_between_two_sockets_and_map_range,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<f32>().read() }
    }
    pub fn direction_between_sockets(
        component: UPtr<crate::bindings::engine::USkeletalMeshComponent>,
        socket_or_bone_name_from: FName,
        socket_or_bone_name_to: FName,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_k2_direction_between_sockets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::USkeletalMeshComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &socket_or_bone_name_from,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &socket_or_bone_name_to,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UKismetAnimationLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_k2_direction_between_sockets,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn calculate_velocity_from_sockets(
        delta_seconds: f32,
        component: UPtr<crate::bindings::engine::USkeletalMeshComponent>,
        socket_or_bone_name: FName,
        reference_socket_or_bone: FName,
        socket_space: crate::bindings::engine::ERelativeTransformSpace,
        offset_in_bone_space: crate::bindings::core_u_object::FVector,
        history: &mut FPositionHistory,
        number_of_samples: i32,
        velocity_min: f32,
        velocity_max: f32,
        easing_type: EEasingFuncType,
        custom_curve: &crate::bindings::engine::FRuntimeFloatCurve,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<276>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_k2_calculate_velocity_from_sockets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::USkeletalMeshComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &socket_or_bone_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &reference_socket_or_bone,
                __buffer.add(28).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &socket_space,
                __buffer
                    .add(40)
                    .cast::<crate::bindings::engine::ERelativeTransformSpace>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &offset_in_bone_space,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                history,
                __buffer.add(72).cast::<FPositionHistory>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &number_of_samples,
                __buffer.add(120).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &velocity_min,
                __buffer.add(124).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &velocity_max,
                __buffer.add(128).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &easing_type,
                __buffer.add(132).cast::<EEasingFuncType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                custom_curve,
                __buffer.add(136).cast::<crate::bindings::engine::FRuntimeFloatCurve>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UKismetAnimationLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_k2_calculate_velocity_from_sockets,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<FPositionHistory>().swap(history);
        }
        unsafe { __buffer.add(272).cast::<f32>().read() }
    }
    pub fn calculate_velocity_from_position_history(
        delta_seconds: f32,
        position: crate::bindings::core_u_object::FVector,
        history: &mut FPositionHistory,
        number_of_samples: i32,
        velocity_min: f32,
        velocity_max: f32,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_k2_calculate_velocity_from_position_history,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                history,
                __buffer.add(32).cast::<FPositionHistory>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &number_of_samples,
                __buffer.add(80).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &velocity_min,
                __buffer.add(84).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &velocity_max,
                __buffer.add(88).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UKismetAnimationLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_k2_calculate_velocity_from_position_history,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<FPositionHistory>().swap(history);
        }
        unsafe { __buffer.add(92).cast::<f32>().read() }
    }
    pub fn calculate_direction(
        velocity: &crate::bindings::core_u_object::FVector,
        base_rotation: &crate::bindings::core_u_object::FRotator,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<52>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_calculate_direction,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                velocity,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                base_rotation,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UKismetAnimationLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_kismet_animation_library_calculate_direction,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct ULayeredBoneBlendLibrary {
    __padding_end: [u8; 48],
}
impl ULayeredBoneBlendLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULayeredBoneBlendLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULayeredBoneBlendLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_blend_mask(
        update_context: &crate::bindings::engine::FAnimUpdateContext,
        layered_bone_blend: &FLayeredBoneBlendReference,
        pose_index: i32,
        blend_mask_name: FName,
    ) -> FLayeredBoneBlendReference {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_layered_bone_blend_library_set_blend_mask,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                update_context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimUpdateContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                layered_bone_blend,
                __buffer.add(16).cast::<FLayeredBoneBlendReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pose_index,
                __buffer.add(32).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_mask_name,
                __buffer.add(36).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::ULayeredBoneBlendLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_layered_bone_blend_library_set_blend_mask,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<FLayeredBoneBlendReference>().read() }
    }
    pub fn get_num_poses(layered_bone_blend: &FLayeredBoneBlendReference) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_layered_bone_blend_library_get_num_poses,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layered_bone_blend,
                __buffer.add(0).cast::<FLayeredBoneBlendReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::ULayeredBoneBlendLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_layered_bone_blend_library_get_num_poses,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn convert_to_layered_bone_blend(
        node: &crate::bindings::engine::FAnimNodeReference,
        result: &mut crate::bindings::engine::EAnimNodeReferenceConversionResult,
    ) -> FLayeredBoneBlendReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_layered_bone_blend_library_convert_to_layered_bone_blend,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::engine::EAnimNodeReferenceConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::ULayeredBoneBlendLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_layered_bone_blend_library_convert_to_layered_bone_blend,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimNodeReferenceConversionResult>()
                .swap(result);
        }
        unsafe { __buffer.add(24).cast::<FLayeredBoneBlendReference>().read() }
    }
    pub fn convert_to_layered_blend_per_bone_pure(
        node: &crate::bindings::engine::FAnimNodeReference,
        layered_bone_blend: &mut FLayeredBoneBlendReference,
        result: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_layered_bone_blend_library_convert_to_layered_blend_per_bone_pure,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                layered_bone_blend,
                __buffer.add(16).cast::<FLayeredBoneBlendReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(result, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::ULayeredBoneBlendLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_layered_bone_blend_library_convert_to_layered_blend_per_bone_pure,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<FLayeredBoneBlendReference>()
                .swap(layered_bone_blend);
        }
        unsafe {
            __buffer.add(32).cast::<bool>().swap(result);
        }
    }
}
#[repr(C, align(8))]
pub struct ULinkedAnimGraphLibrary {
    __padding_end: [u8; 48],
}
impl ULinkedAnimGraphLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULinkedAnimGraphLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULinkedAnimGraphLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn has_linked_anim_instance(node: &FLinkedAnimGraphReference) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_linked_anim_graph_library_has_linked_anim_instance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<FLinkedAnimGraphReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::ULinkedAnimGraphLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_linked_anim_graph_library_has_linked_anim_instance,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_linked_anim_instance(
        node: &FLinkedAnimGraphReference,
    ) -> UPtr<crate::bindings::engine::UAnimInstance> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_linked_anim_graph_library_get_linked_anim_instance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<FLinkedAnimGraphReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::ULinkedAnimGraphLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_linked_anim_graph_library_get_linked_anim_instance,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::engine::UAnimInstance>>()
                .read()
        }
    }
    pub fn convert_to_linked_anim_graph_pure(
        node: &crate::bindings::engine::FAnimNodeReference,
        linked_anim_graph: &mut FLinkedAnimGraphReference,
        result: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_linked_anim_graph_library_convert_to_linked_anim_graph_pure,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                linked_anim_graph,
                __buffer.add(16).cast::<FLinkedAnimGraphReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(result, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::ULinkedAnimGraphLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_linked_anim_graph_library_convert_to_linked_anim_graph_pure,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FLinkedAnimGraphReference>().swap(linked_anim_graph);
        }
        unsafe {
            __buffer.add(32).cast::<bool>().swap(result);
        }
    }
    pub fn convert_to_linked_anim_graph(
        node: &crate::bindings::engine::FAnimNodeReference,
        result: &mut crate::bindings::engine::EAnimNodeReferenceConversionResult,
    ) -> FLinkedAnimGraphReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_linked_anim_graph_library_convert_to_linked_anim_graph,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::engine::EAnimNodeReferenceConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::ULinkedAnimGraphLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_linked_anim_graph_library_convert_to_linked_anim_graph,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimNodeReferenceConversionResult>()
                .swap(result);
        }
        unsafe { __buffer.add(24).cast::<FLinkedAnimGraphReference>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMirrorAnimLibrary {
    __padding_end: [u8; 48],
}
impl UMirrorAnimLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMirrorAnimLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMirrorAnimLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_mirror_transition_blend_time(
        mirror_node: &FMirrorAnimNodeReference,
        in_blend_time: f32,
    ) -> FMirrorAnimNodeReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_mirror_anim_library_set_mirror_transition_blend_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                mirror_node,
                __buffer.add(0).cast::<FMirrorAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_blend_time,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UMirrorAnimLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_mirror_anim_library_set_mirror_transition_blend_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FMirrorAnimNodeReference>().read() }
    }
    pub fn set_mirror(
        mirror_node: &FMirrorAnimNodeReference,
        b_in_mirror: bool,
    ) -> FMirrorAnimNodeReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_mirror_anim_library_set_mirror,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                mirror_node,
                __buffer.add(0).cast::<FMirrorAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_mirror,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UMirrorAnimLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_mirror_anim_library_set_mirror,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FMirrorAnimNodeReference>().read() }
    }
    pub fn get_mirror_transition_blend_time(
        mirror_node: &FMirrorAnimNodeReference,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_mirror_anim_library_get_mirror_transition_blend_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                mirror_node,
                __buffer.add(0).cast::<FMirrorAnimNodeReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UMirrorAnimLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_mirror_anim_library_get_mirror_transition_blend_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn get_mirror_data_table(
        mirror_node: &FMirrorAnimNodeReference,
    ) -> UPtr<crate::bindings::engine::UMirrorDataTable> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_mirror_anim_library_get_mirror_data_table,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                mirror_node,
                __buffer.add(0).cast::<FMirrorAnimNodeReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UMirrorAnimLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_mirror_anim_library_get_mirror_data_table,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::engine::UMirrorDataTable>>()
                .read()
        }
    }
    pub fn get_mirror(mirror_node: &FMirrorAnimNodeReference) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_mirror_anim_library_get_mirror,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                mirror_node,
                __buffer.add(0).cast::<FMirrorAnimNodeReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UMirrorAnimLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_mirror_anim_library_get_mirror,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn convert_to_mirror_node_pure(
        node: &crate::bindings::engine::FAnimNodeReference,
        mirror_node: &mut FMirrorAnimNodeReference,
        result: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_mirror_anim_library_convert_to_mirror_node_pure,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                mirror_node,
                __buffer.add(16).cast::<FMirrorAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(result, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UMirrorAnimLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_mirror_anim_library_convert_to_mirror_node_pure,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FMirrorAnimNodeReference>().swap(mirror_node);
        }
        unsafe {
            __buffer.add(32).cast::<bool>().swap(result);
        }
    }
    pub fn convert_to_mirror_node(
        node: &crate::bindings::engine::FAnimNodeReference,
        result: &mut crate::bindings::engine::EAnimNodeReferenceConversionResult,
    ) -> FMirrorAnimNodeReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_mirror_anim_library_convert_to_mirror_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::engine::EAnimNodeReferenceConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UMirrorAnimLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_mirror_anim_library_convert_to_mirror_node,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimNodeReferenceConversionResult>()
                .swap(result);
        }
        unsafe { __buffer.add(24).cast::<FMirrorAnimNodeReference>().read() }
    }
}
#[repr(C, align(8))]
pub struct UModifyCurveAnimLibrary {
    __padding_end: [u8; 48],
}
impl UModifyCurveAnimLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModifyCurveAnimLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModifyCurveAnimLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_curve_map(
        modify_curve_node: &FModifyCurveAnimNodeReference,
        in_curve_map: &TMap<FName, f32>,
    ) -> FModifyCurveAnimNodeReference {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_modify_curve_anim_library_set_curve_map,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                modify_curve_node,
                __buffer.add(0).cast::<FModifyCurveAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_curve_map,
                __buffer.add(16).cast::<TMap<FName, f32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UModifyCurveAnimLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_modify_curve_anim_library_set_curve_map,
                __buffer,
            )
        };
        unsafe { __buffer.add(96).cast::<FModifyCurveAnimNodeReference>().read() }
    }
    pub fn set_apply_mode(
        modify_curve_node: &FModifyCurveAnimNodeReference,
        in_mode: EModifyCurveApplyMode,
    ) -> FModifyCurveAnimNodeReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_modify_curve_anim_library_set_apply_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                modify_curve_node,
                __buffer.add(0).cast::<FModifyCurveAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_mode,
                __buffer.add(16).cast::<EModifyCurveApplyMode>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UModifyCurveAnimLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_modify_curve_anim_library_set_apply_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FModifyCurveAnimNodeReference>().read() }
    }
    pub fn set_alpha(
        modify_curve_node: &FModifyCurveAnimNodeReference,
        in_alpha: f32,
    ) -> FModifyCurveAnimNodeReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_modify_curve_anim_library_set_alpha,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                modify_curve_node,
                __buffer.add(0).cast::<FModifyCurveAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_alpha, __buffer.add(16).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UModifyCurveAnimLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_modify_curve_anim_library_set_alpha,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FModifyCurveAnimNodeReference>().read() }
    }
    pub fn get_apply_mode(
        modify_curve_node: &FModifyCurveAnimNodeReference,
    ) -> EModifyCurveApplyMode {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_modify_curve_anim_library_get_apply_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                modify_curve_node,
                __buffer.add(0).cast::<FModifyCurveAnimNodeReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UModifyCurveAnimLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_modify_curve_anim_library_get_apply_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<EModifyCurveApplyMode>().read() }
    }
    pub fn get_alpha(modify_curve_node: &FModifyCurveAnimNodeReference) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_modify_curve_anim_library_get_alpha,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                modify_curve_node,
                __buffer.add(0).cast::<FModifyCurveAnimNodeReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UModifyCurveAnimLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_modify_curve_anim_library_get_alpha,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn convert_to_modify_curve_node_pure(
        node: &crate::bindings::engine::FAnimNodeReference,
        modify_curve_node: &mut FModifyCurveAnimNodeReference,
        result: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_modify_curve_anim_library_convert_to_modify_curve_node_pure,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                modify_curve_node,
                __buffer.add(16).cast::<FModifyCurveAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(result, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UModifyCurveAnimLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_modify_curve_anim_library_convert_to_modify_curve_node_pure,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<FModifyCurveAnimNodeReference>()
                .swap(modify_curve_node);
        }
        unsafe {
            __buffer.add(32).cast::<bool>().swap(result);
        }
    }
    pub fn convert_to_modify_curve_node(
        node: &crate::bindings::engine::FAnimNodeReference,
        result: &mut crate::bindings::engine::EAnimNodeReferenceConversionResult,
    ) -> FModifyCurveAnimNodeReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_modify_curve_anim_library_convert_to_modify_curve_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::engine::EAnimNodeReferenceConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UModifyCurveAnimLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_modify_curve_anim_library_convert_to_modify_curve_node,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimNodeReferenceConversionResult>()
                .swap(result);
        }
        unsafe { __buffer.add(24).cast::<FModifyCurveAnimNodeReference>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPlayMontageCallbackProxy {
    __padding_end: [u8; 232],
}
impl UPlayMontageCallbackProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlayMontageCallbackProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlayMontageCallbackProxy")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn create_proxy_object_for_play_montage(
        in_skeletal_mesh_component: UPtr<
            crate::bindings::engine::USkeletalMeshComponent,
        >,
        montage_to_play: UPtr<crate::bindings::engine::UAnimMontage>,
        play_rate: f32,
        starting_position: f32,
        starting_section: FName,
        b_should_stop_all_montages: bool,
    ) -> UPtr<UPlayMontageCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_play_montage_callback_proxy_create_proxy_object_for_play_montage,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_skeletal_mesh_component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::USkeletalMeshComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &montage_to_play,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UAnimMontage>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&play_rate, __buffer.add(16).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &starting_position,
                __buffer.add(20).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &starting_section,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_should_stop_all_montages,
                __buffer.add(36).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::UPlayMontageCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_play_montage_callback_proxy_create_proxy_object_for_play_montage,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<UPtr<UPlayMontageCallbackProxy>>().read() }
    }
}
#[repr(C, align(8))]
pub struct USequenceEvaluatorLibrary {
    __padding_end: [u8; 48],
}
impl USequenceEvaluatorLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequenceEvaluatorLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequenceEvaluatorLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_sequence_with_inertial_blending(
        update_context: &crate::bindings::engine::FAnimUpdateContext,
        sequence_evaluator: &FSequenceEvaluatorReference,
        sequence: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        blend_time: f32,
    ) -> FSequenceEvaluatorReference {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_evaluator_library_set_sequence_with_inertial_blending,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                update_context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimUpdateContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_evaluator,
                __buffer.add(16).cast::<FSequenceEvaluatorReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(32)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_time,
                __buffer.add(40).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequenceEvaluatorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_evaluator_library_set_sequence_with_inertial_blending,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<FSequenceEvaluatorReference>().read() }
    }
    pub fn set_sequence(
        sequence_evaluator: &FSequenceEvaluatorReference,
        sequence: UPtr<crate::bindings::engine::UAnimSequenceBase>,
    ) -> FSequenceEvaluatorReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_evaluator_library_set_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_evaluator,
                __buffer.add(0).cast::<FSequenceEvaluatorReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequenceEvaluatorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_evaluator_library_set_sequence,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FSequenceEvaluatorReference>().read() }
    }
    pub fn set_explicit_time(
        sequence_evaluator: &FSequenceEvaluatorReference,
        time: f32,
    ) -> FSequenceEvaluatorReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_evaluator_library_set_explicit_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_evaluator,
                __buffer.add(0).cast::<FSequenceEvaluatorReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(16).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequenceEvaluatorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_evaluator_library_set_explicit_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FSequenceEvaluatorReference>().read() }
    }
    pub fn set_explicit_frame(
        sequence_evaluator: &FSequenceEvaluatorReference,
        frame: i32,
    ) -> FSequenceEvaluatorReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_evaluator_library_set_explicit_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_evaluator,
                __buffer.add(0).cast::<FSequenceEvaluatorReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&frame, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequenceEvaluatorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_evaluator_library_set_explicit_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FSequenceEvaluatorReference>().read() }
    }
    pub fn get_sequence(
        sequence_evaluator: &FSequenceEvaluatorReference,
    ) -> UPtr<crate::bindings::engine::UAnimSequenceBase> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_evaluator_library_get_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_evaluator,
                __buffer.add(0).cast::<FSequenceEvaluatorReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequenceEvaluatorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_evaluator_library_get_sequence,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>()
                .read()
        }
    }
    pub fn get_accumulated_time(
        sequence_evaluator: &FSequenceEvaluatorReference,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_evaluator_library_get_accumulated_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_evaluator,
                __buffer.add(0).cast::<FSequenceEvaluatorReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequenceEvaluatorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_evaluator_library_get_accumulated_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn convert_to_sequence_evaluator_pure(
        node: &crate::bindings::engine::FAnimNodeReference,
        sequence_evaluator: &mut FSequenceEvaluatorReference,
        result: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_evaluator_library_convert_to_sequence_evaluator_pure,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_evaluator,
                __buffer.add(16).cast::<FSequenceEvaluatorReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(result, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequenceEvaluatorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_evaluator_library_convert_to_sequence_evaluator_pure,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<FSequenceEvaluatorReference>()
                .swap(sequence_evaluator);
        }
        unsafe {
            __buffer.add(32).cast::<bool>().swap(result);
        }
    }
    pub fn convert_to_sequence_evaluator(
        node: &crate::bindings::engine::FAnimNodeReference,
        result: &mut crate::bindings::engine::EAnimNodeReferenceConversionResult,
    ) -> FSequenceEvaluatorReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_evaluator_library_convert_to_sequence_evaluator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::engine::EAnimNodeReferenceConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequenceEvaluatorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_evaluator_library_convert_to_sequence_evaluator,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimNodeReferenceConversionResult>()
                .swap(result);
        }
        unsafe { __buffer.add(24).cast::<FSequenceEvaluatorReference>().read() }
    }
    pub fn advance_time(
        update_context: &crate::bindings::engine::FAnimUpdateContext,
        sequence_evaluator: &FSequenceEvaluatorReference,
        play_rate: f32,
    ) -> FSequenceEvaluatorReference {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_evaluator_library_advance_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                update_context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimUpdateContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_evaluator,
                __buffer.add(16).cast::<FSequenceEvaluatorReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&play_rate, __buffer.add(32).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequenceEvaluatorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_evaluator_library_advance_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<FSequenceEvaluatorReference>().read() }
    }
}
#[repr(C, align(8))]
pub struct USequencePlayerLibrary {
    __padding_end: [u8; 48],
}
impl USequencePlayerLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencePlayerLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencePlayerLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_start_position(
        sequence_player: &FSequencePlayerReference,
        start_position: f32,
    ) -> FSequencePlayerReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_set_start_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_player,
                __buffer.add(0).cast::<FSequencePlayerReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_position,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequencePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_set_start_position,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FSequencePlayerReference>().read() }
    }
    pub fn set_sequence_with_inertial_blending(
        update_context: &crate::bindings::engine::FAnimUpdateContext,
        sequence_player: &FSequencePlayerReference,
        sequence: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        blend_time: f32,
    ) -> FSequencePlayerReference {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_set_sequence_with_inertial_blending,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                update_context,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimUpdateContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_player,
                __buffer.add(16).cast::<FSequencePlayerReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(32)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_time,
                __buffer.add(40).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequencePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_set_sequence_with_inertial_blending,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<FSequencePlayerReference>().read() }
    }
    pub fn set_sequence(
        sequence_player: &FSequencePlayerReference,
        sequence: UPtr<crate::bindings::engine::UAnimSequenceBase>,
    ) -> FSequencePlayerReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_set_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_player,
                __buffer.add(0).cast::<FSequencePlayerReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequencePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_set_sequence,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FSequencePlayerReference>().read() }
    }
    pub fn set_play_rate(
        sequence_player: &FSequencePlayerReference,
        play_rate: f32,
    ) -> FSequencePlayerReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_set_play_rate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_player,
                __buffer.add(0).cast::<FSequencePlayerReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&play_rate, __buffer.add(16).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequencePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_set_play_rate,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FSequencePlayerReference>().read() }
    }
    pub fn set_accumulated_time(
        sequence_player: &FSequencePlayerReference,
        time: f32,
    ) -> FSequencePlayerReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_set_accumulated_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_player,
                __buffer.add(0).cast::<FSequencePlayerReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(16).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequencePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_set_accumulated_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FSequencePlayerReference>().read() }
    }
    pub fn get_start_position(sequence_player: &FSequencePlayerReference) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_get_start_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_player,
                __buffer.add(0).cast::<FSequencePlayerReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequencePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_get_start_position,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn get_sequence_pure(
        sequence_player: &FSequencePlayerReference,
    ) -> UPtr<crate::bindings::engine::UAnimSequenceBase> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_get_sequence_pure,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_player,
                __buffer.add(0).cast::<FSequencePlayerReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequencePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_get_sequence_pure,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>()
                .read()
        }
    }
    pub fn get_sequence(
        sequence_player: &FSequencePlayerReference,
        sequence_base: &mut UPtr<crate::bindings::engine::UAnimSequenceBase>,
    ) -> FSequencePlayerReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_get_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_player,
                __buffer.add(0).cast::<FSequencePlayerReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_base,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequencePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_get_sequence,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>()
                .swap(sequence_base);
        }
        unsafe { __buffer.add(24).cast::<FSequencePlayerReference>().read() }
    }
    pub fn get_play_rate(sequence_player: &FSequencePlayerReference) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_get_play_rate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_player,
                __buffer.add(0).cast::<FSequencePlayerReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequencePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_get_play_rate,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn get_loop_animation(sequence_player: &FSequencePlayerReference) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_get_loop_animation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_player,
                __buffer.add(0).cast::<FSequencePlayerReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequencePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_get_loop_animation,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_accumulated_time(sequence_player: &FSequencePlayerReference) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_get_accumulated_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_player,
                __buffer.add(0).cast::<FSequencePlayerReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequencePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_get_accumulated_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn convert_to_sequence_player_pure(
        node: &crate::bindings::engine::FAnimNodeReference,
        sequence_player: &mut FSequencePlayerReference,
        result: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_convert_to_sequence_player_pure,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_player,
                __buffer.add(16).cast::<FSequencePlayerReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(result, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequencePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_convert_to_sequence_player_pure,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FSequencePlayerReference>().swap(sequence_player);
        }
        unsafe {
            __buffer.add(32).cast::<bool>().swap(result);
        }
    }
    pub fn convert_to_sequence_player(
        node: &crate::bindings::engine::FAnimNodeReference,
        result: &mut crate::bindings::engine::EAnimNodeReferenceConversionResult,
    ) -> FSequencePlayerReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_convert_to_sequence_player,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::engine::EAnimNodeReferenceConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequencePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_convert_to_sequence_player,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimNodeReferenceConversionResult>()
                .swap(result);
        }
        unsafe { __buffer.add(24).cast::<FSequencePlayerReference>().read() }
    }
    pub fn compute_play_rate_from_duration(
        sequence_player: &FSequencePlayerReference,
        duration: f32,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_compute_play_rate_from_duration,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sequence_player,
                __buffer.add(0).cast::<FSequencePlayerReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(16).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USequencePlayerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_sequence_player_library_compute_play_rate_from_duration,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<f32>().read() }
    }
}
pub struct ISequencerAnimationSupport {}
#[repr(C, align(8))]
pub struct USequencerAnimationSupport {
    __padding_end: [u8; 48],
}
impl USequencerAnimationSupport {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerAnimationSupport")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerAnimationSupport")
            .copied()
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
pub struct USkeletalControlLibrary {
    __padding_end: [u8; 48],
}
impl USkeletalControlLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletalControlLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletalControlLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_alpha(
        skeletal_control: &FSkeletalControlReference,
        alpha: f32,
    ) -> FSkeletalControlReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_skeletal_control_library_set_alpha,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                skeletal_control,
                __buffer.add(0).cast::<FSkeletalControlReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&alpha, __buffer.add(16).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USkeletalControlLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_skeletal_control_library_set_alpha,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FSkeletalControlReference>().read() }
    }
    pub fn get_alpha(skeletal_control: &FSkeletalControlReference) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_skeletal_control_library_get_alpha,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                skeletal_control,
                __buffer.add(0).cast::<FSkeletalControlReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USkeletalControlLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_skeletal_control_library_get_alpha,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn convert_to_skeletal_control_pure(
        node: &crate::bindings::engine::FAnimNodeReference,
        skeletal_control: &mut FSkeletalControlReference,
        result: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_skeletal_control_library_convert_to_skeletal_control_pure,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                skeletal_control,
                __buffer.add(16).cast::<FSkeletalControlReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(result, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USkeletalControlLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_skeletal_control_library_convert_to_skeletal_control_pure,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FSkeletalControlReference>().swap(skeletal_control);
        }
        unsafe {
            __buffer.add(32).cast::<bool>().swap(result);
        }
    }
    pub fn convert_to_skeletal_control(
        node: &crate::bindings::engine::FAnimNodeReference,
        result: &mut crate::bindings::engine::EAnimNodeReferenceConversionResult,
    ) -> FSkeletalControlReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_skeletal_control_library_convert_to_skeletal_control,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::engine::EAnimNodeReferenceConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::anim_graph_runtime::USkeletalControlLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::anim_graph_runtime::__FUNCTION_PTRS
                    .u_skeletal_control_library_convert_to_skeletal_control,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimNodeReferenceConversionResult>()
                .swap(result);
        }
        unsafe { __buffer.add(24).cast::<FSkeletalControlReference>().read() }
    }
}
#[repr(C, align(8))]
pub struct FPlayMontageCallbackProxy_OnCompleted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlayMontageCallbackProxy_OnBlendOut {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlayMontageCallbackProxy_OnInterrupted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlayMontageCallbackProxy_OnNotifyBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlayMontageCallbackProxy_OnNotifyEnd {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ERotationComponent(pub u8);
impl ERotationComponent {
    pub const EULER_X: ERotationComponent = ERotationComponent(0);
    pub const EULER_Y: ERotationComponent = ERotationComponent(1);
    pub const EULER_Z: ERotationComponent = ERotationComponent(2);
    pub const QUATERNION_ANGLE: ERotationComponent = ERotationComponent(3);
    pub const SWING_ANGLE: ERotationComponent = ERotationComponent(4);
    pub const TWIST_ANGLE: ERotationComponent = ERotationComponent(5);
}
#[repr(transparent)]
pub struct EEasingFuncType(pub u8);
impl EEasingFuncType {
    pub const LINEAR: EEasingFuncType = EEasingFuncType(0);
    pub const SINUSOIDAL: EEasingFuncType = EEasingFuncType(1);
    pub const CUBIC: EEasingFuncType = EEasingFuncType(2);
    pub const QUADRATIC_IN_OUT: EEasingFuncType = EEasingFuncType(3);
    pub const CUBIC_IN_OUT: EEasingFuncType = EEasingFuncType(4);
    pub const HERMITE_CUBIC: EEasingFuncType = EEasingFuncType(5);
    pub const QUARTIC_IN_OUT: EEasingFuncType = EEasingFuncType(6);
    pub const QUINTIC_IN_OUT: EEasingFuncType = EEasingFuncType(7);
    pub const CIRCULAR_IN: EEasingFuncType = EEasingFuncType(8);
    pub const CIRCULAR_OUT: EEasingFuncType = EEasingFuncType(9);
    pub const CIRCULAR_IN_OUT: EEasingFuncType = EEasingFuncType(10);
    pub const EXP_IN: EEasingFuncType = EEasingFuncType(11);
    pub const EXP_OUT: EEasingFuncType = EEasingFuncType(12);
    pub const EXP_IN_OUT: EEasingFuncType = EEasingFuncType(13);
    pub const CUSTOM_CURVE: EEasingFuncType = EEasingFuncType(14);
}
#[repr(transparent)]
pub struct EBlendListTransitionType(pub u8);
impl EBlendListTransitionType {
    pub const STANDARD_BLEND: EBlendListTransitionType = EBlendListTransitionType(0);
    pub const INERTIALIZATION: EBlendListTransitionType = EBlendListTransitionType(1);
}
#[repr(transparent)]
pub struct EBlendListChildUpdateMode(pub u8);
impl EBlendListChildUpdateMode {
    pub const DEFAULT: EBlendListChildUpdateMode = EBlendListChildUpdateMode(0);
    pub const RESET_CHILD_ON_ACTIVATE: EBlendListChildUpdateMode = EBlendListChildUpdateMode(
        1,
    );
    pub const ALWAYS_TICK_CHILDREN: EBlendListChildUpdateMode = EBlendListChildUpdateMode(
        2,
    );
}
#[repr(transparent)]
pub struct EAnimFunctionCallSite(pub i32);
impl EAnimFunctionCallSite {
    pub const ON_INITIALIZE: EAnimFunctionCallSite = EAnimFunctionCallSite(0);
    pub const ON_UPDATE: EAnimFunctionCallSite = EAnimFunctionCallSite(1);
    pub const ON_BECOME_RELEVANT: EAnimFunctionCallSite = EAnimFunctionCallSite(2);
    pub const ON_EVALUATE: EAnimFunctionCallSite = EAnimFunctionCallSite(3);
    pub const ON_INITIALIZE_POST_RECURSION: EAnimFunctionCallSite = EAnimFunctionCallSite(
        4,
    );
    pub const ON_UPDATE_POST_RECURSION: EAnimFunctionCallSite = EAnimFunctionCallSite(5);
    pub const ON_BECOME_RELEVANT_POST_RECURSION: EAnimFunctionCallSite = EAnimFunctionCallSite(
        6,
    );
    pub const ON_EVALUATE_POST_RECURSION: EAnimFunctionCallSite = EAnimFunctionCallSite(
        7,
    );
    pub const ON_STARTED_BLENDING_OUT: EAnimFunctionCallSite = EAnimFunctionCallSite(8);
    pub const ON_STARTED_BLENDING_IN: EAnimFunctionCallSite = EAnimFunctionCallSite(9);
    pub const ON_FINISHED_BLENDING_OUT: EAnimFunctionCallSite = EAnimFunctionCallSite(
        10,
    );
    pub const ON_FINISHED_BLENDING_IN: EAnimFunctionCallSite = EAnimFunctionCallSite(11);
}
#[repr(transparent)]
pub struct ELayeredBoneBlendMode(pub u8);
impl ELayeredBoneBlendMode {
    pub const BRANCH_FILTER: ELayeredBoneBlendMode = ELayeredBoneBlendMode(0);
    pub const BLEND_MASK: ELayeredBoneBlendMode = ELayeredBoneBlendMode(1);
}
#[repr(transparent)]
pub struct EModifyCurveApplyMode(pub u8);
impl EModifyCurveApplyMode {
    pub const ADD: EModifyCurveApplyMode = EModifyCurveApplyMode(0);
    pub const SCALE: EModifyCurveApplyMode = EModifyCurveApplyMode(1);
    pub const BLEND: EModifyCurveApplyMode = EModifyCurveApplyMode(2);
    pub const WEIGHTED_MOVING_AVERAGE: EModifyCurveApplyMode = EModifyCurveApplyMode(3);
    pub const REMAP_CURVE: EModifyCurveApplyMode = EModifyCurveApplyMode(4);
}
#[repr(transparent)]
pub struct ERBFDistanceMethod(pub u8);
impl ERBFDistanceMethod {
    pub const EUCLIDEAN: ERBFDistanceMethod = ERBFDistanceMethod(0);
    pub const QUATERNION: ERBFDistanceMethod = ERBFDistanceMethod(1);
    pub const SWING_ANGLE: ERBFDistanceMethod = ERBFDistanceMethod(2);
    pub const TWIST_ANGLE: ERBFDistanceMethod = ERBFDistanceMethod(3);
    pub const DEFAULT_METHOD: ERBFDistanceMethod = ERBFDistanceMethod(4);
}
#[repr(transparent)]
pub struct ERBFFunctionType(pub u8);
impl ERBFFunctionType {
    pub const GAUSSIAN: ERBFFunctionType = ERBFFunctionType(0);
    pub const EXPONENTIAL: ERBFFunctionType = ERBFFunctionType(1);
    pub const LINEAR: ERBFFunctionType = ERBFFunctionType(2);
    pub const CUBIC: ERBFFunctionType = ERBFFunctionType(3);
    pub const QUINTIC: ERBFFunctionType = ERBFFunctionType(4);
    pub const DEFAULT_FUNCTION: ERBFFunctionType = ERBFFunctionType(5);
}
#[repr(transparent)]
pub struct ERBFSolverType(pub u8);
impl ERBFSolverType {
    pub const ADDITIVE: ERBFSolverType = ERBFSolverType(0);
    pub const INTERPOLATIVE: ERBFSolverType = ERBFSolverType(1);
}
#[repr(transparent)]
pub struct EPoseDriverSource(pub u8);
impl EPoseDriverSource {
    pub const ROTATION: EPoseDriverSource = EPoseDriverSource(0);
    pub const TRANSLATION: EPoseDriverSource = EPoseDriverSource(1);
}
#[repr(transparent)]
pub struct EPoseDriverOutput(pub u8);
impl EPoseDriverOutput {
    pub const DRIVE_POSES: EPoseDriverOutput = EPoseDriverOutput(0);
    pub const DRIVE_CURVES: EPoseDriverOutput = EPoseDriverOutput(1);
}
#[repr(transparent)]
pub struct EPoseDriverType(pub u8);
impl EPoseDriverType {
    pub const SWING_AND_TWIST: EPoseDriverType = EPoseDriverType(0);
    pub const SWING_ONLY: EPoseDriverType = EPoseDriverType(1);
    pub const TRANSLATION: EPoseDriverType = EPoseDriverType(2);
}
#[repr(transparent)]
pub struct ERBFNormalizeMethod(pub u8);
impl ERBFNormalizeMethod {
    pub const ONLY_NORMALIZE_ABOVE_ONE: ERBFNormalizeMethod = ERBFNormalizeMethod(0);
    pub const ALWAYS_NORMALIZE: ERBFNormalizeMethod = ERBFNormalizeMethod(1);
    pub const NORMALIZE_WITHIN_MEDIAN: ERBFNormalizeMethod = ERBFNormalizeMethod(2);
    pub const NO_NORMALIZATION: ERBFNormalizeMethod = ERBFNormalizeMethod(3);
}
#[repr(transparent)]
pub struct ESnapshotSourceMode(pub u8);
impl ESnapshotSourceMode {
    pub const NAMED_SNAPSHOT: ESnapshotSourceMode = ESnapshotSourceMode(0);
    pub const SNAPSHOT_PIN: ESnapshotSourceMode = ESnapshotSourceMode(1);
}
#[repr(transparent)]
pub struct ERefPoseType(pub u8);
impl ERefPoseType {
    pub const EIT_LOCAL_SPACE: ERefPoseType = ERefPoseType(0);
    pub const EIT_ADDITIVE: ERefPoseType = ERefPoseType(1);
}
#[repr(transparent)]
pub struct ESequenceEvalReinit(pub u8);
impl ESequenceEvalReinit {
    pub const NO_RESET: ESequenceEvalReinit = ESequenceEvalReinit(0);
    pub const START_POSITION: ESequenceEvalReinit = ESequenceEvalReinit(1);
    pub const EXPLICIT_TIME: ESequenceEvalReinit = ESequenceEvalReinit(2);
}
#[repr(transparent)]
pub struct AnimPhysLinearConstraintType(pub u8);
impl AnimPhysLinearConstraintType {
    pub const FREE: AnimPhysLinearConstraintType = AnimPhysLinearConstraintType(0);
    pub const LIMITED: AnimPhysLinearConstraintType = AnimPhysLinearConstraintType(1);
}
#[repr(transparent)]
pub struct AnimPhysAngularConstraintType(pub u8);
impl AnimPhysAngularConstraintType {
    pub const ANGULAR: AnimPhysAngularConstraintType = AnimPhysAngularConstraintType(0);
    pub const CONE: AnimPhysAngularConstraintType = AnimPhysAngularConstraintType(1);
}
#[repr(transparent)]
pub struct ESphericalLimitType(pub u8);
impl ESphericalLimitType {
    pub const INNER: ESphericalLimitType = ESphericalLimitType(0);
    pub const OUTER: ESphericalLimitType = ESphericalLimitType(1);
}
#[repr(transparent)]
pub struct AnimPhysSimSpaceType(pub u8);
impl AnimPhysSimSpaceType {
    pub const COMPONENT: AnimPhysSimSpaceType = AnimPhysSimSpaceType(0);
    pub const ACTOR: AnimPhysSimSpaceType = AnimPhysSimSpaceType(1);
    pub const WORLD: AnimPhysSimSpaceType = AnimPhysSimSpaceType(2);
    pub const ROOT_RELATIVE: AnimPhysSimSpaceType = AnimPhysSimSpaceType(3);
    pub const BONE_RELATIVE: AnimPhysSimSpaceType = AnimPhysSimSpaceType(4);
}
#[repr(transparent)]
pub struct EDrivenDestinationMode(pub u8);
impl EDrivenDestinationMode {
    pub const BONE: EDrivenDestinationMode = EDrivenDestinationMode(0);
    pub const MORPH_TARGET: EDrivenDestinationMode = EDrivenDestinationMode(1);
    pub const MATERIAL_PARAMETER: EDrivenDestinationMode = EDrivenDestinationMode(2);
}
#[repr(transparent)]
pub struct EDrivenBoneModificationMode(pub u8);
impl EDrivenBoneModificationMode {
    pub const ADD_TO_INPUT: EDrivenBoneModificationMode = EDrivenBoneModificationMode(0);
    pub const REPLACE_COMPONENT: EDrivenBoneModificationMode = EDrivenBoneModificationMode(
        1,
    );
    pub const ADD_TO_REF_POSE: EDrivenBoneModificationMode = EDrivenBoneModificationMode(
        2,
    );
}
#[repr(transparent)]
pub struct EConstraintOffsetOption(pub u8);
impl EConstraintOffsetOption {
    pub const NONE: EConstraintOffsetOption = EConstraintOffsetOption(0);
    pub const OFFSET_REF_POSE: EConstraintOffsetOption = EConstraintOffsetOption(1);
}
#[repr(transparent)]
pub struct CopyBoneDeltaMode(pub u8);
impl CopyBoneDeltaMode {
    pub const ACCUMULATE: CopyBoneDeltaMode = CopyBoneDeltaMode(0);
    pub const COPY: CopyBoneDeltaMode = CopyBoneDeltaMode(1);
}
#[repr(transparent)]
pub struct EInterpolationBlend(pub u8);
impl EInterpolationBlend {
    pub const LINEAR: EInterpolationBlend = EInterpolationBlend(0);
    pub const CUBIC: EInterpolationBlend = EInterpolationBlend(1);
    pub const SINUSOIDAL: EInterpolationBlend = EInterpolationBlend(2);
    pub const EASE_IN_OUT_EXPONENT2: EInterpolationBlend = EInterpolationBlend(3);
    pub const EASE_IN_OUT_EXPONENT3: EInterpolationBlend = EInterpolationBlend(4);
    pub const EASE_IN_OUT_EXPONENT4: EInterpolationBlend = EInterpolationBlend(5);
    pub const EASE_IN_OUT_EXPONENT5: EInterpolationBlend = EInterpolationBlend(6);
    pub const MAX: EInterpolationBlend = EInterpolationBlend(7);
}
#[repr(transparent)]
pub struct EBoneModificationMode(pub u8);
impl EBoneModificationMode {
    pub const BMM_IGNORE: EBoneModificationMode = EBoneModificationMode(0);
    pub const BMM_REPLACE: EBoneModificationMode = EBoneModificationMode(1);
    pub const BMM_ADDITIVE: EBoneModificationMode = EBoneModificationMode(2);
}
#[repr(transparent)]
pub struct ESimulationSpace(pub u8);
impl ESimulationSpace {
    pub const COMPONENT_SPACE: ESimulationSpace = ESimulationSpace(0);
    pub const WORLD_SPACE: ESimulationSpace = ESimulationSpace(1);
    pub const BASE_BONE_SPACE: ESimulationSpace = ESimulationSpace(2);
}
#[repr(transparent)]
pub struct ESimulationTiming(pub u8);
impl ESimulationTiming {
    pub const DEFAULT: ESimulationTiming = ESimulationTiming(0);
    pub const SYNCHRONOUS: ESimulationTiming = ESimulationTiming(1);
    pub const DEFERRED: ESimulationTiming = ESimulationTiming(2);
}
#[repr(transparent)]
pub struct EScaleChainInitialLength(pub u8);
impl EScaleChainInitialLength {
    pub const FIXED_DEFAULT_LENGTH_VALUE: EScaleChainInitialLength = EScaleChainInitialLength(
        0,
    );
    pub const DISTANCE: EScaleChainInitialLength = EScaleChainInitialLength(1);
    pub const CHAIN_LENGTH: EScaleChainInitialLength = EScaleChainInitialLength(2);
}
#[repr(transparent)]
pub struct ESplineBoneAxis(pub u8);
impl ESplineBoneAxis {
    pub const NONE: ESplineBoneAxis = ESplineBoneAxis(0);
    pub const X: ESplineBoneAxis = ESplineBoneAxis(1);
    pub const Y: ESplineBoneAxis = ESplineBoneAxis(2);
    pub const Z: ESplineBoneAxis = ESplineBoneAxis(3);
}
#[repr(transparent)]
pub struct EWarpingVectorMode(pub u8);
impl EWarpingVectorMode {
    pub const COMPONENT_SPACE_VECTOR: EWarpingVectorMode = EWarpingVectorMode(0);
    pub const ACTOR_SPACE_VECTOR: EWarpingVectorMode = EWarpingVectorMode(1);
    pub const WORLD_SPACE_VECTOR: EWarpingVectorMode = EWarpingVectorMode(2);
    pub const IK_FOOT_ROOT_LOCAL_SPACE_VECTOR: EWarpingVectorMode = EWarpingVectorMode(
        3,
    );
}
#[repr(transparent)]
pub struct ESwapRootBone(pub u8);
impl ESwapRootBone {
    pub const SWAP_ROOT_BONE_COMPONENT: ESwapRootBone = ESwapRootBone(0);
    pub const SWAP_ROOT_BONE_ACTOR: ESwapRootBone = ESwapRootBone(1);
    pub const SWAP_ROOT_BONE_NONE: ESwapRootBone = ESwapRootBone(2);
}
#[repr(transparent)]
pub struct EWarpingEvaluationMode(pub u8);
impl EWarpingEvaluationMode {
    pub const MANUAL: EWarpingEvaluationMode = EWarpingEvaluationMode(0);
    pub const GRAPH: EWarpingEvaluationMode = EWarpingEvaluationMode(1);
}
